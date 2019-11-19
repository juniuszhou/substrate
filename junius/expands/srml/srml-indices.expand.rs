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

//! An index is a short form of an address. This module handles allocation
//! of indices for a newly created accounts.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


use rstd::{prelude::*, result, marker::PhantomData};
use parity_codec::{Encode, Decode, Codec, Input, Output};
use srml_support::{StorageValue, StorageMap, Parameter, decl_module,
                   decl_event, decl_storage};
use primitives::traits::{One, SimpleArithmetic, As, StaticLookup, Member};
use system::{IsDeadAccount, OnNewAccount};

use self::address::Address as RawAddress;


pub mod address {













    // PUBLIC IMMUTABLES




    // PUBLIC MUTABLES (DANGEROUS)



    // then check to see if this account id identifies a dead account index.
    // yup - this index refers to a dead account. can be reused.


    // insert normally as a back up
    // defensive only: this loop should never iterate since we keep NextEnumSet up to date later.


    // update set.

    // keep NextEnumSet up to date

    // write set.


    //! Address type that is union of index and id for an account.
    #[cfg(feature = "std")]
    use std::fmt;
    use crate::{Member, Decode, Encode, As, Input, Output};
    /// An indices-aware address, which can be either a direct `AccountId` or
    /// an index.
    #[structural_match]
    pub enum Address<AccountId, AccountIndex> where AccountId: Member,
             AccountIndex: Member {

        /// It's an account ID (pubkey).
        Id(AccountId),

        /// It's an account index.
        Index(AccountIndex),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::cmp::PartialEq,
          AccountIndex: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
     Address<AccountId, AccountIndex> where AccountId: Member,
     AccountIndex: Member {
        #[inline]
        fn eq(&self, other: &Address<AccountId, AccountIndex>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Address::Id(ref __self_0),
                         &Address::Id(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&Address::Index(ref __self_0),
                         &Address::Index(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &Address<AccountId, AccountIndex>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Address::Id(ref __self_0),
                         &Address::Id(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&Address::Index(ref __self_0),
                         &Address::Index(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::cmp::Eq, AccountIndex: ::std::cmp::Eq>
     ::std::cmp::Eq for Address<AccountId, AccountIndex> where
     AccountId: Member, AccountIndex: Member {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<AccountId>;
                let _: ::std::cmp::AssertParamIsEq<AccountIndex>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::clone::Clone, AccountIndex: ::std::clone::Clone>
     ::std::clone::Clone for Address<AccountId, AccountIndex> where
     AccountId: Member, AccountIndex: Member {
        #[inline]
        fn clone(&self) -> Address<AccountId, AccountIndex> {
            match (&*self,) {
                (&Address::Id(ref __self_0),) =>
                Address::Id(::std::clone::Clone::clone(&(*__self_0))),
                (&Address::Index(ref __self_0),) =>
                Address::Index(::std::clone::Clone::clone(&(*__self_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::fmt::Debug, AccountIndex: ::std::fmt::Debug>
     ::std::fmt::Debug for Address<AccountId, AccountIndex> where
     AccountId: Member, AccountIndex: Member {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Address::Id(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Id");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Address::Index(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Index");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::hash::Hash, AccountIndex: ::std::hash::Hash>
     ::std::hash::Hash for Address<AccountId, AccountIndex> where
     AccountId: Member, AccountIndex: Member {
        fn hash<__HAccountIdAccountIndex: ::std::hash::Hasher>(&self,
                                                               state:
                                                                   &mut __HAccountIdAccountIndex)
         -> () {
            match (&*self,) {
                (&Address::Id(ref __self_0),) => {
                    ::std::hash::Hash::hash(&unsafe {
                                                 ::std::intrinsics::discriminant_value(self)
                                             }, state);
                    ::std::hash::Hash::hash(&(*__self_0), state)
                }
                (&Address::Index(ref __self_0),) => {
                    ::std::hash::Hash::hash(&unsafe {
                                                 ::std::intrinsics::discriminant_value(self)
                                             }, state);
                    ::std::hash::Hash::hash(&(*__self_0), state)
                }
            }
        }
    }
    #[cfg(feature = "std")]
    impl <AccountId, AccountIndex> fmt::Display for
     Address<AccountId, AccountIndex> where AccountId: Member,
     AccountIndex: Member {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                      &match (&self,) {
                                                           (arg0,) =>
                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                        ::std::fmt::Debug::fmt)],
                                                       }))
        }
    }
    impl <AccountId, AccountIndex> From<AccountId> for
     Address<AccountId, AccountIndex> where AccountId: Member,
     AccountIndex: Member {
        fn from(a: AccountId) -> Self { Address::Id(a) }
    }
    fn need_more_than<T: PartialOrd>(a: T, b: T) -> Option<T> {
        if a < b { Some(b) } else { None }
    }
    impl <AccountId, AccountIndex> Decode for Address<AccountId, AccountIndex>
     where AccountId: Member + Decode, AccountIndex: Member + Decode +
     PartialOrd<AccountIndex> + Ord + As<u32> + As<u16> + As<u8> + Copy {
        fn decode<I: Input>(input: &mut I) -> Option<Self> {
            Some(match input.read_byte()? {
                     x@0x00 ...0xef => Address::Index(As::sa(x)),
                     0xfc =>
                     Address::Index(As::sa(need_more_than(0xef,
                                                          u16::decode(input)?)?)),
                     0xfd =>
                     Address::Index(As::sa(need_more_than(0xffff,
                                                          u32::decode(input)?)?)),
                     0xfe =>
                     Address::Index(need_more_than(As::sa(0xffffffffu32),
                                                   Decode::decode(input)?)?),
                     0xff => Address::Id(Decode::decode(input)?),
                     _ => return None,
                 })
        }
    }
    impl <AccountId, AccountIndex> Encode for Address<AccountId, AccountIndex>
     where AccountId: Member + Encode, AccountIndex: Member + Encode +
     PartialOrd<AccountIndex> + Ord + As<u32> + As<u16> + As<u8> + Copy {
        fn encode_to<T: Output>(&self, dest: &mut T) {
            match *self {
                Address::Id(ref i) => { dest.push_byte(255); dest.push(i); }
                Address::Index(i) if i > As::sa(0xffffffffu32) => {
                    dest.push_byte(254);
                    dest.push(&i);
                }
                Address::Index(i) if i > As::sa(0xffffu32) => {
                    dest.push_byte(253);
                    dest.push(&As::<u32>::as_(i));
                }
                Address::Index(i) if i >= As::sa(0xf0u32) => {
                    dest.push_byte(252);
                    dest.push(&As::<u16>::as_(i));
                }
                Address::Index(i) => dest.push_byte(As::<u8>::as_(i)),
            }
        }
    }
    impl <AccountId, AccountIndex> Default for
     Address<AccountId, AccountIndex> where AccountId: Member + Default,
     AccountIndex: Member {
        fn default() -> Self { Address::Id(Default::default()) }
    }
}
/// Number of account IDs stored per enum set.
const ENUM_SET_SIZE: usize = 64;
pub type Address<T>
    =
    RawAddress<<T as system::Trait>::AccountId, <T as Trait>::AccountIndex>;
/// Turn an Id into an Index, or None for the purpose of getting
/// a hint at a possibly desired index.
pub trait ResolveHint<AccountId: Encode, AccountIndex: As<usize>> {
    /// Turn an Id into an Index, or None for the purpose of getting
    /// a hint at a possibly desired index.
    fn resolve_hint(who: &AccountId)
    -> Option<AccountIndex>;
}
/// Simple encode-based resolve hint implemenntation.
pub struct SimpleResolveHint<AccountId,
                             AccountIndex>(PhantomData<(AccountId,
                                                        AccountIndex)>);
impl <AccountId: Encode, AccountIndex: As<usize>>
 ResolveHint<AccountId, AccountIndex> for
 SimpleResolveHint<AccountId, AccountIndex> {
    fn resolve_hint(who: &AccountId) -> Option<AccountIndex> {
        Some(AccountIndex::sa(who.using_encoded(|e|
                                                    e[0] as usize +
                                                        e[1] as usize * 256)))
    }
}
/// The module's config trait.
pub trait Trait: system::Trait {
    /// Type used for storing an account's index; implies the maximum number of accounts the system
    /// can hold.
    type
    AccountIndex: Parameter +
    Member +
    Codec +
    Default +
    SimpleArithmetic +
    As<u8> +
    As<u16> +
    As<u32> +
    As<u64> +
    As<usize> +
    Copy;
    /// Whether an account is dead or not.
    type
    IsDeadAccount: IsDeadAccount<Self::AccountId>;
    /// How to turn an id into an index.
    type
    ResolveHint: ResolveHint<Self::AccountId, Self::AccountIndex>;
    /// The overarching event type.
    type
    Event: From<Event<Self>> +
    Into<<Self as system::Trait>::Event>;
}
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
    fn deposit_event(event: Event<T>) {
        <system::Module<T>>::deposit_event(<T as Trait>::from(event).into());
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
                                           &("srml/indices/src/lib.rs", 72u32,
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
                                           &("srml/indices/src/lib.rs", 72u32,
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
                                           &("srml/indices/src/lib.rs", 72u32,
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
                                                       &("srml/indices/src/lib.rs",
                                                         72u32, 1u32))
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
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T>
    =
    RawEvent<<T as system::Trait>::AccountId, <T as Trait>::AccountIndex>;
/// Events for this module.
///
#[structural_match]
pub enum RawEvent<AccountId, AccountIndex> {

    #[doc = r" A new account index was assigned."]
    #[doc = r""]
    #[doc =
          r" This event is not triggered when an existing index is reassigned"]
    #[doc = r" to another `AccountId`."]
    NewAccountIndex(AccountId, AccountIndex),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::clone::Clone, AccountIndex: ::std::clone::Clone>
 ::std::clone::Clone for RawEvent<AccountId, AccountIndex> {
    #[inline]
    fn clone(&self) -> RawEvent<AccountId, AccountIndex> {
        match (&*self,) {
            (&RawEvent::NewAccountIndex(ref __self_0, ref __self_1),) =>
            RawEvent::NewAccountIndex(::std::clone::Clone::clone(&(*__self_0)),
                                      ::std::clone::Clone::clone(&(*__self_1))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialEq, AccountIndex: ::std::cmp::PartialEq>
 ::std::cmp::PartialEq for RawEvent<AccountId, AccountIndex> {
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId, AccountIndex>) -> bool {
        match (&*self, &*other) {
            (&RawEvent::NewAccountIndex(ref __self_0, ref __self_1),
             &RawEvent::NewAccountIndex(ref __arg_1_0, ref __arg_1_1)) =>
            (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId, AccountIndex>) -> bool {
        match (&*self, &*other) {
            (&RawEvent::NewAccountIndex(ref __self_0, ref __self_1),
             &RawEvent::NewAccountIndex(ref __arg_1_0, ref __arg_1_1)) =>
            (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::Eq, AccountIndex: ::std::cmp::Eq> ::std::cmp::Eq
 for RawEvent<AccountId, AccountIndex> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<AccountIndex>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawEvent: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, AccountIndex> _parity_codec::Encode for
         RawEvent<AccountId, AccountIndex> where
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountIndex: _parity_codec::Encode,
         AccountIndex: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::NewAccountIndex(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_RawEvent: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, AccountIndex> _parity_codec::Decode for
         RawEvent<AccountId, AccountIndex> where
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountIndex: _parity_codec::Decode,
         AccountIndex: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::NewAccountIndex(_parity_codec::Decode::decode(input)?,
                                                       _parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::fmt::Debug, AccountIndex: ::std::fmt::Debug>
 ::std::fmt::Debug for RawEvent<AccountId, AccountIndex> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawEvent::NewAccountIndex(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("NewAccountIndex");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <AccountId, AccountIndex> From<RawEvent<AccountId, AccountIndex>> for ()
 {
    fn from(_: RawEvent<AccountId, AccountIndex>) -> () { () }
}
impl <AccountId, AccountIndex> RawEvent<AccountId, AccountIndex> {
    #[allow(dead_code)]
    pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
        &[::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("NewAccountIndex"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "AccountIndex"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" A new account index was assigned.",
                                                                                                    r"",
                                                                                                    r" This event is not triggered when an existing index is reassigned",
                                                                                                    r" to another `AccountId`."]),}]
    }
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = " The next free enumeration set."]
pub struct NextEnumSet<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::AccountIndex>
 for NextEnumSet<T> {
    type
    Query
    =
    T::AccountIndex;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Indices NextEnumSet".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::AccountIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                         Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::AccountIndex>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::AccountIndex>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::AccountIndex>>::put(&val,
                                                                                                                                       storage);
        ret
    }
}
#[doc = " The enumeration sets."]
pub struct EnumSet<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountIndex,
                                                                                                   Vec<T::AccountId>>
 for EnumSet<T> {
    type
    Query
    =
    Vec<T::AccountId>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Indices EnumSet".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountIndex)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountIndex,
                                                                                                                  Vec<T::AccountId>>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &T::AccountIndex,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountIndex,
                                                                                                                  Vec<T::AccountId>>>::key_for(key);
        storage.get(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &T::AccountIndex,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountIndex,
                                                                                                                  Vec<T::AccountId>>>::key_for(key);
        storage.take(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::AccountIndex,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountIndex,
                                                                                                                  Vec<T::AccountId>>>::get(key,
                                                                                                                                           storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountIndex,
                                                                                                              Vec<T::AccountId>>>::insert(key,
                                                                                                                                          &val,
                                                                                                                                          storage);
        ret
    }
}
trait Store {
    type
    NextEnumSet;
    type
    EnumSet;
}
#[doc(hidden)]
pub struct __GetByteStructNextEnumSet<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_NextEnumSet:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNextEnumSet<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_NextEnumSet.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        T::AccountIndex =
                                                                    Default::default();
                                                                <T::AccountIndex
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructEnumSet<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_EnumSet:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructEnumSet<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_EnumSet.get_or_init(||
                                                        {
                                                            let def_val:
                                                                    Vec<T::AccountId> =
                                                                Default::default();
                                                            <Vec<T::AccountId>
                                                                as
                                                                Encode>::encode(&def_val)
                                                        }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    NextEnumSet
    =
    NextEnumSet<T>;
    type
    EnumSet
    =
    EnumSet<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc = " The next free enumeration set."]
    pub fn next_enum_set() -> T::AccountIndex {
        <NextEnumSet<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::AccountIndex>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The enumeration sets."]
    pub fn enum_set<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountIndex>>(key:
                                                                                                                             K)
     -> Vec<T::AccountId> {
        <EnumSet<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountIndex,
                                                                                                              Vec<T::AccountId>>>::get(key.borrow(),
                                                                                                                                       &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextEnumSet"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountIndex")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextEnumSet::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next free enumeration set."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EnumSet"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountIndex"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEnumSet::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The enumeration sets."]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextEnumSet"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountIndex")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextEnumSet::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next free enumeration set."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("EnumSet"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountIndex"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructEnumSet::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The enumeration sets."]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Indices" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "Vec < T :: AccountId > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "Vec < T :: AccountId > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    pub ids: Vec<T::AccountId>,
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
         Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
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
                                                                    "ids",
                                                                    &self.ids)
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
         Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
         {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, }
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
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 1")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "ids" => _serde::export::Ok(__Field::__field0),
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
                            b"ids" => _serde::export::Ok(__Field::__field0),
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
                       Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Vec<T::AccountId>>(&mut __seq)
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
                        _serde::export::Ok(GenesisConfig{ids: __field0,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Vec<T::AccountId>> =
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
                                                                       _serde::de::Error>::duplicate_field("ids"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<T::AccountId>>(&mut __map)
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
                                match _serde::private::de::missing_field("ids")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{ids: __field0,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["ids"];
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
    fn default() -> Self { GenesisConfig{ids: Default::default(),} }
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
            let v =
                (|config: &GenesisConfig<T>|
                     {
                         T::AccountIndex::sa(config.ids.len() / ENUM_SET_SIZE)
                     })(&self);
            <NextEnumSet<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::AccountIndex>>::put(&v,
                                                                                                                                           &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let data =
                (|config: &GenesisConfig<T>|
                     {
                         (0..(config.ids.len() + ENUM_SET_SIZE - 1) /
                                 ENUM_SET_SIZE).map(|i|
                                                        (T::AccountIndex::sa(i),
                                                         config.ids[i *
                                                                        ENUM_SET_SIZE..config.ids.len().min((i
                                                                                                                 +
                                                                                                                 1)
                                                                                                                *
                                                                                                                ENUM_SET_SIZE)].to_owned())).collect::<Vec<_>>()
                     })(&self);
            for (k, v) in data.into_iter() {
                <EnumSet<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountIndex,
                                                                                                                      Vec<T::AccountId>>>::insert(&k,
                                                                                                                                                  &v,
                                                                                                                                                  &storage);
            }
        }
        let r = storage.into_inner();
        (|_, _, _| { })(r, c, &self);
        Ok(())
    }
}
impl <T: Trait> Module<T> {
    /// Lookup an T::AccountIndex to get an Id, if there's one there.
    pub fn lookup_index(index: T::AccountIndex) -> Option<T::AccountId> {
        let enum_set_size = Self::enum_set_size();
        let set = Self::enum_set(index / enum_set_size);
        let i: usize = (index % enum_set_size).as_();
        set.get(i).cloned()
    }
    /// `true` if the account `index` is ready for reclaim.
    pub fn can_reclaim(try_index: T::AccountIndex) -> bool {
        let enum_set_size = Self::enum_set_size();
        let try_set = Self::enum_set(try_index / enum_set_size);
        let i = (try_index % enum_set_size).as_();
        i < try_set.len() && T::IsDeadAccount::is_dead_account(&try_set[i])
    }
    /// Lookup an address to get an Id, if there's one there.
    pub fn lookup_address(a: address::Address<T::AccountId, T::AccountIndex>)
     -> Option<T::AccountId> {
        match a {
            address::Address::Id(i) => Some(i),
            address::Address::Index(i) => Self::lookup_index(i),
        }
    }
    fn enum_set_size() -> T::AccountIndex {
        T::AccountIndex::sa(ENUM_SET_SIZE)
    }
}
impl <T: Trait> OnNewAccount<T::AccountId> for Module<T> {
    fn on_new_account(who: &T::AccountId) {
        let enum_set_size = Self::enum_set_size();
        let next_set_index = Self::next_enum_set();
        if let Some(try_index) = T::ResolveHint::resolve_hint(who) {
            let set_index = try_index / enum_set_size;
            let mut try_set = Self::enum_set(set_index);
            let item_index = (try_index % enum_set_size).as_();
            if item_index < try_set.len() {
                if T::IsDeadAccount::is_dead_account(&try_set[item_index]) {
                    try_set[item_index] = who.clone();
                    <EnumSet<T>>::insert(set_index, try_set);
                    return
                }
            }
        }
        let mut set_index = next_set_index;
        let mut set =
            loop  {
                let set = Self::enum_set(set_index);
                if set.len() < ENUM_SET_SIZE { break set ; }
                set_index += One::one();
            };
        let index =
            T::AccountIndex::sa(set_index.as_() * ENUM_SET_SIZE + set.len());
        set.push(who.clone());
        if set.len() == ENUM_SET_SIZE {
            <NextEnumSet<T>>::put(set_index + One::one());
        }
        <EnumSet<T>>::insert(set_index, set);
        Self::deposit_event(RawEvent::NewAccountIndex(who.clone(), index));
    }
}
impl <T: Trait> StaticLookup for Module<T> {
    type
    Source
    =
    address::Address<T::AccountId, T::AccountIndex>;
    type
    Target
    =
    T::AccountId;
    fn lookup(a: Self::Source) -> result::Result<Self::Target, &'static str> {
        Self::lookup_address(a).ok_or("invalid account index")
    }
    fn unlookup(a: Self::Target) -> Self::Source { address::Address::Id(a) }
}
