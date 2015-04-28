// Copyright 2015 MaidSafe.net limited
//
// This Safe Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the Safe Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
// directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
// available at: http://maidsafe.net/network-platform-licensing
//
// Unless required by applicable law or agreed to in writing, the Safe Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
// OF ANY KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations relating to
// use of the Safe Network Software.

#![allow(unused_assignments)]

use cbor::CborTagEncode;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use types;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PutPublicPmid {
  pub public_pmid: types::PublicPmid,
}

impl Encodable for PutPublicPmid {
  fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
    CborTagEncode::new(5483_001, &(&self.public_pmid)).encode(e)
  }
}

impl Decodable for PutPublicPmid {
  fn decode<D: Decoder>(d: &mut D)->Result<PutPublicPmid, D::Error> {
    try!(d.read_u64());
    let public_pmid = try!(Decodable::decode(d));
    Ok(PutPublicPmid { public_pmid: public_pmid })
  }
}

#[cfg(test)]
mod test {
    use super::*;
    use cbor;
    use test_utils::Random;

    #[test]
    fn put_public_pmid_serialisation() {
        let obj_before : PutPublicPmid = Random::generate_random();

        let mut e = cbor::Encoder::from_memory();
        e.encode(&[&obj_before]).unwrap();

        let mut d = cbor::Decoder::from_bytes(e.as_bytes());
        let obj_after: PutPublicPmid = d.decode().next().unwrap().unwrap();

        assert_eq!(obj_before, obj_after);
    }
}
