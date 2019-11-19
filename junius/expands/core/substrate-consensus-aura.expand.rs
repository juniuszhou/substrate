#![feature(prelude_import)]
#![no_std]
// Copyright 2018-2019 Parity Technologies (UK) Ltd.
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

//! Aura (Authority-round) consensus in substrate.
//!
//! Aura works by having a list of authorities A who are expected to roughly
//! agree on the current time. Time is divided up into discrete slots of t
//! seconds each. For each slot s, the author of that slot is A[s % |A|].
//!
//! The author is allowed to issue one block but not more during that slot,
//! and it will be built upon the longest valid chain that has been seen.
//!
//! Blocks from future steps will be either deferred or rejected depending on how
//! far in the future they are.
#![forbid(missing_docs, unsafe_code)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
use std::{sync::Arc, time::Duration, thread, marker::PhantomData, hash::Hash,
          fmt::Debug};

use parity_codec::{Encode, Decode};
use consensus_common::{self, Authorities, BlockImport, Environment, Proposer,
                       ForkChoiceStrategy, ImportBlock, BlockOrigin, Error as
                       ConsensusError, SelectChain, well_known_cache_keys};
use consensus_common::import_queue::{Verifier, BasicQueue, SharedBlockImport,
                                     SharedJustificationImport,
                                     SharedFinalityProofImport,
                                     SharedFinalityProofRequestBuilder};
use client::{block_builder::api::BlockBuilder as BlockBuilderApi,
             blockchain::ProvideCache, runtime_api::{ApiExt, Core as CoreApi},
             error::Result as CResult, backend::AuxStore};
use aura_primitives::AURA_ENGINE_ID;
use runtime_primitives::{generic, generic::BlockId, Justification};
use runtime_primitives::traits::{Block, Header, Digest, DigestItemFor,
                                 DigestItem, ProvideRuntimeApi,
                                 AuthorityIdFor, Zero};
use primitives::Pair;
use inherents::{InherentDataProviders, InherentData, RuntimeString};
use authorities::AuthoritiesApi;

use futures::{Future, IntoFuture, future, stream::Stream};
use tokio::timer::Timeout;
use log::{warn, debug, info, trace};

use srml_aura::{InherentType as AuraInherent, AuraInherentData,
                timestamp::{TimestampInherentData, InherentType as
                            TimestampInherent, InherentError as TIError}};
use substrate_telemetry::{telemetry, CONSENSUS_TRACE, CONSENSUS_DEBUG,
                          CONSENSUS_WARN, CONSENSUS_INFO};

use slots::{CheckedHeader, SlotWorker, SlotInfo, SlotCompatible, slot_now,
            check_equivocation};

pub use aura_primitives::*;
pub use consensus_common::{SyncOracle, ExtraVerification};

type AuthorityId<P> = <P as Pair>::Public;
type Signature<P> = <P as Pair>::Signature;

/// A handle to the network. This is generally implemented by providing some
/// handle to a gossip service or similar.
///
/// Intended to be a lightweight handle such as an `Arc`.
#[deprecated(since = "1.0.1",
             note =
                 "This is dead code and will be removed in a future release")]
pub trait Network: Clone {
    /// A stream of input messages for a topic.
    type
    In: Stream<Item
    =
    Vec<u8>,
    Error
    =
    ()>;

    /// Send a message at a specific round out.
    fn send_message(&self, slot: u64, message: Vec<u8>);
}

/// A slot duration. Create with `get_or_compute`.
#[structural_match]
#[rustc_copy_clone_marker]
pub struct SlotDuration(slots::SlotDuration<u64>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for SlotDuration {
    #[inline]
    fn clone(&self) -> SlotDuration {
        {
            let _: ::std::clone::AssertParamIsClone<slots::SlotDuration<u64>>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for SlotDuration { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for SlotDuration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SlotDuration(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("SlotDuration");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_SlotDuration: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for SlotDuration {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_SlotDuration: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for SlotDuration {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(SlotDuration(_parity_codec::Decode::decode(input)?))
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::hash::Hash for SlotDuration {
    fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            SlotDuration(ref __self_0_0) => {
                ::std::hash::Hash::hash(&(*__self_0_0), state)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialOrd for SlotDuration {
    #[inline]
    fn partial_cmp(&self, other: &SlotDuration)
     -> ::std::option::Option<::std::cmp::Ordering> {
        match *other {
            SlotDuration(ref __self_1_0) =>
            match *self {
                SlotDuration(ref __self_0_0) =>
                match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                          &(*__self_1_0)) {
                    ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                    =>
                    ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &SlotDuration) -> bool {
        match *other {
            SlotDuration(ref __self_1_0) =>
            match *self {
                SlotDuration(ref __self_0_0) =>
                ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                     &(*__self_1_0)),
                                                 ::std::cmp::Ordering::Greater)
                    == ::std::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &SlotDuration) -> bool {
        match *other {
            SlotDuration(ref __self_1_0) =>
            match *self {
                SlotDuration(ref __self_0_0) =>
                ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                     &(*__self_1_0)),
                                                 ::std::cmp::Ordering::Greater)
                    != ::std::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &SlotDuration) -> bool {
        match *other {
            SlotDuration(ref __self_1_0) =>
            match *self {
                SlotDuration(ref __self_0_0) =>
                ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                     &(*__self_1_0)),
                                                 ::std::cmp::Ordering::Less)
                    == ::std::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &SlotDuration) -> bool {
        match *other {
            SlotDuration(ref __self_1_0) =>
            match *self {
                SlotDuration(ref __self_0_0) =>
                ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                     &(*__self_1_0)),
                                                 ::std::cmp::Ordering::Less)
                    != ::std::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Ord for SlotDuration {
    #[inline]
    fn cmp(&self, other: &SlotDuration) -> ::std::cmp::Ordering {
        match *other {
            SlotDuration(ref __self_1_0) =>
            match *self {
                SlotDuration(ref __self_0_0) =>
                match ::std::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::std::cmp::Ordering::Equal =>
                    ::std::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for SlotDuration {
    #[inline]
    fn eq(&self, other: &SlotDuration) -> bool {
        match *other {
            SlotDuration(ref __self_1_0) =>
            match *self {
                SlotDuration(ref __self_0_0) =>
                (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &SlotDuration) -> bool {
        match *other {
            SlotDuration(ref __self_1_0) =>
            match *self {
                SlotDuration(ref __self_0_0) =>
                (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for SlotDuration {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<slots::SlotDuration<u64>>; }
    }
}

impl SlotDuration {
    /// Either fetch the slot duration from disk or compute it from the genesis
    /// state.
    pub fn get_or_compute<B: Block, C>(client: &C) -> CResult<Self> where
     C: AuxStore, C: ProvideRuntimeApi, C::Api: AuraApi<B> {
        slots::SlotDuration::get_or_compute(client,
                                            |a, b|
                                                a.slot_duration(b)).map(Self)
    }

    /// Get the slot duration in milliseconds.
    pub fn get(&self) -> u64 { self.0.get() }
}

/// Get slot author for given block along with authorities.
fn slot_author<P: Pair>(slot_num: u64, authorities: &[AuthorityId<P>])
 -> Option<&AuthorityId<P>> {
    if authorities.is_empty() { return None }

    let idx = slot_num % (authorities.len() as u64);
    if !(idx <= usize::max_value() as u64) {













        // The function we are in is also deprecated.









        // we are the slot author. make a block and sign it.

        // deadline our production to approx. the end of the
        // slot

        // minor hack since we don't have access to the timestamp
        // that is actually set by the proposer.


        // sign the pre-sealed hash of the block and then
        // add it to a digest item.




        //
        // FIXME #1018 needs misbehavior types



        // check the signature is valid under the expected authority and
        // chain state.






        // halt import until timestamp is valid.
        // reject when too far ahead.







        // we add one to allow for some small drift.
        // FIXME #1019 in the future, alter this queue to allow deferring of headers

        // if the body is passed through, we need to use the runtime
        // to check that the internally-set timestamp in the inherents
        // actually matches the slot set in the seal.

        // skip the inherents verification if the runtime API is old.









        // no cache => no initialization

        // check if we already have initialized the cache









































        // wait for all finalized on each.









        // It's ok to sign same headers.

        // But not two different headers at the same slot.

        // Different slot is ok.

        // Here we trigger pruning and save header 4.

        // This fails because header 5 is an equivocation of header 4.

        // This is ok because we pruned the corresponding header. Shows that we are pruning.
        {
            ::std::rt::begin_panic("It is impossible to have a vector with length beyond the address space; qed",
                                   &("core/consensus/aura/src/lib.rs", 115u32,
                                     2u32))
        }
    };
    let current_author =
        authorities.get(idx as
                            usize).expect("authorities not empty; index constrained to list length;\
				this is a valid index; qed");
    Some(current_author)
}
fn inherent_to_common_error(err: RuntimeString) -> consensus_common::Error {
    consensus_common::ErrorKind::InherentData(err.into()).into()
}
/// A digest item which is usable with aura consensus.
pub trait CompatibleDigestItem<T: Pair>: Sized {
    /// Construct a digest item which contains a slot number and a signature on the
    /// hash.
    fn aura_seal(slot_num: u64, signature: Signature<T>)
    -> Self;
    /// If this item is an Aura seal, return the slot number and signature.
    fn as_aura_seal(&self)
    -> Option<(u64, Signature<T>)>;
    /// Return `true` if this seal type is deprecated.  Otherwise, return
    /// `false`.
    fn is_deprecated(&self)
    -> bool;
}
impl <P, Hash> CompatibleDigestItem<P> for
 generic::DigestItem<Hash, P::Public, P::Signature> where P: Pair,
 P::Signature: Clone + Encode + Decode {
    /// Construct a digest item which is a slot number and a signature on the
    /// hash.
    fn aura_seal(slot_number: u64, signature: Signature<P>) -> Self {
        generic::DigestItem::Consensus(AURA_ENGINE_ID,
                                       (slot_number, signature).encode())
    }
    /// If this item is an Aura seal, return the slot number and signature.
    #[allow(deprecated)]
    fn as_aura_seal(&self) -> Option<(u64, Signature<P>)> {
        match self {
            generic::DigestItem::Seal(slot, ref sig) =>
            Some((*slot, (*sig).clone())),
            generic::DigestItem::Consensus(AURA_ENGINE_ID, seal) =>
            Decode::decode(&mut &seal[..]),
            _ => None,
        }
    }
    #[allow(deprecated)]
    fn is_deprecated(&self) -> bool {
        match self { generic::DigestItem::Seal(_, _) => true, _ => false, }
    }
}
#[structural_match]
#[rustc_copy_clone_marker]
struct AuraSlotCompatible;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for AuraSlotCompatible { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for AuraSlotCompatible {
    #[inline]
    fn clone(&self) -> AuraSlotCompatible { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for AuraSlotCompatible {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AuraSlotCompatible => {
                let mut debug_trait_builder =
                    f.debug_tuple("AuraSlotCompatible");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for AuraSlotCompatible {
    #[inline]
    fn eq(&self, other: &AuraSlotCompatible) -> bool {
        match *other {
            AuraSlotCompatible => match *self { AuraSlotCompatible => true, },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for AuraSlotCompatible {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::hash::Hash for AuraSlotCompatible {
    fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self { AuraSlotCompatible => { } }
    }
}
impl SlotCompatible for AuraSlotCompatible {
    fn extract_timestamp_and_slot(data: &InherentData)
     -> Result<(TimestampInherent, AuraInherent), consensus_common::Error> {
        data.timestamp_inherent_data().and_then(|t|
                                                    data.aura_inherent_data().map(|a|
                                                                                      (t,
                                                                                       a))).map_err(inherent_to_common_error)
    }
}
/// Start the aura worker in a separate thread.
#[deprecated(since = "1.1", note = "Please spawn a thread manually")]
pub fn start_aura_thread<B, C, SC, E, I, P, SO, Error,
                         OnExit>(slot_duration: SlotDuration,
                                 local_key: Arc<P>, client: Arc<C>,
                                 select_chain: SC, block_import: Arc<I>,
                                 env: Arc<E>, sync_oracle: SO,
                                 on_exit: OnExit,
                                 inherent_data_providers:
                                     InherentDataProviders,
                                 force_authoring: bool)
 -> Result<(), consensus_common::Error> where B: Block + 'static,
 C: ProvideRuntimeApi + ProvideCache<B> + AuxStore + Send + Sync + 'static,
 C::Api: AuthoritiesApi<B>, SC: SelectChain<B> + Clone + 'static,
 E: Environment<B, Error = Error> + Send + Sync + 'static,
 E::Proposer: Proposer<B, Error = Error> + Send + 'static,
 <<E::Proposer as Proposer<B>>::Create as IntoFuture>::Future: Send + 'static,
 I: BlockImport<B> + Send + Sync + 'static, Error: From<I::Error> + 'static,
 P: Pair + Send + Sync + 'static, P::Public: Encode + Decode + Eq + Clone +
 Debug + Hash + Send + Sync + 'static, P::Signature: Encode, SO: SyncOracle +
 Send + Sync + Clone + 'static, OnExit: Future<Item = (), Error = ()> + Send +
 'static, DigestItemFor<B>: CompatibleDigestItem<P> + DigestItem<AuthorityId =
 AuthorityId<P>> + 'static, Error: ::std::error::Error + Send +
 From<::consensus_common::Error> + 'static {
    let worker =
        AuraWorker{client: client.clone(),
                   block_import,
                   env,
                   local_key,
                   inherent_data_providers: inherent_data_providers.clone(),
                   sync_oracle: sync_oracle.clone(),
                   force_authoring,};

    #[allow(deprecated)]
    slots::start_slot_worker_thread::<_, _, _, _, AuraSlotCompatible, u64,
                                      _>(slot_duration.0, select_chain,
                                         Arc::new(worker), sync_oracle,
                                         on_exit, inherent_data_providers)
}
/// Start the aura worker. The returned future should be run in a tokio runtime.
pub fn start_aura<B, C, SC, E, I, P, SO, Error,
                  OnExit>(slot_duration: SlotDuration, local_key: Arc<P>,
                          client: Arc<C>, select_chain: SC,
                          block_import: Arc<I>, env: Arc<E>, sync_oracle: SO,
                          on_exit: OnExit,
                          inherent_data_providers: InherentDataProviders,
                          force_authoring: bool)
 -> Result<impl Future<Item = (), Error = ()>, consensus_common::Error> where
 B: Block, C: ProvideRuntimeApi + ProvideCache<B> + AuxStore,
 C::Api: AuthoritiesApi<B>, SC: SelectChain<B> + Clone, E: Environment<B,
 Error = Error>, E::Proposer: Proposer<B, Error = Error>,
 <<E::Proposer as Proposer<B>>::Create as IntoFuture>::Future: Send + 'static,
 I: BlockImport<B> + Send + Sync + 'static, P: Pair + Send + Sync + 'static,
 P::Public: Hash + Eq + Send + Sync + Clone + Debug + Encode + Decode +
 'static, P::Signature: Encode, SO: SyncOracle + Send + Sync + Clone,
 DigestItemFor<B>: CompatibleDigestItem<P> + DigestItem<AuthorityId =
 AuthorityId<P>>, Error: ::std::error::Error + Send +
 From<::consensus_common::Error> + From<I::Error> + 'static,
 OnExit: Future<Item = (), Error = ()> {
    let worker =
        AuraWorker{client: client.clone(),
                   block_import,
                   env,
                   local_key,
                   inherent_data_providers: inherent_data_providers.clone(),
                   sync_oracle: sync_oracle.clone(),
                   force_authoring,};
    slots::start_slot_worker::<_, _, _, _, _, AuraSlotCompatible,
                               _>(slot_duration.0, select_chain,
                                  Arc::new(worker), sync_oracle, on_exit,
                                  inherent_data_providers)
}
struct AuraWorker<C, E, I, P, SO> {
    client: Arc<C>,
    block_import: Arc<I>,
    env: Arc<E>,
    local_key: Arc<P>,
    sync_oracle: SO,
    inherent_data_providers: InherentDataProviders,
    force_authoring: bool,
}
impl <B: Block, C, E, I, P, Error, SO> SlotWorker<B> for
 AuraWorker<C, E, I, P, SO> where C: ProvideRuntimeApi + ProvideCache<B> +
 AuxStore, C::Api: AuthoritiesApi<B>, E: Environment<B, Error = Error>,
 E::Proposer: Proposer<B, Error = Error>,
 <<E::Proposer as Proposer<B>>::Create as IntoFuture>::Future: Send + 'static,
 I: BlockImport<B> + Send + Sync + 'static, P: Pair + Send + Sync + 'static,
 P::Public: Hash + Eq + Send + Sync + Clone + Debug + Encode + Decode +
 'static, P::Signature: Encode, SO: SyncOracle + Send + Clone,
 DigestItemFor<B>: CompatibleDigestItem<P> + DigestItem<AuthorityId =
 AuthorityId<P>>, Error: ::std::error::Error + Send +
 From<::consensus_common::Error> + From<I::Error> + 'static {
    type
    OnSlot
    =
    Box<Future<Item = (), Error = consensus_common::Error> + Send>;
    fn on_start(&self, slot_duration: u64)
     -> Result<(), consensus_common::Error> {
        register_aura_inherent_data_provider(&self.inherent_data_providers,
                                             slot_duration)
    }
    fn on_slot(&self, chain_head: B::Header, slot_info: SlotInfo)
     -> Self::OnSlot {
        let pair = self.local_key.clone();
        let public_key = self.local_key.public();
        let client = self.client.clone();
        let block_import = self.block_import.clone();
        let env = self.env.clone();
        let (timestamp, slot_num, slot_duration) =
            (slot_info.timestamp, slot_info.number, slot_info.duration);
        let authorities =
            match authorities(client.as_ref(),
                              &BlockId::Hash(chain_head.hash())) {
                Ok(authorities) => authorities,
                Err(e) => {
                    {
                        let lvl = ::log::Level::Warn;
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
                                                     &("substrate_consensus_aura",
                                                       "substrate_consensus_aura",
                                                       "core/consensus/aura/src/lib.rs",
                                                       334u32));
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
                                                                                                                     "core/consensus/aura/src/lib.rs",
                                                                                                                 line:
                                                                                                                     339u32,
                                                                                                                 column:
                                                                                                                     5u32,
                                                                                                                 function:
                                                                                                                     "",
                                                                                                                 module:
                                                                                                                     "substrate_consensus_aura",};
                                                                                      ::slog::RecordStatic{location:
                                                                                                               &LOC,
                                                                                                           level:
                                                                                                               ::slog::Level::Info,
                                                                                                           tag:
                                                                                                               CONSENSUS_WARN,}
                                                                                  };
                                                                              ::slog::Record::new(&RS,
                                                                                                  &::std::fmt::Arguments::new_v1(&["aura.unable_fetching_authorities"],
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
                                             &("aura",
                                               "substrate_consensus_aura",
                                               "core/consensus/aura/src/lib.rs",
                                               347u32));
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
                                                                                                             "core/consensus/aura/src/lib.rs",
                                                                                                         line:
                                                                                                             348u32,
                                                                                                         column:
                                                                                                             4u32,
                                                                                                         function:
                                                                                                             "",
                                                                                                         module:
                                                                                                             "substrate_consensus_aura",};
                                                                              ::slog::RecordStatic{location:
                                                                                                       &LOC,
                                                                                                   level:
                                                                                                       ::slog::Level::Info,
                                                                                                   tag:
                                                                                                       CONSENSUS_DEBUG,}
                                                                          };
                                                                      ::slog::Record::new(&RS,
                                                                                          &::std::fmt::Arguments::new_v1(&["aura.skipping_proposal_slot"],
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
        let maybe_author = slot_author::<P>(slot_num, &authorities);
        let proposal_work =
            match maybe_author {
                None => return Box::new(future::ok(())),
                Some(author) =>
                if author == &public_key {
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
                                                     &("aura",
                                                       "substrate_consensus_aura",
                                                       "core/consensus/aura/src/lib.rs",
                                                       357u32));
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
                                                                                                                     "core/consensus/aura/src/lib.rs",
                                                                                                                 line:
                                                                                                                     362u32,
                                                                                                                 column:
                                                                                                                     5u32,
                                                                                                                 function:
                                                                                                                     "",
                                                                                                                 module:
                                                                                                                     "substrate_consensus_aura",};
                                                                                      ::slog::RecordStatic{location:
                                                                                                               &LOC,
                                                                                                           level:
                                                                                                               ::slog::Level::Info,
                                                                                                           tag:
                                                                                                               CONSENSUS_DEBUG,}
                                                                                  };
                                                                              ::slog::Record::new(&RS,
                                                                                                  &::std::fmt::Arguments::new_v1(&["aura.starting_authorship"],
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
                                                                 &("substrate_consensus_aura",
                                                                   "substrate_consensus_aura",
                                                                   "core/consensus/aura/src/lib.rs",
                                                                   370u32));
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
                                                                                                                                 "core/consensus/aura/src/lib.rs",
                                                                                                                             line:
                                                                                                                                 371u32,
                                                                                                                             column:
                                                                                                                                 7u32,
                                                                                                                             function:
                                                                                                                                 "",
                                                                                                                             module:
                                                                                                                                 "substrate_consensus_aura",};
                                                                                                  ::slog::RecordStatic{location:
                                                                                                                           &LOC,
                                                                                                                       level:
                                                                                                                           ::slog::Level::Info,
                                                                                                                       tag:
                                                                                                                           CONSENSUS_WARN,}
                                                                                              };
                                                                                          ::slog::Record::new(&RS,
                                                                                                              &::std::fmt::Arguments::new_v1(&["aura.unable_authoring_block"],
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
                    Timeout::new(proposer.propose(slot_info.inherent_data,
                                                  remaining_duration).into_future(),
                                 remaining_duration)
                } else { return Box::new(future::ok(())); },
            };
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
                                                                                &("substrate_consensus_aura",
                                                                                  "substrate_consensus_aura",
                                                                                  "core/consensus/aura/src/lib.rs",
                                                                                  397u32));
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
                                                                                                                                                "core/consensus/aura/src/lib.rs",
                                                                                                                                            line:
                                                                                                                                                401u32,
                                                                                                                                            column:
                                                                                                                                                7u32,
                                                                                                                                            function:
                                                                                                                                                "",
                                                                                                                                            module:
                                                                                                                                                "substrate_consensus_aura",};
                                                                                                                 ::slog::RecordStatic{location:
                                                                                                                                          &LOC,
                                                                                                                                      level:
                                                                                                                                          ::slog::Level::Info,
                                                                                                                                      tag:
                                                                                                                                          CONSENSUS_INFO,}
                                                                                                             };
                                                                                                         ::slog::Record::new(&RS,
                                                                                                                             &::std::fmt::Arguments::new_v1(&["aura.discarding_proposal_took_too_long"],
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
                                               (slot_num, pre_hash).encode();
                                           let signature =
                                               pair.sign(&to_sign[..]);
                                           let item =
                                               <DigestItemFor<B> as
                                                   CompatibleDigestItem<P>>::aura_seal(slot_num,
                                                                                       signature);
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
                                                                            &("substrate_consensus_aura",
                                                                              "substrate_consensus_aura",
                                                                              "core/consensus/aura/src/lib.rs",
                                                                              432u32));
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
                                                                                                                                            "core/consensus/aura/src/lib.rs",
                                                                                                                                        line:
                                                                                                                                            437u32,
                                                                                                                                        column:
                                                                                                                                            6u32,
                                                                                                                                        function:
                                                                                                                                            "",
                                                                                                                                        module:
                                                                                                                                            "substrate_consensus_aura",};
                                                                                                             ::slog::RecordStatic{location:
                                                                                                                                      &LOC,
                                                                                                                                  level:
                                                                                                                                      ::slog::Level::Info,
                                                                                                                                  tag:
                                                                                                                                      CONSENSUS_INFO,}
                                                                                                         };
                                                                                                     ::slog::Record::new(&RS,
                                                                                                                         &::std::fmt::Arguments::new_v1(&["aura.pre_sealed_block"],
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
                                                                                &("aura",
                                                                                  "substrate_consensus_aura",
                                                                                  "core/consensus/aura/src/lib.rs",
                                                                                  444u32));
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
                                                                                                                                                "core/consensus/aura/src/lib.rs",
                                                                                                                                            line:
                                                                                                                                                446u32,
                                                                                                                                            column:
                                                                                                                                                7u32,
                                                                                                                                            function:
                                                                                                                                                "",
                                                                                                                                            module:
                                                                                                                                                "substrate_consensus_aura",};
                                                                                                                 ::slog::RecordStatic{location:
                                                                                                                                          &LOC,
                                                                                                                                      level:
                                                                                                                                          ::slog::Level::Info,
                                                                                                                                      tag:
                                                                                                                                          CONSENSUS_WARN,}
                                                                                                             };
                                                                                                         ::slog::Record::new(&RS,
                                                                                                                             &::std::fmt::Arguments::new_v1(&["aura.err_with_block_built_on"],
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
                                                      consensus_common::ErrorKind::ClientImport(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                   &match (&e,)
                                                                                                                                                        {
                                                                                                                                                        (arg0,)
                                                                                                                                                        =>
                                                                                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                                                                                    }))).into()))
    }
}
/// check a header has been signed by the right key. If the slot is too far in the future, an error will be returned.
/// if it's successful, returns the pre-header and the digest item containing the seal.
///
/// This digest item will always return `Some` when used with `as_aura_seal`.
fn check_header<C, B: Block,
                P: Pair>(client: &Arc<C>, slot_now: u64,
                         mut header: B::Header, hash: B::Hash,
                         authorities: &[AuthorityId<P>],
                         allow_old_seals: bool)
 -> Result<CheckedHeader<B::Header, DigestItemFor<B>>, String> where
 DigestItemFor<B>: CompatibleDigestItem<P>, P::Signature: Decode,
 C: client::backend::AuxStore, P::Public: AsRef<P::Public> + Encode + Decode +
 PartialEq + Clone {
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
    if !allow_old_seals && digest_item.is_deprecated() {
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Header ",
                                                                         " uses old seal format, rejecting"],
                                                                       &match (&hash,)
                                                                            {
                                                                            (arg0,)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Debug::fmt)],
                                                                        }),
                                         lvl,
                                         &("aura", "substrate_consensus_aura",
                                           "core/consensus/aura/src/lib.rs",
                                           481u32));
            }
        };
        return Err(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Header ",
                                                                        " uses old seal format, rejecting"],
                                                                      &match (&hash,)
                                                                           {
                                                                           (arg0,)
                                                                           =>
                                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                                        ::std::fmt::Debug::fmt)],
                                                                       })))
    }
    let (slot_num, sig) =
        digest_item.as_aura_seal().ok_or_else(||
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
                                                                                       &("aura",
                                                                                         "substrate_consensus_aura",
                                                                                         "core/consensus/aura/src/lib.rs",
                                                                                         486u32));
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
    } else {
        let expected_author =
            match slot_author::<P>(slot_num, &authorities) {
                None => return Err("Slot Author not found".to_string()),
                Some(author) => author,
            };
        let pre_hash = header.hash();
        let to_sign = (slot_num, pre_hash).encode();
        let public = expected_author;
        if P::verify(&sig, &to_sign[..], public) {
            match check_equivocation::<_, _,
                                       <P as
                                       Pair>::Public>(client, slot_now,
                                                      slot_num,
                                                      header.clone(),
                                                      public.clone()) {
                Ok(Some(equivocation_proof)) => {
                    let log_str =
                        ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Slot author is equivocating at slot ",
                                                                             " with headers ",
                                                                             " and "],
                                                                           &match (&slot_num,
                                                                                   &equivocation_proof.fst_header().hash(),
                                                                                   &equivocation_proof.snd_header().hash())
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
                                                     &("substrate_consensus_aura",
                                                       "substrate_consensus_aura",
                                                       "core/consensus/aura/src/lib.rs",
                                                       520u32));
                        }
                    };
                    Err(log_str)
                }
                Ok(None) => Ok(CheckedHeader::Checked(header, digest_item)),
                Err(e) => Err(e.to_string()),
            }
        } else {
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
/// A verifier for Aura blocks.
pub struct AuraVerifier<C, E, P> {
    client: Arc<C>,
    extra: E,
    phantom: PhantomData<P>,
    inherent_data_providers: inherents::InherentDataProviders,
    allow_old_seals: bool,
}
impl <C, E, P> AuraVerifier<C, E, P> where P: Send + Sync + 'static {
    fn check_inherents<B: Block>(&self, block: B, block_id: BlockId<B>,
                                 inherent_data: InherentData,
                                 timestamp_now: u64) -> Result<(), String>
     where C: ProvideRuntimeApi, C::Api: BlockBuilderApi<B> {
        const MAX_TIMESTAMP_DRIFT_SECS: u64 = 60;
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
                                                        match TIError::try_from(&i,
                                                                                &e)
                                                            {
                                                            Some(TIError::ValidAtTimestamp(timestamp))
                                                            => {
                                                                if timestamp >
                                                                       timestamp_now
                                                                           +
                                                                           MAX_TIMESTAMP_DRIFT_SECS
                                                                   {
                                                                    return Err("Rejecting block too far in future".into());
                                                                }
                                                                let diff =
                                                                    timestamp.saturating_sub(timestamp_now);
                                                                {
                                                                    let lvl =
                                                                        ::log::Level::Info;
                                                                    if lvl <=
                                                                           ::log::STATIC_MAX_LEVEL
                                                                           &&
                                                                           lvl
                                                                               <=
                                                                               ::log::max_level()
                                                                       {
                                                                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["halting for block ",
                                                                                                                                 " seconds in the future"],
                                                                                                                               &match (&diff,)
                                                                                                                                    {
                                                                                                                                    (arg0,)
                                                                                                                                    =>
                                                                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                                                                }),
                                                                                                 lvl,
                                                                                                 &("aura",
                                                                                                   "substrate_consensus_aura",
                                                                                                   "core/consensus/aura/src/lib.rs",
                                                                                                   573u32));
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
                                                                                                                                                                 "core/consensus/aura/src/lib.rs",
                                                                                                                                                             line:
                                                                                                                                                                 578u32,
                                                                                                                                                             column:
                                                                                                                                                                 7u32,
                                                                                                                                                             function:
                                                                                                                                                                 "",
                                                                                                                                                             module:
                                                                                                                                                                 "substrate_consensus_aura",};
                                                                                                                                  ::slog::RecordStatic{location:
                                                                                                                                                           &LOC,
                                                                                                                                                       level:
                                                                                                                                                           ::slog::Level::Info,
                                                                                                                                                       tag:
                                                                                                                                                           CONSENSUS_INFO,}
                                                                                                                              };
                                                                                                                          ::slog::Record::new(&RS,
                                                                                                                                              &::std::fmt::Arguments::new_v1(&["aura.halting_for_future_block"],
                                                                                                                                                                             &match ()
                                                                                                                                                                                  {
                                                                                                                                                                                  ()
                                                                                                                                                                                  =>
                                                                                                                                                                                  [],
                                                                                                                                                                              }),
                                                                                                                                              ::slog::BorrowedKV(&(::slog::SingleKV::from(("diff",
                                                                                                                                                                                           ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                                         &match (&diff,)
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
                                                                thread::sleep(Duration::from_secs(diff));
                                                                Ok(())
                                                            }
                                                            Some(TIError::Other(e))
                                                            => Err(e.into()),
                                                            None =>
                                                            Err(self.inherent_data_providers.error_to_string(&i,
                                                                                                             &e)),
                                                        })
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
#[forbid(deprecated)]
impl <B: Block, C, E, P> Verifier<B> for AuraVerifier<C, E, P> where
 C: ProvideRuntimeApi + Send + Sync + client::backend::AuxStore,
 C::Api: BlockBuilderApi<B>, DigestItemFor<B>: CompatibleDigestItem<P> +
 DigestItem<AuthorityId = AuthorityId<P>>, E: ExtraVerification<B>, P: Pair +
 Send + Sync + 'static, P::Public: Send + Sync + Hash + Eq + Clone + Decode +
 Encode + Debug + AsRef<P::Public> + 'static, P::Signature: Encode + Decode,
 Self: Authorities<B> {
    fn verify(&self, origin: BlockOrigin, header: B::Header,
              justification: Option<Justification>,
              mut body: Option<Vec<B::Extrinsic>>)
     -> Result<(ImportBlock<B>, Option<Vec<AuthorityId<P>>>), String> {
        let mut inherent_data =
            self.inherent_data_providers.create_inherent_data().map_err(String::from)?;
        let (timestamp_now, slot_now) =
            AuraSlotCompatible::extract_timestamp_and_slot(&inherent_data).map_err(|e|
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
            check_header::<C, B,
                           P>(&self.client, slot_now + 1, header, hash,
                              &authorities[..], self.allow_old_seals)?;
        match checked_header {
            CheckedHeader::Checked(pre_header, seal) => {
                let (slot_num, _) =
                    seal.as_aura_seal().expect("check_header always returns a seal digest item; qed");
                if let Some(inner_body) = body.take() {
                    inherent_data.aura_replace_inherent_data(slot_num);
                    let block = B::new(pre_header.clone(), inner_body);
                    if self.client.runtime_api().has_api_with::<BlockBuilderApi<B>,
                                                                _>(&BlockId::Hash(parent_hash),
                                                                   |v|
                                                                       v >=
                                                                           2).map_err(|e|
                                                                                          ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                             &match (&e,)
                                                                                                                                                  {
                                                                                                                                                  (arg0,)
                                                                                                                                                  =>
                                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                                              })))?
                       {
                        self.check_inherents(block.clone(),
                                             BlockId::Hash(parent_hash),
                                             inherent_data, timestamp_now)?;
                    }
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
                                                 &("aura",
                                                   "substrate_consensus_aura",
                                                   "core/consensus/aura/src/lib.rs",
                                                   676u32));
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
                                                                                                                 "core/consensus/aura/src/lib.rs",
                                                                                                             line:
                                                                                                                 677u32,
                                                                                                             column:
                                                                                                                 5u32,
                                                                                                             function:
                                                                                                                 "",
                                                                                                             module:
                                                                                                                 "substrate_consensus_aura",};
                                                                                  ::slog::RecordStatic{location:
                                                                                                           &LOC,
                                                                                                       level:
                                                                                                           ::slog::Level::Info,
                                                                                                       tag:
                                                                                                           CONSENSUS_TRACE,}
                                                                              };
                                                                          ::slog::Record::new(&RS,
                                                                                              &::std::fmt::Arguments::new_v1(&["aura.checked_and_importing"],
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
                let new_authorities =
                    pre_header.digest().log(DigestItem::as_authorities_change).map(|digest|
                                                                                       digest.to_vec());
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
                Ok((import_block, new_authorities))
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
                                                 &("aura",
                                                   "substrate_consensus_aura",
                                                   "core/consensus/aura/src/lib.rs",
                                                   699u32));
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
                                                                                                                 "core/consensus/aura/src/lib.rs",
                                                                                                             line:
                                                                                                                 700u32,
                                                                                                             column:
                                                                                                                 5u32,
                                                                                                             function:
                                                                                                                 "",
                                                                                                             module:
                                                                                                                 "substrate_consensus_aura",};
                                                                                  ::slog::RecordStatic{location:
                                                                                                           &LOC,
                                                                                                       level:
                                                                                                           ::slog::Level::Info,
                                                                                                       tag:
                                                                                                           CONSENSUS_DEBUG,}
                                                                              };
                                                                          ::slog::Record::new(&RS,
                                                                                              &::std::fmt::Arguments::new_v1(&["aura.header_too_far_in_future"],
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
impl <B, C, E, P> Authorities<B> for AuraVerifier<C, E, P> where B: Block,
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
fn initialize_authorities_cache<B, C>(client: &C)
 -> Result<(), ConsensusError> where B: Block, C: ProvideRuntimeApi +
 ProvideCache<B>, C::Api: AuthoritiesApi<B> {
    let cache =
        match client.cache() { Some(cache) => cache, None => return Ok(()), };
    let genesis_id = BlockId::Number(Zero::zero());
    let genesis_authorities: Option<Vec<AuthorityIdFor<B>>> =
        cache.get_at(&well_known_cache_keys::AUTHORITIES,
                     &genesis_id).and_then(|v| Decode::decode(&mut &v[..]));
    if genesis_authorities.is_some() { return Ok(()); }
    let map_err =
        |error|
            consensus_common::Error::from(consensus_common::ErrorKind::ClientImport(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Error initializing authorities cache: "],
                                                                                                                                       &match (&error,)
                                                                                                                                            {
                                                                                                                                            (arg0,)
                                                                                                                                            =>
                                                                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                                                                        }))));
    let genesis_authorities = authorities(client, &genesis_id)?;
    cache.initialize(&well_known_cache_keys::AUTHORITIES,
                     genesis_authorities.encode()).map_err(map_err)?;
    Ok(())
}
#[allow(deprecated)]
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
                                                                                                                CoreApi::authorities(&*client.runtime_api(),
                                                                                                                                     at).ok()
                                                                                                            }
                                                                                                        }).ok_or_else(||
                                                                                                                          consensus_common::ErrorKind::InvalidAuthoritiesSet.into())
}
/// The Aura import queue type.
pub type AuraImportQueue<B> = BasicQueue<B>;
/// Register the aura inherent data provider, if not registered already.
fn register_aura_inherent_data_provider(inherent_data_providers:
                                            &InherentDataProviders,
                                        slot_duration: u64)
 -> Result<(), consensus_common::Error> {
    if !inherent_data_providers.has_provider(&srml_aura::INHERENT_IDENTIFIER)
       {
        inherent_data_providers.register_provider(srml_aura::InherentDataProvider::new(slot_duration)).map_err(inherent_to_common_error)
    } else { Ok(()) }
}
/// Start an import queue for the Aura consensus algorithm.
pub fn import_queue<B, C, E,
                    P>(slot_duration: SlotDuration,
                       block_import: SharedBlockImport<B>,
                       justification_import:
                           Option<SharedJustificationImport<B>>,
                       finality_proof_import:
                           Option<SharedFinalityProofImport<B>>,
                       finality_proof_request_builder:
                           Option<SharedFinalityProofRequestBuilder<B>>,
                       client: Arc<C>, extra: E,
                       inherent_data_providers: InherentDataProviders)
 -> Result<AuraImportQueue<B>, consensus_common::Error> where B: Block,
 C: 'static + ProvideRuntimeApi + ProvideCache<B> + Send + Sync + AuxStore,
 C::Api: BlockBuilderApi<B> + AuthoritiesApi<B>,
 DigestItemFor<B>: CompatibleDigestItem<P> + DigestItem<AuthorityId =
 AuthorityId<P>>, E: 'static + ExtraVerification<B>, P: Pair + Send + Sync +
 'static, P::Public: Clone + Eq + Send + Sync + Hash + Debug + Encode +
 Decode + AsRef<P::Public>, P::Signature: Encode + Decode {
    register_aura_inherent_data_provider(&inherent_data_providers,
                                         slot_duration.get())?;
    initialize_authorities_cache(&*client)?;
    let verifier =
        Arc::new(AuraVerifier{client: client.clone(),
                              extra,
                              inherent_data_providers,
                              phantom: PhantomData,
                              allow_old_seals: false,});
    Ok(BasicQueue::new(verifier, block_import, justification_import,
                       finality_proof_import, finality_proof_request_builder))
}
/// Start an import queue for the Aura consensus algorithm with backwards compatibility.
#[deprecated(since = "1.0.1",
             note =
                 "should not be used unless backwards compatibility with an older chain is needed.")]
pub fn import_queue_accept_old_seals<B, C, E,
                                     P>(slot_duration: SlotDuration,
                                        block_import: SharedBlockImport<B>,
                                        justification_import:
                                            Option<SharedJustificationImport<B>>,
                                        finality_proof_import:
                                            Option<SharedFinalityProofImport<B>>,
                                        finality_proof_request_builder:
                                            Option<SharedFinalityProofRequestBuilder<B>>,
                                        client: Arc<C>, extra: E,
                                        inherent_data_providers:
                                            InherentDataProviders)
 -> Result<AuraImportQueue<B>, consensus_common::Error> where B: Block,
 C: 'static + ProvideRuntimeApi + ProvideCache<B> + Send + Sync + AuxStore,
 C::Api: BlockBuilderApi<B> + AuthoritiesApi<B>,
 DigestItemFor<B>: CompatibleDigestItem<P> + DigestItem<AuthorityId =
 AuthorityId<P>>, E: 'static + ExtraVerification<B>, P: Pair + Send + Sync +
 'static, P::Public: Clone + Eq + Send + Sync + Hash + Debug + Encode +
 Decode + AsRef<P::Public>, P::Signature: Encode + Decode {
    register_aura_inherent_data_provider(&inherent_data_providers,
                                         slot_duration.get())?;
    initialize_authorities_cache(&*client)?;
    let verifier =
        Arc::new(AuraVerifier{client: client.clone(),
                              extra,
                              inherent_data_providers,
                              phantom: PhantomData,
                              allow_old_seals: true,});
    Ok(BasicQueue::new(verifier, block_import, justification_import,
                       finality_proof_import, finality_proof_request_builder))
}
