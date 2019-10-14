#![feature(prelude_import)]
#![no_std]
// Copyright 2017-2019 Parity Technologies (UK) Ltd.
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

//! # System Module
//!
//! The System module provides low-level access to core types and cross-cutting utilities.
//! It acts as the base layer for other SRML modules to interact with the Substrate framework components.
//!
//! - [`system::Trait`](./trait.Trait.html)
//!
//! ## Overview
//!
//! The System module defines the core data types used in a Substrate runtime.
//! It also provides several utility functions (see [`Module`](./struct.Module.html)) for other runtime modules.
//!
//! In addition, it manages the storage items for extrinsics data, indexes, event records, and digest items,
//! among other things that support the execution of the current block.
//!
//! It also handles low-level tasks like depositing logs, basic set up and take down of
//! temporary storage entries, and access to previous block hashes.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! The System module does not implement any dispatchable functions.
//!
//! ### Public Functions
//!
//! See the [`Module`](./struct.Module.html) struct for details of publicly available functions.
//!
//! ## Usage
//!
//! ### Prerequisites
//!
//! Import the System module and derive your module's configuration trait from the system trait.
//!
//! ### Example - Get random seed and extrinsic count for the current block
//!
//! ```
//! use srml_support::{decl_module, dispatch::Result};
//! use srml_system::{self as system, ensure_signed};
//!
//! pub trait Trait: system::Trait {}
//!
//! decl_module! {
//! 	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
//! 		pub fn system_module_example(origin) -> Result {
//! 			let _sender = ensure_signed(origin)?;
//! 			let _random_seed = <system::Module<T>>::random_seed();
//! 			let _extrinsic_count = <system::Module<T>>::extrinsic_count();
//! 			Ok(())
//! 		}
//! 	}
//! }
//! # fn main() { }
//! ```
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[cfg(feature = "std")]
use serde::Serialize;
use rstd::prelude::*;
#[cfg(any(feature = "std", test))]
use rstd::map;
use primitives::traits::{self, CheckEqual, SimpleArithmetic, SimpleBitOps,
                         One, Bounded, Lookup, Hash, Member, MaybeDisplay,
                         EnsureOrigin, Digest as DigestT, As, CurrentHeight,
                         BlockNumberToHash,
                         MaybeSerializeDebugButNotDeserialize,
                         MaybeSerializeDebug, StaticLookup};
#[cfg(any(feature = "std", test))]
use primitives::traits::Zero;
use substrate_primitives::storage::well_known_keys;
use srml_support::{storage, decl_module, decl_event, decl_storage,
                   StorageDoubleMap, StorageValue, StorageMap, Parameter};
use safe_mix::TripletMix;
use parity_codec::{Encode, Decode};

#[cfg(any(feature = "std", test))]
use runtime_io::{twox_128, TestExternalities, Blake2Hasher};

#[cfg(any(feature = "std", test))]
use substrate_primitives::ChangesTrieConfiguration;

/// Handler for when a new account has been created.
pub trait OnNewAccount<AccountId> {
    /// A new account `who` has been registered.
    fn on_new_account(who: &AccountId);
}

impl <AccountId> OnNewAccount<AccountId> for () {
    fn on_new_account(_who: &AccountId) { }
}

/// Determiner to say whether a given account is unused.
pub trait IsDeadAccount<AccountId> {
    /// Is the given account dead?
    fn is_dead_account(who: &AccountId)
    -> bool;
}

impl <AccountId> IsDeadAccount<AccountId> for () {
    fn is_dead_account(_who: &AccountId) -> bool { true }
}

/// Compute the trie root of a list of extrinsics.
pub fn extrinsics_root<H: Hash, E: parity_codec::Encode>(extrinsics: &[E])
 -> H::Output {
    extrinsics_data_root::<H>(extrinsics.iter().map(parity_codec::Encode::encode).collect())
}

/// Compute the trie root of a list of extrinsics.
pub fn extrinsics_data_root<H: Hash>(xts: Vec<Vec<u8>>) -> H::Output {
    let xts = xts.iter().map(Vec::as_slice).collect::<Vec<_>>();
    H::enumerated_trie_root(&xts)
}

pub trait Trait: 'static + Eq + Clone {
    /// The aggregated `Origin` type used by dispatchable calls.
    type
    Origin: Into<Option<RawOrigin<Self::AccountId>>> +
    From<RawOrigin<Self::AccountId>>;

    /// Account index (aka nonce) type. This stores the number of previous transactions associated with a sender
    /// account.
    type
    Index: Parameter +
    Member +
    MaybeSerializeDebugButNotDeserialize +
    Default +
    MaybeDisplay +
    SimpleArithmetic +
    Copy;

    /// The block number type used by the runtime.
    type
    BlockNumber: Parameter +
    Member +
    MaybeSerializeDebug +
    MaybeDisplay +
    SimpleArithmetic +
    Default +
    Bounded +
    Copy +
    rstd::hash::Hash;

    /// The output of the `Hashing` function.
    type
    Hash: Parameter +
    Member +
    MaybeSerializeDebug +
    MaybeDisplay +
    SimpleBitOps +
    Default +
    Copy +
    CheckEqual +
    rstd::hash::Hash +
    AsRef<[u8]> +
    AsMut<[u8]>;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    type
    Hashing: Hash<Output
    =
    Self::Hash>;

    /// Collection of (light-client-relevant) logs for a block to be included verbatim in the block header.
    type
    Digest: Parameter +
    Member +
    MaybeSerializeDebugButNotDeserialize +
    Default +
    traits::Digest<Hash
    =
    Self::Hash>;

    /// The user account identifier type for the runtime.
    type
    AccountId: Parameter +
    Member +
    MaybeSerializeDebug +
    MaybeDisplay +
    Ord +
    Default;

    /// Converting trait to take a source type and convert to `AccountId`.
    ///
    /// Used to define the type and conversion mechanism for referencing accounts in transactions. It's perfectly
    /// reasonable for this to be an identity conversion (with the source type being `AccountId`), but other modules
    /// (e.g. Indices module) may provide more functional/efficient alternatives.
    type
    Lookup: StaticLookup<Target
    =
    Self::AccountId>;

    /// The block header.
    type
    Header: Parameter +
    traits::Header<Number
    =
    Self::BlockNumber,
    Hash
    =
    Self::Hash,
    Digest
    =
    Self::Digest>;

    /// The aggregated event type of the runtime.
    type
    Event: Parameter +
    Member +
    From<Event>;

    /// A piece of information that can be part of the digest (as a digest item).
    type
    Log: From<Log<Self>> +
    Into<DigestItemOf<Self>>;
}

pub type DigestItemOf<T> = <<T as Trait>::Digest as traits::Digest>::Item;











// Implementation for tests outside of this crate.

// Create a Hash with 69 for each byte,
// only used to build genesis config.



// TODO: https://github.com/paritytech/substrate/issues/2553
// Possibly, we can improve it by using something like:
// `Option<(BlockNumber, Vec<EventIndex>)>`, however in this case we won't be able to use
// `EventTopics::append`.










// Index of the to be added event.
// We've reached the maximum number of events at this block, just
// don't do anything and leave the event_count unaltered.

// Appending can only fail if `Events<T>` can not be decoded or
// when we try to insert more than `u32::max_value()` events.
//
// We perform early return if we've reached the maximum capacity of the event list,
// so `Events<T>` seems to be corrupted. Also, this has happened after the start of execution
// (since the event list is cleared at the block initialization).
// The most sensible thing to do here is to just ignore this event and wait until the
// new block.

// The same applies here.




// populate environment



// we can't compute changes trie root earlier && put it to the Digest
// because it will include all currently existing temporaries.

// The following fields
//
// - <Events<T>>
// - <EventCount<T>>
// - <EventTopics<T>>
//
// stay to be inspected by the client and will be cleared by `Self::initialize`.








// Always the case after block 1 is initialised.























// We deposit a few events with different sets of topics.


// Check that topics are reflected in the event record.

// Check that the topic-events mapping reflects the deposited topics.
// Note that these are indexes of the events.
#[structural_match]
#[rustc_copy_clone_marker]
pub struct Module<T: Trait>(::srml_support::rstd::marker::PhantomData<(T)>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::clone::Clone + Trait> ::std::clone::Clone for Module<T> {
    #[inline]
    fn clone(&self) -> Module<T> {
        match *self {
            Module(ref __self_0_0) =>
            Module(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::marker::Copy + Trait> ::std::marker::Copy for Module<T> { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::cmp::PartialEq + Trait> ::std::cmp::PartialEq for Module<T> {
    #[inline]
    fn eq(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) =>
            match *self {
                Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) =>
            match *self {
                Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::cmp::Eq + Trait> ::std::cmp::Eq for Module<T> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::rstd::marker::PhantomData<(T)>>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::fmt::Debug + Trait> ::std::fmt::Debug for Module<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Module(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Module");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <T: Trait>
 ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber> for
 Module<T> {
}
impl <T: Trait>
 ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber> for
 Module<T> {
}
impl <T: Trait>
 ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
 for Module<T> {
}
impl <T: Trait> Module<T> {
    pub fn deposit_event(event: T::Event) {
        Self::deposit_event_indexed(&[], event);
    }
}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl <T: Trait> Module<T> { }
pub enum Call<T: Trait> {

    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                  ::srml_support::dispatch::Never),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for Call<T> {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Decode for Call<T> {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? { _ => None, }
            }
        }
    };
impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/system/src/lib.rs", 184u32,
                                             1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/system/src/lib.rs", 184u32,
                                             1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::Eq for Call<T> { }
#[cfg(feature = "std")]
impl <T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
    fn fmt(&self, _f: &mut ::srml_support::dispatch::fmt::Formatter)
     ->
         ::srml_support::dispatch::result::Result<(),
                                                  ::srml_support::dispatch::fmt::Error> {
        match *self {
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/system/src/lib.rs", 184u32,
                                             1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
    type
    Trait
    =
    T;
    type
    Origin
    =
    T::Origin;
    fn dispatch(self, _origin: Self::Origin)
     -> ::srml_support::dispatch::Result {
        match self {
            Call::__PhantomItem(_, _) => {
                {
                    {
                        {
                            ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                      &match (&"__PhantomItem should never be used.",)
                                                                                           {
                                                                                           (arg0,)
                                                                                           =>
                                                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                        ::std::fmt::Display::fmt)],
                                                                                       }),
                                                       &("srml/system/src/lib.rs",
                                                         184u32, 1u32))
                        }
                    }
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::Callable for Module<T> {
    type
    Call
    =
    Call<T>;
}
impl <T: Trait> Module<T> {
    #[doc(hidden)]
    pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait =
                    T>>(d: D, origin: D::Origin)
     -> ::srml_support::dispatch::Result {
        d.dispatch(origin)
    }
}
impl <T: Trait> Module<T> {
    #[doc(hidden)]
    pub fn call_functions()
     -> &'static [::srml_support::dispatch::FunctionMetadata] {
        &[]
    }
}
/// A phase of a block's execution.
#[structural_match]
pub enum Phase {

    /// Applying an extrinsic.
    ApplyExtrinsic(u32),

    /// The end.
    Finalization,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Phase: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Phase {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Phase::ApplyExtrinsic(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    Phase::Finalization => { dest.push_byte(1usize as u8); }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Phase: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Phase {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Phase::ApplyExtrinsic(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => { Some(Phase::Finalization) }
                    _ => None,
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Phase: () =
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
        impl _serde::Serialize for Phase {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    Phase::ApplyExtrinsic(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "Phase",
                                                                  0u32,
                                                                  "ApplyExtrinsic",
                                                                  __field0),
                    Phase::Finalization =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "Phase", 1u32,
                                                               "Finalization"),
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Phase {
    #[inline]
    fn eq(&self, other: &Phase) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Phase::ApplyExtrinsic(ref __self_0),
                     &Phase::ApplyExtrinsic(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => true,
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &Phase) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Phase::ApplyExtrinsic(ref __self_0),
                     &Phase::ApplyExtrinsic(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => false,
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Phase {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<u32>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Phase {
    #[inline]
    fn clone(&self) -> Phase {
        match (&*self,) {
            (&Phase::ApplyExtrinsic(ref __self_0),) =>
            Phase::ApplyExtrinsic(::std::clone::Clone::clone(&(*__self_0))),
            (&Phase::Finalization,) => Phase::Finalization,
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Phase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Phase::ApplyExtrinsic(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("ApplyExtrinsic");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Phase::Finalization,) => {
                let mut debug_trait_builder = f.debug_tuple("Finalization");
                debug_trait_builder.finish()
            }
        }
    }
}
/// Record of an event happening.
#[structural_match]
pub struct EventRecord<E: Parameter + Member, T> {
    /// The phase of the block it happened in.
    pub phase: Phase,
    /// The event itself.
    pub event: E,
    /// The list of the topics this event has.
    pub topics: Vec<T>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_EventRecord: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <E: Parameter + Member, T> _parity_codec::Encode for
         EventRecord<E, T> where E: _parity_codec::Encode,
         E: _parity_codec::Encode, Vec<T>: _parity_codec::Encode,
         Vec<T>: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.phase);
                dest.push(&self.event);
                dest.push(&self.topics);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_EventRecord: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <E: Parameter + Member, T> _parity_codec::Decode for
         EventRecord<E, T> where E: _parity_codec::Decode,
         E: _parity_codec::Decode, Vec<T>: _parity_codec::Decode,
         Vec<T>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(EventRecord{phase: _parity_codec::Decode::decode(input)?,
                                 event: _parity_codec::Decode::decode(input)?,
                                 topics:
                                     _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_EventRecord: () =
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
        impl <E: Parameter + Member, T> _serde::Serialize for
         EventRecord<E, T> where E: _serde::Serialize, T: _serde::Serialize {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "EventRecord",
                                                               false as usize
                                                                   + 1 + 1 +
                                                                   1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "phase",
                                                                    &self.phase)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "event",
                                                                    &self.event)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "topics",
                                                                    &self.topics)
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
#[automatically_derived]
#[allow(unused_qualifications)]
impl <E: ::std::cmp::PartialEq + Parameter + Member, T: ::std::cmp::PartialEq>
 ::std::cmp::PartialEq for EventRecord<E, T> {
    #[inline]
    fn eq(&self, other: &EventRecord<E, T>) -> bool {
        match *other {
            EventRecord {
            phase: ref __self_1_0,
            event: ref __self_1_1,
            topics: ref __self_1_2 } =>
            match *self {
                EventRecord {
                phase: ref __self_0_0,
                event: ref __self_0_1,
                topics: ref __self_0_2 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &EventRecord<E, T>) -> bool {
        match *other {
            EventRecord {
            phase: ref __self_1_0,
            event: ref __self_1_1,
            topics: ref __self_1_2 } =>
            match *self {
                EventRecord {
                phase: ref __self_0_0,
                event: ref __self_0_1,
                topics: ref __self_0_2 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <E: ::std::cmp::Eq + Parameter + Member, T: ::std::cmp::Eq>
 ::std::cmp::Eq for EventRecord<E, T> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<Phase>;
            let _: ::std::cmp::AssertParamIsEq<E>;
            let _: ::std::cmp::AssertParamIsEq<Vec<T>>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <E: ::std::clone::Clone + Parameter + Member, T: ::std::clone::Clone>
 ::std::clone::Clone for EventRecord<E, T> {
    #[inline]
    fn clone(&self) -> EventRecord<E, T> {
        match *self {
            EventRecord {
            phase: ref __self_0_0,
            event: ref __self_0_1,
            topics: ref __self_0_2 } =>
            EventRecord{phase: ::std::clone::Clone::clone(&(*__self_0_0)),
                        event: ::std::clone::Clone::clone(&(*__self_0_1)),
                        topics: ::std::clone::Clone::clone(&(*__self_0_2)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <E: ::std::fmt::Debug + Parameter + Member, T: ::std::fmt::Debug>
 ::std::fmt::Debug for EventRecord<E, T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EventRecord {
            phase: ref __self_0_0,
            event: ref __self_0_1,
            topics: ref __self_0_2 } => {
                let mut debug_trait_builder = f.debug_struct("EventRecord");
                let _ = debug_trait_builder.field("phase", &&(*__self_0_0));
                let _ = debug_trait_builder.field("event", &&(*__self_0_1));
                let _ = debug_trait_builder.field("topics", &&(*__self_0_2));
                debug_trait_builder.finish()
            }
        }
    }
}
/// Events for this module.
///
#[doc = r" Event for the System module."]
#[structural_match]
pub enum Event {

    #[doc = r" An extrinsic completed successfully."]
    ExtrinsicSuccess,

    #[doc = r" An extrinsic failed."]
    ExtrinsicFailed,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        match (&*self,) {
            (&Event::ExtrinsicSuccess,) => Event::ExtrinsicSuccess,
            (&Event::ExtrinsicFailed,) => Event::ExtrinsicFailed,
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
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
                match (&*self, &*other) { _ => true, }
            } else { false }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Event {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
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
                    Event::ExtrinsicSuccess => {
                        dest.push_byte(0usize as u8);
                    }
                    Event::ExtrinsicFailed => {
                        dest.push_byte(1usize as u8);
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
                        Some(Event::ExtrinsicSuccess)
                    }
                    x if x == 1usize as u8 => { Some(Event::ExtrinsicFailed) }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Event::ExtrinsicSuccess,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("ExtrinsicSuccess");
                debug_trait_builder.finish()
            }
            (&Event::ExtrinsicFailed,) => {
                let mut debug_trait_builder =
                    f.debug_tuple("ExtrinsicFailed");
                debug_trait_builder.finish()
            }
        }
    }
}
impl From<Event> for () {
    fn from(_: Event) -> () { () }
}
impl Event {
    #[allow(dead_code)]
    pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
        &[::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("ExtrinsicSuccess"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" An extrinsic completed successfully."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("ExtrinsicFailed"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" An extrinsic failed."]),}]
    }
}
/// Origin for the System module.
#[structural_match]
pub enum RawOrigin<AccountId> {

    /// The system itself ordained this dispatch to happen: this is the highest privilege level.
    Root,

    /// It is signed by some public key and we provide the `AccountId`.
    Signed(AccountId),

    /// It is signed by nobody, can be either:
    /// * included and agreed upon by the validators anyway,
    /// * or unsigned transaction validated by a module.
    None,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
 RawOrigin<AccountId> {
    #[inline]
    fn eq(&self, other: &RawOrigin<AccountId>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawOrigin::Signed(ref __self_0),
                     &RawOrigin::Signed(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => true,
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &RawOrigin<AccountId>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawOrigin::Signed(ref __self_0),
                     &RawOrigin::Signed(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => false,
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::Eq> ::std::cmp::Eq for RawOrigin<AccountId> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<AccountId>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::clone::Clone> ::std::clone::Clone for
 RawOrigin<AccountId> {
    #[inline]
    fn clone(&self) -> RawOrigin<AccountId> {
        match (&*self,) {
            (&RawOrigin::Root,) => RawOrigin::Root,
            (&RawOrigin::Signed(ref __self_0),) =>
            RawOrigin::Signed(::std::clone::Clone::clone(&(*__self_0))),
            (&RawOrigin::None,) => RawOrigin::None,
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::fmt::Debug> ::std::fmt::Debug for RawOrigin<AccountId>
 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawOrigin::Root,) => {
                let mut debug_trait_builder = f.debug_tuple("Root");
                debug_trait_builder.finish()
            }
            (&RawOrigin::Signed(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Signed");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawOrigin::None,) => {
                let mut debug_trait_builder = f.debug_tuple("None");
                debug_trait_builder.finish()
            }
        }
    }
}
impl <AccountId> From<Option<AccountId>> for RawOrigin<AccountId> {
    fn from(s: Option<AccountId>) -> RawOrigin<AccountId> {
        match s {
            Some(who) => RawOrigin::Signed(who),
            None => RawOrigin::None,
        }
    }
}
/// Exposed trait-generic origin type.
pub type Origin<T> = RawOrigin<<T as Trait>::AccountId>;
pub type Log<T> = RawLog<<T as Trait>::Hash>;
/// A log in this module.
#[structural_match]
pub enum RawLog<Hash> {

    /// Changes trie has been computed for this block. Contains the root of
    /// changes trie.
    ChangesTrieRoot(Hash),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_RawLog: () =
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
        impl <Hash> _serde::Serialize for RawLog<Hash> where
         Hash: _serde::Serialize {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    RawLog::ChangesTrieRoot(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "RawLog",
                                                                  0u32,
                                                                  "ChangesTrieRoot",
                                                                  __field0),
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Hash: ::std::fmt::Debug> ::std::fmt::Debug for RawLog<Hash> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawLog::ChangesTrieRoot(ref __self_0),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("ChangesTrieRoot");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawLog: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Hash> _parity_codec::Encode for RawLog<Hash> where
         Hash: _parity_codec::Encode, Hash: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawLog::ChangesTrieRoot(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_RawLog: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Hash> _parity_codec::Decode for RawLog<Hash> where
         Hash: _parity_codec::Decode, Hash: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawLog::ChangesTrieRoot(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Hash: ::std::cmp::PartialEq> ::std::cmp::PartialEq for RawLog<Hash> {
    #[inline]
    fn eq(&self, other: &RawLog<Hash>) -> bool {
        match (&*self, &*other) {
            (&RawLog::ChangesTrieRoot(ref __self_0),
             &RawLog::ChangesTrieRoot(ref __arg_1_0)) =>
            (*__self_0) == (*__arg_1_0),
        }
    }
    #[inline]
    fn ne(&self, other: &RawLog<Hash>) -> bool {
        match (&*self, &*other) {
            (&RawLog::ChangesTrieRoot(ref __self_0),
             &RawLog::ChangesTrieRoot(ref __arg_1_0)) =>
            (*__self_0) != (*__arg_1_0),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Hash: ::std::cmp::Eq> ::std::cmp::Eq for RawLog<Hash> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<Hash>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Hash: ::std::clone::Clone> ::std::clone::Clone for RawLog<Hash> {
    #[inline]
    fn clone(&self) -> RawLog<Hash> {
        match (&*self,) {
            (&RawLog::ChangesTrieRoot(ref __self_0),) =>
            RawLog::ChangesTrieRoot(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
impl <Hash: Member> RawLog<Hash> {
    /// Try to cast the log entry as ChangesTrieRoot log entry.
    pub fn as_changes_trie_root(&self) -> Option<&Hash> {
        match *self { RawLog::ChangesTrieRoot(ref item) => Some(item), }
    }
}
#[cfg(any(feature = "std", test))]
impl From<RawLog<substrate_primitives::H256>> for
 primitives::testing::DigestItem {
    fn from(log: RawLog<substrate_primitives::H256>)
     -> primitives::testing::DigestItem {
        match log {
            RawLog::ChangesTrieRoot(root) =>
            primitives::generic::DigestItem::ChangesTrieRoot(root),
        }
    }
}
#[cfg(feature = "std")]
fn hash69<T: AsMut<[u8]> + Default>() -> T {
    let mut h = T::default();
    h.as_mut().iter_mut().for_each(|byte| *byte = 69);
    h
}
/// This type alias represents an index of an event.
///
/// We use `u32` here because this index is used as index for `Events<T>`
/// which can't contain more than `u32::max_value()` items.
type EventIndex = u32;
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = " Extrinsics nonce for accounts."]
pub struct AccountNonce<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   T::Index>
 for AccountNonce<T> {
    type
    Query
    =
    T::Index;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "System AccountNonce".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::Index>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &T::AccountId,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::Index>>::key_for(key);
        storage.get(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &T::AccountId,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::Index>>::key_for(key);
        storage.take(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::AccountId,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::Index>>::get(key,
                                                                                                                                  storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::Index>>::insert(key,
                                                                                                                                 &val,
                                                                                                                                 storage);
        ret
    }
}
#[doc = " Total extrinsics count for the current block."]
struct ExtrinsicCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
 for ExtrinsicCount<T> {
    type
    Query
    =
    Option<u32>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "System ExtrinsicCount".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).or_else(||
                                                                                                                                                      Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).or_else(||
                                                                                                                                                       Default::default())
    }
    #[doc = r" Mutate the value under a key."]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                   F,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&val,
                                                                                                                               storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::kill(storage),
        };
        ret
    }
}
#[doc =
      " Total length in bytes for all extrinsics put together, for the current block."]
struct AllExtrinsicsLen<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
 for AllExtrinsicsLen<T> {
    type
    Query
    =
    Option<u32>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "System AllExtrinsicsLen".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).or_else(||
                                                                                                                                                      Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).or_else(||
                                                                                                                                                       Default::default())
    }
    #[doc = r" Mutate the value under a key."]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                   F,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&val,
                                                                                                                               storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::kill(storage),
        };
        ret
    }
}
#[doc = " Map of block numbers to block hashes."]
pub struct BlockHash<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                   T::Hash>
 for BlockHash<T> {
    type
    Query
    =
    T::Hash;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "System BlockHash".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::BlockNumber)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                                  T::Hash>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &T::BlockNumber,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                                  T::Hash>>::key_for(key);
        storage.get(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &T::BlockNumber,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                                  T::Hash>>::key_for(key);
        storage.take(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::BlockNumber,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                                  T::Hash>>::get(key,
                                                                                                                                 storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                              T::Hash>>::insert(key,
                                                                                                                                &val,
                                                                                                                                storage);
        ret
    }
}
#[doc =
      " Extrinsics data for the current block (maps an extrinsic\'s index to its data)."]
struct ExtrinsicData<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<u32,
                                                                                                   Vec<u8>>
 for ExtrinsicData<T> {
    type
    Query
    =
    Vec<u8>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "System ExtrinsicData".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &u32)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<u32,
                                                                                                                  Vec<u8>>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &u32,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<u32,
                                                                                                                  Vec<u8>>>::key_for(key);
        storage.get(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &u32,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<u32,
                                                                                                                  Vec<u8>>>::key_for(key);
        storage.take(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &u32,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<u32,
                                                                                                                  Vec<u8>>>::get(key,
                                                                                                                                 storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<u32,
                                                                                                              Vec<u8>>>::insert(key,
                                                                                                                                &val,
                                                                                                                                storage);
        ret
    }
}
#[doc =
      " Series of block headers from the last 81 blocks that acts as random seed material. This is arranged as a"]
#[doc =
      " ring buffer with the `i8` prefix being the index into the `Vec` of the oldest hash."]
struct RandomMaterial<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(i8,
                                                                                                      Vec<T::Hash>)>
 for RandomMaterial<T> {
    type
    Query
    =
    (i8, Vec<T::Hash>);
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "System RandomMaterial".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(i8,
                                                                                                                             Vec<T::Hash>)>>::key()).unwrap_or_else(||
                                                                                                                                                                        Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(i8,
                                                                                                                              Vec<T::Hash>)>>::key()).unwrap_or_else(||
                                                                                                                                                                         Default::default())
    }
    #[doc = r" Mutate the value under a key."]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                   F,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(i8,
                                                                                                                     Vec<T::Hash>)>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(i8,
                                                                                                                 Vec<T::Hash>)>>::put(&val,
                                                                                                                                      storage);
        ret
    }
}
#[doc = " The current block number being processed. Set by `execute_block`."]
struct Number<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for Number<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "System Number".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                        Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                         Default::default())
    }
    #[doc = r" Mutate the value under a key."]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                   F,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&val,
                                                                                                                                      storage);
        ret
    }
}
#[doc = " Hash of the previous block."]
struct ParentHash<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>
 for ParentHash<T> {
    type
    Query
    =
    T::Hash;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "System ParentHash".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::key()).unwrap_or_else(||
                                                                                                                                                                 Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::key()).unwrap_or_else(||
                                                                                                                                                                  Default::default())
    }
    #[doc = r" Mutate the value under a key."]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                   F,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::put(&val,
                                                                                                                               storage);
        ret
    }
}
#[doc =
      " Extrinsics root of the current block, also part of the block header."]
struct ExtrinsicsRoot<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>
 for ExtrinsicsRoot<T> {
    type
    Query
    =
    T::Hash;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "System ExtrinsicsRoot".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::key()).unwrap_or_else(||
                                                                                                                                                                 Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::key()).unwrap_or_else(||
                                                                                                                                                                  Default::default())
    }
    #[doc = r" Mutate the value under a key."]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                   F,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::put(&val,
                                                                                                                               storage);
        ret
    }
}
#[doc = " Digest of the current block, also part of the block header."]
struct Digest<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Digest>
 for Digest<T> {
    type
    Query
    =
    T::Digest;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "System Digest".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Digest>>::key()).unwrap_or_else(||
                                                                                                                                                                   Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Digest>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Mutate the value under a key."]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                   F,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Digest>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Digest>>::put(&val,
                                                                                                                                 storage);
        ret
    }
}
#[doc = " Events deposited for the current block."]
struct Events<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<EventRecord<T::Event,
                                                                                                                     T::Hash>>>
 for Events<T> {
    type
    Query
    =
    Vec<EventRecord<T::Event, T::Hash>>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "System Events".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<EventRecord<T::Event,
                                                                                                                                            T::Hash>>>>::key()).unwrap_or_else(||
                                                                                                                                                                                   Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<EventRecord<T::Event,
                                                                                                                                             T::Hash>>>>::key()).unwrap_or_else(||
                                                                                                                                                                                    Default::default())
    }
    #[doc = r" Mutate the value under a key."]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                   F,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<EventRecord<T::Event,
                                                                                                                                    T::Hash>>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<EventRecord<T::Event,
                                                                                                                                T::Hash>>>>::put(&val,
                                                                                                                                                 storage);
        ret
    }
}
#[doc = " The number of events in the `Events<T>` list."]
struct EventCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<EventIndex>
 for EventCount<T> {
    type
    Query
    =
    EventIndex;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "System EventCount".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<EventIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<EventIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                     Default::default())
    }
    #[doc = r" Mutate the value under a key."]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(f:
                                                                                                                                                                   F,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<EventIndex>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<EventIndex>>::put(&val,
                                                                                                                                  storage);
        ret
    }
}
#[doc =
      " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
#[doc = " of events in the `<Events<T>>` list."]
#[doc = ""]
#[doc =
      " The first key serves no purpose. This field is declared as double_map just"]
#[doc = " for convenience of using `remove_prefix`."]
#[doc = ""]
#[doc =
      " All topic vectors have deterministic storage locations depending on the topic. This"]
#[doc =
      " allows light-clients to leverage the changes trie storage tracking mechanism and"]
#[doc = " in case of changes fetch the list of events of interest."]
#[doc = ""]
#[doc =
      " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
#[doc =
      " the `EventIndex` then in case if the topic has the same contents on the next block"]
#[doc = " no notification will be triggered thus the event might be lost."]
struct EventTopics<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::unhashed::generator::StorageDoubleMap<(),
                                                                                                           T::Hash,
                                                                                                           Vec<(T::BlockNumber,
                                                                                                                EventIndex)>>
 for EventTopics<T> {
    type
    Query
    =
    Vec<(T::BlockNumber, EventIndex)>;
    fn prefix_for(k1: &()) -> Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageHasher;
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::unhashed::generator::StorageDoubleMap<(),
                                                                                                                          T::Hash,
                                                                                                                          Vec<(T::BlockNumber,
                                                                                                                               EventIndex)>>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(k1,
                                                                                            &mut key);
        self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256::hash(&key[..]).to_vec()
    }
    fn prefix() -> &'static [u8] { "System EventTopics".as_bytes() }
    fn key_for(k1: &(), k2: &T::Hash) -> Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::unhashed::generator::StorageDoubleMap<(),
                                                                                                                          T::Hash,
                                                                                                                          Vec<(T::BlockNumber,
                                                                                                                               EventIndex)>>>::prefix_for(k1);
        key.extend(&self::sr_api_hidden_includes_decl_storage::hidden_include::Hashable::blake2_256(k2));
        key
    }
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::UnhashedStorage>(key1:
                                                                                              &(),
                                                                                          key2:
                                                                                              &T::Hash,
                                                                                          storage:
                                                                                              &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::unhashed::generator::StorageDoubleMap<(),
                                                                                                                          T::Hash,
                                                                                                                          Vec<(T::BlockNumber,
                                                                                                                               EventIndex)>>>::key_for(key1,
                                                                                                                                                       key2);
        storage.get(&key).unwrap_or_else(|| Default::default())
    }
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::UnhashedStorage>(key1:
                                                                                               &(),
                                                                                           key2:
                                                                                               &T::Hash,
                                                                                           storage:
                                                                                               &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::unhashed::generator::StorageDoubleMap<(),
                                                                                                                          T::Hash,
                                                                                                                          Vec<(T::BlockNumber,
                                                                                                                               EventIndex)>>>::key_for(key1,
                                                                                                                                                       key2);
        storage.take(&key).unwrap_or_else(|| Default::default())
    }
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::UnhashedStorage>(key1:
                                                                                                 &(),
                                                                                             key2:
                                                                                                 &T::Hash,
                                                                                             f:
                                                                                                 F,
                                                                                             storage:
                                                                                                 &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::unhashed::generator::StorageDoubleMap<(),
                                                                                                                          T::Hash,
                                                                                                                          Vec<(T::BlockNumber,
                                                                                                                               EventIndex)>>>::get(key1,
                                                                                                                                                   key2,
                                                                                                                                                   storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::unhashed::generator::StorageDoubleMap<(),
                                                                                                                      T::Hash,
                                                                                                                      Vec<(T::BlockNumber,
                                                                                                                           EventIndex)>>>::insert(key1,
                                                                                                                                                  key2,
                                                                                                                                                  &val,
                                                                                                                                                  storage);
        ret
    }
}
trait Store {
    type
    AccountNonce;
    type
    ExtrinsicCount;
    type
    AllExtrinsicsLen;
    type
    BlockHash;
    type
    ExtrinsicData;
    type
    RandomMaterial;
    type
    Number;
    type
    ParentHash;
    type
    ExtrinsicsRoot;
    type
    Digest;
    type
    Events;
    type
    EventCount;
    type
    EventTopics;
}
#[doc(hidden)]
pub struct __GetByteStructAccountNonce<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_AccountNonce:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructAccountNonce<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_AccountNonce.get_or_init(||
                                                             {
                                                                 let def_val:
                                                                         T::Index =
                                                                     Default::default();
                                                                 <T::Index as
                                                                     Encode>::encode(&def_val)
                                                             }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructExtrinsicCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ExtrinsicCount:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructExtrinsicCount<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ExtrinsicCount.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           Option<u32> =
                                                                       Default::default();
                                                                   <Option<u32>
                                                                       as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructAllExtrinsicsLen<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_AllExtrinsicsLen:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructAllExtrinsicsLen<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_AllExtrinsicsLen.get_or_init(||
                                                                 {
                                                                     let def_val:
                                                                             Option<u32> =
                                                                         Default::default();
                                                                     <Option<u32>
                                                                         as
                                                                         Encode>::encode(&def_val)
                                                                 }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructBlockHash<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_BlockHash:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructBlockHash<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_BlockHash.get_or_init(||
                                                          {
                                                              let def_val:
                                                                      T::Hash =
                                                                  Default::default();
                                                              <T::Hash as
                                                                  Encode>::encode(&def_val)
                                                          }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructExtrinsicData<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ExtrinsicData:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructExtrinsicData<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ExtrinsicData.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          Vec<u8> =
                                                                      Default::default();
                                                                  <Vec<u8> as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructRandomMaterial<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_RandomMaterial:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructRandomMaterial<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_RandomMaterial.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           (i8,
                                                                            Vec<T::Hash>) =
                                                                       Default::default();
                                                                   <(i8,
                                                                     Vec<T::Hash>)
                                                                       as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructNumber<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Number:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNumber<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Number.get_or_init(||
                                                       {
                                                           let def_val:
                                                                   T::BlockNumber =
                                                               Default::default();
                                                           <T::BlockNumber as
                                                               Encode>::encode(&def_val)
                                                       }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructParentHash<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ParentHash:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructParentHash<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ParentHash.get_or_init(||
                                                           {
                                                               let def_val:
                                                                       T::Hash =
                                                                   Default::default();
                                                               <T::Hash as
                                                                   Encode>::encode(&def_val)
                                                           }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructExtrinsicsRoot<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ExtrinsicsRoot:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructExtrinsicsRoot<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ExtrinsicsRoot.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           T::Hash =
                                                                       Default::default();
                                                                   <T::Hash as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructDigest<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Digest:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructDigest<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Digest.get_or_init(||
                                                       {
                                                           let def_val:
                                                                   T::Digest =
                                                               Default::default();
                                                           <T::Digest as
                                                               Encode>::encode(&def_val)
                                                       }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructEvents<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Events:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructEvents<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Events.get_or_init(||
                                                       {
                                                           let def_val:
                                                                   Vec<EventRecord<T::Event,
                                                                                   T::Hash>> =
                                                               Default::default();
                                                           <Vec<EventRecord<T::Event,
                                                                            T::Hash>>
                                                               as
                                                               Encode>::encode(&def_val)
                                                       }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructEventCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_EventCount:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructEventCount<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_EventCount.get_or_init(||
                                                           {
                                                               let def_val:
                                                                       EventIndex =
                                                                   Default::default();
                                                               <EventIndex as
                                                                   Encode>::encode(&def_val)
                                                           }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructEventTopics<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_EventTopics:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructEventTopics<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_EventTopics.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        Vec<(T::BlockNumber,
                                                                             EventIndex)> =
                                                                    Default::default();
                                                                <Vec<(T::BlockNumber,
                                                                      EventIndex)>
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    AccountNonce
    =
    AccountNonce<T>;
    type
    ExtrinsicCount
    =
    ExtrinsicCount<T>;
    type
    AllExtrinsicsLen
    =
    AllExtrinsicsLen<T>;
    type
    BlockHash
    =
    BlockHash<T>;
    type
    ExtrinsicData
    =
    ExtrinsicData<T>;
    type
    RandomMaterial
    =
    RandomMaterial<T>;
    type
    Number
    =
    Number<T>;
    type
    ParentHash
    =
    ParentHash<T>;
    type
    ExtrinsicsRoot
    =
    ExtrinsicsRoot<T>;
    type
    Digest
    =
    Digest<T>;
    type
    Events
    =
    Events<T>;
    type
    EventCount
    =
    EventCount<T>;
    type
    EventTopics
    =
    EventTopics<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc = " Extrinsics nonce for accounts."]
    pub fn account_nonce<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                               K)
     -> T::Index {
        <AccountNonce<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::Index>>::get(key.borrow(),
                                                                                                                              &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Map of block numbers to block hashes."]
    pub fn block_hash<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::BlockNumber>>(key:
                                                                                                                              K)
     -> T::Hash {
        <BlockHash<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                              T::Hash>>::get(key.borrow(),
                                                                                                                             &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Extrinsics data for the current block (maps an extrinsic\'s index to its data)."]
    pub fn extrinsic_data<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<u32>>(key:
                                                                                                                       K)
     -> Vec<u8> {
        <ExtrinsicData<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<u32,
                                                                                                              Vec<u8>>>::get(key.borrow(),
                                                                                                                             &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Series of block headers from the last 81 blocks that acts as random seed material. This is arranged as a"]
    #[doc =
          " ring buffer with the `i8` prefix being the index into the `Vec` of the oldest hash."]
    pub fn random_material() -> (i8, Vec<T::Hash>) {
        <RandomMaterial<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<(i8,
                                                                                                                 Vec<T::Hash>)>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The current block number being processed. Set by `execute_block`."]
    pub fn block_number() -> T::BlockNumber {
        <Number<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Hash of the previous block."]
    pub fn parent_hash() -> T::Hash {
        <ParentHash<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Extrinsics root of the current block, also part of the block header."]
    pub fn extrinsics_root() -> T::Hash {
        <ExtrinsicsRoot<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Digest of the current block, also part of the block header."]
    pub fn digest() -> T::Digest {
        <Digest<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Digest>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Events deposited for the current block."]
    pub fn events() -> Vec<EventRecord<T::Event, T::Hash>> {
        <Events<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<EventRecord<T::Event,
                                                                                                                                T::Hash>>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The number of events in the `Events<T>` list."]
    pub fn event_count() -> EventIndex {
        <EventCount<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<EventIndex>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    pub fn event_topics<KArg1, KArg2>(k1: KArg1, k2: KArg2)
     -> Vec<(T::BlockNumber, EventIndex)> where
     KArg1: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<()>,
     KArg2: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::Hash> {
        <EventTopics<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::unhashed::generator::StorageDoubleMap<(),
                                                                                                                      T::Hash,
                                                                                                                      Vec<(T::BlockNumber,
                                                                                                                           EventIndex)>>>::get(k1.borrow(),
                                                                                                                                               k2.borrow(),
                                                                                                                                               &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AccountNonce"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Index"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructAccountNonce::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Extrinsics nonce for accounts."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ExtrinsicCount"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructExtrinsicCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Total extrinsics count for the current block."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AllExtrinsicsLen"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructAllExtrinsicsLen::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Total length in bytes for all extrinsics put together, for the current block."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BlockHash"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBlockHash::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Map of block numbers to block hashes."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ExtrinsicData"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<u8>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructExtrinsicData::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Extrinsics data for the current block (maps an extrinsic\'s index to its data)."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RandomMaterial"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(i8, Vec<T::Hash>)")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRandomMaterial::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Series of block headers from the last 81 blocks that acts as random seed material. This is arranged as a",
                                                                                                                                                                                                                                                                                                                                                                                                    " ring buffer with the `i8` prefix being the index into the `Vec` of the oldest hash."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Number"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNumber::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The current block number being processed. Set by `execute_block`."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ParentHash"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructParentHash::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Hash of the previous block."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ExtrinsicsRoot"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructExtrinsicsRoot::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Extrinsics root of the current block, also part of the block header."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Digest"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Digest")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDigest::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Digest of the current block, also part of the block header."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Events"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<EventRecord<T::Event, T::Hash>>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEvents::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Events deposited for the current block."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EventCount"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EventIndex")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEventCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of events in the `Events<T>` list."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EventTopics"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::DoubleMap{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                             self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                         key1:
                                                                                                                                                                                                                                                                                                                                                                                                             self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("()"),
                                                                                                                                                                                                                                                                                                                                                                                                         key2:
                                                                                                                                                                                                                                                                                                                                                                                                             self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                                                                                                                                                                                                         value:
                                                                                                                                                                                                                                                                                                                                                                                                             self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(T::BlockNumber, EventIndex)>"),
                                                                                                                                                                                                                                                                                                                                                                                                         key2_hasher:
                                                                                                                                                                                                                                                                                                                                                                                                             self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("blake2_256"),},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEventTopics::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Mapping between a topic (represented by T::Hash) and a vector of indexes",
                                                                                                                                                                                                                                                                                                                                                                                                    " of events in the `<Events<T>>` list.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " The first key serves no purpose. This field is declared as double_map just",
                                                                                                                                                                                                                                                                                                                                                                                                    " for convenience of using `remove_prefix`.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " All topic vectors have deterministic storage locations depending on the topic. This",
                                                                                                                                                                                                                                                                                                                                                                                                    " allows light-clients to leverage the changes trie storage tracking mechanism and",
                                                                                                                                                                                                                                                                                                                                                                                                    " in case of changes fetch the list of events of interest.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just",
                                                                                                                                                                                                                                                                                                                                                                                                    " the `EventIndex` then in case if the topic has the same contents on the next block",
                                                                                                                                                                                                                                                                                                                                                                                                    " no notification will be triggered thus the event might be lost."]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AccountNonce"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Index"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructAccountNonce::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Extrinsics nonce for accounts."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ExtrinsicCount"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructExtrinsicCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Total extrinsics count for the current block."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AllExtrinsicsLen"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructAllExtrinsicsLen::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Total length in bytes for all extrinsics put together, for the current block."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BlockHash"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBlockHash::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Map of block numbers to block hashes."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ExtrinsicData"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<u8>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructExtrinsicData::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Extrinsics data for the current block (maps an extrinsic\'s index to its data)."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RandomMaterial"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(i8, Vec<T::Hash>)")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRandomMaterial::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Series of block headers from the last 81 blocks that acts as random seed material. This is arranged as a",
                                                                                                                                                                                                              " ring buffer with the `i8` prefix being the index into the `Vec` of the oldest hash."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Number"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNumber::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The current block number being processed. Set by `execute_block`."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ParentHash"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructParentHash::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Hash of the previous block."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ExtrinsicsRoot"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructExtrinsicsRoot::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Extrinsics root of the current block, also part of the block header."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Digest"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Digest")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDigest::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Digest of the current block, also part of the block header."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Events"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<EventRecord<T::Event, T::Hash>>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEvents::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Events deposited for the current block."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EventCount"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EventIndex")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEventCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of events in the `Events<T>` list."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EventTopics"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::DoubleMap{hasher:
                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                   key1:
                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("()"),
                                                                                                                                                                                                                   key2:
                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Hash"),
                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(T::BlockNumber, EventIndex)>"),
                                                                                                                                                                                                                   key2_hasher:
                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("blake2_256"),},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEventTopics::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Mapping between a topic (represented by T::Hash) and a vector of indexes",
                                                                                                                                                                                                              " of events in the `<Events<T>>` list.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " The first key serves no purpose. This field is declared as double_map just",
                                                                                                                                                                                                              " for convenience of using `remove_prefix`.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " All topic vectors have deterministic storage locations depending on the topic. This",
                                                                                                                                                                                                              " allows light-clients to leverage the changes trie storage tracking mechanism and",
                                                                                                                                                                                                              " in case of changes fetch the list of events of interest.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just",
                                                                                                                                                                                                              " the `EventIndex` then in case if the topic has the same contents on the next block",
                                                                                                                                                                                                              " no notification will be triggered thus the event might be lost."]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "System" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "Option < ChangesTrieConfiguration > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "Option < ChangesTrieConfiguration > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    #[serde(skip)]
    pub _genesis_phantom_data: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    pub changes_trie_config: Option<ChangesTrieConfiguration>,
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
        impl <T: Trait> _serde::Serialize for GenesisConfig<T> where
         Option<ChangesTrieConfiguration>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
         {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "GenesisConfig",
                                                               false as usize
                                                                   + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "changesTrieConfig",
                                                                    &self.changes_trie_config)
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
        impl <'de, T: Trait> _serde::Deserialize<'de> for GenesisConfig<T>
         where
         Option<ChangesTrieConfiguration>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
         {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field { __field1, }
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
                            0u64 => _serde::export::Ok(__Field::__field1),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 1")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "changesTrieConfig" =>
                            _serde::export::Ok(__Field::__field1),
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
                            b"changesTrieConfig" =>
                            _serde::export::Ok(__Field::__field1),
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
                struct __Visitor<'de, T: Trait> where
                       Option<ChangesTrieConfiguration>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 Option<ChangesTrieConfiguration>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
                 {
                    type
                    Value
                    =
                    GenesisConfig<T>;
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
                        let __field0 = _serde::export::Default::default();
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Option<ChangesTrieConfiguration>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct GenesisConfig with 1 element"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{_genesis_phantom_data:
                                                             __field0,
                                                         changes_trie_config:
                                                             __field1,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field1:
                                _serde::export::Option<Option<ChangesTrieConfiguration>> =
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
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("changesTrieConfig"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Option<ChangesTrieConfiguration>>(&mut __map)
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
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("changesTrieConfig")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{_genesis_phantom_data:
                                                             _serde::export::Default::default(),
                                                         changes_trie_config:
                                                             __field1,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["changesTrieConfig"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "GenesisConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<GenesisConfig<T>>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[cfg(feature = "std")]
impl <T: Trait> Default for GenesisConfig<T> {
    fn default() -> Self {
        GenesisConfig{_genesis_phantom_data: Default::default(),
                      changes_trie_config: Default::default(),}
    }
}
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::BuildStorage
 for GenesisConfig<T> {
    fn assimilate_storage(self,
                          r:
                              &mut self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::StorageOverlay,
                          c:
                              &mut self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::ChildrenStorageOverlay)
     -> ::std::result::Result<(), String> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::cell::RefCell;
        let storage = RefCell::new(r);
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let data =
                (|_|
                     <[_]>::into_vec(box
                                         [(T::BlockNumber::zero(),
                                           hash69())]))(&self);
            for (k, v) in data.into_iter() {
                <BlockHash<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                                      T::Hash>>::insert(&k,
                                                                                                                                        &v,
                                                                                                                                        &storage);
            }
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v = (|_| T::BlockNumber::sa(1u64))(&self);
            <Number<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                          &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v = (|_| hash69())(&self);
            <ParentHash<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Hash>>::put(&v,
                                                                                                                                   &storage);
        }
        let r = storage.into_inner();
        (|storage: &mut primitives::StorageOverlay,
          _: &mut primitives::ChildrenStorageOverlay,
          config: &GenesisConfig<T>|
             {
                 use parity_codec::Encode;
                 storage.insert(well_known_keys::EXTRINSIC_INDEX.to_vec(),
                                0u32.encode());
                 if let Some(ref changes_trie_config) =
                        config.changes_trie_config {
                     storage.insert(well_known_keys::CHANGES_TRIE_CONFIG.to_vec(),
                                    changes_trie_config.encode());
                 }
             })(r, c, &self);
        Ok(())
    }
}
pub struct EnsureRoot<AccountId>(::rstd::marker::PhantomData<AccountId>);
impl <O: Into<Option<RawOrigin<AccountId>>>, AccountId> EnsureOrigin<O> for
 EnsureRoot<AccountId> {
    type
    Success
    =
    ();
    fn ensure_origin(o: O) -> Result<Self::Success, &'static str> {
        ensure_root(o)
    }
}
/// Ensure that the origin `o` represents a signed extrinsic (i.e. transaction).
/// Returns `Ok` with the account that signed the extrinsic or an `Err` otherwise.
pub fn ensure_signed<OuterOrigin, AccountId>(o: OuterOrigin)
 -> Result<AccountId, &'static str> where
 OuterOrigin: Into<Option<RawOrigin<AccountId>>> {
    match o.into() {
        Some(RawOrigin::Signed(t)) => Ok(t),
        _ => Err("bad origin: expected to be a signed origin"),
    }
}
/// Ensure that the origin `o` represents the root. Returns `Ok` or an `Err` otherwise.
pub fn ensure_root<OuterOrigin, AccountId>(o: OuterOrigin)
 -> Result<(), &'static str> where
 OuterOrigin: Into<Option<RawOrigin<AccountId>>> {
    match o.into() {
        Some(RawOrigin::Root) => Ok(()),
        _ => Err("bad origin: expected to be a root origin"),
    }
}
/// Ensure that the origin `o` represents an unsigned extrinsic. Returns `Ok` or an `Err` otherwise.
pub fn ensure_none<OuterOrigin, AccountId>(o: OuterOrigin)
 -> Result<(), &'static str> where
 OuterOrigin: Into<Option<RawOrigin<AccountId>>> {
    match o.into() {
        Some(RawOrigin::None) => Ok(()),
        _ => Err("bad origin: expected to be no origin"),
    }
}
impl <T: Trait> Module<T> {
    /// Deposits an event into this block's event record adding this event
    /// to the corresponding topic indexes.
    ///
    /// This will update storage entries that correpond to the specified topics.
    /// It is expected that light-clients could subscribe to this topics.
    pub fn deposit_event_indexed(topics: &[T::Hash], event: T::Event) {
        let extrinsic_index = Self::extrinsic_index();
        let phase =
            extrinsic_index.map_or(Phase::Finalization,
                                   |c| Phase::ApplyExtrinsic(c));
        let event =
            EventRecord{phase,
                        event,
                        topics: topics.iter().cloned().collect::<Vec<_>>(),};
        let event_idx =
            {
                let old_event_count = <EventCount<T>>::get();
                let new_event_count =
                    match old_event_count.checked_add(1) {
                        None => return,
                        Some(nc) => nc,
                    };
                <EventCount<T>>::put(new_event_count);
                old_event_count
            };
        if <Events<T>>::append(&[event]).is_err() { return; }
        let block_no = Self::block_number();
        for topic in topics {
            if <EventTopics<T>>::append(&(), topic,
                                        &[(block_no, event_idx)]).is_err() {
                return;
            }
        }
    }
    /// Gets the index of extrinsic that is currently executing.
    pub fn extrinsic_index() -> Option<u32> {
        storage::unhashed::get(well_known_keys::EXTRINSIC_INDEX)
    }
    /// Gets extrinsics count.
    pub fn extrinsic_count() -> u32 {
        <ExtrinsicCount<T>>::get().unwrap_or_default()
    }
    /// Gets a total length of all executed extrinsics.
    pub fn all_extrinsics_len() -> u32 {
        <AllExtrinsicsLen<T>>::get().unwrap_or_default()
    }
    /// Start the execution of a particular block.
    pub fn initialize(number: &T::BlockNumber, parent_hash: &T::Hash,
                      txs_root: &T::Hash) {
        storage::unhashed::put(well_known_keys::EXTRINSIC_INDEX, &0u32);
        <Number<T>>::put(number);
        <ParentHash<T>>::put(parent_hash);
        <BlockHash<T>>::insert(*number - One::one(), parent_hash);
        <ExtrinsicsRoot<T>>::put(txs_root);
        <RandomMaterial<T>>::mutate(|&mut (ref mut index, ref mut values)|
                                        if values.len() < 81 {
                                            values.push(parent_hash.clone())
                                        } else {
                                            values[*index as usize] =
                                                parent_hash.clone();
                                            *index = (*index + 1) % 81;
                                        });
        <Events<T>>::kill();
        <EventCount<T>>::kill();
        <EventTopics<T>>::remove_prefix(&());
    }
    /// Remove temporary "environment" entries in storage.
    pub fn finalize() -> T::Header {
        <ExtrinsicCount<T>>::kill();
        <AllExtrinsicsLen<T>>::kill();
        let number = <Number<T>>::take();
        let parent_hash = <ParentHash<T>>::take();
        let mut digest = <Digest<T>>::take();
        let extrinsics_root = <ExtrinsicsRoot<T>>::take();
        let storage_root = T::Hashing::storage_root();
        let storage_changes_root =
            T::Hashing::storage_changes_root(parent_hash, number.as_() - 1);
        if let Some(storage_changes_root) = storage_changes_root {
            let item = RawLog::ChangesTrieRoot(storage_changes_root);
            let item = <T as Trait>::from(item).into();
            digest.push(item);
        }
        <T::Header as
            traits::Header>::new(number, extrinsics_root, storage_root,
                                 parent_hash, digest)
    }
    /// Deposits a log and ensures it matches the block's log data.
    pub fn deposit_log(item: <T::Digest as traits::Digest>::Item) {
        let mut l = <Digest<T>>::get();
        traits::Digest::push(&mut l, item);
        <Digest<T>>::put(l);
    }
    /// Get the basic externalities for this module, useful for tests.
    #[cfg(any(feature = "std", test))]
    pub fn externalities() -> TestExternalities<Blake2Hasher> {
        TestExternalities::new(<[_]>::into_vec(box
                                                   [(twox_128(&<BlockHash<T>>::key_for(T::BlockNumber::zero())).to_vec(),
                                                     [69u8; 32].encode()),
                                                    (twox_128(<Number<T>>::key()).to_vec(),
                                                     T::BlockNumber::one().encode()),
                                                    (twox_128(<ParentHash<T>>::key()).to_vec(),
                                                     [69u8;
                                                         32].encode())]).into_iter().collect())
    }
    /// Set the block number to something in particular. Can be used as an alternative to
    /// `initialize` for tests that don't need to bother with the other environment entries.
    #[cfg(any(feature = "std", test))]
    pub fn set_block_number(n: T::BlockNumber) { <Number<T>>::put(n); }
    /// Sets the index of extrinsic that is currently executing.
    #[cfg(any(feature = "std", test))]
    pub fn set_extrinsic_index(extrinsic_index: u32) {
        storage::unhashed::put(well_known_keys::EXTRINSIC_INDEX,
                               &extrinsic_index)
    }
    /// Set the parent hash number to something in particular. Can be used as an alternative to
    /// `initialize` for tests that don't need to bother with the other environment entries.
    #[cfg(any(feature = "std", test))]
    pub fn set_parent_hash(n: T::Hash) { <ParentHash<T>>::put(n); }
    /// Get the basic random seed.
    ///
    /// In general you won't want to use this, but rather `Self::random` which allows you to give a subject for the
    /// random result and whose value will be independently low-influence random from any other such seeds.
    pub fn random_seed() -> T::Hash { Self::random(&[][..]) }
    /// Get a low-influence "random" value.
    ///
    /// Being a deterministic block chain, real randomness is difficult to come by. This gives you something that
    /// approximates it. `subject` is a context identifier and allows you to get a different result to other callers
    /// of this function; use it like `random(&b"my context"[..])`.
    ///
    /// This is initially implemented through a low-influence "triplet mix" convolution of previous block hash values.
    /// In the future it will be generated from a secure "VRF".
    ///
    /// ### Security Notes
    /// This randomness uses a low-influence function, drawing upon the block hashes from the previous 81 blocks. Its
    /// result for any given subject will be known in advance by the block producer of this block (and, indeed, anyone
    /// who knows the block's `parent_hash`). However, it is mostly impossible for the producer of this block *alone*
    /// to influence the value of this hash. A sizable minority of dishonest and coordinating block producers would be
    /// required in order to affect this value. If that is an insufficient security guarantee then two things can be
    /// used to improve this randomness:
    /// - Name, in advance, the block number whose random value will be used; ensure your module retains a buffer of
    /// previous random values for its subject and then index into these in order to obviate the ability of your user
    /// to look up the parent hash and choose when to transact based upon it.
    /// - Require your user to first commit to an additional value by first posting its hash. Require them to reveal
    /// the value to determine the final result, hashing it with the output of this random function. This reduces the
    /// ability of a cabal of block producers from conspiring against individuals.
    ///
    /// WARNING: Hashing the result of this function will remove any low-infleunce properties it has and mean that
    /// all bits of the resulting value are entirely manipulatable by the author of the parent block, who can determine
    /// the value of `parent_hash`.
    pub fn random(subject: &[u8]) -> T::Hash {
        let (index, hash_series) = <RandomMaterial<T>>::get();
        if hash_series.len() > 0 {
            hash_series.iter().cycle().skip(index as
                                                usize).take(81).enumerate().map(|(i,
                                                                                  h)|
                                                                                    (i
                                                                                         as
                                                                                         i8,
                                                                                     subject,
                                                                                     h).using_encoded(T::Hashing::hash)).triplet_mix()
        } else { T::Hash::default() }
    }
    /// Increment a particular account's nonce by 1.
    pub fn inc_account_nonce(who: &T::AccountId) {
        <AccountNonce<T>>::insert(who,
                                  Self::account_nonce(who) + T::Index::one());
    }
    /// Note what the extrinsic data of the current extrinsic index is. If this is called, then
    /// ensure `derive_extrinsics` is also called before block-building is completed.
    ///
    /// NOTE: This function is called only when the block is being constructed locally.
    /// `execute_block` doesn't note any extrinsics.
    pub fn note_extrinsic(encoded_xt: Vec<u8>) {
        <ExtrinsicData<T>>::insert(Self::extrinsic_index().unwrap_or_default(),
                                   encoded_xt);
    }
    /// To be called immediately after an extrinsic has been applied.
    pub fn note_applied_extrinsic(r: &Result<(), &'static str>,
                                  encoded_len: u32) {
        Self::deposit_event(match r {
                                Ok(_) => Event::ExtrinsicSuccess,
                                Err(_) => Event::ExtrinsicFailed,
                            }.into());
        let next_extrinsic_index =
            Self::extrinsic_index().unwrap_or_default() + 1u32;
        let total_length =
            encoded_len.saturating_add(Self::all_extrinsics_len());
        storage::unhashed::put(well_known_keys::EXTRINSIC_INDEX,
                               &next_extrinsic_index);
        <AllExtrinsicsLen<T>>::put(&total_length);
    }
    /// To be called immediately after `note_applied_extrinsic` of the last extrinsic of the block
    /// has been called.
    pub fn note_finished_extrinsics() {
        let extrinsic_index: u32 =
            storage::unhashed::take(well_known_keys::EXTRINSIC_INDEX).unwrap_or_default();
        <ExtrinsicCount<T>>::put(extrinsic_index);
    }
    /// Remove all extrinsic data and save the extrinsics trie root.
    pub fn derive_extrinsics() {
        let extrinsics =
            (0..<ExtrinsicCount<T>>::get().unwrap_or_default()).map(<ExtrinsicData<T>>::take).collect();
        let xts_root = extrinsics_data_root::<T::Hashing>(extrinsics);
        <ExtrinsicsRoot<T>>::put(xts_root);
    }
}
pub struct ChainContext<T>(::rstd::marker::PhantomData<T>);
impl <T> Default for ChainContext<T> {
    fn default() -> Self { ChainContext(::rstd::marker::PhantomData) }
}
impl <T: Trait> Lookup for ChainContext<T> {
    type
    Source
    =
    <T::Lookup as StaticLookup>::Source;
    type
    Target
    =
    <T::Lookup as StaticLookup>::Target;
    fn lookup(&self, s: Self::Source)
     -> rstd::result::Result<Self::Target, &'static str> {
        <T::Lookup as StaticLookup>::lookup(s)
    }
}
impl <T: Trait> CurrentHeight for ChainContext<T> {
    type
    BlockNumber
    =
    T::BlockNumber;
    fn current_height(&self) -> Self::BlockNumber {
        <Module<T>>::block_number()
    }
}
impl <T: Trait> BlockNumberToHash for ChainContext<T> {
    type
    BlockNumber
    =
    T::BlockNumber;
    type
    Hash
    =
    T::Hash;
    fn block_number_to_hash(&self, n: Self::BlockNumber)
     -> Option<Self::Hash> {
        Some(<Module<T>>::block_hash(n))
    }
}
