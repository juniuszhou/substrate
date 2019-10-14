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

//! The Substrate runtime. This can be compiled with ``#[no_std]`, ready for Wasm.

// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

use rstd::prelude::*;
use support::construct_runtime;
use substrate_primitives::u32_trait::{_2, _4};
use node_primitives::{AccountId, AccountIndex, Balance, BlockNumber, Hash,
                      Index, AuthorityId, Signature, AuthoritySignature};
use grandpa::fg_primitives::{self, ScheduledChange};
use client::{block_builder::api::{self as block_builder_api, InherentData,
                                  CheckInherentsResult}, runtime_api as
             client_api, impl_runtime_apis};
use runtime_primitives::{ApplyResult, generic, create_runtime_str};
use runtime_primitives::transaction_validity::TransactionValidity;
use runtime_primitives::traits::{BlakeTwo256, Block as BlockT, DigestFor,
                                 NumberFor, StaticLookup, AuthorityIdFor,
                                 Convert};
use version::RuntimeVersion;
use council::{motions as council_motions, voting as council_voting};
#[cfg(feature = "std")]
use council::seats as council_seats;
#[cfg(any(feature = "std", test))]
use version::NativeVersion;
use substrate_primitives::OpaqueMetadata;

#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
pub use consensus::Call as ConsensusCall;
pub use timestamp::Call as TimestampCall;
pub use balances::Call as BalancesCall;
pub use runtime_primitives::{Permill, Perbill};
pub use support::StorageValue;
pub use staking::StakerStatus;

/// Runtime version.
pub const VERSION: RuntimeVersion =
    RuntimeVersion{spec_name:










                       // The Aura module handles offline-reports internally
                       // rather than using an explicit report system.































                       { ::std::borrow::Cow::Borrowed("node") },
                   impl_name:
                       { ::std::borrow::Cow::Borrowed("substrate-node") },
                   authoring_version: 10,
                   spec_version: 80,
                   impl_version: 82,
                   apis: RUNTIME_API_VERSIONS,};
/// Native version.
#[cfg(any(feature = "std", test))]
pub fn native_version() -> NativeVersion {
    NativeVersion{runtime_version: VERSION,
                  can_author_with: Default::default(),}
}
pub struct CurrencyToVoteHandler;
impl CurrencyToVoteHandler {
    fn factor() -> u128 {
        (Balances::total_issuance() / u64::max_value() as u128).max(1)
    }
}
impl Convert<u128, u64> for CurrencyToVoteHandler {
    fn convert(x: u128) -> u64 { (x / Self::factor()) as u64 }
}
impl Convert<u128, u128> for CurrencyToVoteHandler {
    fn convert(x: u128) -> u128 { x * Self::factor() }
}
impl system::Trait for Runtime {
    type
    Origin
    =
    Origin;
    type
    Index
    =
    Index;
    type
    BlockNumber
    =
    BlockNumber;
    type
    Hash
    =
    Hash;
    type
    Hashing
    =
    BlakeTwo256;
    type
    Digest
    =
    generic::Digest<Log>;
    type
    AccountId
    =
    AccountId;
    type
    Lookup
    =
    Indices;
    type
    Header
    =
    generic::Header<BlockNumber, BlakeTwo256, Log>;
    type
    Event
    =
    Event;
    type
    Log
    =
    Log;
}
impl aura::Trait for Runtime {
    type
    HandleReport
    =
    aura::StakingSlasher<Runtime>;
}
impl indices::Trait for Runtime {
    type
    AccountIndex
    =
    AccountIndex;
    type
    IsDeadAccount
    =
    Balances;
    type
    ResolveHint
    =
    indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
    type
    Event
    =
    Event;
}
impl balances::Trait for Runtime {
    type
    Balance
    =
    Balance;
    type
    OnFreeBalanceZero
    =
    ((Staking, Contract), Session);
    type
    OnNewAccount
    =
    Indices;
    type
    Event
    =
    Event;
    type
    TransactionPayment
    =
    ();
    type
    DustRemoval
    =
    ();
    type
    TransferPayment
    =
    ();
}
impl consensus::Trait for Runtime {
    type
    Log
    =
    Log;
    type
    SessionKey
    =
    AuthorityId;
    type
    InherentOfflineReport
    =
    ();
}
impl timestamp::Trait for Runtime {
    type
    Moment
    =
    u64;
    type
    OnTimestampSet
    =
    Aura;
}
impl session::Trait for Runtime {
    type
    ConvertAccountIdToSessionKey
    =
    ();
    type
    OnSessionChange
    =
    (Staking, grandpa::SyncedAuthorities<Runtime>);
    type
    Event
    =
    Event;
}
impl staking::Trait for Runtime {
    type
    Currency
    =
    Balances;
    type
    CurrencyToVote
    =
    CurrencyToVoteHandler;
    type
    OnRewardMinted
    =
    Treasury;
    type
    Event
    =
    Event;
    type
    Slash
    =
    ();
    type
    Reward
    =
    ();
}
impl democracy::Trait for Runtime {
    type
    Currency
    =
    Balances;
    type
    Proposal
    =
    Call;
    type
    Event
    =
    Event;
}
impl council::Trait for Runtime {
    type
    Event
    =
    Event;
    type
    BadPresentation
    =
    ();
    type
    BadReaper
    =
    ();
}
impl council::voting::Trait for Runtime {
    type
    Event
    =
    Event;
}
impl council::motions::Trait for Runtime {
    type
    Origin
    =
    Origin;
    type
    Proposal
    =
    Call;
    type
    Event
    =
    Event;
}
impl treasury::Trait for Runtime {
    type
    Currency
    =
    Balances;
    type
    ApproveOrigin
    =
    council_motions::EnsureMembers<_4>;
    type
    RejectOrigin
    =
    council_motions::EnsureMembers<_2>;
    type
    Event
    =
    Event;
    type
    MintedForSpending
    =
    ();
    type
    ProposalRejection
    =
    ();
}
impl contract::Trait for Runtime {
    type
    Currency
    =
    Balances;
    type
    Call
    =
    Call;
    type
    Event
    =
    Event;
    type
    Gas
    =
    u64;
    type
    DetermineContractAddress
    =
    contract::SimpleAddressDeterminator<Runtime>;
    type
    ComputeDispatchFee
    =
    contract::DefaultDispatchFeeComputor<Runtime>;
    type
    TrieIdGenerator
    =
    contract::TrieIdFromParentCounter<Runtime>;
    type
    GasPayment
    =
    ();
}
impl sudo::Trait for Runtime {
    type
    Event
    =
    Event;
    type
    Proposal
    =
    Call;
}
impl grandpa::Trait for Runtime {
    type
    SessionKey
    =
    AuthorityId;
    type
    Log
    =
    Log;
    type
    Event
    =
    Event;
}
impl finality_tracker::Trait for Runtime {
    type
    OnFinalizationStalled
    =
    grandpa::SyncedAuthorities<Runtime>;
}
#[structural_match]
#[rustc_copy_clone_marker]
pub struct Runtime;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Runtime {
    #[inline]
    fn clone(&self) -> Runtime { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for Runtime { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Runtime {
    #[inline]
    fn eq(&self, other: &Runtime) -> bool {
        match *other { Runtime => match *self { Runtime => true, }, }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Runtime {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Runtime => {
                let mut debug_trait_builder = f.debug_tuple("Runtime");
                debug_trait_builder.finish()
            }
        }
    }
}
impl ::srml_support::runtime_primitives::traits::GetNodeBlockType for Runtime
 {
    type
    NodeBlock
    =
    node_primitives::Block;
}
impl ::srml_support::runtime_primitives::traits::GetRuntimeBlockType for
 Runtime {
    type
    RuntimeBlock
    =
    Block;
}
#[allow(non_camel_case_types)]
#[structural_match]
pub enum Event {
    system(system::Event),
    indices(indices::Event<Runtime>),
    balances(balances::Event<Runtime>),
    session(session::Event<Runtime>),
    staking(staking::Event<Runtime>),
    democracy(democracy::Event<Runtime>),
    council(council::Event<Runtime>),
    council_voting(council_voting::Event<Runtime>),
    council_motions(council_motions::Event<Runtime>),
    grandpa(grandpa::Event<Runtime>),
    treasury(treasury::Event<Runtime>),
    contract(contract::Event<Runtime>),
    sudo(sudo::Event<Runtime>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::clone::Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        match (&*self,) {
            (&Event::system(ref __self_0),) =>
            Event::system(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::indices(ref __self_0),) =>
            Event::indices(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::balances(ref __self_0),) =>
            Event::balances(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::session(ref __self_0),) =>
            Event::session(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::staking(ref __self_0),) =>
            Event::staking(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::democracy(ref __self_0),) =>
            Event::democracy(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::council(ref __self_0),) =>
            Event::council(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::council_voting(ref __self_0),) =>
            Event::council_voting(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::council_motions(ref __self_0),) =>
            Event::council_motions(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::grandpa(ref __self_0),) =>
            Event::grandpa(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::treasury(ref __self_0),) =>
            Event::treasury(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::contract(ref __self_0),) =>
            Event::contract(::std::clone::Clone::clone(&(*__self_0))),
            (&Event::sudo(ref __self_0),) =>
            Event::sudo(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::PartialEq for Event {
    #[inline]
    fn eq(&self, other: &Event) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::system(ref __self_0),
                     &Event::system(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::indices(ref __self_0),
                     &Event::indices(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::balances(ref __self_0),
                     &Event::balances(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::session(ref __self_0),
                     &Event::session(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::staking(ref __self_0),
                     &Event::staking(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::democracy(ref __self_0),
                     &Event::democracy(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::council(ref __self_0),
                     &Event::council(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::council_voting(ref __self_0),
                     &Event::council_voting(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::council_motions(ref __self_0),
                     &Event::council_motions(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::grandpa(ref __self_0),
                     &Event::grandpa(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::treasury(ref __self_0),
                     &Event::treasury(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::contract(ref __self_0),
                     &Event::contract(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Event::sudo(ref __self_0), &Event::sudo(ref __arg_1_0))
                    => (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &Event) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::system(ref __self_0),
                     &Event::system(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::indices(ref __self_0),
                     &Event::indices(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::balances(ref __self_0),
                     &Event::balances(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::session(ref __self_0),
                     &Event::session(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::staking(ref __self_0),
                     &Event::staking(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::democracy(ref __self_0),
                     &Event::democracy(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::council(ref __self_0),
                     &Event::council(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::council_voting(ref __self_0),
                     &Event::council_voting(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::council_motions(ref __self_0),
                     &Event::council_motions(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::grandpa(ref __self_0),
                     &Event::grandpa(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::treasury(ref __self_0),
                     &Event::treasury(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::contract(ref __self_0),
                     &Event::contract(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Event::sudo(ref __self_0), &Event::sudo(ref __arg_1_0))
                    => (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::Eq for Event {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<system::Event>;
            let _: ::std::cmp::AssertParamIsEq<indices::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<balances::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<session::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<staking::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<democracy::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<council::Event<Runtime>>;
            let _:
                    ::std::cmp::AssertParamIsEq<council_voting::Event<Runtime>>;
            let _:
                    ::std::cmp::AssertParamIsEq<council_motions::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<grandpa::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<treasury::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<contract::Event<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<sudo::Event<Runtime>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Event: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Event {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Event::system(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    Event::indices(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    Event::balances(ref aa) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                    }
                    Event::session(ref aa) => {
                        dest.push_byte(3usize as u8);
                        dest.push(aa);
                    }
                    Event::staking(ref aa) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                    }
                    Event::democracy(ref aa) => {
                        dest.push_byte(5usize as u8);
                        dest.push(aa);
                    }
                    Event::council(ref aa) => {
                        dest.push_byte(6usize as u8);
                        dest.push(aa);
                    }
                    Event::council_voting(ref aa) => {
                        dest.push_byte(7usize as u8);
                        dest.push(aa);
                    }
                    Event::council_motions(ref aa) => {
                        dest.push_byte(8usize as u8);
                        dest.push(aa);
                    }
                    Event::grandpa(ref aa) => {
                        dest.push_byte(9usize as u8);
                        dest.push(aa);
                    }
                    Event::treasury(ref aa) => {
                        dest.push_byte(10usize as u8);
                        dest.push(aa);
                    }
                    Event::contract(ref aa) => {
                        dest.push_byte(11usize as u8);
                        dest.push(aa);
                    }
                    Event::sudo(ref aa) => {
                        dest.push_byte(12usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Event: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Event {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Event::system(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(Event::indices(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(Event::balances(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 3usize as u8 => {
                        Some(Event::session(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 4usize as u8 => {
                        Some(Event::staking(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 5usize as u8 => {
                        Some(Event::democracy(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 6usize as u8 => {
                        Some(Event::council(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 7usize as u8 => {
                        Some(Event::council_voting(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 8usize as u8 => {
                        Some(Event::council_motions(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 9usize as u8 => {
                        Some(Event::grandpa(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 10usize as u8 => {
                        Some(Event::treasury(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 11usize as u8 => {
                        Some(Event::contract(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 12usize as u8 => {
                        Some(Event::sudo(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Event::system(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("system");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::indices(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("indices");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::balances(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("balances");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::session(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("session");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::staking(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("staking");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::democracy(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("democracy");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::council(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("council");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::council_voting(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("council_voting");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::council_motions(ref __self_0),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("council_motions");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::grandpa(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("grandpa");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::treasury(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("treasury");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::contract(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("contract");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::sudo(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("sudo");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl From<system::Event> for Event {
    fn from(x: system::Event) -> Self { Event::system(x) }
}
impl From<indices::Event<Runtime>> for Event {
    fn from(x: indices::Event<Runtime>) -> Self { Event::indices(x) }
}
impl From<balances::Event<Runtime>> for Event {
    fn from(x: balances::Event<Runtime>) -> Self { Event::balances(x) }
}
impl From<session::Event<Runtime>> for Event {
    fn from(x: session::Event<Runtime>) -> Self { Event::session(x) }
}
impl From<staking::Event<Runtime>> for Event {
    fn from(x: staking::Event<Runtime>) -> Self { Event::staking(x) }
}
impl From<democracy::Event<Runtime>> for Event {
    fn from(x: democracy::Event<Runtime>) -> Self { Event::democracy(x) }
}
impl From<council::Event<Runtime>> for Event {
    fn from(x: council::Event<Runtime>) -> Self { Event::council(x) }
}
impl From<council_voting::Event<Runtime>> for Event {
    fn from(x: council_voting::Event<Runtime>) -> Self {
        Event::council_voting(x)
    }
}
impl From<council_motions::Event<Runtime>> for Event {
    fn from(x: council_motions::Event<Runtime>) -> Self {
        Event::council_motions(x)
    }
}
impl From<grandpa::Event<Runtime>> for Event {
    fn from(x: grandpa::Event<Runtime>) -> Self { Event::grandpa(x) }
}
impl From<treasury::Event<Runtime>> for Event {
    fn from(x: treasury::Event<Runtime>) -> Self { Event::treasury(x) }
}
impl From<contract::Event<Runtime>> for Event {
    fn from(x: contract::Event<Runtime>) -> Self { Event::contract(x) }
}
impl From<sudo::Event<Runtime>> for Event {
    fn from(x: sudo::Event<Runtime>) -> Self { Event::sudo(x) }
}
impl Runtime {
    #[allow(dead_code)]
    pub fn outer_event_metadata()
     -> ::srml_support::event::OuterEventMetadata {
        ::srml_support::event::OuterEventMetadata{name:
                                                      ::srml_support::event::DecodeDifferent::Encode("Event"),
                                                  events:
                                                      ::srml_support::event::DecodeDifferent::Encode(&[("system",
                                                                                                        ::srml_support::event::FnEncode(system::Event::metadata)),
                                                                                                       ("indices",
                                                                                                        ::srml_support::event::FnEncode(indices::Event::<Runtime>::metadata)),
                                                                                                       ("balances",
                                                                                                        ::srml_support::event::FnEncode(balances::Event::<Runtime>::metadata)),
                                                                                                       ("session",
                                                                                                        ::srml_support::event::FnEncode(session::Event::<Runtime>::metadata)),
                                                                                                       ("staking",
                                                                                                        ::srml_support::event::FnEncode(staking::Event::<Runtime>::metadata)),
                                                                                                       ("democracy",
                                                                                                        ::srml_support::event::FnEncode(democracy::Event::<Runtime>::metadata)),
                                                                                                       ("council",
                                                                                                        ::srml_support::event::FnEncode(council::Event::<Runtime>::metadata)),
                                                                                                       ("council_voting",
                                                                                                        ::srml_support::event::FnEncode(council_voting::Event::<Runtime>::metadata)),
                                                                                                       ("council_motions",
                                                                                                        ::srml_support::event::FnEncode(council_motions::Event::<Runtime>::metadata)),
                                                                                                       ("grandpa",
                                                                                                        ::srml_support::event::FnEncode(grandpa::Event::<Runtime>::metadata)),
                                                                                                       ("treasury",
                                                                                                        ::srml_support::event::FnEncode(treasury::Event::<Runtime>::metadata)),
                                                                                                       ("contract",
                                                                                                        ::srml_support::event::FnEncode(contract::Event::<Runtime>::metadata)),
                                                                                                       ("sudo",
                                                                                                        ::srml_support::event::FnEncode(sudo::Event::<Runtime>::metadata))]),}
    }
    #[allow(dead_code)]
    pub fn __module_events_system()
     -> &'static [::srml_support::event::EventMetadata] {
        system::Event::metadata()
    }
    pub fn __module_events_indices()
     -> &'static [::srml_support::event::EventMetadata] {
        indices::Event::<Runtime>::metadata()
    }
    pub fn __module_events_balances()
     -> &'static [::srml_support::event::EventMetadata] {
        balances::Event::<Runtime>::metadata()
    }
    pub fn __module_events_session()
     -> &'static [::srml_support::event::EventMetadata] {
        session::Event::<Runtime>::metadata()
    }
    pub fn __module_events_staking()
     -> &'static [::srml_support::event::EventMetadata] {
        staking::Event::<Runtime>::metadata()
    }
    pub fn __module_events_democracy()
     -> &'static [::srml_support::event::EventMetadata] {
        democracy::Event::<Runtime>::metadata()
    }
    pub fn __module_events_council()
     -> &'static [::srml_support::event::EventMetadata] {
        council::Event::<Runtime>::metadata()
    }
    pub fn __module_events_council_voting()
     -> &'static [::srml_support::event::EventMetadata] {
        council_voting::Event::<Runtime>::metadata()
    }
    pub fn __module_events_council_motions()
     -> &'static [::srml_support::event::EventMetadata] {
        council_motions::Event::<Runtime>::metadata()
    }
    pub fn __module_events_grandpa()
     -> &'static [::srml_support::event::EventMetadata] {
        grandpa::Event::<Runtime>::metadata()
    }
    pub fn __module_events_treasury()
     -> &'static [::srml_support::event::EventMetadata] {
        treasury::Event::<Runtime>::metadata()
    }
    pub fn __module_events_contract()
     -> &'static [::srml_support::event::EventMetadata] {
        contract::Event::<Runtime>::metadata()
    }
    pub fn __module_events_sudo()
     -> &'static [::srml_support::event::EventMetadata] {
        sudo::Event::<Runtime>::metadata()
    }
}
#[allow(non_camel_case_types)]
#[structural_match]
pub enum Origin {
    system(system::Origin<Runtime>),
    council_motions(council_motions::Origin),

    #[allow(dead_code)]
    Void(::srml_support::Void),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::clone::Clone for Origin {
    #[inline]
    fn clone(&self) -> Origin {
        match (&*self,) {
            (&Origin::system(ref __self_0),) =>
            Origin::system(::std::clone::Clone::clone(&(*__self_0))),
            (&Origin::council_motions(ref __self_0),) =>
            Origin::council_motions(::std::clone::Clone::clone(&(*__self_0))),
            (&Origin::Void(ref __self_0),) =>
            Origin::Void(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::PartialEq for Origin {
    #[inline]
    fn eq(&self, other: &Origin) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Origin::system(ref __self_0),
                     &Origin::system(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Origin::council_motions(ref __self_0),
                     &Origin::council_motions(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Origin::Void(ref __self_0),
                     &Origin::Void(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &Origin) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Origin::system(ref __self_0),
                     &Origin::system(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Origin::council_motions(ref __self_0),
                     &Origin::council_motions(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Origin::Void(ref __self_0),
                     &Origin::Void(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::Eq for Origin {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<system::Origin<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<council_motions::Origin>;
            let _: ::std::cmp::AssertParamIsEq<::srml_support::Void>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::fmt::Debug for Origin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Origin::system(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("system");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Origin::council_motions(ref __self_0),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("council_motions");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Origin::Void(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Void");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(dead_code)]
impl Origin {
    pub const
    NONE:
    Self
    =
    Origin::system(system::RawOrigin::None);
    pub const
    ROOT:
    Self
    =
    Origin::system(system::RawOrigin::Root);
    pub fn signed(by: <Runtime as system::Trait>::AccountId) -> Self {
        Origin::system(system::RawOrigin::Signed(by))
    }
}
impl From<system::Origin<Runtime>> for Origin {
    fn from(x: system::Origin<Runtime>) -> Self { Origin::system(x) }
}
impl Into<Option<system::Origin<Runtime>>> for Origin {
    fn into(self) -> Option<system::Origin<Runtime>> {
        if let Origin::system(l) = self { Some(l) } else { None }
    }
}
impl From<Option<<Runtime as system::Trait>::AccountId>> for Origin {
    fn from(x: Option<<Runtime as system::Trait>::AccountId>) -> Self {
        <system::Origin<Runtime>>::from(x).into()
    }
}
impl From<council_motions::Origin> for Origin {
    fn from(x: council_motions::Origin) -> Self { Origin::council_motions(x) }
}
impl Into<Option<council_motions::Origin>> for Origin {
    fn into(self) -> Option<council_motions::Origin> {
        if let Origin::council_motions(l) = self { Some(l) } else { None }
    }
}
pub type System = system::Module<Runtime>;
pub type Aura = aura::Module<Runtime>;
pub type Timestamp = timestamp::Module<Runtime>;
pub type Consensus = consensus::Module<Runtime>;
pub type Indices = indices::Module<Runtime>;
pub type Balances = balances::Module<Runtime>;
pub type Session = session::Module<Runtime>;
pub type Staking = staking::Module<Runtime>;
pub type Democracy = democracy::Module<Runtime>;
pub type Council = council::Module<Runtime>;
pub type CouncilVoting = council_voting::Module<Runtime>;
pub type CouncilMotions = council_motions::Module<Runtime>;
pub type FinalityTracker = finality_tracker::Module<Runtime>;
pub type Grandpa = grandpa::Module<Runtime>;
pub type Treasury = treasury::Module<Runtime>;
pub type Contract = contract::Module<Runtime>;
pub type Sudo = sudo::Module<Runtime>;
type AllModules
    =
    (Aura, Timestamp, Consensus, Indices, Balances, Session, Staking,
     Democracy, Council, CouncilVoting, CouncilMotions, FinalityTracker,
     Grandpa, Treasury, Contract, Sudo);
#[structural_match]
pub enum Call {
    Timestamp(::srml_support::dispatch::CallableCallFor<Timestamp>),
    Consensus(::srml_support::dispatch::CallableCallFor<Consensus>),
    Indices(::srml_support::dispatch::CallableCallFor<Indices>),
    Balances(::srml_support::dispatch::CallableCallFor<Balances>),
    Session(::srml_support::dispatch::CallableCallFor<Session>),
    Staking(::srml_support::dispatch::CallableCallFor<Staking>),
    Democracy(::srml_support::dispatch::CallableCallFor<Democracy>),
    Council(::srml_support::dispatch::CallableCallFor<Council>),
    CouncilVoting(::srml_support::dispatch::CallableCallFor<CouncilVoting>),
    CouncilMotions(::srml_support::dispatch::CallableCallFor<CouncilMotions>),
    FinalityTracker(::srml_support::dispatch::CallableCallFor<FinalityTracker>),
    Grandpa(::srml_support::dispatch::CallableCallFor<Grandpa>),
    Treasury(::srml_support::dispatch::CallableCallFor<Treasury>),
    Contract(::srml_support::dispatch::CallableCallFor<Contract>),
    Sudo(::srml_support::dispatch::CallableCallFor<Sudo>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Call {
    #[inline]
    fn clone(&self) -> Call {
        match (&*self,) {
            (&Call::Timestamp(ref __self_0),) =>
            Call::Timestamp(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Consensus(ref __self_0),) =>
            Call::Consensus(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Indices(ref __self_0),) =>
            Call::Indices(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Balances(ref __self_0),) =>
            Call::Balances(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Session(ref __self_0),) =>
            Call::Session(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Staking(ref __self_0),) =>
            Call::Staking(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Democracy(ref __self_0),) =>
            Call::Democracy(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Council(ref __self_0),) =>
            Call::Council(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::CouncilVoting(ref __self_0),) =>
            Call::CouncilVoting(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::CouncilMotions(ref __self_0),) =>
            Call::CouncilMotions(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::FinalityTracker(ref __self_0),) =>
            Call::FinalityTracker(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Grandpa(ref __self_0),) =>
            Call::Grandpa(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Treasury(ref __self_0),) =>
            Call::Treasury(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Contract(ref __self_0),) =>
            Call::Contract(::std::clone::Clone::clone(&(*__self_0))),
            (&Call::Sudo(ref __self_0),) =>
            Call::Sudo(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Call {
    #[inline]
    fn eq(&self, other: &Call) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::Timestamp(ref __self_0),
                     &Call::Timestamp(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Consensus(ref __self_0),
                     &Call::Consensus(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Indices(ref __self_0),
                     &Call::Indices(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Balances(ref __self_0),
                     &Call::Balances(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Session(ref __self_0),
                     &Call::Session(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Staking(ref __self_0),
                     &Call::Staking(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Democracy(ref __self_0),
                     &Call::Democracy(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Council(ref __self_0),
                     &Call::Council(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::CouncilVoting(ref __self_0),
                     &Call::CouncilVoting(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::CouncilMotions(ref __self_0),
                     &Call::CouncilMotions(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::FinalityTracker(ref __self_0),
                     &Call::FinalityTracker(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Grandpa(ref __self_0),
                     &Call::Grandpa(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Treasury(ref __self_0),
                     &Call::Treasury(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Contract(ref __self_0),
                     &Call::Contract(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &Call) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::Timestamp(ref __self_0),
                     &Call::Timestamp(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Consensus(ref __self_0),
                     &Call::Consensus(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Indices(ref __self_0),
                     &Call::Indices(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Balances(ref __self_0),
                     &Call::Balances(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Session(ref __self_0),
                     &Call::Session(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Staking(ref __self_0),
                     &Call::Staking(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Democracy(ref __self_0),
                     &Call::Democracy(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Council(ref __self_0),
                     &Call::Council(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::CouncilVoting(ref __self_0),
                     &Call::CouncilVoting(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::CouncilMotions(ref __self_0),
                     &Call::CouncilMotions(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::FinalityTracker(ref __self_0),
                     &Call::FinalityTracker(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Grandpa(ref __self_0),
                     &Call::Grandpa(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Treasury(ref __self_0),
                     &Call::Treasury(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Contract(ref __self_0),
                     &Call::Contract(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Call {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Timestamp>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Consensus>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Indices>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Balances>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Session>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Staking>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Democracy>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Council>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<CouncilVoting>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<CouncilMotions>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<FinalityTracker>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Grandpa>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Treasury>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Contract>>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Sudo>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Call {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::Timestamp(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    Call::Consensus(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    Call::Indices(ref aa) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                    }
                    Call::Balances(ref aa) => {
                        dest.push_byte(3usize as u8);
                        dest.push(aa);
                    }
                    Call::Session(ref aa) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                    }
                    Call::Staking(ref aa) => {
                        dest.push_byte(5usize as u8);
                        dest.push(aa);
                    }
                    Call::Democracy(ref aa) => {
                        dest.push_byte(6usize as u8);
                        dest.push(aa);
                    }
                    Call::Council(ref aa) => {
                        dest.push_byte(7usize as u8);
                        dest.push(aa);
                    }
                    Call::CouncilVoting(ref aa) => {
                        dest.push_byte(8usize as u8);
                        dest.push(aa);
                    }
                    Call::CouncilMotions(ref aa) => {
                        dest.push_byte(9usize as u8);
                        dest.push(aa);
                    }
                    Call::FinalityTracker(ref aa) => {
                        dest.push_byte(10usize as u8);
                        dest.push(aa);
                    }
                    Call::Grandpa(ref aa) => {
                        dest.push_byte(11usize as u8);
                        dest.push(aa);
                    }
                    Call::Treasury(ref aa) => {
                        dest.push_byte(12usize as u8);
                        dest.push(aa);
                    }
                    Call::Contract(ref aa) => {
                        dest.push_byte(13usize as u8);
                        dest.push(aa);
                    }
                    Call::Sudo(ref aa) => {
                        dest.push_byte(14usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Call {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::Timestamp(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::Consensus(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(Call::Indices(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 3usize as u8 => {
                        Some(Call::Balances(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 4usize as u8 => {
                        Some(Call::Session(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 5usize as u8 => {
                        Some(Call::Staking(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 6usize as u8 => {
                        Some(Call::Democracy(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 7usize as u8 => {
                        Some(Call::Council(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 8usize as u8 => {
                        Some(Call::CouncilVoting(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 9usize as u8 => {
                        Some(Call::CouncilMotions(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 10usize as u8 => {
                        Some(Call::FinalityTracker(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 11usize as u8 => {
                        Some(Call::Grandpa(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 12usize as u8 => {
                        Some(Call::Treasury(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 13usize as u8 => {
                        Some(Call::Contract(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 14usize as u8 => {
                        Some(Call::Sudo(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Call {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Call::Timestamp(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Timestamp");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Consensus(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Consensus");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Indices(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Indices");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Balances(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Balances");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Session(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Session");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Staking(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Staking");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Democracy(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Democracy");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Council(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Council");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::CouncilVoting(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("CouncilVoting");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::CouncilMotions(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("CouncilMotions");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::FinalityTracker(ref __self_0),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("FinalityTracker");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Grandpa(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Grandpa");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Treasury(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Treasury");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Contract(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Contract");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Sudo(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Sudo");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl ::srml_support::dispatch::Dispatchable for Call {
    type
    Origin
    =
    Origin;
    type
    Trait
    =
    Call;
    fn dispatch(self, origin: Origin) -> ::srml_support::dispatch::Result {
        match self {
            Call::Timestamp(call) => call.dispatch(origin),
            Call::Consensus(call) => call.dispatch(origin),
            Call::Indices(call) => call.dispatch(origin),
            Call::Balances(call) => call.dispatch(origin),
            Call::Session(call) => call.dispatch(origin),
            Call::Staking(call) => call.dispatch(origin),
            Call::Democracy(call) => call.dispatch(origin),
            Call::Council(call) => call.dispatch(origin),
            Call::CouncilVoting(call) => call.dispatch(origin),
            Call::CouncilMotions(call) => call.dispatch(origin),
            Call::FinalityTracker(call) => call.dispatch(origin),
            Call::Grandpa(call) => call.dispatch(origin),
            Call::Treasury(call) => call.dispatch(origin),
            Call::Contract(call) => call.dispatch(origin),
            Call::Sudo(call) => call.dispatch(origin),
        }
    }
}
impl ::srml_support::dispatch::IsSubType<Timestamp> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Timestamp as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Timestamp(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Consensus> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Consensus as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Consensus(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Indices> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Indices as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Indices(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Balances> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Balances as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Balances(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Session> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Session as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Session(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Staking> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Staking as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Staking(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Democracy> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Democracy as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Democracy(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Council> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Council as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Council(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<CouncilVoting> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<CouncilVoting as ::srml_support::dispatch::Callable>::Call> {
        if let Call::CouncilVoting(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<CouncilMotions> for Call {
    fn is_aux_sub_type(&self)
     ->
         Option<&<CouncilMotions as
                 ::srml_support::dispatch::Callable>::Call> {
        if let Call::CouncilMotions(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<FinalityTracker> for Call {
    fn is_aux_sub_type(&self)
     ->
         Option<&<FinalityTracker as
                 ::srml_support::dispatch::Callable>::Call> {
        if let Call::FinalityTracker(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Grandpa> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Grandpa as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Grandpa(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Treasury> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Treasury as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Treasury(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Contract> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Contract as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Contract(ref r) = *self { Some(r) } else { None }
    }
}
impl ::srml_support::dispatch::IsSubType<Sudo> for Call {
    fn is_aux_sub_type(&self)
     -> Option<&<Sudo as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Sudo(ref r) = *self { Some(r) } else { None }
    }
}
impl Runtime {
    pub fn metadata() -> ::srml_support::metadata::RuntimeMetadataPrefixed {
        ::srml_support::metadata::RuntimeMetadata::V4(::srml_support::metadata::RuntimeMetadataV4{modules:
                                                                                                      ::srml_support::metadata::DecodeDifferent::Encode(&[::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("system"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(system::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(system::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       None,
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ system > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_system
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_system
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("aura"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(||
                                                                                                                                                                                                                                                                                                "")),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       None,
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       None,
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       None,},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("timestamp"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(timestamp::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(timestamp::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(timestamp::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       None,},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("consensus"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(consensus::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(consensus::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(consensus::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       None,},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("indices"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(indices::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(indices::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(indices::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ indices > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_indices
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_indices
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("balances"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(balances::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(balances::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(balances::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ balances > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_balances
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_balances
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("session"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(session::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(session::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(session::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ session > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_session
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_session
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("staking"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(staking::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(staking::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(staking::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ staking > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_staking
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_staking
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("democracy"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(democracy::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(democracy::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(democracy::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ democracy > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_democracy
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_democracy
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("council"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(council::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(council::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(council::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ council > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_council
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_council
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("council_voting"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(council_voting::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(council_voting::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(council_voting::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ council_voting > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_council_voting
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_council_voting
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("council_motions"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(council_motions::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(council_motions::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(council_motions::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ council_motions > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_council_motions
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_council_motions
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("finality_tracker"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(||
                                                                                                                                                                                                                                                                                                "")),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       None,
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(finality_tracker::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       None,},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("grandpa"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(grandpa::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(grandpa::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(grandpa::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ grandpa > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_grandpa
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_grandpa
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("treasury"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(treasury::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(treasury::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(treasury::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ treasury > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_treasury
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_treasury
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("contract"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(contract::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(contract::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(contract::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ contract > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_contract
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_contract
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),},
                                                                                                                                                          ::srml_support::metadata::ModuleMetadata{name:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode("sudo"),
                                                                                                                                                                                                   prefix:
                                                                                                                                                                                                       ::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(sudo::Module::<Runtime>::store_metadata_name)),
                                                                                                                                                                                                   storage:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(sudo::Module::<Runtime>::store_metadata_functions))),
                                                                                                                                                                                                   calls:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode(sudo::Module::<Runtime>::call_functions))),
                                                                                                                                                                                                   event:
                                                                                                                                                                                                       Some(::srml_support::metadata::DecodeDifferent::Encode(::srml_support::metadata::FnEncode({
                                                                                                                                                                                                                                                                                                     enum ProcMacroHack
                                                                                                                                                                                                                                                                                                          {
                                                                                                                                                                                                                                                                                                         Value
                                                                                                                                                                                                                                                                                                             =
                                                                                                                                                                                                                                                                                                             ("Runtime :: [ < __module_events_ sudo > ]",
                                                                                                                                                                                                                                                                                                              0).1,
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                     macro_rules! proc_macro_call((

                                                                                                                                                                                                                                                                                                                                  )
                                                                                                                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                                                                                                                  Runtime
                                                                                                                                                                                                                                                                                                                                  ::
                                                                                                                                                                                                                                                                                                                                  __module_events_sudo
                                                                                                                                                                                                                                                                                                                                  }
                                                                                                                                                                                                                                                                                                                                  });
                                                                                                                                                                                                                                                                                                     {
                                                                                                                                                                                                                                                                                                         Runtime::__module_events_sudo
                                                                                                                                                                                                                                                                                                     }
                                                                                                                                                                                                                                                                                                 }))),}]),}).into()
    }
}
/// Wrapper for all possible log entries for the `$trait` runtime. Provides binary-compatible
/// `Encode`/`Decode` implementations with the corresponding `generic::DigestItem`.
#[allow(non_camel_case_types)]
#[structural_match]
pub struct Log(InternalLog);
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::clone::Clone for Log {
    #[inline]
    fn clone(&self) -> Log {
        match *self {
            Log(ref __self_0_0) =>
            Log(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::PartialEq for Log {
    #[inline]
    fn eq(&self, other: &Log) -> bool {
        match *other {
            Log(ref __self_1_0) =>
            match *self {
                Log(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Log) -> bool {
        match *other {
            Log(ref __self_1_0) =>
            match *self {
                Log(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::Eq for Log {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<InternalLog>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::fmt::Debug for Log {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Log(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Log");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Log: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl _serde::Serialize for Log {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "Log", &self.0)
            }
        }
    };
/// All possible log entries for the `$trait` runtime. `Encode`/`Decode` implementations
/// are auto-generated => it is not binary-compatible with `generic::DigestItem`.
#[allow(non_camel_case_types)]
#[structural_match]
pub enum InternalLog {
    system(system::Log<Runtime>),
    consensus(consensus::Log<Runtime>),
    grandpa(grandpa::Log<Runtime>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::clone::Clone for InternalLog {
    #[inline]
    fn clone(&self) -> InternalLog {
        match (&*self,) {
            (&InternalLog::system(ref __self_0),) =>
            InternalLog::system(::std::clone::Clone::clone(&(*__self_0))),
            (&InternalLog::consensus(ref __self_0),) =>
            InternalLog::consensus(::std::clone::Clone::clone(&(*__self_0))),
            (&InternalLog::grandpa(ref __self_0),) =>
            InternalLog::grandpa(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::PartialEq for InternalLog {
    #[inline]
    fn eq(&self, other: &InternalLog) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&InternalLog::system(ref __self_0),
                     &InternalLog::system(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&InternalLog::consensus(ref __self_0),
                     &InternalLog::consensus(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&InternalLog::grandpa(ref __self_0),
                     &InternalLog::grandpa(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &InternalLog) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&InternalLog::system(ref __self_0),
                     &InternalLog::system(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&InternalLog::consensus(ref __self_0),
                     &InternalLog::consensus(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&InternalLog::grandpa(ref __self_0),
                     &InternalLog::grandpa(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::cmp::Eq for InternalLog {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<system::Log<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<consensus::Log<Runtime>>;
            let _: ::std::cmp::AssertParamIsEq<grandpa::Log<Runtime>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_InternalLog: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for InternalLog {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    InternalLog::system(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    InternalLog::consensus(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    InternalLog::grandpa(ref aa) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_InternalLog: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for InternalLog {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(InternalLog::system(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(InternalLog::consensus(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(InternalLog::grandpa(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::std::fmt::Debug for InternalLog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&InternalLog::system(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("system");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&InternalLog::consensus(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("consensus");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&InternalLog::grandpa(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("grandpa");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_InternalLog: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl _serde::Serialize for InternalLog {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    InternalLog::system(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "InternalLog",
                                                                  0u32,
                                                                  "system",
                                                                  __field0),
                    InternalLog::consensus(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "InternalLog",
                                                                  1u32,
                                                                  "consensus",
                                                                  __field0),
                    InternalLog::grandpa(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "InternalLog",
                                                                  2u32,
                                                                  "grandpa",
                                                                  __field0),
                }
            }
        }
    };
impl Log {
    /// Try to convert `$name` into `generic::DigestItemRef`. Returns Some when
    /// `self` is a 'system' log && it has been marked as 'system' in macro call.
    /// Otherwise, None is returned.
    #[allow(unreachable_patterns)]
    fn dref<'a>(&'a self)
     ->
         Option<::sr_primitives::generic::DigestItemRef<'a, Hash, AuthorityId,
                                                        AuthoritySignature>> {
        match self.0 {
            InternalLog::system(system::RawLog::ChangesTrieRoot(ref v)) =>
            Some(::sr_primitives::generic::DigestItemRef::ChangesTrieRoot(v)),
            InternalLog::consensus(consensus::RawLog::AuthoritiesChange(ref v))
            =>
            Some(::sr_primitives::generic::DigestItemRef::AuthoritiesChange(v)),
            _ => None,
        }
    }
}
impl ::sr_primitives::traits::DigestItem for Log {
    type
    Hash
    =
    <::sr_primitives::generic::DigestItem<Hash, AuthorityId,
                                          AuthoritySignature> as
    ::sr_primitives::traits::DigestItem>::Hash;
    type
    AuthorityId
    =
    <::sr_primitives::generic::DigestItem<Hash, AuthorityId,
                                          AuthoritySignature> as
    ::sr_primitives::traits::DigestItem>::AuthorityId;
    fn as_authorities_change(&self) -> Option<&[Self::AuthorityId]> {
        self.dref().and_then(|dref| dref.as_authorities_change())
    }
    fn as_changes_trie_root(&self) -> Option<&Self::Hash> {
        self.dref().and_then(|dref| dref.as_changes_trie_root())
    }
}
impl From<::sr_primitives::generic::DigestItem<Hash, AuthorityId,
                                               AuthoritySignature>> for Log {
    /// Converts `generic::DigestItem` into `$name`. If `generic::DigestItem` represents
    /// a system item which is supported by the runtime, it is returned.
    /// Otherwise we expect a `Other` log item. Trying to convert from anything other
    /// will lead to panic in runtime, since the runtime does not supports this 'system'
    /// log item.
    #[allow(unreachable_patterns)]
    fn from(gen:
                ::sr_primitives::generic::DigestItem<Hash, AuthorityId,
                                                     AuthoritySignature>)
     -> Self {
        match gen {
            ::sr_primitives::generic::DigestItem::ChangesTrieRoot(value) =>
            Log(InternalLog::system(system::RawLog::ChangesTrieRoot(value))),
            ::sr_primitives::generic::DigestItem::AuthoritiesChange(value) =>
            Log(InternalLog::consensus(consensus::RawLog::AuthoritiesChange(value))),
            _ =>
            gen.as_other().and_then(|value|
                                        ::sr_primitives::codec::Decode::decode(&mut &value[..])).map(Log).expect("not allowed to fail in runtime"),
        }
    }
}
impl ::sr_primitives::codec::Decode for Log {
    /// `generic::DigestItem` binary compatible decode.
    fn decode<I: ::sr_primitives::codec::Input>(input: &mut I)
     -> Option<Self> {
        let gen:
                ::sr_primitives::generic::DigestItem<Hash, AuthorityId,
                                                     AuthoritySignature> =
            ::sr_primitives::codec::Decode::decode(input)?;
        Some(Log::from(gen))
    }
}
impl ::sr_primitives::codec::Encode for Log {
    /// `generic::DigestItem` binary compatible encode.
    fn encode(&self) -> Vec<u8> {
        match self.dref() {
            Some(dref) => dref.encode(),
            None => {
                let gen:
                        ::sr_primitives::generic::DigestItem<Hash,
                                                             AuthorityId,
                                                             AuthoritySignature> =
                    ::sr_primitives::generic::DigestItem::Other(self.0.encode());
                gen.encode()
            }
        }
    }
}
impl From<system::Log<Runtime>> for Log {
    /// Converts single module log item into `$name`.
    fn from(x: system::Log<Runtime>) -> Self { Log(x.into()) }
}
impl From<system::Log<Runtime>> for InternalLog {
    /// Converts single module log item into `$internal`.
    fn from(x: system::Log<Runtime>) -> Self { InternalLog::system(x) }
}
impl From<consensus::Log<Runtime>> for Log {
    /// Converts single module log item into `$name`.
    fn from(x: consensus::Log<Runtime>) -> Self { Log(x.into()) }
}
impl From<consensus::Log<Runtime>> for InternalLog {
    /// Converts single module log item into `$internal`.
    fn from(x: consensus::Log<Runtime>) -> Self { InternalLog::consensus(x) }
}
impl From<grandpa::Log<Runtime>> for Log {
    /// Converts single module log item into `$name`.
    fn from(x: grandpa::Log<Runtime>) -> Self { Log(x.into()) }
}
impl From<grandpa::Log<Runtime>> for InternalLog {
    /// Converts single module log item into `$internal`.
    fn from(x: grandpa::Log<Runtime>) -> Self { InternalLog::grandpa(x) }
}
#[cfg(any(feature = "std", test))]
pub type SystemConfig = system::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type TimestampConfig = timestamp::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type ConsensusConfig = consensus::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type IndicesConfig = indices::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type BalancesConfig = balances::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type SessionConfig = session::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type StakingConfig = staking::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type DemocracyConfig = democracy::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type CouncilVotingConfig = council_voting::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type CouncilSeatsConfig = council_seats::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type GrandpaConfig = grandpa::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type TreasuryConfig = treasury::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type ContractConfig = contract::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type SudoConfig = sudo::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GenesisConfig {
    pub system: Option<SystemConfig>,
    pub timestamp: Option<TimestampConfig>,
    pub consensus: Option<ConsensusConfig>,
    pub indices: Option<IndicesConfig>,
    pub balances: Option<BalancesConfig>,
    pub session: Option<SessionConfig>,
    pub staking: Option<StakingConfig>,
    pub democracy: Option<DemocracyConfig>,
    pub council_voting: Option<CouncilVotingConfig>,
    pub council_seats: Option<CouncilSeatsConfig>,
    pub grandpa: Option<GrandpaConfig>,
    pub treasury: Option<TreasuryConfig>,
    pub contract: Option<ContractConfig>,
    pub sudo: Option<SudoConfig>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_GenesisConfig: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl _serde::Serialize for GenesisConfig {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "GenesisConfig",
                                                               false as usize
                                                                   + 1 + 1 + 1
                                                                   + 1 + 1 + 1
                                                                   + 1 + 1 + 1
                                                                   + 1 + 1 + 1
                                                                   + 1 + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "system",
                                                                    &self.system)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "timestamp",
                                                                    &self.timestamp)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "consensus",
                                                                    &self.consensus)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "indices",
                                                                    &self.indices)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "balances",
                                                                    &self.balances)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "session",
                                                                    &self.session)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "staking",
                                                                    &self.staking)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "democracy",
                                                                    &self.democracy)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "councilVoting",
                                                                    &self.council_voting)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "councilSeats",
                                                                    &self.council_seats)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "grandpa",
                                                                    &self.grandpa)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "treasury",
                                                                    &self.treasury)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "contract",
                                                                    &self.contract)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "sudo",
                                                                    &self.sudo)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_GenesisConfig: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[allow(unused_macros)]
        macro_rules! try(( $ __expr : expr ) => {
                         match $ __expr {
                         _serde :: export :: Ok ( __val ) => __val , _serde ::
                         export :: Err ( __err ) => {
                         return _serde :: export :: Err ( __err ) ; } } });
        #[automatically_derived]
        impl <'de> _serde::Deserialize<'de> for GenesisConfig {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                }
                struct __FieldVisitor;
                impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type
                    Value
                    =
                    __Field;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            2u64 => _serde::export::Ok(__Field::__field2),
                            3u64 => _serde::export::Ok(__Field::__field3),
                            4u64 => _serde::export::Ok(__Field::__field4),
                            5u64 => _serde::export::Ok(__Field::__field5),
                            6u64 => _serde::export::Ok(__Field::__field6),
                            7u64 => _serde::export::Ok(__Field::__field7),
                            8u64 => _serde::export::Ok(__Field::__field8),
                            9u64 => _serde::export::Ok(__Field::__field9),
                            10u64 => _serde::export::Ok(__Field::__field10),
                            11u64 => _serde::export::Ok(__Field::__field11),
                            12u64 => _serde::export::Ok(__Field::__field12),
                            13u64 => _serde::export::Ok(__Field::__field13),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 14")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "system" => _serde::export::Ok(__Field::__field0),
                            "timestamp" =>
                            _serde::export::Ok(__Field::__field1),
                            "consensus" =>
                            _serde::export::Ok(__Field::__field2),
                            "indices" =>
                            _serde::export::Ok(__Field::__field3),
                            "balances" =>
                            _serde::export::Ok(__Field::__field4),
                            "session" =>
                            _serde::export::Ok(__Field::__field5),
                            "staking" =>
                            _serde::export::Ok(__Field::__field6),
                            "democracy" =>
                            _serde::export::Ok(__Field::__field7),
                            "councilVoting" =>
                            _serde::export::Ok(__Field::__field8),
                            "councilSeats" =>
                            _serde::export::Ok(__Field::__field9),
                            "grandpa" =>
                            _serde::export::Ok(__Field::__field10),
                            "treasury" =>
                            _serde::export::Ok(__Field::__field11),
                            "contract" =>
                            _serde::export::Ok(__Field::__field12),
                            "sudo" => _serde::export::Ok(__Field::__field13),
                            _ => {
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"system" =>
                            _serde::export::Ok(__Field::__field0),
                            b"timestamp" =>
                            _serde::export::Ok(__Field::__field1),
                            b"consensus" =>
                            _serde::export::Ok(__Field::__field2),
                            b"indices" =>
                            _serde::export::Ok(__Field::__field3),
                            b"balances" =>
                            _serde::export::Ok(__Field::__field4),
                            b"session" =>
                            _serde::export::Ok(__Field::__field5),
                            b"staking" =>
                            _serde::export::Ok(__Field::__field6),
                            b"democracy" =>
                            _serde::export::Ok(__Field::__field7),
                            b"councilVoting" =>
                            _serde::export::Ok(__Field::__field8),
                            b"councilSeats" =>
                            _serde::export::Ok(__Field::__field9),
                            b"grandpa" =>
                            _serde::export::Ok(__Field::__field10),
                            b"treasury" =>
                            _serde::export::Ok(__Field::__field11),
                            b"contract" =>
                            _serde::export::Ok(__Field::__field12),
                            b"sudo" => _serde::export::Ok(__Field::__field13),
                            _ => {
                                let __value =
                                    &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_field(__value,
                                                                                     FIELDS))
                            }
                        }
                    }
                }
                impl <'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        _serde::Deserializer::deserialize_identifier(__deserializer,
                                                                     __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<GenesisConfig>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    GenesisConfig;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct GenesisConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Option<SystemConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Option<TimestampConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Option<ConsensusConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<Option<IndicesConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<Option<BalancesConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<Option<SessionConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<Option<StakingConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(6usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<Option<DemocracyConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(7usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<Option<CouncilVotingConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(8usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field9 =
                            match match _serde::de::SeqAccess::next_element::<Option<CouncilSeatsConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(9usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field10 =
                            match match _serde::de::SeqAccess::next_element::<Option<GrandpaConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(10usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field11 =
                            match match _serde::de::SeqAccess::next_element::<Option<TreasuryConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(11usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field12 =
                            match match _serde::de::SeqAccess::next_element::<Option<ContractConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(12usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        let __field13 =
                            match match _serde::de::SeqAccess::next_element::<Option<SudoConfig>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(13usize,
                                                                                                 &"struct GenesisConfig with 14 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{system: __field0,
                                                         timestamp: __field1,
                                                         consensus: __field2,
                                                         indices: __field3,
                                                         balances: __field4,
                                                         session: __field5,
                                                         staking: __field6,
                                                         democracy: __field7,
                                                         council_voting:
                                                             __field8,
                                                         council_seats:
                                                             __field9,
                                                         grandpa: __field10,
                                                         treasury: __field11,
                                                         contract: __field12,
                                                         sudo: __field13,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Option<SystemConfig>> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<Option<TimestampConfig>> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<Option<ConsensusConfig>> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<Option<IndicesConfig>> =
                            _serde::export::None;
                        let mut __field4:
                                _serde::export::Option<Option<BalancesConfig>> =
                            _serde::export::None;
                        let mut __field5:
                                _serde::export::Option<Option<SessionConfig>> =
                            _serde::export::None;
                        let mut __field6:
                                _serde::export::Option<Option<StakingConfig>> =
                            _serde::export::None;
                        let mut __field7:
                                _serde::export::Option<Option<DemocracyConfig>> =
                            _serde::export::None;
                        let mut __field8:
                                _serde::export::Option<Option<CouncilVotingConfig>> =
                            _serde::export::None;
                        let mut __field9:
                                _serde::export::Option<Option<CouncilSeatsConfig>> =
                            _serde::export::None;
                        let mut __field10:
                                _serde::export::Option<Option<GrandpaConfig>> =
                            _serde::export::None;
                        let mut __field11:
                                _serde::export::Option<Option<TreasuryConfig>> =
                            _serde::export::None;
                        let mut __field12:
                                _serde::export::Option<Option<ContractConfig>> =
                            _serde::export::None;
                        let mut __field13:
                                _serde::export::Option<Option<SudoConfig>> =
                            _serde::export::None;
                        while let _serde::export::Some(__key) =
                                  match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::export::Option::is_some(&__field0)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("system"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<SystemConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("timestamp"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<TimestampConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("consensus"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<ConsensusConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field3 => {
                                    if _serde::export::Option::is_some(&__field3)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("indices"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<IndicesConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("balances"));
                                    }
                                    __field4 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<BalancesConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("session"));
                                    }
                                    __field5 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<SessionConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("staking"));
                                    }
                                    __field6 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<StakingConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("democracy"));
                                    }
                                    __field7 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<DemocracyConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field8 => {
                                    if _serde::export::Option::is_some(&__field8)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("councilVoting"));
                                    }
                                    __field8 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<CouncilVotingConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field9 => {
                                    if _serde::export::Option::is_some(&__field9)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("councilSeats"));
                                    }
                                    __field9 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<CouncilSeatsConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field10 => {
                                    if _serde::export::Option::is_some(&__field10)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("grandpa"));
                                    }
                                    __field10 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<GrandpaConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field11 => {
                                    if _serde::export::Option::is_some(&__field11)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("treasury"));
                                    }
                                    __field11 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<TreasuryConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field12 => {
                                    if _serde::export::Option::is_some(&__field12)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("contract"));
                                    }
                                    __field12 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<ContractConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field13 => {
                                    if _serde::export::Option::is_some(&__field13)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("sudo"));
                                    }
                                    __field13 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<SudoConfig>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("system")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("timestamp")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("consensus")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field3 =
                            match __field3 {
                                _serde::export::Some(__field3) => __field3,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("indices")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field4 =
                            match __field4 {
                                _serde::export::Some(__field4) => __field4,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("balances")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field5 =
                            match __field5 {
                                _serde::export::Some(__field5) => __field5,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("session")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field6 =
                            match __field6 {
                                _serde::export::Some(__field6) => __field6,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("staking")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field7 =
                            match __field7 {
                                _serde::export::Some(__field7) => __field7,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("democracy")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field8 =
                            match __field8 {
                                _serde::export::Some(__field8) => __field8,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("councilVoting")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field9 =
                            match __field9 {
                                _serde::export::Some(__field9) => __field9,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("councilSeats")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field10 =
                            match __field10 {
                                _serde::export::Some(__field10) => __field10,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("grandpa")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field11 =
                            match __field11 {
                                _serde::export::Some(__field11) => __field11,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("treasury")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field12 =
                            match __field12 {
                                _serde::export::Some(__field12) => __field12,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("contract")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field13 =
                            match __field13 {
                                _serde::export::Some(__field13) => __field13,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("sudo")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{system: __field0,
                                                         timestamp: __field1,
                                                         consensus: __field2,
                                                         indices: __field3,
                                                         balances: __field4,
                                                         session: __field5,
                                                         staking: __field6,
                                                         democracy: __field7,
                                                         council_voting:
                                                             __field8,
                                                         council_seats:
                                                             __field9,
                                                         grandpa: __field10,
                                                         treasury: __field11,
                                                         contract: __field12,
                                                         sudo: __field13,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["system", "timestamp", "consensus", "indices",
                      "balances", "session", "staking", "democracy",
                      "councilVoting", "councilSeats", "grandpa", "treasury",
                      "contract", "sudo"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "GenesisConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<GenesisConfig>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[cfg(any(feature = "std", test))]
impl ::sr_primitives::BuildStorage for GenesisConfig {
    fn assimilate_storage(self, top: &mut ::sr_primitives::StorageOverlay,
                          children:
                              &mut ::sr_primitives::ChildrenStorageOverlay)
     -> ::std::result::Result<(), String> {
        if let Some(extra) = self.system {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.timestamp {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.consensus {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.indices {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.balances {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.session {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.staking {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.democracy {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.council_voting {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.council_seats {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.grandpa {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.treasury {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.contract {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.sudo {
            extra.assimilate_storage(top, children)?;
        }
        Ok(())
    }
}
trait InherentDataExt {
    fn create_extrinsics(&self)
    ->
        ::srml_support::inherent::Vec<<Block as
                                      ::srml_support::inherent::BlockT>::Extrinsic>;
    fn check_extrinsics(&self, block: &Block)
    -> ::srml_support::inherent::CheckInherentsResult;
}
impl InherentDataExt for ::srml_support::inherent::InherentData {
    fn create_extrinsics(&self)
     ->
         ::srml_support::inherent::Vec<<Block as
                                       ::srml_support::inherent::BlockT>::Extrinsic> {
        use ::srml_support::inherent::ProvideInherent;
        let mut inherents = Vec::new();
        if let Some(inherent) = Aura::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new_unsigned(Call::Timestamp(inherent)));
        }
        if let Some(inherent) = Timestamp::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new_unsigned(Call::Timestamp(inherent)));
        }
        if let Some(inherent) = Consensus::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new_unsigned(Call::Consensus(inherent)));
        }
        if let Some(inherent) = FinalityTracker::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new_unsigned(Call::FinalityTracker(inherent)));
        }
        inherents
    }
    fn check_extrinsics(&self, block: &Block)
     -> ::srml_support::inherent::CheckInherentsResult {
        use ::srml_support::inherent::{ProvideInherent, IsFatalError};
        let mut result =
            ::srml_support::inherent::CheckInherentsResult::new();
        for xt in block.extrinsics() {
            if ::srml_support::inherent::Extrinsic::is_signed(xt).unwrap_or(false)
               {
                break ;
            }
            match xt.function {
                Call::Timestamp(ref call) => {
                    if let Err(e) = Aura::check_inherent(call, self) {
                        result.put_error(Aura::INHERENT_IDENTIFIER,
                                         &e).expect("There is only one fatal error; qed");
                        if e.is_fatal_error() { return result; }
                    }
                }
                _ => { }
            }
            match xt.function {
                Call::Timestamp(ref call) => {
                    if let Err(e) = Timestamp::check_inherent(call, self) {
                        result.put_error(Timestamp::INHERENT_IDENTIFIER,
                                         &e).expect("There is only one fatal error; qed");
                        if e.is_fatal_error() { return result; }
                    }
                }
                _ => { }
            }
            match xt.function {
                Call::Consensus(ref call) => {
                    if let Err(e) = Consensus::check_inherent(call, self) {
                        result.put_error(Consensus::INHERENT_IDENTIFIER,
                                         &e).expect("There is only one fatal error; qed");
                        if e.is_fatal_error() { return result; }
                    }
                }
                _ => { }
            }
            match xt.function {
                Call::FinalityTracker(ref call) => {
                    if let Err(e) =
                           FinalityTracker::check_inherent(call, self) {
                        result.put_error(FinalityTracker::INHERENT_IDENTIFIER,
                                         &e).expect("There is only one fatal error; qed");
                        if e.is_fatal_error() { return result; }
                    }
                }
                _ => { }
            }
        }
        result
    }
}
impl ::srml_support::unsigned::ValidateUnsigned for Runtime {
    type
    Call
    =
    Call;
    fn validate_unsigned(call: &Self::Call)
     -> ::srml_support::unsigned::TransactionValidity {

        #[allow(unreachable_patterns)]
        match call {
            _ =>
            ::srml_support::unsigned::TransactionValidity::Invalid(::srml_support::unsigned::ApplyError::BadSignature
                                                                       as i8),
        }
    }
}
/// The address format for describing accounts.
pub type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic
    =
    generic::UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Index, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive
    =
    executive::Executive<Runtime, Block, system::ChainContext<Runtime>,
                         Balances, Runtime, AllModules>;
#[doc(hidden)]
mod sr_api_hidden_includes_IMPL_RUNTIME_APIS {
    pub extern crate client as sr_api_client;
}
pub struct RuntimeApi {
}
#[doc = r" Implements all runtime apis for the client side."]
#[cfg(any(feature = "std", test))]
pub struct RuntimeApiImpl<C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                                       as
                                                                                                                       self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
                          'static> {
    call: &'static C,
    commit_on_success: std::cell::RefCell<bool>,
    initialized_block: std::cell::RefCell<Option<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                     as
                                                                                                                                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>>,
    changes: std::cell::RefCell<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::OverlayedChanges>,
    recorder: Option<std::rc::Rc<std::cell::RefCell<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ProofRecorder<<Runtime
                                                                                                                                              as
                                                                                                                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>>>,
}
#[cfg(any(feature = "std", test))]
unsafe impl <C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                          as
                                                                                                          self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>
 Send for RuntimeApiImpl<C> {
}
#[cfg(any(feature = "std", test))]
unsafe impl <C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                          as
                                                                                                          self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>
 Sync for RuntimeApiImpl<C> {
}
#[cfg(any(feature = "std", test))]
impl <C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                   as
                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>
 self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ApiExt<<Runtime
                                                                                    as
                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>
 for RuntimeApiImpl<C> {
    fn map_api_result<F: FnOnce(&Self) -> ::std::result::Result<R, E>, R,
                      E>(&self, map_call: F) -> ::std::result::Result<R, E>
     where Self: Sized {
        *self.commit_on_success.borrow_mut() = false;
        let res = map_call(self);
        *self.commit_on_success.borrow_mut() = true;
        self.commit_on_ok(&res);
        res
    }
    fn runtime_version_at(&self,
                          at:
                              &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                   as
                                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::RuntimeVersion> {
        self.call.runtime_version_at(at)
    }
    fn record_proof(&mut self) { self.recorder = Some(Default::default()); }
    fn extract_proof(&mut self) -> Option<Vec<Vec<u8>>> {
        self.recorder.take().map(|r|
                                     {
                                         r.borrow_mut().drain().into_iter().map(|n|
                                                                                    n.data.to_vec()).collect()
                                     })
    }
}
#[cfg(any(feature = "std", test))]
impl <C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                   as
                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
      'static>
 self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ConstructRuntimeApi<<Runtime
                                                                                                 as
                                                                                                 self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                 C>
 for RuntimeApi {
    type
    RuntimeApi
    =
    RuntimeApiImpl<C>;
    fn construct_runtime_api<'a>(call: &'a C)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ApiRef<'a,
                                                                                            Self::RuntimeApi> {
        RuntimeApiImpl{call: unsafe { ::std::mem::transmute(call) },
                       commit_on_success: true.into(),
                       initialized_block: None.into(),
                       changes: Default::default(),
                       recorder: Default::default(),}.into()
    }
}
#[cfg(any(feature = "std", test))]
impl <C: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                   as
                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>
 RuntimeApiImpl<C> {
    fn call_api_at<R: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode +
                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode +
                   PartialEq,
                   F: FnOnce(&C, &Self,
                             &std::cell::RefCell<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::OverlayedChanges>,
                             &std::cell::RefCell<Option<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                            as
                                                                                                                                            self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>>,
                             &Option<std::rc::Rc<std::cell::RefCell<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ProofRecorder<<Runtime
                                                                                                                                                              as
                                                                                                                                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>>>)
                   ->
                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<R>>>(&self,
                                                                                                                                                                                                 call_api_at:
                                                                                                                                                                                                     F)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<R>> {
        let res =
            unsafe {
                call_api_at(&self.call, self, &self.changes,
                            &self.initialized_block, &self.recorder)
            };
        self.commit_on_ok(&res);
        res
    }
    fn commit_on_ok<R, E>(&self, res: &::std::result::Result<R, E>) {
        if *self.commit_on_success.borrow() {
            if res.is_err() {
                self.changes.borrow_mut().discard_prospective();
            } else { self.changes.borrow_mut().commit_prospective(); }
        }
    }
}
impl client_api::runtime_decl_for_Core::Core<Block> for Runtime {
    fn version() -> RuntimeVersion { VERSION }
    fn execute_block(block: Block) { Executive::execute_block(block) }
    fn initialize_block(header: &<Block as BlockT>::Header) {
        Executive::initialize_block(header)
    }
    fn authorities() -> Vec<AuthorityIdFor<Block>> {
        {
            ::std::rt::begin_panic("Deprecated, please use `AuthoritiesApi`.",
                                   &("node/runtime/src/lib.rs", 269u32, 4u32))
        }
    }
}
impl client_api::runtime_decl_for_Metadata::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata { Runtime::metadata().into() }
}
impl block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block> for
 Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic)
     -> ApplyResult {
        Executive::apply_extrinsic(extrinsic)
    }
    fn finalize_block() -> <Block as BlockT>::Header {
        Executive::finalize_block()
    }
    fn inherent_extrinsics(data: InherentData)
     -> Vec<<Block as BlockT>::Extrinsic> {
        data.create_extrinsics()
    }
    fn check_inherents(block: Block, data: InherentData)
     -> CheckInherentsResult {
        data.check_extrinsics(&block)
    }
    fn random_seed() -> <Block as BlockT>::Hash { System::random_seed() }
}
impl client_api::runtime_decl_for_TaggedTransactionQueue::TaggedTransactionQueue<Block>
 for Runtime {
    fn validate_transaction(tx: <Block as BlockT>::Extrinsic)
     -> TransactionValidity {
        Executive::validate_transaction(tx)
    }
}
impl offchain_primitives::runtime_decl_for_OffchainWorkerApi::OffchainWorkerApi<Block>
 for Runtime {
    fn offchain_worker(number: NumberFor<Block>) {
        Executive::offchain_worker(number)
    }
}
impl fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block> for Runtime
 {
    fn grandpa_pending_change(digest: &DigestFor<Block>)
     -> Option<ScheduledChange<NumberFor<Block>>> {
        for log in
            digest.logs.iter().filter_map(|l|
                                              match l {
                                                  Log(InternalLog::grandpa(grandpa_signal))
                                                  => Some(grandpa_signal),
                                                  _ => None,
                                              }) {
            if let Some(change) = Grandpa::scrape_digest_change(log) {
                return Some(change);
            }
        }
        None
    }
    fn grandpa_forced_change(digest: &DigestFor<Block>)
     -> Option<(NumberFor<Block>, ScheduledChange<NumberFor<Block>>)> {
        for log in
            digest.logs.iter().filter_map(|l|
                                              match l {
                                                  Log(InternalLog::grandpa(grandpa_signal))
                                                  => Some(grandpa_signal),
                                                  _ => None,
                                              }) {
            if let Some(change) = Grandpa::scrape_digest_forced_change(log) {
                return Some(change);
            }
        }
        None
    }
    fn grandpa_authorities() -> Vec<(AuthorityId, u64)> {
        Grandpa::grandpa_authorities()
    }
}
impl consensus_aura::runtime_decl_for_AuraApi::AuraApi<Block> for Runtime {
    fn slot_duration() -> u64 { Aura::slot_duration() }
}
impl consensus_authorities::runtime_decl_for_AuthoritiesApi::AuthoritiesApi<Block>
 for Runtime {
    fn authorities() -> Vec<AuthorityIdFor<Block>> {
        Consensus::authorities()
    }
}
#[cfg(any(feature = "std", test))]
impl <RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                                    as
                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
      'static>
 client_api::Core<<Runtime as
                  self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>
 for RuntimeApiImpl<RuntimeApiImplCall> {
    fn Core_version_runtime_api_impl(&self,
                                     at:
                                         &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                              as
                                                                                                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                     context:
                                         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                     params: Option<()>,
                                     params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<RuntimeVersion>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 client_api::runtime_decl_for_Core::version_call_api_at(call_runtime_at,
                                                                                        core_api,
                                                                                        at,
                                                                                        params_encoded,
                                                                                        changes,
                                                                                        initialized_block,
                                                                                        params.map(|p|
                                                                                                       {
                                                                                                           client_api::runtime_decl_for_Core::version_native_call_generator::<Runtime,
                                                                                                                                                                              <Runtime
                                                                                                                                                                              as
                                                                                                                                                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                              Block>()
                                                                                                       }),
                                                                                        context,
                                                                                        recorder)
                             })
    }
    fn Core_execute_block_runtime_api_impl(&self,
                                           at:
                                               &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                    as
                                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                           context:
                                               self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                           params:
                                               Option<(<Runtime as
                                                       self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock)>,
                                           params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<()>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 client_api::runtime_decl_for_Core::execute_block_call_api_at(call_runtime_at,
                                                                                              core_api,
                                                                                              at,
                                                                                              params_encoded,
                                                                                              changes,
                                                                                              initialized_block,
                                                                                              params.map(|p|
                                                                                                             {
                                                                                                                 client_api::runtime_decl_for_Core::execute_block_native_call_generator::<Runtime,
                                                                                                                                                                                          <Runtime
                                                                                                                                                                                          as
                                                                                                                                                                                          self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                          Block>(p)
                                                                                                             }),
                                                                                              context,
                                                                                              recorder)
                             })
    }
    fn Core_initialize_block_runtime_api_impl(&self,
                                              at:
                                                  &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                       as
                                                                                                                                       self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                              context:
                                                  self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                              params:
                                                  Option<(&<<Runtime as
                                                            self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock
                                                           as
                                                           BlockT>::Header)>,
                                              params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<()>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 client_api::runtime_decl_for_Core::initialize_block_call_api_at(call_runtime_at,
                                                                                                 core_api,
                                                                                                 at,
                                                                                                 params_encoded,
                                                                                                 changes,
                                                                                                 initialized_block,
                                                                                                 params.map(|p|
                                                                                                                {
                                                                                                                    client_api::runtime_decl_for_Core::initialize_block_native_call_generator::<Runtime,
                                                                                                                                                                                                <Runtime
                                                                                                                                                                                                as
                                                                                                                                                                                                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                Block>(p)
                                                                                                                }),
                                                                                                 context,
                                                                                                 recorder)
                             })
    }
    fn Core_authorities_runtime_api_impl(&self,
                                         at:
                                             &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                  as
                                                                                                                                  self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                         context:
                                             self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                         params: Option<()>,
                                         params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Vec<AuthorityIdFor<<Runtime
                                                                                                                                                                                                     as
                                                                                                                                                                                                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 client_api::runtime_decl_for_Core::authorities_call_api_at(call_runtime_at,
                                                                                            core_api,
                                                                                            at,
                                                                                            params_encoded,
                                                                                            changes,
                                                                                            initialized_block,
                                                                                            params.map(|p|
                                                                                                           {
                                                                                                               client_api::runtime_decl_for_Core::authorities_native_call_generator::<Runtime,
                                                                                                                                                                                      <Runtime
                                                                                                                                                                                      as
                                                                                                                                                                                      self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                      Block>()
                                                                                                           }),
                                                                                            context,
                                                                                            recorder)
                             })
    }
}
#[cfg(any(feature = "std", test))]
impl <RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                                    as
                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
      'static>
 client_api::Metadata<<Runtime as
                      self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>
 for RuntimeApiImpl<RuntimeApiImplCall> {
    fn Metadata_metadata_runtime_api_impl(&self,
                                          at:
                                              &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                   as
                                                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                          context:
                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                          params: Option<()>,
                                          params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<OpaqueMetadata>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 client_api::runtime_decl_for_Metadata::metadata_call_api_at(call_runtime_at,
                                                                                             core_api,
                                                                                             at,
                                                                                             params_encoded,
                                                                                             changes,
                                                                                             initialized_block,
                                                                                             params.map(|p|
                                                                                                            {
                                                                                                                client_api::runtime_decl_for_Metadata::metadata_native_call_generator::<Runtime,
                                                                                                                                                                                        <Runtime
                                                                                                                                                                                        as
                                                                                                                                                                                        self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                        Block>()
                                                                                                            }),
                                                                                             context,
                                                                                             recorder)
                             })
    }
}
#[cfg(any(feature = "std", test))]
impl <RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                                    as
                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
      'static>
 block_builder_api::BlockBuilder<<Runtime as
                                 self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>
 for RuntimeApiImpl<RuntimeApiImplCall> {
    fn BlockBuilder_apply_extrinsic_runtime_api_impl(&self,
                                                     at:
                                                         &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                              as
                                                                                                                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                     context:
                                                         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                     params:
                                                         Option<(<<Runtime as
                                                                  self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock
                                                                 as
                                                                 BlockT>::Extrinsic)>,
                                                     params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<ApplyResult>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 block_builder_api::runtime_decl_for_BlockBuilder::apply_extrinsic_call_api_at(call_runtime_at,
                                                                                                               core_api,
                                                                                                               at,
                                                                                                               params_encoded,
                                                                                                               changes,
                                                                                                               initialized_block,
                                                                                                               params.map(|p|
                                                                                                                              {
                                                                                                                                  block_builder_api::runtime_decl_for_BlockBuilder::apply_extrinsic_native_call_generator::<Runtime,
                                                                                                                                                                                                                            <Runtime
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                            Block>(p)
                                                                                                                              }),
                                                                                                               context,
                                                                                                               recorder)
                             })
    }
    fn BlockBuilder_finalize_block_runtime_api_impl(&self,
                                                    at:
                                                        &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                             as
                                                                                                                                             self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                    context:
                                                        self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                    params: Option<()>,
                                                    params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<<<Runtime
                                                                                                                                                                                   as
                                                                                                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock
                                                                                                                                                                                  as
                                                                                                                                                                                  BlockT>::Header>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 block_builder_api::runtime_decl_for_BlockBuilder::finalize_block_call_api_at(call_runtime_at,
                                                                                                              core_api,
                                                                                                              at,
                                                                                                              params_encoded,
                                                                                                              changes,
                                                                                                              initialized_block,
                                                                                                              params.map(|p|
                                                                                                                             {
                                                                                                                                 block_builder_api::runtime_decl_for_BlockBuilder::finalize_block_native_call_generator::<Runtime,
                                                                                                                                                                                                                          <Runtime
                                                                                                                                                                                                                          as
                                                                                                                                                                                                                          self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                          Block>()
                                                                                                                             }),
                                                                                                              context,
                                                                                                              recorder)
                             })
    }
    fn BlockBuilder_inherent_extrinsics_runtime_api_impl(&self,
                                                         at:
                                                             &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                                  as
                                                                                                                                                  self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                         context:
                                                             self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                         params:
                                                             Option<(InherentData)>,
                                                         params_encoded:
                                                             Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Vec<<<Runtime
                                                                                                                                                                                       as
                                                                                                                                                                                       self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock
                                                                                                                                                                                      as
                                                                                                                                                                                      BlockT>::Extrinsic>>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 block_builder_api::runtime_decl_for_BlockBuilder::inherent_extrinsics_call_api_at(call_runtime_at,
                                                                                                                   core_api,
                                                                                                                   at,
                                                                                                                   params_encoded,
                                                                                                                   changes,
                                                                                                                   initialized_block,
                                                                                                                   params.map(|p|
                                                                                                                                  {
                                                                                                                                      block_builder_api::runtime_decl_for_BlockBuilder::inherent_extrinsics_native_call_generator::<Runtime,
                                                                                                                                                                                                                                    <Runtime
                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                                    Block>(p)
                                                                                                                                  }),
                                                                                                                   context,
                                                                                                                   recorder)
                             })
    }
    fn BlockBuilder_check_inherents_runtime_api_impl(&self,
                                                     at:
                                                         &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                              as
                                                                                                                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                     context:
                                                         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                     params:
                                                         Option<(<Runtime as
                                                                 self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                 InherentData)>,
                                                     params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<CheckInherentsResult>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 block_builder_api::runtime_decl_for_BlockBuilder::check_inherents_call_api_at(call_runtime_at,
                                                                                                               core_api,
                                                                                                               at,
                                                                                                               params_encoded,
                                                                                                               changes,
                                                                                                               initialized_block,
                                                                                                               params.map(|p|
                                                                                                                              {
                                                                                                                                  block_builder_api::runtime_decl_for_BlockBuilder::check_inherents_native_call_generator::<Runtime,
                                                                                                                                                                                                                            <Runtime
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                            Block>(p.0,
                                                                                                                                                                                                                                   p.1)
                                                                                                                              }),
                                                                                                               context,
                                                                                                               recorder)
                             })
    }
    fn BlockBuilder_random_seed_runtime_api_impl(&self,
                                                 at:
                                                     &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                          as
                                                                                                                                          self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                 context:
                                                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                 params: Option<()>,
                                                 params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<<<Runtime
                                                                                                                                                                                   as
                                                                                                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock
                                                                                                                                                                                  as
                                                                                                                                                                                  BlockT>::Hash>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 block_builder_api::runtime_decl_for_BlockBuilder::random_seed_call_api_at(call_runtime_at,
                                                                                                           core_api,
                                                                                                           at,
                                                                                                           params_encoded,
                                                                                                           changes,
                                                                                                           initialized_block,
                                                                                                           params.map(|p|
                                                                                                                          {
                                                                                                                              block_builder_api::runtime_decl_for_BlockBuilder::random_seed_native_call_generator::<Runtime,
                                                                                                                                                                                                                    <Runtime
                                                                                                                                                                                                                    as
                                                                                                                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                    Block>()
                                                                                                                          }),
                                                                                                           context,
                                                                                                           recorder)
                             })
    }
}
#[cfg(any(feature = "std", test))]
impl <RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                                    as
                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
      'static>
 client_api::TaggedTransactionQueue<<Runtime as
                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>
 for RuntimeApiImpl<RuntimeApiImplCall> {
    fn TaggedTransactionQueue_validate_transaction_runtime_api_impl(&self,
                                                                    at:
                                                                        &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                                             as
                                                                                                                                                             self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                                    context:
                                                                        self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                                    params:
                                                                        Option<(<<Runtime
                                                                                 as
                                                                                 self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock
                                                                                as
                                                                                BlockT>::Extrinsic)>,
                                                                    params_encoded:
                                                                        Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<TransactionValidity>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 client_api::runtime_decl_for_TaggedTransactionQueue::validate_transaction_call_api_at(call_runtime_at,
                                                                                                                       core_api,
                                                                                                                       at,
                                                                                                                       params_encoded,
                                                                                                                       changes,
                                                                                                                       initialized_block,
                                                                                                                       params.map(|p|
                                                                                                                                      {
                                                                                                                                          client_api::runtime_decl_for_TaggedTransactionQueue::validate_transaction_native_call_generator::<Runtime,
                                                                                                                                                                                                                                            <Runtime
                                                                                                                                                                                                                                            as
                                                                                                                                                                                                                                            self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                                            Block>(p)
                                                                                                                                      }),
                                                                                                                       context,
                                                                                                                       recorder)
                             })
    }
}
#[cfg(any(feature = "std", test))]
impl <RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                                    as
                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
      'static>
 offchain_primitives::OffchainWorkerApi<<Runtime as
                                        self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>
 for RuntimeApiImpl<RuntimeApiImplCall> {
    fn OffchainWorkerApi_offchain_worker_runtime_api_impl(&self,
                                                          at:
                                                              &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                                   as
                                                                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                          context:
                                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                          params:
                                                              Option<(NumberFor<<Runtime
                                                                                as
                                                                                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>)>,
                                                          params_encoded:
                                                              Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<()>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 offchain_primitives::runtime_decl_for_OffchainWorkerApi::offchain_worker_call_api_at(call_runtime_at,
                                                                                                                      core_api,
                                                                                                                      at,
                                                                                                                      params_encoded,
                                                                                                                      changes,
                                                                                                                      initialized_block,
                                                                                                                      params.map(|p|
                                                                                                                                     {
                                                                                                                                         offchain_primitives::runtime_decl_for_OffchainWorkerApi::offchain_worker_native_call_generator::<Runtime,
                                                                                                                                                                                                                                          <Runtime
                                                                                                                                                                                                                                          as
                                                                                                                                                                                                                                          self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                                          Block>(p)
                                                                                                                                     }),
                                                                                                                      context,
                                                                                                                      recorder)
                             })
    }
}
#[cfg(any(feature = "std", test))]
impl <RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                                    as
                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
      'static>
 fg_primitives::GrandpaApi<<Runtime as
                           self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>
 for RuntimeApiImpl<RuntimeApiImplCall> {
    fn GrandpaApi_grandpa_pending_change_runtime_api_impl(&self,
                                                          at:
                                                              &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                                   as
                                                                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                          context:
                                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                          params:
                                                              Option<(&DigestFor<<Runtime
                                                                                 as
                                                                                 self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>)>,
                                                          params_encoded:
                                                              Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Option<ScheduledChange<NumberFor<<Runtime
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>>>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 fg_primitives::runtime_decl_for_GrandpaApi::grandpa_pending_change_call_api_at(call_runtime_at,
                                                                                                                core_api,
                                                                                                                at,
                                                                                                                params_encoded,
                                                                                                                changes,
                                                                                                                initialized_block,
                                                                                                                params.map(|p|
                                                                                                                               {
                                                                                                                                   fg_primitives::runtime_decl_for_GrandpaApi::grandpa_pending_change_native_call_generator::<Runtime,
                                                                                                                                                                                                                              <Runtime
                                                                                                                                                                                                                              as
                                                                                                                                                                                                                              self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                              Block>(p)
                                                                                                                               }),
                                                                                                                context,
                                                                                                                recorder)
                             })
    }
    fn GrandpaApi_grandpa_forced_change_runtime_api_impl(&self,
                                                         at:
                                                             &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                                  as
                                                                                                                                                  self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                         context:
                                                             self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                         params:
                                                             Option<(&DigestFor<<Runtime
                                                                                as
                                                                                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>)>,
                                                         params_encoded:
                                                             Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Option<(NumberFor<<Runtime
                                                                                                                                                                                                    as
                                                                                                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                                                                                                                                                          ScheduledChange<NumberFor<<Runtime
                                                                                                                                                                                                                    as
                                                                                                                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>)>>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 fg_primitives::runtime_decl_for_GrandpaApi::grandpa_forced_change_call_api_at(call_runtime_at,
                                                                                                               core_api,
                                                                                                               at,
                                                                                                               params_encoded,
                                                                                                               changes,
                                                                                                               initialized_block,
                                                                                                               params.map(|p|
                                                                                                                              {
                                                                                                                                  fg_primitives::runtime_decl_for_GrandpaApi::grandpa_forced_change_native_call_generator::<Runtime,
                                                                                                                                                                                                                            <Runtime
                                                                                                                                                                                                                            as
                                                                                                                                                                                                                            self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                            Block>(p)
                                                                                                                              }),
                                                                                                               context,
                                                                                                               recorder)
                             })
    }
    fn GrandpaApi_grandpa_authorities_runtime_api_impl(&self,
                                                       at:
                                                           &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                                as
                                                                                                                                                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                       context:
                                                           self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                       params: Option<()>,
                                                       params_encoded:
                                                           Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Vec<(AuthorityId,
                                                                                                                                                                                       u64)>>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 fg_primitives::runtime_decl_for_GrandpaApi::grandpa_authorities_call_api_at(call_runtime_at,
                                                                                                             core_api,
                                                                                                             at,
                                                                                                             params_encoded,
                                                                                                             changes,
                                                                                                             initialized_block,
                                                                                                             params.map(|p|
                                                                                                                            {
                                                                                                                                fg_primitives::runtime_decl_for_GrandpaApi::grandpa_authorities_native_call_generator::<Runtime,
                                                                                                                                                                                                                        <Runtime
                                                                                                                                                                                                                        as
                                                                                                                                                                                                                        self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                        Block>()
                                                                                                                            }),
                                                                                                             context,
                                                                                                             recorder)
                             })
    }
}
#[cfg(any(feature = "std", test))]
impl <RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                                    as
                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
      'static>
 consensus_aura::AuraApi<<Runtime as
                         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>
 for RuntimeApiImpl<RuntimeApiImplCall> {
    fn AuraApi_slot_duration_runtime_api_impl(&self,
                                              at:
                                                  &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                       as
                                                                                                                                       self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                              context:
                                                  self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                              params: Option<()>,
                                              params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<u64>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 consensus_aura::runtime_decl_for_AuraApi::slot_duration_call_api_at(call_runtime_at,
                                                                                                     core_api,
                                                                                                     at,
                                                                                                     params_encoded,
                                                                                                     changes,
                                                                                                     initialized_block,
                                                                                                     params.map(|p|
                                                                                                                    {
                                                                                                                        consensus_aura::runtime_decl_for_AuraApi::slot_duration_native_call_generator::<Runtime,
                                                                                                                                                                                                        <Runtime
                                                                                                                                                                                                        as
                                                                                                                                                                                                        self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                        Block>()
                                                                                                                    }),
                                                                                                     context,
                                                                                                     recorder)
                             })
    }
}
#[cfg(any(feature = "std", test))]
impl <RuntimeApiImplCall: self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<<Runtime
                                                                                                                    as
                                                                                                                    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock> +
      'static>
 consensus_authorities::AuthoritiesApi<<Runtime as
                                       self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>
 for RuntimeApiImpl<RuntimeApiImplCall> {
    fn AuthoritiesApi_authorities_runtime_api_impl(&self,
                                                   at:
                                                       &self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<<Runtime
                                                                                                                                            as
                                                                                                                                            self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>,
                                                   context:
                                                       self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                   params: Option<()>,
                                                   params_encoded: Vec<u8>)
     ->
         self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Vec<AuthorityIdFor<<Runtime
                                                                                                                                                                                                     as
                                                                                                                                                                                                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock>>>> {
        self.call_api_at(|call_runtime_at, core_api, changes,
                          initialized_block, recorder|
                             {
                                 consensus_authorities::runtime_decl_for_AuthoritiesApi::authorities_call_api_at(call_runtime_at,
                                                                                                                 core_api,
                                                                                                                 at,
                                                                                                                 params_encoded,
                                                                                                                 changes,
                                                                                                                 initialized_block,
                                                                                                                 params.map(|p|
                                                                                                                                {
                                                                                                                                    consensus_authorities::runtime_decl_for_AuthoritiesApi::authorities_native_call_generator::<Runtime,
                                                                                                                                                                                                                                <Runtime
                                                                                                                                                                                                                                as
                                                                                                                                                                                                                                self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::GetNodeBlockType>::NodeBlock,
                                                                                                                                                                                                                                Block>()
                                                                                                                                }),
                                                                                                                 context,
                                                                                                                 recorder)
                             })
    }
}
const RUNTIME_API_VERSIONS:
      self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ApisVec
      =
    ::std::borrow::Cow::Borrowed(&[(client_api::runtime_decl_for_Core::ID,
                                    client_api::runtime_decl_for_Core::VERSION),
                                   (client_api::runtime_decl_for_Metadata::ID,
                                    client_api::runtime_decl_for_Metadata::VERSION),
                                   (block_builder_api::runtime_decl_for_BlockBuilder::ID,
                                    block_builder_api::runtime_decl_for_BlockBuilder::VERSION),
                                   (client_api::runtime_decl_for_TaggedTransactionQueue::ID,
                                    client_api::runtime_decl_for_TaggedTransactionQueue::VERSION),
                                   (offchain_primitives::runtime_decl_for_OffchainWorkerApi::ID,
                                    offchain_primitives::runtime_decl_for_OffchainWorkerApi::VERSION),
                                   (fg_primitives::runtime_decl_for_GrandpaApi::ID,
                                    fg_primitives::runtime_decl_for_GrandpaApi::VERSION),
                                   (consensus_aura::runtime_decl_for_AuraApi::ID,
                                    consensus_aura::runtime_decl_for_AuraApi::VERSION),
                                   (consensus_authorities::runtime_decl_for_AuthoritiesApi::ID,
                                    consensus_authorities::runtime_decl_for_AuthoritiesApi::VERSION)]);
pub mod api {
    use super::*;
    #[cfg(feature = "std")]
    pub fn dispatch(method: &str, mut data: &[u8]) -> Option<Vec<u8>> {
        match method {
            "Core_version" =>
            Some({
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             client_api::runtime_decl_for_Core::Core<Block>>::version();
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "Core_execute_block" =>
            Some({
                     let block: Block =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"execute_block",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             client_api::runtime_decl_for_Core::Core<Block>>::execute_block(block);
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "Core_initialize_block" =>
            Some({
                     let header: <Block as BlockT>::Header =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"initialize_block",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             client_api::runtime_decl_for_Core::Core<Block>>::initialize_block(&header);
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "Core_authorities" =>
            Some({
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             client_api::runtime_decl_for_Core::Core<Block>>::authorities();
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "Metadata_metadata" =>
            Some({
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             client_api::runtime_decl_for_Metadata::Metadata<Block>>::metadata();
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "BlockBuilder_apply_extrinsic" =>
            Some({
                     let extrinsic: <Block as BlockT>::Extrinsic =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"apply_extrinsic",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::apply_extrinsic(extrinsic);
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "BlockBuilder_finalize_block" =>
            Some({
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::finalize_block();
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "BlockBuilder_inherent_extrinsics" =>
            Some({
                     let data: InherentData =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"inherent_extrinsics",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::inherent_extrinsics(data);
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "BlockBuilder_check_inherents" =>
            Some({
                     let block: Block =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"check_inherents",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     let data: InherentData =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"check_inherents",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::check_inherents(block,
                                                                                                                     data);
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "BlockBuilder_random_seed" =>
            Some({
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block>>::random_seed();
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "TaggedTransactionQueue_validate_transaction" =>
            Some({
                     let tx: <Block as BlockT>::Extrinsic =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"validate_transaction",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             client_api::runtime_decl_for_TaggedTransactionQueue::TaggedTransactionQueue<Block>>::validate_transaction(tx);
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "OffchainWorkerApi_offchain_worker" =>
            Some({
                     let number: NumberFor<Block> =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"offchain_worker",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             offchain_primitives::runtime_decl_for_OffchainWorkerApi::OffchainWorkerApi<Block>>::offchain_worker(number);
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "GrandpaApi_grandpa_pending_change" =>
            Some({
                     let digest: DigestFor<Block> =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"grandpa_pending_change",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block>>::grandpa_pending_change(&digest);
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "GrandpaApi_grandpa_forced_change" =>
            Some({
                     let digest: DigestFor<Block> =
                         match self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Decode::decode(&mut data)
                             {
                             Some(input) => input,
                             None => {
                                 ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["Bad input data provided to "],
                                                                                           &match (&"grandpa_forced_change",)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                            &("node/runtime/src/lib.rs",
                                                              254u32, 1u32))
                             }
                         };
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block>>::grandpa_forced_change(&digest);
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "GrandpaApi_grandpa_authorities" =>
            Some({
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             fg_primitives::runtime_decl_for_GrandpaApi::GrandpaApi<Block>>::grandpa_authorities();
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "AuraApi_slot_duration" =>
            Some({
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             consensus_aura::runtime_decl_for_AuraApi::AuraApi<Block>>::slot_duration();
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            "AuthoritiesApi_authorities" =>
            Some({
                     #[allow(deprecated)]
                     let output =
                         <Runtime as
                             consensus_authorities::runtime_decl_for_AuthoritiesApi::AuthoritiesApi<Block>>::authorities();
                     self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&output)
                 }),
            _ => None,
        }
    }
}
