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

//! # Treasury Module
//! 
//! The `treasury` module keeps account of currency in a `pot` and manages the subsequent
//! deployment of these funds.
//! 
//! ## Overview
//! 
//! Funds for treasury are raised in two ways:
//! 1. By minting new tokens, leading to inflation, and
//! 2. By channeling tokens from transaction fees and slashing.
//! 
//! Treasury funds can be used to pay for developers who provide software updates,
//! any changes decided by referenda, and to generally keep the system running smoothly. 
//! 
//! Treasury can be used with other modules, such as to tax validator rewards in the `staking` module.
//! 
//! ### Implementations 
//! 
//! The treasury module provides an implementation for the following trait:
//! - `OnDilution` - Mint extra funds upon dilution; maintain the ratio of `portion` diluted to `total_issuance`.
//! 
//! ## Interface
//! 
//! ### Dispatchable Functions
//! 
//! - `propose_spend` - Propose a spending proposal and stake a proposal deposit.
//! - `set_pot` - Set the spendable balance of funds.
//! - `configure` - Configure the module's proposal requirements.
//! - `reject_proposal` - Reject a proposal and slash the deposit.
//! - `approve_proposal` - Accept the proposal and return the deposit.
//! 
//! Please refer to the [`Call`](./enum.Call.html) enum and its associated variants for documentation on each function.
//! 
//! ### Public Functions
//! 
//! See the [module](./struct.Module.html) for details on publicly available functions.
//! 
//! ## Related Modules
//! 
//! The treasury module depends on the `system` and `srml_support` modules as well as
//! Substrate Core libraries and the Rust standard library.
//! 
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use rstd::prelude::*;
use srml_support::{StorageValue, StorageMap, decl_module, decl_storage,
                   decl_event, ensure};
use srml_support::traits::{Currency, ReservableCurrency, OnDilution,
                           OnUnbalanced, Imbalance};
use runtime_primitives::{Permill,
                         traits::{Zero, EnsureOrigin, StaticLookup,
                                  Saturating, CheckedSub, CheckedMul}};
use parity_codec::{Encode, Decode};
use system::ensure_signed;

type BalanceOf<T>
    =
    <<T as Trait>::Currency as
    Currency<<T as system::Trait>::AccountId>>::Balance;
type PositiveImbalanceOf<T>
    =
    <<T as Trait>::Currency as
    Currency<<T as system::Trait>::AccountId>>::PositiveImbalance;
type NegativeImbalanceOf<T>
    =
    <<T as Trait>::Currency as
    Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;

pub trait Trait: system::Trait {
    /// The staking balance.
    type
    Currency: Currency<Self::AccountId> +
    ReservableCurrency<Self::AccountId>;

    /// Origin from which approvals must come.
    type
    ApproveOrigin: EnsureOrigin<Self::Origin>;

    /// Origin from which rejections must come.
    type
    RejectOrigin: EnsureOrigin<Self::Origin>;

    /// The overarching event type.
    type
    Event: From<Event<Self>> +
    Into<<Self as system::Trait>::Event>;

    /// Handler for the unbalanced increase when minting cash from the "Pot".
    type
    MintedForSpending: OnUnbalanced<PositiveImbalanceOf<Self>>;

    /// Handler for the unbalanced decrease when slashing for a rejected proposal.
    type
    ProposalRejection: OnUnbalanced<NegativeImbalanceOf<Self>>;
}

type ProposalIndex = u32;





// Put the new value into storage.







// Check to see if we should spend some funds!


// Config...





// State...






// Add public immutables and private mutables.


// Spend some money!

// Should always be true, but shouldn't panic if false or we're screwed.

// return their deposit.

// provide the allocation.



// burn some proportion of the remaining budget if we run a surplus.



// Mint extra funds for the treasury to keep the ratio of portion to total_issuance equal
// pre dilution and post-dilution.







// Check that accumulate works when we have Some value in Dummy already.





















// Note: This test demonstrates that `on_dilution` does not increase the pot with good resolution
// with large amounts of the network staked. https://github.com/paritytech/substrate/issues/2579
// A fix to 2579 should include a change of this test.
// minted = 1% of total issuance for all cases

// portion = 33% of total issuance
// should increase by 4 (200 - 66) / 66 * 2

// portion = 33+eps% of total issuance
// should increase by 2 (200 - 67) / 67 * 2

// portion = 50% of total issuance
// should increase by 2 (200 - 100) / 100 * 2

// If any more than 50% of the network is staked (i.e. (2 * portion) > total_issuance)
// then the pot will not increase.
// portion = 50+eps% of total issuance
// should increase by 0 (200 - 101) / 101 * 2

// portion = 67% of total issuance
// should increase by 0 (200 - 134) / 134 * 2




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
        if (n % Self::spend_period()).is_zero() { Self::spend_funds(); }
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
    #[doc =
          r" Put forward a suggestion for spending. A deposit proportional to the value"]
    #[doc =
          r" is reserved and slashed if the proposal is rejected. It is returned once the"]
    #[doc = r" proposal is awarded."]
    fn propose_spend(origin: T::Origin, value: BalanceOf<T>,
                     beneficiary: <T::Lookup as StaticLookup>::Source)
     -> ::srml_support::dispatch::Result {
        {
            let proposer = ensure_signed(origin)?;
            let beneficiary = T::Lookup::lookup(beneficiary)?;
            let bond = Self::calculate_bond(value);
            T::Currency::reserve(&proposer,
                                 bond).map_err(|_|
                                                   "Proposer's balance too low")?;
            let c = Self::proposal_count();
            <ProposalCount<T>>::put(c + 1);
            <Proposals<T>>::insert(c,
                                   Proposal{proposer,
                                            value,
                                            beneficiary,
                                            bond,});
            Self::deposit_event(RawEvent::Proposed(c));
        }
        Ok(())
    }
    #[doc = r" Set the balance of funds available to spend."]
    fn set_pot(new_pot: BalanceOf<T>) -> ::srml_support::dispatch::Result {
        { <Pot<T>>::put(new_pot); }
        Ok(())
    }
    #[doc = r" (Re-)configure this module."]
    fn configure(proposal_bond: Permill, proposal_bond_minimum: BalanceOf<T>,
                 spend_period: T::BlockNumber, burn: Permill)
     -> ::srml_support::dispatch::Result {
        {
            <ProposalBond<T>>::put(proposal_bond);
            <ProposalBondMinimum<T>>::put(proposal_bond_minimum);
            <SpendPeriod<T>>::put(spend_period);
            <Burn<T>>::put(burn);
        }
        Ok(())
    }
    #[doc =
          r" Reject a proposed spend. The original deposit will be slashed."]
    fn reject_proposal(origin: T::Origin, proposal_id: ProposalIndex)
     -> ::srml_support::dispatch::Result {
        {
            T::RejectOrigin::ensure_origin(origin)?;
            let proposal =
                <Proposals<T>>::take(proposal_id).ok_or("No proposal at that index")?;
            let value = proposal.bond;
            let imbalance =
                T::Currency::slash_reserved(&proposal.proposer, value).0;
            T::ProposalRejection::on_unbalanced(imbalance);
        }
        Ok(())
    }
    #[doc =
          r" Approve a proposal. At a later time, the proposal will be allocated to the beneficiary"]
    #[doc = r" and the original deposit will be returned."]
    fn approve_proposal(origin: T::Origin, proposal_id: ProposalIndex)
     -> ::srml_support::dispatch::Result {
        {
            T::ApproveOrigin::ensure_origin(origin)?;
            {
                if !<Proposals<T>>::exists(proposal_id) {
                    { return Err("No proposal at that index"); };
                }
            };
            <Approvals<T>>::mutate(|v| v.push(proposal_id));
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
    propose_spend(
                  #[codec(compact)]
                  BalanceOf<T>, <T::Lookup as StaticLookup>::Source),

    #[allow(non_camel_case_types)]
    set_pot(
            #[codec(compact)]
            BalanceOf<T>),

    #[allow(non_camel_case_types)]
    configure(
              #[codec(compact)]
              Permill,
              #[codec(compact)]
              BalanceOf<T>,
              #[codec(compact)]
              T::BlockNumber,
              #[codec(compact)]
              Permill),

    #[allow(non_camel_case_types)]
    reject_proposal(
                    #[codec(compact)]
                    ProposalIndex),

    #[allow(non_camel_case_types)]
    approve_proposal(
                     #[codec(compact)]
                     ProposalIndex),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for Call<T> where
         <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         BalanceOf<T>: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::BlockNumber: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::propose_spend(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        {
                            dest.push(&<<BalanceOf<T> as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      BalanceOf<T>>>::from(aa));
                        }
                        dest.push(ba);
                    }
                    Call::set_pot(ref aa) => {
                        dest.push_byte(1usize as u8);
                        {
                            dest.push(&<<BalanceOf<T> as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      BalanceOf<T>>>::from(aa));
                        }
                    }
                    Call::configure(ref aa, ref ba, ref ca, ref da) => {
                        dest.push_byte(2usize as u8);
                        {
                            dest.push(&<<Permill as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      Permill>>::from(aa));
                        }
                        {
                            dest.push(&<<BalanceOf<T> as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      BalanceOf<T>>>::from(ba));
                        }
                        {
                            dest.push(&<<T::BlockNumber as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::BlockNumber>>::from(ca));
                        }
                        {
                            dest.push(&<<Permill as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      Permill>>::from(da));
                        }
                    }
                    Call::reject_proposal(ref aa) => {
                        dest.push_byte(3usize as u8);
                        {
                            dest.push(&<<ProposalIndex as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      ProposalIndex>>::from(aa));
                        }
                    }
                    Call::approve_proposal(ref aa) => {
                        dest.push_byte(4usize as u8);
                        {
                            dest.push(&<<ProposalIndex as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      ProposalIndex>>::from(aa));
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
        impl <T: Trait> _parity_codec::Decode for Call<T> where
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         BalanceOf<T>: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::BlockNumber: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::propose_spend(<<BalanceOf<T> as
                                                  _parity_codec::HasCompact>::Type
                                                     as
                                                     _parity_codec::Decode>::decode(input)?.into(),
                                                 _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::set_pot(<<BalanceOf<T> as
                                            _parity_codec::HasCompact>::Type
                                               as
                                               _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 2usize as u8 => {
                        Some(Call::configure(<<Permill as
                                              _parity_codec::HasCompact>::Type
                                                 as
                                                 _parity_codec::Decode>::decode(input)?.into(),
                                             <<BalanceOf<T> as
                                              _parity_codec::HasCompact>::Type
                                                 as
                                                 _parity_codec::Decode>::decode(input)?.into(),
                                             <<T::BlockNumber as
                                              _parity_codec::HasCompact>::Type
                                                 as
                                                 _parity_codec::Decode>::decode(input)?.into(),
                                             <<Permill as
                                              _parity_codec::HasCompact>::Type
                                                 as
                                                 _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 3usize as u8 => {
                        Some(Call::reject_proposal(<<ProposalIndex as
                                                    _parity_codec::HasCompact>::Type
                                                       as
                                                       _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 4usize as u8 => {
                        Some(Call::approve_proposal(<<ProposalIndex as
                                                     _parity_codec::HasCompact>::Type
                                                        as
                                                        _parity_codec::Decode>::decode(input)?.into()))
                    }
                    _ => None,
                }
            }
        }
    };
impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::propose_spend(ref value, ref beneficiary) =>
            Call::propose_spend((*value).clone(), (*beneficiary).clone()),
            Call::set_pot(ref new_pot) => Call::set_pot((*new_pot).clone()),
            Call::configure(ref proposal_bond, ref proposal_bond_minimum,
                            ref spend_period, ref burn) =>
            Call::configure((*proposal_bond).clone(),
                            (*proposal_bond_minimum).clone(),
                            (*spend_period).clone(), (*burn).clone()),
            Call::reject_proposal(ref proposal_id) =>
            Call::reject_proposal((*proposal_id).clone()),
            Call::approve_proposal(ref proposal_id) =>
            Call::approve_proposal((*proposal_id).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/treasury/src/lib.rs",
                                             99u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::propose_spend(ref value, ref beneficiary) => {
                let self_params = (value, beneficiary);
                if let Call::propose_spend(ref value, ref beneficiary) =
                       *_other {
                    self_params == (value, beneficiary)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/treasury/src/lib.rs",
                                                         99u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_pot(ref new_pot) => {
                let self_params = (new_pot,);
                if let Call::set_pot(ref new_pot) = *_other {
                    self_params == (new_pot,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/treasury/src/lib.rs",
                                                         99u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::configure(ref proposal_bond, ref proposal_bond_minimum,
                            ref spend_period, ref burn) => {
                let self_params =
                    (proposal_bond, proposal_bond_minimum, spend_period,
                     burn);
                if let Call::configure(ref proposal_bond,
                                       ref proposal_bond_minimum,
                                       ref spend_period, ref burn) = *_other {
                    self_params ==
                        (proposal_bond, proposal_bond_minimum, spend_period,
                         burn)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/treasury/src/lib.rs",
                                                         99u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::reject_proposal(ref proposal_id) => {
                let self_params = (proposal_id,);
                if let Call::reject_proposal(ref proposal_id) = *_other {
                    self_params == (proposal_id,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/treasury/src/lib.rs",
                                                         99u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::approve_proposal(ref proposal_id) => {
                let self_params = (proposal_id,);
                if let Call::approve_proposal(ref proposal_id) = *_other {
                    self_params == (proposal_id,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/treasury/src/lib.rs",
                                                         99u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/treasury/src/lib.rs",
                                             99u32, 1u32))
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
            Call::propose_spend(ref value, ref beneficiary) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"propose_spend",
                                                               &(value.clone(),
                                                                 beneficiary.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_pot(ref new_pot) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_pot",
                                                               &(new_pot.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::configure(ref proposal_bond, ref proposal_bond_minimum,
                            ref spend_period, ref burn) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"configure",
                                                               &(proposal_bond.clone(),
                                                                 proposal_bond_minimum.clone(),
                                                                 spend_period.clone(),
                                                                 burn.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::reject_proposal(ref proposal_id) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"reject_proposal",
                                                               &(proposal_id.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::approve_proposal(ref proposal_id) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"approve_proposal",
                                                               &(proposal_id.clone(),))
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
                                           &("srml/treasury/src/lib.rs",
                                             99u32, 1u32))
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
            Call::propose_spend(value, beneficiary) => {
                <Module<T>>::propose_spend(_origin, value, beneficiary)
            }
            Call::set_pot(new_pot) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_pot(new_pot)
                }
            }
            Call::configure(proposal_bond, proposal_bond_minimum,
                            spend_period, burn) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::configure(proposal_bond,
                                           proposal_bond_minimum,
                                           spend_period, burn)
                }
            }
            Call::reject_proposal(proposal_id) => {
                <Module<T>>::reject_proposal(_origin, proposal_id)
            }
            Call::approve_proposal(proposal_id) => {
                <Module<T>>::approve_proposal(_origin, proposal_id)
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
                                                       &("srml/treasury/src/lib.rs",
                                                         99u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("propose_spend"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("value"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("beneficiary"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Put forward a suggestion for spending. A deposit proportional to the value",
                                                                                                             r" is reserved and slashed if the proposal is rejected. It is returned once the",
                                                                                                             r" proposal is awarded."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_pot"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("new_pot"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the balance of funds available to spend."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("configure"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("proposal_bond"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<Permill>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("proposal_bond_minimum"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("spend_period"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("burn"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<Permill>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" (Re-)configure this module."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("reject_proposal"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("proposal_id"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<ProposalIndex>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Reject a proposed spend. The original deposit will be slashed."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("approve_proposal"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("proposal_id"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<ProposalIndex>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Approve a proposal. At a later time, the proposal will be allocated to the beneficiary",
                                                                                                             r" and the original deposit will be returned."]),}]
    }
}
/// A spending proposal.
#[structural_match]
pub struct Proposal<AccountId, Balance> {
    proposer: AccountId,
    value: Balance,
    beneficiary: AccountId,
    bond: Balance,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Proposal: () =
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
        impl <AccountId, Balance> _serde::Serialize for
         Proposal<AccountId, Balance> where AccountId: _serde::Serialize,
         Balance: _serde::Serialize {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "Proposal",
                                                               false as usize
                                                                   + 1 + 1 + 1
                                                                   + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "proposer",
                                                                    &self.proposer)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "value",
                                                                    &self.value)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "beneficiary",
                                                                    &self.beneficiary)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "bond",
                                                                    &self.bond)
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
const _IMPL_DESERIALIZE_FOR_Proposal: () =
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
        impl <'de, AccountId, Balance> _serde::Deserialize<'de> for
         Proposal<AccountId, Balance> where
         AccountId: _serde::Deserialize<'de>,
         Balance: _serde::Deserialize<'de> {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
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
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 4")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "proposer" =>
                            _serde::export::Ok(__Field::__field0),
                            "value" => _serde::export::Ok(__Field::__field1),
                            "beneficiary" =>
                            _serde::export::Ok(__Field::__field2),
                            "bond" => _serde::export::Ok(__Field::__field3),
                            _ => { _serde::export::Ok(__Field::__ignore) }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"proposer" =>
                            _serde::export::Ok(__Field::__field0),
                            b"value" => _serde::export::Ok(__Field::__field1),
                            b"beneficiary" =>
                            _serde::export::Ok(__Field::__field2),
                            b"bond" => _serde::export::Ok(__Field::__field3),
                            _ => { _serde::export::Ok(__Field::__ignore) }
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
                struct __Visitor<'de, AccountId, Balance> where
                       AccountId: _serde::Deserialize<'de>,
                       Balance: _serde::Deserialize<'de> {
                    marker: _serde::export::PhantomData<Proposal<AccountId,
                                                                 Balance>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, AccountId, Balance> _serde::de::Visitor<'de> for
                 __Visitor<'de, AccountId, Balance> where
                 AccountId: _serde::Deserialize<'de>,
                 Balance: _serde::Deserialize<'de> {
                    type
                    Value
                    =
                    Proposal<AccountId, Balance>;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct Proposal")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<AccountId>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct Proposal with 4 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Balance>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct Proposal with 4 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<AccountId>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct Proposal with 4 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<Balance>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct Proposal with 4 elements"));
                                }
                            };
                        _serde::export::Ok(Proposal{proposer: __field0,
                                                    value: __field1,
                                                    beneficiary: __field2,
                                                    bond: __field3,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0: _serde::export::Option<AccountId> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<Balance> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<AccountId> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<Balance> =
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
                                                                       _serde::de::Error>::duplicate_field("proposer"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<AccountId>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("value"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Balance>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("beneficiary"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<AccountId>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("bond"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Balance>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                _ => {
                                    let _ =
                                        match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                            {
                                            _serde::export::Ok(__val) =>
                                            __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("proposer")
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
                                match _serde::private::de::missing_field("value")
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
                                match _serde::private::de::missing_field("beneficiary")
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
                                match _serde::private::de::missing_field("bond")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(Proposal{proposer: __field0,
                                                    value: __field1,
                                                    beneficiary: __field2,
                                                    bond: __field3,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["proposer", "value", "beneficiary", "bond"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "Proposal", FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<Proposal<AccountId,
                                                                                                              Balance>>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::fmt::Debug, Balance: ::std::fmt::Debug>
 ::std::fmt::Debug for Proposal<AccountId, Balance> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Proposal {
            proposer: ref __self_0_0,
            value: ref __self_0_1,
            beneficiary: ref __self_0_2,
            bond: ref __self_0_3 } => {
                let mut debug_trait_builder = f.debug_struct("Proposal");
                let _ =
                    debug_trait_builder.field("proposer", &&(*__self_0_0));
                let _ = debug_trait_builder.field("value", &&(*__self_0_1));
                let _ =
                    debug_trait_builder.field("beneficiary", &&(*__self_0_2));
                let _ = debug_trait_builder.field("bond", &&(*__self_0_3));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Proposal: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, Balance> _parity_codec::Encode for
         Proposal<AccountId, Balance> where AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.proposer);
                dest.push(&self.value);
                dest.push(&self.beneficiary);
                dest.push(&self.bond);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Proposal: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, Balance> _parity_codec::Decode for
         Proposal<AccountId, Balance> where AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(Proposal{proposer: _parity_codec::Decode::decode(input)?,
                              value: _parity_codec::Decode::decode(input)?,
                              beneficiary:
                                  _parity_codec::Decode::decode(input)?,
                              bond: _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::clone::Clone, Balance: ::std::clone::Clone>
 ::std::clone::Clone for Proposal<AccountId, Balance> {
    #[inline]
    fn clone(&self) -> Proposal<AccountId, Balance> {
        match *self {
            Proposal {
            proposer: ref __self_0_0,
            value: ref __self_0_1,
            beneficiary: ref __self_0_2,
            bond: ref __self_0_3 } =>
            Proposal{proposer: ::std::clone::Clone::clone(&(*__self_0_0)),
                     value: ::std::clone::Clone::clone(&(*__self_0_1)),
                     beneficiary: ::std::clone::Clone::clone(&(*__self_0_2)),
                     bond: ::std::clone::Clone::clone(&(*__self_0_3)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialEq, Balance: ::std::cmp::PartialEq>
 ::std::cmp::PartialEq for Proposal<AccountId, Balance> {
    #[inline]
    fn eq(&self, other: &Proposal<AccountId, Balance>) -> bool {
        match *other {
            Proposal {
            proposer: ref __self_1_0,
            value: ref __self_1_1,
            beneficiary: ref __self_1_2,
            bond: ref __self_1_3 } =>
            match *self {
                Proposal {
                proposer: ref __self_0_0,
                value: ref __self_0_1,
                beneficiary: ref __self_0_2,
                bond: ref __self_0_3 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Proposal<AccountId, Balance>) -> bool {
        match *other {
            Proposal {
            proposer: ref __self_1_0,
            value: ref __self_1_1,
            beneficiary: ref __self_1_2,
            bond: ref __self_1_3 } =>
            match *self {
                Proposal {
                proposer: ref __self_0_0,
                value: ref __self_0_1,
                beneficiary: ref __self_0_2,
                bond: ref __self_0_3 } =>
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
impl <AccountId: ::std::cmp::Eq, Balance: ::std::cmp::Eq> ::std::cmp::Eq for
 Proposal<AccountId, Balance> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
        }
    }
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc =
      " Proportion of funds that should be bonded in order to place a proposal. An accepted"]
#[doc = " proposal gets these back. A rejected proposal doesn\'t."]
struct ProposalBond<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>
 for ProposalBond<T> {
    type
    Query
    =
    Permill;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Treasury ProposalBond".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::key()).unwrap_or_else(||
                                                                                                                                                                 Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::put(&val,
                                                                                                                               storage);
        ret
    }
}
#[doc =
      " Minimum amount of funds that should be placed in a deposit for making a proposal."]
struct ProposalBondMinimum<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for ProposalBondMinimum<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Treasury ProposalBondMinimum".as_bytes() }
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
#[doc = " Period between successive spends."]
struct SpendPeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for SpendPeriod<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Treasury SpendPeriod".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                        runtime_primitives::traits::One::one())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                         runtime_primitives::traits::One::one())
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
      " Percentage of spare funds (if any) that are burnt per spend period."]
struct Burn<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>
 for Burn<T> {
    type
    Query
    =
    Permill;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Treasury Burn".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::key()).unwrap_or_else(||
                                                                                                                                                                 Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::put(&val,
                                                                                                                               storage);
        ret
    }
}
#[doc = " Total funds available to this module for spending."]
struct Pot<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for Pot<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Treasury Pot".as_bytes() }
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
#[doc = " Number of proposals that have been made."]
struct ProposalCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ProposalIndex>
 for ProposalCount<T> {
    type
    Query
    =
    ProposalIndex;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Treasury ProposalCount".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ProposalIndex>>::key()).unwrap_or_else(||
                                                                                                                                                                       Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ProposalIndex>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ProposalIndex>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ProposalIndex>>::put(&val,
                                                                                                                                     storage);
        ret
    }
}
#[doc = " Proposals that have been made."]
struct Proposals<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ProposalIndex,
                                                                                                   Proposal<T::AccountId,
                                                                                                            BalanceOf<T>>>
 for Proposals<T> {
    type
    Query
    =
    Option<Proposal<T::AccountId, BalanceOf<T>>>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Treasury Proposals".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &ProposalIndex)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ProposalIndex,
                                                                                                                  Proposal<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &ProposalIndex,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ProposalIndex,
                                                                                                                  Proposal<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::key_for(key);
        storage.get(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &ProposalIndex,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ProposalIndex,
                                                                                                                  Proposal<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::key_for(key);
        storage.take(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &ProposalIndex,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ProposalIndex,
                                                                                                                  Proposal<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::get(key,
                                                                                                                                                storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ProposalIndex,
                                                                                                                  Proposal<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::insert(key,
                                                                                                                                                   &val,
                                                                                                                                                   storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ProposalIndex,
                                                                                                                  Proposal<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::remove(key,
                                                                                                                                                   storage),
        };
        ret
    }
}
#[doc = " Proposal indices that have been approved but not yet awarded."]
struct Approvals<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<ProposalIndex>>
 for Approvals<T> {
    type
    Query
    =
    Vec<ProposalIndex>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Treasury Approvals".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<ProposalIndex>>>::key()).unwrap_or_else(||
                                                                                                                                                                            Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<ProposalIndex>>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<ProposalIndex>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<ProposalIndex>>>::put(&val,
                                                                                                                                          storage);
        ret
    }
}
trait Store {
    type
    ProposalBond;
    type
    ProposalBondMinimum;
    type
    SpendPeriod;
    type
    Burn;
    type
    Pot;
    type
    ProposalCount;
    type
    Proposals;
    type
    Approvals;
}
#[doc(hidden)]
pub struct __GetByteStructProposalBond<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ProposalBond:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructProposalBond<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ProposalBond.get_or_init(||
                                                             {
                                                                 let def_val:
                                                                         Permill =
                                                                     Default::default();
                                                                 <Permill as
                                                                     Encode>::encode(&def_val)
                                                             }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructProposalBondMinimum<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ProposalBondMinimum:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructProposalBondMinimum<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ProposalBondMinimum.get_or_init(||
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
pub struct __GetByteStructSpendPeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_SpendPeriod:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructSpendPeriod<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_SpendPeriod.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        T::BlockNumber =
                                                                    runtime_primitives::traits::One::one();
                                                                <T::BlockNumber
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructBurn<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Burn:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructBurn<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Burn.get_or_init(||
                                                     {
                                                         let def_val:
                                                                 Permill =
                                                             Default::default();
                                                         <Permill as
                                                             Encode>::encode(&def_val)
                                                     }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructPot<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Pot:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructPot<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Pot.get_or_init(||
                                                    {
                                                        let def_val:
                                                                BalanceOf<T> =
                                                            Default::default();
                                                        <BalanceOf<T> as
                                                            Encode>::encode(&def_val)
                                                    }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructProposalCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ProposalCount:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructProposalCount<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ProposalCount.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          ProposalIndex =
                                                                      Default::default();
                                                                  <ProposalIndex
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructProposals<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Proposals:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructProposals<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Proposals.get_or_init(||
                                                          {
                                                              let def_val:
                                                                      Option<Proposal<T::AccountId,
                                                                                      BalanceOf<T>>> =
                                                                  Default::default();
                                                              <Option<Proposal<T::AccountId,
                                                                               BalanceOf<T>>>
                                                                  as
                                                                  Encode>::encode(&def_val)
                                                          }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructApprovals<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Approvals:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructApprovals<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Approvals.get_or_init(||
                                                          {
                                                              let def_val:
                                                                      Vec<ProposalIndex> =
                                                                  Default::default();
                                                              <Vec<ProposalIndex>
                                                                  as
                                                                  Encode>::encode(&def_val)
                                                          }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    ProposalBond
    =
    ProposalBond<T>;
    type
    ProposalBondMinimum
    =
    ProposalBondMinimum<T>;
    type
    SpendPeriod
    =
    SpendPeriod<T>;
    type
    Burn
    =
    Burn<T>;
    type
    Pot
    =
    Pot<T>;
    type
    ProposalCount
    =
    ProposalCount<T>;
    type
    Proposals
    =
    Proposals<T>;
    type
    Approvals
    =
    Approvals<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc =
          " Proportion of funds that should be bonded in order to place a proposal. An accepted"]
    #[doc = " proposal gets these back. A rejected proposal doesn\'t."]
    pub fn proposal_bond() -> Permill {
        <ProposalBond<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Minimum amount of funds that should be placed in a deposit for making a proposal."]
    pub fn proposal_bond_minimum() -> BalanceOf<T> {
        <ProposalBondMinimum<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Period between successive spends."]
    pub fn spend_period() -> T::BlockNumber {
        <SpendPeriod<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Percentage of spare funds (if any) that are burnt per spend period."]
    pub fn burn() -> Permill {
        <Burn<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Total funds available to this module for spending."]
    pub fn pot() -> BalanceOf<T> {
        <Pot<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Number of proposals that have been made."]
    pub fn proposal_count() -> ProposalIndex {
        <ProposalCount<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<ProposalIndex>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Proposals that have been made."]
    pub fn proposals<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<ProposalIndex>>(key:
                                                                                                                            K)
     -> Option<Proposal<T::AccountId, BalanceOf<T>>> {
        <Proposals<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<ProposalIndex,
                                                                                                              Proposal<T::AccountId,
                                                                                                                       BalanceOf<T>>>>::get(key.borrow(),
                                                                                                                                            &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Proposal indices that have been approved but not yet awarded."]
    pub fn approvals() -> Vec<ProposalIndex> {
        <Approvals<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<ProposalIndex>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalBond"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Permill")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalBond::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Proportion of funds that should be bonded in order to place a proposal. An accepted",
                                                                                                                                                                                                                                                                                                                                                                                                    " proposal gets these back. A rejected proposal doesn\'t."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalBondMinimum"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalBondMinimum::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Minimum amount of funds that should be placed in a deposit for making a proposal."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SpendPeriod"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSpendPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Period between successive spends."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Burn"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Permill")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBurn::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Percentage of spare funds (if any) that are burnt per spend period."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Pot"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPot::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Total funds available to this module for spending."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalCount"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalIndex")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of proposals that have been made."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proposals"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalIndex"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proposal<T::AccountId, BalanceOf<T>>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposals::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Proposals that have been made."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Approvals"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<ProposalIndex>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructApprovals::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Proposal indices that have been approved but not yet awarded."]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalBond"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Permill")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalBond::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Proportion of funds that should be bonded in order to place a proposal. An accepted",
                                                                                                                                                                                                              " proposal gets these back. A rejected proposal doesn\'t."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalBondMinimum"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalBondMinimum::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Minimum amount of funds that should be placed in a deposit for making a proposal."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SpendPeriod"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSpendPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Period between successive spends."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Burn"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Permill")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBurn::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Percentage of spare funds (if any) that are burnt per spend period."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Pot"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPot::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Total funds available to this module for spending."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalCount"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalIndex")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposalCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of proposals that have been made."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proposals"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ProposalIndex"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Proposal<T::AccountId, BalanceOf<T>>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructProposals::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Proposals that have been made."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Approvals"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<ProposalIndex>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructApprovals::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Proposal indices that have been approved but not yet awarded."]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Treasury" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "Permill : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Permill : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "Permill : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Permill : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    #[doc =
          " Proportion of funds that should be bonded in order to place a proposal. An accepted"]
    #[doc = " proposal gets these back. A rejected proposal doesn\'t."]
    pub proposal_bond: Permill,
    #[doc =
          " Minimum amount of funds that should be placed in a deposit for making a proposal."]
    pub proposal_bond_minimum: BalanceOf<T>,
    #[doc = " Period between successive spends."]
    pub spend_period: T::BlockNumber,
    #[doc =
          " Percentage of spare funds (if any) that are burnt per spend period."]
    pub burn: Permill,
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
         Permill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Permill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
         {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "GenesisConfig",
                                                               false as usize
                                                                   + 1 + 1 + 1
                                                                   + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "proposalBond",
                                                                    &self.proposal_bond)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "proposalBondMinimum",
                                                                    &self.proposal_bond_minimum)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "spendPeriod",
                                                                    &self.spend_period)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "burn",
                                                                    &self.burn)
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
         Permill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Permill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
         {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, __field2, __field3, }
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
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 4")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "proposalBond" =>
                            _serde::export::Ok(__Field::__field0),
                            "proposalBondMinimum" =>
                            _serde::export::Ok(__Field::__field1),
                            "spendPeriod" =>
                            _serde::export::Ok(__Field::__field2),
                            "burn" => _serde::export::Ok(__Field::__field3),
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
                            b"proposalBond" =>
                            _serde::export::Ok(__Field::__field0),
                            b"proposalBondMinimum" =>
                            _serde::export::Ok(__Field::__field1),
                            b"spendPeriod" =>
                            _serde::export::Ok(__Field::__field2),
                            b"burn" => _serde::export::Ok(__Field::__field3),
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
                       Permill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Permill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 Permill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Permill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            match match _serde::de::SeqAccess::next_element::<Permill>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct GenesisConfig with 4 elements"));
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
                                                                                                 &"struct GenesisConfig with 4 elements"));
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
                                                                                                 &"struct GenesisConfig with 4 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<Permill>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct GenesisConfig with 4 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{proposal_bond:
                                                             __field0,
                                                         proposal_bond_minimum:
                                                             __field1,
                                                         spend_period:
                                                             __field2,
                                                         burn: __field3,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0: _serde::export::Option<Permill> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<T::BlockNumber> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<Permill> =
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
                                                                       _serde::de::Error>::duplicate_field("proposalBond"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Permill>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("proposalBondMinimum"));
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
                                                                       _serde::de::Error>::duplicate_field("spendPeriod"));
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
                                                                       _serde::de::Error>::duplicate_field("burn"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Permill>(&mut __map)
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
                                match _serde::private::de::missing_field("proposalBond")
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
                                match _serde::private::de::missing_field("proposalBondMinimum")
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
                                match _serde::private::de::missing_field("spendPeriod")
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
                                match _serde::private::de::missing_field("burn")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{proposal_bond:
                                                             __field0,
                                                         proposal_bond_minimum:
                                                             __field1,
                                                         spend_period:
                                                             __field2,
                                                         burn: __field3,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["proposalBond", "proposalBondMinimum", "spendPeriod",
                      "burn"];
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
        GenesisConfig{proposal_bond: Default::default(),
                      proposal_bond_minimum: Default::default(),
                      spend_period: runtime_primitives::traits::One::one(),
                      burn: Default::default(),}
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
            let v =
                ((|config: &GenesisConfig<T>|
                      config.proposal_bond.clone()))(&self);
            <ProposalBond<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::put(&v,
                                                                                                                                   &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.proposal_bond_minimum.clone()))(&self);
            <ProposalBondMinimum<T> as
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
                      config.spend_period.clone()))(&self);
            <SpendPeriod<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                          &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>| config.burn.clone()))(&self);
            <Burn<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Permill>>::put(&v,
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

    #[doc = r" New proposal."]
    Proposed(ProposalIndex),

    #[doc = r" We have ended a spend period and will now allocate funds."]
    Spending(Balance),

    #[doc = r" Some funds have been allocated."]
    Awarded(ProposalIndex, Balance, AccountId),

    #[doc = r" Some of our funds have been burnt."]
    Burnt(Balance),

    #[doc =
          r" Spending has finished; this is the amount that rolls over until next spend."]
    Rollover(Balance),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::clone::Clone, AccountId: ::std::clone::Clone>
 ::std::clone::Clone for RawEvent<Balance, AccountId> {
    #[inline]
    fn clone(&self) -> RawEvent<Balance, AccountId> {
        match (&*self,) {
            (&RawEvent::Proposed(ref __self_0),) =>
            RawEvent::Proposed(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::Spending(ref __self_0),) =>
            RawEvent::Spending(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::Awarded(ref __self_0, ref __self_1, ref __self_2),) =>
            RawEvent::Awarded(::std::clone::Clone::clone(&(*__self_0)),
                              ::std::clone::Clone::clone(&(*__self_1)),
                              ::std::clone::Clone::clone(&(*__self_2))),
            (&RawEvent::Burnt(ref __self_0),) =>
            RawEvent::Burnt(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::Rollover(ref __self_0),) =>
            RawEvent::Rollover(::std::clone::Clone::clone(&(*__self_0))),
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
                    (&RawEvent::Proposed(ref __self_0),
                     &RawEvent::Proposed(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::Spending(ref __self_0),
                     &RawEvent::Spending(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::Awarded(ref __self_0, ref __self_1,
                                        ref __self_2),
                     &RawEvent::Awarded(ref __arg_1_0, ref __arg_1_1,
                                        ref __arg_1_2)) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2),
                    (&RawEvent::Burnt(ref __self_0),
                     &RawEvent::Burnt(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::Rollover(ref __self_0),
                     &RawEvent::Rollover(ref __arg_1_0)) =>
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
                    (&RawEvent::Proposed(ref __self_0),
                     &RawEvent::Proposed(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::Spending(ref __self_0),
                     &RawEvent::Spending(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::Awarded(ref __self_0, ref __self_1,
                                        ref __self_2),
                     &RawEvent::Awarded(ref __arg_1_0, ref __arg_1_1,
                                        ref __arg_1_2)) =>
                    (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1)
                        || (*__self_2) != (*__arg_1_2),
                    (&RawEvent::Burnt(ref __self_0),
                     &RawEvent::Burnt(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::Rollover(ref __self_0),
                     &RawEvent::Rollover(ref __arg_1_0)) =>
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
            let _: ::std::cmp::AssertParamIsEq<ProposalIndex>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<ProposalIndex>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
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
         Balance: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::Proposed(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::Spending(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::Awarded(ref aa, ref ba, ref ca) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    RawEvent::Burnt(ref aa) => {
                        dest.push_byte(3usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::Rollover(ref aa) => {
                        dest.push_byte(4usize as u8);
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
         Balance: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::Proposed(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(RawEvent::Spending(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(RawEvent::Awarded(_parity_codec::Decode::decode(input)?,
                                               _parity_codec::Decode::decode(input)?,
                                               _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 3usize as u8 => {
                        Some(RawEvent::Burnt(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 4usize as u8 => {
                        Some(RawEvent::Rollover(_parity_codec::Decode::decode(input)?))
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
            (&RawEvent::Proposed(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Proposed");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::Spending(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Spending");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::Awarded(ref __self_0, ref __self_1, ref __self_2),) =>
            {
                let mut debug_trait_builder = f.debug_tuple("Awarded");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                debug_trait_builder.finish()
            }
            (&RawEvent::Burnt(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Burnt");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::Rollover(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Rollover");
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
                                                   ::srml_support::event::DecodeDifferent::Encode(&["ProposalIndex"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" New proposal."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Spending"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" We have ended a spend period and will now allocate funds."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Awarded"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["ProposalIndex",
                                                                                                    "Balance",
                                                                                                    "AccountId"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Some funds have been allocated."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Burnt"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Some of our funds have been burnt."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Rollover"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Spending has finished; this is the amount that rolls over until next spend."]),}]
    }
}
impl <T: Trait> Module<T> {
    /// The needed bond for a proposal whose spend is `value`.
    fn calculate_bond(value: BalanceOf<T>) -> BalanceOf<T> {
        Self::proposal_bond_minimum().max(Self::proposal_bond() * value)
    }
    fn spend_funds() {
        let mut budget_remaining = Self::pot();
        Self::deposit_event(RawEvent::Spending(budget_remaining));
        let mut missed_any = false;
        let mut imbalance = <PositiveImbalanceOf<T>>::zero();
        <Approvals<T>>::mutate(|v|
                                   {
                                       v.retain(|&index|
                                                    {
                                                        if let Some(p) =
                                                               Self::proposals(index)
                                                               {
                                                            if p.value <=
                                                                   budget_remaining
                                                               {
                                                                budget_remaining
                                                                    -=
                                                                    p.value;
                                                                <Proposals<T>>::remove(index);
                                                                let _ =
                                                                    T::Currency::unreserve(&p.proposer,
                                                                                           p.bond);
                                                                imbalance.subsume(T::Currency::deposit_creating(&p.beneficiary,
                                                                                                                p.value));
                                                                Self::deposit_event(RawEvent::Awarded(index,
                                                                                                      p.value,
                                                                                                      p.beneficiary));
                                                                false
                                                            } else {
                                                                missed_any =
                                                                    true;
                                                                true
                                                            }
                                                        } else { false }
                                                    });
                                   });
        T::MintedForSpending::on_unbalanced(imbalance);
        if !missed_any {
            let burn =
                (Self::burn() * budget_remaining).min(budget_remaining);
            budget_remaining -= burn;
            Self::deposit_event(RawEvent::Burnt(burn))
        }
        Self::deposit_event(RawEvent::Rollover(budget_remaining));
        <Pot<T>>::put(budget_remaining);
    }
}
impl <T: Trait> OnDilution<BalanceOf<T>> for Module<T> {
    fn on_dilution(minted: BalanceOf<T>, portion: BalanceOf<T>) {
        if !minted.is_zero() && !portion.is_zero() {
            let total_issuance = T::Currency::total_issuance();
            if let Some(funding) = total_issuance.checked_sub(&portion) {
                let funding = funding / portion;
                if let Some(funding) = funding.checked_mul(&minted) {
                    <Pot<T>>::mutate(|x| *x = x.saturating_add(funding));
                }
            }
        }
    }
}
