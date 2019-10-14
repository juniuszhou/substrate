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

//! # Balances Module
//!
//! The Balances module provides functionality for handling accounts and balances.
//!
//! - [`balances::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! The Balances module provides functions for:
//!
//! - Getting and setting free balances.
//! - Retrieving total, reserved and unreserved balances.
//! - Repatriating a reserved balance to a beneficiary account that exists.
//! - Transferring a balance between accounts (when not reserved).
//! - Slashing an account balance.
//! - Account creation and removal.
//! - Managing total issuance.
//! - Setting and managing locks.
//!
//! ### Terminology
//!
//! - **Existential Deposit:** The minimum balance required to create or keep an account open. This prevents
//! "dust accounts" from filling storage.
//! - **Total Issuance:** The total number of units in existence in a system.
//! - **Reaping an account:** The act of removing an account by resetting its nonce. Happens after its balance is set
//! to zero.
//! - **Free Balance:** The portion of a balance that is not reserved. The free balance is the only balance that matters
//! for most operations. When this balance falls below the existential deposit, most functionality of the account is
//! removed. When both it and the reserved balance are deleted, then the account is said to be dead.
//! - **Reserved Balance:** Reserved balance still belongs to the account holder, but is suspended. Reserved balance
//! can still be slashed, but only after all the free balance has been slashed. If the reserved balance falls below the
//! existential deposit then it and any related functionality will be deleted. When both it and the free balance are
//! deleted, then the account is said to be dead.
//! - **Imbalance:** A condition when some funds were credited or debited without equal and opposite accounting
//! (i.e. a difference between total issuance and account balances). Functions that result in an imbalance will
//! return an object of the `Imbalance` trait that must be handled.
//! - **Lock:** A freeze on a specified amount of an account's free balance until a specified block number. Multiple
//! locks always operate over the same funds, so they "overlay" rather than "stack".
//! - **Vesting:** Similar to a lock, this is another, but independent, liquidity restriction that reduces linearly
//! over time.
//!
//! ### Implementations
//!
//! The Balances module provides implementations for the following traits. If these traits provide the functionality
//! that you need, then you can avoid coupling with the Balances module.
//!
//! - [`Currency`](../srml_support/traits/trait.Currency.html): Functions for dealing with a
//! fungible assets system.
//! - [`ReservableCurrency`](../srml_support/traits/trait.ReservableCurrency.html):
//! Functions for dealing with assets that can be reserved from an account.
//! - [`LockableCurrency`](../srml_support/traits/trait.LockableCurrency.html): Functions for
//! dealing with accounts that allow liquidity restrictions.
//! - [`Imbalance`](../srml_support/traits/trait.Imbalance.html): Functions for handling
//! imbalances between total issuance in the system and account balances. Must be used when a function
//! creates new funds (e.g. a reward) or destroys some funds (e.g. a system fee).
//! - [`MakePayment`](../srml_support/traits/trait.MakePayment.html): Simple trait designed
//! for hooking into a transaction payment.
//! - [`IsDeadAccount`](../srml_system/trait.IsDeadAccount.html): Determiner to say whether a
//! given account is unused.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `transfer` - Transfer some liquid free balance to another account.
//! - `set_balance` - Set the balances of a given account. The origin of this call must be root.
//!
//! ### Public Functions
//!
//! - `vesting_balance` - Get the amount that is currently being vested and cannot be transferred out of this account.
//!
//! ## Usage
//!
//! The following examples show how to use the Balances module in your custom module.
//!
//! ### Examples from the SRML
//!
//! The Contract module uses the `Currency` trait to handle gas payment, and its types inherit from `Currency`:
//!
//! ```
//! use srml_support::traits::Currency;
//! # pub trait Trait: system::Trait {
//! # 	type Currency: Currency<Self::AccountId>;
//! # }
//!
//! pub type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
//! pub type NegativeImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;
//!
//! # fn main() {}
//! ```
//!
//! The Staking module uses the `LockableCurrency` trait to lock a stash account's funds:
//!
//! ```
//! use srml_support::traits::{WithdrawReasons, LockableCurrency};
//! use primitives::traits::Bounded;
//! pub trait Trait: system::Trait {
//! 	type Currency: LockableCurrency<Self::AccountId, Moment=Self::BlockNumber>;
//! }
//! # struct StakingLedger<T: Trait> {
//! # 	stash: <T as system::Trait>::AccountId,
//! # 	total: <<T as Trait>::Currency as srml_support::traits::Currency<<T as system::Trait>::AccountId>>::Balance,
//! # 	phantom: std::marker::PhantomData<T>,
//! # }
//! # const STAKING_ID: [u8; 8] = *b"staking ";
//!
//! fn update_ledger<T: Trait>(
//! 	controller: &T::AccountId,
//! 	ledger: &StakingLedger<T>
//! ) {
//! 	T::Currency::set_lock(
//! 		STAKING_ID,
//! 		&ledger.stash,
//! 		ledger.total,
//! 		T::BlockNumber::max_value(),
//! 		WithdrawReasons::all()
//! 	);
//! 	// <Ledger<T>>::insert(controller, ledger); // Commented out as we don't have access to Staking's storage here.
//! }
//! # fn main() {}
//! ```
//!
//! ## Genesis config
//!
//! The Balances module depends on the [`GenesisConfig`](./struct.GenesisConfig.html).
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


use rstd::prelude::*;
use rstd::{cmp, result};
use parity_codec::{Codec, Encode, Decode};
use srml_support::{StorageValue, StorageMap, Parameter, decl_event,
                   decl_storage, decl_module};
use srml_support::traits::{UpdateBalanceOutcome, Currency, OnFreeBalanceZero,
                           MakePayment, OnUnbalanced, WithdrawReason,
                           WithdrawReasons, LockIdentifier, LockableCurrency,
                           ExistenceRequirement, Imbalance, SignedImbalance,
                           ReservableCurrency};
use srml_support::dispatch::Result;
use primitives::traits::{Zero, SimpleArithmetic, As, StaticLookup, Member,
                         CheckedAdd, CheckedSub, MaybeSerializeDebug,
                         Saturating};
use system::{IsDeadAccount, OnNewAccount, ensure_signed};


pub use self::imbalances::{PositiveImbalance, NegativeImbalance};

pub trait Subtrait<I: Instance = DefaultInstance>: system::Trait {
    /// The balance of an account.
    type
    Balance: Parameter +
    Member +
    SimpleArithmetic +
    Codec +
    Default +
    Copy +
    MaybeSerializeDebug;

    /// A function that is invoked when the free-balance has fallen below the existential deposit and
    /// has been reduced to zero.
    ///
    /// Gives a chance to clean up resources associated with the given account.
    type
    OnFreeBalanceZero: OnFreeBalanceZero<Self::AccountId>;

    /// Handler for when a new account is created.
    type
    OnNewAccount: OnNewAccount<Self::AccountId>;
}

pub trait Trait<I: Instance = DefaultInstance>: system::Trait {
    /// The balance of an account.
    type
    Balance: Parameter +
    Member +
    SimpleArithmetic +
    Codec +
    Default +
    Copy +
    MaybeSerializeDebug;

    /// A function that is invoked when the free-balance has fallen below the existential deposit and
    /// has been reduced to zero.
    ///
    /// Gives a chance to clean up resources associated with the given account.
    type
    OnFreeBalanceZero: OnFreeBalanceZero<Self::AccountId>;

    /// Handler for when a new account is created.
    type
    OnNewAccount: OnNewAccount<Self::AccountId>;

    /// Handler for the unbalanced reduction when taking transaction fees.
    type
    TransactionPayment: OnUnbalanced<NegativeImbalance<Self, I>>;

    /// Handler for the unbalanced reduction when taking fees associated with balance
    /// transfer (which may also include account creation).
    type
    TransferPayment: OnUnbalanced<NegativeImbalance<Self, I>>;

    /// Handler for the unbalanced reduction when removing a dust account.
    type
    DustRemoval: OnUnbalanced<NegativeImbalance<Self, I>>;

    /// The overarching event type.
    type
    Event: From<Event<Self, I>> +
    Into<<Self as system::Trait>::Event>;
}

impl <T: Trait<I>, I: Instance> Subtrait<I> for T {
    type
    Balance
    =
    T::Balance;
    type
    OnFreeBalanceZero
    =
    T::OnFreeBalanceZero;
    type
    OnNewAccount
    =
    T::OnNewAccount;
}







// <= begin it should be >= balance
// >= begin+length it should be <= 0





// begin, length





// PUBLIC IMMUTABLES


// PRIVATE MUTABLES


// Commented out for now - but consider it instructive.
// assert!(!Self::total_balance(who).is_zero());
// assert!(Self::free_balance(who) > Self::existential_deposit());




// underflow should never happen, but if it does, there's not much we can do about it.




// underflow should never happen, but it if does, there's nothing to be done here.


// wrapping these imbalances in a private module is necessary to ensure absolute privacy
// of the inner member.

















// TODO: #2052
// Somewhat ugly hack in order to gain access to module's `increase_total_issuance_by`
// using only the Subtrait (which defines only the types that are not dependent
// on Positive/NegativeImbalance). Subtrait must be used otherwise we end up with a
// circular dependency with Trait having some types be dependent on PositiveImbalance<Trait>
// and PositiveImbalance itself depending back on Trait for its Drop impl (and thus
// its type declaration).
// This works as long as `increase_total_issuance_by` doesn't use the Imbalance
// types (basically for charging fees).
// This should eventually be refactored so that the three type items that do
// depend on the Imbalance type (TransactionPayment, TransferPayment, DustRemoval)
// are placed in their own SRML module.











// NOTE: total stake being stored in the same type means that this could never overflow
// but better to be safe than sorry.




// NOTE: `slash()` prefers free balance, but assumes that reserve balance can be drawn
// from in extreme circumstances. `can_slash()` should be used prior to `slash()` to avoid having
// to draw from reserved funds, however we err on the side of punishment if things are inconsistent
// or `can_slash` wasn't used appropriately.


// Impossible, but be defensive.

// If we're attempting to set an existing account to less than ED, then
// bypass the entire operation. It's a no-op if you follow it through, but
// since this is an instance where we might account for a negative imbalance
// (in the dust cleaner of set_free_balance) before we account for its actual
// equal and opposite cause (returned as an Imbalance), then in the
// instance that there's no other accounts on the system at all, we might
// underflow the issuance and our arithmetic will be off.
// If the balance is too low, then the account is reaped.
// NOTE: There are two balances for every account: `reserved_balance` and
// `free_balance`. This contract subsystem only cares about the latter: whenever
// the term "balance" is used *here* it should be assumed to mean "free balance"
// in the rest of the module.
// Free balance can never be less than ED. If that happens, it gets reduced to zero
// and the account information relevant to this subsystem is deleted (i.e. the
// account is reaped).





// underflow should never happen, but it if does, there's nothing to be done here.







/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T, I = DefaultInstance>
    =
    RawEvent<<T as system::Trait>::AccountId, <T as Trait<I>>::Balance, I>;
/// Events for this module.
///
#[structural_match]
pub enum RawEvent<AccountId, Balance, I> {

    #[doc = r" A new account was created."]
    NewAccount(AccountId, Balance),

    #[doc = r" An account was reaped."]
    ReapedAccount(AccountId),

    #[doc = r" Transfer succeeded (from, to, value, fees)."]
    Transfer(AccountId, AccountId, Balance, Balance),

    #[doc(hidden)]
    #[codec(skip)]
    PhantomData(::srml_support::rstd::marker::PhantomData<I>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::clone::Clone, Balance: ::std::clone::Clone,
      I: ::std::clone::Clone> ::std::clone::Clone for
 RawEvent<AccountId, Balance, I> {
    #[inline]
    fn clone(&self) -> RawEvent<AccountId, Balance, I> {
        match (&*self,) {
            (&RawEvent::NewAccount(ref __self_0, ref __self_1),) =>
            RawEvent::NewAccount(::std::clone::Clone::clone(&(*__self_0)),
                                 ::std::clone::Clone::clone(&(*__self_1))),
            (&RawEvent::ReapedAccount(ref __self_0),) =>
            RawEvent::ReapedAccount(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::Transfer(ref __self_0, ref __self_1, ref __self_2,
                                 ref __self_3),) =>
            RawEvent::Transfer(::std::clone::Clone::clone(&(*__self_0)),
                               ::std::clone::Clone::clone(&(*__self_1)),
                               ::std::clone::Clone::clone(&(*__self_2)),
                               ::std::clone::Clone::clone(&(*__self_3))),
            (&RawEvent::PhantomData(ref __self_0),) =>
            RawEvent::PhantomData(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialEq, Balance: ::std::cmp::PartialEq,
      I: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
 RawEvent<AccountId, Balance, I> {
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId, Balance, I>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::NewAccount(ref __self_0, ref __self_1),
                     &RawEvent::NewAccount(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    (&RawEvent::ReapedAccount(ref __self_0),
                     &RawEvent::ReapedAccount(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::Transfer(ref __self_0, ref __self_1,
                                         ref __self_2, ref __self_3),
                     &RawEvent::Transfer(ref __arg_1_0, ref __arg_1_1,
                                         ref __arg_1_2, ref __arg_1_3)) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2) &&
                        (*__self_3) == (*__arg_1_3),
                    (&RawEvent::PhantomData(ref __self_0),
                     &RawEvent::PhantomData(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId, Balance, I>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::NewAccount(ref __self_0, ref __self_1),
                     &RawEvent::NewAccount(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    (&RawEvent::ReapedAccount(ref __self_0),
                     &RawEvent::ReapedAccount(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::Transfer(ref __self_0, ref __self_1,
                                         ref __self_2, ref __self_3),
                     &RawEvent::Transfer(ref __arg_1_0, ref __arg_1_1,
                                         ref __arg_1_2, ref __arg_1_3)) =>
                    (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1)
                        || (*__self_2) != (*__arg_1_2) ||
                        (*__self_3) != (*__arg_1_3),
                    (&RawEvent::PhantomData(ref __self_0),
                     &RawEvent::PhantomData(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::Eq, Balance: ::std::cmp::Eq, I: ::std::cmp::Eq>
 ::std::cmp::Eq for RawEvent<AccountId, Balance, I> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::rstd::marker::PhantomData<I>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawEvent: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, Balance, I> _parity_codec::Encode for
         RawEvent<AccountId, Balance, I> where
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         Balance: _parity_codec::Encode, Balance: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         Balance: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode, Balance: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::NewAccount(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::ReapedAccount(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::Transfer(ref aa, ref ba, ref ca, ref da) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                        dest.push(da);
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
        impl <AccountId, Balance, I> _parity_codec::Decode for
         RawEvent<AccountId, Balance, I> where
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         Balance: _parity_codec::Decode, Balance: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         Balance: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode, Balance: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::NewAccount(_parity_codec::Decode::decode(input)?,
                                                  _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(RawEvent::ReapedAccount(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(RawEvent::Transfer(_parity_codec::Decode::decode(input)?,
                                                _parity_codec::Decode::decode(input)?,
                                                _parity_codec::Decode::decode(input)?,
                                                _parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::fmt::Debug, Balance: ::std::fmt::Debug,
      I: ::std::fmt::Debug> ::std::fmt::Debug for
 RawEvent<AccountId, Balance, I> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawEvent::NewAccount(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("NewAccount");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
            (&RawEvent::ReapedAccount(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("ReapedAccount");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::Transfer(ref __self_0, ref __self_1, ref __self_2,
                                 ref __self_3),) => {
                let mut debug_trait_builder = f.debug_tuple("Transfer");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                let _ = debug_trait_builder.field(&&(*__self_3));
                debug_trait_builder.finish()
            }
            (&RawEvent::PhantomData(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("PhantomData");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <AccountId, Balance, I> From<RawEvent<AccountId, Balance, I>> for () {
    fn from(_: RawEvent<AccountId, Balance, I>) -> () { () }
}
impl <AccountId, Balance, I> RawEvent<AccountId, Balance, I> {
    #[allow(dead_code)]
    pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
        &[::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("NewAccount"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" A new account was created."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("ReapedAccount"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" An account was reaped."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Transfer"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "AccountId",
                                                                                                    "Balance",
                                                                                                    "Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Transfer succeeded (from, to, value, fees)."]),}]
    }
}
/// Struct to encode the vesting schedule of an individual account.
#[structural_match]
#[rustc_copy_clone_marker]
pub struct VestingSchedule<Balance> {
    /// Locked amount at genesis.
    pub offset: Balance,
    /// Amount that gets unlocked every block from genesis.
    pub per_block: Balance,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_VestingSchedule: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance> _parity_codec::Encode for VestingSchedule<Balance>
         where Balance: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode, Balance: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.offset);
                dest.push(&self.per_block);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_VestingSchedule: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance> _parity_codec::Decode for VestingSchedule<Balance>
         where Balance: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode, Balance: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(VestingSchedule{offset:
                                         _parity_codec::Decode::decode(input)?,
                                     per_block:
                                         _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::marker::Copy> ::std::marker::Copy for
 VestingSchedule<Balance> {
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::clone::Clone> ::std::clone::Clone for
 VestingSchedule<Balance> {
    #[inline]
    fn clone(&self) -> VestingSchedule<Balance> {
        match *self {
            VestingSchedule {
            offset: ref __self_0_0, per_block: ref __self_0_1 } =>
            VestingSchedule{offset:
                                ::std::clone::Clone::clone(&(*__self_0_0)),
                            per_block:
                                ::std::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
 VestingSchedule<Balance> {
    #[inline]
    fn eq(&self, other: &VestingSchedule<Balance>) -> bool {
        match *other {
            VestingSchedule {
            offset: ref __self_1_0, per_block: ref __self_1_1 } =>
            match *self {
                VestingSchedule {
                offset: ref __self_0_0, per_block: ref __self_0_1 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &VestingSchedule<Balance>) -> bool {
        match *other {
            VestingSchedule {
            offset: ref __self_1_0, per_block: ref __self_1_1 } =>
            match *self {
                VestingSchedule {
                offset: ref __self_0_0, per_block: ref __self_0_1 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::Eq> ::std::cmp::Eq for VestingSchedule<Balance> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::fmt::Debug> ::std::fmt::Debug for
 VestingSchedule<Balance> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VestingSchedule {
            offset: ref __self_0_0, per_block: ref __self_0_1 } => {
                let mut debug_trait_builder =
                    f.debug_struct("VestingSchedule");
                let _ = debug_trait_builder.field("offset", &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("per_block", &&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <Balance: SimpleArithmetic + Copy + As<u64>> VestingSchedule<Balance> {
    /// Amount locked at block `n`.
    pub fn locked_at<BlockNumber: As<u64>>(&self, n: BlockNumber) -> Balance {
        if let Some(x) = Balance::sa(n.as_()).checked_mul(&self.per_block) {
            self.offset.max(x) - x
        } else { Zero::zero() }
    }
}
#[structural_match]
pub struct BalanceLock<Balance, BlockNumber> {
    pub id: LockIdentifier,
    pub amount: Balance,
    pub until: BlockNumber,
    pub reasons: WithdrawReasons,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_BalanceLock: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance, BlockNumber> _parity_codec::Encode for
         BalanceLock<Balance, BlockNumber> where
         Balance: _parity_codec::Encode, Balance: _parity_codec::Encode,
         BlockNumber: _parity_codec::Encode,
         BlockNumber: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.id);
                dest.push(&self.amount);
                dest.push(&self.until);
                dest.push(&self.reasons);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_BalanceLock: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance, BlockNumber> _parity_codec::Decode for
         BalanceLock<Balance, BlockNumber> where
         Balance: _parity_codec::Decode, Balance: _parity_codec::Decode,
         BlockNumber: _parity_codec::Decode,
         BlockNumber: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(BalanceLock{id: _parity_codec::Decode::decode(input)?,
                                 amount:
                                     _parity_codec::Decode::decode(input)?,
                                 until: _parity_codec::Decode::decode(input)?,
                                 reasons:
                                     _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::clone::Clone, BlockNumber: ::std::clone::Clone>
 ::std::clone::Clone for BalanceLock<Balance, BlockNumber> {
    #[inline]
    fn clone(&self) -> BalanceLock<Balance, BlockNumber> {
        match *self {
            BalanceLock {
            id: ref __self_0_0,
            amount: ref __self_0_1,
            until: ref __self_0_2,
            reasons: ref __self_0_3 } =>
            BalanceLock{id: ::std::clone::Clone::clone(&(*__self_0_0)),
                        amount: ::std::clone::Clone::clone(&(*__self_0_1)),
                        until: ::std::clone::Clone::clone(&(*__self_0_2)),
                        reasons: ::std::clone::Clone::clone(&(*__self_0_3)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::PartialEq, BlockNumber: ::std::cmp::PartialEq>
 ::std::cmp::PartialEq for BalanceLock<Balance, BlockNumber> {
    #[inline]
    fn eq(&self, other: &BalanceLock<Balance, BlockNumber>) -> bool {
        match *other {
            BalanceLock {
            id: ref __self_1_0,
            amount: ref __self_1_1,
            until: ref __self_1_2,
            reasons: ref __self_1_3 } =>
            match *self {
                BalanceLock {
                id: ref __self_0_0,
                amount: ref __self_0_1,
                until: ref __self_0_2,
                reasons: ref __self_0_3 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &BalanceLock<Balance, BlockNumber>) -> bool {
        match *other {
            BalanceLock {
            id: ref __self_1_0,
            amount: ref __self_1_1,
            until: ref __self_1_2,
            reasons: ref __self_1_3 } =>
            match *self {
                BalanceLock {
                id: ref __self_0_0,
                amount: ref __self_0_1,
                until: ref __self_0_2,
                reasons: ref __self_0_3 } =>
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
impl <Balance: ::std::cmp::Eq, BlockNumber: ::std::cmp::Eq> ::std::cmp::Eq for
 BalanceLock<Balance, BlockNumber> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<LockIdentifier>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<BlockNumber>;
            let _: ::std::cmp::AssertParamIsEq<WithdrawReasons>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::fmt::Debug, BlockNumber: ::std::fmt::Debug>
 ::std::fmt::Debug for BalanceLock<Balance, BlockNumber> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            BalanceLock {
            id: ref __self_0_0,
            amount: ref __self_0_1,
            until: ref __self_0_2,
            reasons: ref __self_0_3 } => {
                let mut debug_trait_builder = f.debug_struct("BalanceLock");
                let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                let _ = debug_trait_builder.field("amount", &&(*__self_0_1));
                let _ = debug_trait_builder.field("until", &&(*__self_0_2));
                let _ = debug_trait_builder.field("reasons", &&(*__self_0_3));
                debug_trait_builder.finish()
            }
        }
    }
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = r" Tag a type as an instance of a module."]
#[doc = r""]
#[doc = r" Defines storage prefixes, they must be unique."]
pub trait Instance: 'static {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str;
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str;
    const
    PREFIX_FOR_TransferFee:
    &'static str;
    const
    PREFIX_FOR_CreationFee:
    &'static str;
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str;
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str;
    const
    PREFIX_FOR_Vesting:
    &'static str;
    const
    PREFIX_FOR_FreeBalance:
    &'static str;
    const
    PREFIX_FOR_ReservedBalance:
    &'static str;
    const
    PREFIX_FOR_Locks:
    &'static str;
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance0;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance0 => {
                let mut debug_trait_builder = f.debug_tuple("Instance0");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance0 {
    #[inline]
    fn clone(&self) -> Instance0 { match *self { Instance0 => Instance0, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance0 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance0 {
    #[inline]
    fn eq(&self, other: &Instance0) -> bool {
        match *other { Instance0 => match *self { Instance0 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance0: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance0 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance0: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance0 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance0)
            }
        }
    };
impl Instance for Instance0 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance0";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance0";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance0";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance0";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance0";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance0";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance0";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance0";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance0";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance0";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance1;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance1 => {
                let mut debug_trait_builder = f.debug_tuple("Instance1");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance1 {
    #[inline]
    fn clone(&self) -> Instance1 { match *self { Instance1 => Instance1, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance1 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance1 {
    #[inline]
    fn eq(&self, other: &Instance1) -> bool {
        match *other { Instance1 => match *self { Instance1 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance1: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance1 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance1: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance1 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance1)
            }
        }
    };
impl Instance for Instance1 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance1";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance1";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance1";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance1";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance1";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance1";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance1";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance1";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance1";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance1";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance2;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance2 => {
                let mut debug_trait_builder = f.debug_tuple("Instance2");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance2 {
    #[inline]
    fn clone(&self) -> Instance2 { match *self { Instance2 => Instance2, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance2 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance2 {
    #[inline]
    fn eq(&self, other: &Instance2) -> bool {
        match *other { Instance2 => match *self { Instance2 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance2: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance2 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance2: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance2 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance2)
            }
        }
    };
impl Instance for Instance2 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance2";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance2";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance2";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance2";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance2";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance2";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance2";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance2";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance2";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance2";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance3;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance3 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance3 => {
                let mut debug_trait_builder = f.debug_tuple("Instance3");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance3 {
    #[inline]
    fn clone(&self) -> Instance3 { match *self { Instance3 => Instance3, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance3 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance3 {
    #[inline]
    fn eq(&self, other: &Instance3) -> bool {
        match *other { Instance3 => match *self { Instance3 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance3: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance3 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance3: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance3 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance3)
            }
        }
    };
impl Instance for Instance3 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance3";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance3";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance3";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance3";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance3";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance3";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance3";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance3";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance3";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance3";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance4;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance4 => {
                let mut debug_trait_builder = f.debug_tuple("Instance4");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance4 {
    #[inline]
    fn clone(&self) -> Instance4 { match *self { Instance4 => Instance4, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance4 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance4 {
    #[inline]
    fn eq(&self, other: &Instance4) -> bool {
        match *other { Instance4 => match *self { Instance4 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance4: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance4 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance4: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance4 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance4)
            }
        }
    };
impl Instance for Instance4 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance4";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance4";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance4";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance4";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance4";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance4";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance4";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance4";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance4";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance4";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance5;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance5 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance5 => {
                let mut debug_trait_builder = f.debug_tuple("Instance5");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance5 {
    #[inline]
    fn clone(&self) -> Instance5 { match *self { Instance5 => Instance5, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance5 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance5 {
    #[inline]
    fn eq(&self, other: &Instance5) -> bool {
        match *other { Instance5 => match *self { Instance5 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance5: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance5 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance5: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance5 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance5)
            }
        }
    };
impl Instance for Instance5 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance5";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance5";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance5";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance5";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance5";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance5";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance5";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance5";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance5";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance5";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance6;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance6 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance6 => {
                let mut debug_trait_builder = f.debug_tuple("Instance6");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance6 {
    #[inline]
    fn clone(&self) -> Instance6 { match *self { Instance6 => Instance6, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance6 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance6 {
    #[inline]
    fn eq(&self, other: &Instance6) -> bool {
        match *other { Instance6 => match *self { Instance6 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance6: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance6 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance6: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance6 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance6)
            }
        }
    };
impl Instance for Instance6 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance6";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance6";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance6";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance6";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance6";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance6";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance6";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance6";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance6";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance6";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance7;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance7 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance7 => {
                let mut debug_trait_builder = f.debug_tuple("Instance7");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance7 {
    #[inline]
    fn clone(&self) -> Instance7 { match *self { Instance7 => Instance7, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance7 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance7 {
    #[inline]
    fn eq(&self, other: &Instance7) -> bool {
        match *other { Instance7 => match *self { Instance7 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance7: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance7 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance7: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance7 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance7)
            }
        }
    };
impl Instance for Instance7 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance7";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance7";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance7";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance7";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance7";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance7";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance7";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance7";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance7";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance7";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance8;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance8 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance8 => {
                let mut debug_trait_builder = f.debug_tuple("Instance8");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance8 {
    #[inline]
    fn clone(&self) -> Instance8 { match *self { Instance8 => Instance8, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance8 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance8 {
    #[inline]
    fn eq(&self, other: &Instance8) -> bool {
        match *other { Instance8 => match *self { Instance8 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance8: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance8 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance8: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance8 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance8)
            }
        }
    };
impl Instance for Instance8 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance8";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance8";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance8";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance8";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance8";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance8";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance8";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance8";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance8";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance8";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance9;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance9 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance9 => {
                let mut debug_trait_builder = f.debug_tuple("Instance9");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance9 {
    #[inline]
    fn clone(&self) -> Instance9 { match *self { Instance9 => Instance9, } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance9 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance9 {
    #[inline]
    fn eq(&self, other: &Instance9) -> bool {
        match *other { Instance9 => match *self { Instance9 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance9: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance9 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance9: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance9 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance9)
            }
        }
    };
impl Instance for Instance9 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance9";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance9";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance9";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance9";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance9";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance9";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance9";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance9";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance9";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance9";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance10;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance10 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance10 => {
                let mut debug_trait_builder = f.debug_tuple("Instance10");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance10 {
    #[inline]
    fn clone(&self) -> Instance10 {
        match *self { Instance10 => Instance10, }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance10 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance10 {
    #[inline]
    fn eq(&self, other: &Instance10) -> bool {
        match *other { Instance10 => match *self { Instance10 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance10: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance10 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance10: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance10 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance10)
            }
        }
    };
impl Instance for Instance10 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance10";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance10";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance10";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance10";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance10";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance10";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance10";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance10";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance10";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance10";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance11;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance11 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance11 => {
                let mut debug_trait_builder = f.debug_tuple("Instance11");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance11 {
    #[inline]
    fn clone(&self) -> Instance11 {
        match *self { Instance11 => Instance11, }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance11 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance11 {
    #[inline]
    fn eq(&self, other: &Instance11) -> bool {
        match *other { Instance11 => match *self { Instance11 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance11: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance11 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance11: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance11 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance11)
            }
        }
    };
impl Instance for Instance11 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance11";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance11";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance11";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance11";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance11";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance11";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance11";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance11";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance11";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance11";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance12;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance12 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance12 => {
                let mut debug_trait_builder = f.debug_tuple("Instance12");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance12 {
    #[inline]
    fn clone(&self) -> Instance12 {
        match *self { Instance12 => Instance12, }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance12 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance12 {
    #[inline]
    fn eq(&self, other: &Instance12) -> bool {
        match *other { Instance12 => match *self { Instance12 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance12: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance12 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance12: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance12 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance12)
            }
        }
    };
impl Instance for Instance12 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance12";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance12";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance12";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance12";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance12";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance12";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance12";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance12";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance12";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance12";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance13;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance13 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance13 => {
                let mut debug_trait_builder = f.debug_tuple("Instance13");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance13 {
    #[inline]
    fn clone(&self) -> Instance13 {
        match *self { Instance13 => Instance13, }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance13 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance13 {
    #[inline]
    fn eq(&self, other: &Instance13) -> bool {
        match *other { Instance13 => match *self { Instance13 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance13: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance13 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance13: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance13 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance13)
            }
        }
    };
impl Instance for Instance13 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance13";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance13";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance13";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance13";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance13";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance13";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance13";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance13";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance13";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance13";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance14;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance14 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance14 => {
                let mut debug_trait_builder = f.debug_tuple("Instance14");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance14 {
    #[inline]
    fn clone(&self) -> Instance14 {
        match *self { Instance14 => Instance14, }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance14 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance14 {
    #[inline]
    fn eq(&self, other: &Instance14) -> bool {
        match *other { Instance14 => match *self { Instance14 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance14: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance14 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance14: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance14 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance14)
            }
        }
    };
impl Instance for Instance14 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance14";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance14";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance14";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance14";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance14";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance14";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance14";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance14";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance14";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance14";
}
#[doc = r"Module instance"]
#[structural_match]
pub struct Instance15;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Instance15 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Instance15 => {
                let mut debug_trait_builder = f.debug_tuple("Instance15");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Instance15 {
    #[inline]
    fn clone(&self) -> Instance15 {
        match *self { Instance15 => Instance15, }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Instance15 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Instance15 {
    #[inline]
    fn eq(&self, other: &Instance15) -> bool {
        match *other { Instance15 => match *self { Instance15 => true, }, }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Instance15: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Instance15 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Instance15: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Instance15 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(Instance15)
            }
        }
    };
impl Instance for Instance15 {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuanceInstance15";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDepositInstance15";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFeeInstance15";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFeeInstance15";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFeeInstance15";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFeeInstance15";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances VestingInstance15";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalanceInstance15";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalanceInstance15";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances LocksInstance15";
}
#[doc = r"Default module instance"]
#[structural_match]
pub struct DefaultInstance;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for DefaultInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DefaultInstance => {
                let mut debug_trait_builder =
                    f.debug_tuple("DefaultInstance");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for DefaultInstance {
    #[inline]
    fn clone(&self) -> DefaultInstance {
        match *self { DefaultInstance => DefaultInstance, }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for DefaultInstance {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for DefaultInstance {
    #[inline]
    fn eq(&self, other: &DefaultInstance) -> bool {
        match *other {
            DefaultInstance => match *self { DefaultInstance => true, },
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_DefaultInstance: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for DefaultInstance {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                drop(dest);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_DefaultInstance: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for DefaultInstance {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                drop(input);
                Some(DefaultInstance)
            }
        }
    };
impl Instance for DefaultInstance {
    const
    PREFIX_FOR_TotalIssuance:
    &'static str
    =
    "Balances TotalIssuance";
    const
    PREFIX_FOR_ExistentialDeposit:
    &'static str
    =
    "Balances ExistentialDeposit";
    const
    PREFIX_FOR_TransferFee:
    &'static str
    =
    "Balances TransferFee";
    const
    PREFIX_FOR_CreationFee:
    &'static str
    =
    "Balances CreationFee";
    const
    PREFIX_FOR_TransactionBaseFee:
    &'static str
    =
    "Balances TransactionBaseFee";
    const
    PREFIX_FOR_TransactionByteFee:
    &'static str
    =
    "Balances TransactionByteFee";
    const
    PREFIX_FOR_Vesting:
    &'static str
    =
    "Balances Vesting";
    const
    PREFIX_FOR_FreeBalance:
    &'static str
    =
    "Balances FreeBalance";
    const
    PREFIX_FOR_ReservedBalance:
    &'static str
    =
    "Balances ReservedBalance";
    const
    PREFIX_FOR_Locks:
    &'static str
    =
    "Balances Locks";
}
#[doc = " The total units issued in the system."]
pub struct TotalIssuance<T: Trait<I>, I: Instance =
                         DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>
 for TotalIssuance<T, I> {
    type
    Query
    =
    T::Balance;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { I::PREFIX_FOR_TotalIssuance.as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&val,
                                                                                                                                  storage);
        ret
    }
}
#[doc = " The minimum amount required to keep an account open."]
pub struct ExistentialDeposit<T: Trait<I>, I: Instance =
                              DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                     I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>
 for ExistentialDeposit<T, I> {
    type
    Query
    =
    T::Balance;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { I::PREFIX_FOR_ExistentialDeposit.as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&val,
                                                                                                                                  storage);
        ret
    }
}
#[doc = " The fee required to make a transfer."]
pub struct TransferFee<T: Trait<I>, I: Instance =
                       DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                              I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>
 for TransferFee<T, I> {
    type
    Query
    =
    T::Balance;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { I::PREFIX_FOR_TransferFee.as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&val,
                                                                                                                                  storage);
        ret
    }
}
#[doc = " The fee required to create an account."]
pub struct CreationFee<T: Trait<I>, I: Instance =
                       DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                              I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>
 for CreationFee<T, I> {
    type
    Query
    =
    T::Balance;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { I::PREFIX_FOR_CreationFee.as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&val,
                                                                                                                                  storage);
        ret
    }
}
#[doc = " The fee to be paid for making a transaction; the base."]
pub struct TransactionBaseFee<T: Trait<I>, I: Instance =
                              DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                     I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>
 for TransactionBaseFee<T, I> {
    type
    Query
    =
    T::Balance;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { I::PREFIX_FOR_TransactionBaseFee.as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&val,
                                                                                                                                  storage);
        ret
    }
}
#[doc = " The fee to be paid for making a transaction; the per-byte portion."]
pub struct TransactionByteFee<T: Trait<I>, I: Instance =
                              DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                     I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>
 for TransactionByteFee<T, I> {
    type
    Query
    =
    T::Balance;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { I::PREFIX_FOR_TransactionByteFee.as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&val,
                                                                                                                                  storage);
        ret
    }
}
#[doc = " Information regarding the vesting of a given account."]
pub struct Vesting<T: Trait<I>, I: Instance =
                   DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                          I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   VestingSchedule<T::Balance>>
 for Vesting<T, I> {
    type
    Query
    =
    Option<VestingSchedule<T::Balance>>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { I::PREFIX_FOR_Vesting.as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  VestingSchedule<T::Balance>>>::prefix().to_vec();
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
                                                                                                                  VestingSchedule<T::Balance>>>::key_for(key);
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
                                                                                                                  VestingSchedule<T::Balance>>>::key_for(key);
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
                                                                                                                  VestingSchedule<T::Balance>>>::get(key,
                                                                                                                                                     storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  VestingSchedule<T::Balance>>>::insert(key,
                                                                                                                                                        &val,
                                                                                                                                                        storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  VestingSchedule<T::Balance>>>::remove(key,
                                                                                                                                                        storage),
        };
        ret
    }
}
#[doc = " The \'free\' balance of a given account."]
#[doc = ""]
#[doc =
      " This is the only balance that matters in terms of most operations on tokens. It"]
#[doc =
      " alone is used to determine the balance when in the contract execution environment. When this"]
#[doc =
      " balance falls below the value of `ExistentialDeposit`, then the \'current account\' is"]
#[doc =
      " deleted: specifically `FreeBalance`. Further, the `OnFreeBalanceZero` callback"]
#[doc =
      " is invoked, giving a chance to external modules to clean up data associated with"]
#[doc = " the deleted account."]
#[doc = ""]
#[doc =
      " `system::AccountNonce` is also deleted if `ReservedBalance` is also zero (it also gets"]
#[doc =
      " collapsed to zero if it ever becomes less than `ExistentialDeposit`."]
pub struct FreeBalance<T: Trait<I>, I: Instance =
                       DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                              I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   T::Balance>
 for FreeBalance<T, I> {
    type
    Query
    =
    T::Balance;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { I::PREFIX_FOR_FreeBalance.as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::Balance>>::prefix().to_vec();
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
                                                                                                                  T::Balance>>::key_for(key);
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
                                                                                                                  T::Balance>>::key_for(key);
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
                                                                                                                  T::Balance>>::get(key,
                                                                                                                                    storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::Balance>>::insert(key,
                                                                                                                                   &val,
                                                                                                                                   storage);
        ret
    }
}
#[doc =
      " The amount of the balance of a given account that is externally reserved; this can still get"]
#[doc = " slashed, but gets slashed last of all."]
#[doc = ""]
#[doc =
      " This balance is a \'reserve\' balance that other subsystems use in order to set aside tokens"]
#[doc =
      " that are still \'owned\' by the account holder, but which are suspendable."]
#[doc = ""]
#[doc =
      " When this balance falls below the value of `ExistentialDeposit`, then this \'reserve account\'"]
#[doc = " is deleted: specifically, `ReservedBalance`."]
#[doc = ""]
#[doc =
      " `system::AccountNonce` is also deleted if `FreeBalance` is also zero (it also gets"]
#[doc =
      " collapsed to zero if it ever becomes less than `ExistentialDeposit`.)"]
pub struct ReservedBalance<T: Trait<I>, I: Instance =
                           DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                  I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   T::Balance>
 for ReservedBalance<T, I> {
    type
    Query
    =
    T::Balance;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { I::PREFIX_FOR_ReservedBalance.as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::Balance>>::prefix().to_vec();
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
                                                                                                                  T::Balance>>::key_for(key);
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
                                                                                                                  T::Balance>>::key_for(key);
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
                                                                                                                  T::Balance>>::get(key,
                                                                                                                                    storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::Balance>>::insert(key,
                                                                                                                                   &val,
                                                                                                                                   storage);
        ret
    }
}
#[doc = " Any liquidity locks on some account balances."]
pub struct Locks<T: Trait<I>, I: Instance =
                 DefaultInstance>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                        I)>);
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   Vec<BalanceLock<T::Balance,
                                                                                                                   T::BlockNumber>>>
 for Locks<T, I> {
    type
    Query
    =
    Vec<BalanceLock<T::Balance, T::BlockNumber>>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { I::PREFIX_FOR_Locks.as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  Vec<BalanceLock<T::Balance,
                                                                                                                                  T::BlockNumber>>>>::prefix().to_vec();
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
                                                                                                                  Vec<BalanceLock<T::Balance,
                                                                                                                                  T::BlockNumber>>>>::key_for(key);
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
                                                                                                                  Vec<BalanceLock<T::Balance,
                                                                                                                                  T::BlockNumber>>>>::key_for(key);
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
                                                                                                                  Vec<BalanceLock<T::Balance,
                                                                                                                                  T::BlockNumber>>>>::get(key,
                                                                                                                                                          storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              Vec<BalanceLock<T::Balance,
                                                                                                                              T::BlockNumber>>>>::insert(key,
                                                                                                                                                         &val,
                                                                                                                                                         storage);
        ret
    }
}
trait Store {
    type
    TotalIssuance;
    type
    ExistentialDeposit;
    type
    TransferFee;
    type
    CreationFee;
    type
    TransactionBaseFee;
    type
    TransactionByteFee;
    type
    Vesting;
    type
    FreeBalance;
    type
    ReservedBalance;
    type
    Locks;
}
#[doc(hidden)]
pub struct __GetByteStructTotalIssuance<T, I: Instance =
                                        DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                                   I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TotalIssuance:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructTotalIssuance<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TotalIssuance.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          T::Balance =
                                                                      Default::default();
                                                                  <T::Balance
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructExistentialDeposit<T, I: Instance =
                                             DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                                        I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ExistentialDeposit:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructExistentialDeposit<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ExistentialDeposit.get_or_init(||
                                                                   {
                                                                       let def_val:
                                                                               T::Balance =
                                                                           Default::default();
                                                                       <T::Balance
                                                                           as
                                                                           Encode>::encode(&def_val)
                                                                   }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructTransferFee<T, I: Instance =
                                      DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                                 I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TransferFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructTransferFee<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TransferFee.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        T::Balance =
                                                                    Default::default();
                                                                <T::Balance as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructCreationFee<T, I: Instance =
                                      DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                                 I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CreationFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCreationFee<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CreationFee.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        T::Balance =
                                                                    Default::default();
                                                                <T::Balance as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructTransactionBaseFee<T, I: Instance =
                                             DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                                        I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TransactionBaseFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructTransactionBaseFee<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TransactionBaseFee.get_or_init(||
                                                                   {
                                                                       let def_val:
                                                                               T::Balance =
                                                                           Default::default();
                                                                       <T::Balance
                                                                           as
                                                                           Encode>::encode(&def_val)
                                                                   }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructTransactionByteFee<T, I: Instance =
                                             DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                                        I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TransactionByteFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructTransactionByteFee<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TransactionByteFee.get_or_init(||
                                                                   {
                                                                       let def_val:
                                                                               T::Balance =
                                                                           Default::default();
                                                                       <T::Balance
                                                                           as
                                                                           Encode>::encode(&def_val)
                                                                   }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructVesting<T, I: Instance =
                                  DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                             I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Vesting:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructVesting<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Vesting.get_or_init(||
                                                        {
                                                            let def_val:
                                                                    Option<VestingSchedule<T::Balance>> =
                                                                Default::default();
                                                            <Option<VestingSchedule<T::Balance>>
                                                                as
                                                                Encode>::encode(&def_val)
                                                        }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructFreeBalance<T, I: Instance =
                                      DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                                 I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_FreeBalance:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructFreeBalance<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_FreeBalance.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        T::Balance =
                                                                    Default::default();
                                                                <T::Balance as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructReservedBalance<T, I: Instance =
                                          DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                                     I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ReservedBalance:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructReservedBalance<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ReservedBalance.get_or_init(||
                                                                {
                                                                    let def_val:
                                                                            T::Balance =
                                                                        Default::default();
                                                                    <T::Balance
                                                                        as
                                                                        Encode>::encode(&def_val)
                                                                }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructLocks<T, I: Instance =
                                DefaultInstance>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T,
                                                                                                                                           I)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Locks:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructLocks<T, I> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Locks.get_or_init(||
                                                      {
                                                          let def_val:
                                                                  Vec<BalanceLock<T::Balance,
                                                                                  T::BlockNumber>> =
                                                              Default::default();
                                                          <Vec<BalanceLock<T::Balance,
                                                                           T::BlockNumber>>
                                                              as
                                                              Encode>::encode(&def_val)
                                                      }).clone()
    }
}
impl <T: Trait<I>, I: Instance> Store for Module<T, I> {
    type
    TotalIssuance
    =
    TotalIssuance<T, I>;
    type
    ExistentialDeposit
    =
    ExistentialDeposit<T, I>;
    type
    TransferFee
    =
    TransferFee<T, I>;
    type
    CreationFee
    =
    CreationFee<T, I>;
    type
    TransactionBaseFee
    =
    TransactionBaseFee<T, I>;
    type
    TransactionByteFee
    =
    TransactionByteFee<T, I>;
    type
    Vesting
    =
    Vesting<T, I>;
    type
    FreeBalance
    =
    FreeBalance<T, I>;
    type
    ReservedBalance
    =
    ReservedBalance<T, I>;
    type
    Locks
    =
    Locks<T, I>;
}
impl <T: 'static + Trait<I>, I: Instance> Module<T, I> {
    #[doc = " The total units issued in the system."]
    pub fn total_issuance() -> T::Balance {
        <TotalIssuance<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The minimum amount required to keep an account open."]
    pub fn existential_deposit() -> T::Balance {
        <ExistentialDeposit<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The fee required to make a transfer."]
    pub fn transfer_fee() -> T::Balance {
        <TransferFee<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The fee required to create an account."]
    pub fn creation_fee() -> T::Balance {
        <CreationFee<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The fee to be paid for making a transaction; the base."]
    pub fn transaction_base_fee() -> T::Balance {
        <TransactionBaseFee<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The fee to be paid for making a transaction; the per-byte portion."]
    pub fn transaction_byte_fee() -> T::Balance {
        <TransactionByteFee<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Information regarding the vesting of a given account."]
    pub fn vesting<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                         K)
     -> Option<VestingSchedule<T::Balance>> {
        <Vesting<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              VestingSchedule<T::Balance>>>::get(key.borrow(),
                                                                                                                                                 &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The \'free\' balance of a given account."]
    #[doc = ""]
    #[doc =
          " This is the only balance that matters in terms of most operations on tokens. It"]
    #[doc =
          " alone is used to determine the balance when in the contract execution environment. When this"]
    #[doc =
          " balance falls below the value of `ExistentialDeposit`, then the \'current account\' is"]
    #[doc =
          " deleted: specifically `FreeBalance`. Further, the `OnFreeBalanceZero` callback"]
    #[doc =
          " is invoked, giving a chance to external modules to clean up data associated with"]
    #[doc = " the deleted account."]
    #[doc = ""]
    #[doc =
          " `system::AccountNonce` is also deleted if `ReservedBalance` is also zero (it also gets"]
    #[doc =
          " collapsed to zero if it ever becomes less than `ExistentialDeposit`."]
    pub fn free_balance<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                              K)
     -> T::Balance {
        <FreeBalance<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::Balance>>::get(key.borrow(),
                                                                                                                                &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The amount of the balance of a given account that is externally reserved; this can still get"]
    #[doc = " slashed, but gets slashed last of all."]
    #[doc = ""]
    #[doc =
          " This balance is a \'reserve\' balance that other subsystems use in order to set aside tokens"]
    #[doc =
          " that are still \'owned\' by the account holder, but which are suspendable."]
    #[doc = ""]
    #[doc =
          " When this balance falls below the value of `ExistentialDeposit`, then this \'reserve account\'"]
    #[doc = " is deleted: specifically, `ReservedBalance`."]
    #[doc = ""]
    #[doc =
          " `system::AccountNonce` is also deleted if `FreeBalance` is also zero (it also gets"]
    #[doc =
          " collapsed to zero if it ever becomes less than `ExistentialDeposit`.)"]
    pub fn reserved_balance<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                                  K)
     -> T::Balance {
        <ReservedBalance<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::Balance>>::get(key.borrow(),
                                                                                                                                &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Any liquidity locks on some account balances."]
    pub fn locks<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                       K)
     -> Vec<BalanceLock<T::Balance, T::BlockNumber>> {
        <Locks<T, I> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              Vec<BalanceLock<T::Balance,
                                                                                                                              T::BlockNumber>>>>::get(key.borrow(),
                                                                                                                                                      &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TotalIssuance"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTotalIssuance::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The total units issued in the system."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ExistentialDeposit"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructExistentialDeposit::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The minimum amount required to keep an account open."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransferFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransferFee::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to make a transfer."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CreationFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCreationFee::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to create an account."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransactionBaseFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransactionBaseFee::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee to be paid for making a transaction; the base."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransactionByteFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransactionByteFee::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee to be paid for making a transaction; the per-byte portion."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vesting"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VestingSchedule<T::Balance>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVesting::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Information regarding the vesting of a given account."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("FreeBalance"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructFreeBalance::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The \'free\' balance of a given account.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " This is the only balance that matters in terms of most operations on tokens. It",
                                                                                                                                                                                                                                                                                                                                                                                                    " alone is used to determine the balance when in the contract execution environment. When this",
                                                                                                                                                                                                                                                                                                                                                                                                    " balance falls below the value of `ExistentialDeposit`, then the \'current account\' is",
                                                                                                                                                                                                                                                                                                                                                                                                    " deleted: specifically `FreeBalance`. Further, the `OnFreeBalanceZero` callback",
                                                                                                                                                                                                                                                                                                                                                                                                    " is invoked, giving a chance to external modules to clean up data associated with",
                                                                                                                                                                                                                                                                                                                                                                                                    " the deleted account.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " `system::AccountNonce` is also deleted if `ReservedBalance` is also zero (it also gets",
                                                                                                                                                                                                                                                                                                                                                                                                    " collapsed to zero if it ever becomes less than `ExistentialDeposit`."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReservedBalance"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructReservedBalance::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The amount of the balance of a given account that is externally reserved; this can still get",
                                                                                                                                                                                                                                                                                                                                                                                                    " slashed, but gets slashed last of all.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " This balance is a \'reserve\' balance that other subsystems use in order to set aside tokens",
                                                                                                                                                                                                                                                                                                                                                                                                    " that are still \'owned\' by the account holder, but which are suspendable.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " When this balance falls below the value of `ExistentialDeposit`, then this \'reserve account\'",
                                                                                                                                                                                                                                                                                                                                                                                                    " is deleted: specifically, `ReservedBalance`.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " `system::AccountNonce` is also deleted if `FreeBalance` is also zero (it also gets",
                                                                                                                                                                                                                                                                                                                                                                                                    " collapsed to zero if it ever becomes less than `ExistentialDeposit`.)"]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Locks"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<BalanceLock<T::Balance, T::BlockNumber>>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLocks::<T,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Any liquidity locks on some account balances."]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TotalIssuance"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTotalIssuance::<T,
                                                                                                                                                                                                                                                                                                                                   I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The total units issued in the system."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ExistentialDeposit"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructExistentialDeposit::<T,
                                                                                                                                                                                                                                                                                                                                        I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The minimum amount required to keep an account open."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransferFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransferFee::<T,
                                                                                                                                                                                                                                                                                                                                 I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to make a transfer."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CreationFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCreationFee::<T,
                                                                                                                                                                                                                                                                                                                                 I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to create an account."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransactionBaseFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransactionBaseFee::<T,
                                                                                                                                                                                                                                                                                                                                        I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee to be paid for making a transaction; the base."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransactionByteFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransactionByteFee::<T,
                                                                                                                                                                                                                                                                                                                                        I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee to be paid for making a transaction; the per-byte portion."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vesting"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("VestingSchedule<T::Balance>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructVesting::<T,
                                                                                                                                                                                                                                                                                                                             I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Information regarding the vesting of a given account."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("FreeBalance"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructFreeBalance::<T,
                                                                                                                                                                                                                                                                                                                                 I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The \'free\' balance of a given account.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " This is the only balance that matters in terms of most operations on tokens. It",
                                                                                                                                                                                                              " alone is used to determine the balance when in the contract execution environment. When this",
                                                                                                                                                                                                              " balance falls below the value of `ExistentialDeposit`, then the \'current account\' is",
                                                                                                                                                                                                              " deleted: specifically `FreeBalance`. Further, the `OnFreeBalanceZero` callback",
                                                                                                                                                                                                              " is invoked, giving a chance to external modules to clean up data associated with",
                                                                                                                                                                                                              " the deleted account.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " `system::AccountNonce` is also deleted if `ReservedBalance` is also zero (it also gets",
                                                                                                                                                                                                              " collapsed to zero if it ever becomes less than `ExistentialDeposit`."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReservedBalance"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructReservedBalance::<T,
                                                                                                                                                                                                                                                                                                                                     I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The amount of the balance of a given account that is externally reserved; this can still get",
                                                                                                                                                                                                              " slashed, but gets slashed last of all.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " This balance is a \'reserve\' balance that other subsystems use in order to set aside tokens",
                                                                                                                                                                                                              " that are still \'owned\' by the account holder, but which are suspendable.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " When this balance falls below the value of `ExistentialDeposit`, then this \'reserve account\'",
                                                                                                                                                                                                              " is deleted: specifically, `ReservedBalance`.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " `system::AccountNonce` is also deleted if `FreeBalance` is also zero (it also gets",
                                                                                                                                                                                                              " collapsed to zero if it ever becomes less than `ExistentialDeposit`.)"]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Locks"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<BalanceLock<T::Balance, T::BlockNumber>>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLocks::<T,
                                                                                                                                                                                                                                                                                                                           I>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Any liquidity locks on some account balances."]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Balances" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Vec < ( T :: AccountId , T :: Balance ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Vec < ( T :: AccountId , T :: BlockNumber , T :: BlockNumber ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Vec < ( T :: AccountId , T :: Balance ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Vec < ( T :: AccountId , T :: BlockNumber , T :: BlockNumber ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait<I>, I: Instance = DefaultInstance> {
    #[doc = " The minimum amount required to keep an account open."]
    pub existential_deposit: T::Balance,
    #[doc = " The fee required to make a transfer."]
    pub transfer_fee: T::Balance,
    #[doc = " The fee required to create an account."]
    pub creation_fee: T::Balance,
    #[doc = " The fee to be paid for making a transaction; the base."]
    pub transaction_base_fee: T::Balance,
    #[doc =
          " The fee to be paid for making a transaction; the per-byte portion."]
    pub transaction_byte_fee: T::Balance,
    pub balances: Vec<(T::AccountId, T::Balance)>,
    pub vesting: Vec<(T::AccountId, T::BlockNumber, T::BlockNumber)>,
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
        impl <T: Trait<I>, I: Instance> _serde::Serialize for
         GenesisConfig<T, I> where
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Vec<(T::AccountId,
              T::Balance)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Vec<(T::AccountId, T::BlockNumber,
              T::BlockNumber)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
         {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "GenesisConfig",
                                                               false as usize
                                                                   + 1 + 1 + 1
                                                                   + 1 + 1 + 1
                                                                   + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "existentialDeposit",
                                                                    &self.existential_deposit)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "transferFee",
                                                                    &self.transfer_fee)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "creationFee",
                                                                    &self.creation_fee)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "transactionBaseFee",
                                                                    &self.transaction_base_fee)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "transactionByteFee",
                                                                    &self.transaction_byte_fee)
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
                                                                    "vesting",
                                                                    &self.vesting)
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
        impl <'de, T: Trait<I>, I: Instance> _serde::Deserialize<'de> for
         GenesisConfig<T, I> where
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Vec<(T::AccountId,
              T::Balance)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Vec<(T::AccountId, T::BlockNumber,
              T::BlockNumber)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                    __field5,
                    __field6,
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
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 7")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "existentialDeposit" =>
                            _serde::export::Ok(__Field::__field0),
                            "transferFee" =>
                            _serde::export::Ok(__Field::__field1),
                            "creationFee" =>
                            _serde::export::Ok(__Field::__field2),
                            "transactionBaseFee" =>
                            _serde::export::Ok(__Field::__field3),
                            "transactionByteFee" =>
                            _serde::export::Ok(__Field::__field4),
                            "balances" =>
                            _serde::export::Ok(__Field::__field5),
                            "vesting" =>
                            _serde::export::Ok(__Field::__field6),
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
                            b"existentialDeposit" =>
                            _serde::export::Ok(__Field::__field0),
                            b"transferFee" =>
                            _serde::export::Ok(__Field::__field1),
                            b"creationFee" =>
                            _serde::export::Ok(__Field::__field2),
                            b"transactionBaseFee" =>
                            _serde::export::Ok(__Field::__field3),
                            b"transactionByteFee" =>
                            _serde::export::Ok(__Field::__field4),
                            b"balances" =>
                            _serde::export::Ok(__Field::__field5),
                            b"vesting" =>
                            _serde::export::Ok(__Field::__field6),
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
                struct __Visitor<'de, T: Trait<I>, I: Instance> where
                       T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Vec<(T::AccountId,
                            T::Balance)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Vec<(T::AccountId, T::BlockNumber,
                            T::BlockNumber)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T, I>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait<I>, I: Instance> _serde::de::Visitor<'de>
                 for __Visitor<'de, T, I> where
                 T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Vec<(T::AccountId,
                      T::Balance)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Vec<(T::AccountId, T::BlockNumber,
                      T::BlockNumber)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
                 {
                    type
                    Value
                    =
                    GenesisConfig<T, I>;
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
                            match match _serde::de::SeqAccess::next_element::<T::Balance>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct GenesisConfig with 7 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<T::Balance>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct GenesisConfig with 7 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<T::Balance>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct GenesisConfig with 7 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<T::Balance>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct GenesisConfig with 7 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<T::Balance>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct GenesisConfig with 7 elements"));
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<Vec<(T::AccountId,
                                                                                   T::Balance)>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                 &"struct GenesisConfig with 7 elements"));
                                }
                            };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<Vec<(T::AccountId,
                                                                                   T::BlockNumber,
                                                                                   T::BlockNumber)>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(6usize,
                                                                                                 &"struct GenesisConfig with 7 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{existential_deposit:
                                                             __field0,
                                                         transfer_fee:
                                                             __field1,
                                                         creation_fee:
                                                             __field2,
                                                         transaction_base_fee:
                                                             __field3,
                                                         transaction_byte_fee:
                                                             __field4,
                                                         balances: __field5,
                                                         vesting: __field6,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0: _serde::export::Option<T::Balance> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<T::Balance> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<T::Balance> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<T::Balance> =
                            _serde::export::None;
                        let mut __field4: _serde::export::Option<T::Balance> =
                            _serde::export::None;
                        let mut __field5:
                                _serde::export::Option<Vec<(T::AccountId,
                                                            T::Balance)>> =
                            _serde::export::None;
                        let mut __field6:
                                _serde::export::Option<Vec<(T::AccountId,
                                                            T::BlockNumber,
                                                            T::BlockNumber)>> =
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
                                                                       _serde::de::Error>::duplicate_field("existentialDeposit"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Balance>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("transferFee"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Balance>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("creationFee"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Balance>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("transactionBaseFee"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Balance>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("transactionByteFee"));
                                    }
                                    __field4 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Balance>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("balances"));
                                    }
                                    __field5 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<(T::AccountId,
                                                                                                            T::Balance)>>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("vesting"));
                                    }
                                    __field6 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<(T::AccountId,
                                                                                                            T::BlockNumber,
                                                                                                            T::BlockNumber)>>(&mut __map)
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
                                match _serde::private::de::missing_field("existentialDeposit")
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
                                match _serde::private::de::missing_field("transferFee")
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
                                match _serde::private::de::missing_field("creationFee")
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
                                match _serde::private::de::missing_field("transactionBaseFee")
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
                                match _serde::private::de::missing_field("transactionByteFee")
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
                                match _serde::private::de::missing_field("balances")
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
                                match _serde::private::de::missing_field("vesting")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{existential_deposit:
                                                             __field0,
                                                         transfer_fee:
                                                             __field1,
                                                         creation_fee:
                                                             __field2,
                                                         transaction_base_fee:
                                                             __field3,
                                                         transaction_byte_fee:
                                                             __field4,
                                                         balances: __field5,
                                                         vesting: __field6,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["existentialDeposit", "transferFee", "creationFee",
                      "transactionBaseFee", "transactionByteFee", "balances",
                      "vesting"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "GenesisConfig",
                                                         FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<GenesisConfig<T,
                                                                                                                   I>>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance> Default for GenesisConfig<T, I> {
    fn default() -> Self {
        GenesisConfig{existential_deposit: Default::default(),
                      transfer_fee: Default::default(),
                      creation_fee: Default::default(),
                      transaction_base_fee: Default::default(),
                      transaction_byte_fee: Default::default(),
                      balances: Default::default(),
                      vesting: Default::default(),}
    }
}
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance>
 self::sr_api_hidden_includes_decl_storage::hidden_include::runtime_primitives::BuildStorage
 for GenesisConfig<T, I> {
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
                (|config: &GenesisConfig<T, I>|
                     {
                         config.balances.iter().fold(Zero::zero(),
                                                     |acc: T::Balance,
                                                      &(_, n)| acc + n)
                     })(&self);
            <TotalIssuance<T, I> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&v,
                                                                                                                                      &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T, I>|
                      config.existential_deposit.clone()))(&self);
            <ExistentialDeposit<T, I> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&v,
                                                                                                                                      &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T, I>|
                      config.transfer_fee.clone()))(&self);
            <TransferFee<T, I> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&v,
                                                                                                                                      &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T, I>|
                      config.creation_fee.clone()))(&self);
            <CreationFee<T, I> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&v,
                                                                                                                                      &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T, I>|
                      config.transaction_base_fee.clone()))(&self);
            <TransactionBaseFee<T, I> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&v,
                                                                                                                                      &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T, I>|
                      config.transaction_byte_fee.clone()))(&self);
            <TransactionByteFee<T, I> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&v,
                                                                                                                                      &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let data =
                (|config: &GenesisConfig<T, I>|
                     {
                         config.vesting.iter().filter_map(|&(ref who, begin,
                                                             length)|
                                                              {
                                                                  let begin:
                                                                          u64 =
                                                                      begin.as_();
                                                                  let length:
                                                                          u64 =
                                                                      length.as_();
                                                                  let begin:
                                                                          T::Balance =
                                                                      As::sa(begin);
                                                                  let length:
                                                                          T::Balance =
                                                                      As::sa(length);
                                                                  config.balances.iter().find(|&&(ref w,
                                                                                                  _)|
                                                                                                  w
                                                                                                      ==
                                                                                                      who).map(|&(_,
                                                                                                                  balance)|
                                                                                                                   {
                                                                                                                       let per_block =
                                                                                                                           balance
                                                                                                                               /
                                                                                                                               length.max(primitives::traits::One::one());
                                                                                                                       let offset =
                                                                                                                           begin
                                                                                                                               *
                                                                                                                               per_block
                                                                                                                               +
                                                                                                                               balance;
                                                                                                                       (who.clone(),
                                                                                                                        VestingSchedule{offset,
                                                                                                                                        per_block,})
                                                                                                                   })
                                                              }).collect::<Vec<_>>()
                     })(&self);
            for (k, v) in data.into_iter() {
                <Vesting<T, I> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      VestingSchedule<T::Balance>>>::insert(&k,
                                                                                                                                                            &v,
                                                                                                                                                            &storage);
            }
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let data =
                (|config: &GenesisConfig<T, I>|
                     config.balances.clone())(&self);
            for (k, v) in data.into_iter() {
                <FreeBalance<T, I> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      T::Balance>>::insert(&k,
                                                                                                                                           &v,
                                                                                                                                           &storage);
            }
        }
        let r = storage.into_inner();
        (|_, _, _| { })(r, c, &self);
        Ok(())
    }
}
#[structural_match]
#[rustc_copy_clone_marker]
pub struct Module<T: Trait<I>, I: Instance =
                  DefaultInstance>(::srml_support::rstd::marker::PhantomData<(T,
                                                                              I)>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::clone::Clone + Trait<I>, I: ::std::clone::Clone + Instance>
 ::std::clone::Clone for Module<T, I> {
    #[inline]
    fn clone(&self) -> Module<T, I> {
        match *self {
            Module(ref __self_0_0) =>
            Module(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::marker::Copy + Trait<I>, I: ::std::marker::Copy + Instance>
 ::std::marker::Copy for Module<T, I> {
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::cmp::PartialEq + Trait<I>, I: ::std::cmp::PartialEq +
      Instance> ::std::cmp::PartialEq for Module<T, I> {
    #[inline]
    fn eq(&self, other: &Module<T, I>) -> bool {
        match *other {
            Module(ref __self_1_0) =>
            match *self {
                Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Module<T, I>) -> bool {
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
impl <T: ::std::cmp::Eq + Trait<I>, I: ::std::cmp::Eq + Instance>
 ::std::cmp::Eq for Module<T, I> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _:
                    ::std::cmp::AssertParamIsEq<::srml_support::rstd::marker::PhantomData<(T,
                                                                                           I)>>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <T: ::std::fmt::Debug + Trait<I>, I: ::std::fmt::Debug + Instance>
 ::std::fmt::Debug for Module<T, I> {
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
impl <T: Trait<I>, I: Instance>
 ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber> for
 Module<T, I> {
}
impl <T: Trait<I>, I: Instance>
 ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber> for
 Module<T, I> {
}
impl <T: Trait<I>, I: Instance>
 ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
 for Module<T, I> {
}
impl <T: Trait<I>, I: Instance> Module<T, I> {
    fn deposit_event(event: Event<T, I>) {
        <system::Module<T>>::deposit_event(<T as
                                               Trait<I>>::from(event).into());
    }
}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl <T: Trait<I>, I: Instance> Module<T, I> {
    #[doc = r" Transfer some liquid free balance to another account."]
    #[doc = r""]
    #[doc =
          r" `transfer` will set the `FreeBalance` of the sender and receiver."]
    #[doc =
          r" It will decrease the total issuance of the system by the `TransferFee`."]
    #[doc =
          r" If the sender's account is below the existential deposit as a result"]
    #[doc = r" of the transfer, the account will be reaped."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be `Signed` by the transactor."]
    pub fn transfer(origin: T::Origin,
                    dest: <T::Lookup as StaticLookup>::Source,
                    value: T::Balance) -> ::srml_support::dispatch::Result {
        {
            let transactor = ensure_signed(origin)?;
            let dest = T::Lookup::lookup(dest)?;
            <Self as Currency<_>>::transfer(&transactor, &dest, value)?;
        }
        Ok(())
    }
    #[doc = r" Set the balances of a given account."]
    #[doc = r""]
    #[doc =
          r" This will alter `FreeBalance` and `ReservedBalance` in storage."]
    #[doc =
          r" If the new free or reserved balance is below the existential deposit,"]
    #[doc =
          r" it will also decrease the total issuance of the system (`TotalIssuance`)"]
    #[doc = r" and reset the account nonce (`system::AccountNonce`)."]
    #[doc = r""]
    #[doc = r" The dispatch origin for this call is `root`."]
    fn set_balance(who: <T::Lookup as StaticLookup>::Source, free: T::Balance,
                   reserved: T::Balance) -> ::srml_support::dispatch::Result {
        {
            let who = T::Lookup::lookup(who)?;
            Self::set_free_balance(&who, free);
            Self::set_reserved_balance(&who, reserved);
        }
        Ok(())
    }
}
pub enum Call<T: Trait<I>, I: Instance = DefaultInstance> {

    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(::srml_support::rstd::marker::PhantomData<(T, I)>,
                  ::srml_support::dispatch::Never),

    #[allow(non_camel_case_types)]
    transfer(<T::Lookup as StaticLookup>::Source,
             #[codec(compact)]
             T::Balance),

    #[allow(non_camel_case_types)]
    set_balance(<T::Lookup as StaticLookup>::Source,
                #[codec(compact)]
                T::Balance,
                #[codec(compact)]
                T::Balance),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait<I>, I: Instance> _parity_codec::Encode for Call<T, I>
         where <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         T::Balance: _parity_codec::HasCompact,
         T::Balance: _parity_codec::HasCompact,
         T::Balance: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::transfer(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        {
                            dest.push(&<<T::Balance as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Balance>>::from(ba));
                        }
                    }
                    Call::set_balance(ref aa, ref ba, ref ca) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                        {
                            dest.push(&<<T::Balance as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Balance>>::from(ba));
                        }
                        {
                            dest.push(&<<T::Balance as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Balance>>::from(ca));
                        }
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
        impl <T: Trait<I>, I: Instance> _parity_codec::Decode for Call<T, I>
         where <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         T::Balance: _parity_codec::HasCompact,
         T::Balance: _parity_codec::HasCompact,
         T::Balance: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::transfer(_parity_codec::Decode::decode(input)?,
                                            <<T::Balance as
                                             _parity_codec::HasCompact>::Type
                                                as
                                                _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::set_balance(_parity_codec::Decode::decode(input)?,
                                               <<T::Balance as
                                                _parity_codec::HasCompact>::Type
                                                   as
                                                   _parity_codec::Decode>::decode(input)?.into(),
                                               <<T::Balance as
                                                _parity_codec::HasCompact>::Type
                                                   as
                                                   _parity_codec::Decode>::decode(input)?.into()))
                    }
                    _ => None,
                }
            }
        }
    };
impl <T: Trait<I>, I: Instance> ::srml_support::dispatch::Clone for Call<T, I>
 {
    fn clone(&self) -> Self {
        match *self {
            Call::transfer(ref dest, ref value) =>
            Call::transfer((*dest).clone(), (*value).clone()),
            Call::set_balance(ref who, ref free, ref reserved) =>
            Call::set_balance((*who).clone(), (*free).clone(),
                              (*reserved).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/balances/src/lib.rs",
                                             334u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait<I>, I: Instance> ::srml_support::dispatch::PartialEq for
 Call<T, I> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::transfer(ref dest, ref value) => {
                let self_params = (dest, value);
                if let Call::transfer(ref dest, ref value) = *_other {
                    self_params == (dest, value)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/balances/src/lib.rs",
                                                         334u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_balance(ref who, ref free, ref reserved) => {
                let self_params = (who, free, reserved);
                if let Call::set_balance(ref who, ref free, ref reserved) =
                       *_other {
                    self_params == (who, free, reserved)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/balances/src/lib.rs",
                                                         334u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/balances/src/lib.rs",
                                             334u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait<I>, I: Instance> ::srml_support::dispatch::Eq for Call<T, I> {
}
#[cfg(feature = "std")]
impl <T: Trait<I>, I: Instance> ::srml_support::dispatch::fmt::Debug for
 Call<T, I> {
    fn fmt(&self, _f: &mut ::srml_support::dispatch::fmt::Formatter)
     ->
         ::srml_support::dispatch::result::Result<(),
                                                  ::srml_support::dispatch::fmt::Error> {
        match *self {
            Call::transfer(ref dest, ref value) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"transfer",
                                                               &(dest.clone(),
                                                                 value.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_balance(ref who, ref free, ref reserved) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_balance",
                                                               &(who.clone(),
                                                                 free.clone(),
                                                                 reserved.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/balances/src/lib.rs",
                                             334u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait<I>, I: Instance> ::srml_support::dispatch::Dispatchable for
 Call<T, I> {
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
            Call::transfer(dest, value) => {
                <Module<T, I>>::transfer(_origin, dest, value)
            }
            Call::set_balance(who, free, reserved) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T, I>>::set_balance(who, free, reserved)
                }
            }
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
                                                       &("srml/balances/src/lib.rs",
                                                         334u32, 1u32))
                        }
                    }
                }
            }
        }
    }
}
impl <T: Trait<I>, I: Instance> ::srml_support::dispatch::Callable for
 Module<T, I> {
    type
    Call
    =
    Call<T, I>;
}
impl <T: Trait<I>, I: Instance> Module<T, I> {
    #[doc(hidden)]
    pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait =
                    T>>(d: D, origin: D::Origin)
     -> ::srml_support::dispatch::Result {
        d.dispatch(origin)
    }
}
impl <T: Trait<I>, I: Instance> Module<T, I> {
    #[doc(hidden)]
    pub fn call_functions()
     -> &'static [::srml_support::dispatch::FunctionMetadata] {
        &[::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("transfer"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("dest"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("value"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Balance>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Transfer some liquid free balance to another account.",
                                                                                                             r"",
                                                                                                             r" `transfer` will set the `FreeBalance` of the sender and receiver.",
                                                                                                             r" It will decrease the total issuance of the system by the `TransferFee`.",
                                                                                                             r" If the sender's account is below the existential deposit as a result",
                                                                                                             r" of the transfer, the account will be reaped.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be `Signed` by the transactor."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_balance"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("who"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("free"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Balance>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("reserved"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Balance>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the balances of a given account.",
                                                                                                             r"",
                                                                                                             r" This will alter `FreeBalance` and `ReservedBalance` in storage.",
                                                                                                             r" If the new free or reserved balance is below the existential deposit,",
                                                                                                             r" it will also decrease the total issuance of the system (`TotalIssuance`)",
                                                                                                             r" and reset the account nonce (`system::AccountNonce`).",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call is `root`."]),}]
    }
}
impl <T: Trait<I>, I: Instance> Module<T, I> {
    /// Get the amount that is currently being vested and cannot be transferred out of this account.
    pub fn vesting_balance(who: &T::AccountId) -> T::Balance {
        if let Some(v) = Self::vesting(who) {
            Self::free_balance(who).min(v.locked_at(<system::Module<T>>::block_number()))
        } else { Zero::zero() }
    }
    /// Set the reserved balance of an account to some new value. Will enforce `ExistentialDeposit`
    /// law, annulling the account as needed.
    ///
    /// Doesn't do any preparatory work for creating a new account, so should only be used when it
    /// is known that the account already exists.
    ///
    /// NOTE: LOW-LEVEL: This will not attempt to maintain total issuance. It is expected that
    /// the caller will do this.
    fn set_reserved_balance(who: &T::AccountId, balance: T::Balance)
     -> UpdateBalanceOutcome {
        if balance < Self::existential_deposit() {
            <ReservedBalance<T, I>>::insert(who, balance);
            Self::on_reserved_too_low(who);
            UpdateBalanceOutcome::AccountKilled
        } else {
            <ReservedBalance<T, I>>::insert(who, balance);
            UpdateBalanceOutcome::Updated
        }
    }
    /// Set the free balance of an account to some new value. Will enforce `ExistentialDeposit`
    /// law, annulling the account as needed.
    ///
    /// Doesn't do any preparatory work for creating a new account, so should only be used when it
    /// is known that the account already exists.
    ///
    /// NOTE: LOW-LEVEL: This will not attempt to maintain total issuance. It is expected that
    /// the caller will do this.
    fn set_free_balance(who: &T::AccountId, balance: T::Balance)
     -> UpdateBalanceOutcome {
        if balance < Self::existential_deposit() {
            <FreeBalance<T, I>>::insert(who, balance);
            Self::on_free_too_low(who);
            UpdateBalanceOutcome::AccountKilled
        } else {
            <FreeBalance<T, I>>::insert(who, balance);
            UpdateBalanceOutcome::Updated
        }
    }
    /// Register a new account (with existential balance).
    ///
    /// This just calls appropriate hooks. It doesn't (necessarily) make any state changes.
    fn new_account(who: &T::AccountId, balance: T::Balance) {
        T::OnNewAccount::on_new_account(&who);
        Self::deposit_event(RawEvent::NewAccount(who.clone(),
                                                 balance.clone()));
    }
    /// Unregister an account.
    ///
    /// This just removes the nonce and leaves an event.
    fn reap_account(who: &T::AccountId) {
        <system::AccountNonce<T>>::remove(who);
        Self::deposit_event(RawEvent::ReapedAccount(who.clone()));
    }
    /// Account's free balance has dropped below existential deposit. Kill its
    /// free side and the account completely if its reserved size is already dead.
    ///
    /// Will maintain total issuance.
    fn on_free_too_low(who: &T::AccountId) {
        let dust = <FreeBalance<T, I>>::take(who);
        <Locks<T, I>>::remove(who);
        if !dust.is_zero() {
            T::DustRemoval::on_unbalanced(NegativeImbalance::new(dust));
        }
        T::OnFreeBalanceZero::on_free_balance_zero(who);
        if Self::reserved_balance(who).is_zero() { Self::reap_account(who); }
    }
    /// Account's reserved balance has dropped below existential deposit. Kill its
    /// reserved side and the account completely if its free size is already dead.
    ///
    /// Will maintain total issuance.
    fn on_reserved_too_low(who: &T::AccountId) {
        let dust = <ReservedBalance<T, I>>::take(who);
        if !dust.is_zero() {
            T::DustRemoval::on_unbalanced(NegativeImbalance::new(dust));
        }
        if Self::free_balance(who).is_zero() { Self::reap_account(who); }
    }
}
mod imbalances {
    use super::{result, Subtrait, DefaultInstance, Imbalance, Trait, Zero,
                Instance, Saturating, StorageValue};
    use rstd::mem;
    /// Opaque, move-only struct with private fields that serves as a token denoting that
    /// funds have been created without any equal and opposite accounting.
    #[must_use]
    pub struct PositiveImbalance<T: Subtrait<I>, I: Instance =
                                 DefaultInstance>(T::Balance);
    impl <T: Subtrait<I>, I: Instance> PositiveImbalance<T, I> {
        /// Create a new positive imbalance from a balance.
        pub fn new(amount: T::Balance) -> Self { PositiveImbalance(amount) }
    }
    /// Opaque, move-only struct with private fields that serves as a token denoting that
    /// funds have been destroyed without any equal and opposite accounting.
    #[must_use]
    pub struct NegativeImbalance<T: Subtrait<I>, I: Instance =
                                 DefaultInstance>(T::Balance);
    impl <T: Subtrait<I>, I: Instance> NegativeImbalance<T, I> {
        /// Create a new negative imbalance from a balance.
        pub fn new(amount: T::Balance) -> Self { NegativeImbalance(amount) }
    }
    impl <T: Trait<I>, I: Instance> Imbalance<T::Balance> for
     PositiveImbalance<T, I> {
        type
        Opposite
        =
        NegativeImbalance<T, I>;
        fn zero() -> Self { Self(Zero::zero()) }
        fn drop_zero(self) -> result::Result<(), Self> {
            if self.0.is_zero() { Ok(()) } else { Err(self) }
        }
        fn split(self, amount: T::Balance) -> (Self, Self) {
            let first = self.0.min(amount);
            let second = self.0 - first;
            mem::forget(self);
            (Self(first), Self(second))
        }
        fn merge(mut self, other: Self) -> Self {
            self.0 = self.0.saturating_add(other.0);
            mem::forget(other);
            self
        }
        fn subsume(&mut self, other: Self) {
            self.0 = self.0.saturating_add(other.0);
            mem::forget(other);
        }
        fn offset(self, other: Self::Opposite)
         -> result::Result<Self, Self::Opposite> {
            let (a, b) = (self.0, other.0);
            mem::forget((self, other));
            if a >= b {
                Ok(Self(a - b))
            } else { Err(NegativeImbalance::new(b - a)) }
        }
        fn peek(&self) -> T::Balance { self.0.clone() }
    }
    impl <T: Trait<I>, I: Instance> Imbalance<T::Balance> for
     NegativeImbalance<T, I> {
        type
        Opposite
        =
        PositiveImbalance<T, I>;
        fn zero() -> Self { Self(Zero::zero()) }
        fn drop_zero(self) -> result::Result<(), Self> {
            if self.0.is_zero() { Ok(()) } else { Err(self) }
        }
        fn split(self, amount: T::Balance) -> (Self, Self) {
            let first = self.0.min(amount);
            let second = self.0 - first;
            mem::forget(self);
            (Self(first), Self(second))
        }
        fn merge(mut self, other: Self) -> Self {
            self.0 = self.0.saturating_add(other.0);
            mem::forget(other);
            self
        }
        fn subsume(&mut self, other: Self) {
            self.0 = self.0.saturating_add(other.0);
            mem::forget(other);
        }
        fn offset(self, other: Self::Opposite)
         -> result::Result<Self, Self::Opposite> {
            let (a, b) = (self.0, other.0);
            mem::forget((self, other));
            if a >= b {
                Ok(Self(a - b))
            } else { Err(PositiveImbalance::new(b - a)) }
        }
        fn peek(&self) -> T::Balance { self.0.clone() }
    }
    impl <T: Subtrait<I>, I: Instance> Drop for PositiveImbalance<T, I> {
        /// Basic drop handler will just square up the total issuance.
        fn drop(&mut self) {
            <super::TotalIssuance<super::ElevatedTrait<T, I>,
                                  I>>::mutate(|v|
                                                  *v =
                                                      v.saturating_add(self.0));
        }
    }
    impl <T: Subtrait<I>, I: Instance> Drop for NegativeImbalance<T, I> {
        /// Basic drop handler will just square up the total issuance.
        fn drop(&mut self) {
            <super::TotalIssuance<super::ElevatedTrait<T, I>,
                                  I>>::mutate(|v|
                                                  *v =
                                                      v.saturating_sub(self.0));
        }
    }
}
struct ElevatedTrait<T: Subtrait<I>, I: Instance>(T, I);
impl <T: Subtrait<I>, I: Instance> Clone for ElevatedTrait<T, I> {
    fn clone(&self) -> Self {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("srml/balances/src/lib.rs", 642u32,
                                     28u32))
        }
    }
}
impl <T: Subtrait<I>, I: Instance> PartialEq for ElevatedTrait<T, I> {
    fn eq(&self, _: &Self) -> bool {
        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("srml/balances/src/lib.rs", 645u32,
                                     35u32))
        }
    }
}
impl <T: Subtrait<I>, I: Instance> Eq for ElevatedTrait<T, I> { }
impl <T: Subtrait<I>, I: Instance> system::Trait for ElevatedTrait<T, I> {
    type
    Origin
    =
    T::Origin;
    type
    Index
    =
    T::Index;
    type
    BlockNumber
    =
    T::BlockNumber;
    type
    Hash
    =
    T::Hash;
    type
    Hashing
    =
    T::Hashing;
    type
    Digest
    =
    T::Digest;
    type
    AccountId
    =
    T::AccountId;
    type
    Lookup
    =
    T::Lookup;
    type
    Header
    =
    T::Header;
    type
    Event
    =
    ();
    type
    Log
    =
    T::Log;
}
impl <T: Subtrait<I>, I: Instance> Trait<I> for ElevatedTrait<T, I> {
    type
    Balance
    =
    T::Balance;
    type
    OnFreeBalanceZero
    =
    T::OnFreeBalanceZero;
    type
    OnNewAccount
    =
    T::OnNewAccount;
    type
    Event
    =
    ();
    type
    TransactionPayment
    =
    ();
    type
    TransferPayment
    =
    ();
    type
    DustRemoval
    =
    ();
}
impl <T: Trait<I>, I: Instance> Currency<T::AccountId> for Module<T, I> where
 T::Balance: MaybeSerializeDebug {
    type
    Balance
    =
    T::Balance;
    type
    PositiveImbalance
    =
    PositiveImbalance<T, I>;
    type
    NegativeImbalance
    =
    NegativeImbalance<T, I>;
    fn total_balance(who: &T::AccountId) -> Self::Balance {
        Self::free_balance(who) + Self::reserved_balance(who)
    }
    fn can_slash(who: &T::AccountId, value: Self::Balance) -> bool {
        Self::free_balance(who) >= value
    }
    fn total_issuance() -> Self::Balance { <TotalIssuance<T, I>>::get() }
    fn minimum_balance() -> Self::Balance { Self::existential_deposit() }
    fn free_balance(who: &T::AccountId) -> Self::Balance {
        <FreeBalance<T, I>>::get(who)
    }
    fn ensure_can_withdraw(who: &T::AccountId, _amount: T::Balance,
                           reason: WithdrawReason, new_balance: T::Balance)
     -> Result {
        match reason {
            WithdrawReason::Reserve | WithdrawReason::Transfer if
            Self::vesting_balance(who) > new_balance =>
            return Err("vesting balance too high to send value"),
            _ => { }
        }
        let locks = Self::locks(who);
        if locks.is_empty() { return Ok(()) }
        let now = <system::Module<T>>::block_number();
        if locks.into_iter().all(|l|
                                     now >= l.until || new_balance >= l.amount
                                         || !l.reasons.contains(reason)) {
            Ok(())
        } else { Err("account liquidity restrictions prevent withdrawal") }
    }
    fn transfer(transactor: &T::AccountId, dest: &T::AccountId,
                value: Self::Balance) -> Result {
        let from_balance = Self::free_balance(transactor);
        let to_balance = Self::free_balance(dest);
        let would_create = to_balance.is_zero();
        let fee =
            if would_create {
                Self::creation_fee()
            } else { Self::transfer_fee() };
        let liability =
            match value.checked_add(&fee) {
                Some(l) => l,
                None =>
                return Err("got overflow after adding a fee to value"),
            };
        let new_from_balance =
            match from_balance.checked_sub(&liability) {
                None => return Err("balance too low to send value"),
                Some(b) => b,
            };
        if would_create && value < Self::existential_deposit() {
            return Err("value too low to create account");
        }
        Self::ensure_can_withdraw(transactor, value, WithdrawReason::Transfer,
                                  new_from_balance)?;
        let new_to_balance =
            match to_balance.checked_add(&value) {
                Some(b) => b,
                None =>
                return Err("destination balance too high to receive value"),
            };
        if transactor != dest {
            Self::set_free_balance(transactor, new_from_balance);
            if !<FreeBalance<T, I>>::exists(dest) {
                Self::new_account(dest, new_to_balance);
            }
            Self::set_free_balance(dest, new_to_balance);
            T::TransferPayment::on_unbalanced(NegativeImbalance::new(fee));
            Self::deposit_event(RawEvent::Transfer(transactor.clone(),
                                                   dest.clone(), value, fee));
        }
        Ok(())
    }
    fn withdraw(who: &T::AccountId, value: Self::Balance,
                reason: WithdrawReason, liveness: ExistenceRequirement)
     -> result::Result<Self::NegativeImbalance, &'static str> {
        if let Some(new_balance) = Self::free_balance(who).checked_sub(&value)
               {
            if liveness == ExistenceRequirement::KeepAlive &&
                   new_balance < Self::existential_deposit() {
                return Err("payment would kill account")
            }
            Self::ensure_can_withdraw(who, value, reason, new_balance)?;
            Self::set_free_balance(who, new_balance);
            Ok(NegativeImbalance::new(value))
        } else { Err("too few free funds in account") }
    }
    fn slash(who: &T::AccountId, value: Self::Balance)
     -> (Self::NegativeImbalance, Self::Balance) {
        let free_balance = Self::free_balance(who);
        let free_slash = cmp::min(free_balance, value);
        Self::set_free_balance(who, free_balance - free_slash);
        let remaining_slash = value - free_slash;
        if !remaining_slash.is_zero() {
            let reserved_balance = Self::reserved_balance(who);
            let reserved_slash = cmp::min(reserved_balance, remaining_slash);
            Self::set_reserved_balance(who,
                                       reserved_balance - reserved_slash);
            (NegativeImbalance::new(free_slash + reserved_slash),
             remaining_slash - reserved_slash)
        } else { (NegativeImbalance::new(value), Zero::zero()) }
    }
    fn deposit_into_existing(who: &T::AccountId, value: Self::Balance)
     -> result::Result<Self::PositiveImbalance, &'static str> {
        if Self::total_balance(who).is_zero() {
            return Err("beneficiary account must pre-exist");
        }
        Self::set_free_balance(who, Self::free_balance(who) + value);
        Ok(PositiveImbalance::new(value))
    }
    fn deposit_creating(who: &T::AccountId, value: Self::Balance)
     -> Self::PositiveImbalance {
        let (imbalance, _) =
            Self::make_free_balance_be(who, Self::free_balance(who) + value);
        if let SignedImbalance::Positive(p) = imbalance {
            p
        } else { Self::PositiveImbalance::zero() }
    }
    fn make_free_balance_be(who: &T::AccountId, balance: T::Balance)
     ->
         (SignedImbalance<Self::Balance, Self::PositiveImbalance>,
          UpdateBalanceOutcome) {
        let original = Self::free_balance(who);
        if balance < Self::existential_deposit() && original.is_zero() {
            return (SignedImbalance::Positive(Self::PositiveImbalance::zero()),
                    UpdateBalanceOutcome::AccountKilled)
        }
        let imbalance =
            if original <= balance {
                SignedImbalance::Positive(PositiveImbalance::new(balance -
                                                                     original))
            } else {
                SignedImbalance::Negative(NegativeImbalance::new(original -
                                                                     balance))
            };
        let outcome =
            if balance < <Module<T, I>>::existential_deposit() {
                Self::set_free_balance(who, balance);
                UpdateBalanceOutcome::AccountKilled
            } else {
                if !<FreeBalance<T, I>>::exists(who) {
                    Self::new_account(&who, balance);
                }
                Self::set_free_balance(who, balance);
                UpdateBalanceOutcome::Updated
            };
        (imbalance, outcome)
    }
}
impl <T: Trait<I>, I: Instance> ReservableCurrency<T::AccountId> for
 Module<T, I> where T::Balance: MaybeSerializeDebug {
    fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
        Self::free_balance(who).checked_sub(&value).map_or(false,
                                                           |new_balance|
                                                               Self::ensure_can_withdraw(who,
                                                                                         value,
                                                                                         WithdrawReason::Reserve,
                                                                                         new_balance).is_ok())
    }
    fn reserved_balance(who: &T::AccountId) -> Self::Balance {
        <ReservedBalance<T, I>>::get(who)
    }
    fn reserve(who: &T::AccountId, value: Self::Balance)
     -> result::Result<(), &'static str> {
        let b = Self::free_balance(who);
        if b < value { return Err("not enough free funds") }
        let new_balance = b - value;
        Self::ensure_can_withdraw(who, value, WithdrawReason::Reserve,
                                  new_balance)?;
        Self::set_reserved_balance(who, Self::reserved_balance(who) + value);
        Self::set_free_balance(who, new_balance);
        Ok(())
    }
    fn unreserve(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
        let b = Self::reserved_balance(who);
        let actual = cmp::min(b, value);
        Self::set_free_balance(who, Self::free_balance(who) + actual);
        Self::set_reserved_balance(who, b - actual);
        value - actual
    }
    fn slash_reserved(who: &T::AccountId, value: Self::Balance)
     -> (Self::NegativeImbalance, Self::Balance) {
        let b = Self::reserved_balance(who);
        let slash = cmp::min(b, value);
        Self::set_reserved_balance(who, b - slash);
        (NegativeImbalance::new(slash), value - slash)
    }
    fn repatriate_reserved(slashed: &T::AccountId, beneficiary: &T::AccountId,
                           value: Self::Balance)
     -> result::Result<Self::Balance, &'static str> {
        if Self::total_balance(beneficiary).is_zero() {
            return Err("beneficiary account must pre-exist");
        }
        let b = Self::reserved_balance(slashed);
        let slash = cmp::min(b, value);
        Self::set_free_balance(beneficiary,
                               Self::free_balance(beneficiary) + slash);
        Self::set_reserved_balance(slashed, b - slash);
        Ok(value - slash)
    }
}
impl <T: Trait<I>, I: Instance> LockableCurrency<T::AccountId> for
 Module<T, I> where T::Balance: MaybeSerializeDebug {
    type
    Moment
    =
    T::BlockNumber;
    fn set_lock(id: LockIdentifier, who: &T::AccountId, amount: T::Balance,
                until: T::BlockNumber, reasons: WithdrawReasons) {
        let now = <system::Module<T>>::block_number();
        let mut new_lock = Some(BalanceLock{id, amount, until, reasons,});
        let mut locks =
            Self::locks(who).into_iter().filter_map(|l|
                                                        if l.id == id {
                                                            new_lock.take()
                                                        } else if l.until >
                                                                      now {
                                                            Some(l)
                                                        } else {
                                                            None
                                                        }).collect::<Vec<_>>();
        if let Some(lock) = new_lock { locks.push(lock) }
        <Locks<T, I>>::insert(who, locks);
    }
    fn extend_lock(id: LockIdentifier, who: &T::AccountId, amount: T::Balance,
                   until: T::BlockNumber, reasons: WithdrawReasons) {
        let now = <system::Module<T>>::block_number();
        let mut new_lock = Some(BalanceLock{id, amount, until, reasons,});
        let mut locks =
            Self::locks(who).into_iter().filter_map(|l|
                                                        if l.id == id {
                                                            new_lock.take().map(|nl|
                                                                                    {
                                                                                        BalanceLock{id:
                                                                                                        l.id,
                                                                                                    amount:
                                                                                                        l.amount.max(nl.amount),
                                                                                                    until:
                                                                                                        l.until.max(nl.until),
                                                                                                    reasons:
                                                                                                        l.reasons
                                                                                                            |
                                                                                                            nl.reasons,}
                                                                                    })
                                                        } else if l.until >
                                                                      now {
                                                            Some(l)
                                                        } else {
                                                            None
                                                        }).collect::<Vec<_>>();
        if let Some(lock) = new_lock { locks.push(lock) }
        <Locks<T, I>>::insert(who, locks);
    }
    fn remove_lock(id: LockIdentifier, who: &T::AccountId) {
        let now = <system::Module<T>>::block_number();
        let locks =
            Self::locks(who).into_iter().filter_map(|l|
                                                        if l.until > now &&
                                                               l.id != id {
                                                            Some(l)
                                                        } else {
                                                            None
                                                        }).collect::<Vec<_>>();
        <Locks<T, I>>::insert(who, locks);
    }
}
impl <T: Trait<I>, I: Instance> MakePayment<T::AccountId> for Module<T, I> {
    fn make_payment(transactor: &T::AccountId, encoded_len: usize) -> Result {
        let encoded_len = <T::Balance as As<u64>>::sa(encoded_len as u64);
        let transaction_fee =
            Self::transaction_base_fee() +
                Self::transaction_byte_fee() * encoded_len;
        let imbalance =
            Self::withdraw(transactor, transaction_fee,
                           WithdrawReason::TransactionPayment,
                           ExistenceRequirement::KeepAlive)?;
        T::TransactionPayment::on_unbalanced(imbalance);
        Ok(())
    }
}
impl <T: Trait<I>, I: Instance> IsDeadAccount<T::AccountId> for Module<T, I>
 where T::Balance: MaybeSerializeDebug {
    fn is_dead_account(who: &T::AccountId) -> bool {
        Self::total_balance(who).is_zero()
    }
}
