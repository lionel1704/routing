// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use sodiumoxide::crypto;
use cbor;
use cbor::CborTagEncode;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rand::random;
use sodiumoxide;
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::{Signature};
use sodiumoxide::crypto::box_;
use std::cmp;
use NameType;
use name_type::closer_to_target;
use std::fmt;
use error::{RoutingError};
use id::Id;
use utils;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct PublicId {
  public_key: box_::PublicKey,
  public_sign_key: sign::PublicKey,
  name: Option<NameType>,
}

impl PublicId {
    pub fn new(id : &Id) -> PublicId {
      PublicId {
        public_key : id.get_public_key(),
        public_sign_key : id.get_public_sign_key(),
        name : id.name,
      }
    }

    pub fn name(&self) -> NameType {
      self.name.clone()
    }

    pub fn client_name(&self) -> NameType {
        utils::public_key_to_client_name(&self.public_sign_key)
    }

    pub fn serialised_contents(&self)->Result<RoutingError, Vec<u8>> {
        let mut e = cbor::Encoder::from_memory();
        try!(e.encode(&[&self]));
        e.into_bytes()
    }

    // checks if the name is equal to the self_relocated name
    pub fn is_self_relocated(&self) -> bool {
        self.name ==  utils::calculate_self_relocated_name(
            &self.public_sign_key.get_crypto_public_sign_key(),
            &self.public_key.get_crypto_public_key(), &self.validation_token)
    }

    // name field is initially same as original_name, this should be replaced by relocated name
    // calculated by the nodes close to original_name by using this method
    pub fn assign_relocated_name(&mut self, relocated_name: NameType) -> bool {
        if None(self.name) || self.name == relocated_name {
            return false;
        }
        self.name = relocated_name;
        return true;
    }
}

#[cfg(test)]
mod test {
}