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

//! # Assets Module
//!
//! A simple, secure module for dealing with fungible assets.
//!
//! ## Overview
//!
//! The Assets module provides functionality for asset management of fungible asset classes
//! with a fixed supply, including:
//!
//! * Asset Issuance
//! * Asset Transfer
//! * Asset Destruction
//!
//! To use it in your runtime, you need to implement the assets [`Trait`](./trait.Trait.html).
//!
//! The supported dispatchable functions are documented in the [`Call`](./enum.Call.html) enum.
//!
//! ### Terminology
//!
//! * **Asset issuance:** The creation of a new asset, whose total supply will belong to the account that issues the asset.
//! * **Asset transfer:** The action of transferring assets from one account to another.
//! * **Asset destruction:** The process of an account removing its entire holding of an asset.
//! * **Fungible asset:** An asset whose units are interchangeable.
//! * **Non-fungible asset:** An asset for which each unit has unique characteristics.
//!
//! ### Goals
//!
//! The assets system in Substrate is designed to make the following possible:
//!
//! * Issue a unique asset to its creator's account.
//! * Move assets between accounts.
//! * Remove an account's balance of an asset when requested by that account's owner and update the asset's total supply.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `issue` - Issues the total supply of a new fungible asset to the account of the caller of the function.
//! * `transfer` - Transfers an `amount` of units of fungible asset `id` from the balance of
//! the function caller's account (`origin`) to a `target` account.
//! * `destroy` - Destroys the entire holding of a fungible asset `id` associated with the account
//! that called the function.
//!
//! Please refer to the [`Call`](./enum.Call.html) enum and its associated variants for documentation on each function.
//!
//! ### Public Functions
//! <!-- Original author of descriptions: @gavofyork -->
//!
//! * `balance` - Get the asset `id` balance of `who`.
//! * `total_supply` - Get the total supply of an asset `id`.
//!
//! Please refer to the [`Module`](./struct.Module.html) struct for details on publicly available functions.
//!
//! ## Usage
//!
//! The following example shows how to use the Assets module in your runtime by exposing public functions to:
//!
//! * Issue a new fungible asset for a token distribution event (airdrop).
//! * Query the fungible asset holding balance of an account.
//! * Query the total supply of a fungible asset that has been issued.
//!
//! ### Prerequisites
//!
//! Import the Assets module and types and derive your runtime's configuration traits from the Assets module trait.
//!
//! ### Simple Code Snippet
//!
//! ```rust,ignore
//! use support::{decl_module, dispatch::Result};
//! use system::ensure_signed;
//!
//! pub trait Trait: assets::Trait { }
//!
//! decl_module! {
//! 	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
//! 		pub fn issue_token_airdrop(origin) -> Result {
//! 			const ACCOUNT_ALICE: u64 = 1;
//! 			const ACCOUNT_BOB: u64 = 2;
//! 			const COUNT_AIRDROP_RECIPIENTS = 2;
//! 			const TOKENS_FIXED_SUPPLY: u64 = 100;
//!
//! 			ensure!(!COUNT_AIRDROP_RECIPIENTS.is_zero(), "Divide by zero error.");
//!
//! 			let sender = ensure_signed(origin)?;
//! 			let asset_id = Self::next_asset_id();
//!
//! 			<NextAssetId<T>>::mutate(|asset_id| *asset_id += 1);
//! 			<Balances<T>>::insert((asset_id, &ACCOUNT_ALICE), TOKENS_FIXED_SUPPLY / COUNT_AIRDROP_RECIPIENTS);
//! 			<Balances<T>>::insert((asset_id, &ACCOUNT_BOB), TOKENS_FIXED_SUPPLY / COUNT_AIRDROP_RECIPIENTS);
//! 			<TotalSupply<T>>::insert(asset_id, TOKENS_FIXED_SUPPLY);
//!
//! 			Self::deposit_event(RawEvent::Issued(asset_id, sender, TOKENS_FIXED_SUPPLY));
//! 			Ok(())
//! 		}
//! 	}
//! }
//! ```
//!
//! ## Related Modules
//!
//! * [`System`](../srml_system/index.html)
//! * [`Support`](../srml_support/index.html)
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

// Ensure we're `no_std` when compiling for Wasm.

use srml_support::{StorageValue, StorageMap, Parameter, decl_module,
                   decl_event, decl_storage, ensure};
use primitives::traits::{Member, SimpleArithmetic, Zero, StaticLookup};
use system::ensure_signed;

/// The module configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type
    Event: From<Event<Self>> +
    Into<<Self as system::Trait>::Event>;

    /// The units in which we record balances.
    type
    Balance: Member +
    Parameter +
    SimpleArithmetic +
    Default +
    Copy;
}

type AssetId = u32;











// The main implementation block for the module.
// Public immutables




// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.


// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.








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
impl <T: Trait> Module<T> {
    #[doc =
          r" Issue a new class of fungible assets. There are, and will only ever be, `total`"]
    #[doc =
          r" such assets and they'll all belong to the `origin` initially. It will have an"]
    #[doc =
          r" identifier `AssetId` instance: this will be specified in the `Issued` event."]
    fn issue(origin: T::Origin, total: T::Balance)
     -> ::srml_support::dispatch::Result {
        {
            let origin = ensure_signed(origin)?;
            let id = Self::next_asset_id();
            <NextAssetId<T>>::mutate(|id| *id += 1);
            <Balances<T>>::insert((id, origin.clone()), total);
            <TotalSupply<T>>::insert(id, total);
            Self::deposit_event(RawEvent::Issued(id, origin, total));
        }
        Ok(())
    }
    #[doc = r" Move some assets from one holder to another."]
    fn transfer(origin: T::Origin, id: AssetId,
                target: <T::Lookup as StaticLookup>::Source,
                amount: T::Balance) -> ::srml_support::dispatch::Result {
        {
            let origin = ensure_signed(origin)?;
            let origin_account = (id, origin.clone());
            let origin_balance = <Balances<T>>::get(&origin_account);
            let target = T::Lookup::lookup(target)?;
            {
                if !!amount.is_zero() {
                    { return Err("transfer amount should be non-zero"); };
                }
            };
            {
                if !(origin_balance >= amount) {
                    {
                        return Err("origin account balance must be greater than or equal to the transfer amount");
                    };
                }
            };
            Self::deposit_event(RawEvent::Transferred(id, origin,
                                                      target.clone(),
                                                      amount));
            <Balances<T>>::insert(origin_account, origin_balance - amount);
            <Balances<T>>::mutate((id, target), |balance| *balance += amount);
        }
        Ok(())
    }
    #[doc = r" Destroy any assets of `id` owned by `origin`."]
    fn destroy(origin: T::Origin, id: AssetId)
     -> ::srml_support::dispatch::Result {
        {
            let origin = ensure_signed(origin)?;
            let balance = <Balances<T>>::take((id, origin.clone()));
            {
                if !!balance.is_zero() {
                    { return Err("origin balance should be non-zero"); };
                }
            };
            <TotalSupply<T>>::mutate(id,
                                     |total_supply| *total_supply -= balance);
            Self::deposit_event(RawEvent::Destroyed(id, origin, balance));
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
    issue(
          #[codec(compact)]
          T::Balance),

    #[allow(non_camel_case_types)]
    transfer(
             #[codec(compact)]
             AssetId, <T::Lookup as StaticLookup>::Source,
             #[codec(compact)]
             T::Balance),

    #[allow(non_camel_case_types)]
    destroy(
            #[codec(compact)]
            AssetId),
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
         T::Balance: _parity_codec::HasCompact,
         T::Balance: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::issue(ref aa) => {
                        dest.push_byte(0usize as u8);
                        {
                            dest.push(&<<T::Balance as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Balance>>::from(aa));
                        }
                    }
                    Call::transfer(ref aa, ref ba, ref ca) => {
                        dest.push_byte(1usize as u8);
                        {
                            dest.push(&<<AssetId as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      AssetId>>::from(aa));
                        }
                        dest.push(ba);
                        {
                            dest.push(&<<T::Balance as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Balance>>::from(ca));
                        }
                    }
                    Call::destroy(ref aa) => {
                        dest.push_byte(2usize as u8);
                        {
                            dest.push(&<<AssetId as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      AssetId>>::from(aa));
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
         T::Balance: _parity_codec::HasCompact,
         T::Balance: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::issue(<<T::Balance as
                                          _parity_codec::HasCompact>::Type as
                                             _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::transfer(<<AssetId as
                                             _parity_codec::HasCompact>::Type
                                                as
                                                _parity_codec::Decode>::decode(input)?.into(),
                                            _parity_codec::Decode::decode(input)?,
                                            <<T::Balance as
                                             _parity_codec::HasCompact>::Type
                                                as
                                                _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 2usize as u8 => {
                        Some(Call::destroy(<<AssetId as
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
            Call::issue(ref total) => Call::issue((*total).clone()),
            Call::transfer(ref id, ref target, ref amount) =>
            Call::transfer((*id).clone(), (*target).clone(),
                           (*amount).clone()),
            Call::destroy(ref id) => Call::destroy((*id).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/assets/src/lib.rs", 138u32,
                                             1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::issue(ref total) => {
                let self_params = (total,);
                if let Call::issue(ref total) = *_other {
                    self_params == (total,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/assets/src/lib.rs",
                                                         138u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::transfer(ref id, ref target, ref amount) => {
                let self_params = (id, target, amount);
                if let Call::transfer(ref id, ref target, ref amount) =
                       *_other {
                    self_params == (id, target, amount)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/assets/src/lib.rs",
                                                         138u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::destroy(ref id) => {
                let self_params = (id,);
                if let Call::destroy(ref id) = *_other {
                    self_params == (id,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/assets/src/lib.rs",
                                                         138u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/assets/src/lib.rs", 138u32,
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
            Call::issue(ref total) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"issue",
                                                               &(total.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::transfer(ref id, ref target, ref amount) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"transfer",
                                                               &(id.clone(),
                                                                 target.clone(),
                                                                 amount.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::destroy(ref id) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"destroy",
                                                               &(id.clone(),))
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
                                           &("srml/assets/src/lib.rs", 138u32,
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
            Call::issue(total) => { <Module<T>>::issue(_origin, total) }
            Call::transfer(id, target, amount) => {
                <Module<T>>::transfer(_origin, id, target, amount)
            }
            Call::destroy(id) => { <Module<T>>::destroy(_origin, id) }
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
                                                       &("srml/assets/src/lib.rs",
                                                         138u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("issue"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("total"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Balance>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Issue a new class of fungible assets. There are, and will only ever be, `total`",
                                                                                                             r" such assets and they'll all belong to the `origin` initially. It will have an",
                                                                                                             r" identifier `AssetId` instance: this will be specified in the `Issued` event."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("transfer"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("id"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<AssetId>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("target"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("amount"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Balance>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Move some assets from one holder to another."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("destroy"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("id"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<AssetId>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Destroy any assets of `id` owned by `origin`."]),}]
    }
}
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T>
    =
    RawEvent<<T as system::Trait>::AccountId, <T as Trait>::Balance>;
/// Events for this module.
///
#[structural_match]
pub enum RawEvent<AccountId, Balance> {

    #[doc = r" Some assets were issued."]
    Issued(AssetId, AccountId, Balance),

    #[doc = r" Some assets were transferred."]
    Transferred(AssetId, AccountId, AccountId, Balance),

    #[doc = r" Some assets were destroyed."]
    Destroyed(AssetId, AccountId, Balance),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::clone::Clone, Balance: ::std::clone::Clone>
 ::std::clone::Clone for RawEvent<AccountId, Balance> {
    #[inline]
    fn clone(&self) -> RawEvent<AccountId, Balance> {
        match (&*self,) {
            (&RawEvent::Issued(ref __self_0, ref __self_1, ref __self_2),) =>
            RawEvent::Issued(::std::clone::Clone::clone(&(*__self_0)),
                             ::std::clone::Clone::clone(&(*__self_1)),
                             ::std::clone::Clone::clone(&(*__self_2))),
            (&RawEvent::Transferred(ref __self_0, ref __self_1, ref __self_2,
                                    ref __self_3),) =>
            RawEvent::Transferred(::std::clone::Clone::clone(&(*__self_0)),
                                  ::std::clone::Clone::clone(&(*__self_1)),
                                  ::std::clone::Clone::clone(&(*__self_2)),
                                  ::std::clone::Clone::clone(&(*__self_3))),
            (&RawEvent::Destroyed(ref __self_0, ref __self_1, ref __self_2),)
            =>
            RawEvent::Destroyed(::std::clone::Clone::clone(&(*__self_0)),
                                ::std::clone::Clone::clone(&(*__self_1)),
                                ::std::clone::Clone::clone(&(*__self_2))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialEq, Balance: ::std::cmp::PartialEq>
 ::std::cmp::PartialEq for RawEvent<AccountId, Balance> {
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId, Balance>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::Issued(ref __self_0, ref __self_1,
                                       ref __self_2),
                     &RawEvent::Issued(ref __arg_1_0, ref __arg_1_1,
                                       ref __arg_1_2)) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2),
                    (&RawEvent::Transferred(ref __self_0, ref __self_1,
                                            ref __self_2, ref __self_3),
                     &RawEvent::Transferred(ref __arg_1_0, ref __arg_1_1,
                                            ref __arg_1_2, ref __arg_1_3)) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2) &&
                        (*__self_3) == (*__arg_1_3),
                    (&RawEvent::Destroyed(ref __self_0, ref __self_1,
                                          ref __self_2),
                     &RawEvent::Destroyed(ref __arg_1_0, ref __arg_1_1,
                                          ref __arg_1_2)) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId, Balance>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::Issued(ref __self_0, ref __self_1,
                                       ref __self_2),
                     &RawEvent::Issued(ref __arg_1_0, ref __arg_1_1,
                                       ref __arg_1_2)) =>
                    (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1)
                        || (*__self_2) != (*__arg_1_2),
                    (&RawEvent::Transferred(ref __self_0, ref __self_1,
                                            ref __self_2, ref __self_3),
                     &RawEvent::Transferred(ref __arg_1_0, ref __arg_1_1,
                                            ref __arg_1_2, ref __arg_1_3)) =>
                    (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1)
                        || (*__self_2) != (*__arg_1_2) ||
                        (*__self_3) != (*__arg_1_3),
                    (&RawEvent::Destroyed(ref __self_0, ref __self_1,
                                          ref __self_2),
                     &RawEvent::Destroyed(ref __arg_1_0, ref __arg_1_1,
                                          ref __arg_1_2)) =>
                    (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1)
                        || (*__self_2) != (*__arg_1_2),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::Eq, Balance: ::std::cmp::Eq> ::std::cmp::Eq for
 RawEvent<AccountId, Balance> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<AssetId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<AssetId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<AssetId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
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
        impl <AccountId, Balance> _parity_codec::Encode for
         RawEvent<AccountId, Balance> where AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::Issued(ref aa, ref ba, ref ca) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    RawEvent::Transferred(ref aa, ref ba, ref ca, ref da) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                        dest.push(da);
                    }
                    RawEvent::Destroyed(ref aa, ref ba, ref ca) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
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
        impl <AccountId, Balance> _parity_codec::Decode for
         RawEvent<AccountId, Balance> where AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::Issued(_parity_codec::Decode::decode(input)?,
                                              _parity_codec::Decode::decode(input)?,
                                              _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(RawEvent::Transferred(_parity_codec::Decode::decode(input)?,
                                                   _parity_codec::Decode::decode(input)?,
                                                   _parity_codec::Decode::decode(input)?,
                                                   _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(RawEvent::Destroyed(_parity_codec::Decode::decode(input)?,
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
impl <AccountId: ::std::fmt::Debug, Balance: ::std::fmt::Debug>
 ::std::fmt::Debug for RawEvent<AccountId, Balance> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawEvent::Issued(ref __self_0, ref __self_1, ref __self_2),) =>
            {
                let mut debug_trait_builder = f.debug_tuple("Issued");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                debug_trait_builder.finish()
            }
            (&RawEvent::Transferred(ref __self_0, ref __self_1, ref __self_2,
                                    ref __self_3),) => {
                let mut debug_trait_builder = f.debug_tuple("Transferred");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                let _ = debug_trait_builder.field(&&(*__self_3));
                debug_trait_builder.finish()
            }
            (&RawEvent::Destroyed(ref __self_0, ref __self_1, ref __self_2),)
            => {
                let mut debug_trait_builder = f.debug_tuple("Destroyed");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <AccountId, Balance> From<RawEvent<AccountId, Balance>> for () {
    fn from(_: RawEvent<AccountId, Balance>) -> () { () }
}
impl <AccountId, Balance> RawEvent<AccountId, Balance> {
    #[allow(dead_code)]
    pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
        &[::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Issued"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AssetId",
                                                                                                    "AccountId",
                                                                                                    "Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Some assets were issued."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Transferred"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AssetId",
                                                                                                    "AccountId",
                                                                                                    "AccountId",
                                                                                                    "Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Some assets were transferred."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Destroyed"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AssetId",
                                                                                                    "AccountId",
                                                                                                    "Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Some assets were destroyed."]),}]
    }
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = " The number of units of assets held by any given account."]
struct Balances<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(AssetId,
                                                                                                    T::AccountId),
                                                                                                   T::Balance>
 for Balances<T> {
    type
    Query
    =
    T::Balance;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Assets Balances".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &(AssetId, T::AccountId))
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(AssetId,
                                                                                                                   T::AccountId),
                                                                                                                  T::Balance>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &(AssetId,
                                                                                                                                                                     T::AccountId),
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(AssetId,
                                                                                                                   T::AccountId),
                                                                                                                  T::Balance>>::key_for(key);
        storage.get(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &(AssetId,
                                                                                                                                                                      T::AccountId),
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(AssetId,
                                                                                                                   T::AccountId),
                                                                                                                  T::Balance>>::key_for(key);
        storage.take(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &(AssetId,
                                                                                                                                                                        T::AccountId),
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(AssetId,
                                                                                                                   T::AccountId),
                                                                                                                  T::Balance>>::get(key,
                                                                                                                                    storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<(AssetId,
                                                                                                               T::AccountId),
                                                                                                              T::Balance>>::insert(key,
                                                                                                                                   &val,
                                                                                                                                   storage);
        ret
    }
}
#[doc = " The next asset identifier up for grabs."]
struct NextAssetId<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<AssetId>
 for NextAssetId<T> {
    type
    Query
    =
    AssetId;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Assets NextAssetId".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<AssetId>>::key()).unwrap_or_else(||
                                                                                                                                                                 Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<AssetId>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<AssetId>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<AssetId>>::put(&val,
                                                                                                                               storage);
        ret
    }
}
#[doc = " The total unit supply of an asset."]
struct TotalSupply<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<AssetId,
                                                                                                   T::Balance>
 for TotalSupply<T> {
    type
    Query
    =
    T::Balance;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Assets TotalSupply".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &AssetId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<AssetId,
                                                                                                                  T::Balance>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &AssetId,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<AssetId,
                                                                                                                  T::Balance>>::key_for(key);
        storage.get(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &AssetId,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<AssetId,
                                                                                                                  T::Balance>>::key_for(key);
        storage.take(&key[..]).unwrap_or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &AssetId,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<AssetId,
                                                                                                                  T::Balance>>::get(key,
                                                                                                                                    storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<AssetId,
                                                                                                              T::Balance>>::insert(key,
                                                                                                                                   &val,
                                                                                                                                   storage);
        ret
    }
}
trait Store {
    type
    Balances;
    type
    NextAssetId;
    type
    TotalSupply;
}
#[doc(hidden)]
pub struct __GetByteStructBalances<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Balances:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructBalances<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Balances.get_or_init(||
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
pub struct __GetByteStructNextAssetId<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_NextAssetId:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNextAssetId<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_NextAssetId.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        AssetId =
                                                                    Default::default();
                                                                <AssetId as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructTotalSupply<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TotalSupply:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructTotalSupply<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TotalSupply.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        T::Balance =
                                                                    Default::default();
                                                                <T::Balance as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    Balances
    =
    Balances<T>;
    type
    NextAssetId
    =
    NextAssetId<T>;
    type
    TotalSupply
    =
    TotalSupply<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc = " The next asset identifier up for grabs."]
    pub fn next_asset_id() -> AssetId {
        <NextAssetId<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<AssetId>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Balances"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(AssetId, T::AccountId)"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBalances::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of units of assets held by any given account."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextAssetId"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AssetId")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextAssetId::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next asset identifier up for grabs."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TotalSupply"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AssetId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTotalSupply::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The total unit supply of an asset."]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Balances"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("(AssetId, T::AccountId)"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBalances::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of units of assets held by any given account."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextAssetId"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AssetId")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextAssetId::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next asset identifier up for grabs."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TotalSupply"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AssetId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTotalSupply::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The total unit supply of an asset."]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Assets" }
}
impl <T: Trait> Module<T> {
    /// Get the asset `id` balance of `who`.
    pub fn balance(id: AssetId, who: T::AccountId) -> T::Balance {
        <Balances<T>>::get((id, who))
    }
    /// Get the total supply of an asset `id`.
    pub fn total_supply(id: AssetId) -> T::Balance {
        <TotalSupply<T>>::get(id)
    }
}
