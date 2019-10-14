#![feature(prelude_import)]
#![no_std]
// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! # BABE consensus
//!
//! BABE (Blind Assignment for Blockchain Extension) consensus in substrate.
//!
//! # Stability
//!
//! This crate is highly unstable and experimental.  Breaking changes may
//! happen at any point.  This crate is also missing features, such as banning
//! of malicious validators, that are essential for a production network.
#![forbid(unsafe_code, missing_docs)]
#![deny(warnings)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
extern crate core;
pub use babe_primitives::*;
pub use consensus_common::SyncOracle;
use consensus_common::ExtraVerification;
use runtime_primitives::{generic, generic::BlockId, Justification};
use runtime_primitives::traits::{Block, Header, Digest, DigestItemFor,
                                 DigestItem, ProvideRuntimeApi,
                                 AuthorityIdFor};
use std::{sync::Arc, u64, fmt::Debug};
use parity_codec::{Decode, Encode, Input};
use primitives::{crypto::Pair,
                 sr25519::{Public, Signature, LocalizedSignature, self}};
use merlin::Transcript;
use inherents::{InherentDataProviders, InherentData, RuntimeString};
use substrate_telemetry::{telemetry, CONSENSUS_TRACE, CONSENSUS_DEBUG,
                          CONSENSUS_WARN, CONSENSUS_INFO};
use schnorrkel::{keys::Keypair,
                 vrf::{VRFProof, VRFProofBatchable, VRFInOut, VRFOutput,
                       VRF_OUTPUT_LENGTH, VRF_PROOF_LENGTH},
                 PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use authorities::AuthoritiesApi;
use consensus_common::{self, Authorities, BlockImport, Environment, Proposer,
                       ForkChoiceStrategy, ImportBlock, BlockOrigin, Error as
                       ConsensusError};
use srml_babe::{BabeInherentData,
                timestamp::{TimestampInherentData, InherentType as
                            TimestampInherent}};
use consensus_common::{SelectChain, well_known_cache_keys};
use consensus_common::import_queue::{Verifier, BasicQueue};
use client::{block_builder::api::BlockBuilder as BlockBuilderApi,
             blockchain::ProvideCache, runtime_api::ApiExt, error::Result as
             CResult, backend::AuxStore};
use slots::{CheckedHeader, check_equivocation};
use futures::{Future, IntoFuture, future};
use tokio::timer::Timeout;
use log::{error, warn, debug, info, trace};

use slots::{SlotWorker, SlotInfo, SlotCompatible, slot_now};

/// A BABE seal.  It includes:
///
/// * The public key
/// * The VRF proof
/// * The signature
/// * The slot number
#[structural_match]
pub struct BabeSeal {
    vrf_output: VRFOutput,
    proof: VRFProof,
    signature: LocalizedSignature,
    slot_num: u64,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for BabeSeal {
    #[inline]
    fn clone(&self) -> BabeSeal {
        match *self {
            BabeSeal {
            vrf_output: ref __self_0_0,
            proof: ref __self_0_1,
            signature: ref __self_0_2,
            slot_num: ref __self_0_3 } =>
            BabeSeal{vrf_output: ::std::clone::Clone::clone(&(*__self_0_0)),
                     proof: ::std::clone::Clone::clone(&(*__self_0_1)),
                     signature: ::std::clone::Clone::clone(&(*__self_0_2)),
                     slot_num: ::std::clone::Clone::clone(&(*__self_0_3)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for BabeSeal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BabeSeal {
            vrf_output: ref __self_0_0,
            proof: ref __self_0_1,
            signature: ref __self_0_2,
            slot_num: ref __self_0_3 } => {
                let mut debug_trait_builder = f.debug_struct("BabeSeal");
                let _ =
                    debug_trait_builder.field("vrf_output", &&(*__self_0_0));
                let _ = debug_trait_builder.field("proof", &&(*__self_0_1));
                let _ =
                    debug_trait_builder.field("signature", &&(*__self_0_2));
                let _ =
                    debug_trait_builder.field("slot_num", &&(*__self_0_3));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for BabeSeal {
    #[inline]
    fn eq(&self, other: &BabeSeal) -> bool {
        match *other {
            BabeSeal {
            vrf_output: ref __self_1_0,
            proof: ref __self_1_1,
            signature: ref __self_1_2,
            slot_num: ref __self_1_3 } =>
            match *self {
                BabeSeal {
                vrf_output: ref __self_0_0,
                proof: ref __self_0_1,
                signature: ref __self_0_2,
                slot_num: ref __self_0_3 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &BabeSeal) -> bool {
        match *other {
            BabeSeal {
            vrf_output: ref __self_1_0,
            proof: ref __self_1_1,
            signature: ref __self_1_2,
            slot_num: ref __self_1_3 } =>
            match *self {
                BabeSeal {
                vrf_output: ref __self_0_0,
                proof: ref __self_0_1,
                signature: ref __self_0_2,
                slot_num: ref __self_0_3 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2) ||
                    (*__self_0_3) != (*__self_1_3),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for BabeSeal {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<VRFOutput>;
            let _: ::std::cmp::AssertParamIsEq<VRFProof>;
            let _: ::std::cmp::AssertParamIsEq<LocalizedSignature>;
            let _: ::std::cmp::AssertParamIsEq<u64>;
        }
    }
}

/// The prefix used by BABE for its VRF keys.
pub const BABE_VRF_PREFIX: &'static [u8] = b"substrate-babe-vrf";

macro_rules! babe_assert_eq(( $ a : expr , $ b : expr ) => {
                            {
                            let ref a = $ a ; let ref b = $ b ; if a != b {
                            error ! (
                            target : "babe" ,
                            "Expected {:?} to equal {:?}, but they were not" ,
                            stringify ! ( $ a ) , stringify ! ( $ b ) , ) ;
                            assert_eq ! ( a , b ) ; } } } ;);

type TmpDecode
    =
    ([u8; VRF_OUTPUT_LENGTH], [u8; VRF_PROOF_LENGTH], [u8; SIGNATURE_LENGTH],
     [u8; PUBLIC_KEY_LENGTH], u64);

impl Encode for BabeSeal {
    fn encode(&self) -> Vec<u8> {
        let tmp: TmpDecode =
            (*self.vrf_output.as_bytes(), self.proof.to_bytes(),
             self.signature.signature.0, self.signature.signer.0,
             self.slot_num);
        let encoded = parity_codec::Encode::encode(&tmp);
        if true {


            // FIXME: Once Rust has higher-kinded types, the duplication between this
            // and `super::aura::Config` can be eliminated.
            // https://github.com/paritytech/substrate/issues/2434






























            // FIXME replace the dummy empty slices with real data
            // https://github.com/paritytech/substrate/issues/2435
            // https://github.com/paritytech/substrate/issues/2436

            // we are the slot author. make a block and sign it.

            // deadline our production to approx. the end of the
            // slot


            // minor hack since we don't have access to the timestamp
            // that is actually set by the proposer.


            // sign the pre-sealed hash of the block and then
            // add it to a digest item.




            //
            // FIXME #1018 needs misbehavior types













            // we add one to allow for some small drift.
            // FIXME #1019 in the future, alter this queue to allow deferring of
            // headers

            // if the body is passed through, we need to use the runtime
            // to check that the internally-set timestamp in the inherents
            // actually matches the slot set in the seal.






            // FIXME #1019 extract authorities










            // Compute the threshold we will use.
            //
            // We already checked that authorities contains `key.public()`, so it can’t
            // be empty.  Therefore, this division is safe.


            // FIXME #2532: need to allow deprecated until refactor is done https://github.com/paritytech/substrate/issues/2532



































            // wait for all finalized on each.













            // It's ok to sign same headers.

            // But not two different headers at the same slot.

            // Different slot is ok.

            // Here we trigger pruning and save header 4.

            // This fails because header 5 is an equivocation of header 4.

            // This is ok because we pruned the corresponding header. Shows that we are pruning.
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Checking if encoding was correct"],
                                                                           &match ()
                                                                                {
                                                                                ()
                                                                                =>
                                                                                [],
                                                                            }),
                                             lvl,
                                             &("babe",
                                               "substrate_consensus_babe",
                                               "core/consensus/babe/src/lib.rs",
                                               131u32));
                }
            };
            let decoded_version =
                Self::decode(&mut &encoded[..]).expect("we just encoded this ourselves, so it is correct; qed");
            {
                let ref a = decoded_version.proof;
                let ref b = self.proof;
                if a != b {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                               lvl <= ::log::max_level() {
                            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Expected ",
                                                                                     " to equal ",
                                                                                     ", but they were not"],
                                                                                   &match (&"decoded_version.proof",
                                                                                           &"self.proof")
                                                                                        {
                                                                                        (arg0,
                                                                                         arg1)
                                                                                        =>
                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                     ::std::fmt::Debug::fmt),
                                                                                         ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                    }),
                                                     lvl,
                                                     &("babe",
                                                       "substrate_consensus_babe",
                                                       "core/consensus/babe/src/lib.rs",
                                                       134u32));
                        }
                    };
                    {
                        match (&a, &b) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    {
                                        ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`"],
                                                                                                  &match (&&*left_val,
                                                                                                          &&*right_val)
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1)
                                                                                                       =>
                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                    ::std::fmt::Debug::fmt),
                                                                                                        ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                                   }),
                                                                   &("core/consensus/babe/src/lib.rs",
                                                                     134u32,
                                                                     4u32))
                                    }
                                }
                            }
                        }
                    };
                }
            };
            {
                let ref a = decoded_version.vrf_output;
                let ref b = self.vrf_output;
                if a != b {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                               lvl <= ::log::max_level() {
                            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Expected ",
                                                                                     " to equal ",
                                                                                     ", but they were not"],
                                                                                   &match (&"decoded_version.vrf_output",
                                                                                           &"self.vrf_output")
                                                                                        {
                                                                                        (arg0,
                                                                                         arg1)
                                                                                        =>
                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                     ::std::fmt::Debug::fmt),
                                                                                         ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                    }),
                                                     lvl,
                                                     &("babe",
                                                       "substrate_consensus_babe",
                                                       "core/consensus/babe/src/lib.rs",
                                                       135u32));
                        }
                    };
                    {
                        match (&a, &b) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    {
                                        ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`"],
                                                                                                  &match (&&*left_val,
                                                                                                          &&*right_val)
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1)
                                                                                                       =>
                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                    ::std::fmt::Debug::fmt),
                                                                                                        ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                                   }),
                                                                   &("core/consensus/babe/src/lib.rs",
                                                                     135u32,
                                                                     4u32))
                                    }
                                }
                            }
                        }
                    };
                }
            };
            {
                let ref a = decoded_version.signature.signature;
                let ref b = self.signature.signature;
                if a != b {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                               lvl <= ::log::max_level() {
                            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Expected ",
                                                                                     " to equal ",
                                                                                     ", but they were not"],
                                                                                   &match (&"decoded_version.signature.signature",
                                                                                           &"self.signature.signature")
                                                                                        {
                                                                                        (arg0,
                                                                                         arg1)
                                                                                        =>
                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                     ::std::fmt::Debug::fmt),
                                                                                         ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                    }),
                                                     lvl,
                                                     &("babe",
                                                       "substrate_consensus_babe",
                                                       "core/consensus/babe/src/lib.rs",
                                                       136u32));
                        }
                    };
                    {
                        match (&a, &b) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    {
                                        ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`"],
                                                                                                  &match (&&*left_val,
                                                                                                          &&*right_val)
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1)
                                                                                                       =>
                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                    ::std::fmt::Debug::fmt),
                                                                                                        ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                                   }),
                                                                   &("core/consensus/babe/src/lib.rs",
                                                                     136u32,
                                                                     4u32))
                                    }
                                }
                            }
                        }
                    };
                }
            };
            {
                let ref a = decoded_version.signature.signer;
                let ref b = self.signature.signer;
                if a != b {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                               lvl <= ::log::max_level() {
                            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Expected ",
                                                                                     " to equal ",
                                                                                     ", but they were not"],
                                                                                   &match (&"decoded_version.signature.signer",
                                                                                           &"self.signature.signer")
                                                                                        {
                                                                                        (arg0,
                                                                                         arg1)
                                                                                        =>
                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                     ::std::fmt::Debug::fmt),
                                                                                         ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                    }),
                                                     lvl,
                                                     &("babe",
                                                       "substrate_consensus_babe",
                                                       "core/consensus/babe/src/lib.rs",
                                                       137u32));
                        }
                    };
                    {
                        match (&a, &b) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    {
                                        ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`"],
                                                                                                  &match (&&*left_val,
                                                                                                          &&*right_val)
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1)
                                                                                                       =>
                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                    ::std::fmt::Debug::fmt),
                                                                                                        ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                                   }),
                                                                   &("core/consensus/babe/src/lib.rs",
                                                                     137u32,
                                                                     4u32))
                                    }
                                }
                            }
                        }
                    };
                }
            };
            {
                let ref a = decoded_version.slot_num;
                let ref b = self.slot_num;
                if a != b {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                               lvl <= ::log::max_level() {
                            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Expected ",
                                                                                     " to equal ",
                                                                                     ", but they were not"],
                                                                                   &match (&"decoded_version.slot_num",
                                                                                           &"self.slot_num")
                                                                                        {
                                                                                        (arg0,
                                                                                         arg1)
                                                                                        =>
                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                     ::std::fmt::Debug::fmt),
                                                                                         ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                    }),
                                                     lvl,
                                                     &("babe",
                                                       "substrate_consensus_babe",
                                                       "core/consensus/babe/src/lib.rs",
                                                       138u32));
                        }
                    };
                    {
                        match (&a, &b) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    {
                                        ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                    "`,\n right: `",
                                                                                                    "`"],
                                                                                                  &match (&&*left_val,
                                                                                                          &&*right_val)
                                                                                                       {
                                                                                                       (arg0,
                                                                                                        arg1)
                                                                                                       =>
                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                    ::std::fmt::Debug::fmt),
                                                                                                        ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                                   }),
                                                                   &("core/consensus/babe/src/lib.rs",
                                                                     138u32,
                                                                     4u32))
                                    }
                                }
                            }
                        }
                    };
                }
            };
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Encoding was correct"],
                                                                           &match ()
                                                                                {
                                                                                ()
                                                                                =>
                                                                                [],
                                                                            }),
                                             lvl,
                                             &("babe",
                                               "substrate_consensus_babe",
                                               "core/consensus/babe/src/lib.rs",
                                               139u32));
                }
            }
        }
        encoded
    }
}
impl Decode for BabeSeal {
    fn decode<R: Input>(i: &mut R) -> Option<Self> {
        let (output, proof, sig, public_key, slot_num): TmpDecode =
            Decode::decode(i)?;
        Some(BabeSeal{proof: VRFProof::from_bytes(&proof).ok()?,
                      vrf_output: VRFOutput::from_bytes(&output).ok()?,
                      signature:
                          LocalizedSignature{signature: Signature(sig),
                                             signer: Public(public_key),},
                      slot_num,})
    }
}
/// A slot duration. Create with `get_or_compute`.
pub struct Config(slots::SlotDuration<BabeConfiguration>);
impl Config {
    /// Either fetch the slot duration from disk or compute it from the genesis
    /// state.
    pub fn get_or_compute<B: Block, C>(client: &C) -> CResult<Self> where
     C: AuxStore, C: ProvideRuntimeApi, C::Api: BabeApi<B> {
        {
            let lvl = ::log::Level::Trace;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Getting slot duration"],
                                                                       &match ()
                                                                            {
                                                                            ()
                                                                            =>
                                                                            [],
                                                                        }),
                                         lvl,
                                         &("babe", "substrate_consensus_babe",
                                           "core/consensus/babe/src/lib.rs",
                                           173u32));
            }
        };
        match slots::SlotDuration::get_or_compute(client,
                                                  |a, b|
                                                      a.startup_data(b)).map(Self)
            {
            Ok(s) => Ok(s),
            Err(s) => {
                {
                    let lvl = ::log::Level::Warn;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Failed to get slot duration"],
                                                                               &match ()
                                                                                    {
                                                                                    ()
                                                                                    =>
                                                                                    [],
                                                                                }),
                                                 lvl,
                                                 &("babe",
                                                   "substrate_consensus_babe",
                                                   "core/consensus/babe/src/lib.rs",
                                                   177u32));
                    }
                };
                Err(s)
            }
        }
    }
    /// Get the slot duration in milliseconds.
    pub fn get(&self) -> u64 { self.0.slot_duration }
    /// Retrieve the threshold for BABE
    pub fn threshold(&self) -> u64 { self.0.threshold }
}
fn inherent_to_common_error(err: RuntimeString) -> consensus_common::Error {
    consensus_common::ErrorKind::InherentData(err.into()).into()
}
/// A digest item which is usable with BABE consensus.
pub trait CompatibleDigestItem: Sized {
    /// Construct a digest item which contains a slot number and a signature
    /// on the hash.
    fn babe_seal(signature: BabeSeal)
    -> Self;
    /// If this item is an Babe seal, return the slot number and signature.
    fn as_babe_seal(&self)
    -> Option<BabeSeal>;
}
impl <T, Hash> CompatibleDigestItem for generic::DigestItem<Hash, Public, T>
 where T: Debug, Hash: Debug {
    /// Construct a digest item which contains a slot number and a signature
    /// on the hash.
    fn babe_seal(signature: BabeSeal) -> Self {
        generic::DigestItem::Consensus(BABE_ENGINE_ID, signature.encode())
    }
    /// If this item is an BABE seal, return the slot number and signature.
    fn as_babe_seal(&self) -> Option<BabeSeal> {
        match self {
            generic::DigestItem::Consensus(BABE_ENGINE_ID, seal) => {
                match Decode::decode(&mut &seal[..]) {
                    s@Some(_) => s,
                    s@None => {
                        {
                            let lvl = ::log::Level::Info;
                            if lvl <= ::log::STATIC_MAX_LEVEL &&
                                   lvl <= ::log::max_level() {
                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Failed to decode "],
                                                                                       &match (&seal,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Debug::fmt)],
                                                                                        }),
                                                         lvl,
                                                         &("babe",
                                                           "substrate_consensus_babe",
                                                           "core/consensus/babe/src/lib.rs",
                                                           224u32));
                            }
                        };
                        s
                    }
                }
            }
            _ => {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Invalid consensus: ",
                                                                                 "!"],
                                                                               &match (&self,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Debug::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("babe",
                                                   "substrate_consensus_babe",
                                                   "core/consensus/babe/src/lib.rs",
                                                   230u32));
                    }
                };
                None
            }
        }
    }
}
struct BabeSlotCompatible;
impl SlotCompatible for BabeSlotCompatible {
    fn extract_timestamp_and_slot(data: &InherentData)
     -> Result<(TimestampInherent, u64), consensus_common::Error> {
        {
            let lvl = ::log::Level::Trace;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["extract timestamp"],
                                                                       &match ()
                                                                            {
                                                                            ()
                                                                            =>
                                                                            [],
                                                                        }),
                                         lvl,
                                         &("babe", "substrate_consensus_babe",
                                           "core/consensus/babe/src/lib.rs",
                                           243u32));
            }
        };
        data.timestamp_inherent_data().and_then(|t|
                                                    data.babe_inherent_data().map(|a|
                                                                                      (t,
                                                                                       a))).map_err(slots::inherent_to_common_error)
    }
}
/// Parameters for BABE.
pub struct BabeParams<C, E, I, SO, SC, OnExit> {
    /// The configuration for BABE.  Includes the slot duration, threshold, and
    /// other parameters.
    pub config: Config,
    /// The key of the node we are running on.
    pub local_key: Arc<sr25519::Pair>,
    /// The client to use
    pub client: Arc<C>,
    /// The SelectChain Strategy
    pub select_chain: SC,
    /// A block importer
    pub block_import: Arc<I>,
    /// The environment
    pub env: Arc<E>,
    /// A sync oracle
    pub sync_oracle: SO,
    /// Exit callback.
    pub on_exit: OnExit,
    /// Providers for inherent data.
    pub inherent_data_providers: InherentDataProviders,
    /// Force authoring of blocks even if we are offline
    pub force_authoring: bool,
}
/// Start the babe worker. The returned future should be run in a tokio runtime.
pub fn start_babe<B, C, E, I, SO, SC, Error,
                  OnExit>(BabeParams {
                              config,
                              local_key,
                              client,
                              select_chain,
                              block_import,
                              env,
                              sync_oracle,
                              on_exit,
                              inherent_data_providers,
                              force_authoring }:
                              BabeParams<C, E, I, SO, SC, OnExit>)
 -> Result<impl Future<Item = (), Error = ()>, consensus_common::Error> where
 B: Block, C: ProvideRuntimeApi + ProvideCache<B>, C::Api: AuthoritiesApi<B>,
 E: Environment<B, Error = Error>, E::Proposer: Proposer<B, Error = Error>,
 <<E::Proposer as Proposer<B>>::Create as IntoFuture>::Future: Send + 'static,
 I: BlockImport<B> + Send + Sync + 'static, SO: SyncOracle + Send + Sync +
 Clone, SC: SelectChain<B>, DigestItemFor<B>: CompatibleDigestItem +
 DigestItem<AuthorityId = Public>, Error: ::std::error::Error + Send +
 From<::consensus_common::Error> + From<I::Error> + 'static,
 OnExit: Future<Item = (), Error = ()> {
    let worker =
        BabeWorker{client: client.clone(),
                   block_import,
                   env,
                   local_key,
                   inherent_data_providers: inherent_data_providers.clone(),
                   sync_oracle: sync_oracle.clone(),
                   force_authoring,
                   threshold: config.threshold(),};
    slots::start_slot_worker::<_, _, _, _, _, BabeSlotCompatible,
                               _>(config.0, select_chain, Arc::new(worker),
                                  sync_oracle, on_exit,
                                  inherent_data_providers)
}
struct BabeWorker<C, E, I, SO> {
    client: Arc<C>,
    block_import: Arc<I>,
    env: Arc<E>,
    local_key: Arc<sr25519::Pair>,
    sync_oracle: SO,
    inherent_data_providers: InherentDataProviders,
    force_authoring: bool,
    threshold: u64,
}
impl <B: Block, C, E, I, Error, SO> SlotWorker<B> for BabeWorker<C, E, I, SO>
 where C: ProvideRuntimeApi + ProvideCache<B>, C::Api: AuthoritiesApi<B>,
 E: Environment<B, Error = Error>, E::Proposer: Proposer<B, Error = Error>,
 <<E::Proposer as Proposer<B>>::Create as IntoFuture>::Future: Send + 'static,
 I: BlockImport<B> + Send + Sync + 'static, SO: SyncOracle + Send + Clone,
 DigestItemFor<B>: CompatibleDigestItem + DigestItem<AuthorityId = Public>,
 Error: std::error::Error + Send + From<::consensus_common::Error> +
 From<I::Error> + 'static {
    type
    OnSlot
    =
    Box<Future<Item = (), Error = consensus_common::Error> + Send>;
    fn on_start(&self, slot_duration: u64)
     -> Result<(), consensus_common::Error> {
        register_babe_inherent_data_provider(&self.inherent_data_providers,
                                             slot_duration)
    }
    fn on_slot(&self, chain_head: B::Header, slot_info: SlotInfo)
     -> Self::OnSlot {
        let pair = self.local_key.clone();
        let ref client = self.client;
        let block_import = self.block_import.clone();
        let ref env = self.env;
        let (timestamp, slot_num, slot_duration) =
            (slot_info.timestamp, slot_info.number, slot_info.duration);
        let authorities =
            match authorities(client.as_ref(),
                              &BlockId::Hash(chain_head.hash())) {
                Ok(authorities) => authorities,
                Err(e) => {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL &&
                               lvl <= ::log::max_level() {
                            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Unable to fetch authorities at block ",
                                                                                     ": "],
                                                                                   &match (&chain_head.hash(),
                                                                                           &e)
                                                                                        {
                                                                                        (arg0,
                                                                                         arg1)
                                                                                        =>
                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                     ::std::fmt::Debug::fmt),
                                                                                         ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                    }),
                                                     lvl,
                                                     &("babe",
                                                       "substrate_consensus_babe",
                                                       "core/consensus/babe/src/lib.rs",
                                                       381u32));
                        }
                    };
                    ::substrate_telemetry::with_logger(|l|
                                                           {
                                                               if ::slog::Level::Info.as_usize()
                                                                      <=
                                                                      ::slog::__slog_static_max_level().as_usize()
                                                                  {
                                                                   l.log(&{
                                                                              static RS:
                                                                                     ::slog::RecordStatic<'static>
                                                                                     =
                                                                                  {
                                                                                      static LOC:
                                                                                             ::slog::RecordLocation
                                                                                             =
                                                                                          ::slog::RecordLocation{file:
                                                                                                                     "core/consensus/babe/src/lib.rs",
                                                                                                                 line:
                                                                                                                     387u32,
                                                                                                                 column:
                                                                                                                     5u32,
                                                                                                                 function:
                                                                                                                     "",
                                                                                                                 module:
                                                                                                                     "substrate_consensus_babe",};
                                                                                      ::slog::RecordStatic{location:
                                                                                                               &LOC,
                                                                                                           level:
                                                                                                               ::slog::Level::Info,
                                                                                                           tag:
                                                                                                               CONSENSUS_WARN,}
                                                                                  };
                                                                              ::slog::Record::new(&RS,
                                                                                                  &::std::fmt::Arguments::new_v1(&["babe.unable_fetching_authorities"],
                                                                                                                                 &match ()
                                                                                                                                      {
                                                                                                                                      ()
                                                                                                                                      =>
                                                                                                                                      [],
                                                                                                                                  }),
                                                                                                  ::slog::BorrowedKV(&(::slog::SingleKV::from(("err",
                                                                                                                                               ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                             &match (&e,)
                                                                                                                                                                                  {
                                                                                                                                                                                  (arg0,)
                                                                                                                                                                                  =>
                                                                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                                                                              }))),
                                                                                                                       (::slog::SingleKV::from(("slot",
                                                                                                                                                ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                              &match (&chain_head.hash(),)
                                                                                                                                                                                   {
                                                                                                                                                                                   (arg0,)
                                                                                                                                                                                   =>
                                                                                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                ::std::fmt::Debug::fmt)],
                                                                                                                                                                               }))),
                                                                                                                        ()))))
                                                                          })
                                                               }
                                                           });
                    return Box::new(future::ok(()));
                }
            };
        if !self.force_authoring && self.sync_oracle.is_offline() &&
               authorities.len() > 1 {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Skipping proposal slot. Waiting for the network."],
                                                                           &match ()
                                                                                {
                                                                                ()
                                                                                =>
                                                                                [],
                                                                            }),
                                             lvl,
                                             &("babe",
                                               "substrate_consensus_babe",
                                               "core/consensus/babe/src/lib.rs",
                                               395u32));
                }
            };
            ::substrate_telemetry::with_logger(|l|
                                                   {
                                                       if ::slog::Level::Info.as_usize()
                                                              <=
                                                              ::slog::__slog_static_max_level().as_usize()
                                                          {
                                                           l.log(&{
                                                                      static RS:
                                                                             ::slog::RecordStatic<'static>
                                                                             =
                                                                          {
                                                                              static LOC:
                                                                                     ::slog::RecordLocation
                                                                                     =
                                                                                  ::slog::RecordLocation{file:
                                                                                                             "core/consensus/babe/src/lib.rs",
                                                                                                         line:
                                                                                                             396u32,
                                                                                                         column:
                                                                                                             4u32,
                                                                                                         function:
                                                                                                             "",
                                                                                                         module:
                                                                                                             "substrate_consensus_babe",};
                                                                              ::slog::RecordStatic{location:
                                                                                                       &LOC,
                                                                                                   level:
                                                                                                       ::slog::Level::Info,
                                                                                                   tag:
                                                                                                       CONSENSUS_DEBUG,}
                                                                          };
                                                                      ::slog::Record::new(&RS,
                                                                                          &::std::fmt::Arguments::new_v1(&["babe.skipping_proposal_slot"],
                                                                                                                         &match ()
                                                                                                                              {
                                                                                                                              ()
                                                                                                                              =>
                                                                                                                              [],
                                                                                                                          }),
                                                                                          ::slog::BorrowedKV(&(::slog::SingleKV::from(("authorities_len",
                                                                                                                                       authorities.len())),
                                                                                                               ())))
                                                                  })
                                                       }
                                                   });
            return Box::new(future::ok(()));
        }
        let authoring_result =
            if let Some((inout, proof, _batchable_proof)) =
                   claim_slot(&[0u8; 0], slot_info.number, &[0u8; 0], 0,
                              &authorities, &pair, self.threshold) {
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Starting authorship at slot ",
                                                                                 "; timestamp = "],
                                                                               &match (&slot_num,
                                                                                       &timestamp)
                                                                                    {
                                                                                    (arg0,
                                                                                     arg1)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Display::fmt),
                                                                                     ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("babe",
                                                   "substrate_consensus_babe",
                                                   "core/consensus/babe/src/lib.rs",
                                                   414u32));
                    }
                };
                ::substrate_telemetry::with_logger(|l|
                                                       {
                                                           if ::slog::Level::Info.as_usize()
                                                                  <=
                                                                  ::slog::__slog_static_max_level().as_usize()
                                                              {
                                                               l.log(&{
                                                                          static RS:
                                                                                 ::slog::RecordStatic<'static>
                                                                                 =
                                                                              {
                                                                                  static LOC:
                                                                                         ::slog::RecordLocation
                                                                                         =
                                                                                      ::slog::RecordLocation{file:
                                                                                                                 "core/consensus/babe/src/lib.rs",
                                                                                                             line:
                                                                                                                 419u32,
                                                                                                             column:
                                                                                                                 4u32,
                                                                                                             function:
                                                                                                                 "",
                                                                                                             module:
                                                                                                                 "substrate_consensus_babe",};
                                                                                  ::slog::RecordStatic{location:
                                                                                                           &LOC,
                                                                                                       level:
                                                                                                           ::slog::Level::Info,
                                                                                                       tag:
                                                                                                           CONSENSUS_DEBUG,}
                                                                              };
                                                                          ::slog::Record::new(&RS,
                                                                                              &::std::fmt::Arguments::new_v1(&["babe.starting_authorship"],
                                                                                                                             &match ()
                                                                                                                                  {
                                                                                                                                  ()
                                                                                                                                  =>
                                                                                                                                  [],
                                                                                                                              }),
                                                                                              ::slog::BorrowedKV(&(::slog::SingleKV::from(("timestamp",
                                                                                                                                           timestamp)),
                                                                                                                   (::slog::SingleKV::from(("slot_num",
                                                                                                                                            slot_num)),
                                                                                                                    ()))))
                                                                      })
                                                           }
                                                       });
                let proposer =
                    match env.init(&chain_head, &authorities) {
                        Ok(p) => p,
                        Err(e) => {
                            {
                                let lvl = ::log::Level::Warn;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Unable to author block in slot ",
                                                                                             ": "],
                                                                                           &match (&slot_num,
                                                                                                   &e)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Debug::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("babe",
                                                               "substrate_consensus_babe",
                                                               "core/consensus/babe/src/lib.rs",
                                                               427u32));
                                }
                            };
                            ::substrate_telemetry::with_logger(|l|
                                                                   {
                                                                       if ::slog::Level::Info.as_usize()
                                                                              <=
                                                                              ::slog::__slog_static_max_level().as_usize()
                                                                          {
                                                                           l.log(&{
                                                                                      static RS:
                                                                                             ::slog::RecordStatic<'static>
                                                                                             =
                                                                                          {
                                                                                              static LOC:
                                                                                                     ::slog::RecordLocation
                                                                                                     =
                                                                                                  ::slog::RecordLocation{file:
                                                                                                                             "core/consensus/babe/src/lib.rs",
                                                                                                                         line:
                                                                                                                             428u32,
                                                                                                                         column:
                                                                                                                             6u32,
                                                                                                                         function:
                                                                                                                             "",
                                                                                                                         module:
                                                                                                                             "substrate_consensus_babe",};
                                                                                              ::slog::RecordStatic{location:
                                                                                                                       &LOC,
                                                                                                                   level:
                                                                                                                       ::slog::Level::Info,
                                                                                                                   tag:
                                                                                                                       CONSENSUS_WARN,}
                                                                                          };
                                                                                      ::slog::Record::new(&RS,
                                                                                                          &::std::fmt::Arguments::new_v1(&["babe.unable_authoring_block"],
                                                                                                                                         &match ()
                                                                                                                                              {
                                                                                                                                              ()
                                                                                                                                              =>
                                                                                                                                              [],
                                                                                                                                          }),
                                                                                                          ::slog::BorrowedKV(&(::slog::SingleKV::from(("err",
                                                                                                                                                       ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                     &match (&e,)
                                                                                                                                                                                          {
                                                                                                                                                                                          (arg0,)
                                                                                                                                                                                          =>
                                                                                                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                       ::std::fmt::Debug::fmt)],
                                                                                                                                                                                      }))),
                                                                                                                               (::slog::SingleKV::from(("slot",
                                                                                                                                                        slot_num)),
                                                                                                                                ()))))
                                                                                  })
                                                                       }
                                                                   });
                            return Box::new(future::ok(()))
                        }
                    };
                let remaining_duration = slot_info.remaining_duration();
                (Timeout::new(proposer.propose(slot_info.inherent_data,
                                               remaining_duration).into_future(),
                              remaining_duration), inout.to_output(), proof)
            } else { return Box::new(future::ok(())); };
        let (proposal_work, vrf_output, proof) = authoring_result;
        Box::new(proposal_work.map(move |b|
                                       {
                                           let slot_after_building =
                                               slot_now(slot_duration);
                                           if slot_after_building !=
                                                  Some(slot_num) {
                                               {
                                                   let lvl =
                                                       ::log::Level::Info;
                                                   if lvl <=
                                                          ::log::STATIC_MAX_LEVEL
                                                          &&
                                                          lvl <=
                                                              ::log::max_level()
                                                      {
                                                       ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Discarding proposal for slot ",
                                                                                                                "; block production took too long"],
                                                                                                              &match (&slot_num,)
                                                                                                                   {
                                                                                                                   (arg0,)
                                                                                                                   =>
                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                ::std::fmt::Display::fmt)],
                                                                                                               }),
                                                                                lvl,
                                                                                &("babe",
                                                                                  "substrate_consensus_babe",
                                                                                  "core/consensus/babe/src/lib.rs",
                                                                                  460u32));
                                                   }
                                               };
                                               ::substrate_telemetry::with_logger(|l|
                                                                                      {
                                                                                          if ::slog::Level::Info.as_usize()
                                                                                                 <=
                                                                                                 ::slog::__slog_static_max_level().as_usize()
                                                                                             {
                                                                                              l.log(&{
                                                                                                         static RS:
                                                                                                                ::slog::RecordStatic<'static>
                                                                                                                =
                                                                                                             {
                                                                                                                 static LOC:
                                                                                                                        ::slog::RecordLocation
                                                                                                                        =
                                                                                                                     ::slog::RecordLocation{file:
                                                                                                                                                "core/consensus/babe/src/lib.rs",
                                                                                                                                            line:
                                                                                                                                                465u32,
                                                                                                                                            column:
                                                                                                                                                7u32,
                                                                                                                                            function:
                                                                                                                                                "",
                                                                                                                                            module:
                                                                                                                                                "substrate_consensus_babe",};
                                                                                                                 ::slog::RecordStatic{location:
                                                                                                                                          &LOC,
                                                                                                                                      level:
                                                                                                                                          ::slog::Level::Info,
                                                                                                                                      tag:
                                                                                                                                          CONSENSUS_INFO,}
                                                                                                             };
                                                                                                         ::slog::Record::new(&RS,
                                                                                                                             &::std::fmt::Arguments::new_v1(&["babe.discarding_proposal_took_too_long"],
                                                                                                                                                            &match ()
                                                                                                                                                                 {
                                                                                                                                                                 ()
                                                                                                                                                                 =>
                                                                                                                                                                 [],
                                                                                                                                                             }),
                                                                                                                             ::slog::BorrowedKV(&(::slog::SingleKV::from(("slot",
                                                                                                                                                                          slot_num)),
                                                                                                                                                  ())))
                                                                                                     })
                                                                                          }
                                                                                      });
                                               return
                                           }
                                           let (header, body) =
                                               b.deconstruct();
                                           let header_num =
                                               header.number().clone();
                                           let pre_hash = header.hash();
                                           let parent_hash =
                                               header.parent_hash().clone();
                                           let to_sign =
                                               (slot_num, pre_hash,
                                                proof.to_bytes()).encode();
                                           let signature =
                                               pair.sign(&to_sign[..]);
                                           let item =
                                               <DigestItemFor<B> as
                                                   CompatibleDigestItem>::babe_seal(BabeSeal{proof,
                                                                                             signature:
                                                                                                 LocalizedSignature{signature,
                                                                                                                    signer:
                                                                                                                        pair.public(),},
                                                                                             slot_num,
                                                                                             vrf_output,});
                                           let import_block: ImportBlock<B> =
                                               ImportBlock{origin:
                                                               BlockOrigin::Own,
                                                           header,
                                                           justification:
                                                               None,
                                                           post_digests:
                                                               <[_]>::into_vec(box
                                                                                   [item]),
                                                           body: Some(body),
                                                           finalized: false,
                                                           auxiliary:
                                                               Vec::new(),
                                                           fork_choice:
                                                               ForkChoiceStrategy::LongestChain,};
                                           {
                                               let lvl = ::log::Level::Info;
                                               if lvl <=
                                                      ::log::STATIC_MAX_LEVEL
                                                      &&
                                                      lvl <=
                                                          ::log::max_level() {
                                                   ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Pre-sealed block for proposal at ",
                                                                                                            ". Hash now ",
                                                                                                            ", previously ",
                                                                                                            "."],
                                                                                                          &match (&header_num,
                                                                                                                  &import_block.post_header().hash(),
                                                                                                                  &pre_hash)
                                                                                                               {
                                                                                                               (arg0,
                                                                                                                arg1,
                                                                                                                arg2)
                                                                                                               =>
                                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                            ::std::fmt::Display::fmt),
                                                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                            ::std::fmt::Debug::fmt),
                                                                                                                ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                                            ::std::fmt::Debug::fmt)],
                                                                                                           }),
                                                                            lvl,
                                                                            &("babe",
                                                                              "substrate_consensus_babe",
                                                                              "core/consensus/babe/src/lib.rs",
                                                                              501u32));
                                               }
                                           };
                                           ::substrate_telemetry::with_logger(|l|
                                                                                  {
                                                                                      if ::slog::Level::Info.as_usize()
                                                                                             <=
                                                                                             ::slog::__slog_static_max_level().as_usize()
                                                                                         {
                                                                                          l.log(&{
                                                                                                     static RS:
                                                                                                            ::slog::RecordStatic<'static>
                                                                                                            =
                                                                                                         {
                                                                                                             static LOC:
                                                                                                                    ::slog::RecordLocation
                                                                                                                    =
                                                                                                                 ::slog::RecordLocation{file:
                                                                                                                                            "core/consensus/babe/src/lib.rs",
                                                                                                                                        line:
                                                                                                                                            507u32,
                                                                                                                                        column:
                                                                                                                                            6u32,
                                                                                                                                        function:
                                                                                                                                            "",
                                                                                                                                        module:
                                                                                                                                            "substrate_consensus_babe",};
                                                                                                             ::slog::RecordStatic{location:
                                                                                                                                      &LOC,
                                                                                                                                  level:
                                                                                                                                      ::slog::Level::Info,
                                                                                                                                  tag:
                                                                                                                                      CONSENSUS_INFO,}
                                                                                                         };
                                                                                                     ::slog::Record::new(&RS,
                                                                                                                         &::std::fmt::Arguments::new_v1(&["babe.pre_sealed_block"],
                                                                                                                                                        &match ()
                                                                                                                                                             {
                                                                                                                                                             ()
                                                                                                                                                             =>
                                                                                                                                                             [],
                                                                                                                                                         }),
                                                                                                                         ::slog::BorrowedKV(&(::slog::SingleKV::from(("hash_previously",
                                                                                                                                                                      ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                    &match (&pre_hash,)
                                                                                                                                                                                                         {
                                                                                                                                                                                                         (arg0,)
                                                                                                                                                                                                         =>
                                                                                                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                      ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                     }))),
                                                                                                                                              (::slog::SingleKV::from(("hash_now",
                                                                                                                                                                       ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                     &match (&import_block.post_header().hash(),)
                                                                                                                                                                                                          {
                                                                                                                                                                                                          (arg0,)
                                                                                                                                                                                                          =>
                                                                                                                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                       ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                      }))),
                                                                                                                                               (::slog::SingleKV::from(("header_num",
                                                                                                                                                                        ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                      &match (&header_num,)
                                                                                                                                                                                                           {
                                                                                                                                                                                                           (arg0,)
                                                                                                                                                                                                           =>
                                                                                                                                                                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                        ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                       }))),
                                                                                                                                                ())))))
                                                                                                 })
                                                                                      }
                                                                                  });
                                           if let Err(e) =
                                                  block_import.import_block(import_block,
                                                                            Default::default())
                                                  {
                                               {
                                                   let lvl =
                                                       ::log::Level::Warn;
                                                   if lvl <=
                                                          ::log::STATIC_MAX_LEVEL
                                                          &&
                                                          lvl <=
                                                              ::log::max_level()
                                                      {
                                                       ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Error with block built on ",
                                                                                                                ": "],
                                                                                                              &match (&parent_hash,
                                                                                                                      &e)
                                                                                                                   {
                                                                                                                   (arg0,
                                                                                                                    arg1)
                                                                                                                   =>
                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                ::std::fmt::Debug::fmt),
                                                                                                                    ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                ::std::fmt::Debug::fmt)],
                                                                                                               }),
                                                                                lvl,
                                                                                &("babe",
                                                                                  "substrate_consensus_babe",
                                                                                  "core/consensus/babe/src/lib.rs",
                                                                                  514u32));
                                                   }
                                               };
                                               ::substrate_telemetry::with_logger(|l|
                                                                                      {
                                                                                          if ::slog::Level::Info.as_usize()
                                                                                                 <=
                                                                                                 ::slog::__slog_static_max_level().as_usize()
                                                                                             {
                                                                                              l.log(&{
                                                                                                         static RS:
                                                                                                                ::slog::RecordStatic<'static>
                                                                                                                =
                                                                                                             {
                                                                                                                 static LOC:
                                                                                                                        ::slog::RecordLocation
                                                                                                                        =
                                                                                                                     ::slog::RecordLocation{file:
                                                                                                                                                "core/consensus/babe/src/lib.rs",
                                                                                                                                            line:
                                                                                                                                                516u32,
                                                                                                                                            column:
                                                                                                                                                7u32,
                                                                                                                                            function:
                                                                                                                                                "",
                                                                                                                                            module:
                                                                                                                                                "substrate_consensus_babe",};
                                                                                                                 ::slog::RecordStatic{location:
                                                                                                                                          &LOC,
                                                                                                                                      level:
                                                                                                                                          ::slog::Level::Info,
                                                                                                                                      tag:
                                                                                                                                          CONSENSUS_WARN,}
                                                                                                             };
                                                                                                         ::slog::Record::new(&RS,
                                                                                                                             &::std::fmt::Arguments::new_v1(&["babe.err_with_block_built_on"],
                                                                                                                                                            &match ()
                                                                                                                                                                 {
                                                                                                                                                                 ()
                                                                                                                                                                 =>
                                                                                                                                                                 [],
                                                                                                                                                             }),
                                                                                                                             ::slog::BorrowedKV(&(::slog::SingleKV::from(("err",
                                                                                                                                                                          ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                        &match (&e,)
                                                                                                                                                                                                             {
                                                                                                                                                                                                             (arg0,)
                                                                                                                                                                                                             =>
                                                                                                                                                                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                          ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                         }))),
                                                                                                                                                  (::slog::SingleKV::from(("hash",
                                                                                                                                                                           ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                         &match (&parent_hash,)
                                                                                                                                                                                                              {
                                                                                                                                                                                                              (arg0,)
                                                                                                                                                                                                              =>
                                                                                                                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                           ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                          }))),
                                                                                                                                                   ()))))
                                                                                                     })
                                                                                          }
                                                                                      });
                                           }
                                       }).map_err(|e|
                                                      {
                                                          {
                                                              let lvl =
                                                                  ::log::Level::Warn;
                                                              if lvl <=
                                                                     ::log::STATIC_MAX_LEVEL
                                                                     &&
                                                                     lvl <=
                                                                         ::log::max_level()
                                                                 {
                                                                  ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Client import failed: "],
                                                                                                                         &match (&e,)
                                                                                                                              {
                                                                                                                              (arg0,)
                                                                                                                              =>
                                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                           ::std::fmt::Debug::fmt)],
                                                                                                                          }),
                                                                                           lvl,
                                                                                           &("substrate_consensus_babe",
                                                                                             "substrate_consensus_babe",
                                                                                             "core/consensus/babe/src/lib.rs",
                                                                                             522u32));
                                                              }
                                                          };
                                                          consensus_common::ErrorKind::ClientImport(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                       &match (&e,)
                                                                                                                                                            {
                                                                                                                                                            (arg0,)
                                                                                                                                                            =>
                                                                                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                         ::std::fmt::Debug::fmt)],
                                                                                                                                                        }))).into()
                                                      }))
    }
}
/// check a header has been signed by the right key. If the slot is too far in
/// the future, an error will be returned. If successful, returns the pre-header
/// and the digest item containing the seal.
///
/// This digest item will always return `Some` when used with `as_babe_seal`.
fn check_header<B: Block + Sized,
                C: AuxStore>(client: &Arc<C>, slot_now: u64,
                             mut header: B::Header, hash: B::Hash,
                             authorities: &[Public], threshold: u64)
 -> Result<CheckedHeader<B::Header, DigestItemFor<B>>, String> where
 DigestItemFor<B>: CompatibleDigestItem {
    {
        let lvl = ::log::Level::Trace;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Checking header"],
                                                                   &match () {
                                                                        () =>
                                                                        [],
                                                                    }), lvl,
                                     &("babe", "substrate_consensus_babe",
                                       "core/consensus/babe/src/lib.rs",
                                       546u32));
        }
    };
    let digest_item =
        match header.digest_mut().pop() {
            Some(x) => x,
            None =>
            return Err(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Header ",
                                                                            " is unsealed"],
                                                                          &match (&hash,)
                                                                               {
                                                                               (arg0,)
                                                                               =>
                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                            ::std::fmt::Debug::fmt)],
                                                                           }))),
        };
    let BabeSeal {
            slot_num,
            signature: LocalizedSignature { signer, signature },
            proof,
            vrf_output } =
        digest_item.as_babe_seal().ok_or_else(||
                                                  {
                                                      {
                                                          let lvl =
                                                              ::log::Level::Debug;
                                                          if lvl <=
                                                                 ::log::STATIC_MAX_LEVEL
                                                                 &&
                                                                 lvl <=
                                                                     ::log::max_level()
                                                             {
                                                              ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Header ",
                                                                                                                       " is unsealed"],
                                                                                                                     &match (&hash,)
                                                                                                                          {
                                                                                                                          (arg0,)
                                                                                                                          =>
                                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                       ::std::fmt::Debug::fmt)],
                                                                                                                      }),
                                                                                       lvl,
                                                                                       &("babe",
                                                                                         "substrate_consensus_babe",
                                                                                         "core/consensus/babe/src/lib.rs",
                                                                                         558u32));
                                                          }
                                                      };
                                                      ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Header ",
                                                                                                           " is unsealed"],
                                                                                                         &match (&hash,)
                                                                                                              {
                                                                                                              (arg0,)
                                                                                                              =>
                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                           ::std::fmt::Debug::fmt)],
                                                                                                          }))
                                                  })?;
    if slot_num > slot_now {
        header.digest_mut().push(digest_item);
        Ok(CheckedHeader::Deferred(header, slot_num))
    } else if !authorities.contains(&signer) {
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Slot Author not found"],
                                                                       &match ()
                                                                            {
                                                                            ()
                                                                            =>
                                                                            [],
                                                                        }),
                                         lvl,
                                         &("babe", "substrate_consensus_babe",
                                           "core/consensus/babe/src/lib.rs",
                                           566u32));
            }
        };
        Err("Slot Author not found".to_string())
    } else {
        let pre_hash = header.hash();
        let to_sign = (slot_num, pre_hash, proof.to_bytes()).encode();
        if sr25519::Pair::verify(&signature, &to_sign[..], &signer) {
            let (inout, _batchable_proof) =
                {
                    let transcript =
                        make_transcript(Default::default(), slot_num,
                                        Default::default(), 0);
                    schnorrkel::PublicKey::from_bytes(signer.as_slice()).and_then(|p|
                                                                                      {
                                                                                          p.vrf_verify(transcript,
                                                                                                       &vrf_output,
                                                                                                       &proof)
                                                                                      }).map_err(|s|
                                                                                                     {
                                                                                                         {
                                                                                                             let lvl =
                                                                                                                 ::log::Level::Debug;
                                                                                                             if lvl
                                                                                                                    <=
                                                                                                                    ::log::STATIC_MAX_LEVEL
                                                                                                                    &&
                                                                                                                    lvl
                                                                                                                        <=
                                                                                                                        ::log::max_level()
                                                                                                                {
                                                                                                                 ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["VRF verification failed: "],
                                                                                                                                                                        &match (&s,)
                                                                                                                                                                             {
                                                                                                                                                                             (arg0,)
                                                                                                                                                                             =>
                                                                                                                                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                          ::std::fmt::Debug::fmt)],
                                                                                                                                                                         }),
                                                                                                                                          lvl,
                                                                                                                                          &("babe",
                                                                                                                                            "substrate_consensus_babe",
                                                                                                                                            "core/consensus/babe/src/lib.rs",
                                                                                                                                            583u32));
                                                                                                             }
                                                                                                         };
                                                                                                         ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["VRF verification failed"],
                                                                                                                                                            &match ()
                                                                                                                                                                 {
                                                                                                                                                                 ()
                                                                                                                                                                 =>
                                                                                                                                                                 [],
                                                                                                                                                             }))
                                                                                                     })?
                };
            if check(&inout, threshold) {
                match check_equivocation(&client, slot_now, slot_num,
                                         header.clone(), signer.clone()) {
                    Ok(Some(equivocation_proof)) => {
                        let log_str =
                            ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Slot author ",
                                                                                 " is equivocating at slot ",
                                                                                 " with headers ",
                                                                                 " and "],
                                                                               &match (&signer,
                                                                                       &slot_num,
                                                                                       &equivocation_proof.fst_header().hash(),
                                                                                       &equivocation_proof.snd_header().hash())
                                                                                    {
                                                                                    (arg0,
                                                                                     arg1,
                                                                                     arg2,
                                                                                     arg3)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Debug::fmt),
                                                                                     ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                 ::std::fmt::Display::fmt),
                                                                                     ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                 ::std::fmt::Debug::fmt),
                                                                                     ::std::fmt::ArgumentV1::new(arg3,
                                                                                                                 ::std::fmt::Debug::fmt)],
                                                                                }));
                        {
                            let lvl = ::log::Level::Info;
                            if lvl <= ::log::STATIC_MAX_LEVEL &&
                                   lvl <= ::log::max_level() {
                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&[""],
                                                                                       &match (&log_str,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        }),
                                                         lvl,
                                                         &("substrate_consensus_babe",
                                                           "substrate_consensus_babe",
                                                           "core/consensus/babe/src/lib.rs",
                                                           598u32));
                            }
                        };
                        Err(log_str)
                    }
                    Ok(None) => {
                        Ok(CheckedHeader::Checked(header, digest_item))
                    }
                    Err(e) => { Err(e.to_string()) }
                }
            } else {
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["VRF verification failed: threshold ",
                                                                                 " exceeded"],
                                                                               &match (&threshold,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("babe",
                                                   "substrate_consensus_babe",
                                                   "core/consensus/babe/src/lib.rs",
                                                   609u32));
                    }
                };
                Err(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Validator ",
                                                                         " made seal when it wasn\u{2019}t its turn"],
                                                                       &match (&signer,)
                                                                            {
                                                                            (arg0,)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Debug::fmt)],
                                                                        })))
            }
        } else {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Bad signature on "],
                                                                           &match (&hash,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Debug::fmt)],
                                                                            }),
                                             lvl,
                                             &("babe",
                                               "substrate_consensus_babe",
                                               "core/consensus/babe/src/lib.rs",
                                               613u32));
                }
            };
            Err(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Bad signature on "],
                                                                   &match (&hash,)
                                                                        {
                                                                        (arg0,)
                                                                        =>
                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                     ::std::fmt::Debug::fmt)],
                                                                    })))
        }
    }
}
/// A verifier for Babe blocks.
pub struct BabeVerifier<C, E> {
    client: Arc<C>,
    extra: E,
    inherent_data_providers: inherents::InherentDataProviders,
    threshold: u64,
}
impl <C, E> BabeVerifier<C, E> {
    fn check_inherents<B: Block>(&self, block: B, block_id: BlockId<B>,
                                 inherent_data: InherentData)
     -> Result<(), String> where C: ProvideRuntimeApi,
     C::Api: BlockBuilderApi<B> {
        let inherent_res =
            self.client.runtime_api().check_inherents(&block_id, block,
                                                      inherent_data).map_err(|e|
                                                                                 ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                    &match (&e,)
                                                                                                                                         {
                                                                                                                                         (arg0,)
                                                                                                                                         =>
                                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                      ::std::fmt::Debug::fmt)],
                                                                                                                                     })))?;
        if !inherent_res.ok() {
            inherent_res.into_errors().try_for_each(|(i, e)|
                                                        Err(self.inherent_data_providers.error_to_string(&i,
                                                                                                         &e)))
        } else { Ok(()) }
    }
}
/// No-op extra verification.
#[rustc_copy_clone_marker]
pub struct NothingExtra;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for NothingExtra {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NothingExtra => {
                let mut debug_trait_builder = f.debug_tuple("NothingExtra");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for NothingExtra {
    #[inline]
    fn clone(&self) -> NothingExtra { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for NothingExtra { }
impl <B: Block> ExtraVerification<B> for NothingExtra {
    type
    Verified
    =
    Result<(), String>;
    fn verify(&self, _: &B::Header, _: Option<&[B::Extrinsic]>)
     -> Self::Verified {
        Ok(())
    }
}
impl <B: Block, C, E> Verifier<B> for BabeVerifier<C, E> where
 C: ProvideRuntimeApi + Send + Sync + AuxStore, C::Api: BlockBuilderApi<B>,
 DigestItemFor<B>: CompatibleDigestItem + DigestItem<AuthorityId = Public>,
 E: ExtraVerification<B>, Self: Authorities<B> {
    fn verify(&self, origin: BlockOrigin, header: B::Header,
              justification: Option<Justification>,
              mut body: Option<Vec<B::Extrinsic>>)
     -> Result<(ImportBlock<B>, Option<Vec<Public>>), String> {
        {
            let lvl = ::log::Level::Trace;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Verifying origin: ",
                                                                         " header: ",
                                                                         " justification: ",
                                                                         " body: "],
                                                                       &match (&origin,
                                                                               &header,
                                                                               &justification,
                                                                               &body)
                                                                            {
                                                                            (arg0,
                                                                             arg1,
                                                                             arg2,
                                                                             arg3)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Debug::fmt),
                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                         ::std::fmt::Debug::fmt),
                                                                             ::std::fmt::ArgumentV1::new(arg2,
                                                                                                         ::std::fmt::Debug::fmt),
                                                                             ::std::fmt::ArgumentV1::new(arg3,
                                                                                                         ::std::fmt::Debug::fmt)],
                                                                        }),
                                         lvl,
                                         &("babe", "substrate_consensus_babe",
                                           "core/consensus/babe/src/lib.rs",
                                           678u32));
            }
        };
        let mut inherent_data =
            self.inherent_data_providers.create_inherent_data().map_err(String::from)?;
        let (_, slot_now) =
            BabeSlotCompatible::extract_timestamp_and_slot(&inherent_data).map_err(|e|
                                                                                       ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Could not extract timestamp and slot: "],
                                                                                                                                          &match (&e,)
                                                                                                                                               {
                                                                                                                                               (arg0,)
                                                                                                                                               =>
                                                                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                            ::std::fmt::Debug::fmt)],
                                                                                                                                           })))?;
        let hash = header.hash();
        let parent_hash = *header.parent_hash();
        let authorities =
            self.authorities(&BlockId::Hash(parent_hash)).map_err(|e|
                                                                      ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Could not fetch authorities at ",
                                                                                                                           ": "],
                                                                                                                         &match (&parent_hash,
                                                                                                                                 &e)
                                                                                                                              {
                                                                                                                              (arg0,
                                                                                                                               arg1)
                                                                                                                              =>
                                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                           ::std::fmt::Debug::fmt),
                                                                                                                               ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                           ::std::fmt::Debug::fmt)],
                                                                                                                          })))?;
        let extra_verification =
            self.extra.verify(&header, body.as_ref().map(|x| &x[..]));
        let checked_header =
            check_header::<B,
                           C>(&self.client, slot_now + 1, header, hash,
                              &authorities[..], self.threshold)?;
        match checked_header {
            CheckedHeader::Checked(pre_header, seal) => {
                let BabeSeal { slot_num, .. } =
                    seal.as_babe_seal().expect("check_header always returns a seal digest item; qed");
                if let Some(inner_body) = body.take() {
                    inherent_data.babe_replace_inherent_data(slot_num);
                    let block = B::new(pre_header.clone(), inner_body);
                    self.check_inherents(block.clone(),
                                         BlockId::Hash(parent_hash),
                                         inherent_data)?;
                    let (_, inner_body) = block.deconstruct();
                    body = Some(inner_body);
                }
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Checked ",
                                                                                 "; importing."],
                                                                               &match (&pre_header,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Debug::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("babe",
                                                   "substrate_consensus_babe",
                                                   "core/consensus/babe/src/lib.rs",
                                                   735u32));
                    }
                };
                ::substrate_telemetry::with_logger(|l|
                                                       {
                                                           if ::slog::Level::Info.as_usize()
                                                                  <=
                                                                  ::slog::__slog_static_max_level().as_usize()
                                                              {
                                                               l.log(&{
                                                                          static RS:
                                                                                 ::slog::RecordStatic<'static>
                                                                                 =
                                                                              {
                                                                                  static LOC:
                                                                                         ::slog::RecordLocation
                                                                                         =
                                                                                      ::slog::RecordLocation{file:
                                                                                                                 "core/consensus/babe/src/lib.rs",
                                                                                                             line:
                                                                                                                 736u32,
                                                                                                             column:
                                                                                                                 5u32,
                                                                                                             function:
                                                                                                                 "",
                                                                                                             module:
                                                                                                                 "substrate_consensus_babe",};
                                                                                  ::slog::RecordStatic{location:
                                                                                                           &LOC,
                                                                                                       level:
                                                                                                           ::slog::Level::Info,
                                                                                                       tag:
                                                                                                           CONSENSUS_TRACE,}
                                                                              };
                                                                          ::slog::Record::new(&RS,
                                                                                              &::std::fmt::Arguments::new_v1(&["babe.checked_and_importing"],
                                                                                                                             &match ()
                                                                                                                                  {
                                                                                                                                  ()
                                                                                                                                  =>
                                                                                                                                  [],
                                                                                                                              }),
                                                                                              ::slog::BorrowedKV(&(::slog::SingleKV::from(("pre_header",
                                                                                                                                           ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                         &match (&pre_header,)
                                                                                                                                                                              {
                                                                                                                                                                              (arg0,)
                                                                                                                                                                              =>
                                                                                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                           ::std::fmt::Debug::fmt)],
                                                                                                                                                                          }))),
                                                                                                                   ())))
                                                                      })
                                                           }
                                                       });
                extra_verification.into_future().wait()?;
                let import_block =
                    ImportBlock{origin,
                                header: pre_header,
                                post_digests: <[_]>::into_vec(box [seal]),
                                body,
                                finalized: false,
                                justification,
                                auxiliary: Vec::new(),
                                fork_choice:
                                    ForkChoiceStrategy::LongestChain,};
                Ok((import_block, None))
            }
            CheckedHeader::Deferred(a, b) => {
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Checking ",
                                                                                 " failed; ",
                                                                                 ", ",
                                                                                 "."],
                                                                               &match (&hash,
                                                                                       &a,
                                                                                       &b)
                                                                                    {
                                                                                    (arg0,
                                                                                     arg1,
                                                                                     arg2)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Debug::fmt),
                                                                                     ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                 ::std::fmt::Debug::fmt),
                                                                                     ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                 ::std::fmt::Debug::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("babe",
                                                   "substrate_consensus_babe",
                                                   "core/consensus/babe/src/lib.rs",
                                                   758u32));
                    }
                };
                ::substrate_telemetry::with_logger(|l|
                                                       {
                                                           if ::slog::Level::Info.as_usize()
                                                                  <=
                                                                  ::slog::__slog_static_max_level().as_usize()
                                                              {
                                                               l.log(&{
                                                                          static RS:
                                                                                 ::slog::RecordStatic<'static>
                                                                                 =
                                                                              {
                                                                                  static LOC:
                                                                                         ::slog::RecordLocation
                                                                                         =
                                                                                      ::slog::RecordLocation{file:
                                                                                                                 "core/consensus/babe/src/lib.rs",
                                                                                                             line:
                                                                                                                 759u32,
                                                                                                             column:
                                                                                                                 5u32,
                                                                                                             function:
                                                                                                                 "",
                                                                                                             module:
                                                                                                                 "substrate_consensus_babe",};
                                                                                  ::slog::RecordStatic{location:
                                                                                                           &LOC,
                                                                                                       level:
                                                                                                           ::slog::Level::Info,
                                                                                                       tag:
                                                                                                           CONSENSUS_DEBUG,}
                                                                              };
                                                                          ::slog::Record::new(&RS,
                                                                                              &::std::fmt::Arguments::new_v1(&["babe.header_too_far_in_future"],
                                                                                                                             &match ()
                                                                                                                                  {
                                                                                                                                  ()
                                                                                                                                  =>
                                                                                                                                  [],
                                                                                                                              }),
                                                                                              ::slog::BorrowedKV(&(::slog::SingleKV::from(("b",
                                                                                                                                           ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                         &match (&b,)
                                                                                                                                                                              {
                                                                                                                                                                              (arg0,)
                                                                                                                                                                              =>
                                                                                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                           ::std::fmt::Debug::fmt)],
                                                                                                                                                                          }))),
                                                                                                                   (::slog::SingleKV::from(("a",
                                                                                                                                            ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                          &match (&a,)
                                                                                                                                                                               {
                                                                                                                                                                               (arg0,)
                                                                                                                                                                               =>
                                                                                                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                            ::std::fmt::Debug::fmt)],
                                                                                                                                                                           }))),
                                                                                                                    (::slog::SingleKV::from(("hash",
                                                                                                                                             ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                           &match (&hash,)
                                                                                                                                                                                {
                                                                                                                                                                                (arg0,)
                                                                                                                                                                                =>
                                                                                                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                                                                                                            }))),
                                                                                                                     ())))))
                                                                      })
                                                           }
                                                       });
                Err(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Header ",
                                                                         " rejected: too far in the future"],
                                                                       &match (&hash,)
                                                                            {
                                                                            (arg0,)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Debug::fmt)],
                                                                        })))
            }
        }
    }
}
impl <B, C, E> Authorities<B> for BabeVerifier<C, E> where B: Block,
 C: ProvideRuntimeApi + ProvideCache<B>, C::Api: AuthoritiesApi<B> {
    type
    Error
    =
    ConsensusError;
    fn authorities(&self, at: &BlockId<B>)
     -> Result<Vec<AuthorityIdFor<B>>, Self::Error> {
        authorities(self.client.as_ref(), at)
    }
}
fn authorities<B, C>(client: &C, at: &BlockId<B>)
 -> Result<Vec<AuthorityIdFor<B>>, ConsensusError> where B: Block,
 C: ProvideRuntimeApi + ProvideCache<B>, C::Api: AuthoritiesApi<B> {
    client.cache().and_then(|cache|
                                cache.get_at(&well_known_cache_keys::AUTHORITIES,
                                             at).and_then(|v|
                                                              Decode::decode(&mut &v[..]))).or_else(||
                                                                                                        {
                                                                                                            if client.runtime_api().has_api::<AuthoritiesApi<B>>(at).unwrap_or(false)
                                                                                                               {
                                                                                                                AuthoritiesApi::authorities(&*client.runtime_api(),
                                                                                                                                            at).ok()
                                                                                                            } else {
                                                                                                                {
                                                                                                                    ::std::rt::begin_panic("We don’t support deprecated code with new consensus algorithms, \
						therefore this is unreachable; qed",
                                                                                                                                           &("core/consensus/babe/src/lib.rs",
                                                                                                                                             796u32,
                                                                                                                                             5u32))
                                                                                                                }
                                                                                                            }
                                                                                                        }).ok_or_else(||
                                                                                                                          consensus_common::ErrorKind::InvalidAuthoritiesSet.into())
}
/// The BABE import queue type.
pub type BabeImportQueue<B> = BasicQueue<B>;
/// Register the babe inherent data provider, if not registered already.
fn register_babe_inherent_data_provider(inherent_data_providers:
                                            &InherentDataProviders,
                                        slot_duration: u64)
 -> Result<(), consensus_common::Error> {
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Registering"],
                                                                   &match () {
                                                                        () =>
                                                                        [],
                                                                    }), lvl,
                                     &("babe", "substrate_consensus_babe",
                                       "core/consensus/babe/src/lib.rs",
                                       810u32));
        }
    };
    if !inherent_data_providers.has_provider(&srml_babe::INHERENT_IDENTIFIER)
       {
        inherent_data_providers.register_provider(srml_babe::InherentDataProvider::new(slot_duration)).map_err(inherent_to_common_error)
    } else { Ok(()) }
}
fn get_keypair(q: &sr25519::Pair) -> &Keypair { q.as_ref() }
fn make_transcript(randomness: &[u8], slot_number: u64, genesis_hash: &[u8],
                   epoch: u64) -> Transcript {
    let mut transcript = Transcript::new(&BABE_ENGINE_ID);
    transcript.commit_bytes(b"slot number", &slot_number.to_le_bytes());
    transcript.commit_bytes(b"genesis block hash", genesis_hash);
    transcript.commit_bytes(b"current epoch", &epoch.to_le_bytes());
    transcript.commit_bytes(b"chain randomness", randomness);
    transcript
}
fn check(inout: &VRFInOut, threshold: u64) -> bool {
    u64::from_le_bytes(inout.make_bytes::<[u8; 8]>(BABE_VRF_PREFIX)) <
        threshold
}
/// Claim a slot if it is our turn.  Returns `None` if it is not our turn.
///
/// This hashes the slot number, epoch, genesis hash, and chain randomness into
/// the VRF.  If the VRF produces a value less than `threshold`, it is our turn,
/// so it returns `Some(_)`.  Otherwise, it returns `None`.
fn claim_slot(randomness: &[u8], slot_number: u64, genesis_hash: &[u8],
              epoch: u64, authorities: &[sr25519::Public],
              key: &sr25519::Pair, threshold: u64)
 -> Option<(VRFInOut, VRFProof, VRFProofBatchable)> {
    if !authorities.contains(&key.public()) { return None }
    let transcript =
        make_transcript(randomness, slot_number, genesis_hash, epoch);
    let threshold = threshold / authorities.len() as u64;
    get_keypair(key).vrf_sign_n_check(transcript,
                                      |inout| check(inout, threshold))
}
