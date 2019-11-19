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

//! Democratic system: Handles administration of general stakeholder voting.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


use rstd::prelude::*;
use rstd::result;
use primitives::traits::{Zero, As, Bounded};
use parity_codec::{Encode, Decode};
use srml_support::{StorageValue, StorageMap, Parameter, Dispatchable,
                   IsSubType, EnumerableStorageMap};
use srml_support::{decl_module, decl_storage, decl_event, ensure};
use srml_support::traits::{Currency, ReservableCurrency, LockableCurrency,
                           WithdrawReason, LockIdentifier, OnFreeBalanceZero};
use srml_support::dispatch::Result;
use system::ensure_signed;

mod vote_threshold {




























    // Currency is locked indefinitely as long as it's delegated.

    // Indefinite lock is reduced to the maximum voting lock that could be possible.












    // exposed immutables.








    // Exposed mutables.




    // private.







    /*defensive only: All current public proposals have an amount locked*/

    //: (BalanceOf<T>, Vec<T::AccountId>) =
    // refund depositors




    // Logic defined in https://www.slideshare.net/gavofyork/governance-in-polkadot-poc3
    // Essentially, we extend the lock-period of the coins behind the winning votes to be the
    // vote strength times the public delay period from now.
    // ^^^ defensive only: all items come from `voters`; for an item to be in `voters` there must be a vote registered; qed
    // Just the winning coins
    // now plus: the base lock period multiplied by the number of periods this voter offered to
    // lock should they win...
    // ...extend their bondage until at least then.



    // pick out another public referendum if it's time.

    // tally up votes for any expiring referenda.







    // Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.















    // Can't set when already set.

    // But this works because 11 isn't proxying.

    // 2 cannot fire 1's proxy:

    // 1 fires his proxy:

    // 11 resigns:








    // Delegate vote.



    // Delegated vote is counted.






    // Check behavior with cycle.



    // Delegated vote is counted.






    // Vote.

    // Delegate vote.


    // Delegated vote is not counted.





    // Delegate and undelegate vote.



    // Delegated vote is not counted.







    // Delegate vote.

    // Vote.


    // Delegated vote is not counted.



















































    //! Voting thresholds.
    #[cfg(feature = "std")]
    use serde::{Serialize, Deserialize};
    use parity_codec::{Encode, Decode};
    use primitives::traits::{Zero, IntegerSquareRoot};
    use rstd::ops::{Add, Mul, Div, Rem};
    /// A means of determining if a vote is past pass threshold.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum VoteThreshold {

        /// A supermajority of approvals is needed to pass this vote.
        SuperMajorityApprove,

        /// A supermajority of rejects is needed to fail this vote.
        SuperMajorityAgainst,

        /// A simple majority of approvals is needed to pass this vote.
        SimpleMajority,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for VoteThreshold {
        #[inline]
        fn clone(&self) -> VoteThreshold { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for VoteThreshold { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for VoteThreshold {
        #[inline]
        fn eq(&self, other: &VoteThreshold) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) { _ => true, }
                } else { false }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for VoteThreshold {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_VoteThreshold: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for VoteThreshold {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        VoteThreshold::SuperMajorityApprove => {
                            dest.push_byte(0usize as u8);
                        }
                        VoteThreshold::SuperMajorityAgainst => {
                            dest.push_byte(1usize as u8);
                        }
                        VoteThreshold::SimpleMajority => {
                            dest.push_byte(2usize as u8);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_VoteThreshold: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for VoteThreshold {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => {
                            Some(VoteThreshold::SuperMajorityApprove)
                        }
                        x if x == 1usize as u8 => {
                            Some(VoteThreshold::SuperMajorityAgainst)
                        }
                        x if x == 2usize as u8 => {
                            Some(VoteThreshold::SimpleMajority)
                        }
                        _ => None,
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_VoteThreshold: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl _serde::Serialize for VoteThreshold {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    match *self {
                        VoteThreshold::SuperMajorityApprove =>
                        _serde::Serializer::serialize_unit_variant(__serializer,
                                                                   "VoteThreshold",
                                                                   0u32,
                                                                   "SuperMajorityApprove"),
                        VoteThreshold::SuperMajorityAgainst =>
                        _serde::Serializer::serialize_unit_variant(__serializer,
                                                                   "VoteThreshold",
                                                                   1u32,
                                                                   "SuperMajorityAgainst"),
                        VoteThreshold::SimpleMajority =>
                        _serde::Serializer::serialize_unit_variant(__serializer,
                                                                   "VoteThreshold",
                                                                   2u32,
                                                                   "SimpleMajority"),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_VoteThreshold: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <'de> _serde::Deserialize<'de> for VoteThreshold {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type
                        Value
                        =
                        __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "variant identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                _ =>
                                _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                     &"variant index 0 <= i < 3")),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                "SuperMajorityApprove" =>
                                _serde::export::Ok(__Field::__field0),
                                "SuperMajorityAgainst" =>
                                _serde::export::Ok(__Field::__field1),
                                "SimpleMajority" =>
                                _serde::export::Ok(__Field::__field2),
                                _ => {
                                    _serde::export::Err(_serde::de::Error::unknown_variant(__value,
                                                                                           VARIANTS))
                                }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"SuperMajorityApprove" =>
                                _serde::export::Ok(__Field::__field0),
                                b"SuperMajorityAgainst" =>
                                _serde::export::Ok(__Field::__field1),
                                b"SimpleMajority" =>
                                _serde::export::Ok(__Field::__field2),
                                _ => {
                                    let __value =
                                        &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_variant(__value,
                                                                                           VARIANTS))
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
                        marker: _serde::export::PhantomData<VoteThreshold>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        VoteThreshold;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "enum VoteThreshold")
                        }
                        fn visit_enum<__A>(self, __data: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::EnumAccess<'de> {
                            match match _serde::de::EnumAccess::variant(__data)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                (__Field::__field0, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant)
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(VoteThreshold::SuperMajorityApprove)
                                }
                                (__Field::__field1, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant)
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(VoteThreshold::SuperMajorityAgainst)
                                }
                                (__Field::__field2, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant)
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(VoteThreshold::SimpleMajority)
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["SuperMajorityApprove", "SuperMajorityAgainst",
                          "SimpleMajority"];
                    _serde::Deserializer::deserialize_enum(__deserializer,
                                                           "VoteThreshold",
                                                           VARIANTS,
                                                           __Visitor{marker:
                                                                         _serde::export::PhantomData::<VoteThreshold>,
                                                                     lifetime:
                                                                         _serde::export::PhantomData,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for VoteThreshold {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&VoteThreshold::SuperMajorityApprove,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SuperMajorityApprove");
                    debug_trait_builder.finish()
                }
                (&VoteThreshold::SuperMajorityAgainst,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SuperMajorityAgainst");
                    debug_trait_builder.finish()
                }
                (&VoteThreshold::SimpleMajority,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SimpleMajority");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub trait Approved<Balance> {
        /// Given `approve` votes for and `against` votes against from a total electorate size of
        /// `electorate` (`electorate - (approve + against)` are abstainers), then returns true if the
        /// overall outcome is in favor of approval.
        fn approved(&self, approve: Balance, against: Balance,
                    voters: Balance, electorate: Balance)
        -> bool;
    }
    /// Return `true` iff `n1 / d1 < n2 / d2`. `d1` and `d2` may not be zero.
    fn compare_rationals<T: Zero + Mul<T, Output = T> + Div<T, Output = T> +
                         Rem<T, Output = T> + Ord +
                         Copy>(mut n1: T, mut d1: T, mut n2: T, mut d2: T)
     -> bool {
        loop  {
            let q1 = n1 / d1;
            let q2 = n2 / d2;
            if q1 < q2 { return true; }
            if q2 < q1 { return false; }
            let r1 = n1 % d1;
            let r2 = n2 % d2;
            if r2.is_zero() { return false; }
            if r1.is_zero() { return true; }
            n1 = d2;
            n2 = d1;
            d1 = r2;
            d2 = r1;
        }
    }
    impl <Balance: IntegerSquareRoot + Zero + Ord + Add<Balance, Output =
          Balance> + Mul<Balance, Output = Balance> + Div<Balance, Output =
          Balance> + Rem<Balance, Output = Balance> + Copy> Approved<Balance>
     for VoteThreshold {
        /// Given `approve` votes for and `against` votes against from a total electorate size of
        /// `electorate` of whom `voters` voted (`electorate - voters` are abstainers) then returns true if the
        /// overall outcome is in favor of approval.
        ///
        /// We assume each *voter* may cast more than one *vote*, hence `voters` is not necessarily equal to
        /// `approve + against`.
        fn approved(&self, approve: Balance, against: Balance,
                    voters: Balance, electorate: Balance) -> bool {
            let sqrt_voters = voters.integer_sqrt();
            let sqrt_electorate = electorate.integer_sqrt();
            if sqrt_voters.is_zero() { return false; }
            match *self {
                VoteThreshold::SuperMajorityApprove =>
                compare_rationals(against, sqrt_voters, approve,
                                  sqrt_electorate),
                VoteThreshold::SuperMajorityAgainst =>
                compare_rationals(against, sqrt_electorate, approve,
                                  sqrt_voters),
                VoteThreshold::SimpleMajority => approve > against,
            }
        }
    }
}
pub use vote_threshold::{Approved, VoteThreshold};
const DEMOCRACY_ID: LockIdentifier = *b"democrac";
/// A proposal index.
pub type PropIndex = u32;
/// A referendum index.
pub type ReferendumIndex = u32;
/// A number of lock periods.
pub type LockPeriods = i8;
const MAX_RECURSION_LIMIT: u32 = 16;
/// A number of lock periods, plus a vote, one way or the other.
#[structural_match]
#[rustc_copy_clone_marker]
pub struct Vote(i8);
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Vote: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Vote {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Vote: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Vote {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(Vote(_parity_codec::Decode::decode(input)?))
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for Vote { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Vote {
    #[inline]
    fn clone(&self) -> Vote {
        { let _: ::std::clone::AssertParamIsClone<i8>; *self }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Vote {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<i8>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Vote {
    #[inline]
    fn eq(&self, other: &Vote) -> bool {
        match *other {
            Vote(ref __self_1_0) =>
            match *self {
                Vote(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Vote) -> bool {
        match *other {
            Vote(ref __self_1_0) =>
            match *self {
                Vote(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::default::Default for Vote {
    #[inline]
    fn default() -> Vote { Vote(::std::default::Default::default()) }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Vote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Vote(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Vote");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl Vote {
    /// Create a new instance.
    pub fn new(aye: bool, multiplier: LockPeriods) -> Self {
        let m = multiplier.max(1) - 1;
        Vote(if aye { -1 - m } else { m })
    }
    /// Is this an aye vote?
    pub fn is_aye(self) -> bool { self.0 < 0 }
    /// The strength (measured in lock periods).
    pub fn multiplier(self) -> LockPeriods {
        1 + if self.0 < 0 { -(self.0 + 1) } else { self.0 }
    }
}
type BalanceOf<T>
    =
    <<T as Trait>::Currency as
    Currency<<T as system::Trait>::AccountId>>::Balance;
pub trait Trait: system::Trait + Sized {
    type
    Currency: ReservableCurrency<Self::AccountId> +
    LockableCurrency<Self::AccountId,
    Moment
    =
    Self::BlockNumber>;
    type
    Proposal: Parameter +
    Dispatchable<Origin
    =
    Self::Origin> +
    IsSubType<Module<Self>>;
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
    fn on_finalize(n: T::BlockNumber) {
        if let Err(e) = Self::end_block(n) { runtime_io::print(e); }
    }
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
impl <T: Trait> Module<T> {
    #[doc = r" Propose a sensitive action to be taken."]
    fn propose(origin: T::Origin, proposal: Box<T::Proposal>,
               value: BalanceOf<T>) -> ::srml_support::dispatch::Result {
        {
            let who = ensure_signed(origin)?;
            {
                if !(value >= Self::minimum_deposit()) {
                    { return Err("value too low"); };
                }
            };
            T::Currency::reserve(&who,
                                 value).map_err(|_|
                                                    "proposer's balance too low")?;
            let index = Self::public_prop_count();
            <PublicPropCount<T>>::put(index + 1);
            <DepositOf<T>>::insert(index,
                                   (value,
                                    <[_]>::into_vec(box [who.clone()])));
            let mut props = Self::public_props();
            props.push((index, (*proposal).clone(), who));
            <PublicProps<T>>::put(props);
            Self::deposit_event(RawEvent::Proposed(index, value));
        }
        Ok(())
    }
    #[doc = r" Propose a sensitive action to be taken."]
    fn second(origin: T::Origin, proposal: PropIndex)
     -> ::srml_support::dispatch::Result {
        {
            let who = ensure_signed(origin)?;
            let mut deposit =
                Self::deposit_of(proposal).ok_or("can only second an existing proposal")?;
            T::Currency::reserve(&who,
                                 deposit.0).map_err(|_|
                                                        "seconder's balance too low")?;
            deposit.1.push(who);
            <DepositOf<T>>::insert(proposal, deposit);
        }
        Ok(())
    }
    #[doc =
          r" Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;"]
    #[doc = r" otherwise it is a vote to keep the status quo."]
    fn vote(origin: T::Origin, ref_index: ReferendumIndex, vote: Vote)
     -> Result {
        let who = ensure_signed(origin)?;
        Self::do_vote(who, ref_index, vote)
    }
    #[doc =
          r" Vote in a referendum on behalf of a stash. If `vote.is_aye()`, the vote is to enact the proposal;"]
    #[doc = r" otherwise it is a vote to keep the status quo."]
    fn proxy_vote(origin: T::Origin, ref_index: ReferendumIndex, vote: Vote)
     -> Result {
        let who = Self::proxy(ensure_signed(origin)?).ok_or("not a proxy")?;
        Self::do_vote(who, ref_index, vote)
    }
    #[doc = r" Start a referendum."]
    fn start_referendum(proposal: Box<T::Proposal>, threshold: VoteThreshold,
                        delay: T::BlockNumber) -> Result {
        Self::inject_referendum(<system::Module<T>>::block_number() +
                                    Self::voting_period(), *proposal,
                                threshold, delay).map(|_| ())
    }
    #[doc = r" Remove a referendum."]
    fn cancel_referendum(ref_index: ReferendumIndex)
     -> ::srml_support::dispatch::Result {
        { Self::clear_referendum(ref_index); }
        Ok(())
    }
    #[doc = r" Cancel a proposal queued for enactment."]
    pub fn cancel_queued(when: T::BlockNumber, which: u32)
     -> ::srml_support::dispatch::Result {
        {
            let which = which as usize;
            <DispatchQueue<T>>::mutate(when,
                                       |items|
                                           if items.len() > which {
                                               items[which] = None
                                           });
        }
        Ok(())
    }
    #[doc = r" Specify a proxy. Called by the stash."]
    fn set_proxy(origin: T::Origin, proxy: T::AccountId)
     -> ::srml_support::dispatch::Result {
        {
            let who = ensure_signed(origin)?;
            {
                if !!<Proxy<T>>::exists(&proxy) {
                    { return Err("already a proxy"); };
                }
            };
            <Proxy<T>>::insert(proxy, who)
        }
        Ok(())
    }
    #[doc = r" Clear the proxy. Called by the proxy."]
    fn resign_proxy(origin: T::Origin) -> ::srml_support::dispatch::Result {
        { let who = ensure_signed(origin)?; <Proxy<T>>::remove(who); }
        Ok(())
    }
    #[doc = r" Clear the proxy. Called by the stash."]
    fn remove_proxy(origin: T::Origin, proxy: T::AccountId)
     -> ::srml_support::dispatch::Result {
        {
            let who = ensure_signed(origin)?;
            {
                if !(&Self::proxy(&proxy).ok_or("not a proxy")? == &who) {
                    { return Err("wrong proxy"); };
                }
            };
            <Proxy<T>>::remove(proxy);
        }
        Ok(())
    }
    #[doc = r" Delegate vote."]
    pub fn delegate(origin: T::Origin, to: T::AccountId,
                    lock_periods: LockPeriods)
     -> ::srml_support::dispatch::Result {
        {
            let who = ensure_signed(origin)?;
            <Delegations<T>>::insert(who.clone(),
                                     (to.clone(), lock_periods.clone()));
            T::Currency::extend_lock(DEMOCRACY_ID, &who, Bounded::max_value(),
                                     T::BlockNumber::max_value(),
                                     WithdrawReason::Transfer.into());
            Self::deposit_event(RawEvent::Delegated(who, to));
        }
        Ok(())
    }
    #[doc = r" Undelegate vote."]
    fn undelegate(origin: T::Origin) -> ::srml_support::dispatch::Result {
        {
            let who = ensure_signed(origin)?;
            {
                if !<Delegations<T>>::exists(&who) {
                    { return Err("not delegated"); };
                }
            };
            let d = <Delegations<T>>::take(&who);
            let lock_period = Self::public_delay();
            let now = <system::Module<T>>::block_number();
            let locked_until =
                now + lock_period * T::BlockNumber::sa(d.1 as u64);
            T::Currency::set_lock(DEMOCRACY_ID, &who, Bounded::max_value(),
                                  locked_until,
                                  WithdrawReason::Transfer.into());
            Self::deposit_event(RawEvent::Undelegated(who));
        }
        Ok(())
    }
}
pub enum Call<T: Trait> {

    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                  ::srml_support::dispatch::Never),

    #[allow(non_camel_case_types)]
    propose(Box<T::Proposal>,
            #[codec(compact)]
            BalanceOf<T>),

    #[allow(non_camel_case_types)]
    second(
           #[codec(compact)]
           PropIndex),

    #[allow(non_camel_case_types)]
    vote(
         #[codec(compact)]
         ReferendumIndex, Vote),

    #[allow(non_camel_case_types)]
    proxy_vote(
               #[codec(compact)]
               ReferendumIndex, Vote),

    #[allow(non_camel_case_types)]
    start_referendum(Box<T::Proposal>, VoteThreshold, T::BlockNumber),

    #[allow(non_camel_case_types)]
    cancel_referendum(
                      #[codec(compact)]
                      ReferendumIndex),

    #[allow(non_camel_case_types)]
    cancel_queued(
                  #[codec(compact)]
                  T::BlockNumber,
                  #[codec(compact)]
                  u32),

    #[allow(non_camel_case_types)]
    set_proxy(T::AccountId),

    #[allow(non_camel_case_types)]
    #[doc = r" Clear the proxy. Called by the proxy."]
    resign_proxy(),

    #[allow(non_camel_case_types)]
    remove_proxy(T::AccountId),

    #[allow(non_camel_case_types)]
    delegate(T::AccountId, LockPeriods),

    #[allow(non_camel_case_types)]
    #[doc = r" Undelegate vote."]
    undelegate(),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for Call<T> where
         Box<T::Proposal>: _parity_codec::Encode,
         Box<T::Proposal>: _parity_codec::Encode,
         Box<T::Proposal>: _parity_codec::Encode,
         Box<T::Proposal>: _parity_codec::Encode,
         T::BlockNumber: _parity_codec::Encode,
         T::BlockNumber: _parity_codec::Encode,
         T::AccountId: _parity_codec::Encode,
         T::AccountId: _parity_codec::Encode,
         T::AccountId: _parity_codec::Encode,
         T::AccountId: _parity_codec::Encode,
         T::AccountId: _parity_codec::Encode,
         T::AccountId: _parity_codec::Encode,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::BlockNumber: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::propose(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        {
                            dest.push(&<<BalanceOf<T> as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      BalanceOf<T>>>::from(ba));
                        }
                    }
                    Call::second(ref aa) => {
                        dest.push_byte(1usize as u8);
                        {
                            dest.push(&<<PropIndex as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      PropIndex>>::from(aa));
                        }
                    }
                    Call::vote(ref aa, ref ba) => {
                        dest.push_byte(2usize as u8);
                        {
                            dest.push(&<<ReferendumIndex as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      ReferendumIndex>>::from(aa));
                        }
                        dest.push(ba);
                    }
                    Call::proxy_vote(ref aa, ref ba) => {
                        dest.push_byte(3usize as u8);
                        {
                            dest.push(&<<ReferendumIndex as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      ReferendumIndex>>::from(aa));
                        }
                        dest.push(ba);
                    }
                    Call::start_referendum(ref aa, ref ba, ref ca) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    Call::cancel_referendum(ref aa) => {
                        dest.push_byte(5usize as u8);
                        {
                            dest.push(&<<ReferendumIndex as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      ReferendumIndex>>::from(aa));
                        }
                    }
                    Call::cancel_queued(ref aa, ref ba) => {
                        dest.push_byte(6usize as u8);
                        {
                            dest.push(&<<T::BlockNumber as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::BlockNumber>>::from(aa));
                        }
                        {
                            dest.push(&<<u32 as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      u32>>::from(ba));
                        }
                    }
                    Call::set_proxy(ref aa) => {
                        dest.push_byte(7usize as u8);
                        dest.push(aa);
                    }
                    Call::resign_proxy() => { dest.push_byte(8usize as u8); }
                    Call::remove_proxy(ref aa) => {
                        dest.push_byte(9usize as u8);
                        dest.push(aa);
                    }
                    Call::delegate(ref aa, ref ba) => {
                        dest.push_byte(10usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    Call::undelegate() => { dest.push_byte(11usize as u8); }
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
        impl <T: Trait> _parity_codec::Decode for Call<T> where
         Box<T::Proposal>: _parity_codec::Decode,
         Box<T::Proposal>: _parity_codec::Decode,
         Box<T::Proposal>: _parity_codec::Decode,
         Box<T::Proposal>: _parity_codec::Decode,
         T::BlockNumber: _parity_codec::Decode,
         T::BlockNumber: _parity_codec::Decode,
         T::AccountId: _parity_codec::Decode,
         T::AccountId: _parity_codec::Decode,
         T::AccountId: _parity_codec::Decode,
         T::AccountId: _parity_codec::Decode,
         T::AccountId: _parity_codec::Decode,
         T::AccountId: _parity_codec::Decode,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::BlockNumber: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::propose(_parity_codec::Decode::decode(input)?,
                                           <<BalanceOf<T> as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::second(<<PropIndex as
                                           _parity_codec::HasCompact>::Type as
                                              _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 2usize as u8 => {
                        Some(Call::vote(<<ReferendumIndex as
                                         _parity_codec::HasCompact>::Type as
                                            _parity_codec::Decode>::decode(input)?.into(),
                                        _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 3usize as u8 => {
                        Some(Call::proxy_vote(<<ReferendumIndex as
                                               _parity_codec::HasCompact>::Type
                                                  as
                                                  _parity_codec::Decode>::decode(input)?.into(),
                                              _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 4usize as u8 => {
                        Some(Call::start_referendum(_parity_codec::Decode::decode(input)?,
                                                    _parity_codec::Decode::decode(input)?,
                                                    _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 5usize as u8 => {
                        Some(Call::cancel_referendum(<<ReferendumIndex as
                                                      _parity_codec::HasCompact>::Type
                                                         as
                                                         _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 6usize as u8 => {
                        Some(Call::cancel_queued(<<T::BlockNumber as
                                                  _parity_codec::HasCompact>::Type
                                                     as
                                                     _parity_codec::Decode>::decode(input)?.into(),
                                                 <<u32 as
                                                  _parity_codec::HasCompact>::Type
                                                     as
                                                     _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 7usize as u8 => {
                        Some(Call::set_proxy(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 8usize as u8 => { Some(Call::resign_proxy()) }
                    x if x == 9usize as u8 => {
                        Some(Call::remove_proxy(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 10usize as u8 => {
                        Some(Call::delegate(_parity_codec::Decode::decode(input)?,
                                            _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 11usize as u8 => { Some(Call::undelegate()) }
                    _ => None,
                }
            }
        }
    };
impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::propose(ref proposal, ref value) =>
            Call::propose((*proposal).clone(), (*value).clone()),
            Call::second(ref proposal) => Call::second((*proposal).clone()),
            Call::vote(ref ref_index, ref vote) =>
            Call::vote((*ref_index).clone(), (*vote).clone()),
            Call::proxy_vote(ref ref_index, ref vote) =>
            Call::proxy_vote((*ref_index).clone(), (*vote).clone()),
            Call::start_referendum(ref proposal, ref threshold, ref delay) =>
            Call::start_referendum((*proposal).clone(), (*threshold).clone(),
                                   (*delay).clone()),
            Call::cancel_referendum(ref ref_index) =>
            Call::cancel_referendum((*ref_index).clone()),
            Call::cancel_queued(ref when, ref which) =>
            Call::cancel_queued((*when).clone(), (*which).clone()),
            Call::set_proxy(ref proxy) => Call::set_proxy((*proxy).clone()),
            Call::resign_proxy() => Call::resign_proxy(),
            Call::remove_proxy(ref proxy) =>
            Call::remove_proxy((*proxy).clone()),
            Call::delegate(ref to, ref lock_periods) =>
            Call::delegate((*to).clone(), (*lock_periods).clone()),
            Call::undelegate() => Call::undelegate(),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/democracy/src/lib.rs",
                                             83u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::propose(ref proposal, ref value) => {
                let self_params = (proposal, value);
                if let Call::propose(ref proposal, ref value) = *_other {
                    self_params == (proposal, value)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::second(ref proposal) => {
                let self_params = (proposal,);
                if let Call::second(ref proposal) = *_other {
                    self_params == (proposal,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::vote(ref ref_index, ref vote) => {
                let self_params = (ref_index, vote);
                if let Call::vote(ref ref_index, ref vote) = *_other {
                    self_params == (ref_index, vote)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::proxy_vote(ref ref_index, ref vote) => {
                let self_params = (ref_index, vote);
                if let Call::proxy_vote(ref ref_index, ref vote) = *_other {
                    self_params == (ref_index, vote)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::start_referendum(ref proposal, ref threshold, ref delay) =>
            {
                let self_params = (proposal, threshold, delay);
                if let Call::start_referendum(ref proposal, ref threshold,
                                              ref delay) = *_other {
                    self_params == (proposal, threshold, delay)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::cancel_referendum(ref ref_index) => {
                let self_params = (ref_index,);
                if let Call::cancel_referendum(ref ref_index) = *_other {
                    self_params == (ref_index,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::cancel_queued(ref when, ref which) => {
                let self_params = (when, which);
                if let Call::cancel_queued(ref when, ref which) = *_other {
                    self_params == (when, which)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_proxy(ref proxy) => {
                let self_params = (proxy,);
                if let Call::set_proxy(ref proxy) = *_other {
                    self_params == (proxy,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::resign_proxy() => {
                let self_params = ();
                if let Call::resign_proxy() = *_other {
                    self_params == ()
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::remove_proxy(ref proxy) => {
                let self_params = (proxy,);
                if let Call::remove_proxy(ref proxy) = *_other {
                    self_params == (proxy,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::delegate(ref to, ref lock_periods) => {
                let self_params = (to, lock_periods);
                if let Call::delegate(ref to, ref lock_periods) = *_other {
                    self_params == (to, lock_periods)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::undelegate() => {
                let self_params = ();
                if let Call::undelegate() = *_other {
                    self_params == ()
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/democracy/src/lib.rs",
                                             83u32, 1u32))
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
            Call::propose(ref proposal, ref value) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"propose",
                                                               &(proposal.clone(),
                                                                 value.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::second(ref proposal) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"second",
                                                               &(proposal.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::vote(ref ref_index, ref vote) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"vote",
                                                               &(ref_index.clone(),
                                                                 vote.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::proxy_vote(ref ref_index, ref vote) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"proxy_vote",
                                                               &(ref_index.clone(),
                                                                 vote.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::start_referendum(ref proposal, ref threshold, ref delay) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"start_referendum",
                                                               &(proposal.clone(),
                                                                 threshold.clone(),
                                                                 delay.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::cancel_referendum(ref ref_index) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"cancel_referendum",
                                                               &(ref_index.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::cancel_queued(ref when, ref which) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"cancel_queued",
                                                               &(when.clone(),
                                                                 which.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_proxy(ref proxy) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_proxy",
                                                               &(proxy.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::resign_proxy() =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"resign_proxy",
                                                               &()) {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::remove_proxy(ref proxy) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"remove_proxy",
                                                               &(proxy.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::delegate(ref to, ref lock_periods) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"delegate",
                                                               &(to.clone(),
                                                                 lock_periods.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::undelegate() =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"undelegate",
                                                               &()) {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/democracy/src/lib.rs",
                                             83u32, 1u32))
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
            Call::propose(proposal, value) => {
                <Module<T>>::propose(_origin, proposal, value)
            }
            Call::second(proposal) => {
                <Module<T>>::second(_origin, proposal)
            }
            Call::vote(ref_index, vote) => {
                <Module<T>>::vote(_origin, ref_index, vote)
            }
            Call::proxy_vote(ref_index, vote) => {
                <Module<T>>::proxy_vote(_origin, ref_index, vote)
            }
            Call::start_referendum(proposal, threshold, delay) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::start_referendum(proposal, threshold, delay)
                }
            }
            Call::cancel_referendum(ref_index) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::cancel_referendum(ref_index)
                }
            }
            Call::cancel_queued(when, which) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::cancel_queued(when, which)
                }
            }
            Call::set_proxy(proxy) => {
                <Module<T>>::set_proxy(_origin, proxy)
            }
            Call::resign_proxy() => { <Module<T>>::resign_proxy(_origin) }
            Call::remove_proxy(proxy) => {
                <Module<T>>::remove_proxy(_origin, proxy)
            }
            Call::delegate(to, lock_periods) => {
                <Module<T>>::delegate(_origin, to, lock_periods)
            }
            Call::undelegate() => { <Module<T>>::undelegate(_origin) }
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
                                                       &("srml/democracy/src/lib.rs",
                                                         83u32, 1u32))
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
        &[::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("propose"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("proposal"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Box<T::Proposal>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("value"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Propose a sensitive action to be taken."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("second"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("proposal"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<PropIndex>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Propose a sensitive action to be taken."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("vote"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("ref_index"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<ReferendumIndex>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("vote"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vote"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Vote in a referendum. If `vote.is_aye()`, the vote is to enact the proposal;",
                                                                                                             r" otherwise it is a vote to keep the status quo."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("proxy_vote"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("ref_index"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<ReferendumIndex>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("vote"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vote"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Vote in a referendum on behalf of a stash. If `vote.is_aye()`, the vote is to enact the proposal;",
                                                                                                             r" otherwise it is a vote to keep the status quo."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("start_referendum"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("proposal"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Box<T::Proposal>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("threshold"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("VoteThreshold"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("delay"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("T::BlockNumber"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Start a referendum."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("cancel_referendum"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("ref_index"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<ReferendumIndex>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Remove a referendum."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("cancel_queued"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("when"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("which"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<u32>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Cancel a proposal queued for enactment."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_proxy"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("proxy"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("T::AccountId"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Specify a proxy. Called by the stash."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("resign_proxy"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Clear the proxy. Called by the proxy."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("remove_proxy"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("proxy"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("T::AccountId"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Clear the proxy. Called by the stash."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("delegate"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("to"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("T::AccountId"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("lock_periods"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("LockPeriods"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Delegate vote."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("undelegate"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Undelegate vote."]),}]
    }
}
/// Info regarding an ongoing referendum.
#[structural_match]
pub struct ReferendumInfo<BlockNumber: Parameter, Proposal: Parameter> {
    /// When voting on this referendum will end.
    end: BlockNumber,
    /// The proposal being voted on.
    proposal: Proposal,
    /// The thresholding mechanism to determine whether it passed.
    threshold: VoteThreshold,
    /// The delay (in blocks) to wait after a successful referendum before deploying.
    delay: BlockNumber,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_ReferendumInfo: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <BlockNumber: Parameter, Proposal: Parameter>
         _parity_codec::Encode for ReferendumInfo<BlockNumber, Proposal> where
         BlockNumber: _parity_codec::Encode,
         BlockNumber: _parity_codec::Encode, Proposal: _parity_codec::Encode,
         Proposal: _parity_codec::Encode, BlockNumber: _parity_codec::Encode,
         BlockNumber: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.end);
                dest.push(&self.proposal);
                dest.push(&self.threshold);
                dest.push(&self.delay);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_ReferendumInfo: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <BlockNumber: Parameter, Proposal: Parameter>
         _parity_codec::Decode for ReferendumInfo<BlockNumber, Proposal> where
         BlockNumber: _parity_codec::Decode,
         BlockNumber: _parity_codec::Decode, Proposal: _parity_codec::Decode,
         Proposal: _parity_codec::Decode, BlockNumber: _parity_codec::Decode,
         BlockNumber: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(ReferendumInfo{end:
                                        _parity_codec::Decode::decode(input)?,
                                    proposal:
                                        _parity_codec::Decode::decode(input)?,
                                    threshold:
                                        _parity_codec::Decode::decode(input)?,
                                    delay:
                                        _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <BlockNumber: ::std::clone::Clone + Parameter,
      Proposal: ::std::clone::Clone + Parameter> ::std::clone::Clone for
 ReferendumInfo<BlockNumber, Proposal> {
    #[inline]
    fn clone(&self) -> ReferendumInfo<BlockNumber, Proposal> {
        match *self {
            ReferendumInfo {
            end: ref __self_0_0,
            proposal: ref __self_0_1,
            threshold: ref __self_0_2,
            delay: ref __self_0_3 } =>
            ReferendumInfo{end: ::std::clone::Clone::clone(&(*__self_0_0)),
                           proposal:
                               ::std::clone::Clone::clone(&(*__self_0_1)),
                           threshold:
                               ::std::clone::Clone::clone(&(*__self_0_2)),
                           delay:
                               ::std::clone::Clone::clone(&(*__self_0_3)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <BlockNumber: ::std::cmp::PartialEq + Parameter,
      Proposal: ::std::cmp::PartialEq + Parameter> ::std::cmp::PartialEq for
 ReferendumInfo<BlockNumber, Proposal> {
    #[inline]
    fn eq(&self, other: &ReferendumInfo<BlockNumber, Proposal>) -> bool {
        match *other {
            ReferendumInfo {
            end: ref __self_1_0,
            proposal: ref __self_1_1,
            threshold: ref __self_1_2,
            delay: ref __self_1_3 } =>
            match *self {
                ReferendumInfo {
                end: ref __self_0_0,
                proposal: ref __self_0_1,
                threshold: ref __self_0_2,
                delay: ref __self_0_3 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &ReferendumInfo<BlockNumber, Proposal>) -> bool {
        match *other {
            ReferendumInfo {
            end: ref __self_1_0,
            proposal: ref __self_1_1,
            threshold: ref __self_1_2,
            delay: ref __self_1_3 } =>
            match *self {
                ReferendumInfo {
                end: ref __self_0_0,
                proposal: ref __self_0_1,
                threshold: ref __self_0_2,
                delay: ref __self_0_3 } =>
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
impl <BlockNumber: ::std::cmp::Eq + Parameter, Proposal: ::std::cmp::Eq +
      Parameter> ::std::cmp::Eq for ReferendumInfo<BlockNumber, Proposal> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<BlockNumber>;
            let _: ::std::cmp::AssertParamIsEq<Proposal>;
            let _: ::std::cmp::AssertParamIsEq<VoteThreshold>;
            let _: ::std::cmp::AssertParamIsEq<BlockNumber>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <BlockNumber: ::std::fmt::Debug + Parameter,
      Proposal: ::std::fmt::Debug + Parameter> ::std::fmt::Debug for
 ReferendumInfo<BlockNumber, Proposal> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ReferendumInfo {
            end: ref __self_0_0,
            proposal: ref __self_0_1,
            threshold: ref __self_0_2,
            delay: ref __self_0_3 } => {
                let mut debug_trait_builder =
                    f.debug_struct("ReferendumInfo");
                let _ = debug_trait_builder.field("end", &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("proposal", &&(*__self_0_1));
                let _ =
                    debug_trait_builder.field("threshold", &&(*__self_0_2));
                let _ = debug_trait_builder.field("delay", &&(*__self_0_3));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <BlockNumber: Parameter, Proposal: Parameter>
 ReferendumInfo<BlockNumber, Proposal> {
    /// Create a new instance.
    pub fn new(end: BlockNumber, proposal: Proposal, threshold: VoteThreshold,
               delay: BlockNumber) -> Self {
        ReferendumInfo{end, proposal, threshold, delay,}
    }
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = " The number of (public) proposals that have been made so far."]
pub struct PublicPropCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<PropIndex>
 for PublicPropCount<T> {
    type
    Query
    =
    PropIndex;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Democracy PublicPropCount".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<PropIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                   Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<PropIndex>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<PropIndex>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<PropIndex>>::put(&val,
                                                                                                                                 storage);
        ret
    }
}
#[doc = " The public proposals. Unsorted."]
pub struct PublicProps<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(PropIndex,
                                                                                                          T::Proposal,
                                                                                                          T::AccountId)>>
 for PublicProps<T> {
    type
    Query
    =
    Vec<(PropIndex, T::Proposal, T::AccountId)>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Democracy PublicProps".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(PropIndex,
                                                                                                                                 T::Proposal,
                                                                                                                                 T::AccountId)>>>::key()).unwrap_or_else(||
                                                                                                                                                                             Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(PropIndex,
                                                                                                                                  T::Proposal,
                                                                                                                                  T::AccountId)>>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(PropIndex,
                                                                                                                         T::Proposal,
                                                                                                                         T::AccountId)>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(PropIndex,
                                                                                                                     T::Proposal,
                                                                                                                     T::AccountId)>>>::put(&val,
                                                                                                                                           storage);
        ret
    }
}
#[doc = " Those who have locked a deposit."]
pub struct DepositOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<PropIndex,
                                                                                                   (BalanceOf<T>,
                                                                                                    Vec<T::AccountId>)>
 for DepositOf<T> {
    type
    Query
    =
    Option<(BalanceOf<T>, Vec<T::AccountId>)>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Democracy DepositOf".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &PropIndex)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<PropIndex,
                                                                                                                  (BalanceOf<T>,
                                                                                                                   Vec<T::AccountId>)>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &PropIndex,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<PropIndex,
                                                                                                                  (BalanceOf<T>,
                                                                                                                   Vec<T::AccountId>)>>::key_for(key);
        storage.get(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &PropIndex,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<PropIndex,
                                                                                                                  (BalanceOf<T>,
                                                                                                                   Vec<T::AccountId>)>>::key_for(key);
        storage.take(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &PropIndex,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<PropIndex,
                                                                                                                  (BalanceOf<T>,
                                                                                                                   Vec<T::AccountId>)>>::get(key,
                                                                                                                                             storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<PropIndex,
                                                                                                                  (BalanceOf<T>,
                                                                                                                   Vec<T::AccountId>)>>::insert(key,
                                                                                                                                                &val,
                                                                                                                                                storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<PropIndex,
                                                                                                                  (BalanceOf<T>,
                                                                                                                   Vec<T::AccountId>)>>::remove(key,
                                                                                                                                                storage),
        };
        ret
    }
}
#[doc = " How often (in blocks) new public referenda are launched."]
pub struct LaunchPeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for LaunchPeriod<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Democracy LaunchPeriod".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                        T::BlockNumber::sa(1000))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                         T::BlockNumber::sa(1000))
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
#[doc =
      " The minimum amount to be used as a deposit for a public referendum proposal."]
pub struct MinimumDeposit<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for MinimumDeposit<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Democracy MinimumDeposit".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                      Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&val,
                                                                                                                                    storage);
        ret
    }
}
#[doc = " The delay before enactment for all public referenda."]
pub struct PublicDelay<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for PublicDelay<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Democracy PublicDelay".as_bytes() }
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
#[doc =
      " The maximum number of additional lock periods a voter may offer to strengthen their vote. Multiples of `PublicDelay`."]
pub struct MaxLockPeriods<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<LockPeriods>
 for MaxLockPeriods<T> {
    type
    Query
    =
    LockPeriods;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Democracy MaxLockPeriods".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<LockPeriods>>::key()).unwrap_or_else(||
                                                                                                                                                                     Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<LockPeriods>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<LockPeriods>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<LockPeriods>>::put(&val,
                                                                                                                                   storage);
        ret
    }
}
#[doc = " How often (in blocks) to check for new votes."]
pub struct VotingPeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for VotingPeriod<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Democracy VotingPeriod".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                        T::BlockNumber::sa(1000))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                         T::BlockNumber::sa(1000))
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
#[doc =
      " The next free referendum index, aka the number of referenda started so far."]
pub struct ReferendumCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>
 for ReferendumCount<T> {
    type
    Query
    =
    ReferendumIndex;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Democracy ReferendumCount".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                         Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::put(&val,
                                                                                                                                       storage);
        ret
    }
}
#[doc = " The next referendum index that should be tallied."]
pub struct NextTally<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>
 for NextTally<T> {
    type
    Query
    =
    ReferendumIndex;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Democracy NextTally".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                         Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::put(&val,
                                                                                                                                       storage);
        ret
    }
}
#[doc = " Information concerning any given referendum."]
pub struct ReferendumInfoOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                   (ReferendumInfo<T::BlockNumber,
                                                                                                                   T::Proposal>)>
 for ReferendumInfoOf<T> {
    type
    Query
    =
    Option<(ReferendumInfo<T::BlockNumber, T::Proposal>)>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Democracy ReferendumInfoOf".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &ReferendumIndex)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  (ReferendumInfo<T::BlockNumber,
                                                                                                                                  T::Proposal>)>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &ReferendumIndex,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  (ReferendumInfo<T::BlockNumber,
                                                                                                                                  T::Proposal>)>>::key_for(key);
        storage.get(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &ReferendumIndex,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  (ReferendumInfo<T::BlockNumber,
                                                                                                                                  T::Proposal>)>>::key_for(key);
        storage.take(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &ReferendumIndex,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  (ReferendumInfo<T::BlockNumber,
                                                                                                                                  T::Proposal>)>>::get(key,
                                                                                                                                                       storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  (ReferendumInfo<T::BlockNumber,
                                                                                                                                  T::Proposal>)>>::insert(key,
                                                                                                                                                          &val,
                                                                                                                                                          storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  (ReferendumInfo<T::BlockNumber,
                                                                                                                                  T::Proposal>)>>::remove(key,
                                                                                                                                                          storage),
        };
        ret
    }
}
#[doc = " Queue of successful referenda to be dispatched."]
pub struct DispatchQueue<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                   Vec<Option<(T::Proposal,
                                                                                                               ReferendumIndex)>>>
 for DispatchQueue<T> {
    type
    Query
    =
    Vec<Option<(T::Proposal, ReferendumIndex)>>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Democracy DispatchQueue".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::BlockNumber)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                                  Vec<Option<(T::Proposal,
                                                                                                                              ReferendumIndex)>>>>::prefix().to_vec();
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
                                                                                                                  Vec<Option<(T::Proposal,
                                                                                                                              ReferendumIndex)>>>>::key_for(key);
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
                                                                                                                  Vec<Option<(T::Proposal,
                                                                                                                              ReferendumIndex)>>>>::key_for(key);
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
                                                                                                                  Vec<Option<(T::Proposal,
                                                                                                                              ReferendumIndex)>>>>::get(key,
                                                                                                                                                        storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                              Vec<Option<(T::Proposal,
                                                                                                                          ReferendumIndex)>>>>::insert(key,
                                                                                                                                                       &val,
                                                                                                                                                       storage);
        ret
    }
}
#[doc = " Get the voters for the current proposal."]
pub struct VotersFor<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                   Vec<T::AccountId>>
 for VotersFor<T> {
    type
    Query
    =
    Vec<T::AccountId>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Democracy VotersFor".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &ReferendumIndex)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  Vec<T::AccountId>>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &ReferendumIndex,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  Vec<T::AccountId>>>::key_for(key);
        storage.get(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &ReferendumIndex,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  Vec<T::AccountId>>>::key_for(key);
        storage.take(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &ReferendumIndex,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                                  Vec<T::AccountId>>>::get(key,
                                                                                                                                           storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                              Vec<T::AccountId>>>::insert(key,
                                                                                                                                          &val,
                                                                                                                                          storage);
        ret
    }
}
#[doc =
      " Get the vote in a given referendum of a particular voter. The result is meaningful only if `voters_for` includes the"]
#[doc =
      " voter when called with the referendum (you\'ll get the default `Vote` value otherwise). If you don\'t want to check"]
#[doc =
      " `voters_for`, then you can also check for simple existence with `VoteOf::exists` first."]
pub struct VoteOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(ReferendumIndex,
                                                                                                    T::AccountId),
                                                                                                   Vote>
 for VoteOf<T> {
    type
    Query
    =
    Vote;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Democracy VoteOf".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &(ReferendumIndex, T::AccountId))
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(ReferendumIndex,
                                                                                                                   T::AccountId),
                                                                                                                  Vote>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &(ReferendumIndex,
                                                                                                                                                                     T::AccountId),
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(ReferendumIndex,
                                                                                                                   T::AccountId),
                                                                                                                  Vote>>::key_for(key);
        storage.get(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &(ReferendumIndex,
                                                                                                                                                                      T::AccountId),
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(ReferendumIndex,
                                                                                                                   T::AccountId),
                                                                                                                  Vote>>::key_for(key);
        storage.take(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &(ReferendumIndex,
                                                                                                                                                                        T::AccountId),
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(ReferendumIndex,
                                                                                                                   T::AccountId),
                                                                                                                  Vote>>::get(key,
                                                                                                                              storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(ReferendumIndex,
                                                                                                               T::AccountId),
                                                                                                              Vote>>::insert(key,
                                                                                                                             &val,
                                                                                                                             storage);
        ret
    }
}
#[doc =
      " Who is able to vote for whom. Value is the fund-holding account, key is the vote-transaction-sending account."]
pub struct Proxy<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   T::AccountId>
 for Proxy<T> {
    type
    Query
    =
    Option<T::AccountId>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Democracy Proxy".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::AccountId>>::prefix().to_vec();
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
                                                                                                                  T::AccountId>>::key_for(key);
        storage.get(&key[..]).or_else(|| Default::default())
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
                                                                                                                  T::AccountId>>::key_for(key);
        storage.take(&key[..]).or_else(|| Default::default())
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
                                                                                                                  T::AccountId>>::get(key,
                                                                                                                                      storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::AccountId>>::insert(key,
                                                                                                                                         &val,
                                                                                                                                         storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::AccountId>>::remove(key,
                                                                                                                                         storage),
        };
        ret
    }
}
#[doc = r" Linkage data of an element (it's successor and predecessor)"]
pub(crate) struct __LinkageForDelegationsDoNotUse<Key> {
    #[doc = r" Previous element key in storage (None for the first element)"]
    pub previous: Option<Key>,
    #[doc = r" Next element key in storage (None for the last element)"]
    pub next: Option<Key>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR___LinkageForDelegationsDoNotUse: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Key> _parity_codec::Encode for
         __LinkageForDelegationsDoNotUse<Key> where
         Option<Key>: _parity_codec::Encode,
         Option<Key>: _parity_codec::Encode,
         Option<Key>: _parity_codec::Encode,
         Option<Key>: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.previous);
                dest.push(&self.next);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR___LinkageForDelegationsDoNotUse: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Key> _parity_codec::Decode for
         __LinkageForDelegationsDoNotUse<Key> where
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(__LinkageForDelegationsDoNotUse{previous:
                                                         _parity_codec::Decode::decode(input)?,
                                                     next:
                                                         _parity_codec::Decode::decode(input)?,})
            }
        }
    };
mod __linked_map_details_for_delegations_do_not_use {
    use super::*;
    #[doc =
          r" Re-exported version of linkage to overcome proc-macro derivation issue."]
    pub(crate) use super::__LinkageForDelegationsDoNotUse as Linkage;
    impl <Key> Default for Linkage<Key> {
        fn default() -> Self { Self{previous: None, next: None,} }
    }
    #[doc = r" A key-value pair iterator for enumerable map."]
    pub(crate) struct Enumerator<'a, S, K, V> {
        pub storage: &'a S,
        pub next: Option<K>,
        pub _data: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<V>,
    }
    impl <'a,
          S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>,
          T: Trait> Iterator for
     Enumerator<'a, S, T::AccountId, ((T::AccountId, LockPeriods), T)> where
     T: 'a {
        type
        Item
        =
        (T::AccountId, (T::AccountId, LockPeriods));
        fn next(&mut self) -> Option<Self::Item> {
            let next = self.next.take()?;
            let key_for =
                <super::Delegations<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      (T::AccountId,
                                                                                                                       LockPeriods)>>::key_for(&next);
            let (val, linkage):
                    ((T::AccountId, LockPeriods), Linkage<T::AccountId>) =
                self.storage.get(&*key_for).expect("previous/next only contain existing entires; we enumerate using next; entry exists; qed");
            self.next = linkage.next;
            Some((next, val))
        }
    }
    pub(crate) trait Utils<T: Trait> {
        #[doc = r" Update linkage when this element is removed."]
        #[doc = r""]
        #[doc = r" Takes care of updating previous and next elements points"]
        #[doc = r" as well as updates head if the element is first or last."]
        fn remove_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(linkage:
                                                                                                                                                                                  Linkage<T::AccountId>,
                                                                                                                                                                              storage:
                                                                                                                                                                                  &S);
        #[doc = r" Read the contained data and it's linkage."]
        fn read_with_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                     &S,
                                                                                                                                                                                 key:
                                                                                                                                                                                     &[u8])
        -> Option<((T::AccountId, LockPeriods), Linkage<T::AccountId>)>;
        #[doc = r" Generate linkage for newly inserted element."]
        #[doc = r""]
        #[doc = r" Takes care of updating head and previous head's pointer."]
        fn new_head_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                    &S,
                                                                                                                                                                                key:
                                                                                                                                                                                    &T::AccountId)
        -> Linkage<T::AccountId>;
        #[doc = r" Read current head pointer."]
        fn read_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                             &S)
        -> Option<T::AccountId>;
        #[doc = r" Overwrite current head pointer."]
        #[doc = r""]
        #[doc = r" If `None` is given head is removed from storage."]
        fn write_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                              &S,
                                                                                                                                                                          head:
                                                                                                                                                                              Option<&T::AccountId>);
    }
}
#[doc =
      " Get the account (and lock periods) to which another account is delegating vote."]
pub struct Delegations<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::__linked_map_details_for_delegations_do_not_use::Utils<T> for
 Delegations<T> {
    fn remove_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(linkage:
                                                                                                                                                                              self::__linked_map_details_for_delegations_do_not_use::Linkage<T::AccountId>,
                                                                                                                                                                          storage:
                                                                                                                                                                              &S) {
        use self::__linked_map_details_for_delegations_do_not_use::Utils;
        let next_key =
            linkage.next.as_ref().map(|x|
                                          <Self as
                                              self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                                (T::AccountId,
                                                                                                                                                 LockPeriods)>>::key_for(x));
        let prev_key =
            linkage.previous.as_ref().map(|x|
                                              <Self as
                                                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                                    (T::AccountId,
                                                                                                                                                     LockPeriods)>>::key_for(x));
        if let Some(prev_key) = prev_key {
            let mut res =
                Self::read_with_linkage(storage,
                                        &*prev_key).expect("Linkage is updated in case entry is removed; it always points to existing keys; qed");
            res.1.next = linkage.next;
            storage.put(&*prev_key, &res);
        } else { Self::write_head(storage, linkage.next.as_ref()); }
        if let Some(next_key) = next_key {
            let mut res =
                Self::read_with_linkage(storage,
                                        &*next_key).expect("Linkage is updated in case entry is removed; it always points to existing keys; qed");
            res.1.previous = linkage.previous;
            storage.put(&*next_key, &res);
        }
    }
    fn read_with_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                 &S,
                                                                                                                                                                             key:
                                                                                                                                                                                 &[u8])
     ->
         Option<((T::AccountId, LockPeriods),
                 self::__linked_map_details_for_delegations_do_not_use::Linkage<T::AccountId>)> {
        storage.get(key)
    }
    fn new_head_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                &S,
                                                                                                                                                                            key:
                                                                                                                                                                                &T::AccountId)
     ->
         self::__linked_map_details_for_delegations_do_not_use::Linkage<T::AccountId> {
        use self::__linked_map_details_for_delegations_do_not_use::Utils;
        if let Some(head) = Self::read_head(storage) {
            {
                let head_key =
                    <Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                          (T::AccountId,
                                                                                                                           LockPeriods)>>::key_for(&head);
                let (data, linkage) =
                    Self::read_with_linkage(storage,
                                            &*head_key).expect(r#"
								head is set when first element is inserted and unset when last element is removed;
								if head is Some then it points to existing key; qed
							"#);
                storage.put(&*head_key,
                            &(data,
                              self::__linked_map_details_for_delegations_do_not_use::Linkage{next:
                                                                                                 linkage.next.as_ref(),
                                                                                             previous:
                                                                                                 Some(key),}));
            }
            Self::write_head(storage, Some(key));
            let mut linkage =
                self::__linked_map_details_for_delegations_do_not_use::Linkage::default();
            linkage.next = Some(head);
            linkage
        } else {
            Self::write_head(storage, Some(key));
            self::__linked_map_details_for_delegations_do_not_use::Linkage::default()
        }
    }
    fn read_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                         &S)
     -> Option<T::AccountId> {
        storage.get("head of Democracy Delegations".as_bytes())
    }
    fn write_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                          &S,
                                                                                                                                                                      head:
                                                                                                                                                                          Option<&T::AccountId>) {
        match head {
            Some(head) =>
            storage.put("head of Democracy Delegations".as_bytes(), head),
            None => storage.kill("head of Democracy Delegations".as_bytes()),
        }
    }
}
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   (T::AccountId,
                                                                                                    LockPeriods)>
 for Delegations<T> {
    type
    Query
    =
    (T::AccountId, LockPeriods);
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Democracy Delegations".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(key: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key_for =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  (T::AccountId,
                                                                                                                   LockPeriods)>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(&key,
                                                                                            &mut key_for);
        key_for
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &T::AccountId,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        storage.get(&*<Self as
                          self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                            (T::AccountId,
                                                                                                                             LockPeriods)>>::key_for(key)).unwrap_or_else(||
                                                                                                                                                                              Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &T::AccountId,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        use self::__linked_map_details_for_delegations_do_not_use::Utils;
        let res:
                Option<((T::AccountId, LockPeriods),
                        self::__linked_map_details_for_delegations_do_not_use::Linkage<T::AccountId>)> =
            storage.take(&*<Self as
                               self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                 (T::AccountId,
                                                                                                                                  LockPeriods)>>::key_for(key));
        match res {
            Some((data, linkage)) => {
                Self::remove_linkage(linkage, storage);
                data
            }
            None => Default::default(),
        }
    }
    #[doc = r" Remove the value under a key."]
    fn remove<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::AccountId,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S) {
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              (T::AccountId,
                                                                                                               LockPeriods)>>::take(key,
                                                                                                                                    storage);
    }
    #[doc =
          r" Store a value to be associated with the given key from the map."]
    fn insert<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::AccountId,
                                                                                                                                                                  val:
                                                                                                                                                                      &(T::AccountId,
                                                                                                                                                                        LockPeriods),
                                                                                                                                                                  storage:
                                                                                                                                                                      &S) {
        use self::__linked_map_details_for_delegations_do_not_use::Utils;
        let key_for =
            &*<Self as
                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                    (T::AccountId,
                                                                                                                     LockPeriods)>>::key_for(key);
        let linkage =
            match Self::read_with_linkage(storage, key_for) {
                Some((_data, linkage)) => linkage,
                None => Self::new_head_linkage(storage, key),
            };
        storage.put(key_for, &(val, linkage))
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
        use self::__linked_map_details_for_delegations_do_not_use::Utils;
        let key_for =
            &*<Self as
                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                    (T::AccountId,
                                                                                                                     LockPeriods)>>::key_for(key);
        let (mut val, linkage) =
            Self::read_with_linkage(storage,
                                    key_for).map(|(data, linkage)|
                                                     (data,
                                                      Some(linkage))).unwrap_or_else(||
                                                                                         (Default::default(),
                                                                                          None));
        let ret = f(&mut val);
        match linkage {
            Some(linkage) => storage.put(key_for, &(val, linkage)),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  (T::AccountId,
                                                                                                                   LockPeriods)>>::insert(key,
                                                                                                                                          &val,
                                                                                                                                          storage),
        };
        ret
    }
}
impl <T: 'static + Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::EnumerableStorageMap<T::AccountId,
                                                                                                             (T::AccountId,
                                                                                                              LockPeriods)>
 for Delegations<T> {
    fn head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                    &S)
     -> Option<T::AccountId> {
        use self::__linked_map_details_for_delegations_do_not_use::Utils;
        Self::read_head(storage)
    }
    fn enumerate<'a,
                 S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                         &'a S)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::boxed::Box<dyn Iterator<Item
                                                                                     =
                                                                                     (T::AccountId,
                                                                                      (T::AccountId,
                                                                                       LockPeriods))> +
                                                                                     'a>
     where T::AccountId: 'a, (T::AccountId, LockPeriods): 'a {
        use self::__linked_map_details_for_delegations_do_not_use::{Utils,
                                                                    Enumerator};
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::boxed::Box::new(Enumerator{next:
                                                                                                        Self::read_head(storage),
                                                                                                    storage,
                                                                                                    _data:
                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData::<((T::AccountId,
                                                                                                                                                                                                 LockPeriods),
                                                                                                                                                                                                T)>::default(),})
    }
}
trait Store {
    type
    PublicPropCount;
    type
    PublicProps;
    type
    DepositOf;
    type
    LaunchPeriod;
    type
    MinimumDeposit;
    type
    PublicDelay;
    type
    MaxLockPeriods;
    type
    VotingPeriod;
    type
    ReferendumCount;
    type
    NextTally;
    type
    ReferendumInfoOf;
    type
    DispatchQueue;
    type
    VotersFor;
    type
    VoteOf;
    type
    Proxy;
    type
    Delegations;
}
#[doc(hidden)]
pub struct __GetByteStructPublicPropCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_PublicPropCount:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructPublicPropCount<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_PublicPropCount.get_or_init(||
                                                                {
                                                                    let def_val:
                                                                            PropIndex =
                                                                        Default::default();
                                                                    <PropIndex
                                                                        as
                                                                        Encode>::encode(&def_val)
                                                                }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructPublicProps<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_PublicProps:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructPublicProps<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_PublicProps.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        Vec<(PropIndex,
                                                                             T::Proposal,
                                                                             T::AccountId)> =
                                                                    Default::default();
                                                                <Vec<(PropIndex,
                                                                      T::Proposal,
                                                                      T::AccountId)>
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructDepositOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_DepositOf:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructDepositOf<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_DepositOf.get_or_init(||
                                                          {
                                                              let def_val:
                                                                      Option<(BalanceOf<T>,
                                                                              Vec<T::AccountId>)> =
                                                                  Default::default();
                                                              <Option<(BalanceOf<T>,
                                                                       Vec<T::AccountId>)>
                                                                  as
                                                                  Encode>::encode(&def_val)
                                                          }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructLaunchPeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_LaunchPeriod:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructLaunchPeriod<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_LaunchPeriod.get_or_init(||
                                                             {
                                                                 let def_val:
                                                                         T::BlockNumber =
                                                                     T::BlockNumber::sa(1000);
                                                                 <T::BlockNumber
                                                                     as
                                                                     Encode>::encode(&def_val)
                                                             }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructMinimumDeposit<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_MinimumDeposit:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructMinimumDeposit<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_MinimumDeposit.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           BalanceOf<T> =
                                                                       Default::default();
                                                                   <BalanceOf<T>
                                                                       as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructPublicDelay<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_PublicDelay:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructPublicDelay<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_PublicDelay.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        T::BlockNumber =
                                                                    Default::default();
                                                                <T::BlockNumber
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructMaxLockPeriods<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_MaxLockPeriods:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructMaxLockPeriods<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_MaxLockPeriods.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           LockPeriods =
                                                                       Default::default();
                                                                   <LockPeriods
                                                                       as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructVotingPeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_VotingPeriod:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructVotingPeriod<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_VotingPeriod.get_or_init(||
                                                             {
                                                                 let def_val:
                                                                         T::BlockNumber =
                                                                     T::BlockNumber::sa(1000);
                                                                 <T::BlockNumber
                                                                     as
                                                                     Encode>::encode(&def_val)
                                                             }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructReferendumCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ReferendumCount:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructReferendumCount<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ReferendumCount.get_or_init(||
                                                                {
                                                                    let def_val:
                                                                            ReferendumIndex =
                                                                        Default::default();
                                                                    <ReferendumIndex
                                                                        as
                                                                        Encode>::encode(&def_val)
                                                                }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructNextTally<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_NextTally:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNextTally<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_NextTally.get_or_init(||
                                                          {
                                                              let def_val:
                                                                      ReferendumIndex =
                                                                  Default::default();
                                                              <ReferendumIndex
                                                                  as
                                                                  Encode>::encode(&def_val)
                                                          }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructReferendumInfoOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ReferendumInfoOf:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructReferendumInfoOf<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ReferendumInfoOf.get_or_init(||
                                                                 {
                                                                     let def_val:
                                                                             Option<(ReferendumInfo<T::BlockNumber,
                                                                                                    T::Proposal>)> =
                                                                         Default::default();
                                                                     <Option<(ReferendumInfo<T::BlockNumber,
                                                                                             T::Proposal>)>
                                                                         as
                                                                         Encode>::encode(&def_val)
                                                                 }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructDispatchQueue<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_DispatchQueue:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructDispatchQueue<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_DispatchQueue.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          Vec<Option<(T::Proposal,
                                                                                      ReferendumIndex)>> =
                                                                      Default::default();
                                                                  <Vec<Option<(T::Proposal,
                                                                               ReferendumIndex)>>
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructVotersFor<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_VotersFor:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructVotersFor<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_VotersFor.get_or_init(||
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
#[doc(hidden)]
pub struct __GetByteStructVoteOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_VoteOf:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructVoteOf<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_VoteOf.get_or_init(||
                                                       {
                                                           let def_val: Vote =
                                                               Default::default();
                                                           <Vote as
                                                               Encode>::encode(&def_val)
                                                       }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructProxy<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Proxy:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructProxy<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Proxy.get_or_init(||
                                                      {
                                                          let def_val:
                                                                  Option<T::AccountId> =
                                                              Default::default();
                                                          <Option<T::AccountId>
                                                              as
                                                              Encode>::encode(&def_val)
                                                      }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructDelegations<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Delegations:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructDelegations<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Delegations.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        (T::AccountId,
                                                                         LockPeriods) =
                                                                    Default::default();
                                                                <(T::AccountId,
                                                                  LockPeriods)
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    PublicPropCount
    =
    PublicPropCount<T>;
    type
    PublicProps
    =
    PublicProps<T>;
    type
    DepositOf
    =
    DepositOf<T>;
    type
    LaunchPeriod
    =
    LaunchPeriod<T>;
    type
    MinimumDeposit
    =
    MinimumDeposit<T>;
    type
    PublicDelay
    =
    PublicDelay<T>;
    type
    MaxLockPeriods
    =
    MaxLockPeriods<T>;
    type
    VotingPeriod
    =
    VotingPeriod<T>;
    type
    ReferendumCount
    =
    ReferendumCount<T>;
    type
    NextTally
    =
    NextTally<T>;
    type
    ReferendumInfoOf
    =
    ReferendumInfoOf<T>;
    type
    DispatchQueue
    =
    DispatchQueue<T>;
    type
    VotersFor
    =
    VotersFor<T>;
    type
    VoteOf
    =
    VoteOf<T>;
    type
    Proxy
    =
    Proxy<T>;
    type
    Delegations
    =
    Delegations<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc = " The number of (public) proposals that have been made so far."]
    pub fn public_prop_count() -> PropIndex {
        <PublicPropCount<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<PropIndex>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The public proposals. Unsorted."]
    pub fn public_props() -> Vec<(PropIndex, T::Proposal, T::AccountId)> {
        <PublicProps<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(PropIndex,
                                                                                                                     T::Proposal,
                                                                                                                     T::AccountId)>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Those who have locked a deposit."]
    pub fn deposit_of<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<PropIndex>>(key:
                                                                                                                         K)
     -> Option<(BalanceOf<T>, Vec<T::AccountId>)> {
        <DepositOf<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<PropIndex,
                                                                                                              (BalanceOf<T>,
                                                                                                               Vec<T::AccountId>)>>::get(key.borrow(),
                                                                                                                                         &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " How often (in blocks) new public referenda are launched."]
    pub fn launch_period() -> T::BlockNumber {
        <LaunchPeriod<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The minimum amount to be used as a deposit for a public referendum proposal."]
    pub fn minimum_deposit() -> BalanceOf<T> {
        <MinimumDeposit<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The delay before enactment for all public referenda."]
    pub fn public_delay() -> T::BlockNumber {
        <PublicDelay<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The maximum number of additional lock periods a voter may offer to strengthen their vote. Multiples of `PublicDelay`."]
    pub fn max_lock_periods() -> LockPeriods {
        <MaxLockPeriods<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<LockPeriods>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " How often (in blocks) to check for new votes."]
    pub fn voting_period() -> T::BlockNumber {
        <VotingPeriod<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The next free referendum index, aka the number of referenda started so far."]
    pub fn referendum_count() -> ReferendumIndex {
        <ReferendumCount<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The next referendum index that should be tallied."]
    pub fn next_tally() -> ReferendumIndex {
        <NextTally<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Information concerning any given referendum."]
    pub fn referendum_info<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<ReferendumIndex>>(key:
                                                                                                                                    K)
     -> Option<(ReferendumInfo<T::BlockNumber, T::Proposal>)> {
        <ReferendumInfoOf<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                              (ReferendumInfo<T::BlockNumber,
                                                                                                                              T::Proposal>)>>::get(key.borrow(),
                                                                                                                                                   &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Queue of successful referenda to be dispatched."]
    pub fn dispatch_queue<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::BlockNumber>>(key:
                                                                                                                                  K)
     -> Vec<Option<(T::Proposal, ReferendumIndex)>> {
        <DispatchQueue<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::BlockNumber,
                                                                                                              Vec<Option<(T::Proposal,
                                                                                                                          ReferendumIndex)>>>>::get(key.borrow(),
                                                                                                                                                    &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Get the voters for the current proposal."]
    pub fn voters_for<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<ReferendumIndex>>(key:
                                                                                                                               K)
     -> Vec<T::AccountId> {
        <VotersFor<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ReferendumIndex,
                                                                                                              Vec<T::AccountId>>>::get(key.borrow(),
                                                                                                                                       &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Get the vote in a given referendum of a particular voter. The result is meaningful only if `voters_for` includes the"]
    #[doc =
          " voter when called with the referendum (you\'ll get the default `Vote` value otherwise). If you don\'t want to check"]
    #[doc =
          " `voters_for`, then you can also check for simple existence with `VoteOf::exists` first."]
    pub fn vote_of<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(ReferendumIndex,
                                                                                                       T::AccountId)>>(key:
                                                                                                                           K)
     -> Vote {
        <VoteOf<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(ReferendumIndex,
                                                                                                               T::AccountId),
                                                                                                              Vote>>::get(key.borrow(),
                                                                                                                          &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Who is able to vote for whom. Value is the fund-holding account, key is the vote-transaction-sending account."]
    pub fn proxy<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                       K)
     -> Option<T::AccountId> {
        <Proxy<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::AccountId>>::get(key.borrow(),
                                                                                                                                  &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Get the account (and lock periods) to which another account is delegating vote."]
    pub fn delegations<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                             K)
     -> (T::AccountId, LockPeriods) {
        <Delegations<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              (T::AccountId,
                                                                                                               LockPeriods)>>::get(key.borrow(),
                                                                                                                                   &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PublicPropCount"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PropIndex")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPublicPropCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of (public) proposals that have been made so far."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PublicProps"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(PropIndex, T::Proposal, T::AccountId)>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPublicProps::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The public proposals. Unsorted."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("DepositOf"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PropIndex"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(BalanceOf<T>, Vec<T::AccountId>)"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDepositOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Those who have locked a deposit."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LaunchPeriod"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLaunchPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How often (in blocks) new public referenda are launched."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MinimumDeposit"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMinimumDeposit::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The minimum amount to be used as a deposit for a public referendum proposal."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PublicDelay"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPublicDelay::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The delay before enactment for all public referenda."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MaxLockPeriods"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LockPeriods")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMaxLockPeriods::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The maximum number of additional lock periods a voter may offer to strengthen their vote. Multiples of `PublicDelay`."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotingPeriod"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotingPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How often (in blocks) to check for new votes."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumCount"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumIndex")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructReferendumCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next free referendum index, aka the number of referenda started so far."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextTally"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumIndex")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextTally::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next referendum index that should be tallied."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumInfoOf"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumIndex"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(ReferendumInfo<T::BlockNumber, T::Proposal>)"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructReferendumInfoOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Information concerning any given referendum."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("DispatchQueue"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<Option<(T::Proposal, ReferendumIndex)>>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDispatchQueue::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Queue of successful referenda to be dispatched."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotersFor"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumIndex"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotersFor::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Get the voters for the current proposal."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteOf"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(ReferendumIndex, T::AccountId)"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vote"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVoteOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Get the vote in a given referendum of a particular voter. The result is meaningful only if `voters_for` includes the",
                                                                                                                                                                                                                                                                                                                                                                                                    " voter when called with the referendum (you\'ll get the default `Vote` value otherwise). If you don\'t want to check",
                                                                                                                                                                                                                                                                                                                                                                                                    " `voters_for`, then you can also check for simple existence with `VoteOf::exists` first."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proxy"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProxy::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Who is able to vote for whom. Value is the fund-holding account, key is the vote-transaction-sending account."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Delegations"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(T::AccountId, LockPeriods)"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       true,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDelegations::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Get the account (and lock periods) to which another account is delegating vote."]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PublicPropCount"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PropIndex")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPublicPropCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of (public) proposals that have been made so far."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PublicProps"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(PropIndex, T::Proposal, T::AccountId)>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPublicProps::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The public proposals. Unsorted."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("DepositOf"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PropIndex"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(BalanceOf<T>, Vec<T::AccountId>)"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDepositOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Those who have locked a deposit."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LaunchPeriod"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLaunchPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How often (in blocks) new public referenda are launched."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MinimumDeposit"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMinimumDeposit::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The minimum amount to be used as a deposit for a public referendum proposal."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PublicDelay"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPublicDelay::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The delay before enactment for all public referenda."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MaxLockPeriods"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LockPeriods")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMaxLockPeriods::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The maximum number of additional lock periods a voter may offer to strengthen their vote. Multiples of `PublicDelay`."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotingPeriod"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotingPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" How often (in blocks) to check for new votes."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumCount"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumIndex")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructReferendumCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next free referendum index, aka the number of referenda started so far."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextTally"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumIndex")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextTally::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next referendum index that should be tallied."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumInfoOf"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumIndex"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(ReferendumInfo<T::BlockNumber, T::Proposal>)"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructReferendumInfoOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Information concerning any given referendum."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("DispatchQueue"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<Option<(T::Proposal, ReferendumIndex)>>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDispatchQueue::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Queue of successful referenda to be dispatched."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VotersFor"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReferendumIndex"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVotersFor::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Get the voters for the current proposal."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VoteOf"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(ReferendumIndex, T::AccountId)"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vote"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVoteOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Get the vote in a given referendum of a particular voter. The result is meaningful only if `voters_for` includes the",
                                                                                                                                                                                                              " voter when called with the referendum (you\'ll get the default `Vote` value otherwise). If you don\'t want to check",
                                                                                                                                                                                                              " `voters_for`, then you can also check for simple existence with `VoteOf::exists` first."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proxy"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProxy::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Who is able to vote for whom. Value is the fund-holding account, key is the vote-transaction-sending account."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Delegations"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(T::AccountId, LockPeriods)"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 true,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDelegations::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Get the account (and lock periods) to which another account is delegating vote."]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Democracy" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, LockPeriods : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, LockPeriods : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    #[doc = " How often (in blocks) new public referenda are launched."]
    pub launch_period: T::BlockNumber,
    #[doc =
          " The minimum amount to be used as a deposit for a public referendum proposal."]
    pub minimum_deposit: BalanceOf<T>,
    #[doc = " The delay before enactment for all public referenda."]
    pub public_delay: T::BlockNumber,
    #[doc =
          " The maximum number of additional lock periods a voter may offer to strengthen their vote. Multiples of `PublicDelay`."]
    pub max_lock_periods: LockPeriods,
    #[doc = " How often (in blocks) to check for new votes."]
    pub voting_period: T::BlockNumber,
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
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         LockPeriods: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
         {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "GenesisConfig",
                                                               false as usize
                                                                   + 1 + 1 + 1
                                                                   + 1 + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "launchPeriod",
                                                                    &self.launch_period)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "minimumDeposit",
                                                                    &self.minimum_deposit)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "publicDelay",
                                                                    &self.public_delay)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "maxLockPeriods",
                                                                    &self.max_lock_periods)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "votingPeriod",
                                                                    &self.voting_period)
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
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         LockPeriods: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
         {
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
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 5")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "launchPeriod" =>
                            _serde::export::Ok(__Field::__field0),
                            "minimumDeposit" =>
                            _serde::export::Ok(__Field::__field1),
                            "publicDelay" =>
                            _serde::export::Ok(__Field::__field2),
                            "maxLockPeriods" =>
                            _serde::export::Ok(__Field::__field3),
                            "votingPeriod" =>
                            _serde::export::Ok(__Field::__field4),
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
                            b"launchPeriod" =>
                            _serde::export::Ok(__Field::__field0),
                            b"minimumDeposit" =>
                            _serde::export::Ok(__Field::__field1),
                            b"publicDelay" =>
                            _serde::export::Ok(__Field::__field2),
                            b"maxLockPeriods" =>
                            _serde::export::Ok(__Field::__field3),
                            b"votingPeriod" =>
                            _serde::export::Ok(__Field::__field4),
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
                       T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       LockPeriods: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 LockPeriods: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct GenesisConfig with 5 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct GenesisConfig with 5 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct GenesisConfig with 5 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<LockPeriods>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct GenesisConfig with 5 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct GenesisConfig with 5 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{launch_period:
                                                             __field0,
                                                         minimum_deposit:
                                                             __field1,
                                                         public_delay:
                                                             __field2,
                                                         max_lock_periods:
                                                             __field3,
                                                         voting_period:
                                                             __field4,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<T::BlockNumber> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<T::BlockNumber> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<LockPeriods> =
                            _serde::export::None;
                        let mut __field4:
                                _serde::export::Option<T::BlockNumber> =
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
                                                                       _serde::de::Error>::duplicate_field("launchPeriod"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::BlockNumber>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("minimumDeposit"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<BalanceOf<T>>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("publicDelay"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::BlockNumber>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("maxLockPeriods"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<LockPeriods>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("votingPeriod"));
                                    }
                                    __field4 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::BlockNumber>(&mut __map)
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
                                match _serde::private::de::missing_field("launchPeriod")
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
                                match _serde::private::de::missing_field("minimumDeposit")
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
                                match _serde::private::de::missing_field("publicDelay")
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
                                match _serde::private::de::missing_field("maxLockPeriods")
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
                                match _serde::private::de::missing_field("votingPeriod")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{launch_period:
                                                             __field0,
                                                         minimum_deposit:
                                                             __field1,
                                                         public_delay:
                                                             __field2,
                                                         max_lock_periods:
                                                             __field3,
                                                         voting_period:
                                                             __field4,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["launchPeriod", "minimumDeposit", "publicDelay",
                      "maxLockPeriods", "votingPeriod"];
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
        GenesisConfig{launch_period: T::BlockNumber::sa(1000),
                      minimum_deposit: Default::default(),
                      public_delay: Default::default(),
                      max_lock_periods: Default::default(),
                      voting_period: T::BlockNumber::sa(1000),}
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
            let v = (|_| 0 as PropIndex)(&self);
            <PublicPropCount<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<PropIndex>>::put(&v,
                                                                                                                                     &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.launch_period.clone()))(&self);
            <LaunchPeriod<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                          &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.minimum_deposit.clone()))(&self);
            <MinimumDeposit<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&v,
                                                                                                                                        &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.public_delay.clone()))(&self);
            <PublicDelay<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                          &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.max_lock_periods.clone()))(&self);
            <MaxLockPeriods<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<LockPeriods>>::put(&v,
                                                                                                                                       &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.voting_period.clone()))(&self);
            <VotingPeriod<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                          &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v = (|_| 0 as ReferendumIndex)(&self);
            <ReferendumCount<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::put(&v,
                                                                                                                                           &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v = (|_| 0 as ReferendumIndex)(&self);
            <NextTally<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ReferendumIndex>>::put(&v,
                                                                                                                                           &storage);
        }
        let r = storage.into_inner();
        (|_, _, _| { })(r, c, &self);
        Ok(())
    }
}
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T> = RawEvent<BalanceOf<T>, <T as system::Trait>::AccountId>;
/// Events for this module.
///
#[structural_match]
pub enum RawEvent<Balance, AccountId> {
    Proposed(PropIndex, Balance),
    Tabled(PropIndex, Balance, Vec<AccountId>),
    Started(ReferendumIndex, VoteThreshold),
    Passed(ReferendumIndex),
    NotPassed(ReferendumIndex),
    Cancelled(ReferendumIndex),
    Executed(ReferendumIndex, bool),
    Delegated(AccountId, AccountId),
    Undelegated(AccountId),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::clone::Clone, AccountId: ::std::clone::Clone>
 ::std::clone::Clone for RawEvent<Balance, AccountId> {
    #[inline]
    fn clone(&self) -> RawEvent<Balance, AccountId> {
        match (&*self,) {
            (&RawEvent::Proposed(ref __self_0, ref __self_1),) =>
            RawEvent::Proposed(::std::clone::Clone::clone(&(*__self_0)),
                               ::std::clone::Clone::clone(&(*__self_1))),
            (&RawEvent::Tabled(ref __self_0, ref __self_1, ref __self_2),) =>
            RawEvent::Tabled(::std::clone::Clone::clone(&(*__self_0)),
                             ::std::clone::Clone::clone(&(*__self_1)),
                             ::std::clone::Clone::clone(&(*__self_2))),
            (&RawEvent::Started(ref __self_0, ref __self_1),) =>
            RawEvent::Started(::std::clone::Clone::clone(&(*__self_0)),
                              ::std::clone::Clone::clone(&(*__self_1))),
            (&RawEvent::Passed(ref __self_0),) =>
            RawEvent::Passed(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::NotPassed(ref __self_0),) =>
            RawEvent::NotPassed(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::Cancelled(ref __self_0),) =>
            RawEvent::Cancelled(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::Executed(ref __self_0, ref __self_1),) =>
            RawEvent::Executed(::std::clone::Clone::clone(&(*__self_0)),
                               ::std::clone::Clone::clone(&(*__self_1))),
            (&RawEvent::Delegated(ref __self_0, ref __self_1),) =>
            RawEvent::Delegated(::std::clone::Clone::clone(&(*__self_0)),
                                ::std::clone::Clone::clone(&(*__self_1))),
            (&RawEvent::Undelegated(ref __self_0),) =>
            RawEvent::Undelegated(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::PartialEq, AccountId: ::std::cmp::PartialEq>
 ::std::cmp::PartialEq for RawEvent<Balance, AccountId> {
    #[inline]
    fn eq(&self, other: &RawEvent<Balance, AccountId>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::Proposed(ref __self_0, ref __self_1),
                     &RawEvent::Proposed(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    (&RawEvent::Tabled(ref __self_0, ref __self_1,
                                       ref __self_2),
                     &RawEvent::Tabled(ref __arg_1_0, ref __arg_1_1,
                                       ref __arg_1_2)) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2),
                    (&RawEvent::Started(ref __self_0, ref __self_1),
                     &RawEvent::Started(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    (&RawEvent::Passed(ref __self_0),
                     &RawEvent::Passed(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::NotPassed(ref __self_0),
                     &RawEvent::NotPassed(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::Cancelled(ref __self_0),
                     &RawEvent::Cancelled(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::Executed(ref __self_0, ref __self_1),
                     &RawEvent::Executed(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    (&RawEvent::Delegated(ref __self_0, ref __self_1),
                     &RawEvent::Delegated(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    (&RawEvent::Undelegated(ref __self_0),
                     &RawEvent::Undelegated(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<Balance, AccountId>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::Proposed(ref __self_0, ref __self_1),
                     &RawEvent::Proposed(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    (&RawEvent::Tabled(ref __self_0, ref __self_1,
                                       ref __self_2),
                     &RawEvent::Tabled(ref __arg_1_0, ref __arg_1_1,
                                       ref __arg_1_2)) =>
                    (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1)
                        || (*__self_2) != (*__arg_1_2),
                    (&RawEvent::Started(ref __self_0, ref __self_1),
                     &RawEvent::Started(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    (&RawEvent::Passed(ref __self_0),
                     &RawEvent::Passed(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::NotPassed(ref __self_0),
                     &RawEvent::NotPassed(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::Cancelled(ref __self_0),
                     &RawEvent::Cancelled(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::Executed(ref __self_0, ref __self_1),
                     &RawEvent::Executed(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    (&RawEvent::Delegated(ref __self_0, ref __self_1),
                     &RawEvent::Delegated(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    (&RawEvent::Undelegated(ref __self_0),
                     &RawEvent::Undelegated(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::Eq, AccountId: ::std::cmp::Eq> ::std::cmp::Eq for
 RawEvent<Balance, AccountId> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<PropIndex>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<PropIndex>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<Vec<AccountId>>;
            let _: ::std::cmp::AssertParamIsEq<ReferendumIndex>;
            let _: ::std::cmp::AssertParamIsEq<VoteThreshold>;
            let _: ::std::cmp::AssertParamIsEq<ReferendumIndex>;
            let _: ::std::cmp::AssertParamIsEq<ReferendumIndex>;
            let _: ::std::cmp::AssertParamIsEq<ReferendumIndex>;
            let _: ::std::cmp::AssertParamIsEq<ReferendumIndex>;
            let _: ::std::cmp::AssertParamIsEq<bool>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawEvent: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance, AccountId> _parity_codec::Encode for
         RawEvent<Balance, AccountId> where Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode,
         Vec<AccountId>: _parity_codec::Encode,
         Vec<AccountId>: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::Proposed(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::Tabled(ref aa, ref ba, ref ca) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    RawEvent::Started(ref aa, ref ba) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::Passed(ref aa) => {
                        dest.push_byte(3usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::NotPassed(ref aa) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::Cancelled(ref aa) => {
                        dest.push_byte(5usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::Executed(ref aa, ref ba) => {
                        dest.push_byte(6usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::Delegated(ref aa, ref ba) => {
                        dest.push_byte(7usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::Undelegated(ref aa) => {
                        dest.push_byte(8usize as u8);
                        dest.push(aa);
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
        impl <Balance, AccountId> _parity_codec::Decode for
         RawEvent<Balance, AccountId> where Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode,
         Vec<AccountId>: _parity_codec::Decode,
         Vec<AccountId>: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::Proposed(_parity_codec::Decode::decode(input)?,
                                                _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(RawEvent::Tabled(_parity_codec::Decode::decode(input)?,
                                              _parity_codec::Decode::decode(input)?,
                                              _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(RawEvent::Started(_parity_codec::Decode::decode(input)?,
                                               _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 3usize as u8 => {
                        Some(RawEvent::Passed(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 4usize as u8 => {
                        Some(RawEvent::NotPassed(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 5usize as u8 => {
                        Some(RawEvent::Cancelled(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 6usize as u8 => {
                        Some(RawEvent::Executed(_parity_codec::Decode::decode(input)?,
                                                _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 7usize as u8 => {
                        Some(RawEvent::Delegated(_parity_codec::Decode::decode(input)?,
                                                 _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 8usize as u8 => {
                        Some(RawEvent::Undelegated(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::fmt::Debug, AccountId: ::std::fmt::Debug>
 ::std::fmt::Debug for RawEvent<Balance, AccountId> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawEvent::Proposed(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("Proposed");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
            (&RawEvent::Tabled(ref __self_0, ref __self_1, ref __self_2),) =>
            {
                let mut debug_trait_builder = f.debug_tuple("Tabled");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                debug_trait_builder.finish()
            }
            (&RawEvent::Started(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("Started");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
            (&RawEvent::Passed(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Passed");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::NotPassed(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("NotPassed");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::Cancelled(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Cancelled");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::Executed(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("Executed");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
            (&RawEvent::Delegated(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("Delegated");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
            (&RawEvent::Undelegated(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Undelegated");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <Balance, AccountId> From<RawEvent<Balance, AccountId>> for () {
    fn from(_: RawEvent<Balance, AccountId>) -> () { () }
}
impl <Balance, AccountId> RawEvent<Balance, AccountId> {
    #[allow(dead_code)]
    pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
        &[::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Proposed"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["PropIndex",
                                                                                                    "Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Tabled"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["PropIndex",
                                                                                                    "Balance",
                                                                                                    "Vec<AccountId>"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Started"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["ReferendumIndex",
                                                                                                    "VoteThreshold"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Passed"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["ReferendumIndex"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("NotPassed"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["ReferendumIndex"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Cancelled"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["ReferendumIndex"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Executed"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["ReferendumIndex",
                                                                                                    "bool"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Delegated"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "AccountId"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Undelegated"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[]),}]
    }
}
impl <T: Trait> Module<T> {
    /// Get the amount locked in support of `proposal`; `None` if proposal isn't a valid proposal
    /// index.
    pub fn locked_for(proposal: PropIndex) -> Option<BalanceOf<T>> {
        Self::deposit_of(proposal).map(|(d, l)|
                                           d *
                                               BalanceOf::<T>::sa(l.len() as
                                                                      u64))
    }
    /// Return true if `ref_index` is an on-going referendum.
    pub fn is_active_referendum(ref_index: ReferendumIndex) -> bool {
        <ReferendumInfoOf<T>>::exists(ref_index)
    }
    /// Get all referenda currently active.
    pub fn active_referenda()
     -> Vec<(ReferendumIndex, ReferendumInfo<T::BlockNumber, T::Proposal>)> {
        let next = Self::next_tally();
        let last = Self::referendum_count();
        (next..last).into_iter().filter_map(|i|
                                                Self::referendum_info(i).map(|info|
                                                                                 (i,
                                                                                  info))).collect()
    }
    /// Get all referenda ready for tally at block `n`.
    pub fn maturing_referenda_at(n: T::BlockNumber)
     -> Vec<(ReferendumIndex, ReferendumInfo<T::BlockNumber, T::Proposal>)> {
        let next = Self::next_tally();
        let last = Self::referendum_count();
        (next..last).into_iter().filter_map(|i|
                                                Self::referendum_info(i).map(|info|
                                                                                 (i,
                                                                                  info))).take_while(|&(_,
                                                                                                        ref info)|
                                                                                                         info.end
                                                                                                             ==
                                                                                                             n).collect()
    }
    /// Get the voters for the current proposal.
    pub fn tally(ref_index: ReferendumIndex)
     -> (BalanceOf<T>, BalanceOf<T>, BalanceOf<T>) {
        let (approve, against, capital):
                (BalanceOf<T>, BalanceOf<T>, BalanceOf<T>) =
            Self::voters_for(ref_index).iter().map(|voter|
                                                       (T::Currency::total_balance(voter),
                                                        Self::vote_of((ref_index,
                                                                       voter.clone())))).map(|(bal,
                                                                                               vote)|
                                                                                                 if vote.is_aye()
                                                                                                    {
                                                                                                     (bal
                                                                                                          *
                                                                                                          BalanceOf::<T>::sa(vote.multiplier()
                                                                                                                                 as
                                                                                                                                 u64),
                                                                                                      Zero::zero(),
                                                                                                      bal)
                                                                                                 } else {
                                                                                                     (Zero::zero(),
                                                                                                      bal
                                                                                                          *
                                                                                                          BalanceOf::<T>::sa(vote.multiplier()
                                                                                                                                 as
                                                                                                                                 u64),
                                                                                                      bal)
                                                                                                 }).fold((Zero::zero(),
                                                                                                          Zero::zero(),
                                                                                                          Zero::zero()),
                                                                                                         |(a,
                                                                                                           b,
                                                                                                           c),
                                                                                                          (d,
                                                                                                           e,
                                                                                                           f)|
                                                                                                             (a
                                                                                                                  +
                                                                                                                  d,
                                                                                                              b
                                                                                                                  +
                                                                                                                  e,
                                                                                                              c
                                                                                                                  +
                                                                                                                  f));
        let (del_approve, del_against, del_capital) =
            Self::tally_delegation(ref_index);
        (approve + del_approve, against + del_against, capital + del_capital)
    }
    /// Get the delegated voters for the current proposal.
    /// I think this goes into a worker once https://github.com/paritytech/substrate/issues/1458 is done.
    fn tally_delegation(ref_index: ReferendumIndex)
     -> (BalanceOf<T>, BalanceOf<T>, BalanceOf<T>) {
        Self::voters_for(ref_index).iter().fold((Zero::zero(), Zero::zero(),
                                                 Zero::zero()),
                                                |(approve_acc, against_acc,
                                                  capital_acc), voter|
                                                    {
                                                        let vote =
                                                            Self::vote_of((ref_index,
                                                                           voter.clone()));
                                                        let (votes, balance) =
                                                            Self::delegated_votes(ref_index,
                                                                                  voter.clone(),
                                                                                  vote.multiplier(),
                                                                                  MAX_RECURSION_LIMIT);
                                                        if vote.is_aye() {
                                                            (approve_acc +
                                                                 votes,
                                                             against_acc,
                                                             capital_acc +
                                                                 balance)
                                                        } else {
                                                            (approve_acc,
                                                             against_acc +
                                                                 votes,
                                                             capital_acc +
                                                                 balance)
                                                        }
                                                    })
    }
    fn delegated_votes(ref_index: ReferendumIndex, to: T::AccountId,
                       min_lock_periods: LockPeriods, recursion_limit: u32)
     -> (BalanceOf<T>, BalanceOf<T>) {
        if recursion_limit == 0 { return (Zero::zero(), Zero::zero()); }
        <Delegations<T>>::enumerate().filter(|(delegator, (delegate, _))|
                                                 *delegate == to &&
                                                     !<VoteOf<T>>::exists(&(ref_index,
                                                                            delegator.clone()))).fold((Zero::zero(),
                                                                                                       Zero::zero()),
                                                                                                      |(votes_acc,
                                                                                                        balance_acc),
                                                                                                       (delegator,
                                                                                                        (_delegate,
                                                                                                         periods))|
                                                                                                          {
                                                                                                              let lock_periods =
                                                                                                                  if min_lock_periods
                                                                                                                         <=
                                                                                                                         periods
                                                                                                                     {
                                                                                                                      min_lock_periods
                                                                                                                  } else {
                                                                                                                      periods
                                                                                                                  };
                                                                                                              let balance =
                                                                                                                  T::Currency::total_balance(&delegator);
                                                                                                              let votes =
                                                                                                                  T::Currency::total_balance(&delegator)
                                                                                                                      *
                                                                                                                      BalanceOf::<T>::sa(lock_periods
                                                                                                                                             as
                                                                                                                                             u64);
                                                                                                              let (del_votes,
                                                                                                                   del_balance) =
                                                                                                                  Self::delegated_votes(ref_index,
                                                                                                                                        delegator,
                                                                                                                                        lock_periods,
                                                                                                                                        recursion_limit
                                                                                                                                            -
                                                                                                                                            1);
                                                                                                              (votes_acc
                                                                                                                   +
                                                                                                                   votes
                                                                                                                   +
                                                                                                                   del_votes,
                                                                                                               balance_acc
                                                                                                                   +
                                                                                                                   balance
                                                                                                                   +
                                                                                                                   del_balance)
                                                                                                          })
    }
    #[cfg(feature = "std")]
    pub fn force_proxy(stash: T::AccountId, proxy: T::AccountId) {
        <Proxy<T>>::insert(proxy, stash)
    }
    /// Start a referendum. Can be called directly by the council.
    pub fn internal_start_referendum(proposal: T::Proposal,
                                     threshold: VoteThreshold,
                                     delay: T::BlockNumber)
     -> result::Result<ReferendumIndex, &'static str> {
        <Module<T>>::inject_referendum(<system::Module<T>>::block_number() +
                                           <Module<T>>::voting_period(),
                                       proposal, threshold, delay)
    }
    /// Remove a referendum. Can be called directly by the council.
    pub fn internal_cancel_referendum(ref_index: ReferendumIndex) {
        Self::deposit_event(RawEvent::Cancelled(ref_index));
        <Module<T>>::clear_referendum(ref_index);
    }
    /// Actually enact a vote, if legit.
    fn do_vote(who: T::AccountId, ref_index: ReferendumIndex, vote: Vote)
     -> Result {
        {
            if !(vote.multiplier() <= Self::max_lock_periods()) {
                { return Err("vote has too great a strength"); };
            }
        };
        {
            if !Self::is_active_referendum(ref_index) {
                { return Err("vote given for invalid referendum."); };
            }
        };
        if !<VoteOf<T>>::exists(&(ref_index, who.clone())) {
            <VotersFor<T>>::mutate(ref_index,
                                   |voters| voters.push(who.clone()));
        }
        <VoteOf<T>>::insert(&(ref_index, who), vote);
        Ok(())
    }
    /// Start a referendum
    fn inject_referendum(end: T::BlockNumber, proposal: T::Proposal,
                         threshold: VoteThreshold, delay: T::BlockNumber)
     -> result::Result<ReferendumIndex, &'static str> {
        let ref_index = Self::referendum_count();
        if ref_index > 0 &&
               Self::referendum_info(ref_index -
                                         1).map(|i|
                                                    i.end >
                                                        end).unwrap_or(false)
           {
            Err("Cannot inject a referendum that ends earlier than preceeding referendum")?
        }
        <ReferendumCount<T>>::put(ref_index + 1);
        <ReferendumInfoOf<T>>::insert(ref_index,
                                      ReferendumInfo{end,
                                                     proposal,
                                                     threshold,
                                                     delay,});
        Self::deposit_event(RawEvent::Started(ref_index, threshold));
        Ok(ref_index)
    }
    /// Remove all info on a referendum.
    fn clear_referendum(ref_index: ReferendumIndex) {
        <ReferendumInfoOf<T>>::remove(ref_index);
        <VotersFor<T>>::remove(ref_index);
        for v in Self::voters_for(ref_index) {
            <VoteOf<T>>::remove((ref_index, v));
        }
    }
    /// Enact a proposal from a referendum.
    fn enact_proposal(proposal: T::Proposal, index: ReferendumIndex) {
        let ok = proposal.dispatch(system::RawOrigin::Root.into()).is_ok();
        Self::deposit_event(RawEvent::Executed(index, ok));
    }
    fn launch_next(now: T::BlockNumber) -> Result {
        let mut public_props = Self::public_props();
        if let Some((winner_index, _)) =
               public_props.iter().enumerate().max_by_key(|x|
                                                              Self::locked_for((x.1).0).unwrap_or_else(Zero::zero))
               {
            let (prop_index, proposal, _) =
                public_props.swap_remove(winner_index);
            <PublicProps<T>>::put(public_props);
            if let Some((deposit, depositors)) =
                   <DepositOf<T>>::take(prop_index) {
                for d in &depositors { T::Currency::unreserve(d, deposit); }
                Self::deposit_event(RawEvent::Tabled(prop_index, deposit,
                                                     depositors));
                Self::inject_referendum(now + Self::voting_period(), proposal,
                                        VoteThreshold::SuperMajorityApprove,
                                        Self::public_delay())?;
            }
        }
        Ok(())
    }
    fn bake_referendum(now: T::BlockNumber, index: ReferendumIndex,
                       info: ReferendumInfo<T::BlockNumber, T::Proposal>)
     -> Result {
        let (approve, against, capital) = Self::tally(index);
        let total_issuance = T::Currency::total_issuance();
        let approved =
            info.threshold.approved(approve, against, capital,
                                    total_issuance);
        let lock_period = Self::public_delay();
        for (a, vote) in
            Self::voters_for(index).into_iter().map(|a|
                                                        (a.clone(),
                                                         Self::vote_of((index,
                                                                        a)))).filter(|&(_,
                                                                                        vote)|
                                                                                         vote.is_aye()
                                                                                             ==
                                                                                             approved)
            {
            let locked_until =
                now +
                    lock_period *
                        T::BlockNumber::sa((vote.multiplier()) as u64);
            T::Currency::extend_lock(DEMOCRACY_ID, &a, Bounded::max_value(),
                                     locked_until,
                                     WithdrawReason::Transfer.into());
        }
        Self::clear_referendum(index);
        if approved {
            Self::deposit_event(RawEvent::Passed(index));
            if info.delay.is_zero() {
                Self::enact_proposal(info.proposal, index);
            } else {
                <DispatchQueue<T>>::mutate(now + info.delay,
                                           |q|
                                               q.push(Some((info.proposal,
                                                            index))));
            }
        } else { Self::deposit_event(RawEvent::NotPassed(index)); }
        <NextTally<T>>::put(index + 1);
        Ok(())
    }
    /// Current era is ending; we should finish up any proposals.
    fn end_block(now: T::BlockNumber) -> Result {
        if (now % Self::launch_period()).is_zero() {
            Self::launch_next(now.clone())?;
        }
        for (index, info) in Self::maturing_referenda_at(now).into_iter() {
            Self::bake_referendum(now.clone(), index, info)?;
        }
        for (proposal, index) in
            <DispatchQueue<T>>::take(now).into_iter().filter_map(|x| x) {
            Self::enact_proposal(proposal, index);
        }
        Ok(())
    }
}
impl <T: Trait> OnFreeBalanceZero<T::AccountId> for Module<T> {
    fn on_free_balance_zero(who: &T::AccountId) { <Proxy<T>>::remove(who) }
}
