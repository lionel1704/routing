// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use super::{
    candidate::Candidate,
    shared_state::{PrefixChange, SharedState},
    GenesisPfxInfo, NeighbourSigs, NetworkEvent, OnlinePayload, Proof, ProofSet, ProvingSection,
    SectionInfo,
};
use crate::error::RoutingError;
use crate::id::PublicId;
use crate::messages::SignedRoutingMessage;
use crate::routing_table::{Authority, Error};
use crate::sha3::Digest256;
use crate::utils::LogIdent;
use crate::utils::XorTargetInterval;
use crate::{Prefix, XorName, Xorable};
use itertools::Itertools;
use log::LogLevel;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Debug, Display, Formatter};
use std::iter;
use std::mem;

/// Amount added to `min_section_size` when deciding whether a bucket split can happen. This helps
/// protect against rapid splitting and merging in the face of moderate churn.
const SPLIT_BUFFER: usize = 1;

/// Returns the delivery group size based on the section size `n`
pub fn delivery_group_size(n: usize) -> usize {
    // this is an integer that is ≥ n/3
    (n + 2) / 3
}

/// Data chain.
pub struct Chain {
    /// Minimum number of nodes we consider acceptable in a section
    min_sec_size: usize,
    /// This node's public ID.
    our_id: PublicId,
    /// The shared state of the section.
    state: SharedState,
    /// If we're a member of the section yet. This will be toggled once we get a `SectionInfo`
    /// block accumulated which bears `our_id` as one of the members
    is_member: bool,
    /// Maps our neighbours' prefixes to their latest signed section infos, together with the
    /// signatures by some version of our own section. Note that after a split, the neighbour's
    /// latest section info could be the one from the pre-split parent section, so the value's
    /// prefix doesn't always match the key.
    neighbour_infos: BTreeMap<Prefix<XorName>, NeighbourSigs>,
    /// Their knowledge of us
    their_knowledge: BTreeMap<Prefix<XorName>, u64>,
    /// A map containing network events that have not been handled yet, together with their proofs
    /// that have been collected so far. We are still waiting for more proofs, or to reach a state
    /// where we can handle the event.
    // FIXME: Purge votes that are older than a given period.
    chain_accumulator: BTreeMap<NetworkEvent, ProofSet>,
    /// Events that were handled: Further incoming proofs for these can be ignored.
    completed_events: BTreeSet<NetworkEvent>,
    /// Pending events whose handling has been deferred due to an ongoing split or merge.
    event_cache: BTreeSet<NetworkEvent>,
    /// Current consensused candidate.
    candidate: Candidate,
}

#[allow(clippy::len_without_is_empty)]
impl Chain {
    /// Returns the minimum section size.
    pub fn min_sec_size(&self) -> usize {
        self.min_sec_size
    }

    /// Returns the number of nodes which need to exist in each subsection of a given section to
    /// allow it to be split.
    pub fn min_split_size(&self) -> usize {
        self.min_sec_size + SPLIT_BUFFER
    }

    /// Collects prefixes of all sections known by the routing table into a `BTreeSet`.
    pub fn prefixes(&self) -> BTreeSet<Prefix<XorName>> {
        self.other_prefixes()
            .iter()
            .chain(iter::once(self.state.our_info().prefix()))
            .cloned()
            .collect()
    }

    /// Create a new chain given genesis information
    pub fn new(min_sec_size: usize, our_id: PublicId, gen_info: GenesisPfxInfo) -> Self {
        // TODO validate `gen_info` to contain adequate proofs
        let is_member = gen_info.first_info.members().contains(&our_id);
        Self {
            min_sec_size,
            our_id,
            state: SharedState::new(gen_info.first_info),
            is_member,
            neighbour_infos: Default::default(),
            their_knowledge: Default::default(),
            chain_accumulator: Default::default(),
            completed_events: Default::default(),
            event_cache: Default::default(),
            candidate: Candidate::None,
        }
    }

    /// Handles an accumulated parsec Observation for membership mutation.
    ///
    /// The provided proofs wouldn't be validated against the mapped NetworkEvent as they're
    /// for parsec::Observation::Add/Remove.
    pub fn handle_churn_event(
        &mut self,
        event: &NetworkEvent,
        proof_set: ProofSet,
    ) -> Result<(), RoutingError> {
        match event {
            NetworkEvent::AddElder(_, _) | NetworkEvent::RemoveElder(_) => (),
            _ => {
                log_or_panic!(
                    LogLevel::Error,
                    "{} Invalid NetworkEvent to handle membership mutation - {:?}",
                    self,
                    event
                );
                return Err(RoutingError::InvalidStateForOperation);
            }
        }

        if !self.can_handle_vote(event) {
            // force cache with our_id as this is an accumulated event we can trust.
            let our_id = self.our_id;
            self.cache_event(event, &our_id)?;
            return Ok(());
        }

        if self.completed_events.contains(event) {
            log_or_panic!(
                LogLevel::Error,
                "{} Duplicate membership change event.",
                self
            );
            return Ok(());
        }

        if self
            .chain_accumulator
            .insert(event.clone(), proof_set)
            .is_some()
        {
            log_or_panic!(
                LogLevel::Warn,
                "{} Ejected existing ProofSet while handling membership mutation.",
                self
            );
        }

        Ok(())
    }

    /// Handles an opaque parsec Observation as a NetworkEvent.
    pub fn handle_opaque_event(
        &mut self,
        event: &NetworkEvent,
        proof: Proof,
    ) -> Result<(), RoutingError> {
        if self.should_skip_accumulator(event) {
            self.add_extra_proof_for_neighbour(event, proof);
            return Ok(());
        }

        if !self.can_handle_vote(event) {
            self.cache_event(event, proof.pub_id())?;
            return Ok(());
        }

        if self.completed_events.contains(event) {
            return Ok(());
        }

        if !self
            .chain_accumulator
            .entry(event.clone())
            .or_insert_with(ProofSet::new)
            .add_proof(proof)
        {
            // TODO: If detecting duplicate vote from peer, penalise.
            log_or_panic!(
                LogLevel::Warn,
                "{} Duplicate proof for {:?} in chain accumulator. {:?}",
                self,
                event,
                self.chain_accumulator
            );
        }
        Ok(())
    }

    /// Returns the next accumulated event.
    ///
    /// If the event is a `SectionInfo` or `NeighbourInfo`, it also updates the corresponding
    /// containers.
    pub fn poll(&mut self) -> Result<Option<NetworkEvent>, RoutingError> {
        let opt_event_proofs = self
            .chain_accumulator
            .iter()
            .find(|&(event, proofs)| self.is_valid_transition(event, proofs))
            .map(|(event, proofs)| (event.clone(), proofs.clone()));
        let (event, proofs) = match opt_event_proofs {
            None => return Ok(None),
            Some((event, proofs)) => (event, proofs),
        };
        if !self.completed_events.insert(event.clone()) {
            log_or_panic!(LogLevel::Warn, "Duplicate insert in completed events.");
        }
        let _ = self.chain_accumulator.remove(&event);

        match event {
            NetworkEvent::SectionInfo(ref sec_info) => {
                self.add_section_info(sec_info.clone(), proofs)?;
                if let Some((ref cached_sec_info, _)) = self.state.split_cache {
                    if cached_sec_info == sec_info {
                        return Ok(None);
                    }
                }
            }
            NetworkEvent::OurMerge => {
                // use new_info here as our_info might still be accumulating signatures
                // and we'd want to perform the merge eventually with our current latest state.
                let our_hash = *self.state.new_info.hash();
                let _ = self.state.merging.insert(our_hash);
                self.state.change = PrefixChange::Merging;
            }
            NetworkEvent::NeighbourMerge(digest) => {
                // TODO: Check that the section is known and not already merged.
                let _ = self.state.merging.insert(digest);
            }
            _ => (),
        }
        Ok(Some(event))
    }

    /// Adds a member to our section, creating a new `SectionInfo` in the process.
    /// If we need to split also returns an additional sibling `SectionInfo`.
    /// Should not be called while a pfx change is in progress.
    pub fn add_member(&mut self, pub_id: PublicId) -> Result<Vec<SectionInfo>, RoutingError> {
        if self.state.change != PrefixChange::None {
            log_or_panic!(
                LogLevel::Warn,
                "Adding {:?} to chain during pfx change.",
                pub_id
            );
        }

        if !self.our_prefix().matches(&pub_id.name()) {
            log_or_panic!(
                LogLevel::Error,
                "Invalid Online event {:?} for self prefix.",
                pub_id
            );
        }

        let mut members = self.state.new_info.members().clone();
        let _ = members.insert(pub_id);

        if self.should_split(&members)? {
            let (our_info, other_info) = self.split_self(members.clone())?;
            self.state.change = PrefixChange::Splitting;
            return Ok(vec![our_info, other_info]);
        }

        self.state.new_info = SectionInfo::new(
            members,
            *self.state.new_info.prefix(),
            Some(&self.state.new_info),
        )?;

        Ok(vec![self.state.new_info.clone()])
    }

    /// Removes a member from our section, creating a new `our_info` in the process.
    /// Should not be called while a pfx change is in progress.
    pub fn remove_member(&mut self, pub_id: PublicId) -> Result<SectionInfo, RoutingError> {
        if self.state.change != PrefixChange::None {
            log_or_panic!(
                LogLevel::Warn,
                "Removing {:?} from chain during pfx change.",
                pub_id
            );
        }

        if !self.our_prefix().matches(&pub_id.name()) {
            log_or_panic!(
                LogLevel::Error,
                "Invalid Offline event {:?} for self prefix.",
                pub_id
            );
        }

        let mut members = self.state.new_info.members().clone();
        let _ = members.remove(&pub_id);

        self.state.new_info = SectionInfo::new(
            members,
            *self.state.new_info.prefix(),
            Some(&self.state.new_info),
        )?;

        if self.state.new_info.members().len() < self.min_sec_size {
            // set to merge state to prevent extending chain any further.
            // We'd still not Vote for OurMerge until we've updated our_infos
            self.state.change = PrefixChange::Merging;
        }

        Ok(self.state.new_info.clone())
    }

    /// Returns the next section info if both we and our sibling have signalled for merging.
    pub fn try_merge(&mut self) -> Result<Option<SectionInfo>, RoutingError> {
        let their_info = match self.neighbour_infos.get(&self.our_prefix().sibling()) {
            Some(ni) => ni.sec_info(),
            None => return Ok(None),
        };

        self.state.try_merge(their_info)
    }

    /// Returns `true` if we have accumulated self `NetworkEvent::OurMerge`.
    pub fn is_self_merge_ready(&self) -> bool {
        self.state.is_self_merge_ready()
    }

    /// Returns `true` if we should merge.
    pub fn should_vote_for_merge(&self) -> bool {
        self.state.should_vote_for_merge(
            self.min_sec_size,
            self.neighbour_infos.values().map(NeighbourSigs::sec_info),
        )
    }

    /// Check inside the `neighbour_infos` failing which inside the chain accumulator if we have a
    /// SectionInfo with our proof for it that can validate the given SectionInfo as its next link
    pub fn is_valid_neighbour_info(&self, sec_info: &SectionInfo, proofs: &ProofSet) -> bool {
        self.compatible_neighbour_info(sec_info)
            .map_or(false, |n_info| {
                n_info == sec_info || n_info.proves_successor(sec_info, proofs)
            })
            || self
                .signed_events()
                .any(|ni_event| ni_event.proves_successor_info(sec_info, proofs))
    }

    /// Finalises a split or merge - creates a `GenesisPfxInfo` for the new graph and returns the
    /// cached and currently accumulated events.
    pub fn finalise_prefix_change(&mut self) -> Result<PrefixChangeOutcome, RoutingError> {
        // Check the lowest version of our info that any neighbour has and remove everything less
        // than it
        let our_oldest_ver = self.their_knowledge.values().min().map_or(0, |&v| v);

        if !self.state.our_infos.clean_older(our_oldest_ver) {
            log_or_panic!(
                LogLevel::Warn,
                "Oldest version indicated by neighbours not found in our infos"
            );
        }

        self.check_and_clean_neighbour_infos(None);
        self.state.change = PrefixChange::None;

        let completed_events = mem::replace(&mut self.completed_events, Default::default());
        let chain_acc = mem::replace(&mut self.chain_accumulator, Default::default());
        let event_cache = mem::replace(&mut self.event_cache, Default::default());
        let merges = mem::replace(&mut self.state.merging, Default::default())
            .into_iter()
            .map(NetworkEvent::NeighbourMerge);

        Ok(PrefixChangeOutcome {
            gen_pfx_info: GenesisPfxInfo {
                first_info: self.our_info().clone(),
                latest_info: Default::default(),
            },
            cached_events: chain_acc
                .into_iter()
                .filter(|&(ref event, ref proofs)| {
                    !completed_events.contains(event) && proofs.contains_id(&self.our_id)
                })
                .map(|(event, _)| event)
                .chain(event_cache)
                .chain(merges)
                .collect(),
            completed_events,
        })
    }

    /// Returns our public ID
    pub fn our_id(&self) -> &PublicId {
        &self.our_id
    }

    /// Returns our own current section info.
    pub fn our_info(&self) -> &SectionInfo {
        self.state.our_info()
    }

    /// Returns our own current section's prefix.
    pub fn our_prefix(&self) -> &Prefix<XorName> {
        self.state.our_prefix()
    }

    /// Returns whether our section is in the process of changing (splitting or merging).
    pub fn prefix_change(&self) -> PrefixChange {
        self.state.change
    }

    /// Returns our section info with the given hash, if it exists.
    pub fn our_info_by_hash(&self, hash: &Digest256) -> Option<&SectionInfo> {
        self.state.our_info_by_hash(hash)
    }

    /// If we are a member of the section yet. We consider ourselves to be one after we receive a
    /// `SectionInfo` block that contains us. After that we are expected to be involved in futher
    /// votings.
    pub fn is_member(&self) -> bool {
        self.is_member
    }

    /// Neighbour infos signed by our section
    pub fn neighbour_infos(&self) -> impl Iterator<Item = &SectionInfo> {
        self.neighbour_infos.values().map(NeighbourSigs::sec_info)
    }

    /// Return prefixes of all our neighbours
    pub fn other_prefixes(&self) -> BTreeSet<Prefix<XorName>> {
        self.neighbour_infos.keys().cloned().collect()
    }

    /// Inserts the `version` of our own section into `their_knowledge` for `pfx`.
    pub fn update_their_knowledge(&mut self, pfx: Prefix<XorName>, version: u64) {
        // TODO: Don't replace with earlier? What if the neighbour split or merged?
        let _ = self.their_knowledge.insert(pfx, version);
    }

    /// Checks if given `PublicId` is a valid peer by checking if we have them as a member of self
    /// section or neighbours.
    pub fn is_peer_valid(&self, pub_id: &PublicId) -> bool {
        self.neighbour_infos()
            .chain(iter::once(self.state.our_info()))
            .chain(iter::once(&self.state.new_info))
            .any(|si| si.members().contains(pub_id))
    }

    /// Returns a set of valid peers we should be connected to.
    pub fn valid_peers(&self) -> BTreeSet<&PublicId> {
        self.neighbour_infos()
            .chain(iter::once(self.state.our_info()))
            .flat_map(SectionInfo::members)
            .chain(self.state.new_info.members())
            .collect()
    }

    /// Returns `true` if we know the section `sec_info`.
    ///
    /// If `check_signed` is `true`, also trust sections that we have signed but that haven't
    /// accumulated yet.
    pub fn is_trusted(&self, sec_info: &SectionInfo, check_signed: bool) -> bool {
        let is_proof = |si: &SectionInfo| si == sec_info || si.is_successor_of(sec_info);
        let mut signed = self.signed_events().filter_map(NetworkEvent::section_info);
        if check_signed && signed.any(is_proof) {
            return true;
        }
        if sec_info.prefix().matches(self.our_id.name()) {
            self.state.our_infos().any(is_proof)
        } else {
            self.neighbour_infos().any(is_proof)
        }
    }

    /// Returns `true` if the `SectionInfo` isn't known to us yet.
    pub fn is_new(&self, sec_info: &SectionInfo) -> bool {
        let is_newer = |si: &SectionInfo| {
            si.version() >= sec_info.version() && si.prefix().is_compatible(sec_info.prefix())
        };
        let mut signed = self.signed_events().filter_map(NetworkEvent::section_info);
        if signed.any(is_newer) {
            return false;
        }
        if sec_info.prefix().matches(self.our_id.name()) {
            !self.state.our_infos().any(is_newer)
        } else {
            !self.neighbour_infos().any(is_newer)
        }
    }

    /// Returns `true` if the `SectionInfo` isn't known to us yet and is a neighbouring section.
    pub fn is_new_neighbour(&self, sec_info: &SectionInfo) -> bool {
        self.our_prefix().is_neighbour(sec_info.prefix()) && self.is_new(sec_info)
    }

    /// Appends a list of `ProvingSection`s that authenticates the message, if possible. The last
    /// section will then belong to the next hop.
    pub fn extend_proving_sections(
        &self,
        msg: &mut SignedRoutingMessage,
    ) -> Result<(), RoutingError> {
        let dst_name = msg.routing_message().dst.name();

        while (msg.previous_hop()).map_or(false, |hop| !self.is_trusted(hop, false)) {
            // We don't know the last hop! Try the one before that.
            if !msg.pop_previous_hop() {
                return Err(RoutingError::UnknownPrevHop); // We don't know any proving section.
            };
        }

        // If the previous hop is a neighbour, insert the proof from our own section.
        let proving_sections = if let Some(prev_hop) = msg.previous_hop() {
            self.get_proving_sections(prev_hop, dst_name)?
        } else {
            return Ok(()); // No sending section.
        };
        msg.extend_proving_sections(proving_sections);

        Ok(())
    }

    /// Returns a chain of proving sections leading back to `sec_info`.
    pub fn get_proving_sections(
        &self,
        sec_info: &SectionInfo,
        dst_name: XorName,
    ) -> Result<Vec<ProvingSection>, RoutingError> {
        let mut result = Vec::new();
        if !sec_info.prefix().matches(self.our_id.name()) {
            // Find the neighbour info that matches the hop or the hop's successor.
            let proves_si = |ns: &&NeighbourSigs| {
                ns.sec_info() == sec_info || ns.sec_info().is_successor_of(sec_info)
            };
            let opt_ns = self.neighbour_infos.values().find(proves_si);
            let ns = opt_ns.ok_or_else(|| {
                // Should be unreachable if section info is known.
                error!(
                    "Our prefix: {:?}, all prefixes: {:?}, sec_info.prefix(): {:?}",
                    self.our_prefix(),
                    self.prefixes(),
                    sec_info.prefix(),
                );
                log_or_panic!(LogLevel::Error, "Unknown previous section info.");
                RoutingError::UnknownPrevHop
            })?;

            // If it's not `sec_info` itself but its successor, insert it as a proving section.
            if ns.sec_info().is_successor_of(sec_info) {
                result.push(ProvingSection::successor(ns.sec_info()));
            }

            // Now insert our own proof of the neighbour section.
            let our_info = self
                .state
                .our_info_by_version(ns.our_version())
                .ok_or_else(|| {
                    log_or_panic!(
                        LogLevel::Error,
                        "No matching own section info for signed neighbour section."
                    );
                    RoutingError::InvalidStateForOperation
                })?;
            result.push(ProvingSection::signatures(our_info, ns.proofs()));
        }

        // If the destination is our own section, insert links up to the latest version and return.
        // Otherwise, insert links up to the version known by the next hop.
        let from_ver = *result
            .last()
            .map_or(sec_info, |last| &last.sec_info)
            .version();
        let our_ver = *self.our_info().version();
        if self.our_prefix().matches(&dst_name) {
            result.extend(self.state.proving_sections_to_own(from_ver, our_ver));
        } else {
            let to_version = |(_, version): (_, &u64)| *version;
            let is_closer = |&(pfx, _): &(&Prefix<_>, _)| {
                pfx.common_prefix(&dst_name) > self.our_prefix().common_prefix(&dst_name)
            };
            let known_version = self.their_knowledge.iter().find(is_closer).map(to_version);
            let to_ver = known_version.unwrap_or_else(|| {
                self.state
                    .our_infos()
                    .nth(0)
                    .map(SectionInfo::version)
                    .cloned()
                    .unwrap_or(0)
            });
            result.extend(self.state.proving_sections_to_own(from_ver, to_ver));
            result.extend(self.state.proving_sections_to_own(to_ver, our_ver));
        }
        Ok(result)
    }

    /// Returns `true` if the given `NetworkEvent` is already accumulated and can be skipped.
    fn should_skip_accumulator(&self, event: &NetworkEvent) -> bool {
        // FIXME: may also need to handle non SI votes to not get handled multiple times
        let si = match *event {
            NetworkEvent::SectionInfo(ref si) => si,
            _ => return false,
        };

        // we can ignore self SI additional votes we do not require.
        if si.prefix().matches(self.our_id.name()) && self.our_info().version() >= si.version() {
            return true;
        }

        // we can skip neighbour infos we've already accumulated
        if self
            .neighbour_infos
            .iter()
            .any(|(pfx, nsigs)| pfx == si.prefix() && nsigs.sec_info().version() >= si.version())
        {
            return true;
        }

        false
    }

    /// If incoming event is for a neighbour we currently hold, accept the proof directly.
    /// This helps with keeping the neighbour_infos signed by latest self section.
    fn add_extra_proof_for_neighbour(&mut self, event: &NetworkEvent, proof: Proof) {
        let si = match *event {
            NetworkEvent::SectionInfo(ref si) => si,
            _ => return,
        };

        let add_proof =
            |nsigs: &mut NeighbourSigs| nsigs.sec_info() == si && nsigs.add_proof(proof);
        if self.neighbour_infos.values_mut().any(add_proof) {
            self.check_and_clean_neighbour_infos(Some(si.prefix()));
        }
    }

    /// If given `NetworkEvent` is a `SectionInfo`, returns `true` if we have the previous
    /// `SectionInfo` in our_infos/neighbour_infos OR if its a valid neighbour pfx
    /// we do not currently have in our chain.
    /// Returns `true` for other types of `NetworkEvent`.
    fn is_valid_transition(&self, network_event: &NetworkEvent, proofs: &ProofSet) -> bool {
        match *network_event {
            NetworkEvent::SectionInfo(ref info) => {
                // Reject any info we have a newer compatible info for.
                let is_newer = |i: &SectionInfo| {
                    info.prefix().is_compatible(i.prefix()) && i.version() >= info.version()
                };
                if self
                    .compatible_neighbour_info(info)
                    .into_iter()
                    .chain(iter::once(self.our_info()))
                    .any(is_newer)
                {
                    return false;
                }

                // Ensure our infos is forming an unbroken sequence.
                if info.prefix().matches(self.our_id.name()) {
                    return info.is_successor_of(self.our_info())
                        && self.our_info().is_quorum(proofs);
                }

                self.our_info().is_quorum(proofs)
            }

            NetworkEvent::AddElder(_, _)
            | NetworkEvent::RemoveElder(_)
            | NetworkEvent::Online(_)
            | NetworkEvent::Offline(_)
            | NetworkEvent::ExpectCandidate(_)
            | NetworkEvent::PurgeCandidate(_) => {
                self.state.change == PrefixChange::None && self.our_info().is_quorum(proofs)
            }
            NetworkEvent::ProvingSections(_, _) => true,

            NetworkEvent::OurMerge | NetworkEvent::NeighbourMerge(_) => {
                self.our_info().is_quorum(proofs)
            }
        }
    }

    fn compatible_neighbour_info<'a>(&'a self, si: &'a SectionInfo) -> Option<&'a SectionInfo> {
        self.neighbour_infos
            .iter()
            .find(move |&(pfx, _)| pfx.is_compatible(si.prefix()))
            .map(|(_, nsigs)| nsigs.sec_info())
    }

    /// Check if we can handle a given event immediately.
    /// Returns `true` if we are not in the process of waiting for a pfx change
    /// or if incoming event is a vote for the ongoing pfx change.
    fn can_handle_vote(&self, event: &NetworkEvent) -> bool {
        // TODO: is the merge state check even needed in the following match?
        // we only seem to set self.state = Merging after accumulation of OurMerge
        match (self.state.change, event) {
            (PrefixChange::None, _)
            | (PrefixChange::Merging, NetworkEvent::OurMerge)
            | (PrefixChange::Merging, NetworkEvent::NeighbourMerge(_)) => true,
            (_, NetworkEvent::SectionInfo(sec_info)) => {
                if sec_info.prefix().is_compatible(self.our_prefix())
                    && sec_info.version() > self.state.new_info.version()
                {
                    log_or_panic!(
                        LogLevel::Error,
                        "We shouldn't have progressed past the split/merged version."
                    );
                    return false;
                }
                true
            }
            (_, _) => false, // Don't want to handle any events other than `SectionInfo`.
        }
    }

    /// Store given event if created by us for use later on.
    fn cache_event(
        &mut self,
        net_event: &NetworkEvent,
        sender_id: &PublicId,
    ) -> Result<(), RoutingError> {
        if self.state.change == PrefixChange::None {
            log_or_panic!(
                LogLevel::Error,
                "Shouldn't be caching events while not splitting or merging."
            );
        }
        if self.our_id == *sender_id {
            let _ = self.event_cache.insert(net_event.clone());
        }
        Ok(())
    }

    /// Handles our own section info, or the section info of our sibling directly after a split.
    fn add_section_info(
        &mut self,
        sec_info: SectionInfo,
        proofs: ProofSet,
    ) -> Result<(), RoutingError> {
        // Split handling alone. wouldn't cater to merge
        if sec_info.prefix().is_extension_of(self.our_prefix()) {
            match self.state.split_cache.take() {
                None => {
                    self.state.split_cache = Some((sec_info, proofs));
                    return Ok(());
                }
                Some((cache_info, cache_proofs)) => {
                    let cache_pfx = *cache_info.prefix();

                    // Add our_info first so when we add sibling info, its a valid neighbour prefix
                    // which does not get immediately purged.
                    if cache_pfx.matches(self.our_id.name()) {
                        self.do_add_section_info(cache_info, cache_proofs)?;
                        self.do_add_section_info(sec_info, proofs)?;
                    } else {
                        self.do_add_section_info(sec_info, proofs)?;
                        self.do_add_section_info(cache_info, cache_proofs)?;
                    }
                    return Ok(());
                }
            }
        }

        self.do_add_section_info(sec_info, proofs)
    }

    fn do_add_section_info(
        &mut self,
        sec_info: SectionInfo,
        proofs: ProofSet,
    ) -> Result<(), RoutingError> {
        let pfx = *sec_info.prefix();
        if pfx.matches(self.our_id.name()) {
            self.state.our_infos.push((sec_info.clone(), proofs));
            if !self.is_member && sec_info.members().contains(&self.our_id) {
                self.is_member = true;
            }
            self.check_and_clean_neighbour_infos(None);
        } else {
            let ppfx = sec_info.prefix().popped();
            let spfx = sec_info.prefix().sibling();
            let new_nsigs_version = *sec_info.version();
            let nsigs = self
                .state
                .our_infos()
                .rev()
                .find(|our_info| our_info.is_quorum(&proofs))
                .map(|our_info| NeighbourSigs::new(sec_info, proofs, our_info))
                .ok_or(RoutingError::InvalidMessage)?;

            if let Some(old_nsigs) = self.neighbour_infos.insert(pfx, nsigs) {
                if *old_nsigs.sec_info().version() > new_nsigs_version {
                    log_or_panic!(
                        LogLevel::Error,
                        "{} Ejected newer neighbour info {:?}",
                        self,
                        old_nsigs
                    );
                }
            }

            // If we just split an existing neighbour and we also need its sibling,
            // add the sibling prefix with the parent prefix sigs.
            if let Some(ssigs) = self
                .neighbour_infos
                .get(&ppfx)
                .filter(|psigs| {
                    *psigs.sec_info().version() < new_nsigs_version
                        && self.our_prefix().is_neighbour(&spfx)
                        && !self.neighbour_infos.contains_key(&spfx)
                })
                .cloned()
            {
                let _ = self.neighbour_infos.insert(spfx, ssigs);
            }

            self.check_and_clean_neighbour_infos(Some(&pfx));
        }
        Ok(())
    }

    /// Returns whether we should split into two sections.
    fn should_split(&self, members: &BTreeSet<PublicId>) -> Result<bool, RoutingError> {
        if self.state.change != PrefixChange::None || self.should_vote_for_merge() {
            return Ok(false);
        }

        let new_size = members
            .iter()
            .filter(|id| {
                self.our_id.name().common_prefix(id.name()) > self.our_prefix().bit_count()
            })
            .count();
        let min_split_size = self.min_split_size();
        // If either of the two new sections will not contain enough entries, return `false`.
        Ok(new_size >= min_split_size && members.len() >= min_split_size + new_size)
    }

    /// Splits our section and generates new section infos for the child sections.
    fn split_self(
        &mut self,
        members: BTreeSet<PublicId>,
    ) -> Result<(SectionInfo, SectionInfo), RoutingError> {
        let next_bit = self.our_id.name().bit(self.our_prefix().bit_count());

        let our_prefix = self.our_prefix().pushed(next_bit);
        let other_prefix = self.our_prefix().pushed(!next_bit);

        let (our_new_section, other_section) =
            members.iter().partition(|id| our_prefix.matches(id.name()));

        let our_new_info =
            SectionInfo::new(our_new_section, our_prefix, Some(&self.state.new_info))?;
        let other_info = SectionInfo::new(other_section, other_prefix, Some(&self.state.new_info))?;

        self.state.new_info = our_new_info.clone();

        Ok((our_new_info, other_info))
    }

    /// Update our version which has signed the neighbour infos to whichever latest version
    /// possible.
    ///
    /// If we want to do for a particular `NeighbourInfo` then supply that else we will go over the
    /// entire list.
    fn check_and_clean_neighbour_infos(&mut self, for_pfx: Option<&Prefix<XorName>>) {
        // Update neighbour version signed by self section
        let our_infos: Vec<_> = self.state.our_infos().collect();

        let update_version = |nsigs: &mut NeighbourSigs| {
            for our_info in our_infos.iter().rev() {
                if nsigs.update_version(our_info) {
                    break;
                }
            }
        };

        if let Some(pfx) = for_pfx {
            self.neighbour_infos.get_mut(pfx).map_or((), update_version);
        } else {
            self.neighbour_infos.values_mut().for_each(update_version);
        }

        // Remove invalid neighbour pfx, older version of compatible pfx.
        let to_remove: Vec<Prefix<XorName>> = self
            .neighbour_infos
            .iter()
            .filter_map(|(pfx, nsigs)| {
                if !self.our_prefix().is_neighbour(pfx) {
                    // we just split making old neighbour no longer needed
                    return Some(*pfx);
                }

                // Remove older compatible neighbour prefixes.
                let is_newer = |(other_pfx, other_sigs): (&Prefix<XorName>, &NeighbourSigs)| {
                    other_pfx.is_compatible(pfx)
                        && other_sigs.sec_info().version() > nsigs.sec_info().version()
                };

                if self.neighbour_infos.iter().any(is_newer) {
                    return Some(*pfx);
                }

                None
            })
            .collect();
        for pfx in to_remove {
            let _ = self.neighbour_infos.remove(&pfx);
        }
    }

    /// Returns all network events that we have signed but haven't accumulated yet.
    fn signed_events(&self) -> impl Iterator<Item = &NetworkEvent> {
        self.chain_accumulator
            .iter()
            .filter(move |(_, proofs)| proofs.contains_id(&self.our_id))
            .map(|(event, _)| event)
    }

    // Set of methods ported over from routing_table mostly as-is. The idea is to refactor and
    // restructure them after they've all been ported over.

    /// Returns an iterator over all neighbouring sections and our own, together with their prefix
    /// in the map.
    pub fn all_sections(&self) -> impl Iterator<Item = (Prefix<XorName>, &SectionInfo)> {
        self.neighbour_infos
            .iter()
            .map(|(pfx, sec_sigs)| (*pfx, sec_sigs.sec_info()))
            .chain(iter::once((
                *self.state.our_info().prefix(),
                self.state.our_info(),
            )))
    }

    /// Finds the `count` names closest to `name` in the whole routing table.
    fn closest_known_names(
        &self,
        name: &XorName,
        count: usize,
        connected_peers: &[&XorName],
    ) -> Vec<XorName> {
        self.all_sections()
            .sorted_by(|&(pfx0, _), &(pfx1, _)| pfx0.cmp_distance(&pfx1, name))
            .into_iter()
            .flat_map(|(_, si)| {
                si.member_names()
                    .into_iter()
                    .sorted_by(|name0, name1| name.cmp_distance(name0, name1))
            })
            .filter(|name| connected_peers.contains(&name))
            .take(count)
            .collect_vec()
    }

    /// Returns whether the table contains the given `name`.
    fn has(&self, name: &XorName) -> bool {
        self.get_section_legacy(name)
            .map_or(false, |section| section.contains(name))
    }

    /// Returns the section matching the given `name`, if present.
    /// Includes our own name in the case that our prefix matches `name`.
    fn get_section_legacy(&self, name: &XorName) -> Option<BTreeSet<XorName>> {
        if self.our_prefix().matches(name) {
            return Some(self.our_info().member_names());
        }
        self.neighbour_infos
            .iter()
            .find(|&(ref pfx, _)| pfx.matches(name))
            .map(|(_, ref ni)| ni.sec_info().member_names())
    }

    /// If our section is the closest one to `name`, returns all names in our section *including
    /// ours*, otherwise returns `None`.
    pub fn close_names(&self, name: &XorName) -> Option<Vec<XorName>> {
        if self.our_prefix().matches(name) {
            Some(
                self.our_info()
                    .members()
                    .iter()
                    .map(|id| *id.name())
                    .collect(),
            )
        } else {
            None
        }
    }

    /// If our section is the closest one to `name`, returns all names in our section *excluding
    /// ours*, otherwise returns `None`.
    pub fn other_close_names(&self, name: &XorName) -> Option<BTreeSet<XorName>> {
        if self.our_prefix().matches(name) {
            let mut section = self.our_info().member_names();
            let _ = section.remove(&self.our_id().name());
            Some(section)
        } else {
            None
        }
    }

    /// Returns the `count` closest entries to `name` in the routing table, including our own name,
    /// sorted by ascending distance to `name`. If we are not close, returns `None`.
    pub fn closest_names(
        &self,
        name: &XorName,
        count: usize,
        connected_peers: &[&XorName],
    ) -> Option<Vec<XorName>> {
        let result = self.closest_known_names(name, count, connected_peers);
        if result.contains(&&self.our_id().name()) {
            Some(result)
        } else {
            None
        }
    }

    /// Returns the `count-1` closest entries to `name` in the routing table, excluding
    /// our own name, sorted by ascending distance to `name` -  or `None`, if our name
    /// isn't among `count` names closest to `name`.
    fn other_closest_names(
        &self,
        name: &XorName,
        count: usize,
        connected_peers: &[&XorName],
    ) -> Option<Vec<XorName>> {
        self.closest_names(name, count, connected_peers)
            .map(|mut result| {
                result.retain(|name| name != self.our_id().name());
                result
            })
    }

    /// Returns the prefix of the closest non-empty section to `name`, regardless of whether `name`
    /// belongs in that section or not, and the section itself.
    fn closest_section(&self, name: &XorName) -> (Prefix<XorName>, BTreeSet<XorName>) {
        let mut best_pfx = *self.our_prefix();
        let mut best_si = self.our_info();
        for (pfx, ni) in &self.neighbour_infos {
            // TODO: Remove the first check after verifying that section infos are never empty.
            if !ni.sec_info().members().is_empty()
                && best_pfx.cmp_distance(&pfx, name) == Ordering::Greater
            {
                best_pfx = *pfx;
                best_si = ni.sec_info();
            }
        }
        (best_pfx, best_si.member_names())
    }

    /// Returns the known sections sorted by the distance from a given XorName.
    fn closest_sections(&self, name: &XorName) -> Vec<(Prefix<XorName>, BTreeSet<XorName>)> {
        let mut result = vec![(*self.our_prefix(), self.our_info().member_names())];
        for (pfx, ni) in &self.neighbour_infos {
            result.push((*pfx, ni.sec_info().member_names()));
        }
        result.sort_by(|lhs, rhs| lhs.0.cmp_distance(&rhs.0, name));
        result
    }

    /// Returns a set of nodes to which a message for the given `Authority` could be sent
    /// onwards, sorted by priority, along with the number of targets the message should be sent to.
    /// If the total number of targets returned is larger than this number, the spare targets can
    /// be used if the message can't be delivered to some of the initial ones.
    ///
    /// * If the destination is an `Authority::Section`:
    ///     - if our section is the closest on the network (i.e. our section's prefix is a prefix of
    ///       the destination), returns all other members of our section; otherwise
    ///     - returns the `N/3` closest members of the RT to the target
    ///
    /// * If the destination is an `Authority::PrefixSection`:
    ///     - if the prefix is compatible with our prefix and is fully-covered by prefixes in our
    ///       RT, returns all members in these prefixes except ourself; otherwise
    ///     - if the prefix is compatible with our prefix and is *not* fully-covered by prefixes in
    ///       our RT, returns `Err(Error::CannotRoute)`; otherwise
    ///     - returns the `N/3` closest members of the RT to the lower bound of the target
    ///       prefix
    ///
    /// * If the destination is a group (`ClientManager`, `NaeManager` or `NodeManager`):
    ///     - if our section is the closest on the network (i.e. our section's prefix is a prefix of
    ///       the destination), returns all other members of our section; otherwise
    ///     - returns the `N/3` closest members of the RT to the target
    ///
    /// * If the destination is an individual node (`ManagedNode` or `Client`):
    ///     - if our name *is* the destination, returns an empty set; otherwise
    ///     - if the destination name is an entry in the routing table, returns it; otherwise
    ///     - returns the `N/3` closest members of the RT to the target
    pub fn targets(
        &self,
        dst: &Authority<XorName>,
        connected_peers: &[&XorName],
    ) -> Result<(Vec<XorName>, usize), Error> {
        // FIXME: only filtering for now to match RT.
        // should confirm if needed esp after msg_relay changes.
        let is_connected = |target_name: &XorName| connected_peers.contains(&target_name);

        let candidates = |target_name: &XorName| {
            let mut filtered_sections =
                self.closest_sections(target_name)
                    .into_iter()
                    .map(|(_, members)| {
                        (
                            members.len(),
                            members.into_iter().filter(is_connected).collect::<Vec<_>>(),
                        )
                    });
            let best_section = filtered_sections
                .find(|(len, connected)| connected.len() * 3 >= *len)
                .ok_or(Error::CannotRoute);
            best_section.map(|(len, connected)| {
                (
                    len,
                    connected
                        .into_iter()
                        .sorted_by(|lhs, rhs| target_name.cmp_distance(lhs, rhs)),
                )
            })
        };

        let (best_section_len, mut best_section) = match *dst {
            Authority::ManagedNode(ref target_name)
            | Authority::Client {
                proxy_node_name: ref target_name,
                ..
            } => {
                if target_name == self.our_id().name() {
                    return Ok((Vec::new(), 0));
                }
                if self.has(target_name) && is_connected(&target_name) {
                    return Ok((vec![*target_name], 1));
                }
                candidates(target_name)?
            }
            Authority::ClientManager(ref target_name)
            | Authority::NaeManager(ref target_name)
            | Authority::NodeManager(ref target_name) => {
                if let Some(group) =
                    self.other_closest_names(target_name, self.min_sec_size, &connected_peers)
                {
                    let group_len = group.len();
                    return Ok((group, group_len));
                }
                candidates(target_name)?
            }
            Authority::Section(ref target_name) => {
                let (prefix, section) = self.closest_section(target_name);
                if &prefix == self.our_prefix() {
                    // Exclude our name since we don't need to send to ourself
                    let mut section = section.clone();
                    let _ = section.remove(&self.our_id().name());

                    // FIXME: only doing this for now to match RT.
                    // should confirm if needed esp after msg_relay changes.
                    let section: Vec<_> = section.into_iter().filter(is_connected).collect();
                    let dg_size = section.len();
                    return Ok((section, dg_size));
                }
                candidates(target_name)?
            }
            Authority::PrefixSection(ref prefix) => {
                if prefix.is_compatible(&self.our_prefix()) {
                    // only route the message when we have all the targets in our routing table -
                    // this is to prevent spamming the network by sending messages with
                    // intentionally short prefixes
                    if !prefix.is_covered_by(self.prefixes().iter()) {
                        return Err(Error::CannotRoute);
                    }

                    let is_compatible = |(ref pfx, section)| {
                        if prefix.is_compatible(pfx) {
                            Some(section)
                        } else {
                            None
                        }
                    };

                    let targets = Iterator::flatten(
                        self.all_sections()
                            .filter_map(is_compatible)
                            .map(SectionInfo::member_names),
                    )
                    .filter(is_connected)
                    .filter(|name| name != self.our_id().name())
                    .collect::<Vec<_>>();
                    let dg_size = targets.len();
                    return Ok((targets, dg_size));
                }
                candidates(&prefix.lower_bound())?
            }
        };

        best_section.retain(|&x| x != *self.our_id().name());
        Ok((best_section, delivery_group_size(best_section_len)))
    }

    /// Returns our own section, including our own name.
    pub fn our_section(&self) -> BTreeSet<XorName> {
        self.state.our_info().member_names()
    }

    /// Are we among the `count` closest nodes to `name`?
    fn is_closest(&self, name: &XorName, count: usize, connected_peers: &[&XorName]) -> bool {
        self.closest_names(name, count, connected_peers).is_some()
    }

    /// Returns whether we are a part of the given authority.
    pub fn in_authority(&self, auth: &Authority<XorName>, connected_peers: &[&XorName]) -> bool {
        match *auth {
            // clients have no routing tables
            Authority::Client { .. } => false,
            Authority::ManagedNode(ref name) => self.our_id().name() == name,
            Authority::ClientManager(ref name)
            | Authority::NaeManager(ref name)
            | Authority::NodeManager(ref name) => {
                self.is_closest(name, self.min_sec_size, connected_peers)
            }
            Authority::Section(ref name) => self.our_prefix().matches(name),
            Authority::PrefixSection(ref prefix) => self.our_prefix().is_compatible(prefix),
        }
    }

    /// Returns the total number of entries in the routing table, excluding our own name.
    pub fn len(&self) -> usize {
        self.neighbour_infos
            .values()
            .map(|ni| ni.sec_info().members().len())
            .sum::<usize>()
            + self.state.our_info().members().len()
            - 1
    }

    /// Compute an estimate of the size of the network from the size of our routing table.
    ///
    /// Return (estimate, exact), with exact = true iff we have the whole network in our
    /// routing table.
    pub fn network_size_estimate(&self) -> (u64, bool) {
        let known_prefixes = self.prefixes();
        let is_exact = Prefix::default().is_covered_by(known_prefixes.iter());

        // Estimated fraction of the network that we have in our RT.
        // Computed as the sum of 1 / 2^(prefix.bit_count) for all known section prefixes.
        let network_fraction: f64 = known_prefixes
            .iter()
            .map(|p| 1.0 / (p.bit_count() as f64).exp2())
            .sum();

        // Total size estimate = known_nodes / network_fraction
        let network_size = self.valid_peers().len() as f64 / network_fraction;

        (network_size.ceil() as u64, is_exact)
    }

    /// Return a minimum length prefix, favouring our prefix if it is one of the shortest.
    pub fn min_len_prefix(&self) -> Prefix<XorName> {
        *iter::once(self.our_prefix())
            .chain(self.neighbour_infos.keys())
            .min_by_key(|prefix| prefix.bit_count())
            .unwrap_or(&self.our_prefix())
    }

    /// Return true if already has a candidate
    pub fn has_resource_proof_candidate(&self) -> bool {
        !self.candidate.is_none()
    }

    /// Forget about the current candidate.
    pub fn reset_candidate(&mut self) {
        self.candidate.reset()
    }

    /// Forget about the current candidate if it is a member of the given section.
    pub fn reset_candidate_if_member_of(&mut self, members: &BTreeSet<PublicId>) {
        self.candidate.reset_if_member_of(members)
    }

    /// Return true if we are waiting for candidate info for that PublicId.
    pub fn matching_candidate_target_interval(
        &self,
        old_pub_id: &PublicId,
    ) -> Option<&XorTargetInterval> {
        self.candidate.matching_target_interval(old_pub_id)
    }

    /// Our section decided that the candidate should be selected next.
    /// Pre-condition: !has_resource_proof_candidate.
    pub fn accept_as_candidate(
        &mut self,
        old_pub_id: PublicId,
        target_interval: XorTargetInterval,
    ) {
        self.candidate
            .accept_for_resource_proof(old_pub_id, target_interval)
    }

    /// Handle consensus on `Online`. Marks the candidate as `ApprovedWaitingSectionInfo`.
    /// If the candidate was already purged or is unexpected, return false.
    pub fn try_accept_candidate_as_member(&mut self, online_payload: &OnlinePayload) -> bool {
        self.candidate.try_accept_as_member(online_payload)
    }

    /// The public id of the candidate we are waiting to approve.
    pub fn candidate_old_public_id(&self) -> Option<&PublicId> {
        self.candidate.old_public_id()
    }

    /// Logs info about ongoing candidate state, if any.
    pub fn show_candidate_status(&self, log_ident: &LogIdent) {
        self.candidate.show_status(log_ident)
    }
}

/// The outcome of a prefix change.
pub struct PrefixChangeOutcome {
    /// The new genesis prefix info.
    pub gen_pfx_info: GenesisPfxInfo,
    /// The cached events that should be revoted.
    pub cached_events: BTreeSet<NetworkEvent>,
    /// The completed events.
    pub completed_events: BTreeSet<NetworkEvent>,
}

impl Debug for Chain {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "Chain {{")?;
        writeln!(formatter, "\tchange: {:?},", self.state.change)?;
        writeln!(formatter, "\tour_id: {},", self.our_id)?;
        writeln!(formatter, "\tour_version: {}", self.state.our_version())?;
        writeln!(formatter, "\tis_member: {},", self.is_member)?;
        writeln!(formatter, "\tnew_info: {}", self.state.new_info)?;
        writeln!(formatter, "\tmerging: {:?}", self.state.merging)?;

        writeln!(formatter, "\tour_infos: len {}", self.state.our_infos.len())?;
        for sec_info in self.state.our_infos() {
            writeln!(formatter, "\t{}", sec_info)?;
        }

        writeln!(formatter, "\tneighbour_infos:")?;
        for (pfx, nsigs) in &self.neighbour_infos {
            writeln!(
                formatter,
                "\t {:?} signed by our_version: {}",
                pfx,
                nsigs.our_version()
            )?;
            writeln!(formatter, "\t {}", nsigs.sec_info())?;
        }

        writeln!(
            formatter,
            "\ttheir_knowledge: len {}",
            self.their_knowledge.len()
        )?;
        for (pfx, version) in &self.their_knowledge {
            writeln!(formatter, "\t{:?} knows our_version: {}", pfx, version)?;
        }

        writeln!(formatter, "}}")
    }
}

impl Display for Chain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Node({}({:b}))", self.our_id(), self.state.our_prefix())
    }
}

#[cfg(any(test, feature = "mock_base"))]
impl Chain {
    /// Returns the members of the section with the given prefix (if it exists)
    pub fn get_section(&self, pfx: &Prefix<XorName>) -> Option<&SectionInfo> {
        if self.our_prefix() == pfx {
            Some(self.our_info())
        } else {
            self.neighbour_infos.get(pfx).map(NeighbourSigs::sec_info)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{GenesisPfxInfo, Proof, ProofSet, SectionInfo};
    use super::Chain;
    use crate::id::{FullId, PublicId};
    use crate::{Prefix, XorName, MIN_SECTION_SIZE};
    use rand::{thread_rng, Rng};
    use serde::Serialize;
    use std::collections::{BTreeSet, HashMap};
    use std::str::FromStr;
    use unwrap::unwrap;

    enum SecInfoGen<'a> {
        New(Prefix<XorName>, usize),
        Add(&'a SectionInfo),
        Remove(&'a SectionInfo),
    }

    fn gen_section_info(gen: SecInfoGen) -> (SectionInfo, HashMap<PublicId, FullId>) {
        match gen {
            SecInfoGen::New(pfx, n) => {
                let mut full_ids = HashMap::new();
                let mut members = BTreeSet::new();
                for _ in 0..n {
                    let some_id = FullId::within_range(&pfx.range_inclusive());
                    let _ = members.insert(*some_id.public_id());
                    let _ = full_ids.insert(*some_id.public_id(), some_id);
                }
                (SectionInfo::new(members, pfx, None).unwrap(), full_ids)
            }
            SecInfoGen::Add(info) => {
                let mut members = info.members().clone();
                let some_id = FullId::within_range(&info.prefix().range_inclusive());
                let _ = members.insert(*some_id.public_id());
                let mut full_ids = HashMap::new();
                let _ = full_ids.insert(*some_id.public_id(), some_id);
                (
                    SectionInfo::new(members, *info.prefix(), Some(info)).unwrap(),
                    full_ids,
                )
            }
            SecInfoGen::Remove(info) => {
                let members = info.members().clone();
                (
                    SectionInfo::new(members, *info.prefix(), Some(info)).unwrap(),
                    Default::default(),
                )
            }
        }
    }

    fn gen_proofs<'a, S, I>(
        full_ids: &HashMap<PublicId, FullId>,
        members: I,
        payload: &S,
    ) -> ProofSet
    where
        S: Serialize,
        I: IntoIterator<Item = &'a PublicId>,
    {
        let mut proofs = ProofSet::new();
        for member in members {
            let _ = full_ids.get(member).map(|full_id| {
                let proof = unwrap!(Proof::new(
                    *full_id.public_id(),
                    full_id.signing_private_key(),
                    payload,
                ));
                let _ = proofs.add_proof(proof);
            });
        }
        proofs
    }

    fn gen_chain<T>(min_sec_size: usize, sections: T) -> (Chain, HashMap<PublicId, FullId>)
    where
        T: IntoIterator<Item = (Prefix<XorName>, usize)>,
    {
        let mut full_ids = HashMap::new();
        let mut our_id = None;
        let mut section_members = vec![];
        for (pfx, size) in sections {
            let (info, ids) = gen_section_info(SecInfoGen::New(pfx, size));
            if our_id.is_none() {
                our_id = Some(unwrap!(ids.values().next()).clone());
            }
            full_ids.extend(ids);
            section_members.push(info);
        }

        let our_id = unwrap!(our_id);
        let mut sections_iter = section_members.into_iter();

        let first_info = sections_iter.next().expect("section members");
        let our_members = first_info.members().clone();
        let genesis_info = GenesisPfxInfo {
            first_info,
            latest_info: Default::default(),
        };

        let mut chain = Chain::new(min_sec_size, *our_id.public_id(), genesis_info);

        for neighbour_info in sections_iter {
            let proofs = gen_proofs(&full_ids, &our_members, &neighbour_info);
            unwrap!(chain.add_section_info(neighbour_info, proofs));
        }

        (chain, full_ids)
    }

    #[test]
    fn generate_chain() {
        let (chain, _ids) = gen_chain(
            MIN_SECTION_SIZE,
            vec![
                (Prefix::from_str("00").unwrap(), 8),
                (Prefix::from_str("01").unwrap(), 8),
                (Prefix::from_str("10").unwrap(), 8),
            ],
        );
        assert!(!chain
            .get_section(&Prefix::from_str("00").unwrap())
            .expect("No section 00 found!")
            .members()
            .is_empty());
        assert!(chain.get_section(&Prefix::from_str("").unwrap()).is_none());
    }

    fn check_infos_for_duplication(chain: &Chain) {
        let mut prefixes: Vec<Prefix<XorName>> = vec![];
        for info in chain.neighbour_infos() {
            if let Some(pfx) = prefixes.iter().find(|x| x.is_compatible(info.prefix())) {
                panic!(
                    "Found compatible prefixes! {:?} and {:?}",
                    pfx,
                    info.prefix()
                );
            }
            prefixes.push(*info.prefix());
        }
    }

    #[test]
    fn neighbour_info_cleaning() {
        let mut rng = thread_rng();
        let p_00 = Prefix::from_str("00").unwrap();
        let p_01 = Prefix::from_str("01").unwrap();
        let p_10 = Prefix::from_str("10").unwrap();
        let (mut chain, mut full_ids) =
            gen_chain(MIN_SECTION_SIZE, vec![(p_00, 8), (p_01, 8), (p_10, 8)]);
        for _ in 0..1000 {
            let (new_info, new_ids) = {
                let old_info: Vec<_> = chain.neighbour_infos().collect();
                let info = rng.choose(&old_info).expect("neighbour infos");
                if rng.gen_weighted_bool(2) {
                    gen_section_info(SecInfoGen::Add(info))
                } else {
                    gen_section_info(SecInfoGen::Remove(info))
                }
            };
            full_ids.extend(new_ids);
            let proofs = gen_proofs(&full_ids, chain.our_info().members(), &new_info);
            unwrap!(chain.add_section_info(new_info, proofs));
            check_infos_for_duplication(&chain);
        }
    }
}
