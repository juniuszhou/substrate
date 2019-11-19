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
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

//! # Contract Module
//!
//! The Contract module provides functionality for the runtime to deploy and execute WebAssembly smart-contracts.
//!
//! - [`contract::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//!
//! This module extends accounts based on the `Currency` trait to have smart-contract functionality. It can
//! be used with other modules that implement accounts based on `Currency`. These "smart-contract accounts"
//! have the ability to create smart-contracts and make calls to other contract and non-contract accounts.
//!
//! The smart-contract code is stored once in a `code_cache`, and later retrievable via its `code_hash`.
//! This means that multiple smart-contracts can be instantiated from the same `code_cache`, without replicating
//! the code each time.
//!
//! When a smart-contract is called, its associated code is retrieved via the code hash and gets executed.
//! This call can alter the storage entries of the smart-contract account, create new smart-contracts,
//! or call other smart-contracts.
//!
//! Finally, when an account is reaped, its associated code and storage of the smart-contract account
//! will also be deleted.
//!
//! ### Gas
//!
//! Senders must specify a gas limit with every call, as all instructions invoked by the smart-contract require gas.
//! Unused gas is refunded after the call, regardless of the execution outcome.
//!
//! If the gas limit is reached, then all calls and state changes (including balance transfers) are only
//! reverted at the current call's contract level. For example, if contract A calls B and B runs out of gas mid-call,
//! then all of B's calls are reverted. Assuming correct error handling by contract A, A's other calls and state
//! changes still persist.
//!
//! ### Notable Scenarios
//!
//! Contract call failures are not always cascading. When failures occur in a sub-call, they do not "bubble up",
//! and the call will only revert at the specific contract level. For example, if contract A calls contract B, and B
//! fails, A can decide how to handle that failure, either proceeding or reverting A's changes.
//!
//! ## Interface
//!
//! ### Dispatchable functions
//!
//! * `put_code` - Stores the given binary Wasm code into the chain's storage and returns its `code_hash`.
//! * `create` - Deploys a new contract from the given `code_hash`, optionally transferring some balance.
//! This creates a new smart contract account and calls its contract deploy handler to initialize the contract.
//! * `call` - Makes a call to an account, optionally transferring some balance.
//!
//! ## Usage
//!
//! The Contract module is a work in progress. The following examples show how this Contract module can be
//! used to create and call contracts.
//!
//! * [`pDSL`](https://github.com/Robbepop/pdsl) is a domain specific language that enables writing
//! WebAssembly based smart contracts in the Rust programming language. This is a work in progress.
//!
//! ## Related Modules
//!
//! * [Balances](../srml_balances/index.html)
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[macro_use]
mod gas {












    // Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.





    // Note that skipping a value due to error is not an issue here.
    // We only need uniqueness, not sequence.


    // TODO: see https://github.com/paritytech/substrate/issues/2325





    // `As<u32>` is needed for wasm-utils



















    // Pay for the gas upfront.
    //
    // NOTE: it is very important to avoid any state changes before
    // paying for the gas.



    // Commit all changes that made it thus far into the persistent storage.

    // Then deposit all events produced.

    // Refund cost of the unused gas.
    //
    // NOTE: This should go after the commit to the storage, since the storage changes
    // can alter the balance of the caller.

    // Dispatch every recorded call with an appropriate origin.



    // Commit the gas upfront.
    //
    // NOTE: It is very important to avoid any state changes before
    // paying for the gas.


    // Commit all changes that made it thus far into the persistent storage.

    // Then deposit all events produced.

    // Refund cost of the unused gas.
    //
    // NOTE: This should go after the commit to the storage, since the storage changes
    // can alter the balance of the caller.

    // Dispatch every recorded call with an appropriate origin.



    // Add some advantage for block producers (who send unsigned extrinsics) by
    // adding a handicap: for signed extrinsics we use a slightly older block number
    // for the eviction check. This can be viewed as if we pushed regular users back in past.

    // If poking the contract has lead to eviction of the contract, give out the rewards.


























    use crate::{GasSpent, Module, Trait, BalanceOf, NegativeImbalanceOf};
    use runtime_primitives::BLOCK_FULL;
    use runtime_primitives::traits::{As, CheckedMul, CheckedSub, Zero};
    use srml_support::{StorageValue,
                       traits::{OnUnbalanced, ExistenceRequirement,
                                WithdrawReason, Currency, Imbalance}};
    #[must_use]
    #[structural_match]
    pub enum GasMeterResult { Proceed, OutOfGas, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for GasMeterResult {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&GasMeterResult::Proceed,) => {
                    let mut debug_trait_builder = f.debug_tuple("Proceed");
                    debug_trait_builder.finish()
                }
                (&GasMeterResult::OutOfGas,) => {
                    let mut debug_trait_builder = f.debug_tuple("OutOfGas");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for GasMeterResult {
        #[inline]
        fn eq(&self, other: &GasMeterResult) -> bool {
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
    impl ::std::cmp::Eq for GasMeterResult {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    impl GasMeterResult {
        pub fn is_out_of_gas(&self) -> bool {
            match *self {
                GasMeterResult::OutOfGas => true,
                GasMeterResult::Proceed => false,
            }
        }
    }
    #[cfg(not(test))]
    pub trait TestAuxiliaries { }
    #[cfg(not(test))]
    impl <T> TestAuxiliaries for T { }
    /// This trait represents a token that can be used for charging `GasMeter`.
    /// There is no other way of charging it.
    ///
    /// Implementing type is expected to be super lightweight hence `Copy` (`Clone` is added
    /// for consistency). If inlined there should be no observable difference compared
    /// to a hand-written code.
    pub trait Token<T: Trait>: Copy + Clone + TestAuxiliaries {
        /// Metadata type, which the token can require for calculating the amount
        /// of gas to charge. Can be a some configuration type or
        /// just the `()`.
        type
        Metadata;
        /// Calculate amount of gas that should be taken by this token.
        ///
        /// This function should be really lightweight and must not fail. It is not
        /// expected that implementors will query the storage or do any kinds of heavy operations.
        ///
        /// That said, implementors of this function still can run into overflows
        /// while calculating the amount. In this case it is ok to use saturating operations
        /// since on overflow they will return `max_value` which should consume all gas.
        fn calculate_amount(&self, metadata: &Self::Metadata)
        -> T::Gas;
    }
    pub struct GasMeter<T: Trait> {
        limit: T::Gas,
        /// Amount of gas left from initial gas limit. Can reach zero.
        gas_left: T::Gas,
        gas_price: BalanceOf<T>,
    }
    impl <T: Trait> GasMeter<T> {
        /// Account for used gas.
        ///
        /// Amount is calculated by the given `token`.
        ///
        /// Returns `OutOfGas` if there is not enough gas or addition of the specified
        /// amount of gas has lead to overflow. On success returns `Proceed`.
        ///
        /// NOTE that amount is always consumed, i.e. if there is not enough gas
        /// then the counter will be set to zero.
        #[inline]
        pub fn charge<Tok: Token<T>>(&mut self, metadata: &Tok::Metadata,
                                     token: Tok) -> GasMeterResult {
            let amount = token.calculate_amount(metadata);
            let new_value =
                match self.gas_left.checked_sub(&amount) {
                    None => None,
                    Some(val) if val.is_zero() => None,
                    Some(val) => Some(val),
                };
            self.gas_left = new_value.unwrap_or_else(Zero::zero);
            match new_value {
                Some(_) => GasMeterResult::Proceed,
                None => GasMeterResult::OutOfGas,
            }
        }
        /// Allocate some amount of gas and perform some work with
        /// a newly created nested gas meter.
        ///
        /// Invokes `f` with either the gas meter that has `amount` gas left or
        /// with `None`, if this gas meter has not enough gas to allocate given `amount`.
        ///
        /// All unused gas in the nested gas meter is returned to this gas meter.
        pub fn with_nested<R, F: FnOnce(Option<&mut GasMeter<T>>) ->
                           R>(&mut self, amount: T::Gas, f: F) -> R {
            if self.gas_left < amount {
                f(None)
            } else {
                self.gas_left = self.gas_left - amount;
                let mut nested =
                    GasMeter{limit: amount,
                             gas_left: amount,
                             gas_price: self.gas_price,};
                let r = f(Some(&mut nested));
                self.gas_left = self.gas_left + nested.gas_left;
                r
            }
        }
        pub fn gas_price(&self) -> BalanceOf<T> { self.gas_price }
        /// Returns how much gas left from the initial budget.
        pub fn gas_left(&self) -> T::Gas { self.gas_left }
        /// Returns how much gas was spent.
        fn spent(&self) -> T::Gas { self.limit - self.gas_left }
    }
    /// Buy the given amount of gas.
    ///
    /// Cost is calculated by multiplying the gas cost (taken from the storage) by the `gas_limit`.
    /// The funds are deducted from `transactor`.
    pub fn buy_gas<T: Trait>(transactor: &T::AccountId, gas_limit: T::Gas)
     -> Result<(GasMeter<T>, NegativeImbalanceOf<T>), &'static str> {
        let gas_available =
            <Module<T>>::block_gas_limit() - <Module<T>>::gas_spent();
        if gas_limit > gas_available { return Err(BLOCK_FULL); }
        let gas_price = <Module<T>>::gas_price();
        let cost =
            <T::Gas as
                As<BalanceOf<T>>>::as_(gas_limit.clone()).checked_mul(&gas_price).ok_or("overflow multiplying gas limit by price")?;
        let imbalance =
            T::Currency::withdraw(transactor, cost, WithdrawReason::Fee,
                                  ExistenceRequirement::KeepAlive)?;
        Ok((GasMeter{limit: gas_limit, gas_left: gas_limit, gas_price,},
            imbalance))
    }
    /// Refund the unused gas.
    pub fn refund_unused_gas<T: Trait>(transactor: &T::AccountId,
                                       gas_meter: GasMeter<T>,
                                       imbalance: NegativeImbalanceOf<T>) {
        let gas_spent = gas_meter.spent();
        let gas_left = gas_meter.gas_left();
        <GasSpent<T>>::mutate(|block_gas_spent|
                                  *block_gas_spent += gas_spent);
        let refund =
            <T::Gas as As<BalanceOf<T>>>::as_(gas_left) * gas_meter.gas_price;
        let refund_imbalance =
            T::Currency::deposit_creating(transactor, refund);
        if let Ok(imbalance) = imbalance.offset(refund_imbalance) {
            T::GasPayment::on_unbalanced(imbalance);
        }
    }
    /// A little handy utility for converting a value in balance units into approximate value in gas units
    /// at the given gas price.
    pub fn approx_gas_for_balance<T: Trait>(gas_price: BalanceOf<T>,
                                            balance: BalanceOf<T>) -> T::Gas {
        let amount_in_gas: BalanceOf<T> = balance / gas_price;
        <T::Gas as As<BalanceOf<T>>>::sa(amount_in_gas)
    }
    /// A simple utility macro that helps to match against a
    /// list of tokens.
    #[macro_export]
    macro_rules! match_tokens(( $ tokens_iter : ident , ) => {  } ; (
                              $ tokens_iter : ident , $ x : expr , $ (
                              $ rest : tt ) * ) => {
                              {
                              let next = ( $ tokens_iter ) . next (  ) .
                              unwrap (  ) ; let pattern = $ x ; let mut
                              _pattern_typed_next_ref = & pattern ;
                              _pattern_typed_next_ref = match next . token .
                              downcast_ref (  ) {
                              Some ( p ) => {
                              assert_eq ! ( p , & pattern ) ; p } None => {
                              panic ! (
                              "expected type {} got {}" , stringify ! ( $ x )
                              , next . description ) ; } } ; } match_tokens !
                              ( $ tokens_iter , $ ( $ rest ) * ) ; } ;);
}
mod account_db {
    //! Auxilliaries to help with managing partial changes to accounts state.
    use super::{AliveContractInfo, BalanceOf, CodeHash, ContractInfo,
                ContractInfoOf, Module, Trait, TrieId, TrieIdGenerator};
    use crate::exec::StorageKey;
    use rstd::cell::RefCell;
    use rstd::collections::btree_map::{BTreeMap, Entry};
    use rstd::prelude::*;
    use runtime_io::blake2_256;
    use runtime_primitives::traits::{Bounded, Zero};
    use srml_support::traits::{Currency, Imbalance, SignedImbalance,
                               UpdateBalanceOutcome};
    use srml_support::{storage::child, StorageMap};
    use system;
    pub struct ChangeEntry<T: Trait> {
        balance: Option<BalanceOf<T>>,
        /// If None, the code_hash remains untouched.
        code_hash: Option<CodeHash<T>>,
        rent_allowance: Option<BalanceOf<T>>,
        storage: BTreeMap<StorageKey, Option<Vec<u8>>>,
    }
    impl <T: Trait> Default for ChangeEntry<T> {
        fn default() -> Self {
            ChangeEntry{rent_allowance: Default::default(),
                        balance: Default::default(),
                        code_hash: Default::default(),
                        storage: Default::default(),}
        }
    }
    pub type ChangeSet<T>
        =
        BTreeMap<<T as system::Trait>::AccountId, ChangeEntry<T>>;
    pub trait AccountDb<T: Trait> {
        /// Account is used when overlayed otherwise trie_id must be provided.
        /// This is for performance reason.
        ///
        /// Trie id is None iff account doesn't have an associated trie id in <ContractInfoOf<T>>.
        /// Because DirectAccountDb bypass the lookup for this association.
        fn get_storage(&self, account: &T::AccountId,
                       trie_id: Option<&TrieId>, location: &StorageKey)
        -> Option<Vec<u8>>;
        /// If account has an alive contract then return the code hash associated.
        fn get_code_hash(&self, account: &T::AccountId)
        -> Option<CodeHash<T>>;
        /// If account has an alive contract then return the rent allowance associated.
        fn get_rent_allowance(&self, account: &T::AccountId)
        -> Option<BalanceOf<T>>;
        /// Returns false iff account has no alive contract nor tombstone.
        fn contract_exists(&self, account: &T::AccountId)
        -> bool;
        fn get_balance(&self, account: &T::AccountId)
        -> BalanceOf<T>;
        fn commit(&mut self, change_set: ChangeSet<T>);
    }
    pub struct DirectAccountDb;
    impl <T: Trait> AccountDb<T> for DirectAccountDb {
        fn get_storage(&self, _account: &T::AccountId,
                       trie_id: Option<&TrieId>, location: &StorageKey)
         -> Option<Vec<u8>> {
            trie_id.and_then(|id| child::get_raw(id, &blake2_256(location)))
        }
        fn get_code_hash(&self, account: &T::AccountId)
         -> Option<CodeHash<T>> {
            <ContractInfoOf<T>>::get(account).and_then(|i|
                                                           i.as_alive().map(|i|
                                                                                i.code_hash))
        }
        fn get_rent_allowance(&self, account: &T::AccountId)
         -> Option<BalanceOf<T>> {
            <ContractInfoOf<T>>::get(account).and_then(|i|
                                                           i.as_alive().map(|i|
                                                                                i.rent_allowance))
        }
        fn contract_exists(&self, account: &T::AccountId) -> bool {
            <ContractInfoOf<T>>::exists(account)
        }
        fn get_balance(&self, account: &T::AccountId) -> BalanceOf<T> {
            T::Currency::free_balance(account)
        }
        fn commit(&mut self, s: ChangeSet<T>) {
            let mut total_imbalance = SignedImbalance::zero();
            for (address, changed) in s.into_iter() {
                if let Some(balance) = changed.balance {
                    let (imbalance, outcome) =
                        T::Currency::make_free_balance_be(&address, balance);
                    total_imbalance = total_imbalance.merge(imbalance);
                    if let UpdateBalanceOutcome::AccountKilled = outcome {
                        continue ;
                    }
                }
                if changed.code_hash.is_some() ||
                       changed.rent_allowance.is_some() ||
                       !changed.storage.is_empty() {
                    let old_info =
                        match <ContractInfoOf<T>>::get(&address) {
                            Some(ContractInfo::Alive(alive)) => Some(alive),
                            None => None,
                            Some(ContractInfo::Tombstone(_)) => continue ,
                        };
                    let mut new_info =
                        if let Some(info) = old_info.clone() {
                            info
                        } else if let Some(code_hash) = changed.code_hash {
                            AliveContractInfo::<T>{code_hash,
                                                   storage_size:
                                                       <Module<T>>::storage_size_offset(),
                                                   trie_id:
                                                       <T as
                                                           Trait>::trie_id(&address),
                                                   deduct_block:
                                                       <system::Module<T>>::block_number(),
                                                   rent_allowance:
                                                       <BalanceOf<T>>::max_value(),}
                        } else { continue ; };
                    if let Some(rent_allowance) = changed.rent_allowance {
                        new_info.rent_allowance = rent_allowance;
                    }
                    if let Some(code_hash) = changed.code_hash {
                        new_info.code_hash = code_hash;
                    }
                    for (k, v) in changed.storage.into_iter() {
                        if let Some(value) =
                               child::get_raw(&new_info.trie_id[..],
                                              &blake2_256(&k)) {
                            new_info.storage_size -= value.len() as u64;
                        }
                        if let Some(value) = v {
                            new_info.storage_size += value.len() as u64;
                            child::put_raw(&new_info.trie_id[..],
                                           &blake2_256(&k), &value[..]);
                        } else {
                            child::kill(&new_info.trie_id[..],
                                        &blake2_256(&k));
                        }
                    }
                    if old_info.map(|old_info|
                                        old_info != new_info).unwrap_or(true)
                       {
                        <ContractInfoOf<T>>::insert(&address,
                                                    ContractInfo::Alive(new_info));
                    }
                }
            }
            match total_imbalance {
                SignedImbalance::Positive(ref p) if !p.peek().is_zero() => {
                    ::std::rt::begin_panic("contract subsystem resulting in positive imbalance!",
                                           &("srml/contract/src/account_db.rs",
                                             168u32, 5u32))
                }
                _ => { }
            }
        }
    }
    pub struct OverlayAccountDb<'a, T: Trait + 'a> {
        local: RefCell<ChangeSet<T>>,
        underlying: &'a AccountDb<T>,
    }
    impl <'a, T: Trait> OverlayAccountDb<'a, T> {
        pub fn new(underlying: &'a AccountDb<T>) -> OverlayAccountDb<'a, T> {
            OverlayAccountDb{local: RefCell::new(ChangeSet::new()),
                             underlying,}
        }
        pub fn into_change_set(self) -> ChangeSet<T> {
            self.local.into_inner()
        }
        pub fn set_storage(&mut self, account: &T::AccountId,
                           location: StorageKey, value: Option<Vec<u8>>) {
            self.local.borrow_mut().entry(account.clone()).or_insert(Default::default()).storage.insert(location,
                                                                                                        value);
        }
        /// Return an error if contract already exists (either if it is alive or tombstone)
        pub fn create_contract(&mut self, account: &T::AccountId,
                               code_hash: CodeHash<T>)
         -> Result<(), &'static str> {
            if self.contract_exists(account) {
                return Err("Alive contract or tombstone already exists");
            }
            let mut local = self.local.borrow_mut();
            let contract =
                local.entry(account.clone()).or_insert_with(||
                                                                Default::default());
            contract.code_hash = Some(code_hash);
            contract.rent_allowance = Some(<BalanceOf<T>>::max_value());
            Ok(())
        }
        /// Assume contract exists
        pub fn set_rent_allowance(&mut self, account: &T::AccountId,
                                  rent_allowance: BalanceOf<T>) {
            self.local.borrow_mut().entry(account.clone()).or_insert(Default::default()).rent_allowance
                = Some(rent_allowance);
        }
        pub fn set_balance(&mut self, account: &T::AccountId,
                           balance: BalanceOf<T>) {
            self.local.borrow_mut().entry(account.clone()).or_insert(Default::default()).balance
                = Some(balance);
        }
    }
    impl <'a, T: Trait> AccountDb<T> for OverlayAccountDb<'a, T> {
        fn get_storage(&self, account: &T::AccountId,
                       trie_id: Option<&TrieId>, location: &StorageKey)
         -> Option<Vec<u8>> {
            self.local.borrow().get(account).and_then(|a|
                                                          a.storage.get(location)).cloned().unwrap_or_else(||
                                                                                                               self.underlying.get_storage(account,
                                                                                                                                           trie_id,
                                                                                                                                           location))
        }
        fn get_code_hash(&self, account: &T::AccountId)
         -> Option<CodeHash<T>> {
            self.local.borrow().get(account).and_then(|changes|
                                                          changes.code_hash).or_else(||
                                                                                         self.underlying.get_code_hash(account))
        }
        fn get_rent_allowance(&self, account: &T::AccountId)
         -> Option<BalanceOf<T>> {
            self.local.borrow().get(account).and_then(|changes|
                                                          changes.rent_allowance).or_else(||
                                                                                              self.underlying.get_rent_allowance(account))
        }
        fn contract_exists(&self, account: &T::AccountId) -> bool {
            self.local.borrow().get(account).map(|a|
                                                     a.code_hash.is_some()).unwrap_or_else(||
                                                                                               self.underlying.contract_exists(account))
        }
        fn get_balance(&self, account: &T::AccountId) -> BalanceOf<T> {
            self.local.borrow().get(account).and_then(|a|
                                                          a.balance).unwrap_or_else(||
                                                                                        self.underlying.get_balance(account))
        }
        fn commit(&mut self, s: ChangeSet<T>) {
            let mut local = self.local.borrow_mut();
            for (address, changed) in s.into_iter() {
                match local.entry(address) {
                    Entry::Occupied(e) => {
                        let mut value = e.into_mut();
                        value.balance = changed.balance.or(value.balance);
                        value.code_hash =
                            changed.code_hash.or(value.code_hash);
                        value.rent_allowance =
                            changed.rent_allowance.or(value.rent_allowance);
                        value.storage.extend(changed.storage.into_iter());
                    }
                    Entry::Vacant(e) => { e.insert(changed); }
                }
            }
        }
    }
}
mod exec {
    use super::{CodeHash, Config, ContractAddressFor, Event, RawEvent, Trait,
                TrieId, BalanceOf, ContractInfoOf};
    use crate::account_db::{AccountDb, DirectAccountDb, OverlayAccountDb};
    use crate::gas::{GasMeter, Token, approx_gas_for_balance};
    use rstd::prelude::*;
    use runtime_primitives::traits::{Bounded, CheckedAdd, CheckedSub, Zero};
    use srml_support::{StorageMap, traits::{WithdrawReason, Currency}};
    use timestamp;
    pub type AccountIdOf<T> = <T as system::Trait>::AccountId;
    pub type CallOf<T> = <T as Trait>::Call;
    pub type MomentOf<T> = <T as timestamp::Trait>::Moment;
    pub type SeedOf<T> = <T as system::Trait>::Hash;
    /// A type that represents a topic of an event. At the moment a hash is used.
    pub type TopicOf<T> = <T as system::Trait>::Hash;
    pub struct InstantiateReceipt<AccountId> {
        pub address: AccountId,
    }
    pub struct CallReceipt {
        /// Output data received as a result of a call.
        pub output_data: Vec<u8>,
    }
    pub type StorageKey = [u8; 32];
    /// An interface that provides access to the external environment in which the
    /// smart-contract is executed.
    ///
    /// This interface is specialized to an account of the executing code, so all
    /// operations are implicitly performed on that account.
    pub trait Ext {
        type
        T: Trait;
        /// Returns the storage entry of the executing account by the given `key`.
        ///
        /// Returns `None` if the `key` wasn't previously set by `set_storage` or
        /// was deleted.
        fn get_storage(&self, key: &StorageKey)
        -> Option<Vec<u8>>;
        /// Sets the storage entry by the given key to the specified value.
        ///
        /// If `value` is `None` then the storage entry is deleted.
        fn set_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>);
        /// Instantiate a contract from the given code.
        ///
        /// The newly created account will be associated with `code`. `value` specifies the amount of value
        /// transfered from this to the newly created account (also known as endowment).
        fn instantiate(&mut self, code: &CodeHash<Self::T>,
                       value: BalanceOf<Self::T>,
                       gas_meter: &mut GasMeter<Self::T>, input_data: &[u8])
        -> Result<InstantiateReceipt<AccountIdOf<Self::T>>, &'static str>;
        /// Call (possibly transfering some amount of funds) into the specified account.
        fn call(&mut self, to: &AccountIdOf<Self::T>,
                value: BalanceOf<Self::T>, gas_meter: &mut GasMeter<Self::T>,
                input_data: &[u8], empty_output_buf: EmptyOutputBuf)
        -> Result<CallReceipt, &'static str>;
        /// Notes a call dispatch.
        fn note_dispatch_call(&mut self, call: CallOf<Self::T>);
        /// Returns a reference to the account id of the caller.
        fn caller(&self)
        -> &AccountIdOf<Self::T>;
        /// Returns a reference to the account id of the current contract.
        fn address(&self)
        -> &AccountIdOf<Self::T>;
        /// Returns the balance of the current contract.
        ///
        /// The `value_transferred` is already added.
        fn balance(&self)
        -> BalanceOf<Self::T>;
        /// Returns the value transfered along with this call or as endowment.
        fn value_transferred(&self)
        -> BalanceOf<Self::T>;
        /// Returns a reference to the timestamp of the current block
        fn now(&self)
        -> &MomentOf<Self::T>;
        /// Returns a reference to the random seed for the current block
        fn random_seed(&self)
        -> &SeedOf<Self::T>;
        /// Deposit an event with the given topics.
        ///
        /// There should not be any duplicates in `topics`.
        fn deposit_event(&mut self, topics: Vec<TopicOf<Self::T>>,
                         data: Vec<u8>);
        /// Set rent allowance of the contract
        fn set_rent_allowance(&mut self, rent_allowance: BalanceOf<Self::T>);
        /// Rent allowance of the contract
        fn rent_allowance(&self)
        -> BalanceOf<Self::T>;
    }
    /// Loader is a companion of the `Vm` trait. It loads an appropriate abstract
    /// executable to be executed by an accompanying `Vm` implementation.
    pub trait Loader<T: Trait> {
        type
        Executable;
        /// Load the initializer portion of the code specified by the `code_hash`. This
        /// executable is called upon instantiation.
        fn load_init(&self, code_hash: &CodeHash<T>)
        -> Result<Self::Executable, &'static str>;
        /// Load the main portion of the code specified by the `code_hash`. This executable
        /// is called for each call to a contract.
        fn load_main(&self, code_hash: &CodeHash<T>)
        -> Result<Self::Executable, &'static str>;
    }
    /// An `EmptyOutputBuf` is used as an optimization for reusing empty vectors when
    /// available.
    ///
    /// You can create this structure from a spare vector if you have any and then
    /// you can fill it (only once), converting it to `OutputBuf`.
    pub struct EmptyOutputBuf(Vec<u8>);
    impl EmptyOutputBuf {
        /// Create an output buffer from a spare vector which is not longer needed.
        ///
        /// All contents are discarded, but capacity is preserved.
        pub fn from_spare_vec(mut v: Vec<u8>) -> Self {
            v.clear();
            EmptyOutputBuf(v)
        }
        /// Create an output buffer ready for receiving a result.
        ///
        /// Use this function to create output buffer if you don't have a spare
        /// vector. Otherwise, use `from_spare_vec`.
        pub fn new() -> Self { EmptyOutputBuf(Vec::new()) }
        /// Write to the buffer result of the specified size.
        ///
        /// Calls closure with the buffer of the requested size.
        pub fn fill<E, F: FnOnce(&mut [u8]) ->
                    Result<(), E>>(mut self, size: usize, f: F)
         -> Result<OutputBuf, E> {
            if !(self.0.len() == 0) {
                {
                    ::std::rt::begin_panic("the vector is always cleared; it's written only once",
                                           &("srml/contract/src/exec.rs",
                                             165u32, 3u32))
                }
            };
            self.0.resize(size, 0);
            f(&mut self.0).map(|()| OutputBuf(self.0))
        }
    }
    /// `OutputBuf` is the end result of filling an `EmptyOutputBuf`.
    pub struct OutputBuf(Vec<u8>);
    #[must_use]
    pub enum VmExecResult {
        Ok,
        Returned(OutputBuf),

        /// A program executed some forbidden operation.
        ///
        /// This can include, e.g.: division by 0, OOB access or failure to satisfy some precondition
        /// of a system call.
        ///
        /// Contains some vm-specific description of an trap.
        Trap(&'static str),
    }
    impl VmExecResult {
        pub fn into_result(self) -> Result<Vec<u8>, &'static str> {
            match self {
                VmExecResult::Ok => Ok(Vec::new()),
                VmExecResult::Returned(buf) => Ok(buf.0),
                VmExecResult::Trap(description) => Err(description),
            }
        }
    }
    /// Struct that records a request to deposit an event with a list of topics.
    #[structural_match]
    pub struct IndexedEvent<T: Trait> {
        /// A list of topics this event will be deposited with.
        pub topics: Vec<T::Hash>,
        /// The event to deposit.
        pub event: Event<T>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::fmt::Debug + Trait> ::std::fmt::Debug for IndexedEvent<T>
     where T::Hash: ::std::fmt::Debug {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                IndexedEvent { topics: ref __self_0_0, event: ref __self_0_1 }
                => {
                    let mut debug_trait_builder =
                        f.debug_struct("IndexedEvent");
                    let _ =
                        debug_trait_builder.field("topics", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("event", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::cmp::PartialEq + Trait> ::std::cmp::PartialEq for
     IndexedEvent<T> where T::Hash: ::std::cmp::PartialEq {
        #[inline]
        fn eq(&self, other: &IndexedEvent<T>) -> bool {
            match *other {
                IndexedEvent { topics: ref __self_1_0, event: ref __self_1_1 }
                =>
                match *self {
                    IndexedEvent {
                    topics: ref __self_0_0, event: ref __self_0_1 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &IndexedEvent<T>) -> bool {
            match *other {
                IndexedEvent { topics: ref __self_1_0, event: ref __self_1_1 }
                =>
                match *self {
                    IndexedEvent {
                    topics: ref __self_0_0, event: ref __self_0_1 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::cmp::Eq + Trait> ::std::cmp::Eq for IndexedEvent<T> where
     T::Hash: ::std::cmp::Eq {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<Vec<T::Hash>>;
                let _: ::std::cmp::AssertParamIsEq<Event<T>>;
            }
        }
    }
    /// A trait that represent a virtual machine.
    ///
    /// You can view a virtual machine as something that takes code, an input data buffer,
    /// queries it and/or performs actions on the given `Ext` and optionally
    /// returns an output data buffer. The type of code depends on the particular virtual machine.
    ///
    /// Execution of code can end by either implicit termination (that is, reached the end of
    /// executable), explicit termination via returning a buffer or termination due to a trap.
    ///
    /// You can optionally provide a vector for collecting output if a spare is available. If you don't have
    /// it will be created anyway.
    pub trait Vm<T: Trait> {
        type
        Executable;
        fn execute<E: Ext<T =
                   T>>(&self, exec: &Self::Executable, ext: &mut E,
                       input_data: &[u8], empty_output_buf: EmptyOutputBuf,
                       gas_meter: &mut GasMeter<T>)
        -> VmExecResult;
    }
    #[rustc_copy_clone_marker]
    pub enum ExecFeeToken {

        /// Base fee charged for a call.
        Call,

        /// Base fee charged for a instantiate.
        Instantiate,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for ExecFeeToken { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ExecFeeToken {
        #[inline]
        fn clone(&self) -> ExecFeeToken { { *self } }
    }
    impl <T: Trait> Token<T> for ExecFeeToken {
        type
        Metadata
        =
        Config<T>;
        #[inline]
        fn calculate_amount(&self, metadata: &Config<T>) -> T::Gas {
            match *self {
                ExecFeeToken::Call => metadata.call_base_fee,
                ExecFeeToken::Instantiate => metadata.instantiate_base_fee,
            }
        }
    }
    pub struct ExecutionContext<'a, T: Trait + 'a, V, L> {
        pub self_account: T::AccountId,
        pub self_trie_id: Option<TrieId>,
        pub overlay: OverlayAccountDb<'a, T>,
        pub depth: usize,
        pub events: Vec<IndexedEvent<T>>,
        pub calls: Vec<(T::AccountId, T::Call)>,
        pub config: &'a Config<T>,
        pub vm: &'a V,
        pub loader: &'a L,
    }
    impl <'a, T, E, V, L> ExecutionContext<'a, T, V, L> where T: Trait,
     L: Loader<T, Executable = E>, V: Vm<T, Executable = E> {
        /// Create the top level execution context.
        ///
        /// The specified `origin` address will be used as `sender` for
        pub fn top_level(origin: T::AccountId, cfg: &'a Config<T>, vm: &'a V,
                         loader: &'a L) -> Self {
            ExecutionContext{self_trie_id:
                                 <ContractInfoOf<T>>::get(&origin).and_then(|i|
                                                                                i.as_alive().map(|i|
                                                                                                     i.trie_id.clone())),
                             self_account: origin,
                             overlay:
                                 OverlayAccountDb::<T>::new(&DirectAccountDb),
                             depth: 0,
                             events: Vec::new(),
                             calls: Vec::new(),
                             config: &cfg,
                             vm: &vm,
                             loader: &loader,}
        }
        fn nested(&self, overlay: OverlayAccountDb<'a, T>, dest: T::AccountId)
         -> Self {
            ExecutionContext{self_trie_id:
                                 <ContractInfoOf<T>>::get(&dest).and_then(|i|
                                                                              i.as_alive().map(|i|
                                                                                                   i.trie_id.clone())),
                             self_account: dest,
                             overlay,
                             depth: self.depth + 1,
                             events: Vec::new(),
                             calls: Vec::new(),
                             config: self.config,
                             vm: self.vm,
                             loader: self.loader,}
        }
        /// Make a call to the specified address, optionally transfering some funds.
        pub fn call(&mut self, dest: T::AccountId, value: BalanceOf<T>,
                    gas_meter: &mut GasMeter<T>, input_data: &[u8],
                    empty_output_buf: EmptyOutputBuf)
         -> Result<CallReceipt, &'static str> {
            if self.depth == self.config.max_depth as usize {
                return Err("reached maximum depth, cannot make a call");
            }
            if gas_meter.charge(self.config,
                                ExecFeeToken::Call).is_out_of_gas() {
                return Err("not enough gas to pay base call fee");
            }
            crate::rent::pay_rent::<T>(&dest);
            let mut output_data = Vec::new();
            let (change_set, events, calls) =
                {
                    let mut nested =
                        self.nested(OverlayAccountDb::new(&self.overlay),
                                    dest.clone());
                    if value > BalanceOf::<T>::zero() {
                        transfer(gas_meter, TransferCause::Call,
                                 &self.self_account, &dest, value,
                                 &mut nested)?;
                    }
                    if let Some(dest_code_hash) =
                           self.overlay.get_code_hash(&dest) {
                        let executable =
                            self.loader.load_main(&dest_code_hash)?;
                        output_data =
                            self.vm.execute(&executable,
                                            &mut CallContext{ctx: &mut nested,
                                                             caller:
                                                                 self.self_account.clone(),
                                                             value_transferred:
                                                                 value,
                                                             timestamp:
                                                                 timestamp::Module::<T>::now(),
                                                             random_seed:
                                                                 system::Module::<T>::random_seed(),},
                                            input_data, empty_output_buf,
                                            gas_meter).into_result()?;
                    }
                    (nested.overlay.into_change_set(), nested.events,
                     nested.calls)
                };
            self.overlay.commit(change_set);
            self.events.extend(events);
            self.calls.extend(calls);
            Ok(CallReceipt{output_data,})
        }
        pub fn instantiate(&mut self, endowment: BalanceOf<T>,
                           gas_meter: &mut GasMeter<T>,
                           code_hash: &CodeHash<T>, input_data: &[u8])
         -> Result<InstantiateReceipt<T::AccountId>, &'static str> {
            if self.depth == self.config.max_depth as usize {
                return Err("reached maximum depth, cannot create");
            }
            if gas_meter.charge(self.config,
                                ExecFeeToken::Instantiate).is_out_of_gas() {
                return Err("not enough gas to pay base instantiate fee");
            }
            let dest =
                T::DetermineContractAddress::contract_address_for(code_hash,
                                                                  input_data,
                                                                  &self.self_account);
            let (change_set, events, calls) =
                {
                    let mut overlay = OverlayAccountDb::new(&self.overlay);
                    overlay.create_contract(&dest, code_hash.clone())?;
                    let mut nested = self.nested(overlay, dest.clone());
                    transfer(gas_meter, TransferCause::Instantiate,
                             &self.self_account, &dest, endowment,
                             &mut nested)?;
                    let executable = self.loader.load_init(&code_hash)?;
                    self.vm.execute(&executable,
                                    &mut CallContext{ctx: &mut nested,
                                                     caller:
                                                         self.self_account.clone(),
                                                     value_transferred:
                                                         endowment,
                                                     timestamp:
                                                         timestamp::Module::<T>::now(),
                                                     random_seed:
                                                         system::Module::<T>::random_seed(),},
                                    input_data, EmptyOutputBuf::new(),
                                    gas_meter).into_result()?;
                    nested.events.push(IndexedEvent{event:
                                                        RawEvent::Instantiated(self.self_account.clone(),
                                                                               dest.clone()),
                                                    topics: Vec::new(),});
                    (nested.overlay.into_change_set(), nested.events,
                     nested.calls)
                };
            self.overlay.commit(change_set);
            self.events.extend(events);
            self.calls.extend(calls);
            Ok(InstantiateReceipt{address: dest,})
        }
    }
    #[rustc_copy_clone_marker]
    pub enum TransferFeeKind { ContractInstantiate, AccountCreate, Transfer, }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for TransferFeeKind { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for TransferFeeKind {
        #[inline]
        fn clone(&self) -> TransferFeeKind { { *self } }
    }
    #[rustc_copy_clone_marker]
    pub struct TransferFeeToken<Balance> {
        kind: TransferFeeKind,
        gas_price: Balance,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Balance: ::std::marker::Copy> ::std::marker::Copy for
     TransferFeeToken<Balance> {
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Balance: ::std::clone::Clone> ::std::clone::Clone for
     TransferFeeToken<Balance> {
        #[inline]
        fn clone(&self) -> TransferFeeToken<Balance> {
            match *self {
                TransferFeeToken {
                kind: ref __self_0_0, gas_price: ref __self_0_1 } =>
                TransferFeeToken{kind:
                                     ::std::clone::Clone::clone(&(*__self_0_0)),
                                 gas_price:
                                     ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    impl <T: Trait> Token<T> for TransferFeeToken<BalanceOf<T>> {
        type
        Metadata
        =
        Config<T>;
        #[inline]
        fn calculate_amount(&self, metadata: &Config<T>) -> T::Gas {
            let balance_fee =
                match self.kind {
                    TransferFeeKind::ContractInstantiate =>
                    metadata.contract_account_instantiate_fee,
                    TransferFeeKind::AccountCreate =>
                    metadata.account_create_fee,
                    TransferFeeKind::Transfer => metadata.transfer_fee,
                };
            approx_gas_for_balance::<T>(self.gas_price, balance_fee)
        }
    }
    /// Describes possible transfer causes.
    enum TransferCause { Call, Instantiate, }
    /// Transfer some funds from `transactor` to `dest`.
    ///
    /// All balance changes are performed in the `overlay`.
    ///
    /// This function also handles charging the fee. The fee depends
    /// on whether the transfer happening because of contract instantiation
    /// (transfering endowment) or because of a transfer via `call`. This
    /// is specified using the `cause` parameter.
    ///
    /// NOTE: that the fee is denominated in `BalanceOf<T>` units, but
    /// charged in `T::Gas` from the provided `gas_meter`. This means
    /// that the actual amount charged might differ.
    ///
    /// NOTE: that we allow for draining all funds of the contract so it
    /// can go below existential deposit, essentially giving a contract
    /// the chance to give up it's life.
    fn transfer<'a, T: Trait, V: Vm<T>,
                L: Loader<T>>(gas_meter: &mut GasMeter<T>,
                              cause: TransferCause, transactor: &T::AccountId,
                              dest: &T::AccountId, value: BalanceOf<T>,
                              ctx: &mut ExecutionContext<'a, T, V, L>)
     -> Result<(), &'static str> {
        use self::TransferCause::*;
        use self::TransferFeeKind::*;
        let to_balance = ctx.overlay.get_balance(dest);
        let would_create = to_balance.is_zero();
        let token =
            {
                let kind: TransferFeeKind =
                    match cause {
                        Instantiate => ContractInstantiate,
                        Call =>
                        if would_create {
                            TransferFeeKind::AccountCreate
                        } else { TransferFeeKind::Transfer },
                    };
                TransferFeeToken{kind, gas_price: gas_meter.gas_price(),}
            };
        if gas_meter.charge(ctx.config, token).is_out_of_gas() {
            return Err("not enough gas to pay transfer fee");
        }
        let from_balance = ctx.overlay.get_balance(transactor);
        let new_from_balance =
            match from_balance.checked_sub(&value) {
                Some(b) => b,
                None => return Err("balance too low to send value"),
            };
        if would_create && value < ctx.config.existential_deposit {
            return Err("value too low to create account");
        }
        T::Currency::ensure_can_withdraw(transactor, value,
                                         WithdrawReason::Transfer,
                                         new_from_balance)?;
        let new_to_balance =
            match to_balance.checked_add(&value) {
                Some(b) => b,
                None =>
                return Err("destination balance too high to receive value"),
            };
        if transactor != dest {
            ctx.overlay.set_balance(transactor, new_from_balance);
            ctx.overlay.set_balance(dest, new_to_balance);
            ctx.events.push(IndexedEvent{event:
                                             RawEvent::Transfer(transactor.clone(),
                                                                dest.clone(),
                                                                value),
                                         topics: Vec::new(),});
        }
        Ok(())
    }
    struct CallContext<'a, 'b: 'a, T: Trait + 'b, V: Vm<T> + 'b,
                       L: Loader<T>> {
        ctx: &'a mut ExecutionContext<'b, T, V, L>,
        caller: T::AccountId,
        value_transferred: BalanceOf<T>,
        timestamp: T::Moment,
        random_seed: T::Hash,
    }
    impl <'a, 'b: 'a, T, E, V, L> Ext for CallContext<'a, 'b, T, V, L> where
     T: Trait + 'b, V: Vm<T, Executable = E>, L: Loader<T, Executable = E> {
        type
        T
        =
        T;
        fn get_storage(&self, key: &StorageKey) -> Option<Vec<u8>> {
            self.ctx.overlay.get_storage(&self.ctx.self_account,
                                         self.ctx.self_trie_id.as_ref(), key)
        }
        fn set_storage(&mut self, key: StorageKey, value: Option<Vec<u8>>) {
            self.ctx.overlay.set_storage(&self.ctx.self_account, key, value)
        }
        fn instantiate(&mut self, code_hash: &CodeHash<T>,
                       endowment: BalanceOf<T>, gas_meter: &mut GasMeter<T>,
                       input_data: &[u8])
         -> Result<InstantiateReceipt<AccountIdOf<T>>, &'static str> {
            self.ctx.instantiate(endowment, gas_meter, code_hash, input_data)
        }
        fn call(&mut self, to: &T::AccountId, value: BalanceOf<T>,
                gas_meter: &mut GasMeter<T>, input_data: &[u8],
                empty_output_buf: EmptyOutputBuf)
         -> Result<CallReceipt, &'static str> {
            self.ctx.call(to.clone(), value, gas_meter, input_data,
                          empty_output_buf)
        }
        /// Notes a call dispatch.
        fn note_dispatch_call(&mut self, call: CallOf<Self::T>) {
            self.ctx.calls.push((self.ctx.self_account.clone(), call));
        }
        fn address(&self) -> &T::AccountId { &self.ctx.self_account }
        fn caller(&self) -> &T::AccountId { &self.caller }
        fn balance(&self) -> BalanceOf<T> {
            self.ctx.overlay.get_balance(&self.ctx.self_account)
        }
        fn value_transferred(&self) -> BalanceOf<T> { self.value_transferred }
        fn random_seed(&self) -> &T::Hash { &self.random_seed }
        fn now(&self) -> &T::Moment { &self.timestamp }
        fn deposit_event(&mut self, topics: Vec<T::Hash>, data: Vec<u8>) {
            self.ctx.events.push(IndexedEvent{topics,
                                              event:
                                                  RawEvent::Contract(self.ctx.self_account.clone(),
                                                                     data),});
        }
        fn set_rent_allowance(&mut self, rent_allowance: BalanceOf<T>) {
            self.ctx.overlay.set_rent_allowance(&self.ctx.self_account,
                                                rent_allowance)
        }
        fn rent_allowance(&self) -> BalanceOf<T> {
            self.ctx.overlay.get_rent_allowance(&self.ctx.self_account).unwrap_or(<BalanceOf<T>>::max_value())
        }
    }
}
mod wasm {
    //! This module provides a means for executing contracts
    //! represented in wasm.
    use crate::{CodeHash, Schedule, Trait};
    use crate::wasm::env_def::FunctionImplProvider;
    use crate::exec::{Ext, EmptyOutputBuf, VmExecResult};
    use crate::gas::GasMeter;
    use rstd::prelude::*;
    use parity_codec::{Encode, Decode};
    use sandbox;
    #[macro_use]
    mod env_def {
        use super::Runtime;
        use crate::exec::Ext;
        use sandbox::{self, TypedValue};
        use parity_wasm::elements::{FunctionType, ValueType};
        #[macro_use]
        pub(crate) mod macros {
            //! Definition of macros that hides boilerplate of defining external environment
            //! for a wasm module.
            //!
            //! Most likely you should use `define_env` macro.
            #[macro_export]
            macro_rules! convert_args((  ) => ( vec ! [  ] ) ; (
                                      $ ( $ t : ty ) , * ) => (
                                      vec ! [
                                      $ (
                                      {
                                      use $ crate :: wasm :: env_def ::
                                      ConvertibleToWasm ; < $ t > ::
                                      VALUE_TYPE } , ) * ] ) ;);
            #[macro_export]
            macro_rules! gen_signature(( ( $ ( $ params : ty ) , * ) ) => (
                                       {
                                       parity_wasm :: elements :: FunctionType
                                       :: new (
                                       convert_args ! ( $ ( $ params ) , * ) ,
                                       None ) } ) ; (
                                       ( $ ( $ params : ty ) , * ) -> $
                                       returns : ty ) => (
                                       {
                                       parity_wasm :: elements :: FunctionType
                                       :: new (
                                       convert_args ! ( $ ( $ params ) , * ) ,
                                       Some (
                                       {
                                       use $ crate :: wasm :: env_def ::
                                       ConvertibleToWasm ; < $ returns > ::
                                       VALUE_TYPE } ) ) } ) ;);
            #[macro_export]
            macro_rules! gen_signature_dispatch((
                                                $ needle_name : ident , $
                                                needle_sig : ident ; $ name :
                                                ident (
                                                $ ctx : ident $ (
                                                , $ names : ident : $ params :
                                                ty ) * ) $ ( -> $ returns : ty
                                                ) * , $ ( $ rest : tt ) * ) =>
                                                {
                                                if stringify ! ( $ name ) .
                                                as_bytes (  ) == $ needle_name
                                                {
                                                let signature = gen_signature
                                                ! (
                                                ( $ ( $ params ) , * ) $ (
                                                -> $ returns ) * ) ; if $
                                                needle_sig == & signature {
                                                return true ; } } else {
                                                gen_signature_dispatch ! (
                                                $ needle_name , $ needle_sig ;
                                                $ ( $ rest ) * ) ; } } ; (
                                                $ needle_name : ident , $
                                                needle_sig : ident ; ) => {  }
                                                ;);
            /// Unmarshall arguments and then execute `body` expression and return its result.
            macro_rules! unmarshall_then_body((
                                              $ body : tt , $ ctx : ident , $
                                              args_iter : ident , $ (
                                              $ names : ident : $ params : ty
                                              ) , * ) => (
                                              {
                                              $ (
                                              let $ names : < $ params as $
                                              crate :: wasm :: env_def ::
                                              ConvertibleToWasm > ::
                                              NativeType = $ args_iter . next
                                              (  ) . and_then (
                                              | v | < $ params as $ crate ::
                                              wasm :: env_def ::
                                              ConvertibleToWasm > ::
                                              from_typed_value (
                                              v . clone (  ) ) ) . expect (
                                              "precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						"
                                              ) ; ) * $ body } ));
            /// Since we can't specify the type of closure directly at binding site:
            ///
            /// ```nocompile
            /// let f: FnOnce() -> Result<<u32 as ConvertibleToWasm>::NativeType, _> = || { /* ... */ };
            /// ```
            ///
            /// we use this function to constrain the type of the closure.
            #[inline(always)]
            pub fn constrain_closure<R, F>(f: F) -> F where F: FnOnce() ->
             Result<R, sandbox::HostError> {
                f
            }
            #[macro_export]
            macro_rules! unmarshall_then_body_then_marshall((
                                                            $ args_iter :
                                                            ident , $ ctx :
                                                            ident , (
                                                            $ (
                                                            $ names : ident :
                                                            $ params : ty ) ,
                                                            * ) -> $ returns :
                                                            ty => $ body : tt
                                                            ) => (
                                                            {
                                                            let body = $ crate
                                                            :: wasm :: env_def
                                                            :: macros ::
                                                            constrain_closure
                                                            :: < < $ returns
                                                            as $ crate :: wasm
                                                            :: env_def ::
                                                            ConvertibleToWasm
                                                            > :: NativeType ,
                                                            _ > (
                                                            || {
                                                            unmarshall_then_body
                                                            ! (
                                                            $ body , $ ctx , $
                                                            args_iter , $ (
                                                            $ names : $ params
                                                            ) , * ) } ) ; let
                                                            r = body (  ) ? ;
                                                            return Ok (
                                                            sandbox ::
                                                            ReturnValue ::
                                                            Value (
                                                            {
                                                            use $ crate ::
                                                            wasm :: env_def ::
                                                            ConvertibleToWasm
                                                            ; r .
                                                            to_typed_value (
                                                            ) } ) ) } ) ; (
                                                            $ args_iter :
                                                            ident , $ ctx :
                                                            ident , (
                                                            $ (
                                                            $ names : ident :
                                                            $ params : ty ) ,
                                                            * ) => $ body : tt
                                                            ) => (
                                                            {
                                                            let body = $ crate
                                                            :: wasm :: env_def
                                                            :: macros ::
                                                            constrain_closure
                                                            :: < (  ) , _ > (
                                                            || {
                                                            unmarshall_then_body
                                                            ! (
                                                            $ body , $ ctx , $
                                                            args_iter , $ (
                                                            $ names : $ params
                                                            ) , * ) } ) ; body
                                                            (  ) ? ; return Ok
                                                            (
                                                            sandbox ::
                                                            ReturnValue ::
                                                            Unit ) } ));
            #[macro_export]
            macro_rules! define_func((
                                     < E : $ ext_ty : tt > $ name : ident (
                                     $ ctx : ident $ (
                                     , $ names : ident : $ params : ty ) * ) $
                                     ( -> $ returns : ty ) * => $ body : tt )
                                     => {
                                     fn $ name < E : $ ext_ty > (
                                     $ ctx : & mut $ crate :: wasm :: Runtime
                                     < E > , args : & [ sandbox :: TypedValue
                                     ] , ) -> Result < sandbox :: ReturnValue
                                     , sandbox :: HostError > {
                                     # [ allow ( unused ) ] let mut args =
                                     args . iter (  ) ;
                                     unmarshall_then_body_then_marshall ! (
                                     args , $ ctx , (
                                     $ ( $ names : $ params ) , * ) $ (
                                     -> $ returns ) * => $ body ) } } ;);
            #[macro_export]
            macro_rules! register_func((
                                       $ reg_cb : ident , < E : $ ext_ty : tt
                                       > ; ) => {  } ; (
                                       $ reg_cb : ident , < E : $ ext_ty : tt
                                       > ; $ name : ident (
                                       $ ctx : ident $ (
                                       , $ names : ident : $ params : ty ) * )
                                       $ ( -> $ returns : ty ) * => $ body :
                                       tt $ ( $ rest : tt ) * ) => {
                                       $ reg_cb (
                                       stringify ! ( $ name ) . as_bytes (  )
                                       , {
                                       define_func ! (
                                       < E : $ ext_ty > $ name (
                                       $ ctx $ ( , $ names : $ params ) * ) $
                                       ( -> $ returns ) * => $ body ) ; $ name
                                       :: < E > } ) ; register_func ! (
                                       $ reg_cb , < E : $ ext_ty > ; $ (
                                       $ rest ) * ) ; } ;);
            /// Define a function set that can be imported by executing wasm code.
            ///
            /// **NB**: Be advised that all functions defined by this macro
            /// will panic if called with unexpected arguments.
            ///
            /// It's up to the user of this macro to check signatures of wasm code to be executed
            /// and reject the code if any imported function has a mismatched signature.
            macro_rules! define_env((
                                    $ init_name : ident , < E : $ ext_ty : tt
                                    > , $ (
                                    $ name : ident (
                                    $ ctx : ident $ (
                                    , $ names : ident : $ params : ty ) * ) $
                                    ( -> $ returns : ty ) * => $ body : tt , )
                                    * ) => {
                                    pub struct $ init_name ; impl $ crate ::
                                    wasm :: env_def :: ImportSatisfyCheck for
                                    $ init_name {
                                    fn can_satisfy (
                                    name : & [ u8 ] , func_type : &
                                    parity_wasm :: elements :: FunctionType )
                                    -> bool {
                                    gen_signature_dispatch ! (
                                    name , func_type ; $ (
                                    $ name (
                                    $ ctx $ ( , $ names : $ params ) * ) $ (
                                    -> $ returns ) * , ) * ) ; return false ;
                                    } } impl < E : Ext > $ crate :: wasm ::
                                    env_def :: FunctionImplProvider < E > for
                                    $ init_name {
                                    fn impls < F : FnMut (
                                    & [ u8 ] , $ crate :: wasm :: env_def ::
                                    HostFunc < E > ) > ( f : & mut F ) {
                                    register_func ! (
                                    f , < E : $ ext_ty > ; $ (
                                    $ name (
                                    $ ctx $ ( , $ names : $ params ) * ) $ (
                                    -> $ returns ) * => $ body ) * ) ; } } }
                                    ;);
        }
        pub trait ConvertibleToWasm: Sized {
            const
            VALUE_TYPE:
            ValueType;
            type
            NativeType;
            fn to_typed_value(self)
            -> TypedValue;
            fn from_typed_value(_: TypedValue)
            -> Option<Self>;
        }
        impl ConvertibleToWasm for i32 {
            type
            NativeType
            =
            i32;
            const
            VALUE_TYPE:
            ValueType
            =
            ValueType::I32;
            fn to_typed_value(self) -> TypedValue { TypedValue::I32(self) }
            fn from_typed_value(v: TypedValue) -> Option<Self> { v.as_i32() }
        }
        impl ConvertibleToWasm for u32 {
            type
            NativeType
            =
            u32;
            const
            VALUE_TYPE:
            ValueType
            =
            ValueType::I32;
            fn to_typed_value(self) -> TypedValue {
                TypedValue::I32(self as i32)
            }
            fn from_typed_value(v: TypedValue) -> Option<Self> {
                match v { TypedValue::I32(v) => Some(v as u32), _ => None, }
            }
        }
        impl ConvertibleToWasm for u64 {
            type
            NativeType
            =
            u64;
            const
            VALUE_TYPE:
            ValueType
            =
            ValueType::I64;
            fn to_typed_value(self) -> TypedValue {
                TypedValue::I64(self as i64)
            }
            fn from_typed_value(v: TypedValue) -> Option<Self> {
                match v { TypedValue::I64(v) => Some(v as u64), _ => None, }
            }
        }
        pub(crate) type HostFunc<E>
            =
            fn(&mut Runtime<E>, &[sandbox::TypedValue])
                -> Result<sandbox::ReturnValue, sandbox::HostError>;
        pub(crate) trait FunctionImplProvider<E: Ext> {
            fn impls<F: FnMut(&[u8], HostFunc<E>)>(f: &mut F);
        }
        /// This trait can be used to check whether the host environment can satisfy
        /// a requested function import.
        pub trait ImportSatisfyCheck {
            /// Returns `true` if the host environment contains a function with
            /// the specified name and its type matches to the given type, or `false`
            /// otherwise.
            fn can_satisfy(name: &[u8], func_type: &FunctionType)
            -> bool;
        }
    }
    mod code_cache {
        //! A module that implements instrumented code cache.
        //!
        //! - In order to run contract code we need to instrument it with gas metering.
        //! To do that we need to provide the schedule which will supply exact gas costs values.
        //! We cache this code in the storage saving the schedule version.
        //! - Before running contract code we check if the cached code has the schedule version that is equal to the current saved schedule.
        //! If it is equal then run the code, if it isn't reinstrument with the current schedule.
        //! - When we update the schedule we want it to have strictly greater version than the current saved one:
        //! this guarantees that every instrumented contract code in cache cannot have the version equal to the current one.
        //! Thus, before executing a contract it should be reinstrument with new schedule.
        use crate::gas::{GasMeter, Token};
        use crate::wasm::{prepare, runtime::Env, PrefabWasmModule};
        use crate::{CodeHash, CodeStorage, PristineCode, Schedule, Trait};
        use rstd::prelude::*;
        use runtime_primitives::traits::{As, CheckedMul, Hash, Bounded};
        use srml_support::StorageMap;
        /// Gas metering token that used for charging storing code into the code storage.
        ///
        /// Specifies the code length in bytes.
        #[rustc_copy_clone_marker]
        pub struct PutCodeToken(u64);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::marker::Copy for PutCodeToken { }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for PutCodeToken {
            #[inline]
            fn clone(&self) -> PutCodeToken {
                { let _: ::std::clone::AssertParamIsClone<u64>; *self }
            }
        }
        impl <T: Trait> Token<T> for PutCodeToken {
            type
            Metadata
            =
            Schedule<T::Gas>;
            fn calculate_amount(&self, metadata: &Schedule<T::Gas>)
             -> T::Gas {
                let code_len_in_gas = <T::Gas as As<u64>>::sa(self.0);
                metadata.put_code_per_byte_cost.checked_mul(&code_len_in_gas).unwrap_or_else(||
                                                                                                 Bounded::max_value())
            }
        }
        /// Put code in the storage. The hash of code is used as a key and is returned
        /// as a result of this function.
        ///
        /// This function instruments the given code and caches it in the storage.
        pub fn save<T: Trait>(original_code: Vec<u8>,
                              gas_meter: &mut GasMeter<T>,
                              schedule: &Schedule<T::Gas>)
         -> Result<CodeHash<T>, &'static str> {
            if gas_meter.charge(schedule,
                                PutCodeToken(original_code.len() as
                                                 u64)).is_out_of_gas() {
                return Err("there is not enough gas for storing the code");
            }
            let prefab_module =
                prepare::prepare_contract::<T,
                                            Env>(&original_code, schedule)?;
            let code_hash = T::Hashing::hash(&original_code);
            <CodeStorage<T>>::insert(code_hash, prefab_module);
            <PristineCode<T>>::insert(code_hash, original_code);
            Ok(code_hash)
        }
        /// Load code with the given code hash.
        ///
        /// If the module was instrumented with a lower version of schedule than
        /// the current one given as an argument, then this function will perform
        /// re-instrumentation and update the cache in the storage.
        pub fn load<T: Trait>(code_hash: &CodeHash<T>,
                              schedule: &Schedule<T::Gas>)
         -> Result<PrefabWasmModule, &'static str> {
            let mut prefab_module =
                <CodeStorage<T>>::get(code_hash).ok_or_else(||
                                                                "code is not found")?;
            if prefab_module.schedule_version < schedule.version {
                let original_code =
                    <PristineCode<T>>::get(code_hash).ok_or_else(||
                                                                     "pristine code is not found")?;
                prefab_module =
                    prepare::prepare_contract::<T,
                                                Env>(&original_code,
                                                     schedule)?;
                <CodeStorage<T>>::insert(code_hash, prefab_module.clone());
            }
            Ok(prefab_module)
        }
    }
    mod prepare {
        //! This module takes care of loading, checking and preprocessing of a
        //! wasm module before execution. It also extracts some essential information
        //! from a module.
        use crate::wasm::env_def::ImportSatisfyCheck;
        use crate::wasm::PrefabWasmModule;
        use crate::{Schedule, Trait};
        use parity_wasm::elements::{self, Internal, External, MemoryType,
                                    Type};
        use pwasm_utils;
        use pwasm_utils::rules;
        use rstd::prelude::*;
        use runtime_primitives::traits::As;
        struct ContractModule<'a, Gas: 'a> {
            /// A deserialized module. The module is valid (this is Guaranteed by `new` method).
            ///
            /// An `Option` is used here for loaning (`take()`-ing) the module.
            /// Invariant: Can't be `None` (i.e. on enter and on exit from the function
            /// the value *must* be `Some`).
            module: Option<elements::Module>,
            schedule: &'a Schedule<Gas>,
        }
        impl <'a, Gas: 'a + As<u32> + Clone> ContractModule<'a, Gas> {
            /// Creates a new instance of `ContractModule`.
            ///
            /// Returns `Err` if the `original_code` couldn't be decoded or
            /// if it contains an invalid module.
            fn new(original_code: &[u8], schedule: &'a Schedule<Gas>)
             -> Result<ContractModule<'a, Gas>, &'static str> {
                use wasmi_validation::{validate_module, PlainValidator};
                let module =
                    elements::deserialize_buffer(original_code).map_err(|_|
                                                                            "Can't decode wasm code")?;
                validate_module::<PlainValidator>(&module).map_err(|_|
                                                                       "Module is not valid")?;
                Ok(ContractModule{module: Some(module), schedule,})
            }
            /// Ensures that module doesn't declare internal memories.
            ///
            /// In this runtime we only allow wasm module to import memory from the environment.
            /// Memory section contains declarations of internal linear memories, so if we find one
            /// we reject such a module.
            fn ensure_no_internal_memory(&self) -> Result<(), &'static str> {
                let module =
                    self.module.as_ref().expect("On entry to the function `module` can't be None; qed");
                if module.memory_section().map_or(false,
                                                  |ms| ms.entries().len() > 0)
                   {
                    return Err("module declares internal memory");
                }
                Ok(())
            }
            fn inject_gas_metering(&mut self) -> Result<(), &'static str> {
                let gas_rules =
                    rules::Set::new(self.schedule.regular_op_cost.clone().as_(),
                                    Default::default()).with_grow_cost(self.schedule.grow_mem_cost.clone().as_()).with_forbidden_floats();
                let module =
                    self.module.take().expect("On entry to the function `module` can't be `None`; qed");
                let contract_module =
                    pwasm_utils::inject_gas_counter(module,
                                                    &gas_rules).map_err(|_|
                                                                            "gas instrumentation failed")?;
                self.module = Some(contract_module);
                Ok(())
            }
            fn inject_stack_height_metering(&mut self)
             -> Result<(), &'static str> {
                let module =
                    self.module.take().expect("On entry to the function `module` can't be `None`; qed");
                let contract_module =
                    pwasm_utils::stack_height::inject_limiter(module,
                                                              self.schedule.max_stack_height).map_err(|_|
                                                                                                          "stack height instrumentation failed")?;
                self.module = Some(contract_module);
                Ok(())
            }
            /// Check that the module has required exported functions. For now
            /// these are just entrypoints:
            ///
            /// - 'call'
            /// - 'deploy'
            ///
            /// Any other exports are not allowed.
            fn scan_exports(&self) -> Result<(), &'static str> {
                let mut deploy_found = false;
                let mut call_found = false;
                let module =
                    self.module.as_ref().expect("On entry to the function `module` can't be `None`; qed");
                let types =
                    module.type_section().map(|ts| ts.types()).unwrap_or(&[]);
                let export_entries =
                    module.export_section().map(|is|
                                                    is.entries()).unwrap_or(&[]);
                let func_entries =
                    module.function_section().map(|fs|
                                                      fs.entries()).unwrap_or(&[]);
                let fn_space_offset =
                    module.import_section().map(|is|
                                                    is.entries()).unwrap_or(&[]).iter().filter(|entry|
                                                                                                   {
                                                                                                       match *entry.external()
                                                                                                           {
                                                                                                           External::Function(_)
                                                                                                           =>
                                                                                                           true,
                                                                                                           _
                                                                                                           =>
                                                                                                           false,
                                                                                                       }
                                                                                                   }).count();
                for export in export_entries {
                    match export.field() {
                        "call" => call_found = true,
                        "deploy" => deploy_found = true,
                        _ =>
                        return Err("unknown export: expecting only deploy and call functions"),
                    }
                    let fn_idx =
                        match export.internal() {
                            Internal::Function(ref fn_idx) => *fn_idx,
                            _ => return Err("expected a function"),
                        };
                    let fn_idx =
                        match fn_idx.checked_sub(fn_space_offset as u32) {
                            Some(fn_idx) => fn_idx,
                            None => {
                                return Err("entry point points to an imported function");
                            }
                        };
                    let func_ty_idx =
                        func_entries.get(fn_idx as
                                             usize).ok_or_else(||
                                                                   "export refers to non-existent function")?.type_ref();
                    let Type::Function(ref func_ty) =
                        types.get(func_ty_idx as
                                      usize).ok_or_else(||
                                                            "function has a non-existent type")?;
                    if !(func_ty.params().is_empty() &&
                             func_ty.return_type().is_none()) {
                        return Err("entry point has wrong signature");
                    }
                }
                if !deploy_found {
                    return Err("deploy function isn't exported");
                }
                if !call_found { return Err("call function isn't exported"); }
                Ok(())
            }
            /// Scan an import section if any.
            ///
            /// This accomplishes two tasks:
            ///
            /// - checks any imported function against defined host functions set, incl.
            ///   their signatures.
            /// - if there is a memory import, returns it's descriptor
            fn scan_imports<C: ImportSatisfyCheck>(&self)
             -> Result<Option<&MemoryType>, &'static str> {
                let module =
                    self.module.as_ref().expect("On entry to the function `module` can't be `None`; qed");
                let types =
                    module.type_section().map(|ts| ts.types()).unwrap_or(&[]);
                let import_entries =
                    module.import_section().map(|is|
                                                    is.entries()).unwrap_or(&[]);
                let mut imported_mem_type = None;
                for import in import_entries {
                    if import.module() != "env" {
                        return Err("module has imports from a non-'env' namespace");
                    }
                    let type_idx =
                        match import.external() {
                            &External::Table(_) =>
                            return Err("Cannot import tables"),
                            &External::Global(_) =>
                            return Err("Cannot import globals"),
                            &External::Function(ref type_idx) => type_idx,
                            &External::Memory(ref memory_type) => {
                                if import.field() != "memory" {
                                    return Err("Memory import must have the field name 'memory'")
                                }
                                if imported_mem_type.is_some() {
                                    return Err("Multiple memory imports defined")
                                }
                                imported_mem_type = Some(memory_type);
                                continue ;
                            }
                        };
                    let Type::Function(ref func_ty) =
                        types.get(*type_idx as
                                      usize).ok_or_else(||
                                                            "validation: import entry points to a non-existent type")?;
                    if !self.schedule.enable_println &&
                           import.field().as_bytes() == b"ext_println" {
                        return Err("module imports `ext_println` but debug features disabled");
                    }
                    if import.field().as_bytes() == b"gas" ||
                           !C::can_satisfy(import.field().as_bytes(), func_ty)
                       {
                        return Err("module imports a non-existent function");
                    }
                }
                Ok(imported_mem_type)
            }
            fn into_wasm_code(mut self) -> Result<Vec<u8>, &'static str> {
                elements::serialize(self.module.take().expect("On entry to the function `module` can't be `None`; qed")).map_err(|_|
                                                                                                                                     "error serializing instrumented module")
            }
        }
        /// Loads the given module given in `original_code`, performs some checks on it and
        /// does some preprocessing.
        ///
        /// The checks are:
        ///
        /// - provided code is a valid wasm module.
        /// - the module doesn't define an internal memory instance,
        /// - imported memory (if any) doesn't reserve more memory than permitted by the `schedule`,
        /// - all imported functions from the external environment matches defined by `env` module,
        ///
        /// The preprocessing includes injecting code for gas metering and metering the height of stack.
        pub fn prepare_contract<T: Trait,
                                C: ImportSatisfyCheck>(original_code: &[u8],
                                                       schedule:
                                                           &Schedule<T::Gas>)
         -> Result<PrefabWasmModule, &'static str> {
            let mut contract_module =
                ContractModule::new(original_code, schedule)?;
            contract_module.scan_exports()?;
            contract_module.ensure_no_internal_memory()?;
            struct MemoryDefinition {
                initial: u32,
                maximum: u32,
            }
            let memory_def =
                if let Some(memory_type) =
                       contract_module.scan_imports::<C>()? {
                    let limits = memory_type.limits();
                    match (limits.initial(), limits.maximum()) {
                        (initial, Some(maximum)) if initial > maximum => {
                            return Err("Requested initial number of pages should not exceed the requested maximum");
                        }
                        (_, Some(maximum)) if
                        maximum > schedule.max_memory_pages => {
                            return Err("Maximum number of pages should not exceed the configured maximum.");
                        }
                        (initial, Some(maximum)) =>
                        MemoryDefinition{initial, maximum,},
                        (_, None) => {
                            return Err("Maximum number of pages should be always declared.");
                        }
                    }
                } else { MemoryDefinition{initial: 0, maximum: 0,} };
            contract_module.inject_gas_metering()?;
            contract_module.inject_stack_height_metering()?;
            Ok(PrefabWasmModule{schedule_version: schedule.version,
                                initial: memory_def.initial,
                                maximum: memory_def.maximum,
                                _reserved: None,
                                code: contract_module.into_wasm_code()?,})
        }
    }
    mod runtime {
        //! Environment definition of the wasm smart-contract runtime.
        use crate::{Schedule, Trait, CodeHash, ComputeDispatchFee, BalanceOf};
        use crate::exec::{Ext, VmExecResult, OutputBuf, EmptyOutputBuf,
                          CallReceipt, InstantiateReceipt, StorageKey,
                          TopicOf};
        use crate::gas::{GasMeter, Token, GasMeterResult,
                         approx_gas_for_balance};
        use sandbox;
        use system;
        use rstd::prelude::*;
        use rstd::mem;
        use parity_codec::{Decode, Encode};
        use runtime_primitives::traits::{As, CheckedMul, CheckedAdd, Bounded};
        /// Enumerates all possible *special* trap conditions.
        ///
        /// In this runtime traps used not only for signaling about errors but also
        /// to just terminate quickly in some cases.
        enum SpecialTrap {

            /// Signals that trap was generated in response to call `ext_return` host function.
            Return(OutputBuf),
        }
        /// Can only be used for one call.
        pub(crate) struct Runtime<'a, 'data, E: Ext + 'a> {
            ext: &'a mut E,
            input_data: &'data [u8],
            empty_output_buf: Option<EmptyOutputBuf>,
            scratch_buf: Vec<u8>,
            schedule: &'a Schedule<<E::T as Trait>::Gas>,
            memory: sandbox::Memory,
            gas_meter: &'a mut GasMeter<E::T>,
            special_trap: Option<SpecialTrap>,
        }
        impl <'a, 'data, E: Ext + 'a> Runtime<'a, 'data, E> {
            pub(crate) fn new(ext: &'a mut E, input_data: &'data [u8],
                              empty_output_buf: EmptyOutputBuf,
                              schedule: &'a Schedule<<E::T as Trait>::Gas>,
                              memory: sandbox::Memory,
                              gas_meter: &'a mut GasMeter<E::T>) -> Self {
                Runtime{ext,
                        input_data,
                        empty_output_buf: Some(empty_output_buf),
                        scratch_buf: Vec::new(),
                        schedule,
                        memory,
                        gas_meter,
                        special_trap: None,}
            }
            fn memory(&self) -> &sandbox::Memory { &self.memory }
        }
        pub(crate) fn to_execution_result<E: Ext>(runtime: Runtime<E>,
                                                  sandbox_err:
                                                      Option<sandbox::Error>)
         -> VmExecResult {
            match (sandbox_err, runtime.special_trap) {
                (None, None) => VmExecResult::Ok,
                (Some(sandbox::Error::Execution),
                 Some(SpecialTrap::Return(buf))) =>
                VmExecResult::Returned(buf),
                (Some(_), _) => VmExecResult::Trap("during execution"),
                _ => {
                    {
                        ::std::rt::begin_panic("internal error: entered unreachable code",
                                               &("srml/contract/src/wasm/runtime.rs",
                                                 95u32, 8u32))
                    }
                }
            }
        }
        #[rustc_copy_clone_marker]
        pub enum RuntimeToken<Gas> {

            /// Explicit call to the `gas` function. Charge the gas meter
            /// with the value provided.
            Explicit(u32),

            /// The given number of bytes is read from the sandbox memory.
            ReadMemory(u32),

            /// The given number of bytes is written to the sandbox memory.
            WriteMemory(u32),

            /// The given number of bytes is read from the sandbox memory and
            /// is returned as the return data buffer of the call.
            ReturnData(u32),

            /// Dispatch fee calculated by `T::ComputeDispatchFee`.
            ComputedDispatchFee(Gas),

            /// (topic_count, data_bytes): A buffer of the given size is posted as an event indexed with the
            /// given number of topics.
            DepositEvent(u32, u32),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Gas: ::std::marker::Copy> ::std::marker::Copy for
         RuntimeToken<Gas> {
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Gas: ::std::clone::Clone> ::std::clone::Clone for
         RuntimeToken<Gas> {
            #[inline]
            fn clone(&self) -> RuntimeToken<Gas> {
                match (&*self,) {
                    (&RuntimeToken::Explicit(ref __self_0),) =>
                    RuntimeToken::Explicit(::std::clone::Clone::clone(&(*__self_0))),
                    (&RuntimeToken::ReadMemory(ref __self_0),) =>
                    RuntimeToken::ReadMemory(::std::clone::Clone::clone(&(*__self_0))),
                    (&RuntimeToken::WriteMemory(ref __self_0),) =>
                    RuntimeToken::WriteMemory(::std::clone::Clone::clone(&(*__self_0))),
                    (&RuntimeToken::ReturnData(ref __self_0),) =>
                    RuntimeToken::ReturnData(::std::clone::Clone::clone(&(*__self_0))),
                    (&RuntimeToken::ComputedDispatchFee(ref __self_0),) =>
                    RuntimeToken::ComputedDispatchFee(::std::clone::Clone::clone(&(*__self_0))),
                    (&RuntimeToken::DepositEvent(ref __self_0, ref __self_1),)
                    =>
                    RuntimeToken::DepositEvent(::std::clone::Clone::clone(&(*__self_0)),
                                               ::std::clone::Clone::clone(&(*__self_1))),
                }
            }
        }
        impl <T: Trait> Token<T> for RuntimeToken<T::Gas> {
            type
            Metadata
            =
            Schedule<T::Gas>;
            fn calculate_amount(&self, metadata: &Schedule<T::Gas>)
             -> T::Gas {
                use self::RuntimeToken::*;
                let value =
                    match *self {
                        Explicit(amount) =>
                        Some(<T::Gas as As<u32>>::sa(amount)),
                        ReadMemory(byte_count) =>
                        metadata.sandbox_data_read_cost.checked_mul(&<T::Gas
                                                                         as
                                                                         As<u32>>::sa(byte_count)),
                        WriteMemory(byte_count) =>
                        metadata.sandbox_data_write_cost.checked_mul(&<T::Gas
                                                                          as
                                                                          As<u32>>::sa(byte_count)),
                        ReturnData(byte_count) =>
                        metadata.return_data_per_byte_cost.checked_mul(&<T::Gas
                                                                            as
                                                                            As<u32>>::sa(byte_count)),
                        DepositEvent(topic_count, data_byte_count) => {
                            let data_cost =
                                metadata.event_data_per_byte_cost.checked_mul(&<T::Gas
                                                                                   as
                                                                                   As<u32>>::sa(data_byte_count));
                            let topics_cost =
                                metadata.event_per_topic_cost.checked_mul(&<T::Gas
                                                                               as
                                                                               As<u32>>::sa(topic_count));
                            data_cost.and_then(|data_cost|
                                                   {
                                                       topics_cost.and_then(|topics_cost|
                                                                                {
                                                                                    data_cost.checked_add(&topics_cost)
                                                                                })
                                                   }).and_then(|data_and_topics_cost|
                                                                   data_and_topics_cost.checked_add(&metadata.event_base_cost))
                        }
                        ComputedDispatchFee(gas) => Some(gas),
                    };
                value.unwrap_or_else(|| Bounded::max_value())
            }
        }
        /// Charge the gas meter with the specified token.
        ///
        /// Returns `Err(HostError)` if there is not enough gas.
        fn charge_gas<T: Trait,
                      Tok: Token<T>>(gas_meter: &mut GasMeter<T>,
                                     metadata: &Tok::Metadata, token: Tok)
         -> Result<(), sandbox::HostError> {
            match gas_meter.charge(metadata, token) {
                GasMeterResult::Proceed => Ok(()),
                GasMeterResult::OutOfGas => Err(sandbox::HostError),
            }
        }
        /// Read designated chunk from the sandbox memory, consuming an appropriate amount of
        /// gas.
        ///
        /// Returns `Err` if one of the following conditions occurs:
        ///
        /// - calculating the gas cost resulted in overflow.
        /// - out of gas
        /// - requested buffer is not within the bounds of the sandbox memory.
        fn read_sandbox_memory<E: Ext>(ctx: &mut Runtime<E>, ptr: u32,
                                       len: u32)
         -> Result<Vec<u8>, sandbox::HostError> {
            charge_gas(ctx.gas_meter, ctx.schedule,
                       RuntimeToken::ReadMemory(len))?;
            let mut buf = Vec::new();
            buf.resize(len as usize, 0);
            ctx.memory().get(ptr, &mut buf)?;
            Ok(buf)
        }
        /// Read designated chunk from the sandbox memory into the supplied buffer, consuming
        /// an appropriate amount of gas.
        ///
        /// Returns `Err` if one of the following conditions occurs:
        ///
        /// - calculating the gas cost resulted in overflow.
        /// - out of gas
        /// - requested buffer is not within the bounds of the sandbox memory.
        fn read_sandbox_memory_into_buf<E: Ext>(ctx: &mut Runtime<E>,
                                                ptr: u32, buf: &mut [u8])
         -> Result<(), sandbox::HostError> {
            charge_gas(ctx.gas_meter, ctx.schedule,
                       RuntimeToken::ReadMemory(buf.len() as u32))?;
            ctx.memory().get(ptr, buf).map_err(Into::into)
        }
        /// Write the given buffer to the designated location in the sandbox memory, consuming
        /// an appropriate amount of gas.
        ///
        /// Returns `Err` if one of the following conditions occurs:
        ///
        /// - calculating the gas cost resulted in overflow.
        /// - out of gas
        /// - designated area is not within the bounds of the sandbox memory.
        fn write_sandbox_memory<T: Trait>(schedule: &Schedule<T::Gas>,
                                          gas_meter: &mut GasMeter<T>,
                                          memory: &sandbox::Memory, ptr: u32,
                                          buf: &[u8])
         -> Result<(), sandbox::HostError> {
            charge_gas(gas_meter, schedule,
                       RuntimeToken::WriteMemory(buf.len() as u32))?;
            memory.set(ptr, buf)?;
            Ok(())
        }
        pub struct Env;
        impl crate::wasm::env_def::ImportSatisfyCheck for Env {
            fn can_satisfy(name: &[u8],
                           func_type: &parity_wasm::elements::FunctionType)
             -> bool {
                if "gas".as_bytes() == name {
                    let signature =
                        {
                            parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                         [{
                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                              <u32>::VALUE_TYPE
                                                                                          }]),
                                                                     None)
                        };
                    if func_type == &signature { return true; }
                } else {
                    if "ext_set_storage".as_bytes() == name {
                        let signature =
                            {
                                parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                             [{
                                                                                                  use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                  <u32>::VALUE_TYPE
                                                                                              },
                                                                                              {
                                                                                                  use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                  <u32>::VALUE_TYPE
                                                                                              },
                                                                                              {
                                                                                                  use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                  <u32>::VALUE_TYPE
                                                                                              },
                                                                                              {
                                                                                                  use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                  <u32>::VALUE_TYPE
                                                                                              }]),
                                                                         None)
                            };
                        if func_type == &signature { return true; }
                    } else {
                        if "ext_get_storage".as_bytes() == name {
                            let signature =
                                {
                                    parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                 [{
                                                                                                      use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                      <u32>::VALUE_TYPE
                                                                                                  }]),
                                                                             Some({
                                                                                      use crate::wasm::env_def::ConvertibleToWasm;
                                                                                      <u32>::VALUE_TYPE
                                                                                  }))
                                };
                            if func_type == &signature { return true; }
                        } else {
                            if "ext_call".as_bytes() == name {
                                let signature =
                                    {
                                        parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                     [{
                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                          <u32>::VALUE_TYPE
                                                                                                      },
                                                                                                      {
                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                          <u32>::VALUE_TYPE
                                                                                                      },
                                                                                                      {
                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                          <u64>::VALUE_TYPE
                                                                                                      },
                                                                                                      {
                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                          <u32>::VALUE_TYPE
                                                                                                      },
                                                                                                      {
                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                          <u32>::VALUE_TYPE
                                                                                                      },
                                                                                                      {
                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                          <u32>::VALUE_TYPE
                                                                                                      },
                                                                                                      {
                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                          <u32>::VALUE_TYPE
                                                                                                      }]),
                                                                                 Some({
                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                          <u32>::VALUE_TYPE
                                                                                      }))
                                    };
                                if func_type == &signature { return true; }
                            } else {
                                if "ext_create".as_bytes() == name {
                                    let signature =
                                        {
                                            parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                         [{
                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                              <u32>::VALUE_TYPE
                                                                                                          },
                                                                                                          {
                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                              <u32>::VALUE_TYPE
                                                                                                          },
                                                                                                          {
                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                              <u64>::VALUE_TYPE
                                                                                                          },
                                                                                                          {
                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                              <u32>::VALUE_TYPE
                                                                                                          },
                                                                                                          {
                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                              <u32>::VALUE_TYPE
                                                                                                          },
                                                                                                          {
                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                              <u32>::VALUE_TYPE
                                                                                                          },
                                                                                                          {
                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                              <u32>::VALUE_TYPE
                                                                                                          }]),
                                                                                     Some({
                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                              <u32>::VALUE_TYPE
                                                                                          }))
                                        };
                                    if func_type == &signature {
                                        return true;
                                    }
                                } else {
                                    if "ext_return".as_bytes() == name {
                                        let signature =
                                            {
                                                parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                             [{
                                                                                                                  use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                  <u32>::VALUE_TYPE
                                                                                                              },
                                                                                                              {
                                                                                                                  use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                  <u32>::VALUE_TYPE
                                                                                                              }]),
                                                                                         None)
                                            };
                                        if func_type == &signature {
                                            return true;
                                        }
                                    } else {
                                        if "ext_caller".as_bytes() == name {
                                            let signature =
                                                {
                                                    parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                 []),
                                                                                             None)
                                                };
                                            if func_type == &signature {
                                                return true;
                                            }
                                        } else {
                                            if "ext_address".as_bytes() ==
                                                   name {
                                                let signature =
                                                    {
                                                        parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                     []),
                                                                                                 None)
                                                    };
                                                if func_type == &signature {
                                                    return true;
                                                }
                                            } else {
                                                if "ext_gas_price".as_bytes()
                                                       == name {
                                                    let signature =
                                                        {
                                                            parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                         []),
                                                                                                     None)
                                                        };
                                                    if func_type == &signature
                                                       {
                                                        return true;
                                                    }
                                                } else {
                                                    if "ext_gas_left".as_bytes()
                                                           == name {
                                                        let signature =
                                                            {
                                                                parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                             []),
                                                                                                         None)
                                                            };
                                                        if func_type ==
                                                               &signature {
                                                            return true;
                                                        }
                                                    } else {
                                                        if "ext_balance".as_bytes()
                                                               == name {
                                                            let signature =
                                                                {
                                                                    parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                 []),
                                                                                                             None)
                                                                };
                                                            if func_type ==
                                                                   &signature
                                                               {
                                                                return true;
                                                            }
                                                        } else {
                                                            if "ext_value_transferred".as_bytes()
                                                                   == name {
                                                                let signature =
                                                                    {
                                                                        parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                     []),
                                                                                                                 None)
                                                                    };
                                                                if func_type
                                                                       ==
                                                                       &signature
                                                                   {
                                                                    return true;
                                                                }
                                                            } else {
                                                                if "ext_random_seed".as_bytes()
                                                                       == name
                                                                   {
                                                                    let signature =
                                                                        {
                                                                            parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                         []),
                                                                                                                     None)
                                                                        };
                                                                    if func_type
                                                                           ==
                                                                           &signature
                                                                       {
                                                                        return true;
                                                                    }
                                                                } else {
                                                                    if "ext_now".as_bytes()
                                                                           ==
                                                                           name
                                                                       {
                                                                        let signature =
                                                                            {
                                                                                parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                             []),
                                                                                                                         None)
                                                                            };
                                                                        if func_type
                                                                               ==
                                                                               &signature
                                                                           {
                                                                            return true;
                                                                        }
                                                                    } else {
                                                                        if "ext_dispatch_call".as_bytes()
                                                                               ==
                                                                               name
                                                                           {
                                                                            let signature =
                                                                                {
                                                                                    parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                                 [{
                                                                                                                                                      use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                      <u32>::VALUE_TYPE
                                                                                                                                                  },
                                                                                                                                                  {
                                                                                                                                                      use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                      <u32>::VALUE_TYPE
                                                                                                                                                  }]),
                                                                                                                             None)
                                                                                };
                                                                            if func_type
                                                                                   ==
                                                                                   &signature
                                                                               {
                                                                                return true;
                                                                            }
                                                                        } else {
                                                                            if "ext_input_size".as_bytes()
                                                                                   ==
                                                                                   name
                                                                               {
                                                                                let signature =
                                                                                    {
                                                                                        parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                                     []),
                                                                                                                                 Some({
                                                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                          <u32>::VALUE_TYPE
                                                                                                                                      }))
                                                                                    };
                                                                                if func_type
                                                                                       ==
                                                                                       &signature
                                                                                   {
                                                                                    return true;
                                                                                }
                                                                            } else {
                                                                                if "ext_input_copy".as_bytes()
                                                                                       ==
                                                                                       name
                                                                                   {
                                                                                    let signature =
                                                                                        {
                                                                                            parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                                         [{
                                                                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                              <u32>::VALUE_TYPE
                                                                                                                                                          },
                                                                                                                                                          {
                                                                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                              <u32>::VALUE_TYPE
                                                                                                                                                          },
                                                                                                                                                          {
                                                                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                              <u32>::VALUE_TYPE
                                                                                                                                                          }]),
                                                                                                                                     None)
                                                                                        };
                                                                                    if func_type
                                                                                           ==
                                                                                           &signature
                                                                                       {
                                                                                        return true;
                                                                                    }
                                                                                } else {
                                                                                    if "ext_scratch_size".as_bytes()
                                                                                           ==
                                                                                           name
                                                                                       {
                                                                                        let signature =
                                                                                            {
                                                                                                parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                                             []),
                                                                                                                                         Some({
                                                                                                                                                  use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                  <u32>::VALUE_TYPE
                                                                                                                                              }))
                                                                                            };
                                                                                        if func_type
                                                                                               ==
                                                                                               &signature
                                                                                           {
                                                                                            return true;
                                                                                        }
                                                                                    } else {
                                                                                        if "ext_scratch_copy".as_bytes()
                                                                                               ==
                                                                                               name
                                                                                           {
                                                                                            let signature =
                                                                                                {
                                                                                                    parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                                                 [{
                                                                                                                                                                      use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                      <u32>::VALUE_TYPE
                                                                                                                                                                  },
                                                                                                                                                                  {
                                                                                                                                                                      use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                      <u32>::VALUE_TYPE
                                                                                                                                                                  },
                                                                                                                                                                  {
                                                                                                                                                                      use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                      <u32>::VALUE_TYPE
                                                                                                                                                                  }]),
                                                                                                                                             None)
                                                                                                };
                                                                                            if func_type
                                                                                                   ==
                                                                                                   &signature
                                                                                               {
                                                                                                return true;
                                                                                            }
                                                                                        } else {
                                                                                            if "ext_deposit_event".as_bytes()
                                                                                                   ==
                                                                                                   name
                                                                                               {
                                                                                                let signature =
                                                                                                    {
                                                                                                        parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                                                     [{
                                                                                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                          <u32>::VALUE_TYPE
                                                                                                                                                                      },
                                                                                                                                                                      {
                                                                                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                          <u32>::VALUE_TYPE
                                                                                                                                                                      },
                                                                                                                                                                      {
                                                                                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                          <u32>::VALUE_TYPE
                                                                                                                                                                      },
                                                                                                                                                                      {
                                                                                                                                                                          use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                          <u32>::VALUE_TYPE
                                                                                                                                                                      }]),
                                                                                                                                                 None)
                                                                                                    };
                                                                                                if func_type
                                                                                                       ==
                                                                                                       &signature
                                                                                                   {
                                                                                                    return true;
                                                                                                }
                                                                                            } else {
                                                                                                if "ext_set_rent_allowance".as_bytes()
                                                                                                       ==
                                                                                                       name
                                                                                                   {
                                                                                                    let signature =
                                                                                                        {
                                                                                                            parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                                                         [{
                                                                                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                              <u32>::VALUE_TYPE
                                                                                                                                                                          },
                                                                                                                                                                          {
                                                                                                                                                                              use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                              <u32>::VALUE_TYPE
                                                                                                                                                                          }]),
                                                                                                                                                     None)
                                                                                                        };
                                                                                                    if func_type
                                                                                                           ==
                                                                                                           &signature
                                                                                                       {
                                                                                                        return true;
                                                                                                    }
                                                                                                } else {
                                                                                                    if "ext_rent_allowance".as_bytes()
                                                                                                           ==
                                                                                                           name
                                                                                                       {
                                                                                                        let signature =
                                                                                                            {
                                                                                                                parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                                                             []),
                                                                                                                                                         None)
                                                                                                            };
                                                                                                        if func_type
                                                                                                               ==
                                                                                                               &signature
                                                                                                           {
                                                                                                            return true;
                                                                                                        }
                                                                                                    } else {
                                                                                                        if "ext_println".as_bytes()
                                                                                                               ==
                                                                                                               name
                                                                                                           {
                                                                                                            let signature =
                                                                                                                {
                                                                                                                    parity_wasm::elements::FunctionType::new(<[_]>::into_vec(box
                                                                                                                                                                                 [{
                                                                                                                                                                                      use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                                      <u32>::VALUE_TYPE
                                                                                                                                                                                  },
                                                                                                                                                                                  {
                                                                                                                                                                                      use crate::wasm::env_def::ConvertibleToWasm;
                                                                                                                                                                                      <u32>::VALUE_TYPE
                                                                                                                                                                                  }]),
                                                                                                                                                             None)
                                                                                                                };
                                                                                                            if func_type
                                                                                                                   ==
                                                                                                                   &signature
                                                                                                               {
                                                                                                                return true;
                                                                                                            }
                                                                                                        } else {
                                                                                                        };
                                                                                                    };
                                                                                                };
                                                                                            };
                                                                                        };
                                                                                    };
                                                                                };
                                                                            };
                                                                        };
                                                                    };
                                                                };
                                                            };
                                                        };
                                                    };
                                                };
                                            };
                                        };
                                    };
                                };
                            };
                        };
                    };
                };
                return false;
            }
        }
        impl <E: Ext> crate::wasm::env_def::FunctionImplProvider<E> for Env {
            fn impls<F: FnMut(&[u8],
                              crate::wasm::env_def::HostFunc<E>)>(f: &mut F) {
                f("gas".as_bytes(),
                  {
                      fn gas<E: Ext>(ctx: &mut crate::wasm::Runtime<E>,
                                     args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let amount:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       charge_gas(&mut ctx.gas_meter,
                                                                                                                  ctx.schedule,
                                                                                                                  RuntimeToken::Explicit(amount))?;
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      gas::<E>
                  });
                f("ext_set_storage".as_bytes(),
                  {
                      fn ext_set_storage<E: Ext>(ctx:
                                                     &mut crate::wasm::Runtime<E>,
                                                 args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let key_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let value_non_null:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let value_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let value_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let mut key:
                                                                                                               StorageKey =
                                                                                                           [0;
                                                                                                               32];
                                                                                                       read_sandbox_memory_into_buf(ctx,
                                                                                                                                    key_ptr,
                                                                                                                                    &mut key)?;
                                                                                                       let value =
                                                                                                           if value_non_null
                                                                                                                  !=
                                                                                                                  0
                                                                                                              {
                                                                                                               Some(read_sandbox_memory(ctx,
                                                                                                                                        value_ptr,
                                                                                                                                        value_len)?)
                                                                                                           } else {
                                                                                                               None
                                                                                                           };
                                                                                                       ctx.ext.set_storage(key,
                                                                                                                           value);
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_set_storage::<E>
                  });
                f("ext_get_storage".as_bytes(),
                  {
                      fn ext_get_storage<E: Ext>(ctx:
                                                     &mut crate::wasm::Runtime<E>,
                                                 args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<<u32
                                                                                    as
                                                                                    crate::wasm::env_def::ConvertibleToWasm>::NativeType,
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let key_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let mut key:
                                                                                                               StorageKey =
                                                                                                           [0;
                                                                                                               32];
                                                                                                       read_sandbox_memory_into_buf(ctx,
                                                                                                                                    key_ptr,
                                                                                                                                    &mut key)?;
                                                                                                       if let Some(value)
                                                                                                              =
                                                                                                              ctx.ext.get_storage(&key)
                                                                                                              {
                                                                                                           ctx.scratch_buf
                                                                                                               =
                                                                                                               value;
                                                                                                           Ok(0)
                                                                                                       } else {
                                                                                                           ctx.scratch_buf.clear();
                                                                                                           Ok(1)
                                                                                                       }
                                                                                                   }
                                                                                               }
                                                                                           });
                              let r = body()?;
                              return Ok(sandbox::ReturnValue::Value({
                                                                        use crate::wasm::env_def::ConvertibleToWasm;
                                                                        r.to_typed_value()
                                                                    }))
                          }
                      }
                      ext_get_storage::<E>
                  });
                f("ext_call".as_bytes(),
                  {
                      fn ext_call<E: Ext>(ctx: &mut crate::wasm::Runtime<E>,
                                          args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<<u32
                                                                                    as
                                                                                    crate::wasm::env_def::ConvertibleToWasm>::NativeType,
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let callee_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let callee_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let gas:
                                                                                                           <u64
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u64
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let value_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let value_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let input_data_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let input_data_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let callee =
                                                                                                           {
                                                                                                               let callee_buf =
                                                                                                                   read_sandbox_memory(ctx,
                                                                                                                                       callee_ptr,
                                                                                                                                       callee_len)?;
                                                                                                               <<E
                                                                                                                as
                                                                                                                Ext>::T
                                                                                                                   as
                                                                                                                   system::Trait>::decode(&mut &callee_buf[..]).ok_or_else(||
                                                                                                                                                                               sandbox::HostError)?
                                                                                                           };
                                                                                                       let value =
                                                                                                           {
                                                                                                               let value_buf =
                                                                                                                   read_sandbox_memory(ctx,
                                                                                                                                       value_ptr,
                                                                                                                                       value_len)?;
                                                                                                               BalanceOf::<<E
                                                                                                                           as
                                                                                                                           Ext>::T>::decode(&mut &value_buf[..]).ok_or_else(||
                                                                                                                                                                                sandbox::HostError)?
                                                                                                           };
                                                                                                       let input_data =
                                                                                                           read_sandbox_memory(ctx,
                                                                                                                               input_data_ptr,
                                                                                                                               input_data_len)?;
                                                                                                       let scratch_buf =
                                                                                                           mem::replace(&mut ctx.scratch_buf,
                                                                                                                        Vec::new());
                                                                                                       let empty_output_buf =
                                                                                                           EmptyOutputBuf::from_spare_vec(scratch_buf);
                                                                                                       let nested_gas_limit =
                                                                                                           if gas
                                                                                                                  ==
                                                                                                                  0
                                                                                                              {
                                                                                                               ctx.gas_meter.gas_left()
                                                                                                           } else {
                                                                                                               <<E::T
                                                                                                                as
                                                                                                                Trait>::Gas
                                                                                                                   as
                                                                                                                   As<u64>>::sa(gas)
                                                                                                           };
                                                                                                       let ext =
                                                                                                           &mut ctx.ext;
                                                                                                       let call_outcome =
                                                                                                           ctx.gas_meter.with_nested(nested_gas_limit,
                                                                                                                                     |nested_meter|
                                                                                                                                         {
                                                                                                                                             match nested_meter
                                                                                                                                                 {
                                                                                                                                                 Some(nested_meter)
                                                                                                                                                 =>
                                                                                                                                                 {
                                                                                                                                                     ext.call(&callee,
                                                                                                                                                              value,
                                                                                                                                                              nested_meter,
                                                                                                                                                              &input_data,
                                                                                                                                                              empty_output_buf).map_err(|_|
                                                                                                                                                                                            ())
                                                                                                                                                 }
                                                                                                                                                 None
                                                                                                                                                 =>
                                                                                                                                                 Err(()),
                                                                                                                                             }
                                                                                                                                         });
                                                                                                       match call_outcome
                                                                                                           {
                                                                                                           Ok(CallReceipt {
                                                                                                              output_data
                                                                                                              })
                                                                                                           =>
                                                                                                           {
                                                                                                               ctx.scratch_buf
                                                                                                                   =
                                                                                                                   output_data;
                                                                                                               Ok(0)
                                                                                                           }
                                                                                                           Err(_)
                                                                                                           =>
                                                                                                           Ok(1),
                                                                                                       }
                                                                                                   }
                                                                                               }
                                                                                           });
                              let r = body()?;
                              return Ok(sandbox::ReturnValue::Value({
                                                                        use crate::wasm::env_def::ConvertibleToWasm;
                                                                        r.to_typed_value()
                                                                    }))
                          }
                      }
                      ext_call::<E>
                  });
                f("ext_create".as_bytes(),
                  {
                      fn ext_create<E: Ext>(ctx: &mut crate::wasm::Runtime<E>,
                                            args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<<u32
                                                                                    as
                                                                                    crate::wasm::env_def::ConvertibleToWasm>::NativeType,
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let init_code_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let init_code_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let gas:
                                                                                                           <u64
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u64
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let value_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let value_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let input_data_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let input_data_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let code_hash =
                                                                                                           {
                                                                                                               let code_hash_buf =
                                                                                                                   read_sandbox_memory(ctx,
                                                                                                                                       init_code_ptr,
                                                                                                                                       init_code_len)?;
                                                                                                               <CodeHash<<E
                                                                                                                         as
                                                                                                                         Ext>::T>>::decode(&mut &code_hash_buf[..]).ok_or_else(||
                                                                                                                                                                                   sandbox::HostError)?
                                                                                                           };
                                                                                                       let value =
                                                                                                           {
                                                                                                               let value_buf =
                                                                                                                   read_sandbox_memory(ctx,
                                                                                                                                       value_ptr,
                                                                                                                                       value_len)?;
                                                                                                               BalanceOf::<<E
                                                                                                                           as
                                                                                                                           Ext>::T>::decode(&mut &value_buf[..]).ok_or_else(||
                                                                                                                                                                                sandbox::HostError)?
                                                                                                           };
                                                                                                       let input_data =
                                                                                                           read_sandbox_memory(ctx,
                                                                                                                               input_data_ptr,
                                                                                                                               input_data_len)?;
                                                                                                       ctx.scratch_buf.clear();
                                                                                                       let nested_gas_limit =
                                                                                                           if gas
                                                                                                                  ==
                                                                                                                  0
                                                                                                              {
                                                                                                               ctx.gas_meter.gas_left()
                                                                                                           } else {
                                                                                                               <<E::T
                                                                                                                as
                                                                                                                Trait>::Gas
                                                                                                                   as
                                                                                                                   As<u64>>::sa(gas)
                                                                                                           };
                                                                                                       let ext =
                                                                                                           &mut ctx.ext;
                                                                                                       let instantiate_outcome =
                                                                                                           ctx.gas_meter.with_nested(nested_gas_limit,
                                                                                                                                     |nested_meter|
                                                                                                                                         {
                                                                                                                                             match nested_meter
                                                                                                                                                 {
                                                                                                                                                 Some(nested_meter)
                                                                                                                                                 =>
                                                                                                                                                 {
                                                                                                                                                     ext.instantiate(&code_hash,
                                                                                                                                                                     value,
                                                                                                                                                                     nested_meter,
                                                                                                                                                                     &input_data).map_err(|_|
                                                                                                                                                                                              ())
                                                                                                                                                 }
                                                                                                                                                 None
                                                                                                                                                 =>
                                                                                                                                                 Err(()),
                                                                                                                                             }
                                                                                                                                         });
                                                                                                       match instantiate_outcome
                                                                                                           {
                                                                                                           Ok(InstantiateReceipt {
                                                                                                              address
                                                                                                              })
                                                                                                           =>
                                                                                                           {
                                                                                                               address.encode_to(&mut ctx.scratch_buf);
                                                                                                               Ok(0)
                                                                                                           }
                                                                                                           Err(_)
                                                                                                           =>
                                                                                                           Ok(1),
                                                                                                       }
                                                                                                   }
                                                                                               }
                                                                                           });
                              let r = body()?;
                              return Ok(sandbox::ReturnValue::Value({
                                                                        use crate::wasm::env_def::ConvertibleToWasm;
                                                                        r.to_typed_value()
                                                                    }))
                          }
                      }
                      ext_create::<E>
                  });
                f("ext_return".as_bytes(),
                  {
                      fn ext_return<E: Ext>(ctx: &mut crate::wasm::Runtime<E>,
                                            args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let data_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let data_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       match ctx.gas_meter.charge(ctx.schedule,
                                                                                                                                  RuntimeToken::ReturnData(data_len))
                                                                                                           {
                                                                                                           GasMeterResult::Proceed
                                                                                                           =>
                                                                                                           (),
                                                                                                           GasMeterResult::OutOfGas
                                                                                                           =>
                                                                                                           return Err(sandbox::HostError),
                                                                                                       }
                                                                                                       let empty_output_buf =
                                                                                                           ctx.empty_output_buf.take().expect("`empty_output_buf` is taken only here;
				`ext_return` traps;
				`Runtime` can only be used only for one execution;
				qed");
                                                                                                       let output_buf =
                                                                                                           empty_output_buf.fill(data_len
                                                                                                                                     as
                                                                                                                                     usize,
                                                                                                                                 |slice_mut|
                                                                                                                                     {
                                                                                                                                         ctx.memory.get(data_ptr,
                                                                                                                                                        slice_mut)
                                                                                                                                     })?;
                                                                                                       ctx.special_trap
                                                                                                           =
                                                                                                           Some(SpecialTrap::Return(output_buf));
                                                                                                       Err(sandbox::HostError)
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_return::<E>
                  });
                f("ext_caller".as_bytes(),
                  {
                      fn ext_caller<E: Ext>(ctx: &mut crate::wasm::Runtime<E>,
                                            args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       ctx.scratch_buf
                                                                                                           =
                                                                                                           ctx.ext.caller().encode();
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_caller::<E>
                  });
                f("ext_address".as_bytes(),
                  {
                      fn ext_address<E: Ext>(ctx:
                                                 &mut crate::wasm::Runtime<E>,
                                             args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       ctx.scratch_buf
                                                                                                           =
                                                                                                           ctx.ext.address().encode();
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_address::<E>
                  });
                f("ext_gas_price".as_bytes(),
                  {
                      fn ext_gas_price<E: Ext>(ctx:
                                                   &mut crate::wasm::Runtime<E>,
                                               args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       ctx.scratch_buf
                                                                                                           =
                                                                                                           ctx.gas_meter.gas_price().encode();
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_gas_price::<E>
                  });
                f("ext_gas_left".as_bytes(),
                  {
                      fn ext_gas_left<E: Ext>(ctx:
                                                  &mut crate::wasm::Runtime<E>,
                                              args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       ctx.scratch_buf
                                                                                                           =
                                                                                                           ctx.gas_meter.gas_left().encode();
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_gas_left::<E>
                  });
                f("ext_balance".as_bytes(),
                  {
                      fn ext_balance<E: Ext>(ctx:
                                                 &mut crate::wasm::Runtime<E>,
                                             args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       ctx.scratch_buf
                                                                                                           =
                                                                                                           ctx.ext.balance().encode();
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_balance::<E>
                  });
                f("ext_value_transferred".as_bytes(),
                  {
                      fn ext_value_transferred<E: Ext>(ctx:
                                                           &mut crate::wasm::Runtime<E>,
                                                       args:
                                                           &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       ctx.scratch_buf
                                                                                                           =
                                                                                                           ctx.ext.value_transferred().encode();
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_value_transferred::<E>
                  });
                f("ext_random_seed".as_bytes(),
                  {
                      fn ext_random_seed<E: Ext>(ctx:
                                                     &mut crate::wasm::Runtime<E>,
                                                 args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       ctx.scratch_buf
                                                                                                           =
                                                                                                           ctx.ext.random_seed().encode();
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_random_seed::<E>
                  });
                f("ext_now".as_bytes(),
                  {
                      fn ext_now<E: Ext>(ctx: &mut crate::wasm::Runtime<E>,
                                         args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       let now:
                                                                                                               u64 =
                                                                                                           As::as_(ctx.ext.now().clone());
                                                                                                       ctx.scratch_buf
                                                                                                           =
                                                                                                           now.encode();
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_now::<E>
                  });
                f("ext_dispatch_call".as_bytes(),
                  {
                      fn ext_dispatch_call<E: Ext>(ctx:
                                                       &mut crate::wasm::Runtime<E>,
                                                   args:
                                                       &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let call_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let call_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let call =
                                                                                                           {
                                                                                                               let call_buf =
                                                                                                                   read_sandbox_memory(ctx,
                                                                                                                                       call_ptr,
                                                                                                                                       call_len)?;
                                                                                                               <<<E
                                                                                                                 as
                                                                                                                 Ext>::T
                                                                                                                as
                                                                                                                Trait>::Call>::decode(&mut &call_buf[..]).ok_or_else(||
                                                                                                                                                                         sandbox::HostError)?
                                                                                                           };
                                                                                                       let fee =
                                                                                                           {
                                                                                                               let balance_fee =
                                                                                                                   <<E
                                                                                                                    as
                                                                                                                    Ext>::T
                                                                                                                       as
                                                                                                                       Trait>::compute_dispatch_fee(&call);
                                                                                                               approx_gas_for_balance::<<E
                                                                                                                                        as
                                                                                                                                        Ext>::T>(ctx.gas_meter.gas_price(),
                                                                                                                                                 balance_fee)
                                                                                                           };
                                                                                                       charge_gas(&mut ctx.gas_meter,
                                                                                                                  ctx.schedule,
                                                                                                                  RuntimeToken::ComputedDispatchFee(fee))?;
                                                                                                       ctx.ext.note_dispatch_call(call);
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_dispatch_call::<E>
                  });
                f("ext_input_size".as_bytes(),
                  {
                      fn ext_input_size<E: Ext>(ctx:
                                                    &mut crate::wasm::Runtime<E>,
                                                args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<<u32
                                                                                    as
                                                                                    crate::wasm::env_def::ConvertibleToWasm>::NativeType,
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       Ok(ctx.input_data.len()
                                                                                                              as
                                                                                                              u32)
                                                                                                   }
                                                                                               }
                                                                                           });
                              let r = body()?;
                              return Ok(sandbox::ReturnValue::Value({
                                                                        use crate::wasm::env_def::ConvertibleToWasm;
                                                                        r.to_typed_value()
                                                                    }))
                          }
                      }
                      ext_input_size::<E>
                  });
                f("ext_input_copy".as_bytes(),
                  {
                      fn ext_input_copy<E: Ext>(ctx:
                                                    &mut crate::wasm::Runtime<E>,
                                                args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let dest_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let offset:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let offset =
                                                                                                           offset
                                                                                                               as
                                                                                                               usize;
                                                                                                       if offset
                                                                                                              >
                                                                                                              ctx.input_data.len()
                                                                                                          {
                                                                                                           return Err(sandbox::HostError);
                                                                                                       }
                                                                                                       let src =
                                                                                                           &ctx.input_data[offset..];
                                                                                                       if src.len()
                                                                                                              !=
                                                                                                              len
                                                                                                                  as
                                                                                                                  usize
                                                                                                          {
                                                                                                           return Err(sandbox::HostError);
                                                                                                       }
                                                                                                       write_sandbox_memory(ctx.schedule,
                                                                                                                            ctx.gas_meter,
                                                                                                                            &ctx.memory,
                                                                                                                            dest_ptr,
                                                                                                                            src)?;
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_input_copy::<E>
                  });
                f("ext_scratch_size".as_bytes(),
                  {
                      fn ext_scratch_size<E: Ext>(ctx:
                                                      &mut crate::wasm::Runtime<E>,
                                                  args:
                                                      &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<<u32
                                                                                    as
                                                                                    crate::wasm::env_def::ConvertibleToWasm>::NativeType,
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       Ok(ctx.scratch_buf.len()
                                                                                                              as
                                                                                                              u32)
                                                                                                   }
                                                                                               }
                                                                                           });
                              let r = body()?;
                              return Ok(sandbox::ReturnValue::Value({
                                                                        use crate::wasm::env_def::ConvertibleToWasm;
                                                                        r.to_typed_value()
                                                                    }))
                          }
                      }
                      ext_scratch_size::<E>
                  });
                f("ext_scratch_copy".as_bytes(),
                  {
                      fn ext_scratch_copy<E: Ext>(ctx:
                                                      &mut crate::wasm::Runtime<E>,
                                                  args:
                                                      &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let dest_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let offset:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let offset =
                                                                                                           offset
                                                                                                               as
                                                                                                               usize;
                                                                                                       if offset
                                                                                                              >
                                                                                                              ctx.scratch_buf.len()
                                                                                                          {
                                                                                                           return Err(sandbox::HostError);
                                                                                                       }
                                                                                                       let src =
                                                                                                           &ctx.scratch_buf[offset..];
                                                                                                       if src.len()
                                                                                                              !=
                                                                                                              len
                                                                                                                  as
                                                                                                                  usize
                                                                                                          {
                                                                                                           return Err(sandbox::HostError);
                                                                                                       }
                                                                                                       write_sandbox_memory(ctx.schedule,
                                                                                                                            ctx.gas_meter,
                                                                                                                            &ctx.memory,
                                                                                                                            dest_ptr,
                                                                                                                            src)?;
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_scratch_copy::<E>
                  });
                f("ext_deposit_event".as_bytes(),
                  {
                      fn ext_deposit_event<E: Ext>(ctx:
                                                       &mut crate::wasm::Runtime<E>,
                                                   args:
                                                       &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let topics_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let topics_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let data_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let data_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let mut topics =
                                                                                                           match topics_len
                                                                                                               {
                                                                                                               0
                                                                                                               =>
                                                                                                               Vec::new(),
                                                                                                               _
                                                                                                               =>
                                                                                                               {
                                                                                                                   let topics_buf =
                                                                                                                       read_sandbox_memory(ctx,
                                                                                                                                           topics_ptr,
                                                                                                                                           topics_len)?;
                                                                                                                   Vec::<TopicOf<<E
                                                                                                                                 as
                                                                                                                                 Ext>::T>>::decode(&mut &topics_buf[..]).ok_or_else(||
                                                                                                                                                                                        sandbox::HostError)?
                                                                                                               }
                                                                                                           };
                                                                                                       if topics.len()
                                                                                                              >
                                                                                                              ctx.schedule.max_event_topics
                                                                                                                  as
                                                                                                                  usize
                                                                                                          {
                                                                                                           return Err(sandbox::HostError);
                                                                                                       }
                                                                                                       if has_duplicates(&mut topics)
                                                                                                          {
                                                                                                           return Err(sandbox::HostError);
                                                                                                       }
                                                                                                       let event_data =
                                                                                                           read_sandbox_memory(ctx,
                                                                                                                               data_ptr,
                                                                                                                               data_len)?;
                                                                                                       match ctx.gas_meter.charge(ctx.schedule,
                                                                                                                                  RuntimeToken::DepositEvent(topics.len()
                                                                                                                                                                 as
                                                                                                                                                                 u32,
                                                                                                                                                             data_len))
                                                                                                           {
                                                                                                           GasMeterResult::Proceed
                                                                                                           =>
                                                                                                           (),
                                                                                                           GasMeterResult::OutOfGas
                                                                                                           =>
                                                                                                           return Err(sandbox::HostError),
                                                                                                       }
                                                                                                       ctx.ext.deposit_event(topics,
                                                                                                                             event_data);
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_deposit_event::<E>
                  });
                f("ext_set_rent_allowance".as_bytes(),
                  {
                      fn ext_set_rent_allowance<E: Ext>(ctx:
                                                            &mut crate::wasm::Runtime<E>,
                                                        args:
                                                            &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let value_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let value_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let value =
                                                                                                           {
                                                                                                               let value_buf =
                                                                                                                   read_sandbox_memory(ctx,
                                                                                                                                       value_ptr,
                                                                                                                                       value_len)?;
                                                                                                               BalanceOf::<<E
                                                                                                                           as
                                                                                                                           Ext>::T>::decode(&mut &value_buf[..]).ok_or_else(||
                                                                                                                                                                                sandbox::HostError)?
                                                                                                           };
                                                                                                       ctx.ext.set_rent_allowance(value);
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_set_rent_allowance::<E>
                  });
                f("ext_rent_allowance".as_bytes(),
                  {
                      fn ext_rent_allowance<E: Ext>(ctx:
                                                        &mut crate::wasm::Runtime<E>,
                                                    args:
                                                        &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   {
                                                                                                       ctx.scratch_buf
                                                                                                           =
                                                                                                           ctx.ext.rent_allowance().encode();
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_rent_allowance::<E>
                  });
                f("ext_println".as_bytes(),
                  {
                      fn ext_println<E: Ext>(ctx:
                                                 &mut crate::wasm::Runtime<E>,
                                             args: &[sandbox::TypedValue])
                       -> Result<sandbox::ReturnValue, sandbox::HostError> {
                          #[allow(unused)]
                          let mut args = args.iter();
                          {
                              let body =
                                  crate::wasm::env_def::macros::constrain_closure::<(),
                                                                                    _>(||
                                                                                           {
                                                                                               {
                                                                                                   let str_ptr:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   let str_len:
                                                                                                           <u32
                                                                                                           as
                                                                                                           crate::wasm::env_def::ConvertibleToWasm>::NativeType =
                                                                                                       args.next().and_then(|v|
                                                                                                                                <u32
                                                                                                                                    as
                                                                                                                                    crate::wasm::env_def::ConvertibleToWasm>::from_typed_value(v.clone())).expect("precondition: all imports should be checked against the signatures of corresponding
						functions defined by `define_env!` macro by the user of the macro;
						signatures of these functions defined by `$params`;
						calls always made with arguments types of which are defined by the corresponding imports;
						thus types of arguments should be equal to type list in `$params` and
						length of argument list and $params should be equal;
						thus this can never be `None`;
						qed;
						");
                                                                                                   {
                                                                                                       let data =
                                                                                                           read_sandbox_memory(ctx,
                                                                                                                               str_ptr,
                                                                                                                               str_len)?;
                                                                                                       if let Ok(utf8)
                                                                                                              =
                                                                                                              core::str::from_utf8(&data)
                                                                                                              {
                                                                                                           runtime_io::print(utf8);
                                                                                                       }
                                                                                                       Ok(())
                                                                                                   }
                                                                                               }
                                                                                           });
                              body()?;
                              return Ok(sandbox::ReturnValue::Unit)
                          }
                      }
                      ext_println::<E>
                  });
            }
        }
        /// Finds duplicates in a given vector.
        ///
        /// This function has complexity of O(n log n) and no additional memory is required, although
        /// the order of items is not preserved.
        fn has_duplicates<T: PartialEq + AsRef<[u8]>>(items: &mut Vec<T>)
         -> bool {
            items.sort_unstable_by(|a, b|
                                       { Ord::cmp(a.as_ref(), b.as_ref()) });
            items.windows(2).any(|w|
                                     {
                                         match w {
                                             &[ref a, ref b] => a == b,
                                             _ => false,
                                         }
                                     })
        }
    }
    use self::runtime::{to_execution_result, Runtime};
    use self::code_cache::load as load_code;
    pub use self::code_cache::save as save_code;
    /// A prepared wasm module ready for execution.
    pub struct PrefabWasmModule {
        /// Version of the schedule with which the code was instrumented.
        #[codec(compact)]
        schedule_version: u32,
        #[codec(compact)]
        initial: u32,
        #[codec(compact)]
        maximum: u32,
        /// This field is reserved for future evolution of format.
        ///
        /// Basically, for now this field will be serialized as `None`. In the future
        /// we would be able to extend this structure with.
        _reserved: Option<()>,
        /// Code instrumented with the latest schedule.
        code: Vec<u8>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for PrefabWasmModule {
        #[inline]
        fn clone(&self) -> PrefabWasmModule {
            match *self {
                PrefabWasmModule {
                schedule_version: ref __self_0_0,
                initial: ref __self_0_1,
                maximum: ref __self_0_2,
                _reserved: ref __self_0_3,
                code: ref __self_0_4 } =>
                PrefabWasmModule{schedule_version:
                                     ::std::clone::Clone::clone(&(*__self_0_0)),
                                 initial:
                                     ::std::clone::Clone::clone(&(*__self_0_1)),
                                 maximum:
                                     ::std::clone::Clone::clone(&(*__self_0_2)),
                                 _reserved:
                                     ::std::clone::Clone::clone(&(*__self_0_3)),
                                 code:
                                     ::std::clone::Clone::clone(&(*__self_0_4)),},
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_PrefabWasmModule: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for PrefabWasmModule {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    {
                        dest.push(&<<u32 as _parity_codec::HasCompact>::Type
                                       as
                                       _parity_codec::EncodeAsRef<'_,
                                                                  u32>>::from(&self.schedule_version));
                    }
                    {
                        dest.push(&<<u32 as _parity_codec::HasCompact>::Type
                                       as
                                       _parity_codec::EncodeAsRef<'_,
                                                                  u32>>::from(&self.initial));
                    }
                    {
                        dest.push(&<<u32 as _parity_codec::HasCompact>::Type
                                       as
                                       _parity_codec::EncodeAsRef<'_,
                                                                  u32>>::from(&self.maximum));
                    }
                    dest.push(&self._reserved);
                    dest.push(&self.code);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_PrefabWasmModule: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for PrefabWasmModule {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(PrefabWasmModule{schedule_version:
                                              <<u32 as
                                               _parity_codec::HasCompact>::Type
                                                  as
                                                  _parity_codec::Decode>::decode(input)?.into(),
                                          initial:
                                              <<u32 as
                                               _parity_codec::HasCompact>::Type
                                                  as
                                                  _parity_codec::Decode>::decode(input)?.into(),
                                          maximum:
                                              <<u32 as
                                               _parity_codec::HasCompact>::Type
                                                  as
                                                  _parity_codec::Decode>::decode(input)?.into(),
                                          _reserved:
                                              _parity_codec::Decode::decode(input)?,
                                          code:
                                              _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    /// Wasm executable loaded by `WasmLoader` and executed by `WasmVm`.
    pub struct WasmExecutable {
        entrypoint_name: &'static [u8],
        prefab_module: PrefabWasmModule,
    }
    /// Loader which fetches `WasmExecutable` from the code cache.
    pub struct WasmLoader<'a, T: Trait> {
        schedule: &'a Schedule<T::Gas>,
    }
    impl <'a, T: Trait> WasmLoader<'a, T> {
        pub fn new(schedule: &'a Schedule<T::Gas>) -> Self {
            WasmLoader{schedule,}
        }
    }
    impl <'a, T: Trait> crate::exec::Loader<T> for WasmLoader<'a, T> {
        type
        Executable
        =
        WasmExecutable;
        fn load_init(&self, code_hash: &CodeHash<T>)
         -> Result<WasmExecutable, &'static str> {
            let prefab_module = load_code::<T>(code_hash, self.schedule)?;
            Ok(WasmExecutable{entrypoint_name: b"deploy", prefab_module,})
        }
        fn load_main(&self, code_hash: &CodeHash<T>)
         -> Result<WasmExecutable, &'static str> {
            let prefab_module = load_code::<T>(code_hash, self.schedule)?;
            Ok(WasmExecutable{entrypoint_name: b"call", prefab_module,})
        }
    }
    /// Implementation of `Vm` that takes `WasmExecutable` and executes it.
    pub struct WasmVm<'a, T: Trait> {
        schedule: &'a Schedule<T::Gas>,
    }
    impl <'a, T: Trait> WasmVm<'a, T> {
        pub fn new(schedule: &'a Schedule<T::Gas>) -> Self {
            WasmVm{schedule,}
        }
    }
    impl <'a, T: Trait> crate::exec::Vm<T> for WasmVm<'a, T> {
        type
        Executable
        =
        WasmExecutable;
        fn execute<E: Ext<T =
                   T>>(&self, exec: &WasmExecutable, ext: &mut E,
                       input_data: &[u8], empty_output_buf: EmptyOutputBuf,
                       gas_meter: &mut GasMeter<E::T>) -> VmExecResult {
            let memory =
                sandbox::Memory::new(exec.prefab_module.initial,
                                     Some(exec.prefab_module.maximum)).unwrap_or_else(|_|
                                                                                          {
                                                                                              {
                                                                                                  ::std::rt::begin_panic("exec.prefab_module.initial can't be greater than exec.prefab_module.maximum;
						thus Memory::new must not fail;
						qed",
                                                                                                                         &("srml/contract/src/wasm/mod.rs",
                                                                                                                           122u32,
                                                                                                                           6u32))
                                                                                              }
                                                                                          });
            let mut imports = sandbox::EnvironmentDefinitionBuilder::new();
            imports.add_memory("env", "memory", memory.clone());
            runtime::Env::impls(&mut (|name, func_ptr|
                                          {
                                              imports.add_host_func("env",
                                                                    name,
                                                                    func_ptr);
                                          }));
            let mut runtime =
                Runtime::new(ext, input_data, empty_output_buf,
                             &self.schedule, memory, gas_meter);
            match sandbox::Instance::new(&exec.prefab_module.code, &imports,
                                         &mut runtime) {
                Ok(mut instance) => {
                    let err =
                        instance.invoke(exec.entrypoint_name, &[],
                                        &mut runtime).err();
                    to_execution_result(runtime, err)
                }
                Err(err@sandbox::Error::Execution) =>
                to_execution_result(runtime, Some(err)),
                Err(_err@sandbox::Error::Module) => {
                    return VmExecResult::Trap("validation error");
                }
                Err(_) => return VmExecResult::Trap("during start function"),
            }
        }
    }
}
mod rent {
    use crate::{BalanceOf, ContractInfo, ContractInfoOf, Module,
                TombstoneContractInfo, Trait};
    use runtime_primitives::traits::{As, Bounded, CheckedDiv, CheckedMul,
                                     Saturating, Zero};
    use srml_support::traits::{Currency, ExistenceRequirement, Imbalance,
                               WithdrawReason};
    use srml_support::StorageMap;
    #[must_use]
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum RentOutcome {

        /// Exempted from rent iff:
        /// * rent is offset completely by the `rent_deposit_offset`,
        /// * or rent has already been paid for this block number,
        /// * or account doesn't have a contract,
        /// * or account has a tombstone.
        Exempted,

        /// Evicted iff:
        /// * rent exceed rent allowance,
        /// * or can't withdraw the rent,
        /// * or go below subsistence threshold.
        Evicted,

        /// The outstanding dues were paid or were able to be paid.
        Ok,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for RentOutcome {
        #[inline]
        fn eq(&self, other: &RentOutcome) -> bool {
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
    impl ::std::cmp::Eq for RentOutcome {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for RentOutcome { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for RentOutcome {
        #[inline]
        fn clone(&self) -> RentOutcome { { *self } }
    }
    /// Evict and optionally pay dues (or check account can pay them otherwise) at the current
    /// block number (modulo `handicap`, read on).
    ///
    /// `pay_rent` gives an ability to pay or skip paying rent.
    /// `handicap` gives a way to check or pay the rent up to a moment in the past instead
    /// of current block.
    ///
    /// NOTE: This function acts eagerly, all modification are committed into the storage.
    fn try_evict_or_and_pay_rent<T: Trait>(account: &T::AccountId,
                                           handicap: T::BlockNumber,
                                           pay_rent: bool) -> RentOutcome {
        let contract =
            match <ContractInfoOf<T>>::get(account) {
                None | Some(ContractInfo::Tombstone(_)) =>
                return RentOutcome::Exempted,
                Some(ContractInfo::Alive(contract)) => contract,
            };
        let current_block_number = <system::Module<T>>::block_number();
        let blocks_passed =
            {
                let effective_block_number =
                    current_block_number.saturating_sub(handicap);
                let n =
                    effective_block_number.saturating_sub(contract.deduct_block);
                if n.is_zero() { return RentOutcome::Exempted; }
                n
            };
        let balance = T::Currency::free_balance(account);
        let fee_per_block =
            {
                let free_storage =
                    balance.checked_div(&<Module<T>>::rent_deposit_offset()).unwrap_or(<BalanceOf<T>>::sa(0));
                let effective_storage_size =
                    <BalanceOf<T>>::sa(contract.storage_size).saturating_sub(free_storage);
                effective_storage_size.checked_mul(&<Module<T>>::rent_byte_price()).unwrap_or(<BalanceOf<T>>::max_value())
            };
        if fee_per_block.is_zero() { return RentOutcome::Exempted; }
        let subsistence_threshold =
            T::Currency::minimum_balance() + <Module<T>>::tombstone_deposit();
        let dues =
            fee_per_block.checked_mul(&<BalanceOf<T>>::sa(blocks_passed.as_())).unwrap_or(<BalanceOf<T>>::max_value());
        let dues_limited = dues.min(contract.rent_allowance);
        let rent_allowance_exceeded = dues > contract.rent_allowance;
        let is_below_subsistence = balance < subsistence_threshold;
        let go_below_subsistence =
            balance.saturating_sub(dues_limited) < subsistence_threshold;
        let can_withdraw_rent =
            T::Currency::ensure_can_withdraw(account, dues_limited,
                                             WithdrawReason::Fee,
                                             balance.saturating_sub(dues_limited)).is_ok();
        if !rent_allowance_exceeded && can_withdraw_rent &&
               !go_below_subsistence {
            if pay_rent {
                let imbalance =
                    T::Currency::withdraw(account, dues, WithdrawReason::Fee,
                                          ExistenceRequirement::KeepAlive).expect("Withdraw has been checked above;
				go_below_subsistence is false and subsistence > existencial_deposit;
				qed");
                <ContractInfoOf<T>>::mutate(account,
                                            |contract|
                                                {
                                                    let contract =
                                                        contract.as_mut().and_then(|c|
                                                                                       c.as_alive_mut()).expect("Dead or inexistent account has been exempt above; qed");
                                                    contract.rent_allowance -=
                                                        imbalance.peek();
                                                    contract.deduct_block =
                                                        current_block_number;
                                                })
            }
            RentOutcome::Ok
        } else {
            if can_withdraw_rent && !go_below_subsistence {
                T::Currency::withdraw(account, dues, WithdrawReason::Fee,
                                      ExistenceRequirement::KeepAlive).expect("Can withdraw and don't go below subsistence");
            } else if !is_below_subsistence {
                T::Currency::make_free_balance_be(account,
                                                  subsistence_threshold);
            } else {
                T::Currency::make_free_balance_be(account,
                                                  <BalanceOf<T>>::zero());
            }
            if !is_below_subsistence {
                let child_storage_root =
                    runtime_io::child_storage_root(&contract.trie_id);
                let tombstone =
                    TombstoneContractInfo::new(child_storage_root,
                                               contract.storage_size,
                                               contract.code_hash);
                <ContractInfoOf<T>>::insert(account,
                                            ContractInfo::Tombstone(tombstone));
                runtime_io::kill_child_storage(&contract.trie_id);
            }
            RentOutcome::Evicted
        }
    }
    /// Make account paying the rent for the current block number
    ///
    /// NOTE: This function acts eagerly.
    pub fn pay_rent<T: Trait>(account: &T::AccountId) {
        let _ = try_evict_or_and_pay_rent::<T>(account, Zero::zero(), true);
    }
    /// Evict the account if it should be evicted at the given block number.
    ///
    /// `handicap` gives a way to check or pay the rent up to a moment in the past instead
    /// of current block. E.g. if the contract is going to be evicted at the current block,
    /// `handicap=1` can defer the eviction for 1 block.
    ///
    /// NOTE: This function acts eagerly.
    pub fn try_evict<T: Trait>(account: &T::AccountId,
                               handicap: T::BlockNumber) -> RentOutcome {
        try_evict_or_and_pay_rent::<T>(account, handicap, false)
    }
}
use crate::exec::ExecutionContext;
use crate::account_db::{AccountDb, DirectAccountDb};
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use substrate_primitives::crypto::UncheckedFrom;
use rstd::prelude::*;
use rstd::marker::PhantomData;
use parity_codec::{Codec, Encode, Decode};
use runtime_primitives::traits::{Hash, As, SimpleArithmetic, Bounded,
                                 StaticLookup, Zero};
use srml_support::dispatch::{Result, Dispatchable};
use srml_support::{Parameter, StorageMap, StorageValue, decl_module,
                   decl_event, decl_storage, storage::child};
use srml_support::traits::{OnFreeBalanceZero, OnUnbalanced, Currency};
use system::{ensure_signed, RawOrigin};
use substrate_primitives::storage::well_known_keys::CHILD_STORAGE_KEY_PREFIX;
use timestamp;
pub type CodeHash<T> = <T as system::Trait>::Hash;
pub type TrieId = Vec<u8>;
/// A function that generates an `AccountId` for a contract upon instantiation.
pub trait ContractAddressFor<CodeHash, AccountId> {
    fn contract_address_for(code_hash: &CodeHash, data: &[u8],
                            origin: &AccountId)
    -> AccountId;
}
/// A function that returns the fee for dispatching a `Call`.
pub trait ComputeDispatchFee<Call, Balance> {
    fn compute_dispatch_fee(call: &Call)
    -> Balance;
}
/// Information for managing an acocunt and its sub trie abstraction.
/// This is the required info to cache for an account
pub enum ContractInfo<T: Trait> {
    Alive(AliveContractInfo<T>),
    Tombstone(TombstoneContractInfo<T>),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_ContractInfo: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for ContractInfo<T> where
         AliveContractInfo<T>: _parity_codec::Encode,
         AliveContractInfo<T>: _parity_codec::Encode,
         TombstoneContractInfo<T>: _parity_codec::Encode,
         TombstoneContractInfo<T>: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    ContractInfo::Alive(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    ContractInfo::Tombstone(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_ContractInfo: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Decode for ContractInfo<T> where
         AliveContractInfo<T>: _parity_codec::Decode,
         AliveContractInfo<T>: _parity_codec::Decode,
         TombstoneContractInfo<T>: _parity_codec::Decode,
         TombstoneContractInfo<T>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(ContractInfo::Alive(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(ContractInfo::Tombstone(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
impl <T: Trait> ContractInfo<T> {
    /// If contract is alive then return some alive info
    pub fn get_alive(self) -> Option<AliveContractInfo<T>> {
        if let ContractInfo::Alive(alive) = self { Some(alive) } else { None }
    }
    /// If contract is alive then return some reference to alive info
    pub fn as_alive(&self) -> Option<&AliveContractInfo<T>> {
        if let ContractInfo::Alive(ref alive) = self {
            Some(alive)
        } else { None }
    }
    /// If contract is alive then return some mutable reference to alive info
    pub fn as_alive_mut(&mut self) -> Option<&mut AliveContractInfo<T>> {
        if let ContractInfo::Alive(ref mut alive) = self {
            Some(alive)
        } else { None }
    }
    /// If contract is tombstone then return some alive info
    pub fn get_tombstone(self) -> Option<TombstoneContractInfo<T>> {
        if let ContractInfo::Tombstone(tombstone) = self {
            Some(tombstone)
        } else { None }
    }
    /// If contract is tombstone then return some reference to tombstone info
    pub fn as_tombstone(&self) -> Option<&TombstoneContractInfo<T>> {
        if let ContractInfo::Tombstone(ref tombstone) = self {
            Some(tombstone)
        } else { None }
    }
    /// If contract is tombstone then return some mutable reference to tombstone info
    pub fn as_tombstone_mut(&mut self)
     -> Option<&mut TombstoneContractInfo<T>> {
        if let ContractInfo::Tombstone(ref mut tombstone) = self {
            Some(tombstone)
        } else { None }
    }
}
pub type AliveContractInfo<T>
    =
    RawAliveContractInfo<CodeHash<T>, BalanceOf<T>,
                         <T as system::Trait>::BlockNumber>;
/// Information for managing an account and its sub trie abstraction.
/// This is the required info to cache for an account.
#[structural_match]
pub struct RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
    /// Unique ID for the subtree encoded as a bytes vector.
    pub trie_id: TrieId,
    /// The size of stored value in octet.
    pub storage_size: u64,
    /// The code associated with a given account.
    pub code_hash: CodeHash,
    pub rent_allowance: Balance,
    pub deduct_block: BlockNumber,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawAliveContractInfo: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <CodeHash, Balance, BlockNumber> _parity_codec::Encode for
         RawAliveContractInfo<CodeHash, Balance, BlockNumber> where
         CodeHash: _parity_codec::Encode, CodeHash: _parity_codec::Encode,
         Balance: _parity_codec::Encode, Balance: _parity_codec::Encode,
         BlockNumber: _parity_codec::Encode,
         BlockNumber: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.trie_id);
                dest.push(&self.storage_size);
                dest.push(&self.code_hash);
                dest.push(&self.rent_allowance);
                dest.push(&self.deduct_block);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_RawAliveContractInfo: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <CodeHash, Balance, BlockNumber> _parity_codec::Decode for
         RawAliveContractInfo<CodeHash, Balance, BlockNumber> where
         CodeHash: _parity_codec::Decode, CodeHash: _parity_codec::Decode,
         Balance: _parity_codec::Decode, Balance: _parity_codec::Decode,
         BlockNumber: _parity_codec::Decode,
         BlockNumber: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(RawAliveContractInfo{trie_id:
                                              _parity_codec::Decode::decode(input)?,
                                          storage_size:
                                              _parity_codec::Decode::decode(input)?,
                                          code_hash:
                                              _parity_codec::Decode::decode(input)?,
                                          rent_allowance:
                                              _parity_codec::Decode::decode(input)?,
                                          deduct_block:
                                              _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <CodeHash: ::std::clone::Clone, Balance: ::std::clone::Clone,
      BlockNumber: ::std::clone::Clone> ::std::clone::Clone for
 RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
    #[inline]
    fn clone(&self) -> RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
        match *self {
            RawAliveContractInfo {
            trie_id: ref __self_0_0,
            storage_size: ref __self_0_1,
            code_hash: ref __self_0_2,
            rent_allowance: ref __self_0_3,
            deduct_block: ref __self_0_4 } =>
            RawAliveContractInfo{trie_id:
                                     ::std::clone::Clone::clone(&(*__self_0_0)),
                                 storage_size:
                                     ::std::clone::Clone::clone(&(*__self_0_1)),
                                 code_hash:
                                     ::std::clone::Clone::clone(&(*__self_0_2)),
                                 rent_allowance:
                                     ::std::clone::Clone::clone(&(*__self_0_3)),
                                 deduct_block:
                                     ::std::clone::Clone::clone(&(*__self_0_4)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <CodeHash: ::std::cmp::PartialEq, Balance: ::std::cmp::PartialEq,
      BlockNumber: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
 RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
    #[inline]
    fn eq(&self, other: &RawAliveContractInfo<CodeHash, Balance, BlockNumber>)
     -> bool {
        match *other {
            RawAliveContractInfo {
            trie_id: ref __self_1_0,
            storage_size: ref __self_1_1,
            code_hash: ref __self_1_2,
            rent_allowance: ref __self_1_3,
            deduct_block: ref __self_1_4 } =>
            match *self {
                RawAliveContractInfo {
                trie_id: ref __self_0_0,
                storage_size: ref __self_0_1,
                code_hash: ref __self_0_2,
                rent_allowance: ref __self_0_3,
                deduct_block: ref __self_0_4 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3) &&
                    (*__self_0_4) == (*__self_1_4),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &RawAliveContractInfo<CodeHash, Balance, BlockNumber>)
     -> bool {
        match *other {
            RawAliveContractInfo {
            trie_id: ref __self_1_0,
            storage_size: ref __self_1_1,
            code_hash: ref __self_1_2,
            rent_allowance: ref __self_1_3,
            deduct_block: ref __self_1_4 } =>
            match *self {
                RawAliveContractInfo {
                trie_id: ref __self_0_0,
                storage_size: ref __self_0_1,
                code_hash: ref __self_0_2,
                rent_allowance: ref __self_0_3,
                deduct_block: ref __self_0_4 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2) ||
                    (*__self_0_3) != (*__self_1_3) ||
                    (*__self_0_4) != (*__self_1_4),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <CodeHash: ::std::cmp::Eq, Balance: ::std::cmp::Eq,
      BlockNumber: ::std::cmp::Eq> ::std::cmp::Eq for
 RawAliveContractInfo<CodeHash, Balance, BlockNumber> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<TrieId>;
            let _: ::std::cmp::AssertParamIsEq<u64>;
            let _: ::std::cmp::AssertParamIsEq<CodeHash>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<BlockNumber>;
        }
    }
}
pub struct TombstoneContractInfo<T: Trait>(T::Hash);
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_TombstoneContractInfo: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for TombstoneContractInfo<T>
         where T::Hash: _parity_codec::Encode, T::Hash: _parity_codec::Encode
         {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_TombstoneContractInfo: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Decode for TombstoneContractInfo<T>
         where T::Hash: _parity_codec::Decode, T::Hash: _parity_codec::Decode
         {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(TombstoneContractInfo(_parity_codec::Decode::decode(input)?))
            }
        }
    };
impl <T: Trait> TombstoneContractInfo<T> {
    fn new(storage_root: Vec<u8>, storage_size: u64, code_hash: CodeHash<T>)
     -> Self {
        let mut buf = Vec::new();
        storage_root.using_encoded(|encoded| buf.extend_from_slice(encoded));
        storage_size.using_encoded(|encoded| buf.extend_from_slice(encoded));
        buf.extend_from_slice(code_hash.as_ref());
        TombstoneContractInfo(T::Hashing::hash(&buf[..]))
    }
}
/// Get a trie id (trie id must be unique and collision resistant depending upon its context).
/// Note that it is different than encode because trie id should be collision resistant
/// (being a proper unique identifier).
pub trait TrieIdGenerator<AccountId> {
    /// Get a trie id for an account, using reference to parent account trie id to ensure
    /// uniqueness of trie id.
    ///
    /// The implementation must ensure every new trie id is unique: two consecutive calls with the
    /// same parameter needs to return different trie id values.
    ///
    /// Also, the implementation is responsible for ensuring that `TrieId` starts with
    /// `:child_storage:`.
    /// TODO: We want to change this, see https://github.com/paritytech/substrate/issues/2325
    fn trie_id(account_id: &AccountId)
    -> TrieId;
}
/// Get trie id from `account_id`.
pub struct TrieIdFromParentCounter<T: Trait>(PhantomData<T>);
/// This generator uses inner counter for account id and applies the hash over `AccountId +
/// accountid_counter`.
impl <T: Trait> TrieIdGenerator<T::AccountId> for TrieIdFromParentCounter<T>
 where T::AccountId: AsRef<[u8]> {
    fn trie_id(account_id: &T::AccountId) -> TrieId {
        let new_seed = <AccountCounter<T>>::mutate(|v| v.wrapping_add(1));
        let mut buf = Vec::new();
        buf.extend_from_slice(account_id.as_ref());
        buf.extend_from_slice(&new_seed.to_le_bytes()[..]);
        CHILD_STORAGE_KEY_PREFIX.iter().chain(b"default:").chain(T::Hashing::hash(&buf[..]).as_ref().iter()).cloned().collect()
    }
}
pub type BalanceOf<T>
    =
    <<T as Trait>::Currency as
    Currency<<T as system::Trait>::AccountId>>::Balance;
pub type NegativeImbalanceOf<T>
    =
    <<T as Trait>::Currency as
    Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;
pub trait Trait: timestamp::Trait {
    type
    Currency: Currency<Self::AccountId>;
    /// The outer call dispatch type.
    type
    Call: Parameter +
    Dispatchable<Origin
    =
    <Self as system::Trait>::Origin>;
    /// The overarching event type.
    type
    Event: From<Event<Self>> +
    Into<<Self as system::Trait>::Event>;
    type
    Gas: Parameter +
    Default +
    Codec +
    SimpleArithmetic +
    Bounded +
    Copy +
    As<BalanceOf<Self>> +
    As<u64> +
    As<u32>;
    /// A function type to get the contract address given the creator.
    type
    DetermineContractAddress: ContractAddressFor<CodeHash<Self>,
                                                 Self::AccountId>;
    /// A function type that computes the fee for dispatching the given `Call`.
    ///
    /// It is recommended (though not required) for this function to return a fee that would be taken
    /// by the Executive module for regular dispatch.
    type
    ComputeDispatchFee: ComputeDispatchFee<Self::Call, BalanceOf<Self>>;
    /// trieid id generator
    type
    TrieIdGenerator: TrieIdGenerator<Self::AccountId>;
    /// Handler for the unbalanced reduction when making a gas payment.
    type
    GasPayment: OnUnbalanced<NegativeImbalanceOf<Self>>;
}
/// Simple contract address determiner.
///
/// Address calculated from the code (of the constructor), input data to the constructor,
/// and the account id that requested the account creation.
///
/// Formula: `blake2_256(blake2_256(code) + blake2_256(data) + origin)`
pub struct SimpleAddressDeterminator<T: Trait>(PhantomData<T>);
impl <T: Trait> ContractAddressFor<CodeHash<T>, T::AccountId> for
 SimpleAddressDeterminator<T> where T::AccountId: UncheckedFrom<T::Hash> +
 AsRef<[u8]> {
    fn contract_address_for(code_hash: &CodeHash<T>, data: &[u8],
                            origin: &T::AccountId) -> T::AccountId {
        let data_hash = T::Hashing::hash(data);
        let mut buf = Vec::new();
        buf.extend_from_slice(code_hash.as_ref());
        buf.extend_from_slice(data_hash.as_ref());
        buf.extend_from_slice(origin.as_ref());
        UncheckedFrom::unchecked_from(T::Hashing::hash(&buf[..]))
    }
}
/// The default dispatch fee computor computes the fee in the same way that
/// the implementation of `MakePayment` for the Balances module does.
pub struct DefaultDispatchFeeComputor<T: Trait>(PhantomData<T>);
impl <T: Trait> ComputeDispatchFee<T::Call, BalanceOf<T>> for
 DefaultDispatchFeeComputor<T> {
    fn compute_dispatch_fee(call: &T::Call) -> BalanceOf<T> {
        let encoded_len = call.using_encoded(|encoded| encoded.len());
        let base_fee = <Module<T>>::transaction_base_fee();
        let byte_fee = <Module<T>>::transaction_byte_fee();
        base_fee +
            byte_fee * <BalanceOf<T> as As<u64>>::sa(encoded_len as u64)
    }
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
    fn on_finalize(_block_number_not_used: T::BlockNumber) {
        <GasSpent<T>>::kill();
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
    #[doc = r" Updates the schedule for metering contracts."]
    #[doc = r""]
    #[doc =
          r" The schedule must have a greater version than the stored schedule."]
    pub fn update_schedule(schedule: Schedule<T::Gas>) -> Result {
        if <Module<T>>::current_schedule().version >= schedule.version {
            return Err("new schedule must have a greater version than current");
        }
        Self::deposit_event(RawEvent::ScheduleUpdated(schedule.version));
        <CurrentSchedule<T>>::put(schedule);
        Ok(())
    }
    #[doc =
          r" Stores the given binary Wasm code into the chain's storage and returns its `codehash`."]
    #[doc = r" You can instantiate contracts only with stored code."]
    pub fn put_code(origin: <T as system::Trait>::Origin, gas_limit: T::Gas,
                    code: Vec<u8>) -> Result {
        let origin = ensure_signed(origin)?;
        let schedule = <Module<T>>::current_schedule();
        let (mut gas_meter, imbalance) =
            gas::buy_gas::<T>(&origin, gas_limit)?;
        let result = wasm::save_code::<T>(code, &mut gas_meter, &schedule);
        if let Ok(code_hash) = result {
            Self::deposit_event(RawEvent::CodeStored(code_hash));
        }
        gas::refund_unused_gas::<T>(&origin, gas_meter, imbalance);
        result.map(|_| ())
    }
    #[doc =
          r" Makes a call to an account, optionally transferring some balance."]
    #[doc = r""]
    #[doc =
          r" * If the account is a smart-contract account, the associated code will be"]
    #[doc = r" executed and any value will be transferred."]
    #[doc =
          r" * If the account is a regular account, any value will be transferred."]
    #[doc =
          r" * If no account exists and the call value is not less than `existential_deposit`,"]
    #[doc =
          r" a regular account will be created and any value will be transferred."]
    pub fn call(origin: <T as system::Trait>::Origin,
                dest: <T::Lookup as StaticLookup>::Source,
                value: BalanceOf<T>, gas_limit: T::Gas, data: Vec<u8>)
     -> Result {
        let origin = ensure_signed(origin)?;
        let dest = T::Lookup::lookup(dest)?;
        let (mut gas_meter, imbalance) =
            gas::buy_gas::<T>(&origin, gas_limit)?;
        let cfg = Config::preload();
        let vm = crate::wasm::WasmVm::new(&cfg.schedule);
        let loader = crate::wasm::WasmLoader::new(&cfg.schedule);
        let mut ctx =
            ExecutionContext::top_level(origin.clone(), &cfg, &vm, &loader);
        let result =
            ctx.call(dest, value, &mut gas_meter, &data,
                     exec::EmptyOutputBuf::new());
        if let Ok(_) = result {
            DirectAccountDb.commit(ctx.overlay.into_change_set());
            ctx.events.into_iter().for_each(|indexed_event|
                                                {
                                                    <system::Module<T>>::deposit_event_indexed(&*indexed_event.topics,
                                                                                               <T
                                                                                                   as
                                                                                                   Trait>::from(indexed_event.event).into());
                                                });
        }
        gas::refund_unused_gas::<T>(&origin, gas_meter, imbalance);
        ctx.calls.into_iter().for_each(|(who, call)|
                                           {
                                               let result =
                                                   call.dispatch(RawOrigin::Signed(who.clone()).into());
                                               Self::deposit_event(RawEvent::Dispatched(who,
                                                                                        result.is_ok()));
                                           });
        result.map(|_| ())
    }
    #[doc =
          r" Creates a new contract from the `codehash` generated by `put_code`, optionally transferring some balance."]
    #[doc = r""]
    #[doc = r" Creation is executed as follows:"]
    #[doc = r""]
    #[doc =
          r" - The destination address is computed based on the sender and hash of the code."]
    #[doc =
          r" - The smart-contract account is created at the computed address."]
    #[doc =
          r" - The `ctor_code` is executed in the context of the newly-created account. Buffer returned"]
    #[doc =
          r"   after the execution is saved as the `code` of the account. That code will be invoked"]
    #[doc = r"   upon any call received by this account."]
    #[doc = r" - The contract is initialized."]
    pub fn create(origin: <T as system::Trait>::Origin,
                  endowment: BalanceOf<T>, gas_limit: T::Gas,
                  code_hash: CodeHash<T>, data: Vec<u8>) -> Result {
        let origin = ensure_signed(origin)?;
        let (mut gas_meter, imbalance) =
            gas::buy_gas::<T>(&origin, gas_limit)?;
        let cfg = Config::preload();
        let vm = crate::wasm::WasmVm::new(&cfg.schedule);
        let loader = crate::wasm::WasmLoader::new(&cfg.schedule);
        let mut ctx =
            ExecutionContext::top_level(origin.clone(), &cfg, &vm, &loader);
        let result =
            ctx.instantiate(endowment, &mut gas_meter, &code_hash, &data);
        if let Ok(_) = result {
            DirectAccountDb.commit(ctx.overlay.into_change_set());
            ctx.events.into_iter().for_each(|indexed_event|
                                                {
                                                    <system::Module<T>>::deposit_event_indexed(&*indexed_event.topics,
                                                                                               <T
                                                                                                   as
                                                                                                   Trait>::from(indexed_event.event).into());
                                                });
        }
        gas::refund_unused_gas::<T>(&origin, gas_meter, imbalance);
        ctx.calls.into_iter().for_each(|(who, call)|
                                           {
                                               let result =
                                                   call.dispatch(RawOrigin::Signed(who.clone()).into());
                                               Self::deposit_event(RawEvent::Dispatched(who,
                                                                                        result.is_ok()));
                                           });
        result.map(|_| ())
    }
    #[doc =
          r" Allows block producers to claim a small reward for evicting a contract. If a block producer"]
    #[doc =
          r" fails to do so, a regular users will be allowed to claim the reward."]
    #[doc = r""]
    #[doc =
          r" If contract is not evicted as a result of this call, no actions are taken and"]
    #[doc = r" the sender is not eligible for the reward."]
    fn claim_surcharge(origin: <T as system::Trait>::Origin,
                       dest: T::AccountId, aux_sender: Option<T::AccountId>)
     -> ::srml_support::dispatch::Result {
        {
            let origin = origin.into();
            let (signed, rewarded) =
                match origin {
                    Some(system::RawOrigin::Signed(ref account)) if
                    aux_sender.is_none() => {
                        (true, account)
                    }
                    Some(system::RawOrigin::None) if aux_sender.is_some() => {
                        (false, aux_sender.as_ref().expect("checked above"))
                    }
                    _ =>
                    return Err("Invalid surcharge claim: origin must be signed or \
								inherent and auxiliary sender only provided on inherent"),
                };
            let handicap =
                if signed {
                    <Module<T>>::signed_claim_handicap()
                } else { Zero::zero() };
            if rent::try_evict::<T>(&dest, handicap) ==
                   rent::RentOutcome::Evicted {
                T::Currency::deposit_into_existing(rewarded,
                                                   Self::surcharge_reward())?;
            }
        }
        Ok(())
    }
}
#[doc = r" Contracts module."]
pub enum Call<T: Trait> {

    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                  ::srml_support::dispatch::Never),

    #[allow(non_camel_case_types)]
    update_schedule(Schedule<T::Gas>),

    #[allow(non_camel_case_types)]
    put_code(
             #[codec(compact)]
             T::Gas, Vec<u8>),

    #[allow(non_camel_case_types)]
    call(<T::Lookup as StaticLookup>::Source,
         #[codec(compact)]
         BalanceOf<T>,
         #[codec(compact)]
         T::Gas, Vec<u8>),

    #[allow(non_camel_case_types)]
    create(
           #[codec(compact)]
           BalanceOf<T>,
           #[codec(compact)]
           T::Gas, CodeHash<T>, Vec<u8>),

    #[allow(non_camel_case_types)]
    claim_surcharge(T::AccountId, Option<T::AccountId>),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for Call<T> where
         Schedule<T::Gas>: _parity_codec::Encode,
         Schedule<T::Gas>: _parity_codec::Encode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         CodeHash<T>: _parity_codec::Encode,
         CodeHash<T>: _parity_codec::Encode,
         T::AccountId: _parity_codec::Encode,
         T::AccountId: _parity_codec::Encode,
         Option<T::AccountId>: _parity_codec::Encode,
         Option<T::AccountId>: _parity_codec::Encode,
         T::Gas: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::Gas: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::Gas: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::update_schedule(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    Call::put_code(ref aa, ref ba) => {
                        dest.push_byte(1usize as u8);
                        {
                            dest.push(&<<T::Gas as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Gas>>::from(aa));
                        }
                        dest.push(ba);
                    }
                    Call::call(ref aa, ref ba, ref ca, ref da) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                        {
                            dest.push(&<<BalanceOf<T> as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      BalanceOf<T>>>::from(ba));
                        }
                        {
                            dest.push(&<<T::Gas as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Gas>>::from(ca));
                        }
                        dest.push(da);
                    }
                    Call::create(ref aa, ref ba, ref ca, ref da) => {
                        dest.push_byte(3usize as u8);
                        {
                            dest.push(&<<BalanceOf<T> as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      BalanceOf<T>>>::from(aa));
                        }
                        {
                            dest.push(&<<T::Gas as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Gas>>::from(ba));
                        }
                        dest.push(ca);
                        dest.push(da);
                    }
                    Call::claim_surcharge(ref aa, ref ba) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                        dest.push(ba);
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
         Schedule<T::Gas>: _parity_codec::Decode,
         Schedule<T::Gas>: _parity_codec::Decode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         CodeHash<T>: _parity_codec::Decode,
         CodeHash<T>: _parity_codec::Decode,
         T::AccountId: _parity_codec::Decode,
         T::AccountId: _parity_codec::Decode,
         Option<T::AccountId>: _parity_codec::Decode,
         Option<T::AccountId>: _parity_codec::Decode,
         T::Gas: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::Gas: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::Gas: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::update_schedule(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::put_code(<<T::Gas as
                                             _parity_codec::HasCompact>::Type
                                                as
                                                _parity_codec::Decode>::decode(input)?.into(),
                                            _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(Call::call(_parity_codec::Decode::decode(input)?,
                                        <<BalanceOf<T> as
                                         _parity_codec::HasCompact>::Type as
                                            _parity_codec::Decode>::decode(input)?.into(),
                                        <<T::Gas as
                                         _parity_codec::HasCompact>::Type as
                                            _parity_codec::Decode>::decode(input)?.into(),
                                        _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 3usize as u8 => {
                        Some(Call::create(<<BalanceOf<T> as
                                           _parity_codec::HasCompact>::Type as
                                              _parity_codec::Decode>::decode(input)?.into(),
                                          <<T::Gas as
                                           _parity_codec::HasCompact>::Type as
                                              _parity_codec::Decode>::decode(input)?.into(),
                                          _parity_codec::Decode::decode(input)?,
                                          _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 4usize as u8 => {
                        Some(Call::claim_surcharge(_parity_codec::Decode::decode(input)?,
                                                   _parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::update_schedule(ref schedule) =>
            Call::update_schedule((*schedule).clone()),
            Call::put_code(ref gas_limit, ref code) =>
            Call::put_code((*gas_limit).clone(), (*code).clone()),
            Call::call(ref dest, ref value, ref gas_limit, ref data) =>
            Call::call((*dest).clone(), (*value).clone(),
                       (*gas_limit).clone(), (*data).clone()),
            Call::create(ref endowment, ref gas_limit, ref code_hash,
                         ref data) =>
            Call::create((*endowment).clone(), (*gas_limit).clone(),
                         (*code_hash).clone(), (*data).clone()),
            Call::claim_surcharge(ref dest, ref aux_sender) =>
            Call::claim_surcharge((*dest).clone(), (*aux_sender).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/contract/src/lib.rs",
                                             320u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::update_schedule(ref schedule) => {
                let self_params = (schedule,);
                if let Call::update_schedule(ref schedule) = *_other {
                    self_params == (schedule,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/contract/src/lib.rs",
                                                         320u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::put_code(ref gas_limit, ref code) => {
                let self_params = (gas_limit, code);
                if let Call::put_code(ref gas_limit, ref code) = *_other {
                    self_params == (gas_limit, code)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/contract/src/lib.rs",
                                                         320u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::call(ref dest, ref value, ref gas_limit, ref data) => {
                let self_params = (dest, value, gas_limit, data);
                if let Call::call(ref dest, ref value, ref gas_limit,
                                  ref data) = *_other {
                    self_params == (dest, value, gas_limit, data)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/contract/src/lib.rs",
                                                         320u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::create(ref endowment, ref gas_limit, ref code_hash,
                         ref data) => {
                let self_params = (endowment, gas_limit, code_hash, data);
                if let Call::create(ref endowment, ref gas_limit,
                                    ref code_hash, ref data) = *_other {
                    self_params == (endowment, gas_limit, code_hash, data)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/contract/src/lib.rs",
                                                         320u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::claim_surcharge(ref dest, ref aux_sender) => {
                let self_params = (dest, aux_sender);
                if let Call::claim_surcharge(ref dest, ref aux_sender) =
                       *_other {
                    self_params == (dest, aux_sender)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/contract/src/lib.rs",
                                                         320u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/contract/src/lib.rs",
                                             320u32, 1u32))
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
            Call::update_schedule(ref schedule) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"update_schedule",
                                                               &(schedule.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::put_code(ref gas_limit, ref code) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"put_code",
                                                               &(gas_limit.clone(),
                                                                 code.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::call(ref dest, ref value, ref gas_limit, ref data) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"call",
                                                               &(dest.clone(),
                                                                 value.clone(),
                                                                 gas_limit.clone(),
                                                                 data.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::create(ref endowment, ref gas_limit, ref code_hash,
                         ref data) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"create",
                                                               &(endowment.clone(),
                                                                 gas_limit.clone(),
                                                                 code_hash.clone(),
                                                                 data.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::claim_surcharge(ref dest, ref aux_sender) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"claim_surcharge",
                                                               &(dest.clone(),
                                                                 aux_sender.clone()))
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
                                           &("srml/contract/src/lib.rs",
                                             320u32, 1u32))
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
    <T as system::Trait>::Origin;
    fn dispatch(self, _origin: Self::Origin)
     -> ::srml_support::dispatch::Result {
        match self {
            Call::update_schedule(schedule) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::update_schedule(schedule)
                }
            }
            Call::put_code(gas_limit, code) => {
                <Module<T>>::put_code(_origin, gas_limit, code)
            }
            Call::call(dest, value, gas_limit, data) => {
                <Module<T>>::call(_origin, dest, value, gas_limit, data)
            }
            Call::create(endowment, gas_limit, code_hash, data) => {
                <Module<T>>::create(_origin, endowment, gas_limit, code_hash,
                                    data)
            }
            Call::claim_surcharge(dest, aux_sender) => {
                <Module<T>>::claim_surcharge(_origin, dest, aux_sender)
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
                                                       &("srml/contract/src/lib.rs",
                                                         320u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("update_schedule"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("schedule"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Schedule<T::Gas>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Updates the schedule for metering contracts.",
                                                                                                             r"",
                                                                                                             r" The schedule must have a greater version than the stored schedule."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("put_code"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("gas_limit"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Gas>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("code"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Stores the given binary Wasm code into the chain's storage and returns its `codehash`.",
                                                                                                             r" You can instantiate contracts only with stored code."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("call"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("dest"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("value"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("gas_limit"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Gas>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("data"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Makes a call to an account, optionally transferring some balance.",
                                                                                                             r"",
                                                                                                             r" * If the account is a smart-contract account, the associated code will be",
                                                                                                             r" executed and any value will be transferred.",
                                                                                                             r" * If the account is a regular account, any value will be transferred.",
                                                                                                             r" * If no account exists and the call value is not less than `existential_deposit`,",
                                                                                                             r" a regular account will be created and any value will be transferred."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("create"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("endowment"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("gas_limit"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Gas>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("code_hash"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("CodeHash<T>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("data"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Creates a new contract from the `codehash` generated by `put_code`, optionally transferring some balance.",
                                                                                                             r"",
                                                                                                             r" Creation is executed as follows:",
                                                                                                             r"",
                                                                                                             r" - The destination address is computed based on the sender and hash of the code.",
                                                                                                             r" - The smart-contract account is created at the computed address.",
                                                                                                             r" - The `ctor_code` is executed in the context of the newly-created account. Buffer returned",
                                                                                                             r"   after the execution is saved as the `code` of the account. That code will be invoked",
                                                                                                             r"   upon any call received by this account.",
                                                                                                             r" - The contract is initialized."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("claim_surcharge"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("dest"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("T::AccountId"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("aux_sender"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Option<T::AccountId>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Allows block producers to claim a small reward for evicting a contract. If a block producer",
                                                                                                             r" fails to do so, a regular users will be allowed to claim the reward.",
                                                                                                             r"",
                                                                                                             r" If contract is not evicted as a result of this call, no actions are taken and",
                                                                                                             r" the sender is not eligible for the reward."]),}]
    }
}
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T>
    =
    RawEvent<BalanceOf<T>, <T as system::Trait>::AccountId,
             <T as system::Trait>::Hash>;
/// Events for this module.
///
#[structural_match]
pub enum RawEvent<Balance, AccountId, Hash> {

    #[doc =
          r" Transfer happened `from` to `to` with given `value` as part of a `call` or `create`."]
    Transfer(AccountId, AccountId, Balance),

    #[doc = r" Contract deployed by address at the specified address."]
    Instantiated(AccountId, AccountId),

    #[doc = r" Code with the specified hash has been stored."]
    CodeStored(Hash),

    #[doc = r" Triggered when the current schedule is updated."]
    ScheduleUpdated(u32),

    #[doc =
          r" A call was dispatched from the given account. The bool signals whether it was"]
    #[doc = r" successful execution or not."]
    Dispatched(AccountId, bool),

    #[doc = r" An event from contract of account."]
    Contract(AccountId, Vec<u8>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::clone::Clone, AccountId: ::std::clone::Clone,
      Hash: ::std::clone::Clone> ::std::clone::Clone for
 RawEvent<Balance, AccountId, Hash> {
    #[inline]
    fn clone(&self) -> RawEvent<Balance, AccountId, Hash> {
        match (&*self,) {
            (&RawEvent::Transfer(ref __self_0, ref __self_1, ref __self_2),)
            =>
            RawEvent::Transfer(::std::clone::Clone::clone(&(*__self_0)),
                               ::std::clone::Clone::clone(&(*__self_1)),
                               ::std::clone::Clone::clone(&(*__self_2))),
            (&RawEvent::Instantiated(ref __self_0, ref __self_1),) =>
            RawEvent::Instantiated(::std::clone::Clone::clone(&(*__self_0)),
                                   ::std::clone::Clone::clone(&(*__self_1))),
            (&RawEvent::CodeStored(ref __self_0),) =>
            RawEvent::CodeStored(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::ScheduleUpdated(ref __self_0),) =>
            RawEvent::ScheduleUpdated(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::Dispatched(ref __self_0, ref __self_1),) =>
            RawEvent::Dispatched(::std::clone::Clone::clone(&(*__self_0)),
                                 ::std::clone::Clone::clone(&(*__self_1))),
            (&RawEvent::Contract(ref __self_0, ref __self_1),) =>
            RawEvent::Contract(::std::clone::Clone::clone(&(*__self_0)),
                               ::std::clone::Clone::clone(&(*__self_1))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::PartialEq, AccountId: ::std::cmp::PartialEq,
      Hash: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
 RawEvent<Balance, AccountId, Hash> {
    #[inline]
    fn eq(&self, other: &RawEvent<Balance, AccountId, Hash>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::Transfer(ref __self_0, ref __self_1,
                                         ref __self_2),
                     &RawEvent::Transfer(ref __arg_1_0, ref __arg_1_1,
                                         ref __arg_1_2)) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2),
                    (&RawEvent::Instantiated(ref __self_0, ref __self_1),
                     &RawEvent::Instantiated(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    (&RawEvent::CodeStored(ref __self_0),
                     &RawEvent::CodeStored(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::ScheduleUpdated(ref __self_0),
                     &RawEvent::ScheduleUpdated(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::Dispatched(ref __self_0, ref __self_1),
                     &RawEvent::Dispatched(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    (&RawEvent::Contract(ref __self_0, ref __self_1),
                     &RawEvent::Contract(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<Balance, AccountId, Hash>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::Transfer(ref __self_0, ref __self_1,
                                         ref __self_2),
                     &RawEvent::Transfer(ref __arg_1_0, ref __arg_1_1,
                                         ref __arg_1_2)) =>
                    (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1)
                        || (*__self_2) != (*__arg_1_2),
                    (&RawEvent::Instantiated(ref __self_0, ref __self_1),
                     &RawEvent::Instantiated(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    (&RawEvent::CodeStored(ref __self_0),
                     &RawEvent::CodeStored(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::ScheduleUpdated(ref __self_0),
                     &RawEvent::ScheduleUpdated(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::Dispatched(ref __self_0, ref __self_1),
                     &RawEvent::Dispatched(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    (&RawEvent::Contract(ref __self_0, ref __self_1),
                     &RawEvent::Contract(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::Eq, AccountId: ::std::cmp::Eq,
      Hash: ::std::cmp::Eq> ::std::cmp::Eq for
 RawEvent<Balance, AccountId, Hash> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Hash>;
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<bool>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawEvent: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance, AccountId, Hash> _parity_codec::Encode for
         RawEvent<Balance, AccountId, Hash> where
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         Balance: _parity_codec::Encode, Balance: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         Hash: _parity_codec::Encode, Hash: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::Transfer(ref aa, ref ba, ref ca) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    RawEvent::Instantiated(ref aa, ref ba) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::CodeStored(ref aa) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::ScheduleUpdated(ref aa) => {
                        dest.push_byte(3usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::Dispatched(ref aa, ref ba) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::Contract(ref aa, ref ba) => {
                        dest.push_byte(5usize as u8);
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
        impl <Balance, AccountId, Hash> _parity_codec::Decode for
         RawEvent<Balance, AccountId, Hash> where
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         Balance: _parity_codec::Decode, Balance: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         Hash: _parity_codec::Decode, Hash: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::Transfer(_parity_codec::Decode::decode(input)?,
                                                _parity_codec::Decode::decode(input)?,
                                                _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(RawEvent::Instantiated(_parity_codec::Decode::decode(input)?,
                                                    _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(RawEvent::CodeStored(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 3usize as u8 => {
                        Some(RawEvent::ScheduleUpdated(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 4usize as u8 => {
                        Some(RawEvent::Dispatched(_parity_codec::Decode::decode(input)?,
                                                  _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 5usize as u8 => {
                        Some(RawEvent::Contract(_parity_codec::Decode::decode(input)?,
                                                _parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::fmt::Debug, AccountId: ::std::fmt::Debug,
      Hash: ::std::fmt::Debug> ::std::fmt::Debug for
 RawEvent<Balance, AccountId, Hash> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawEvent::Transfer(ref __self_0, ref __self_1, ref __self_2),)
            => {
                let mut debug_trait_builder = f.debug_tuple("Transfer");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                debug_trait_builder.finish()
            }
            (&RawEvent::Instantiated(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("Instantiated");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
            (&RawEvent::CodeStored(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("CodeStored");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::ScheduleUpdated(ref __self_0),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("ScheduleUpdated");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::Dispatched(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("Dispatched");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
            (&RawEvent::Contract(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("Contract");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <Balance, AccountId, Hash> From<RawEvent<Balance, AccountId, Hash>> for
 () {
    fn from(_: RawEvent<Balance, AccountId, Hash>) -> () { () }
}
impl <Balance, AccountId, Hash> RawEvent<Balance, AccountId, Hash> {
    #[allow(dead_code)]
    pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
        &[::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Transfer"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "AccountId",
                                                                                                    "Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Transfer happened `from` to `to` with given `value` as part of a `call` or `create`."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Instantiated"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "AccountId"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Contract deployed by address at the specified address."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("CodeStored"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["Hash"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Code with the specified hash has been stored."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("ScheduleUpdated"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["u32"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Triggered when the current schedule is updated."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Dispatched"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "bool"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" A call was dispatched from the given account. The bool signals whether it was",
                                                                                                    r" successful execution or not."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Contract"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "Vec<u8>"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" An event from contract of account."]),}]
    }
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = " Number of block delay an extrinsic claim surcharge has."]
#[doc = ""]
#[doc = " When claim surchage is called by an extrinsic the rent is checked"]
#[doc = " for current_block - delay"]
struct SignedClaimHandicap<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for SignedClaimHandicap<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract SignedClaimHandicap".as_bytes() }
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
#[doc = " The minimum amount required to generate a tombstone."]
struct TombstoneDeposit<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for TombstoneDeposit<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract TombstoneDeposit".as_bytes() }
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
#[doc =
      " Size of a contract at the time of creation. This is a simple way to ensure"]
#[doc = " that empty contracts eventually gets deleted."]
struct StorageSizeOffset<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>
 for StorageSizeOffset<T> {
    type
    Query
    =
    u64;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract StorageSizeOffset".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::key()).unwrap_or_else(||
                                                                                                                                                             Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::put(&val,
                                                                                                                           storage);
        ret
    }
}
#[doc =
      " Price of a byte of storage per one block interval. Should be greater than 0."]
struct RentByteFee<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for RentByteFee<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract RentByteFee".as_bytes() }
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
#[doc = " The amount of funds a contract should deposit in order to offset"]
#[doc = " the cost of one byte."]
#[doc = ""]
#[doc =
      " Let\'s suppose the deposit is 1,000 BU (balance units)/byte and the rent is 1 BU/byte/day,"]
#[doc =
      " then a contract with 1,000,000 BU that uses 1,000 bytes of storage would pay no rent."]
#[doc =
      " But if the balance reduced to 500,000 BU and the storage stayed the same at 1,000,"]
#[doc = " then it would pay 500 BU/day."]
struct RentDepositOffset<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for RentDepositOffset<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract RentDepositOffset".as_bytes() }
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
#[doc = " Reward that is received by the party whose touch has led"]
#[doc = " to removal of a contract."]
struct SurchargeReward<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for SurchargeReward<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract SurchargeReward".as_bytes() }
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
#[doc = " The fee required to make a transfer."]
struct TransferFee<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for TransferFee<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract TransferFee".as_bytes() }
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
#[doc = " The fee required to create an account."]
struct CreationFee<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for CreationFee<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract CreationFee".as_bytes() }
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
#[doc = " The fee to be paid for making a transaction; the base."]
struct TransactionBaseFee<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for TransactionBaseFee<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract TransactionBaseFee".as_bytes() }
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
#[doc = " The fee to be paid for making a transaction; the per-byte portion."]
struct TransactionByteFee<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for TransactionByteFee<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract TransactionByteFee".as_bytes() }
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
#[doc = " The fee required to create a contract instance."]
struct ContractFee<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for ContractFee<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract ContractFee".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                      BalanceOf::<T>::sa(21))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                       BalanceOf::<T>::sa(21))
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
#[doc = " The base fee charged for calling into a contract."]
struct CallBaseFee<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>
 for CallBaseFee<T> {
    type
    Query
    =
    T::Gas;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract CallBaseFee".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::key()).unwrap_or_else(||
                                                                                                                                                                T::Gas::sa(135))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::key()).unwrap_or_else(||
                                                                                                                                                                 T::Gas::sa(135))
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::put(&val,
                                                                                                                              storage);
        ret
    }
}
#[doc = " The base fee charged for creating a contract."]
struct CreateBaseFee<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>
 for CreateBaseFee<T> {
    type
    Query
    =
    T::Gas;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract CreateBaseFee".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::key()).unwrap_or_else(||
                                                                                                                                                                T::Gas::sa(175))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::key()).unwrap_or_else(||
                                                                                                                                                                 T::Gas::sa(175))
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::put(&val,
                                                                                                                              storage);
        ret
    }
}
#[doc = " The price of one unit of gas."]
struct GasPrice<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for GasPrice<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract GasPrice".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                      BalanceOf::<T>::sa(1))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::key()).unwrap_or_else(||
                                                                                                                                                                       BalanceOf::<T>::sa(1))
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
#[doc = " The maximum nesting level of a call/create stack."]
struct MaxDepth<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
 for MaxDepth<T> {
    type
    Query
    =
    u32;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract MaxDepth".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                             100)
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                              100)
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
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&val,
                                                                                                                           storage);
        ret
    }
}
#[doc = " The maximum amount of gas that could be expended per block."]
struct BlockGasLimit<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>
 for BlockGasLimit<T> {
    type
    Query
    =
    T::Gas;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract BlockGasLimit".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::key()).unwrap_or_else(||
                                                                                                                                                                T::Gas::sa(10_000_000))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::key()).unwrap_or_else(||
                                                                                                                                                                 T::Gas::sa(10_000_000))
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::put(&val,
                                                                                                                              storage);
        ret
    }
}
#[doc = " Gas spent so far in this block."]
struct GasSpent<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>
 for GasSpent<T> {
    type
    Query
    =
    T::Gas;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract GasSpent".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::key()).unwrap_or_else(||
                                                                                                                                                                Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::put(&val,
                                                                                                                              storage);
        ret
    }
}
#[doc = " Current cost schedule for contracts."]
struct CurrentSchedule<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Schedule<T::Gas>>
 for CurrentSchedule<T> {
    type
    Query
    =
    Schedule<T::Gas>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract CurrentSchedule".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Schedule<T::Gas>>>::key()).unwrap_or_else(||
                                                                                                                                                                          Schedule::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Schedule<T::Gas>>>::key()).unwrap_or_else(||
                                                                                                                                                                           Schedule::default())
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Schedule<T::Gas>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Schedule<T::Gas>>>::put(&val,
                                                                                                                                        storage);
        ret
    }
}
#[doc =
      " A mapping from an original code hash to the original code, untouched by instrumentation."]
pub struct PristineCode<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                   Vec<u8>>
 for PristineCode<T> {
    type
    Query
    =
    Option<Vec<u8>>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Contract PristineCode".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &CodeHash<T>)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  Vec<u8>>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &CodeHash<T>,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  Vec<u8>>>::key_for(key);
        storage.get(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &CodeHash<T>,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  Vec<u8>>>::key_for(key);
        storage.take(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &CodeHash<T>,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  Vec<u8>>>::get(key,
                                                                                                                                 storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  Vec<u8>>>::insert(key,
                                                                                                                                    &val,
                                                                                                                                    storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  Vec<u8>>>::remove(key,
                                                                                                                                    storage),
        };
        ret
    }
}
#[doc =
      " A mapping between an original code hash and instrumented wasm code, ready for execution."]
pub struct CodeStorage<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                   wasm::PrefabWasmModule>
 for CodeStorage<T> {
    type
    Query
    =
    Option<wasm::PrefabWasmModule>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Contract CodeStorage".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &CodeHash<T>)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  wasm::PrefabWasmModule>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(x,
                                                                                            &mut key);
        key
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &CodeHash<T>,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  wasm::PrefabWasmModule>>::key_for(key);
        storage.get(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &CodeHash<T>,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        let key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  wasm::PrefabWasmModule>>::key_for(key);
        storage.take(&key[..]).or_else(|| Default::default())
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &CodeHash<T>,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        let mut val =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  wasm::PrefabWasmModule>>::get(key,
                                                                                                                                                storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  wasm::PrefabWasmModule>>::insert(key,
                                                                                                                                                   &val,
                                                                                                                                                   storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<CodeHash<T>,
                                                                                                                  wasm::PrefabWasmModule>>::remove(key,
                                                                                                                                                   storage),
        };
        ret
    }
}
#[doc = " The subtrie counter."]
pub struct AccountCounter<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>
 for AccountCounter<T> {
    type
    Query
    =
    u64;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Contract AccountCounter".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::key()).unwrap_or_else(||
                                                                                                                                                             0)
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::key()).unwrap_or_else(||
                                                                                                                                                              0)
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::put(&val,
                                                                                                                           storage);
        ret
    }
}
#[doc = " The code associated with a given account."]
pub struct ContractInfoOf<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   ContractInfo<T>>
 for ContractInfoOf<T> {
    type
    Query
    =
    Option<ContractInfo<T>>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Contract ContractInfoOf".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  ContractInfo<T>>>::prefix().to_vec();
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
                                                                                                                  ContractInfo<T>>>::key_for(key);
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
                                                                                                                  ContractInfo<T>>>::key_for(key);
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
                                                                                                                  ContractInfo<T>>>::get(key,
                                                                                                                                         storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  ContractInfo<T>>>::insert(key,
                                                                                                                                            &val,
                                                                                                                                            storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  ContractInfo<T>>>::remove(key,
                                                                                                                                            storage),
        };
        ret
    }
}
trait Store {
    type
    SignedClaimHandicap;
    type
    TombstoneDeposit;
    type
    StorageSizeOffset;
    type
    RentByteFee;
    type
    RentDepositOffset;
    type
    SurchargeReward;
    type
    TransferFee;
    type
    CreationFee;
    type
    TransactionBaseFee;
    type
    TransactionByteFee;
    type
    ContractFee;
    type
    CallBaseFee;
    type
    CreateBaseFee;
    type
    GasPrice;
    type
    MaxDepth;
    type
    BlockGasLimit;
    type
    GasSpent;
    type
    CurrentSchedule;
    type
    PristineCode;
    type
    CodeStorage;
    type
    AccountCounter;
    type
    ContractInfoOf;
}
#[doc(hidden)]
pub struct __GetByteStructSignedClaimHandicap<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_SignedClaimHandicap:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructSignedClaimHandicap<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_SignedClaimHandicap.get_or_init(||
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
pub struct __GetByteStructTombstoneDeposit<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TombstoneDeposit:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructTombstoneDeposit<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TombstoneDeposit.get_or_init(||
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
pub struct __GetByteStructStorageSizeOffset<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_StorageSizeOffset:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructStorageSizeOffset<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_StorageSizeOffset.get_or_init(||
                                                                  {
                                                                      let def_val:
                                                                              u64 =
                                                                          Default::default();
                                                                      <u64 as
                                                                          Encode>::encode(&def_val)
                                                                  }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructRentByteFee<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_RentByteFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructRentByteFee<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_RentByteFee.get_or_init(||
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
pub struct __GetByteStructRentDepositOffset<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_RentDepositOffset:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructRentDepositOffset<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_RentDepositOffset.get_or_init(||
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
pub struct __GetByteStructSurchargeReward<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_SurchargeReward:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructSurchargeReward<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_SurchargeReward.get_or_init(||
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
pub struct __GetByteStructTransferFee<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TransferFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructTransferFee<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TransferFee.get_or_init(||
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
pub struct __GetByteStructCreationFee<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CreationFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCreationFee<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CreationFee.get_or_init(||
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
pub struct __GetByteStructTransactionBaseFee<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TransactionBaseFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructTransactionBaseFee<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TransactionBaseFee.get_or_init(||
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
pub struct __GetByteStructTransactionByteFee<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TransactionByteFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructTransactionByteFee<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TransactionByteFee.get_or_init(||
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
pub struct __GetByteStructContractFee<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ContractFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructContractFee<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ContractFee.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        BalanceOf<T> =
                                                                    BalanceOf::<T>::sa(21);
                                                                <BalanceOf<T>
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructCallBaseFee<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CallBaseFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCallBaseFee<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CallBaseFee.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        T::Gas =
                                                                    T::Gas::sa(135);
                                                                <T::Gas as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructCreateBaseFee<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CreateBaseFee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCreateBaseFee<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CreateBaseFee.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          T::Gas =
                                                                      T::Gas::sa(175);
                                                                  <T::Gas as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructGasPrice<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_GasPrice:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructGasPrice<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_GasPrice.get_or_init(||
                                                         {
                                                             let def_val:
                                                                     BalanceOf<T> =
                                                                 BalanceOf::<T>::sa(1);
                                                             <BalanceOf<T> as
                                                                 Encode>::encode(&def_val)
                                                         }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructMaxDepth<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_MaxDepth:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructMaxDepth<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_MaxDepth.get_or_init(||
                                                         {
                                                             let def_val:
                                                                     u32 =
                                                                 100;
                                                             <u32 as
                                                                 Encode>::encode(&def_val)
                                                         }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructBlockGasLimit<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_BlockGasLimit:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructBlockGasLimit<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_BlockGasLimit.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          T::Gas =
                                                                      T::Gas::sa(10_000_000);
                                                                  <T::Gas as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructGasSpent<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_GasSpent:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructGasSpent<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_GasSpent.get_or_init(||
                                                         {
                                                             let def_val:
                                                                     T::Gas =
                                                                 Default::default();
                                                             <T::Gas as
                                                                 Encode>::encode(&def_val)
                                                         }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructCurrentSchedule<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CurrentSchedule:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCurrentSchedule<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CurrentSchedule.get_or_init(||
                                                                {
                                                                    let def_val:
                                                                            Schedule<T::Gas> =
                                                                        Schedule::default();
                                                                    <Schedule<T::Gas>
                                                                        as
                                                                        Encode>::encode(&def_val)
                                                                }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructPristineCode<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_PristineCode:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructPristineCode<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_PristineCode.get_or_init(||
                                                             {
                                                                 let def_val:
                                                                         Option<Vec<u8>> =
                                                                     Default::default();
                                                                 <Option<Vec<u8>>
                                                                     as
                                                                     Encode>::encode(&def_val)
                                                             }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructCodeStorage<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CodeStorage:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCodeStorage<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CodeStorage.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        Option<wasm::PrefabWasmModule> =
                                                                    Default::default();
                                                                <Option<wasm::PrefabWasmModule>
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructAccountCounter<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_AccountCounter:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructAccountCounter<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_AccountCounter.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           u64 =
                                                                       0;
                                                                   <u64 as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructContractInfoOf<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ContractInfoOf:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructContractInfoOf<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ContractInfoOf.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           Option<ContractInfo<T>> =
                                                                       Default::default();
                                                                   <Option<ContractInfo<T>>
                                                                       as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    SignedClaimHandicap
    =
    SignedClaimHandicap<T>;
    type
    TombstoneDeposit
    =
    TombstoneDeposit<T>;
    type
    StorageSizeOffset
    =
    StorageSizeOffset<T>;
    type
    RentByteFee
    =
    RentByteFee<T>;
    type
    RentDepositOffset
    =
    RentDepositOffset<T>;
    type
    SurchargeReward
    =
    SurchargeReward<T>;
    type
    TransferFee
    =
    TransferFee<T>;
    type
    CreationFee
    =
    CreationFee<T>;
    type
    TransactionBaseFee
    =
    TransactionBaseFee<T>;
    type
    TransactionByteFee
    =
    TransactionByteFee<T>;
    type
    ContractFee
    =
    ContractFee<T>;
    type
    CallBaseFee
    =
    CallBaseFee<T>;
    type
    CreateBaseFee
    =
    CreateBaseFee<T>;
    type
    GasPrice
    =
    GasPrice<T>;
    type
    MaxDepth
    =
    MaxDepth<T>;
    type
    BlockGasLimit
    =
    BlockGasLimit<T>;
    type
    GasSpent
    =
    GasSpent<T>;
    type
    CurrentSchedule
    =
    CurrentSchedule<T>;
    type
    PristineCode
    =
    PristineCode<T>;
    type
    CodeStorage
    =
    CodeStorage<T>;
    type
    AccountCounter
    =
    AccountCounter<T>;
    type
    ContractInfoOf
    =
    ContractInfoOf<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc = " Number of block delay an extrinsic claim surcharge has."]
    #[doc = ""]
    #[doc =
          " When claim surchage is called by an extrinsic the rent is checked"]
    #[doc = " for current_block - delay"]
    pub fn signed_claim_handicap() -> T::BlockNumber {
        <SignedClaimHandicap<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The minimum amount required to generate a tombstone."]
    pub fn tombstone_deposit() -> BalanceOf<T> {
        <TombstoneDeposit<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Size of a contract at the time of creation. This is a simple way to ensure"]
    #[doc = " that empty contracts eventually gets deleted."]
    pub fn storage_size_offset() -> u64 {
        <StorageSizeOffset<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Price of a byte of storage per one block interval. Should be greater than 0."]
    pub fn rent_byte_price() -> BalanceOf<T> {
        <RentByteFee<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The amount of funds a contract should deposit in order to offset"]
    #[doc = " the cost of one byte."]
    #[doc = ""]
    #[doc =
          " Let\'s suppose the deposit is 1,000 BU (balance units)/byte and the rent is 1 BU/byte/day,"]
    #[doc =
          " then a contract with 1,000,000 BU that uses 1,000 bytes of storage would pay no rent."]
    #[doc =
          " But if the balance reduced to 500,000 BU and the storage stayed the same at 1,000,"]
    #[doc = " then it would pay 500 BU/day."]
    pub fn rent_deposit_offset() -> BalanceOf<T> {
        <RentDepositOffset<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Reward that is received by the party whose touch has led"]
    #[doc = " to removal of a contract."]
    pub fn surcharge_reward() -> BalanceOf<T> {
        <SurchargeReward<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The fee required to make a transfer."]
    pub fn transfer_fee() -> BalanceOf<T> {
        <TransferFee<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The fee required to create an account."]
    pub fn creation_fee() -> BalanceOf<T> {
        <CreationFee<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The fee to be paid for making a transaction; the base."]
    pub fn transaction_base_fee() -> BalanceOf<T> {
        <TransactionBaseFee<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The fee to be paid for making a transaction; the per-byte portion."]
    pub fn transaction_byte_fee() -> BalanceOf<T> {
        <TransactionByteFee<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The fee required to create a contract instance."]
    pub fn contract_fee() -> BalanceOf<T> {
        <ContractFee<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The base fee charged for calling into a contract."]
    pub fn call_base_fee() -> T::Gas {
        <CallBaseFee<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The base fee charged for creating a contract."]
    pub fn create_base_fee() -> T::Gas {
        <CreateBaseFee<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The price of one unit of gas."]
    pub fn gas_price() -> BalanceOf<T> {
        <GasPrice<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The maximum nesting level of a call/create stack."]
    pub fn max_depth() -> u32 {
        <MaxDepth<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The maximum amount of gas that could be expended per block."]
    pub fn block_gas_limit() -> T::Gas {
        <BlockGasLimit<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Gas spent so far in this block."]
    pub fn gas_spent() -> T::Gas {
        <GasSpent<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Current cost schedule for contracts."]
    pub fn current_schedule() -> Schedule<T::Gas> {
        <CurrentSchedule<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Schedule<T::Gas>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SignedClaimHandicap"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSignedClaimHandicap::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of block delay an extrinsic claim surcharge has.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " When claim surchage is called by an extrinsic the rent is checked",
                                                                                                                                                                                                                                                                                                                                                                                                    " for current_block - delay"]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TombstoneDeposit"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTombstoneDeposit::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The minimum amount required to generate a tombstone."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("StorageSizeOffset"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u64")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructStorageSizeOffset::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Size of a contract at the time of creation. This is a simple way to ensure",
                                                                                                                                                                                                                                                                                                                                                                                                    " that empty contracts eventually gets deleted."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RentByteFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRentByteFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Price of a byte of storage per one block interval. Should be greater than 0."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RentDepositOffset"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRentDepositOffset::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The amount of funds a contract should deposit in order to offset",
                                                                                                                                                                                                                                                                                                                                                                                                    " the cost of one byte.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " Let\'s suppose the deposit is 1,000 BU (balance units)/byte and the rent is 1 BU/byte/day,",
                                                                                                                                                                                                                                                                                                                                                                                                    " then a contract with 1,000,000 BU that uses 1,000 bytes of storage would pay no rent.",
                                                                                                                                                                                                                                                                                                                                                                                                    " But if the balance reduced to 500,000 BU and the storage stayed the same at 1,000,",
                                                                                                                                                                                                                                                                                                                                                                                                    " then it would pay 500 BU/day."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SurchargeReward"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSurchargeReward::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Reward that is received by the party whose touch has led",
                                                                                                                                                                                                                                                                                                                                                                                                    " to removal of a contract."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransferFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransferFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to make a transfer."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CreationFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCreationFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to create an account."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransactionBaseFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransactionBaseFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee to be paid for making a transaction; the base."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransactionByteFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransactionByteFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee to be paid for making a transaction; the per-byte portion."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ContractFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructContractFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to create a contract instance."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CallBaseFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Gas")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCallBaseFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The base fee charged for calling into a contract."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CreateBaseFee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Gas")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCreateBaseFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The base fee charged for creating a contract."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("GasPrice"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructGasPrice::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The price of one unit of gas."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MaxDepth"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMaxDepth::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The maximum nesting level of a call/create stack."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BlockGasLimit"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Gas")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBlockGasLimit::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The maximum amount of gas that could be expended per block."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("GasSpent"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Gas")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructGasSpent::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Gas spent so far in this block."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentSchedule"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Schedule<T::Gas>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentSchedule::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Current cost schedule for contracts."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PristineCode"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CodeHash<T>"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<u8>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPristineCode::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" A mapping from an original code hash to the original code, untouched by instrumentation."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CodeStorage"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CodeHash<T>"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("wasm::PrefabWasmModule"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCodeStorage::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" A mapping between an original code hash and instrumented wasm code, ready for execution."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AccountCounter"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u64")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructAccountCounter::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The subtrie counter."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ContractInfoOf"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ContractInfo<T>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructContractInfoOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The code associated with a given account."]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SignedClaimHandicap"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSignedClaimHandicap::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of block delay an extrinsic claim surcharge has.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " When claim surchage is called by an extrinsic the rent is checked",
                                                                                                                                                                                                              " for current_block - delay"]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TombstoneDeposit"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTombstoneDeposit::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The minimum amount required to generate a tombstone."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("StorageSizeOffset"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u64")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructStorageSizeOffset::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Size of a contract at the time of creation. This is a simple way to ensure",
                                                                                                                                                                                                              " that empty contracts eventually gets deleted."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RentByteFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRentByteFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Price of a byte of storage per one block interval. Should be greater than 0."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RentDepositOffset"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRentDepositOffset::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The amount of funds a contract should deposit in order to offset",
                                                                                                                                                                                                              " the cost of one byte.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " Let\'s suppose the deposit is 1,000 BU (balance units)/byte and the rent is 1 BU/byte/day,",
                                                                                                                                                                                                              " then a contract with 1,000,000 BU that uses 1,000 bytes of storage would pay no rent.",
                                                                                                                                                                                                              " But if the balance reduced to 500,000 BU and the storage stayed the same at 1,000,",
                                                                                                                                                                                                              " then it would pay 500 BU/day."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SurchargeReward"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSurchargeReward::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Reward that is received by the party whose touch has led",
                                                                                                                                                                                                              " to removal of a contract."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransferFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransferFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to make a transfer."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CreationFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCreationFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to create an account."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransactionBaseFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransactionBaseFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee to be paid for making a transaction; the base."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("TransactionByteFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructTransactionByteFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee to be paid for making a transaction; the per-byte portion."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ContractFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructContractFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The fee required to create a contract instance."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CallBaseFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Gas")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCallBaseFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The base fee charged for calling into a contract."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CreateBaseFee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Gas")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCreateBaseFee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The base fee charged for creating a contract."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("GasPrice"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructGasPrice::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The price of one unit of gas."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MaxDepth"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMaxDepth::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The maximum nesting level of a call/create stack."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BlockGasLimit"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Gas")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBlockGasLimit::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The maximum amount of gas that could be expended per block."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("GasSpent"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Gas")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructGasSpent::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Gas spent so far in this block."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentSchedule"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Schedule<T::Gas>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentSchedule::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Current cost schedule for contracts."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PristineCode"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CodeHash<T>"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<u8>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPristineCode::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" A mapping from an original code hash to the original code, untouched by instrumentation."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CodeStorage"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CodeHash<T>"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("wasm::PrefabWasmModule"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCodeStorage::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" A mapping between an original code hash and instrumented wasm code, ready for execution."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("AccountCounter"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u64")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructAccountCounter::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The subtrie counter."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ContractInfoOf"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ContractInfo<T>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructContractInfoOf::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The code associated with a given account."]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Contract" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, u64 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Gas : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Gas : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Gas : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Schedule < T :: Gas > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, u64 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Gas : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Gas : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Gas : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Schedule < T :: Gas > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    #[doc = " Number of block delay an extrinsic claim surcharge has."]
    #[doc = ""]
    #[doc =
          " When claim surchage is called by an extrinsic the rent is checked"]
    #[doc = " for current_block - delay"]
    pub signed_claim_handicap: T::BlockNumber,
    #[doc = " The minimum amount required to generate a tombstone."]
    pub tombstone_deposit: BalanceOf<T>,
    #[doc =
          " Size of a contract at the time of creation. This is a simple way to ensure"]
    #[doc = " that empty contracts eventually gets deleted."]
    pub storage_size_offset: u64,
    #[doc =
          " Price of a byte of storage per one block interval. Should be greater than 0."]
    pub rent_byte_price: BalanceOf<T>,
    #[doc =
          " The amount of funds a contract should deposit in order to offset"]
    #[doc = " the cost of one byte."]
    #[doc = ""]
    #[doc =
          " Let\'s suppose the deposit is 1,000 BU (balance units)/byte and the rent is 1 BU/byte/day,"]
    #[doc =
          " then a contract with 1,000,000 BU that uses 1,000 bytes of storage would pay no rent."]
    #[doc =
          " But if the balance reduced to 500,000 BU and the storage stayed the same at 1,000,"]
    #[doc = " then it would pay 500 BU/day."]
    pub rent_deposit_offset: BalanceOf<T>,
    #[doc = " Reward that is received by the party whose touch has led"]
    #[doc = " to removal of a contract."]
    pub surcharge_reward: BalanceOf<T>,
    #[doc = " The fee required to make a transfer."]
    pub transfer_fee: BalanceOf<T>,
    #[doc = " The fee required to create an account."]
    pub creation_fee: BalanceOf<T>,
    #[doc = " The fee to be paid for making a transaction; the base."]
    pub transaction_base_fee: BalanceOf<T>,
    #[doc =
          " The fee to be paid for making a transaction; the per-byte portion."]
    pub transaction_byte_fee: BalanceOf<T>,
    #[doc = " The fee required to create a contract instance."]
    pub contract_fee: BalanceOf<T>,
    #[doc = " The base fee charged for calling into a contract."]
    pub call_base_fee: T::Gas,
    #[doc = " The base fee charged for creating a contract."]
    pub create_base_fee: T::Gas,
    #[doc = " The price of one unit of gas."]
    pub gas_price: BalanceOf<T>,
    #[doc = " The maximum nesting level of a call/create stack."]
    pub max_depth: u32,
    #[doc = " The maximum amount of gas that could be expended per block."]
    pub block_gas_limit: T::Gas,
    #[doc = " Current cost schedule for contracts."]
    pub current_schedule: Schedule<T::Gas>,
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
         u64: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Schedule<T::Gas>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
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
                                                                    "signedClaimHandicap",
                                                                    &self.signed_claim_handicap)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "tombstoneDeposit",
                                                                    &self.tombstone_deposit)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "storageSizeOffset",
                                                                    &self.storage_size_offset)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "rentBytePrice",
                                                                    &self.rent_byte_price)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "rentDepositOffset",
                                                                    &self.rent_deposit_offset)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "surchargeReward",
                                                                    &self.surcharge_reward)
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
                                                                    "contractFee",
                                                                    &self.contract_fee)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "callBaseFee",
                                                                    &self.call_base_fee)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "createBaseFee",
                                                                    &self.create_base_fee)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "gasPrice",
                                                                    &self.gas_price)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "maxDepth",
                                                                    &self.max_depth)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "blockGasLimit",
                                                                    &self.block_gas_limit)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "currentSchedule",
                                                                    &self.current_schedule)
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
         u64: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Schedule<T::Gas>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __field15,
                    __field16,
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
                            14u64 => _serde::export::Ok(__Field::__field14),
                            15u64 => _serde::export::Ok(__Field::__field15),
                            16u64 => _serde::export::Ok(__Field::__field16),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 17")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "signedClaimHandicap" =>
                            _serde::export::Ok(__Field::__field0),
                            "tombstoneDeposit" =>
                            _serde::export::Ok(__Field::__field1),
                            "storageSizeOffset" =>
                            _serde::export::Ok(__Field::__field2),
                            "rentBytePrice" =>
                            _serde::export::Ok(__Field::__field3),
                            "rentDepositOffset" =>
                            _serde::export::Ok(__Field::__field4),
                            "surchargeReward" =>
                            _serde::export::Ok(__Field::__field5),
                            "transferFee" =>
                            _serde::export::Ok(__Field::__field6),
                            "creationFee" =>
                            _serde::export::Ok(__Field::__field7),
                            "transactionBaseFee" =>
                            _serde::export::Ok(__Field::__field8),
                            "transactionByteFee" =>
                            _serde::export::Ok(__Field::__field9),
                            "contractFee" =>
                            _serde::export::Ok(__Field::__field10),
                            "callBaseFee" =>
                            _serde::export::Ok(__Field::__field11),
                            "createBaseFee" =>
                            _serde::export::Ok(__Field::__field12),
                            "gasPrice" =>
                            _serde::export::Ok(__Field::__field13),
                            "maxDepth" =>
                            _serde::export::Ok(__Field::__field14),
                            "blockGasLimit" =>
                            _serde::export::Ok(__Field::__field15),
                            "currentSchedule" =>
                            _serde::export::Ok(__Field::__field16),
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
                            b"signedClaimHandicap" =>
                            _serde::export::Ok(__Field::__field0),
                            b"tombstoneDeposit" =>
                            _serde::export::Ok(__Field::__field1),
                            b"storageSizeOffset" =>
                            _serde::export::Ok(__Field::__field2),
                            b"rentBytePrice" =>
                            _serde::export::Ok(__Field::__field3),
                            b"rentDepositOffset" =>
                            _serde::export::Ok(__Field::__field4),
                            b"surchargeReward" =>
                            _serde::export::Ok(__Field::__field5),
                            b"transferFee" =>
                            _serde::export::Ok(__Field::__field6),
                            b"creationFee" =>
                            _serde::export::Ok(__Field::__field7),
                            b"transactionBaseFee" =>
                            _serde::export::Ok(__Field::__field8),
                            b"transactionByteFee" =>
                            _serde::export::Ok(__Field::__field9),
                            b"contractFee" =>
                            _serde::export::Ok(__Field::__field10),
                            b"callBaseFee" =>
                            _serde::export::Ok(__Field::__field11),
                            b"createBaseFee" =>
                            _serde::export::Ok(__Field::__field12),
                            b"gasPrice" =>
                            _serde::export::Ok(__Field::__field13),
                            b"maxDepth" =>
                            _serde::export::Ok(__Field::__field14),
                            b"blockGasLimit" =>
                            _serde::export::Ok(__Field::__field15),
                            b"currentSchedule" =>
                            _serde::export::Ok(__Field::__field16),
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
                       u64: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Schedule<T::Gas>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 u64: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Gas: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Schedule<T::Gas>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                                                                                                 &"struct GenesisConfig with 17 elements"));
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
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(6usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(7usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(8usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field9 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(9usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field10 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(10usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field11 =
                            match match _serde::de::SeqAccess::next_element::<T::Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(11usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field12 =
                            match match _serde::de::SeqAccess::next_element::<T::Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(12usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field13 =
                            match match _serde::de::SeqAccess::next_element::<BalanceOf<T>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(13usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field14 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(14usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field15 =
                            match match _serde::de::SeqAccess::next_element::<T::Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(15usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        let __field16 =
                            match match _serde::de::SeqAccess::next_element::<Schedule<T::Gas>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(16usize,
                                                                                                 &"struct GenesisConfig with 17 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{signed_claim_handicap:
                                                             __field0,
                                                         tombstone_deposit:
                                                             __field1,
                                                         storage_size_offset:
                                                             __field2,
                                                         rent_byte_price:
                                                             __field3,
                                                         rent_deposit_offset:
                                                             __field4,
                                                         surcharge_reward:
                                                             __field5,
                                                         transfer_fee:
                                                             __field6,
                                                         creation_fee:
                                                             __field7,
                                                         transaction_base_fee:
                                                             __field8,
                                                         transaction_byte_fee:
                                                             __field9,
                                                         contract_fee:
                                                             __field10,
                                                         call_base_fee:
                                                             __field11,
                                                         create_base_fee:
                                                             __field12,
                                                         gas_price: __field13,
                                                         max_depth: __field14,
                                                         block_gas_limit:
                                                             __field15,
                                                         current_schedule:
                                                             __field16,})
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
                        let mut __field2: _serde::export::Option<u64> =
                            _serde::export::None;
                        let mut __field3:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field4:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field5:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field6:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field7:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field8:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field9:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field10:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field11: _serde::export::Option<T::Gas> =
                            _serde::export::None;
                        let mut __field12: _serde::export::Option<T::Gas> =
                            _serde::export::None;
                        let mut __field13:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field14: _serde::export::Option<u32> =
                            _serde::export::None;
                        let mut __field15: _serde::export::Option<T::Gas> =
                            _serde::export::None;
                        let mut __field16:
                                _serde::export::Option<Schedule<T::Gas>> =
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
                                                                       _serde::de::Error>::duplicate_field("signedClaimHandicap"));
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
                                                                       _serde::de::Error>::duplicate_field("tombstoneDeposit"));
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
                                                                       _serde::de::Error>::duplicate_field("storageSizeOffset"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<u64>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("rentBytePrice"));
                                    }
                                    __field3 =
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
                                __Field::__field4 => {
                                    if _serde::export::Option::is_some(&__field4)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("rentDepositOffset"));
                                    }
                                    __field4 =
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
                                __Field::__field5 => {
                                    if _serde::export::Option::is_some(&__field5)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("surchargeReward"));
                                    }
                                    __field5 =
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
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("transferFee"));
                                    }
                                    __field6 =
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
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("creationFee"));
                                    }
                                    __field7 =
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
                                __Field::__field8 => {
                                    if _serde::export::Option::is_some(&__field8)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("transactionBaseFee"));
                                    }
                                    __field8 =
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
                                __Field::__field9 => {
                                    if _serde::export::Option::is_some(&__field9)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("transactionByteFee"));
                                    }
                                    __field9 =
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
                                __Field::__field10 => {
                                    if _serde::export::Option::is_some(&__field10)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("contractFee"));
                                    }
                                    __field10 =
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
                                __Field::__field11 => {
                                    if _serde::export::Option::is_some(&__field11)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("callBaseFee"));
                                    }
                                    __field11 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("createBaseFee"));
                                    }
                                    __field12 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("gasPrice"));
                                    }
                                    __field13 =
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
                                __Field::__field14 => {
                                    if _serde::export::Option::is_some(&__field14)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("maxDepth"));
                                    }
                                    __field14 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field15 => {
                                    if _serde::export::Option::is_some(&__field15)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("blockGasLimit"));
                                    }
                                    __field15 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Gas>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field16 => {
                                    if _serde::export::Option::is_some(&__field16)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("currentSchedule"));
                                    }
                                    __field16 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Schedule<T::Gas>>(&mut __map)
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
                                match _serde::private::de::missing_field("signedClaimHandicap")
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
                                match _serde::private::de::missing_field("tombstoneDeposit")
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
                                match _serde::private::de::missing_field("storageSizeOffset")
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
                                match _serde::private::de::missing_field("rentBytePrice")
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
                                match _serde::private::de::missing_field("rentDepositOffset")
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
                                match _serde::private::de::missing_field("surchargeReward")
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
                                match _serde::private::de::missing_field("transferFee")
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
                                match _serde::private::de::missing_field("creationFee")
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
                                match _serde::private::de::missing_field("transactionBaseFee")
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
                                match _serde::private::de::missing_field("transactionByteFee")
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
                                match _serde::private::de::missing_field("contractFee")
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
                                match _serde::private::de::missing_field("callBaseFee")
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
                                match _serde::private::de::missing_field("createBaseFee")
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
                                match _serde::private::de::missing_field("gasPrice")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field14 =
                            match __field14 {
                                _serde::export::Some(__field14) => __field14,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("maxDepth")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field15 =
                            match __field15 {
                                _serde::export::Some(__field15) => __field15,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("blockGasLimit")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field16 =
                            match __field16 {
                                _serde::export::Some(__field16) => __field16,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("currentSchedule")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{signed_claim_handicap:
                                                             __field0,
                                                         tombstone_deposit:
                                                             __field1,
                                                         storage_size_offset:
                                                             __field2,
                                                         rent_byte_price:
                                                             __field3,
                                                         rent_deposit_offset:
                                                             __field4,
                                                         surcharge_reward:
                                                             __field5,
                                                         transfer_fee:
                                                             __field6,
                                                         creation_fee:
                                                             __field7,
                                                         transaction_base_fee:
                                                             __field8,
                                                         transaction_byte_fee:
                                                             __field9,
                                                         contract_fee:
                                                             __field10,
                                                         call_base_fee:
                                                             __field11,
                                                         create_base_fee:
                                                             __field12,
                                                         gas_price: __field13,
                                                         max_depth: __field14,
                                                         block_gas_limit:
                                                             __field15,
                                                         current_schedule:
                                                             __field16,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["signedClaimHandicap", "tombstoneDeposit",
                      "storageSizeOffset", "rentBytePrice",
                      "rentDepositOffset", "surchargeReward", "transferFee",
                      "creationFee", "transactionBaseFee",
                      "transactionByteFee", "contractFee", "callBaseFee",
                      "createBaseFee", "gasPrice", "maxDepth",
                      "blockGasLimit", "currentSchedule"];
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
        GenesisConfig{signed_claim_handicap: Default::default(),
                      tombstone_deposit: Default::default(),
                      storage_size_offset: Default::default(),
                      rent_byte_price: Default::default(),
                      rent_deposit_offset: Default::default(),
                      surcharge_reward: Default::default(),
                      transfer_fee: Default::default(),
                      creation_fee: Default::default(),
                      transaction_base_fee: Default::default(),
                      transaction_byte_fee: Default::default(),
                      contract_fee: BalanceOf::<T>::sa(21),
                      call_base_fee: T::Gas::sa(135),
                      create_base_fee: T::Gas::sa(175),
                      gas_price: BalanceOf::<T>::sa(1),
                      max_depth: 100,
                      block_gas_limit: T::Gas::sa(10_000_000),
                      current_schedule: Schedule::default(),}
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
                      config.signed_claim_handicap.clone()))(&self);
            <SignedClaimHandicap<T> as
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
                      config.tombstone_deposit.clone()))(&self);
            <TombstoneDeposit<T> as
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
                      config.storage_size_offset.clone()))(&self);
            <StorageSizeOffset<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u64>>::put(&v,
                                                                                                                               &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.rent_byte_price.clone()))(&self);
            <RentByteFee<T> as
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
                      config.rent_deposit_offset.clone()))(&self);
            <RentDepositOffset<T> as
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
                      config.surcharge_reward.clone()))(&self);
            <SurchargeReward<T> as
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
                      config.transfer_fee.clone()))(&self);
            <TransferFee<T> as
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
                      config.creation_fee.clone()))(&self);
            <CreationFee<T> as
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
                      config.transaction_base_fee.clone()))(&self);
            <TransactionBaseFee<T> as
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
                      config.transaction_byte_fee.clone()))(&self);
            <TransactionByteFee<T> as
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
                      config.contract_fee.clone()))(&self);
            <ContractFee<T> as
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
                      config.call_base_fee.clone()))(&self);
            <CallBaseFee<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::put(&v,
                                                                                                                                  &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.create_base_fee.clone()))(&self);
            <CreateBaseFee<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::put(&v,
                                                                                                                                  &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.gas_price.clone()))(&self);
            <GasPrice<T> as
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
                      config.max_depth.clone()))(&self);
            <MaxDepth<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&v,
                                                                                                                               &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.block_gas_limit.clone()))(&self);
            <BlockGasLimit<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Gas>>::put(&v,
                                                                                                                                  &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.current_schedule.clone()))(&self);
            <CurrentSchedule<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Schedule<T::Gas>>>::put(&v,
                                                                                                                                            &storage);
        }
        let r = storage.into_inner();
        (|_, _, _| { })(r, c, &self);
        Ok(())
    }
}
impl <T: Trait> OnFreeBalanceZero<T::AccountId> for Module<T> {
    fn on_free_balance_zero(who: &T::AccountId) {
        if let Some(ContractInfo::Alive(info)) = <ContractInfoOf<T>>::get(who)
               {
            child::kill_storage(&info.trie_id);
        }
        <ContractInfoOf<T>>::remove(who);
    }
}
/// In-memory cache of configuration values.
///
/// We assume that these values can't be changed in the
/// course of transaction execution.
pub struct Config<T: Trait> {
    pub schedule: Schedule<T::Gas>,
    pub existential_deposit: BalanceOf<T>,
    pub max_depth: u32,
    pub contract_account_instantiate_fee: BalanceOf<T>,
    pub account_create_fee: BalanceOf<T>,
    pub transfer_fee: BalanceOf<T>,
    pub call_base_fee: T::Gas,
    pub instantiate_base_fee: T::Gas,
}
impl <T: Trait> Config<T> {
    fn preload() -> Config<T> {
        Config{schedule: <Module<T>>::current_schedule(),
               existential_deposit: T::Currency::minimum_balance(),
               max_depth: <Module<T>>::max_depth(),
               contract_account_instantiate_fee: <Module<T>>::contract_fee(),
               account_create_fee: <Module<T>>::creation_fee(),
               transfer_fee: <Module<T>>::transfer_fee(),
               call_base_fee: <Module<T>>::call_base_fee(),
               instantiate_base_fee: <Module<T>>::create_base_fee(),}
    }
}
/// Definition of the cost schedule and other parameterizations for wasm vm.
#[structural_match]
pub struct Schedule<Gas> {
    /// Version of the schedule.
    pub version: u32,
    /// Cost of putting a byte of code into storage.
    pub put_code_per_byte_cost: Gas,
    /// Gas cost of a growing memory by single page.
    pub grow_mem_cost: Gas,
    /// Gas cost of a regular operation.
    pub regular_op_cost: Gas,
    /// Gas cost per one byte returned.
    pub return_data_per_byte_cost: Gas,
    /// Gas cost to deposit an event; the per-byte portion.
    pub event_data_per_byte_cost: Gas,
    /// Gas cost to deposit an event; the cost per topic.
    pub event_per_topic_cost: Gas,
    /// Gas cost to deposit an event; the base.
    pub event_base_cost: Gas,
    /// Gas cost per one byte read from the sandbox memory.
    pub sandbox_data_read_cost: Gas,
    /// Gas cost per one byte written to the sandbox memory.
    pub sandbox_data_write_cost: Gas,
    /// The maximum number of topics supported by an event.
    pub max_event_topics: u32,
    /// Maximum allowed stack height.
    ///
    /// See https://wiki.parity.io/WebAssembly-StackHeight to find out
    /// how the stack frame cost is calculated.
    pub max_stack_height: u32,
    /// Maximum number of memory pages allowed for a contract.
    pub max_memory_pages: u32,
    /// Whether the `ext_println` function is allowed to be used contracts.
    /// MUST only be enabled for `dev` chains, NOT for production chains
    pub enable_println: bool,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Schedule: () =
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
        impl <Gas> _serde::Serialize for Schedule<Gas> where
         Gas: _serde::Serialize {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "Schedule",
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
                                                                    "version",
                                                                    &self.version)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "put_code_per_byte_cost",
                                                                    &self.put_code_per_byte_cost)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "grow_mem_cost",
                                                                    &self.grow_mem_cost)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "regular_op_cost",
                                                                    &self.regular_op_cost)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "return_data_per_byte_cost",
                                                                    &self.return_data_per_byte_cost)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "event_data_per_byte_cost",
                                                                    &self.event_data_per_byte_cost)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "event_per_topic_cost",
                                                                    &self.event_per_topic_cost)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "event_base_cost",
                                                                    &self.event_base_cost)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "sandbox_data_read_cost",
                                                                    &self.sandbox_data_read_cost)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "sandbox_data_write_cost",
                                                                    &self.sandbox_data_write_cost)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "max_event_topics",
                                                                    &self.max_event_topics)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "max_stack_height",
                                                                    &self.max_stack_height)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "max_memory_pages",
                                                                    &self.max_memory_pages)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "enable_println",
                                                                    &self.enable_println)
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
const _IMPL_DESERIALIZE_FOR_Schedule: () =
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
        impl <'de, Gas> _serde::Deserialize<'de> for Schedule<Gas> where
         Gas: _serde::Deserialize<'de> {
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
                            "version" =>
                            _serde::export::Ok(__Field::__field0),
                            "put_code_per_byte_cost" =>
                            _serde::export::Ok(__Field::__field1),
                            "grow_mem_cost" =>
                            _serde::export::Ok(__Field::__field2),
                            "regular_op_cost" =>
                            _serde::export::Ok(__Field::__field3),
                            "return_data_per_byte_cost" =>
                            _serde::export::Ok(__Field::__field4),
                            "event_data_per_byte_cost" =>
                            _serde::export::Ok(__Field::__field5),
                            "event_per_topic_cost" =>
                            _serde::export::Ok(__Field::__field6),
                            "event_base_cost" =>
                            _serde::export::Ok(__Field::__field7),
                            "sandbox_data_read_cost" =>
                            _serde::export::Ok(__Field::__field8),
                            "sandbox_data_write_cost" =>
                            _serde::export::Ok(__Field::__field9),
                            "max_event_topics" =>
                            _serde::export::Ok(__Field::__field10),
                            "max_stack_height" =>
                            _serde::export::Ok(__Field::__field11),
                            "max_memory_pages" =>
                            _serde::export::Ok(__Field::__field12),
                            "enable_println" =>
                            _serde::export::Ok(__Field::__field13),
                            _ => { _serde::export::Ok(__Field::__ignore) }
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8])
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            b"version" =>
                            _serde::export::Ok(__Field::__field0),
                            b"put_code_per_byte_cost" =>
                            _serde::export::Ok(__Field::__field1),
                            b"grow_mem_cost" =>
                            _serde::export::Ok(__Field::__field2),
                            b"regular_op_cost" =>
                            _serde::export::Ok(__Field::__field3),
                            b"return_data_per_byte_cost" =>
                            _serde::export::Ok(__Field::__field4),
                            b"event_data_per_byte_cost" =>
                            _serde::export::Ok(__Field::__field5),
                            b"event_per_topic_cost" =>
                            _serde::export::Ok(__Field::__field6),
                            b"event_base_cost" =>
                            _serde::export::Ok(__Field::__field7),
                            b"sandbox_data_read_cost" =>
                            _serde::export::Ok(__Field::__field8),
                            b"sandbox_data_write_cost" =>
                            _serde::export::Ok(__Field::__field9),
                            b"max_event_topics" =>
                            _serde::export::Ok(__Field::__field10),
                            b"max_stack_height" =>
                            _serde::export::Ok(__Field::__field11),
                            b"max_memory_pages" =>
                            _serde::export::Ok(__Field::__field12),
                            b"enable_println" =>
                            _serde::export::Ok(__Field::__field13),
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
                struct __Visitor<'de, Gas> where
                       Gas: _serde::Deserialize<'de> {
                    marker: _serde::export::PhantomData<Schedule<Gas>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, Gas> _serde::de::Visitor<'de> for
                 __Visitor<'de, Gas> where Gas: _serde::Deserialize<'de> {
                    type
                    Value
                    =
                    Schedule<Gas>;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "struct Schedule")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(6usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(7usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(8usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field9 =
                            match match _serde::de::SeqAccess::next_element::<Gas>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(9usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field10 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(10usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field11 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(11usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field12 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(12usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        let __field13 =
                            match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(13usize,
                                                                                                 &"struct Schedule with 14 elements"));
                                }
                            };
                        _serde::export::Ok(Schedule{version: __field0,
                                                    put_code_per_byte_cost:
                                                        __field1,
                                                    grow_mem_cost: __field2,
                                                    regular_op_cost: __field3,
                                                    return_data_per_byte_cost:
                                                        __field4,
                                                    event_data_per_byte_cost:
                                                        __field5,
                                                    event_per_topic_cost:
                                                        __field6,
                                                    event_base_cost: __field7,
                                                    sandbox_data_read_cost:
                                                        __field8,
                                                    sandbox_data_write_cost:
                                                        __field9,
                                                    max_event_topics:
                                                        __field10,
                                                    max_stack_height:
                                                        __field11,
                                                    max_memory_pages:
                                                        __field12,
                                                    enable_println:
                                                        __field13,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0: _serde::export::Option<u32> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<Gas> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<Gas> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<Gas> =
                            _serde::export::None;
                        let mut __field4: _serde::export::Option<Gas> =
                            _serde::export::None;
                        let mut __field5: _serde::export::Option<Gas> =
                            _serde::export::None;
                        let mut __field6: _serde::export::Option<Gas> =
                            _serde::export::None;
                        let mut __field7: _serde::export::Option<Gas> =
                            _serde::export::None;
                        let mut __field8: _serde::export::Option<Gas> =
                            _serde::export::None;
                        let mut __field9: _serde::export::Option<Gas> =
                            _serde::export::None;
                        let mut __field10: _serde::export::Option<u32> =
                            _serde::export::None;
                        let mut __field11: _serde::export::Option<u32> =
                            _serde::export::None;
                        let mut __field12: _serde::export::Option<u32> =
                            _serde::export::None;
                        let mut __field13: _serde::export::Option<bool> =
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
                                                                       _serde::de::Error>::duplicate_field("version"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("put_code_per_byte_cost"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("grow_mem_cost"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("regular_op_cost"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("return_data_per_byte_cost"));
                                    }
                                    __field4 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("event_data_per_byte_cost"));
                                    }
                                    __field5 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("event_per_topic_cost"));
                                    }
                                    __field6 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("event_base_cost"));
                                    }
                                    __field7 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("sandbox_data_read_cost"));
                                    }
                                    __field8 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("sandbox_data_write_cost"));
                                    }
                                    __field9 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Gas>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("max_event_topics"));
                                    }
                                    __field10 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("max_stack_height"));
                                    }
                                    __field11 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("max_memory_pages"));
                                    }
                                    __field12 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<u32>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("enable_println"));
                                    }
                                    __field13 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<bool>(&mut __map)
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
                                match _serde::private::de::missing_field("version")
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
                                match _serde::private::de::missing_field("put_code_per_byte_cost")
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
                                match _serde::private::de::missing_field("grow_mem_cost")
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
                                match _serde::private::de::missing_field("regular_op_cost")
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
                                match _serde::private::de::missing_field("return_data_per_byte_cost")
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
                                match _serde::private::de::missing_field("event_data_per_byte_cost")
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
                                match _serde::private::de::missing_field("event_per_topic_cost")
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
                                match _serde::private::de::missing_field("event_base_cost")
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
                                match _serde::private::de::missing_field("sandbox_data_read_cost")
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
                                match _serde::private::de::missing_field("sandbox_data_write_cost")
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
                                match _serde::private::de::missing_field("max_event_topics")
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
                                match _serde::private::de::missing_field("max_stack_height")
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
                                match _serde::private::de::missing_field("max_memory_pages")
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
                                match _serde::private::de::missing_field("enable_println")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(Schedule{version: __field0,
                                                    put_code_per_byte_cost:
                                                        __field1,
                                                    grow_mem_cost: __field2,
                                                    regular_op_cost: __field3,
                                                    return_data_per_byte_cost:
                                                        __field4,
                                                    event_data_per_byte_cost:
                                                        __field5,
                                                    event_per_topic_cost:
                                                        __field6,
                                                    event_base_cost: __field7,
                                                    sandbox_data_read_cost:
                                                        __field8,
                                                    sandbox_data_write_cost:
                                                        __field9,
                                                    max_event_topics:
                                                        __field10,
                                                    max_stack_height:
                                                        __field11,
                                                    max_memory_pages:
                                                        __field12,
                                                    enable_println:
                                                        __field13,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["version", "put_code_per_byte_cost", "grow_mem_cost",
                      "regular_op_cost", "return_data_per_byte_cost",
                      "event_data_per_byte_cost", "event_per_topic_cost",
                      "event_base_cost", "sandbox_data_read_cost",
                      "sandbox_data_write_cost", "max_event_topics",
                      "max_stack_height", "max_memory_pages",
                      "enable_println"];
                _serde::Deserializer::deserialize_struct(__deserializer,
                                                         "Schedule", FIELDS,
                                                         __Visitor{marker:
                                                                       _serde::export::PhantomData::<Schedule<Gas>>,
                                                                   lifetime:
                                                                       _serde::export::PhantomData,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Gas: ::std::fmt::Debug> ::std::fmt::Debug for Schedule<Gas> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Schedule {
            version: ref __self_0_0,
            put_code_per_byte_cost: ref __self_0_1,
            grow_mem_cost: ref __self_0_2,
            regular_op_cost: ref __self_0_3,
            return_data_per_byte_cost: ref __self_0_4,
            event_data_per_byte_cost: ref __self_0_5,
            event_per_topic_cost: ref __self_0_6,
            event_base_cost: ref __self_0_7,
            sandbox_data_read_cost: ref __self_0_8,
            sandbox_data_write_cost: ref __self_0_9,
            max_event_topics: ref __self_0_10,
            max_stack_height: ref __self_0_11,
            max_memory_pages: ref __self_0_12,
            enable_println: ref __self_0_13 } => {
                let mut debug_trait_builder = f.debug_struct("Schedule");
                let _ = debug_trait_builder.field("version", &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("put_code_per_byte_cost",
                                              &&(*__self_0_1));
                let _ =
                    debug_trait_builder.field("grow_mem_cost",
                                              &&(*__self_0_2));
                let _ =
                    debug_trait_builder.field("regular_op_cost",
                                              &&(*__self_0_3));
                let _ =
                    debug_trait_builder.field("return_data_per_byte_cost",
                                              &&(*__self_0_4));
                let _ =
                    debug_trait_builder.field("event_data_per_byte_cost",
                                              &&(*__self_0_5));
                let _ =
                    debug_trait_builder.field("event_per_topic_cost",
                                              &&(*__self_0_6));
                let _ =
                    debug_trait_builder.field("event_base_cost",
                                              &&(*__self_0_7));
                let _ =
                    debug_trait_builder.field("sandbox_data_read_cost",
                                              &&(*__self_0_8));
                let _ =
                    debug_trait_builder.field("sandbox_data_write_cost",
                                              &&(*__self_0_9));
                let _ =
                    debug_trait_builder.field("max_event_topics",
                                              &&(*__self_0_10));
                let _ =
                    debug_trait_builder.field("max_stack_height",
                                              &&(*__self_0_11));
                let _ =
                    debug_trait_builder.field("max_memory_pages",
                                              &&(*__self_0_12));
                let _ =
                    debug_trait_builder.field("enable_println",
                                              &&(*__self_0_13));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Gas: ::std::clone::Clone> ::std::clone::Clone for Schedule<Gas> {
    #[inline]
    fn clone(&self) -> Schedule<Gas> {
        match *self {
            Schedule {
            version: ref __self_0_0,
            put_code_per_byte_cost: ref __self_0_1,
            grow_mem_cost: ref __self_0_2,
            regular_op_cost: ref __self_0_3,
            return_data_per_byte_cost: ref __self_0_4,
            event_data_per_byte_cost: ref __self_0_5,
            event_per_topic_cost: ref __self_0_6,
            event_base_cost: ref __self_0_7,
            sandbox_data_read_cost: ref __self_0_8,
            sandbox_data_write_cost: ref __self_0_9,
            max_event_topics: ref __self_0_10,
            max_stack_height: ref __self_0_11,
            max_memory_pages: ref __self_0_12,
            enable_println: ref __self_0_13 } =>
            Schedule{version: ::std::clone::Clone::clone(&(*__self_0_0)),
                     put_code_per_byte_cost:
                         ::std::clone::Clone::clone(&(*__self_0_1)),
                     grow_mem_cost:
                         ::std::clone::Clone::clone(&(*__self_0_2)),
                     regular_op_cost:
                         ::std::clone::Clone::clone(&(*__self_0_3)),
                     return_data_per_byte_cost:
                         ::std::clone::Clone::clone(&(*__self_0_4)),
                     event_data_per_byte_cost:
                         ::std::clone::Clone::clone(&(*__self_0_5)),
                     event_per_topic_cost:
                         ::std::clone::Clone::clone(&(*__self_0_6)),
                     event_base_cost:
                         ::std::clone::Clone::clone(&(*__self_0_7)),
                     sandbox_data_read_cost:
                         ::std::clone::Clone::clone(&(*__self_0_8)),
                     sandbox_data_write_cost:
                         ::std::clone::Clone::clone(&(*__self_0_9)),
                     max_event_topics:
                         ::std::clone::Clone::clone(&(*__self_0_10)),
                     max_stack_height:
                         ::std::clone::Clone::clone(&(*__self_0_11)),
                     max_memory_pages:
                         ::std::clone::Clone::clone(&(*__self_0_12)),
                     enable_println:
                         ::std::clone::Clone::clone(&(*__self_0_13)),},
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Schedule: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Gas> _parity_codec::Encode for Schedule<Gas> where
         Gas: _parity_codec::Encode, Gas: _parity_codec::Encode,
         Gas: _parity_codec::Encode, Gas: _parity_codec::Encode,
         Gas: _parity_codec::Encode, Gas: _parity_codec::Encode,
         Gas: _parity_codec::Encode, Gas: _parity_codec::Encode,
         Gas: _parity_codec::Encode, Gas: _parity_codec::Encode,
         Gas: _parity_codec::Encode, Gas: _parity_codec::Encode,
         Gas: _parity_codec::Encode, Gas: _parity_codec::Encode,
         Gas: _parity_codec::Encode, Gas: _parity_codec::Encode,
         Gas: _parity_codec::Encode, Gas: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.version);
                dest.push(&self.put_code_per_byte_cost);
                dest.push(&self.grow_mem_cost);
                dest.push(&self.regular_op_cost);
                dest.push(&self.return_data_per_byte_cost);
                dest.push(&self.event_data_per_byte_cost);
                dest.push(&self.event_per_topic_cost);
                dest.push(&self.event_base_cost);
                dest.push(&self.sandbox_data_read_cost);
                dest.push(&self.sandbox_data_write_cost);
                dest.push(&self.max_event_topics);
                dest.push(&self.max_stack_height);
                dest.push(&self.max_memory_pages);
                dest.push(&self.enable_println);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Schedule: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Gas> _parity_codec::Decode for Schedule<Gas> where
         Gas: _parity_codec::Decode, Gas: _parity_codec::Decode,
         Gas: _parity_codec::Decode, Gas: _parity_codec::Decode,
         Gas: _parity_codec::Decode, Gas: _parity_codec::Decode,
         Gas: _parity_codec::Decode, Gas: _parity_codec::Decode,
         Gas: _parity_codec::Decode, Gas: _parity_codec::Decode,
         Gas: _parity_codec::Decode, Gas: _parity_codec::Decode,
         Gas: _parity_codec::Decode, Gas: _parity_codec::Decode,
         Gas: _parity_codec::Decode, Gas: _parity_codec::Decode,
         Gas: _parity_codec::Decode, Gas: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(Schedule{version: _parity_codec::Decode::decode(input)?,
                              put_code_per_byte_cost:
                                  _parity_codec::Decode::decode(input)?,
                              grow_mem_cost:
                                  _parity_codec::Decode::decode(input)?,
                              regular_op_cost:
                                  _parity_codec::Decode::decode(input)?,
                              return_data_per_byte_cost:
                                  _parity_codec::Decode::decode(input)?,
                              event_data_per_byte_cost:
                                  _parity_codec::Decode::decode(input)?,
                              event_per_topic_cost:
                                  _parity_codec::Decode::decode(input)?,
                              event_base_cost:
                                  _parity_codec::Decode::decode(input)?,
                              sandbox_data_read_cost:
                                  _parity_codec::Decode::decode(input)?,
                              sandbox_data_write_cost:
                                  _parity_codec::Decode::decode(input)?,
                              max_event_topics:
                                  _parity_codec::Decode::decode(input)?,
                              max_stack_height:
                                  _parity_codec::Decode::decode(input)?,
                              max_memory_pages:
                                  _parity_codec::Decode::decode(input)?,
                              enable_println:
                                  _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Gas: ::std::cmp::PartialEq> ::std::cmp::PartialEq for Schedule<Gas> {
    #[inline]
    fn eq(&self, other: &Schedule<Gas>) -> bool {
        match *other {
            Schedule {
            version: ref __self_1_0,
            put_code_per_byte_cost: ref __self_1_1,
            grow_mem_cost: ref __self_1_2,
            regular_op_cost: ref __self_1_3,
            return_data_per_byte_cost: ref __self_1_4,
            event_data_per_byte_cost: ref __self_1_5,
            event_per_topic_cost: ref __self_1_6,
            event_base_cost: ref __self_1_7,
            sandbox_data_read_cost: ref __self_1_8,
            sandbox_data_write_cost: ref __self_1_9,
            max_event_topics: ref __self_1_10,
            max_stack_height: ref __self_1_11,
            max_memory_pages: ref __self_1_12,
            enable_println: ref __self_1_13 } =>
            match *self {
                Schedule {
                version: ref __self_0_0,
                put_code_per_byte_cost: ref __self_0_1,
                grow_mem_cost: ref __self_0_2,
                regular_op_cost: ref __self_0_3,
                return_data_per_byte_cost: ref __self_0_4,
                event_data_per_byte_cost: ref __self_0_5,
                event_per_topic_cost: ref __self_0_6,
                event_base_cost: ref __self_0_7,
                sandbox_data_read_cost: ref __self_0_8,
                sandbox_data_write_cost: ref __self_0_9,
                max_event_topics: ref __self_0_10,
                max_stack_height: ref __self_0_11,
                max_memory_pages: ref __self_0_12,
                enable_println: ref __self_0_13 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3) &&
                    (*__self_0_4) == (*__self_1_4) &&
                    (*__self_0_5) == (*__self_1_5) &&
                    (*__self_0_6) == (*__self_1_6) &&
                    (*__self_0_7) == (*__self_1_7) &&
                    (*__self_0_8) == (*__self_1_8) &&
                    (*__self_0_9) == (*__self_1_9) &&
                    (*__self_0_10) == (*__self_1_10) &&
                    (*__self_0_11) == (*__self_1_11) &&
                    (*__self_0_12) == (*__self_1_12) &&
                    (*__self_0_13) == (*__self_1_13),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Schedule<Gas>) -> bool {
        match *other {
            Schedule {
            version: ref __self_1_0,
            put_code_per_byte_cost: ref __self_1_1,
            grow_mem_cost: ref __self_1_2,
            regular_op_cost: ref __self_1_3,
            return_data_per_byte_cost: ref __self_1_4,
            event_data_per_byte_cost: ref __self_1_5,
            event_per_topic_cost: ref __self_1_6,
            event_base_cost: ref __self_1_7,
            sandbox_data_read_cost: ref __self_1_8,
            sandbox_data_write_cost: ref __self_1_9,
            max_event_topics: ref __self_1_10,
            max_stack_height: ref __self_1_11,
            max_memory_pages: ref __self_1_12,
            enable_println: ref __self_1_13 } =>
            match *self {
                Schedule {
                version: ref __self_0_0,
                put_code_per_byte_cost: ref __self_0_1,
                grow_mem_cost: ref __self_0_2,
                regular_op_cost: ref __self_0_3,
                return_data_per_byte_cost: ref __self_0_4,
                event_data_per_byte_cost: ref __self_0_5,
                event_per_topic_cost: ref __self_0_6,
                event_base_cost: ref __self_0_7,
                sandbox_data_read_cost: ref __self_0_8,
                sandbox_data_write_cost: ref __self_0_9,
                max_event_topics: ref __self_0_10,
                max_stack_height: ref __self_0_11,
                max_memory_pages: ref __self_0_12,
                enable_println: ref __self_0_13 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2) ||
                    (*__self_0_3) != (*__self_1_3) ||
                    (*__self_0_4) != (*__self_1_4) ||
                    (*__self_0_5) != (*__self_1_5) ||
                    (*__self_0_6) != (*__self_1_6) ||
                    (*__self_0_7) != (*__self_1_7) ||
                    (*__self_0_8) != (*__self_1_8) ||
                    (*__self_0_9) != (*__self_1_9) ||
                    (*__self_0_10) != (*__self_1_10) ||
                    (*__self_0_11) != (*__self_1_11) ||
                    (*__self_0_12) != (*__self_1_12) ||
                    (*__self_0_13) != (*__self_1_13),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Gas: ::std::cmp::Eq> ::std::cmp::Eq for Schedule<Gas> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<Gas>;
            let _: ::std::cmp::AssertParamIsEq<Gas>;
            let _: ::std::cmp::AssertParamIsEq<Gas>;
            let _: ::std::cmp::AssertParamIsEq<Gas>;
            let _: ::std::cmp::AssertParamIsEq<Gas>;
            let _: ::std::cmp::AssertParamIsEq<Gas>;
            let _: ::std::cmp::AssertParamIsEq<Gas>;
            let _: ::std::cmp::AssertParamIsEq<Gas>;
            let _: ::std::cmp::AssertParamIsEq<Gas>;
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<bool>;
        }
    }
}
impl <Gas: As<u64>> Default for Schedule<Gas> {
    fn default() -> Schedule<Gas> {
        Schedule{version: 0,
                 put_code_per_byte_cost: Gas::sa(1),
                 grow_mem_cost: Gas::sa(1),
                 regular_op_cost: Gas::sa(1),
                 return_data_per_byte_cost: Gas::sa(1),
                 event_data_per_byte_cost: Gas::sa(1),
                 event_per_topic_cost: Gas::sa(1),
                 event_base_cost: Gas::sa(1),
                 sandbox_data_read_cost: Gas::sa(1),
                 sandbox_data_write_cost: Gas::sa(1),
                 max_event_topics: 4,
                 max_stack_height: 64 * 1024,
                 max_memory_pages: 16,
                 enable_println: false,}
    }
}
