// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use super::Relocated;
use crate::{
    chain::{
        Chain, ExpectCandidatePayload, NetworkEvent, Proof, ProofSet, ProvingSection, SectionInfo,
    },
    error::RoutingError,
    id::PublicId,
    outbox::EventBox,
    parsec::{Block, Observation},
    routing_table::{Authority, Prefix},
    state_machine::Transition,
    xor_name::XorName,
};
use maidsafe_utilities::serialisation;

/// Common functionality for node states post resource proof.
pub trait Approved: Relocated {
    fn parsec_poll_one(&mut self) -> Option<Block>;
    fn chain_mut(&mut self) -> &mut Chain;

    /// Handles an accumulated `Online` event.
    fn handle_online_event(
        &mut self,
        new_pub_id: PublicId,
        new_client_auth: Authority<XorName>,
        outbox: &mut EventBox,
    ) -> Result<(), RoutingError>;

    /// Handles an accumulated `Offline` event.
    fn handle_offline_event(
        &mut self,
        pub_id: PublicId,
        outbox: &mut EventBox,
    ) -> Result<(), RoutingError>;

    /// Handles an accumulated `OurMerge` event.
    fn handle_our_merge_event(&mut self) -> Result<(), RoutingError>;

    /// Handles an accumulated `NeighbourMerge` event.
    fn handle_neighbour_merge_event(&mut self) -> Result<(), RoutingError>;

    /// Handles an accumulated `SectionInfo` event.
    fn handle_section_info_event(
        &mut self,
        sec_info: SectionInfo,
        old_pfx: Prefix<XorName>,
        outbox: &mut EventBox,
    ) -> Result<Transition, RoutingError>;

    // Handles an accumulated `ExpectCandidate` event.
    // Context: a node is joining our section. Send the node our section. If the
    // network is unbalanced, send `ExpectCandidate` on to a section with a shorter prefix.
    fn handle_expect_candidate_event(
        &mut self,
        vote: ExpectCandidatePayload,
    ) -> Result<(), RoutingError>;

    /// Handles an accumulated `ProvingSections` event.
    fn handle_proving_sections_event(
        &mut self,
        proving_secs: Vec<ProvingSection>,
        sec_info: SectionInfo,
    ) -> Result<(), RoutingError>;

    fn parsec_poll_all(&mut self, outbox: &mut EventBox) -> Result<Transition, RoutingError> {
        while let Some(block) = self.parsec_poll_one() {
            match block.payload() {
                Observation::Accusation { .. } => {
                    // FIXME: Handle properly
                    unreachable!("...")
                }
                Observation::Genesis(_) => {
                    // FIXME: Validate with Chain info.
                    continue;
                }
                Observation::OpaquePayload(event) => {
                    if let Some(proof) = block.proofs().iter().next().map(|p| Proof {
                        pub_id: *p.public_id(),
                        sig: *p.signature(),
                    }) {
                        trace!(
                            "{} Parsec OpaquePayload: {} - {:?}",
                            self,
                            proof.pub_id(),
                            event
                        );
                        self.chain_mut().handle_opaque_event(event, proof)?;
                    }
                }
                Observation::Add {
                    peer_id,
                    related_info,
                } => {
                    let event =
                        NetworkEvent::Online(*peer_id, serialisation::deserialise(&related_info)?);
                    let proof_set = to_proof_set(&block);
                    trace!("{} Parsec Add: - {}", self, peer_id);
                    self.chain_mut().handle_churn_event(&event, proof_set)?;
                }
                Observation::Remove { peer_id, .. } => {
                    let event = NetworkEvent::Offline(*peer_id);
                    let proof_set = to_proof_set(&block);
                    trace!("{} Parsec Remove: - {}", self, peer_id);
                    self.chain_mut().handle_churn_event(&event, proof_set)?;
                }
            }

            match self.chain_poll_all(outbox)? {
                Transition::Stay => (),
                transition => return Ok(transition),
            }
        }

        Ok(Transition::Stay)
    }

    fn chain_poll_all(&mut self, outbox: &mut EventBox) -> Result<Transition, RoutingError> {
        let mut our_pfx = *self.chain_mut().our_prefix();
        while let Some(event) = self.chain_mut().poll()? {
            trace!("{} Handle accumulated event: {:?}", self, event);

            match event {
                NetworkEvent::Online(pub_id, client_auth) => {
                    self.handle_online_event(pub_id, client_auth, outbox)?;
                }
                NetworkEvent::Offline(pub_id) => {
                    self.handle_offline_event(pub_id, outbox)?;
                }
                NetworkEvent::OurMerge => self.handle_our_merge_event()?,
                NetworkEvent::NeighbourMerge(_) => self.handle_neighbour_merge_event()?,
                NetworkEvent::SectionInfo(sec_info) => {
                    match self.handle_section_info_event(sec_info, our_pfx, outbox)? {
                        Transition::Stay => (),
                        transition => return Ok(transition),
                    }
                }
                NetworkEvent::ExpectCandidate(vote) => self.handle_expect_candidate_event(vote)?,
                NetworkEvent::ProvingSections(proving_secs, sec_info) => {
                    self.handle_proving_sections_event(proving_secs, sec_info)?;
                }
            }

            our_pfx = *self.chain_mut().our_prefix();
        }

        Ok(Transition::Stay)
    }
}

fn to_proof_set(block: &Block) -> ProofSet {
    let sigs = block
        .proofs()
        .iter()
        .map(|proof| (*proof.public_id(), *proof.signature()))
        .collect();
    ProofSet { sigs }
}
