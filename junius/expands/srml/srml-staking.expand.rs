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

//! # Staking Module
//!
//! The Staking module is used to manage funds at stake by network maintainers.
//!
//! - [`staking::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! The Staking module is the means by which a set of network maintainers (known as _authorities_ in some contexts
//! and _validators_ in others) are chosen based upon those who voluntarily place funds under deposit. Under deposit,
//! those funds are rewarded under normal operation but are held at pain of _slash_ (expropriation) should the
//! staked maintainer be found not to be discharging its duties properly.
//!
//! ### Terminology
//! <!-- Original author of paragraph: @gavofyork -->
//!
//! - Staking: The process of locking up funds for some time, placing them at risk of slashing (loss)
//! in order to become a rewarded maintainer of the network.
//! - Validating: The process of running a node to actively maintain the network, either by producing
//! blocks or guaranteeing finality of the chain.
//! - Nominating: The process of placing staked funds behind one or more validators in order to share
//! in any reward, and punishment, they take.
//! - Stash account: The account holding an owner's funds used for staking.
//! - Controller account: The account that controls an owner's funds for staking.
//! - Era: A (whole) number of sessions, which is the period that the validator set (and each validator's
//! active nominator set) is recalculated and where rewards are paid out.
//! - Slash: The punishment of a staker by reducing its funds.
//!
//! ### Goals
//! <!-- Original author of paragraph: @gavofyork -->
//!
//! The staking system in Substrate NPoS is designed to make the following possible:
//!
//! - Stake funds that are controlled by a cold wallet.
//! - Withdraw some, or deposit more, funds without interrupting the role of an entity.
//! - Switch between roles (nominator, validator, idle) with minimal overhead.
//!
//! ### Scenarios
//!
//! #### Staking
//!
//! Almost any interaction with the Staking module requires a process of _**bonding**_ (also known as
//! being a _staker_). To become *bonded*, a fund-holding account known as the _stash account_, which holds
//! some or all of the funds that become frozen in place as part of the staking process, is paired with an
//! active **controller** account, which issues instructions on how they shall be used.
//!
//! An account pair can become bonded using the [`bond`](./enum.Call.html#variant.bond) call.
//!
//! Stash accounts can change their associated controller using the
//! [`set_controller`](./enum.Call.html#variant.set_controller) call.
//!
//! There are three possible roles that any staked account pair can be in: `Validator`, `Nominator` and `Idle`
//! (defined in [`StakerStatus`](./enum.StakerStatus.html)). There are three corresponding instructions to change between roles, namely:
//! [`validate`](./enum.Call.html#variant.validate), [`nominate`](./enum.Call.html#variant.nominate),
//! and [`chill`](./enum.Call.html#variant.chill).
//!
//! #### Validating
//!
//! A **validator** takes the role of either validating blocks or ensuring their finality, maintaining the veracity of
//! the network. A validator should avoid both any sort of malicious misbehavior and going offline.
//! Bonded accounts that state interest in being a validator do NOT get immediately chosen as a validator. Instead, they
//! are declared as a _candidate_ and they _might_ get elected at the _next era_ as a validator. The result of the
//! election is determined by nominators and their votes.
//!
//! An account can become a validator candidate via the [`validate`](./enum.Call.html#variant.validate) call.
//!
//! #### Nomination
//!
//! A **nominator** does not take any _direct_ role in maintaining the network, instead, it votes on a set of validators
//! to be elected. Once interest in nomination is stated by an account, it takes effect at the next election round. The
//! funds in the nominator's stash account indicate the _weight_ of its vote.
//! Both the rewards and any punishment that a validator earns are shared between the validator and its nominators.
//! This rule incentivizes the nominators to NOT vote for the misbehaving/offline validators as much as possible, simply
//! because the nominators will also lose funds if they vote poorly.
//!
//! An account can become a nominator via the [`nominate`](enum.Call.html#variant.nominate) call.
//!
//! #### Rewards and Slash
//!
//! The **reward and slashing** procedure is the core of the Staking module, attempting to _embrace valid behavior_
//! while _punishing any misbehavior or lack of availability_.
//!
//! Slashing can occur at any point in time, once misbehavior is reported. Once slashing is determined, a value is
//! deducted from the balance of the validator and all the nominators who voted for this validator
//! (values are deducted from the _stash_ account of the slashed entity).
//!
//! Similar to slashing, rewards are also shared among a validator and its associated nominators.
//! Yet, the reward funds are not always transferred to the stash account and can be configured.
//! See [Reward Calculation](#reward-calculation) for more details.
//!
//! #### Chilling
//!
//! Finally, any of the roles above can choose to step back temporarily and just chill for a while. This means that if
//! they are a nominator, they will not be considered as voters anymore and if they are validators, they will no longer
//! be a candidate for the next election.
//!
//! An account can step back via the [`chill`](enum.Call.html#variant.chill) call.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! The dispatchable functions of the Staking module enable the steps needed for entities to accept and change their
//! role, alongside some helper functions to get/set the metadata of the module.
//!
//! ### Public Functions
//!
//! The Staking module contains many public storage items and (im)mutable functions.
//!
//! ## Usage
//!
//! ### Example: Reporting Misbehavior
//!
//! ```
//! use srml_support::{decl_module, dispatch::Result};
//! use system::ensure_signed;
//! use srml_staking::{self as staking};
//!
//! pub trait Trait: staking::Trait {}
//!
//! decl_module! {
//! 	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
//!			/// Report whoever calls this function as offline once.
//! 		pub fn report_sender(origin) -> Result {
//! 			let reported = ensure_signed(origin)?;
//! 			<staking::Module<T>>::on_offline_validator(reported, 1);
//! 			Ok(())
//! 		}
//! 	}
//! }
//! # fn main() { }
//! ```
//!
//! ## Implementation Details
//!
//! ### Slot Stake
//!
//! The term [`SlotStake`](./struct.Module.html#method.slot_stake) will be used throughout this section. It refers
//! to a value calculated at the end of each era, containing the _minimum value at stake among all validators._
//! Note that a validator's value at stake might be a combination of The validator's own stake
//! and the votes it received. See [`Exposure`](./struct.Exposure.html) for more details.
//!
//! ### Reward Calculation
//!
//! Rewards are recorded **per-session** and paid **per-era**. The value of the reward for each session is calculated at
//! the end of the session based on the timeliness of the session, then accumulated to be paid later. The value of
//! the new _per-session-reward_ is calculated at the end of each era by multiplying `SlotStake` and `SessionReward`
//! (`SessionReward` is the multiplication factor, represented by a number between 0 and 1).
//! Once a new era is triggered, rewards are paid to the validators and their associated nominators.
//!
//! The validator can declare an amount, named
//! [`validator_payment`](./struct.ValidatorPrefs.html#structfield.validator_payment), that does not get shared
//! with the nominators at each reward payout through its [`ValidatorPrefs`](./struct.ValidatorPrefs.html). This value
//! gets deducted from the total reward that can be paid. The remaining portion is split among the validator and all
//! of the nominators that nominated the validator, proportional to the value staked behind this validator (_i.e._
//! dividing the [`own`](./struct.Exposure.html#structfield.own) or [`others`](./struct.Exposure.html#structfield.others)
//! by [`total`](./struct.Exposure.html#structfield.total) in [`Exposure`](./struct.Exposure.html)).
//!
//! All entities who receive a reward have the option to choose their reward destination
//! through the [`Payee`](./struct.Payee.html) storage item (see [`set_payee`](enum.Call.html#variant.set_payee)),
//! to be one of the following:
//!
//! - Controller account, (obviously) not increasing the staked value.
//! - Stash account, not increasing the staked value.
//! - Stash account, also increasing the staked value.
//!
//! ### Slashing details
//!
//! A validator can be _reported_ to be offline at any point via the public function
//! [`on_offline_validator`](enum.Call.html#variant.on_offline_validator). Each validator declares how many times it
//! can be _reported_ before it actually gets slashed via its
//! [`unstake_threshold`](./struct.ValidatorPrefs.html#structfield.unstake_threshold).
//!
//! On top of this, the Staking module also introduces an
//! [`OfflineSlashGrace`](./struct.Module.html#method.offline_slash_grace), which applies
//! to all validators and prevents them from getting immediately slashed.
//!
//! Essentially, a validator gets slashed once they have been reported more than
//! [`OfflineSlashGrace`] + [`unstake_threshold`] times. Getting slashed due to offline report always leads
//! to being _unstaked_ (_i.e._ removed as a validator candidate) as the consequence.
//!
//! The base slash value is computed _per slash-event_ by multiplying
//! [`OfflineSlash`](./struct.Module.html#method.offline_slash) and the `total` `Exposure`. This value is then
//! multiplied by `2.pow(unstake_threshold)` to obtain the final slash value. All individual accounts' punishments are
//! capped at their total stake (NOTE: This cap should never come into force in a correctly implemented,
//! non-corrupted, well-configured system).
//!
//! ### Additional Fund Management Operations
//!
//! Any funds already placed into stash can be the target of the following operations:
//!
//! The controller account can free a portion (or all) of the funds using the [`unbond`](enum.Call.html#variant.unbond)
//! call. Note that the funds are not immediately accessible. Instead, a duration denoted by
//! [`BondingDuration`](./struct.BondingDuration.html) (in number of eras) must pass until the funds can actually be
//! removed. Once the `BondingDuration` is over, the [`withdraw_unbonded`](./enum.Call.html#variant.withdraw_unbonded) call can be used
//! to actually withdraw the funds.
//!
//! ### Election Algorithm
//!
//! The current election algorithm is implemented based on Phragmén.
//! The reference implementation can be found [here](https://github.com/w3f/consensus/tree/master/NPoS).
//!
//! The election algorithm, aside from electing the validators with the most stake value and votes, tries to divide
//! the nominator votes among candidates in an equal manner. To further assure this, an optional post-processing
//! can be applied that iteractively normalizes the nominator staked values until the total difference among
//! votes of a particular nominator are less than a threshold.
//!
//! ## GenesisConfig
//!
//! The Staking module depends on the [`GenesisConfig`](./struct.GenesisConfig.html).
//!
//! ## Related Modules
//!
//! - [Balances](../srml_balances/index.html): Used to manage values at stake.
//! - [Session](../srml_session/index.html): Used to manage sessions. Also, a list of new validators is
//! stored in the Session module's `Validators` at the end of each era.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[cfg(feature = "std")]
use runtime_io::with_storage;
use rstd::{prelude::*, result, collections::btree_map::BTreeMap};
use parity_codec::{HasCompact, Encode, Decode};
use srml_support::{StorageValue, StorageMap, EnumerableStorageMap,
                   dispatch::Result};
use srml_support::{decl_module, decl_event, decl_storage, ensure};
use srml_support::traits::{Currency, OnFreeBalanceZero, OnDilution,
                           LockIdentifier, LockableCurrency, WithdrawReasons,
                           OnUnbalanced, Imbalance};
use session::OnSessionChange;
use primitives::Perbill;
use primitives::traits::{Convert, Zero, One, As, StaticLookup, CheckedSub,
                         CheckedShl, Saturating, Bounded};
#[cfg(feature = "std")]
use primitives::{Serialize, Deserialize};
use system::ensure_signed;

mod phragmen {






























    // The historical validators and their nominations for a given era. Stored as a trie root of the mapping
    // `T::AccountId` => `Exposure<T::AccountId, BalanceOf<T>>`, which is just the contents of `Stakers`,
    // under a key that is the `era`.
    //
    // Every era change, this will be appended with the trie root of the contents of `Stakers`, and the oldest
    // entry removed down to a specific number of entries (probably around 90 for a 3 month history).
    // pub HistoricalStakers get(historical_stakers): map T::BlockNumber => Option<H256>;
















    // You're auto-bonded forever, here. We might improve this by only bonding when
    // you actually validate/nominate.









    // Avoid there being a dust balance left in the staking system.

















    // PUBLIC IMMUTABLES



    // MUTABLES (DANGEROUS)


    // The exposure (backing stake) information of the validator to be slashed.
    // The amount we are actually going to slash (can't be bigger than the validator's total exposure)
    // The amount we'll slash from the validator's stash directly.
    // The amount remaining that we can't slash from the validator, that must be taken from the nominators.
    // The total to be slashed from the nominators.
    // FIXME #1572 avoid overflow
    // best effort - not much that can be done on fail.


    // FIXME #1572:  avoid overflow


    // accumulate good session reward


    // Payout
    // validators length can never overflow u64

    // Increment current era.

    // Enact era length change.

    // Reassign all Stakers.

    // Update the balances for rewarding according to the stakes.




    // helper closure.

    // The return value of this is safe to be converted to u64.
    // The original balance, `b` is within the scope of u64. It is just extended to u128
    // to be properly multiplied by a ratio, which will lead to another value
    // less than u64 for sure. The result can then be safely passed to `to_balance`.
    // For now the backward convert is used. A simple `TryFrom<u64>` is also safe.

    // Compute the actual stake from nominator's ratio.

    // update elected candidate exposures.

    // NOTE: simple example where this saturates:
    // candidate with max_value stake. 1 nominator with max_value stake.
    // Nuked. Sadly there is not much that we can do about this.
    // See this test: phragmen_should_not_overflow_xxx()

    // This optimization will most likely be only applied off-chain.

    // Clear Stakers and reduce their slash_count.

    // Populate Stakers and figure out the minimum stake behind a slot.

    // Set the new validator set.

    // There were not enough candidates for even our minimal level of functionality.
    // This is bad.
    // We should probably disable all functionality except for block production
    // and let the chain keep producing blocks until we can decide on a sufficiently
    // substantial set.
    // TODO: #2494


    // Early exit if validator is invulnerable.




    // They're bailing.
    // Multiply slash_mantissa by 2^(unstake_threshold with upper bound)





    //! Rust implementation of the Phragmén election algorithm.
    use rstd::{prelude::*, collections::btree_map::BTreeMap};
    use primitives::{PerU128};
    use primitives::traits::{Zero, Convert, Saturating};
    use parity_codec::{Encode, Decode};
    use crate::{BalanceOf, Assignment, RawAssignment, ExpoMap, Trait,
                ValidatorPrefs};
    type Fraction = PerU128;
    /// Wrapper around the type used as the _safe_ wrapper around a `balance`.
    pub type ExtendedBalance = u128;
    const SCALE_FACTOR: ExtendedBalance =
        u32::max_value() as ExtendedBalance + 1;
    /// These are used to expose a fixed accuracy to the caller function. The bigger they are,
    /// the more accurate we get, but the more likely it is for us to overflow. The case of overflow
    /// is handled but accuracy will be lost. 32 or 16 are reasonable values.
    pub const ACCURACY: ExtendedBalance =
        u32::max_value() as ExtendedBalance + 1;
    /// Wrapper around validation candidates some metadata.
    pub struct Candidate<AccountId> {
        /// The validator's account
        pub who: AccountId,
        /// Intermediary value used to sort candidates.
        pub score: Fraction,
        /// Accumulator of the stake of this candidate based on received votes.
        approval_stake: ExtendedBalance,
        /// Flag for being elected.
        elected: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::clone::Clone> ::std::clone::Clone for
     Candidate<AccountId> {
        #[inline]
        fn clone(&self) -> Candidate<AccountId> {
            match *self {
                Candidate {
                who: ref __self_0_0,
                score: ref __self_0_1,
                approval_stake: ref __self_0_2,
                elected: ref __self_0_3 } =>
                Candidate{who: ::std::clone::Clone::clone(&(*__self_0_0)),
                          score: ::std::clone::Clone::clone(&(*__self_0_1)),
                          approval_stake:
                              ::std::clone::Clone::clone(&(*__self_0_2)),
                          elected:
                              ::std::clone::Clone::clone(&(*__self_0_3)),},
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Candidate: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <AccountId> _parity_codec::Encode for Candidate<AccountId>
             where AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.who);
                    dest.push(&self.score);
                    dest.push(&self.approval_stake);
                    dest.push(&self.elected);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Candidate: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <AccountId> _parity_codec::Decode for Candidate<AccountId>
             where AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Candidate{who: _parity_codec::Decode::decode(input)?,
                                   score:
                                       _parity_codec::Decode::decode(input)?,
                                   approval_stake:
                                       _parity_codec::Decode::decode(input)?,
                                   elected:
                                       _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::default::Default> ::std::default::Default for
     Candidate<AccountId> {
        #[inline]
        fn default() -> Candidate<AccountId> {
            Candidate{who: ::std::default::Default::default(),
                      score: ::std::default::Default::default(),
                      approval_stake: ::std::default::Default::default(),
                      elected: ::std::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::fmt::Debug> ::std::fmt::Debug for
     Candidate<AccountId> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Candidate {
                who: ref __self_0_0,
                score: ref __self_0_1,
                approval_stake: ref __self_0_2,
                elected: ref __self_0_3 } => {
                    let mut debug_trait_builder = f.debug_struct("Candidate");
                    let _ = debug_trait_builder.field("who", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("score", &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("approval_stake",
                                                  &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("elected", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Wrapper around the nomination info of a single nominator for a group of validators.
    pub struct Nominator<AccountId> {
        /// The nominator's account.
        who: AccountId,
        /// List of validators proposed by this nominator.
        edges: Vec<Edge<AccountId>>,
        /// the stake amount proposed by the nominator as a part of the vote.
        budget: ExtendedBalance,
        /// Incremented each time a nominee that this nominator voted for has been elected.
        load: Fraction,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::clone::Clone> ::std::clone::Clone for
     Nominator<AccountId> {
        #[inline]
        fn clone(&self) -> Nominator<AccountId> {
            match *self {
                Nominator {
                who: ref __self_0_0,
                edges: ref __self_0_1,
                budget: ref __self_0_2,
                load: ref __self_0_3 } =>
                Nominator{who: ::std::clone::Clone::clone(&(*__self_0_0)),
                          edges: ::std::clone::Clone::clone(&(*__self_0_1)),
                          budget: ::std::clone::Clone::clone(&(*__self_0_2)),
                          load: ::std::clone::Clone::clone(&(*__self_0_3)),},
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Nominator: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <AccountId> _parity_codec::Encode for Nominator<AccountId>
             where AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode,
             Vec<Edge<AccountId>>: _parity_codec::Encode,
             Vec<Edge<AccountId>>: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.who);
                    dest.push(&self.edges);
                    dest.push(&self.budget);
                    dest.push(&self.load);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Nominator: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <AccountId> _parity_codec::Decode for Nominator<AccountId>
             where AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode,
             Vec<Edge<AccountId>>: _parity_codec::Decode,
             Vec<Edge<AccountId>>: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Nominator{who: _parity_codec::Decode::decode(input)?,
                                   edges:
                                       _parity_codec::Decode::decode(input)?,
                                   budget:
                                       _parity_codec::Decode::decode(input)?,
                                   load:
                                       _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::default::Default> ::std::default::Default for
     Nominator<AccountId> {
        #[inline]
        fn default() -> Nominator<AccountId> {
            Nominator{who: ::std::default::Default::default(),
                      edges: ::std::default::Default::default(),
                      budget: ::std::default::Default::default(),
                      load: ::std::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::fmt::Debug> ::std::fmt::Debug for
     Nominator<AccountId> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Nominator {
                who: ref __self_0_0,
                edges: ref __self_0_1,
                budget: ref __self_0_2,
                load: ref __self_0_3 } => {
                    let mut debug_trait_builder = f.debug_struct("Nominator");
                    let _ = debug_trait_builder.field("who", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("edges", &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("budget", &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("load", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Wrapper around a nominator vote and the load of that vote.
    pub struct Edge<AccountId> {
        /// Account being voted for
        who: AccountId,
        /// Load of this vote.
        load: Fraction,
        /// Equal to `edge.load / nom.load`. Stored only to be used with post-processing.
        ratio: ExtendedBalance,
        /// Index of the candidate stored in the 'candidates' vector.
        candidate_index: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::clone::Clone> ::std::clone::Clone for
     Edge<AccountId> {
        #[inline]
        fn clone(&self) -> Edge<AccountId> {
            match *self {
                Edge {
                who: ref __self_0_0,
                load: ref __self_0_1,
                ratio: ref __self_0_2,
                candidate_index: ref __self_0_3 } =>
                Edge{who: ::std::clone::Clone::clone(&(*__self_0_0)),
                     load: ::std::clone::Clone::clone(&(*__self_0_1)),
                     ratio: ::std::clone::Clone::clone(&(*__self_0_2)),
                     candidate_index:
                         ::std::clone::Clone::clone(&(*__self_0_3)),},
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Edge: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <AccountId> _parity_codec::Encode for Edge<AccountId> where
             AccountId: _parity_codec::Encode,
             AccountId: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.who);
                    dest.push(&self.load);
                    dest.push(&self.ratio);
                    dest.push(&self.candidate_index);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Edge: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <AccountId> _parity_codec::Decode for Edge<AccountId> where
             AccountId: _parity_codec::Decode,
             AccountId: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Edge{who: _parity_codec::Decode::decode(input)?,
                              load: _parity_codec::Decode::decode(input)?,
                              ratio: _parity_codec::Decode::decode(input)?,
                              candidate_index:
                                  _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::default::Default> ::std::default::Default for
     Edge<AccountId> {
        #[inline]
        fn default() -> Edge<AccountId> {
            Edge{who: ::std::default::Default::default(),
                 load: ::std::default::Default::default(),
                 ratio: ::std::default::Default::default(),
                 candidate_index: ::std::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <AccountId: ::std::fmt::Debug> ::std::fmt::Debug for Edge<AccountId>
     {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Edge {
                who: ref __self_0_0,
                load: ref __self_0_1,
                ratio: ref __self_0_2,
                candidate_index: ref __self_0_3 } => {
                    let mut debug_trait_builder = f.debug_struct("Edge");
                    let _ = debug_trait_builder.field("who", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("load", &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("ratio", &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("candidate_index",
                                                  &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Perform election based on Phragmén algorithm.
    ///
    /// Reference implementation: https://github.com/w3f/consensus
    ///
    /// Returns an Option of elected candidates, if election is performed.
    /// Returns None if not enough candidates exist.
    ///
    /// The returned Option is a tuple consisting of:
    ///   - The list of elected candidates.
    ///   - The list of nominators and their associated vote weights.
    pub fn elect<T: Trait + 'static, FV, FN,
                 FS>(validator_count: usize, minimum_validator_count: usize,
                     validator_iter: FV, nominator_iter: FN, stash_of: FS)
     ->
         Option<(Vec<T::AccountId>,
                 Vec<(T::AccountId, Vec<RawAssignment<T>>)>)> where
     FV: Iterator<Item = (T::AccountId, ValidatorPrefs<BalanceOf<T>>)>,
     FN: Iterator<Item = (T::AccountId, Vec<T::AccountId>)>,
     for<'r> FS: Fn(&'r T::AccountId) -> BalanceOf<T> {
        let to_votes =
            |b: BalanceOf<T>|
                <T::CurrencyToVote as Convert<BalanceOf<T>, u64>>::convert(b)
                    as ExtendedBalance;
        let mut elected_candidates: Vec<T::AccountId>;
        let mut assigned: Vec<(T::AccountId, Vec<RawAssignment<T>>)>;
        let mut c_idx_cache = BTreeMap::<T::AccountId, usize>::new();
        let mut nominators: Vec<Nominator<T::AccountId>> =
            Vec::with_capacity(validator_iter.size_hint().0 +
                                   nominator_iter.size_hint().0);
        let mut candidates =
            validator_iter.map(|(who, _)|
                                   {
                                       let stash_balance = stash_of(&who);
                                       (Candidate{who, ..Default::default()},
                                        stash_balance)
                                   }).filter_map(|(mut c, s)|
                                                     {
                                                         c.approval_stake +=
                                                             to_votes(s);
                                                         if c.approval_stake.is_zero()
                                                            {
                                                             None
                                                         } else {
                                                             Some((c, s))
                                                         }
                                                     }).enumerate().map(|(idx,
                                                                          (c,
                                                                           s))|
                                                                            {
                                                                                nominators.push(Nominator{who:
                                                                                                              c.who.clone(),
                                                                                                          edges:
                                                                                                              <[_]>::into_vec(box
                                                                                                                                  [Edge{who:
                                                                                                                                            c.who.clone(),
                                                                                                                                        candidate_index:
                                                                                                                                            idx,
                                                                                                                                                   ..Default::default()}]),
                                                                                                          budget:
                                                                                                              to_votes(s),
                                                                                                          load:
                                                                                                              Fraction::zero(),});
                                                                                c_idx_cache.insert(c.who.clone(),
                                                                                                   idx);
                                                                                c
                                                                            }).collect::<Vec<Candidate<T::AccountId>>>();
        nominators.extend(nominator_iter.map(|(who, nominees)|
                                                 {
                                                     let nominator_stake =
                                                         stash_of(&who);
                                                     let mut edges:
                                                             Vec<Edge<T::AccountId>> =
                                                         Vec::with_capacity(nominees.len());
                                                     for n in &nominees {
                                                         if let Some(idx) =
                                                                c_idx_cache.get(n)
                                                                {
                                                             candidates[*idx].approval_stake
                                                                 =
                                                                 candidates[*idx].approval_stake.saturating_add(to_votes(nominator_stake));
                                                             edges.push(Edge{who:
                                                                                 n.clone(),
                                                                             candidate_index:
                                                                                 *idx,
                                                                                         ..Default::default()});
                                                         }
                                                     }
                                                     Nominator{who,
                                                               edges: edges,
                                                               budget:
                                                                   to_votes(nominator_stake),
                                                               load:
                                                                   Fraction::zero(),}
                                                 }));
        if candidates.len() >= minimum_validator_count {
            let validator_count = validator_count.min(candidates.len());
            elected_candidates = Vec::with_capacity(validator_count);
            assigned = Vec::with_capacity(validator_count);
            for _round in 0..validator_count {
                for c in &mut candidates {
                    if !c.elected {
                        c.score = Fraction::from_xth(c.approval_stake);
                    }
                }
                for n in &nominators {
                    for e in &n.edges {
                        let c = &mut candidates[e.candidate_index];
                        if !c.elected && !c.approval_stake.is_zero() {
                            let temp =
                                n.budget.saturating_mul(SCALE_FACTOR) /
                                    c.approval_stake *
                                    (*n.load / SCALE_FACTOR);
                            c.score =
                                Fraction::from_parts((*c.score).saturating_add(temp));
                        }
                    }
                }
                if let Some(winner) =
                       candidates.iter_mut().filter(|c|
                                                        !c.elected).min_by_key(|c|
                                                                                   *c.score)
                       {
                    winner.elected = true;
                    for n in &mut nominators {
                        for e in &mut n.edges {
                            if e.who == winner.who {
                                e.load =
                                    Fraction::from_parts(*winner.score -
                                                             *n.load);
                                n.load = winner.score;
                            }
                        }
                    }
                    elected_candidates.push(winner.who.clone());
                } else { break  }
            }
            for n in &mut nominators {
                let mut assignment = (n.who.clone(), <[_]>::into_vec(box []));
                for e in &mut n.edges {
                    if let Some(c) =
                           elected_candidates.iter().find(|c| **c == e.who) {
                        if *c != n.who {
                            let ratio =
                                {
                                    if *n.load == *e.load {
                                        ACCURACY
                                    } else {
                                        if let Some(r) =
                                               ACCURACY.checked_mul(*e.load) {
                                            r / n.load.max(1)
                                        } else {
                                            *e.load /
                                                (n.load.max(1) / ACCURACY)
                                        }
                                    }
                                };
                            e.ratio = ratio;
                            assignment.1.push((e.who.clone(), ratio));
                        }
                    }
                }
                if assignment.1.len() > 0 {
                    let vote_count = assignment.1.len() as ExtendedBalance;
                    let l = assignment.1.len();
                    let sum =
                        assignment.1.iter().map(|a|
                                                    a.1).sum::<ExtendedBalance>();
                    let diff = ACCURACY.checked_sub(sum).unwrap_or(0);
                    let diff_per_vote = diff / vote_count;
                    if diff_per_vote > 0 {
                        for i in 0..l {
                            assignment.1[i % l].1 =
                                assignment.1[i %
                                                 l].1.saturating_add(diff_per_vote);
                        }
                    }
                    let remainder = diff - diff_per_vote * vote_count;
                    for i in 0..remainder as usize {
                        assignment.1[i % l].1 =
                            assignment.1[i % l].1.saturating_add(1);
                    }
                    assigned.push(assignment);
                }
            }
        } else { return None }
        Some((elected_candidates, assigned))
    }
    /// Performs equalize post-processing to the output of the election algorithm
    /// This function mutates the input parameters, most noticeably it updates the exposure of
    /// the elected candidates.
    ///
    /// No value is returned from the function and the `expo_map` parameter is updated.
    pub fn equalize<T: Trait +
                    'static>(assignments:
                                 &mut Vec<(T::AccountId, BalanceOf<T>,
                                           Vec<Assignment<T>>)>,
                             expo_map: &mut ExpoMap<T>,
                             tolerance: ExtendedBalance, iterations: usize) {
        for _i in 0..iterations {
            let mut max_diff = 0;
            assignments.iter_mut().for_each(|(n, budget, assignment)|
                                                {
                                                    let diff =
                                                        do_equalize::<T>(&n,
                                                                         *budget,
                                                                         assignment,
                                                                         expo_map,
                                                                         tolerance);
                                                    if diff > max_diff {
                                                        max_diff = diff;
                                                    }
                                                });
            if max_diff < tolerance { break ; }
        }
    }
    fn do_equalize<T: Trait +
                   'static>(nominator: &T::AccountId,
                            budget_balance: BalanceOf<T>,
                            elected_edges_balance: &mut Vec<Assignment<T>>,
                            expo_map: &mut ExpoMap<T>,
                            tolerance: ExtendedBalance) -> ExtendedBalance {
        let to_votes =
            |b: BalanceOf<T>|
                <T::CurrencyToVote as Convert<BalanceOf<T>, u64>>::convert(b)
                    as ExtendedBalance;
        let to_balance =
            |v: ExtendedBalance|
                <T::CurrencyToVote as
                    Convert<ExtendedBalance, BalanceOf<T>>>::convert(v);
        let budget = to_votes(budget_balance);
        let mut elected_edges =
            elected_edges_balance.into_iter().map(|e|
                                                      (e.0.clone(), e.1,
                                                       to_votes(e.2))).collect::<Vec<(T::AccountId,
                                                                                      ExtendedBalance,
                                                                                      ExtendedBalance)>>();
        let stake_used =
            elected_edges.iter().fold(0 as ExtendedBalance,
                                      |s, e| s.saturating_add(e.2));
        let backed_stakes_iter =
            elected_edges.iter().filter_map(|e|
                                                expo_map.get(&e.0)).map(|e|
                                                                            to_votes(e.total));
        let backing_backed_stake =
            elected_edges.iter().filter(|e|
                                            e.2 >
                                                0).filter_map(|e|
                                                                  expo_map.get(&e.0)).map(|e|
                                                                                              to_votes(e.total)).collect::<Vec<ExtendedBalance>>();
        let mut difference;
        if backing_backed_stake.len() > 0 {
            let max_stake =
                backing_backed_stake.iter().max().expect("vector with positive length will have a max; qed");
            let min_stake =
                backed_stakes_iter.min().expect("iterator with positive length will have a min; qed");
            difference = max_stake.saturating_sub(min_stake);
            difference =
                difference.saturating_add(budget.saturating_sub(stake_used));
            if difference < tolerance { return difference; }
        } else { difference = budget; }
        elected_edges.iter_mut().for_each(|e|
                                              {
                                                  if let Some(expo) =
                                                         expo_map.get_mut(&e.0)
                                                         {
                                                      expo.total =
                                                          expo.total.saturating_sub(to_balance(e.2));
                                                  }
                                                  e.2 = 0;
                                              });
        elected_edges.sort_unstable_by_key(|e| e.2);
        let mut cumulative_stake: ExtendedBalance = 0;
        let mut last_index = elected_edges.len() - 1;
        elected_edges.iter_mut().enumerate().for_each(|(idx, e)|
                                                          {
                                                              if let Some(expo)
                                                                     =
                                                                     expo_map.get_mut(&e.0)
                                                                     {
                                                                  let stake:
                                                                          ExtendedBalance =
                                                                      to_votes(expo.total);
                                                                  let stake_mul =
                                                                      stake.saturating_mul(idx
                                                                                               as
                                                                                               ExtendedBalance);
                                                                  let stake_sub =
                                                                      stake_mul.saturating_sub(cumulative_stake);
                                                                  if stake_sub
                                                                         >
                                                                         budget
                                                                     {
                                                                      last_index
                                                                          =
                                                                          idx.checked_sub(1).unwrap_or(0);
                                                                      return
                                                                  }
                                                                  cumulative_stake
                                                                      =
                                                                      cumulative_stake.saturating_add(stake);
                                                              }
                                                          });
        let last_stake = elected_edges[last_index].2;
        let split_ways = last_index + 1;
        let excess =
            budget.saturating_add(cumulative_stake).saturating_sub(last_stake.saturating_mul(split_ways
                                                                                                 as
                                                                                                 ExtendedBalance));
        elected_edges.iter_mut().take(split_ways).for_each(|e|
                                                               {
                                                                   if let Some(expo)
                                                                          =
                                                                          expo_map.get_mut(&e.0)
                                                                          {
                                                                       e.2 =
                                                                           (excess
                                                                                /
                                                                                split_ways
                                                                                    as
                                                                                    ExtendedBalance).saturating_add(last_stake).saturating_sub(to_votes(expo.total));
                                                                       expo.total
                                                                           =
                                                                           expo.total.saturating_add(to_balance(e.2));
                                                                       if let Some(i_expo)
                                                                              =
                                                                              expo.others.iter_mut().find(|i|
                                                                                                              i.who
                                                                                                                  ==
                                                                                                                  nominator.clone())
                                                                              {
                                                                           i_expo.value
                                                                               =
                                                                               to_balance(e.2);
                                                                       }
                                                                   }
                                                               });
        elected_edges.iter().enumerate().for_each(|(idx, e)|
                                                      elected_edges_balance[idx].2
                                                          = to_balance(e.2));
        difference
    }
}
use phragmen::{elect, ACCURACY, ExtendedBalance};
const RECENT_OFFLINE_COUNT: usize = 32;
const DEFAULT_MINIMUM_VALIDATOR_COUNT: u32 = 4;
const MAX_NOMINATIONS: usize = 16;
const MAX_UNSTAKE_THRESHOLD: u32 = 10;
/// Indicates the initial status of the staker.
pub enum StakerStatus<AccountId> {

    /// Chilling.
    Idle,

    /// Declared desire in validating or already participating in it.
    Validator,

    /// Nominating for a group of other stakers.
    Nominator(Vec<AccountId>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::fmt::Debug> ::std::fmt::Debug for
 StakerStatus<AccountId> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&StakerStatus::Idle,) => {
                let mut debug_trait_builder = f.debug_tuple("Idle");
                debug_trait_builder.finish()
            }
            (&StakerStatus::Validator,) => {
                let mut debug_trait_builder = f.debug_tuple("Validator");
                debug_trait_builder.finish()
            }
            (&StakerStatus::Nominator(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Nominator");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_StakerStatus: () =
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
        impl <AccountId> _serde::Serialize for StakerStatus<AccountId> where
         AccountId: _serde::Serialize {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    StakerStatus::Idle =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "StakerStatus",
                                                               0u32, "Idle"),
                    StakerStatus::Validator =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "StakerStatus",
                                                               1u32,
                                                               "Validator"),
                    StakerStatus::Nominator(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "StakerStatus",
                                                                  2u32,
                                                                  "Nominator",
                                                                  __field0),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_StakerStatus: () =
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
        impl <'de, AccountId> _serde::Deserialize<'de> for
         StakerStatus<AccountId> where AccountId: _serde::Deserialize<'de> {
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
                                 __formatter: &mut _serde::export::Formatter)
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
                            "Idle" => _serde::export::Ok(__Field::__field0),
                            "Validator" =>
                            _serde::export::Ok(__Field::__field1),
                            "Nominator" =>
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
                            b"Idle" => _serde::export::Ok(__Field::__field0),
                            b"Validator" =>
                            _serde::export::Ok(__Field::__field1),
                            b"Nominator" =>
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
                struct __Visitor<'de, AccountId> where
                       AccountId: _serde::Deserialize<'de> {
                    marker: _serde::export::PhantomData<StakerStatus<AccountId>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, AccountId> _serde::de::Visitor<'de> for
                 __Visitor<'de, AccountId> where
                 AccountId: _serde::Deserialize<'de> {
                    type
                    Value
                    =
                    StakerStatus<AccountId>;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "enum StakerStatus")
                    }
                    fn visit_enum<__A>(self, __data: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::EnumAccess<'de> {
                        match match _serde::de::EnumAccess::variant(__data) {
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
                                _serde::export::Ok(StakerStatus::Idle)
                            }
                            (__Field::__field1, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant)
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(StakerStatus::Validator)
                            }
                            (__Field::__field2, __variant) =>
                            _serde::export::Result::map(_serde::de::VariantAccess::newtype_variant::<Vec<AccountId>>(__variant),
                                                        StakerStatus::Nominator),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] =
                    &["Idle", "Validator", "Nominator"];
                _serde::Deserializer::deserialize_enum(__deserializer,
                                                       "StakerStatus",
                                                       VARIANTS,
                                                       __Visitor{marker:
                                                                     _serde::export::PhantomData::<StakerStatus<AccountId>>,
                                                                 lifetime:
                                                                     _serde::export::PhantomData,})
            }
        }
    };
/// A destination account for payment.
#[structural_match]
#[rustc_copy_clone_marker]
pub enum RewardDestination {

    /// Pay into the stash account, increasing the amount at stake accordingly.
    Staked,

    /// Pay into the stash account, not increasing the amount at stake.
    Stash,

    /// Pay into the controller account.
    Controller,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for RewardDestination {
    #[inline]
    fn eq(&self, other: &RewardDestination) -> bool {
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
impl ::std::cmp::Eq for RewardDestination {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for RewardDestination { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for RewardDestination {
    #[inline]
    fn clone(&self) -> RewardDestination { { *self } }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RewardDestination: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for RewardDestination {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RewardDestination::Staked => {
                        dest.push_byte(0usize as u8);
                    }
                    RewardDestination::Stash => {
                        dest.push_byte(1usize as u8);
                    }
                    RewardDestination::Controller => {
                        dest.push_byte(2usize as u8);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_RewardDestination: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for RewardDestination {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RewardDestination::Staked)
                    }
                    x if x == 1usize as u8 => {
                        Some(RewardDestination::Stash)
                    }
                    x if x == 2usize as u8 => {
                        Some(RewardDestination::Controller)
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for RewardDestination {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RewardDestination::Staked,) => {
                let mut debug_trait_builder = f.debug_tuple("Staked");
                debug_trait_builder.finish()
            }
            (&RewardDestination::Stash,) => {
                let mut debug_trait_builder = f.debug_tuple("Stash");
                debug_trait_builder.finish()
            }
            (&RewardDestination::Controller,) => {
                let mut debug_trait_builder = f.debug_tuple("Controller");
                debug_trait_builder.finish()
            }
        }
    }
}
impl Default for RewardDestination {
    fn default() -> Self { RewardDestination::Staked }
}
/// Preference of what happens on a slash event.
#[structural_match]
pub struct ValidatorPrefs<Balance: HasCompact> {
    /// Validator should ensure this many more slashes than is necessary before being unstaked.
    #[codec(compact)]
    pub unstake_threshold: u32,
    /// Reward that validator takes up-front; only the rest is split between themselves and nominators.
    #[codec(compact)]
    pub validator_payment: Balance,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::PartialEq + HasCompact> ::std::cmp::PartialEq for
 ValidatorPrefs<Balance> {
    #[inline]
    fn eq(&self, other: &ValidatorPrefs<Balance>) -> bool {
        match *other {
            ValidatorPrefs {
            unstake_threshold: ref __self_1_0,
            validator_payment: ref __self_1_1 } =>
            match *self {
                ValidatorPrefs {
                unstake_threshold: ref __self_0_0,
                validator_payment: ref __self_0_1 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &ValidatorPrefs<Balance>) -> bool {
        match *other {
            ValidatorPrefs {
            unstake_threshold: ref __self_1_0,
            validator_payment: ref __self_1_1 } =>
            match *self {
                ValidatorPrefs {
                unstake_threshold: ref __self_0_0,
                validator_payment: ref __self_0_1 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::Eq + HasCompact> ::std::cmp::Eq for
 ValidatorPrefs<Balance> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::clone::Clone + HasCompact> ::std::clone::Clone for
 ValidatorPrefs<Balance> {
    #[inline]
    fn clone(&self) -> ValidatorPrefs<Balance> {
        match *self {
            ValidatorPrefs {
            unstake_threshold: ref __self_0_0,
            validator_payment: ref __self_0_1 } =>
            ValidatorPrefs{unstake_threshold:
                               ::std::clone::Clone::clone(&(*__self_0_0)),
                           validator_payment:
                               ::std::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_ValidatorPrefs: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance: HasCompact> _parity_codec::Encode for
         ValidatorPrefs<Balance> where Balance: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                {
                    dest.push(&<<u32 as _parity_codec::HasCompact>::Type as
                                   _parity_codec::EncodeAsRef<'_,
                                                              u32>>::from(&self.unstake_threshold));
                }
                {
                    dest.push(&<<Balance as _parity_codec::HasCompact>::Type
                                   as
                                   _parity_codec::EncodeAsRef<'_,
                                                              Balance>>::from(&self.validator_payment));
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_ValidatorPrefs: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance: HasCompact> _parity_codec::Decode for
         ValidatorPrefs<Balance> where Balance: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(ValidatorPrefs{unstake_threshold:
                                        <<u32 as
                                         _parity_codec::HasCompact>::Type as
                                            _parity_codec::Decode>::decode(input)?.into(),
                                    validator_payment:
                                        <<Balance as
                                         _parity_codec::HasCompact>::Type as
                                            _parity_codec::Decode>::decode(input)?.into(),})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::fmt::Debug + HasCompact> ::std::fmt::Debug for
 ValidatorPrefs<Balance> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ValidatorPrefs {
            unstake_threshold: ref __self_0_0,
            validator_payment: ref __self_0_1 } => {
                let mut debug_trait_builder =
                    f.debug_struct("ValidatorPrefs");
                let _ =
                    debug_trait_builder.field("unstake_threshold",
                                              &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("validator_payment",
                                              &&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <B: Default + HasCompact + Copy> Default for ValidatorPrefs<B> {
    fn default() -> Self {
        ValidatorPrefs{unstake_threshold: 3,
                       validator_payment: Default::default(),}
    }
}
/// Just a Balance/BlockNumber tuple to encode when a chunk of funds will be unlocked.
#[structural_match]
pub struct UnlockChunk<Balance: HasCompact, BlockNumber: HasCompact> {
    /// Amount of funds to be unlocked.
    #[codec(compact)]
    value: Balance,
    /// Era number at which point it'll be unlocked.
    #[codec(compact)]
    era: BlockNumber,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::PartialEq + HasCompact,
      BlockNumber: ::std::cmp::PartialEq + HasCompact> ::std::cmp::PartialEq
 for UnlockChunk<Balance, BlockNumber> {
    #[inline]
    fn eq(&self, other: &UnlockChunk<Balance, BlockNumber>) -> bool {
        match *other {
            UnlockChunk { value: ref __self_1_0, era: ref __self_1_1 } =>
            match *self {
                UnlockChunk { value: ref __self_0_0, era: ref __self_0_1 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &UnlockChunk<Balance, BlockNumber>) -> bool {
        match *other {
            UnlockChunk { value: ref __self_1_0, era: ref __self_1_1 } =>
            match *self {
                UnlockChunk { value: ref __self_0_0, era: ref __self_0_1 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::cmp::Eq + HasCompact, BlockNumber: ::std::cmp::Eq +
      HasCompact> ::std::cmp::Eq for UnlockChunk<Balance, BlockNumber> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<BlockNumber>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::clone::Clone + HasCompact,
      BlockNumber: ::std::clone::Clone + HasCompact> ::std::clone::Clone for
 UnlockChunk<Balance, BlockNumber> {
    #[inline]
    fn clone(&self) -> UnlockChunk<Balance, BlockNumber> {
        match *self {
            UnlockChunk { value: ref __self_0_0, era: ref __self_0_1 } =>
            UnlockChunk{value: ::std::clone::Clone::clone(&(*__self_0_0)),
                        era: ::std::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_UnlockChunk: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance: HasCompact, BlockNumber: HasCompact>
         _parity_codec::Encode for UnlockChunk<Balance, BlockNumber> where
         Balance: _parity_codec::HasCompact,
         BlockNumber: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                {
                    dest.push(&<<Balance as _parity_codec::HasCompact>::Type
                                   as
                                   _parity_codec::EncodeAsRef<'_,
                                                              Balance>>::from(&self.value));
                }
                {
                    dest.push(&<<BlockNumber as
                                _parity_codec::HasCompact>::Type as
                                   _parity_codec::EncodeAsRef<'_,
                                                              BlockNumber>>::from(&self.era));
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_UnlockChunk: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Balance: HasCompact, BlockNumber: HasCompact>
         _parity_codec::Decode for UnlockChunk<Balance, BlockNumber> where
         Balance: _parity_codec::HasCompact,
         BlockNumber: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(UnlockChunk{value:
                                     <<Balance as
                                      _parity_codec::HasCompact>::Type as
                                         _parity_codec::Decode>::decode(input)?.into(),
                                 era:
                                     <<BlockNumber as
                                      _parity_codec::HasCompact>::Type as
                                         _parity_codec::Decode>::decode(input)?.into(),})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::fmt::Debug + HasCompact,
      BlockNumber: ::std::fmt::Debug + HasCompact> ::std::fmt::Debug for
 UnlockChunk<Balance, BlockNumber> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            UnlockChunk { value: ref __self_0_0, era: ref __self_0_1 } => {
                let mut debug_trait_builder = f.debug_struct("UnlockChunk");
                let _ = debug_trait_builder.field("value", &&(*__self_0_0));
                let _ = debug_trait_builder.field("era", &&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
/// The ledger of a (bonded) stash.
#[structural_match]
pub struct StakingLedger<AccountId, Balance: HasCompact,
                         BlockNumber: HasCompact> {
    /// The stash account whose balance is actually locked and at stake.
    pub stash: AccountId,
    /// The total amount of the stash's balance that we are currently accounting for.
    /// It's just `active` plus all the `unlocking` balances.
    #[codec(compact)]
    pub total: Balance,
    /// The total amount of the stash's balance that will be at stake in any forthcoming
    /// rounds.
    #[codec(compact)]
    pub active: Balance,
    /// Any balance that is becoming free, which may eventually be transferred out
    /// of the stash (assuming it doesn't get slashed first).
    pub unlocking: Vec<UnlockChunk<Balance, BlockNumber>>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialEq, Balance: ::std::cmp::PartialEq +
      HasCompact, BlockNumber: ::std::cmp::PartialEq + HasCompact>
 ::std::cmp::PartialEq for StakingLedger<AccountId, Balance, BlockNumber> {
    #[inline]
    fn eq(&self, other: &StakingLedger<AccountId, Balance, BlockNumber>)
     -> bool {
        match *other {
            StakingLedger {
            stash: ref __self_1_0,
            total: ref __self_1_1,
            active: ref __self_1_2,
            unlocking: ref __self_1_3 } =>
            match *self {
                StakingLedger {
                stash: ref __self_0_0,
                total: ref __self_0_1,
                active: ref __self_0_2,
                unlocking: ref __self_0_3 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &StakingLedger<AccountId, Balance, BlockNumber>)
     -> bool {
        match *other {
            StakingLedger {
            stash: ref __self_1_0,
            total: ref __self_1_1,
            active: ref __self_1_2,
            unlocking: ref __self_1_3 } =>
            match *self {
                StakingLedger {
                stash: ref __self_0_0,
                total: ref __self_0_1,
                active: ref __self_0_2,
                unlocking: ref __self_0_3 } =>
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
impl <AccountId: ::std::cmp::Eq, Balance: ::std::cmp::Eq + HasCompact,
      BlockNumber: ::std::cmp::Eq + HasCompact> ::std::cmp::Eq for
 StakingLedger<AccountId, Balance, BlockNumber> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _:
                    ::std::cmp::AssertParamIsEq<Vec<UnlockChunk<Balance,
                                                                BlockNumber>>>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::clone::Clone, Balance: ::std::clone::Clone +
      HasCompact, BlockNumber: ::std::clone::Clone + HasCompact>
 ::std::clone::Clone for StakingLedger<AccountId, Balance, BlockNumber> {
    #[inline]
    fn clone(&self) -> StakingLedger<AccountId, Balance, BlockNumber> {
        match *self {
            StakingLedger {
            stash: ref __self_0_0,
            total: ref __self_0_1,
            active: ref __self_0_2,
            unlocking: ref __self_0_3 } =>
            StakingLedger{stash: ::std::clone::Clone::clone(&(*__self_0_0)),
                          total: ::std::clone::Clone::clone(&(*__self_0_1)),
                          active: ::std::clone::Clone::clone(&(*__self_0_2)),
                          unlocking:
                              ::std::clone::Clone::clone(&(*__self_0_3)),},
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_StakingLedger: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, Balance: HasCompact, BlockNumber: HasCompact>
         _parity_codec::Encode for
         StakingLedger<AccountId, Balance, BlockNumber> where
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         Vec<UnlockChunk<Balance, BlockNumber>>: _parity_codec::Encode,
         Vec<UnlockChunk<Balance, BlockNumber>>: _parity_codec::Encode,
         Balance: _parity_codec::HasCompact,
         Balance: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.stash);
                {
                    dest.push(&<<Balance as _parity_codec::HasCompact>::Type
                                   as
                                   _parity_codec::EncodeAsRef<'_,
                                                              Balance>>::from(&self.total));
                }
                {
                    dest.push(&<<Balance as _parity_codec::HasCompact>::Type
                                   as
                                   _parity_codec::EncodeAsRef<'_,
                                                              Balance>>::from(&self.active));
                }
                dest.push(&self.unlocking);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_StakingLedger: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, Balance: HasCompact, BlockNumber: HasCompact>
         _parity_codec::Decode for
         StakingLedger<AccountId, Balance, BlockNumber> where
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         Vec<UnlockChunk<Balance, BlockNumber>>: _parity_codec::Decode,
         Vec<UnlockChunk<Balance, BlockNumber>>: _parity_codec::Decode,
         Balance: _parity_codec::HasCompact,
         Balance: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(StakingLedger{stash:
                                       _parity_codec::Decode::decode(input)?,
                                   total:
                                       <<Balance as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::Decode>::decode(input)?.into(),
                                   active:
                                       <<Balance as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::Decode>::decode(input)?.into(),
                                   unlocking:
                                       _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::fmt::Debug, Balance: ::std::fmt::Debug + HasCompact,
      BlockNumber: ::std::fmt::Debug + HasCompact> ::std::fmt::Debug for
 StakingLedger<AccountId, Balance, BlockNumber> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            StakingLedger {
            stash: ref __self_0_0,
            total: ref __self_0_1,
            active: ref __self_0_2,
            unlocking: ref __self_0_3 } => {
                let mut debug_trait_builder = f.debug_struct("StakingLedger");
                let _ = debug_trait_builder.field("stash", &&(*__self_0_0));
                let _ = debug_trait_builder.field("total", &&(*__self_0_1));
                let _ = debug_trait_builder.field("active", &&(*__self_0_2));
                let _ =
                    debug_trait_builder.field("unlocking", &&(*__self_0_3));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <AccountId, Balance: HasCompact + Copy + Saturating,
      BlockNumber: HasCompact + PartialOrd>
 StakingLedger<AccountId, Balance, BlockNumber> {
    /// Remove entries from `unlocking` that are sufficiently old and reduce the
    /// total by the sum of their balances.
    fn consolidate_unlocked(self, current_era: BlockNumber) -> Self {
        let mut total = self.total;
        let unlocking =
            self.unlocking.into_iter().filter(|chunk|
                                                  if chunk.era > current_era {
                                                      true
                                                  } else {
                                                      total =
                                                          total.saturating_sub(chunk.value);
                                                      false
                                                  }).collect();
        Self{total, active: self.active, stash: self.stash, unlocking,}
    }
}
/// The amount of exposure (to slashing) than an individual nominator has.
#[structural_match]
pub struct IndividualExposure<AccountId, Balance: HasCompact> {
    /// The stash account of the nominator in question.
    who: AccountId,
    /// Amount of funds exposed.
    #[codec(compact)]
    value: Balance,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialEq, Balance: ::std::cmp::PartialEq +
      HasCompact> ::std::cmp::PartialEq for
 IndividualExposure<AccountId, Balance> {
    #[inline]
    fn eq(&self, other: &IndividualExposure<AccountId, Balance>) -> bool {
        match *other {
            IndividualExposure { who: ref __self_1_0, value: ref __self_1_1 }
            =>
            match *self {
                IndividualExposure {
                who: ref __self_0_0, value: ref __self_0_1 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &IndividualExposure<AccountId, Balance>) -> bool {
        match *other {
            IndividualExposure { who: ref __self_1_0, value: ref __self_1_1 }
            =>
            match *self {
                IndividualExposure {
                who: ref __self_0_0, value: ref __self_0_1 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::Eq, Balance: ::std::cmp::Eq + HasCompact>
 ::std::cmp::Eq for IndividualExposure<AccountId, Balance> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialOrd, Balance: ::std::cmp::PartialOrd +
      HasCompact> ::std::cmp::PartialOrd for
 IndividualExposure<AccountId, Balance> {
    #[inline]
    fn partial_cmp(&self, other: &IndividualExposure<AccountId, Balance>)
     -> ::std::option::Option<::std::cmp::Ordering> {
        match *other {
            IndividualExposure { who: ref __self_1_0, value: ref __self_1_1 }
            =>
            match *self {
                IndividualExposure {
                who: ref __self_0_0, value: ref __self_0_1 } =>
                match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                          &(*__self_1_0)) {
                    ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                    =>
                    match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                              &(*__self_1_1))
                        {
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                        =>
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &IndividualExposure<AccountId, Balance>) -> bool {
        match *other {
            IndividualExposure { who: ref __self_1_0, value: ref __self_1_1 }
            =>
            match *self {
                IndividualExposure {
                who: ref __self_0_0, value: ref __self_0_1 } =>
                ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                     &(*__self_1_0)),
                                                                                 ::std::cmp::Ordering::Equal),
                                                ||
                                                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                         &(*__self_1_1)),
                                                                                     ::std::cmp::Ordering::Greater))
                    == ::std::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &IndividualExposure<AccountId, Balance>) -> bool {
        match *other {
            IndividualExposure { who: ref __self_1_0, value: ref __self_1_1 }
            =>
            match *self {
                IndividualExposure {
                who: ref __self_0_0, value: ref __self_0_1 } =>
                ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                     &(*__self_1_0)),
                                                                                 ::std::cmp::Ordering::Equal),
                                                ||
                                                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                         &(*__self_1_1)),
                                                                                     ::std::cmp::Ordering::Greater))
                    != ::std::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &IndividualExposure<AccountId, Balance>) -> bool {
        match *other {
            IndividualExposure { who: ref __self_1_0, value: ref __self_1_1 }
            =>
            match *self {
                IndividualExposure {
                who: ref __self_0_0, value: ref __self_0_1 } =>
                ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                     &(*__self_1_0)),
                                                                                 ::std::cmp::Ordering::Equal),
                                                ||
                                                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                         &(*__self_1_1)),
                                                                                     ::std::cmp::Ordering::Less))
                    == ::std::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &IndividualExposure<AccountId, Balance>) -> bool {
        match *other {
            IndividualExposure { who: ref __self_1_0, value: ref __self_1_1 }
            =>
            match *self {
                IndividualExposure {
                who: ref __self_0_0, value: ref __self_0_1 } =>
                ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                     &(*__self_1_0)),
                                                                                 ::std::cmp::Ordering::Equal),
                                                ||
                                                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                         &(*__self_1_1)),
                                                                                     ::std::cmp::Ordering::Less))
                    != ::std::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::Ord, Balance: ::std::cmp::Ord + HasCompact>
 ::std::cmp::Ord for IndividualExposure<AccountId, Balance> {
    #[inline]
    fn cmp(&self, other: &IndividualExposure<AccountId, Balance>)
     -> ::std::cmp::Ordering {
        match *other {
            IndividualExposure { who: ref __self_1_0, value: ref __self_1_1 }
            =>
            match *self {
                IndividualExposure {
                who: ref __self_0_0, value: ref __self_0_1 } =>
                match ::std::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::std::cmp::Ordering::Equal =>
                    match ::std::cmp::Ord::cmp(&(*__self_0_1), &(*__self_1_1))
                        {
                        ::std::cmp::Ordering::Equal =>
                        ::std::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::clone::Clone, Balance: ::std::clone::Clone +
      HasCompact> ::std::clone::Clone for
 IndividualExposure<AccountId, Balance> {
    #[inline]
    fn clone(&self) -> IndividualExposure<AccountId, Balance> {
        match *self {
            IndividualExposure { who: ref __self_0_0, value: ref __self_0_1 }
            =>
            IndividualExposure{who:
                                   ::std::clone::Clone::clone(&(*__self_0_0)),
                               value:
                                   ::std::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_IndividualExposure: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, Balance: HasCompact> _parity_codec::Encode for
         IndividualExposure<AccountId, Balance> where
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         Balance: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.who);
                {
                    dest.push(&<<Balance as _parity_codec::HasCompact>::Type
                                   as
                                   _parity_codec::EncodeAsRef<'_,
                                                              Balance>>::from(&self.value));
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_IndividualExposure: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, Balance: HasCompact> _parity_codec::Decode for
         IndividualExposure<AccountId, Balance> where
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         Balance: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(IndividualExposure{who:
                                            _parity_codec::Decode::decode(input)?,
                                        value:
                                            <<Balance as
                                             _parity_codec::HasCompact>::Type
                                                as
                                                _parity_codec::Decode>::decode(input)?.into(),})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::fmt::Debug, Balance: ::std::fmt::Debug + HasCompact>
 ::std::fmt::Debug for IndividualExposure<AccountId, Balance> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            IndividualExposure { who: ref __self_0_0, value: ref __self_0_1 }
            => {
                let mut debug_trait_builder =
                    f.debug_struct("IndividualExposure");
                let _ = debug_trait_builder.field("who", &&(*__self_0_0));
                let _ = debug_trait_builder.field("value", &&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
/// A snapshot of the stake backing a single validator in the system.
#[structural_match]
pub struct Exposure<AccountId, Balance: HasCompact> {
    /// The total balance backing this validator.
    #[codec(compact)]
    pub total: Balance,
    /// The validator's own stash that is exposed.
    #[codec(compact)]
    pub own: Balance,
    /// The portions of nominators stashes that are exposed.
    pub others: Vec<IndividualExposure<AccountId, Balance>>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialEq, Balance: ::std::cmp::PartialEq +
      HasCompact> ::std::cmp::PartialEq for Exposure<AccountId, Balance> {
    #[inline]
    fn eq(&self, other: &Exposure<AccountId, Balance>) -> bool {
        match *other {
            Exposure {
            total: ref __self_1_0, own: ref __self_1_1, others: ref __self_1_2
            } =>
            match *self {
                Exposure {
                total: ref __self_0_0,
                own: ref __self_0_1,
                others: ref __self_0_2 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Exposure<AccountId, Balance>) -> bool {
        match *other {
            Exposure {
            total: ref __self_1_0, own: ref __self_1_1, others: ref __self_1_2
            } =>
            match *self {
                Exposure {
                total: ref __self_0_0,
                own: ref __self_0_1,
                others: ref __self_0_2 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::Eq, Balance: ::std::cmp::Eq + HasCompact>
 ::std::cmp::Eq for Exposure<AccountId, Balance> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _:
                    ::std::cmp::AssertParamIsEq<Vec<IndividualExposure<AccountId,
                                                                       Balance>>>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::PartialOrd, Balance: ::std::cmp::PartialOrd +
      HasCompact> ::std::cmp::PartialOrd for Exposure<AccountId, Balance> {
    #[inline]
    fn partial_cmp(&self, other: &Exposure<AccountId, Balance>)
     -> ::std::option::Option<::std::cmp::Ordering> {
        match *other {
            Exposure {
            total: ref __self_1_0, own: ref __self_1_1, others: ref __self_1_2
            } =>
            match *self {
                Exposure {
                total: ref __self_0_0,
                own: ref __self_0_1,
                others: ref __self_0_2 } =>
                match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                          &(*__self_1_0)) {
                    ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                    =>
                    match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                              &(*__self_1_1))
                        {
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                        =>
                        match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                  &(*__self_1_2))
                            {
                            ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                            =>
                            ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &Exposure<AccountId, Balance>) -> bool {
        match *other {
            Exposure {
            total: ref __self_1_0, own: ref __self_1_1, others: ref __self_1_2
            } =>
            match *self {
                Exposure {
                total: ref __self_0_0,
                own: ref __self_0_1,
                others: ref __self_0_2 } =>
                ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                     &(*__self_1_0)),
                                                                                 ::std::cmp::Ordering::Equal),
                                                ||
                                                    ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                         &(*__self_1_1)),
                                                                                                                     ::std::cmp::Ordering::Equal),
                                                                                    ||
                                                                                        ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                             &(*__self_1_2)),
                                                                                                                         ::std::cmp::Ordering::Greater)))
                    == ::std::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &Exposure<AccountId, Balance>) -> bool {
        match *other {
            Exposure {
            total: ref __self_1_0, own: ref __self_1_1, others: ref __self_1_2
            } =>
            match *self {
                Exposure {
                total: ref __self_0_0,
                own: ref __self_0_1,
                others: ref __self_0_2 } =>
                ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                     &(*__self_1_0)),
                                                                                 ::std::cmp::Ordering::Equal),
                                                ||
                                                    ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                         &(*__self_1_1)),
                                                                                                                     ::std::cmp::Ordering::Equal),
                                                                                    ||
                                                                                        ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                             &(*__self_1_2)),
                                                                                                                         ::std::cmp::Ordering::Greater)))
                    != ::std::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &Exposure<AccountId, Balance>) -> bool {
        match *other {
            Exposure {
            total: ref __self_1_0, own: ref __self_1_1, others: ref __self_1_2
            } =>
            match *self {
                Exposure {
                total: ref __self_0_0,
                own: ref __self_0_1,
                others: ref __self_0_2 } =>
                ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                     &(*__self_1_0)),
                                                                                 ::std::cmp::Ordering::Equal),
                                                ||
                                                    ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                         &(*__self_1_1)),
                                                                                                                     ::std::cmp::Ordering::Equal),
                                                                                    ||
                                                                                        ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                             &(*__self_1_2)),
                                                                                                                         ::std::cmp::Ordering::Less)))
                    == ::std::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &Exposure<AccountId, Balance>) -> bool {
        match *other {
            Exposure {
            total: ref __self_1_0, own: ref __self_1_1, others: ref __self_1_2
            } =>
            match *self {
                Exposure {
                total: ref __self_0_0,
                own: ref __self_0_1,
                others: ref __self_0_2 } =>
                ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                     &(*__self_1_0)),
                                                                                 ::std::cmp::Ordering::Equal),
                                                ||
                                                    ::std::cmp::Ordering::then_with(::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                                         &(*__self_1_1)),
                                                                                                                     ::std::cmp::Ordering::Equal),
                                                                                    ||
                                                                                        ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_2),
                                                                                                                                                             &(*__self_1_2)),
                                                                                                                         ::std::cmp::Ordering::Less)))
                    != ::std::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::cmp::Ord, Balance: ::std::cmp::Ord + HasCompact>
 ::std::cmp::Ord for Exposure<AccountId, Balance> {
    #[inline]
    fn cmp(&self, other: &Exposure<AccountId, Balance>)
     -> ::std::cmp::Ordering {
        match *other {
            Exposure {
            total: ref __self_1_0, own: ref __self_1_1, others: ref __self_1_2
            } =>
            match *self {
                Exposure {
                total: ref __self_0_0,
                own: ref __self_0_1,
                others: ref __self_0_2 } =>
                match ::std::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::std::cmp::Ordering::Equal =>
                    match ::std::cmp::Ord::cmp(&(*__self_0_1), &(*__self_1_1))
                        {
                        ::std::cmp::Ordering::Equal =>
                        match ::std::cmp::Ord::cmp(&(*__self_0_2),
                                                   &(*__self_1_2)) {
                            ::std::cmp::Ordering::Equal =>
                            ::std::cmp::Ordering::Equal,
                            cmp => cmp,
                        },
                        cmp => cmp,
                    },
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::clone::Clone, Balance: ::std::clone::Clone +
      HasCompact> ::std::clone::Clone for Exposure<AccountId, Balance> {
    #[inline]
    fn clone(&self) -> Exposure<AccountId, Balance> {
        match *self {
            Exposure {
            total: ref __self_0_0, own: ref __self_0_1, others: ref __self_0_2
            } =>
            Exposure{total: ::std::clone::Clone::clone(&(*__self_0_0)),
                     own: ::std::clone::Clone::clone(&(*__self_0_1)),
                     others: ::std::clone::Clone::clone(&(*__self_0_2)),},
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Exposure: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, Balance: HasCompact> _parity_codec::Encode for
         Exposure<AccountId, Balance> where
         Vec<IndividualExposure<AccountId, Balance>>: _parity_codec::Encode,
         Vec<IndividualExposure<AccountId, Balance>>: _parity_codec::Encode,
         Balance: _parity_codec::HasCompact,
         Balance: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                {
                    dest.push(&<<Balance as _parity_codec::HasCompact>::Type
                                   as
                                   _parity_codec::EncodeAsRef<'_,
                                                              Balance>>::from(&self.total));
                }
                {
                    dest.push(&<<Balance as _parity_codec::HasCompact>::Type
                                   as
                                   _parity_codec::EncodeAsRef<'_,
                                                              Balance>>::from(&self.own));
                }
                dest.push(&self.others);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Exposure: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <AccountId, Balance: HasCompact> _parity_codec::Decode for
         Exposure<AccountId, Balance> where
         Vec<IndividualExposure<AccountId, Balance>>: _parity_codec::Decode,
         Vec<IndividualExposure<AccountId, Balance>>: _parity_codec::Decode,
         Balance: _parity_codec::HasCompact,
         Balance: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(Exposure{total:
                                  <<Balance as
                                   _parity_codec::HasCompact>::Type as
                                      _parity_codec::Decode>::decode(input)?.into(),
                              own:
                                  <<Balance as
                                   _parity_codec::HasCompact>::Type as
                                      _parity_codec::Decode>::decode(input)?.into(),
                              others: _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::default::Default, Balance: ::std::default::Default +
      HasCompact> ::std::default::Default for Exposure<AccountId, Balance> {
    #[inline]
    fn default() -> Exposure<AccountId, Balance> {
        Exposure{total: ::std::default::Default::default(),
                 own: ::std::default::Default::default(),
                 others: ::std::default::Default::default(),}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <AccountId: ::std::fmt::Debug, Balance: ::std::fmt::Debug + HasCompact>
 ::std::fmt::Debug for Exposure<AccountId, Balance> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Exposure {
            total: ref __self_0_0, own: ref __self_0_1, others: ref __self_0_2
            } => {
                let mut debug_trait_builder = f.debug_struct("Exposure");
                let _ = debug_trait_builder.field("total", &&(*__self_0_0));
                let _ = debug_trait_builder.field("own", &&(*__self_0_1));
                let _ = debug_trait_builder.field("others", &&(*__self_0_2));
                debug_trait_builder.finish()
            }
        }
    }
}
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
type RawAssignment<T> = (<T as system::Trait>::AccountId, ExtendedBalance);
type Assignment<T>
    =
    (<T as system::Trait>::AccountId, ExtendedBalance, BalanceOf<T>);
type ExpoMap<T>
    =
    BTreeMap<<T as system::Trait>::AccountId,
             Exposure<<T as system::Trait>::AccountId, BalanceOf<T>>>;
pub trait Trait: system::Trait + session::Trait {
    /// The staking balance.
    type
    Currency: LockableCurrency<Self::AccountId,
    Moment
    =
    Self::BlockNumber>;
    /// Convert a balance into a number used for election calculation.
    /// This must fit into a `u64` but is allowed to be sensibly lossy.
    /// TODO: #1377
    /// The backward convert should be removed as the new Phragmen API returns ratio.
    /// The post-processing needs it but will be moved to off-chain.
    type
    CurrencyToVote: Convert<BalanceOf<Self>, u64> +
    Convert<u128, BalanceOf<Self>>;
    /// Some tokens minted.
    type
    OnRewardMinted: OnDilution<BalanceOf<Self>>;
    /// The overarching event type.
    type
    Event: From<Event<Self>> +
    Into<<Self as system::Trait>::Event>;
    /// Handler for the unbalanced reduction when slashing a staker.
    type
    Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
    /// Handler for the unbalanced increment when rewarding a staker.
    type
    Reward: OnUnbalanced<PositiveImbalanceOf<Self>>;
}
const STAKING_ID: LockIdentifier = *b"staking ";
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = " The ideal number of staking participants."]
pub struct ValidatorCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
 for ValidatorCount<T> {
    type
    Query
    =
    u32;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking ValidatorCount".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                             Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
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
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&val,
                                                                                                                           storage);
        ret
    }
}
#[doc =
      " Minimum number of staking participants before emergency conditions are imposed."]
pub struct MinimumValidatorCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
 for MinimumValidatorCount<T> {
    type
    Query
    =
    u32;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking MinimumValidatorCount".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                             DEFAULT_MINIMUM_VALIDATOR_COUNT)
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                              DEFAULT_MINIMUM_VALIDATOR_COUNT)
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
#[doc = " The length of a staking era in sessions."]
pub struct SessionsPerEra<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for SessionsPerEra<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking SessionsPerEra".as_bytes() }
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
      " Maximum reward, per validator, that is provided per acceptable session."]
pub struct SessionReward<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>
 for SessionReward<T> {
    type
    Query
    =
    Perbill;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking SessionReward".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::key()).unwrap_or_else(||
                                                                                                                                                                 Perbill::from_parts(60))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::key()).unwrap_or_else(||
                                                                                                                                                                  Perbill::from_parts(60))
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::put(&val,
                                                                                                                               storage);
        ret
    }
}
#[doc =
      " Slash, per validator that is taken for the first time they are found to be offline."]
pub struct OfflineSlash<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>
 for OfflineSlash<T> {
    type
    Query
    =
    Perbill;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking OfflineSlash".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::key()).unwrap_or_else(||
                                                                                                                                                                 Perbill::from_millionths(1000))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::key()).unwrap_or_else(||
                                                                                                                                                                  Perbill::from_millionths(1000))
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::put(&val,
                                                                                                                               storage);
        ret
    }
}
#[doc =
      " Number of instances of offline reports before slashing begins for validators."]
pub struct OfflineSlashGrace<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>
 for OfflineSlashGrace<T> {
    type
    Query
    =
    u32;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking OfflineSlashGrace".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
                                                                                                                                                             Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::key()).unwrap_or_else(||
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
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::put(&val,
                                                                                                                           storage);
        ret
    }
}
#[doc = " The length of the bonding duration in eras."]
pub struct BondingDuration<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for BondingDuration<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking BondingDuration".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                        T::BlockNumber::sa(12))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                         T::BlockNumber::sa(12))
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
      " Any validators that may never be slashed or forcibly kicked. It\'s a Vec since they\'re easy to initialize"]
#[doc =
      " and the performance hit is minimal (we expect no more than four invulnerables) and restricted to testnets."]
pub struct Invulnerables<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>
 for Invulnerables<T> {
    type
    Query
    =
    Vec<T::AccountId>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking Invulnerables".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::key()).unwrap_or_else(||
                                                                                                                                                                           Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::put(&val,
                                                                                                                                         storage);
        ret
    }
}
#[doc = " Map from all locked \"stash\" accounts to the controller account."]
pub struct Bonded<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   T::AccountId>
 for Bonded<T> {
    type
    Query
    =
    Option<T::AccountId>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Staking Bonded".as_bytes() }
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
#[doc =
      " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
pub struct Ledger<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   StakingLedger<T::AccountId,
                                                                                                                 BalanceOf<T>,
                                                                                                                 T::BlockNumber>>
 for Ledger<T> {
    type
    Query
    =
    Option<StakingLedger<T::AccountId, BalanceOf<T>, T::BlockNumber>>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Staking Ledger".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  StakingLedger<T::AccountId,
                                                                                                                                BalanceOf<T>,
                                                                                                                                T::BlockNumber>>>::prefix().to_vec();
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
                                                                                                                  StakingLedger<T::AccountId,
                                                                                                                                BalanceOf<T>,
                                                                                                                                T::BlockNumber>>>::key_for(key);
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
                                                                                                                  StakingLedger<T::AccountId,
                                                                                                                                BalanceOf<T>,
                                                                                                                                T::BlockNumber>>>::key_for(key);
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
                                                                                                                  StakingLedger<T::AccountId,
                                                                                                                                BalanceOf<T>,
                                                                                                                                T::BlockNumber>>>::get(key,
                                                                                                                                                       storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  StakingLedger<T::AccountId,
                                                                                                                                BalanceOf<T>,
                                                                                                                                T::BlockNumber>>>::insert(key,
                                                                                                                                                          &val,
                                                                                                                                                          storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  StakingLedger<T::AccountId,
                                                                                                                                BalanceOf<T>,
                                                                                                                                T::BlockNumber>>>::remove(key,
                                                                                                                                                          storage),
        };
        ret
    }
}
#[doc = " Where the reward payment should be made. Keyed by stash."]
pub struct Payee<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   RewardDestination>
 for Payee<T> {
    type
    Query
    =
    RewardDestination;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Staking Payee".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  RewardDestination>>::prefix().to_vec();
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
                                                                                                                  RewardDestination>>::key_for(key);
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
                                                                                                                  RewardDestination>>::key_for(key);
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
                                                                                                                  RewardDestination>>::get(key,
                                                                                                                                           storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              RewardDestination>>::insert(key,
                                                                                                                                          &val,
                                                                                                                                          storage);
        ret
    }
}
#[doc = r" Linkage data of an element (it's successor and predecessor)"]
pub(crate) struct __LinkageForValidatorsDoNotUse<Key> {
    #[doc = r" Previous element key in storage (None for the first element)"]
    pub previous: Option<Key>,
    #[doc = r" Next element key in storage (None for the last element)"]
    pub next: Option<Key>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR___LinkageForValidatorsDoNotUse: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Key> _parity_codec::Encode for
         __LinkageForValidatorsDoNotUse<Key> where
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
const _IMPL_DECODE_FOR___LinkageForValidatorsDoNotUse: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Key> _parity_codec::Decode for
         __LinkageForValidatorsDoNotUse<Key> where
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(__LinkageForValidatorsDoNotUse{previous:
                                                        _parity_codec::Decode::decode(input)?,
                                                    next:
                                                        _parity_codec::Decode::decode(input)?,})
            }
        }
    };
mod __linked_map_details_for_validators_do_not_use {
    use super::*;
    #[doc =
          r" Re-exported version of linkage to overcome proc-macro derivation issue."]
    pub(crate) use super::__LinkageForValidatorsDoNotUse as Linkage;
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
     Enumerator<'a, S, T::AccountId, (ValidatorPrefs<BalanceOf<T>>, T)> where
     T: 'a {
        type
        Item
        =
        (T::AccountId, ValidatorPrefs<BalanceOf<T>>);
        fn next(&mut self) -> Option<Self::Item> {
            let next = self.next.take()?;
            let key_for =
                <super::Validators<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      ValidatorPrefs<BalanceOf<T>>>>::key_for(&next);
            let (val, linkage):
                    (ValidatorPrefs<BalanceOf<T>>, Linkage<T::AccountId>) =
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
        -> Option<(ValidatorPrefs<BalanceOf<T>>, Linkage<T::AccountId>)>;
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
      " The map from (wannabe) validator stash key to the preferences of that validator."]
pub struct Validators<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait> self::__linked_map_details_for_validators_do_not_use::Utils<T>
 for Validators<T> {
    fn remove_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(linkage:
                                                                                                                                                                              self::__linked_map_details_for_validators_do_not_use::Linkage<T::AccountId>,
                                                                                                                                                                          storage:
                                                                                                                                                                              &S) {
        use self::__linked_map_details_for_validators_do_not_use::Utils;
        let next_key =
            linkage.next.as_ref().map(|x|
                                          <Self as
                                              self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                                ValidatorPrefs<BalanceOf<T>>>>::key_for(x));
        let prev_key =
            linkage.previous.as_ref().map(|x|
                                              <Self as
                                                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                                    ValidatorPrefs<BalanceOf<T>>>>::key_for(x));
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
         Option<(ValidatorPrefs<BalanceOf<T>>,
                 self::__linked_map_details_for_validators_do_not_use::Linkage<T::AccountId>)> {
        storage.get(key)
    }
    fn new_head_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                &S,
                                                                                                                                                                            key:
                                                                                                                                                                                &T::AccountId)
     ->
         self::__linked_map_details_for_validators_do_not_use::Linkage<T::AccountId> {
        use self::__linked_map_details_for_validators_do_not_use::Utils;
        if let Some(head) = Self::read_head(storage) {
            {
                let head_key =
                    <Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                          ValidatorPrefs<BalanceOf<T>>>>::key_for(&head);
                let (data, linkage) =
                    Self::read_with_linkage(storage,
                                            &*head_key).expect(r#"
								head is set when first element is inserted and unset when last element is removed;
								if head is Some then it points to existing key; qed
							"#);
                storage.put(&*head_key,
                            &(data,
                              self::__linked_map_details_for_validators_do_not_use::Linkage{next:
                                                                                                linkage.next.as_ref(),
                                                                                            previous:
                                                                                                Some(key),}));
            }
            Self::write_head(storage, Some(key));
            let mut linkage =
                self::__linked_map_details_for_validators_do_not_use::Linkage::default();
            linkage.next = Some(head);
            linkage
        } else {
            Self::write_head(storage, Some(key));
            self::__linked_map_details_for_validators_do_not_use::Linkage::default()
        }
    }
    fn read_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                         &S)
     -> Option<T::AccountId> {
        storage.get("head of Staking Validators".as_bytes())
    }
    fn write_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                          &S,
                                                                                                                                                                      head:
                                                                                                                                                                          Option<&T::AccountId>) {
        match head {
            Some(head) =>
            storage.put("head of Staking Validators".as_bytes(), head),
            None => storage.kill("head of Staking Validators".as_bytes()),
        }
    }
}
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   ValidatorPrefs<BalanceOf<T>>>
 for Validators<T> {
    type
    Query
    =
    ValidatorPrefs<BalanceOf<T>>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Staking Validators".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(key: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key_for =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  ValidatorPrefs<BalanceOf<T>>>>::prefix().to_vec();
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
                                                                                                                            ValidatorPrefs<BalanceOf<T>>>>::key_for(key)).unwrap_or_else(||
                                                                                                                                                                                             Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &T::AccountId,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        use self::__linked_map_details_for_validators_do_not_use::Utils;
        let res:
                Option<(ValidatorPrefs<BalanceOf<T>>,
                        self::__linked_map_details_for_validators_do_not_use::Linkage<T::AccountId>)> =
            storage.take(&*<Self as
                               self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                 ValidatorPrefs<BalanceOf<T>>>>::key_for(key));
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
                                                                                                              ValidatorPrefs<BalanceOf<T>>>>::take(key,
                                                                                                                                                   storage);
    }
    #[doc =
          r" Store a value to be associated with the given key from the map."]
    fn insert<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::AccountId,
                                                                                                                                                                  val:
                                                                                                                                                                      &ValidatorPrefs<BalanceOf<T>>,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S) {
        use self::__linked_map_details_for_validators_do_not_use::Utils;
        let key_for =
            &*<Self as
                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                    ValidatorPrefs<BalanceOf<T>>>>::key_for(key);
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
        use self::__linked_map_details_for_validators_do_not_use::Utils;
        let key_for =
            &*<Self as
                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                    ValidatorPrefs<BalanceOf<T>>>>::key_for(key);
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
                                                                                                                  ValidatorPrefs<BalanceOf<T>>>>::insert(key,
                                                                                                                                                         &val,
                                                                                                                                                         storage),
        };
        ret
    }
}
impl <T: 'static + Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::EnumerableStorageMap<T::AccountId,
                                                                                                             ValidatorPrefs<BalanceOf<T>>>
 for Validators<T> {
    fn head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                    &S)
     -> Option<T::AccountId> {
        use self::__linked_map_details_for_validators_do_not_use::Utils;
        Self::read_head(storage)
    }
    fn enumerate<'a,
                 S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                         &'a S)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::boxed::Box<dyn Iterator<Item
                                                                                     =
                                                                                     (T::AccountId,
                                                                                      ValidatorPrefs<BalanceOf<T>>)> +
                                                                                     'a>
     where T::AccountId: 'a, ValidatorPrefs<BalanceOf<T>>: 'a {
        use self::__linked_map_details_for_validators_do_not_use::{Utils,
                                                                   Enumerator};
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::boxed::Box::new(Enumerator{next:
                                                                                                        Self::read_head(storage),
                                                                                                    storage,
                                                                                                    _data:
                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData::<(ValidatorPrefs<BalanceOf<T>>,
                                                                                                                                                                                                T)>::default(),})
    }
}
#[doc = r" Linkage data of an element (it's successor and predecessor)"]
pub(crate) struct __LinkageForNominatorsDoNotUse<Key> {
    #[doc = r" Previous element key in storage (None for the first element)"]
    pub previous: Option<Key>,
    #[doc = r" Next element key in storage (None for the last element)"]
    pub next: Option<Key>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR___LinkageForNominatorsDoNotUse: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Key> _parity_codec::Encode for
         __LinkageForNominatorsDoNotUse<Key> where
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
const _IMPL_DECODE_FOR___LinkageForNominatorsDoNotUse: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Key> _parity_codec::Decode for
         __LinkageForNominatorsDoNotUse<Key> where
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(__LinkageForNominatorsDoNotUse{previous:
                                                        _parity_codec::Decode::decode(input)?,
                                                    next:
                                                        _parity_codec::Decode::decode(input)?,})
            }
        }
    };
mod __linked_map_details_for_nominators_do_not_use {
    use super::*;
    #[doc =
          r" Re-exported version of linkage to overcome proc-macro derivation issue."]
    pub(crate) use super::__LinkageForNominatorsDoNotUse as Linkage;
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
     Enumerator<'a, S, T::AccountId, (Vec<T::AccountId>, T)> where T: 'a {
        type
        Item
        =
        (T::AccountId, Vec<T::AccountId>);
        fn next(&mut self) -> Option<Self::Item> {
            let next = self.next.take()?;
            let key_for =
                <super::Nominators<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      Vec<T::AccountId>>>::key_for(&next);
            let (val, linkage): (Vec<T::AccountId>, Linkage<T::AccountId>) =
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
        -> Option<(Vec<T::AccountId>, Linkage<T::AccountId>)>;
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
      " The map from nominator stash key to the set of stash keys of all validators to nominate."]
pub struct Nominators<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait> self::__linked_map_details_for_nominators_do_not_use::Utils<T>
 for Nominators<T> {
    fn remove_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(linkage:
                                                                                                                                                                              self::__linked_map_details_for_nominators_do_not_use::Linkage<T::AccountId>,
                                                                                                                                                                          storage:
                                                                                                                                                                              &S) {
        use self::__linked_map_details_for_nominators_do_not_use::Utils;
        let next_key =
            linkage.next.as_ref().map(|x|
                                          <Self as
                                              self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                                Vec<T::AccountId>>>::key_for(x));
        let prev_key =
            linkage.previous.as_ref().map(|x|
                                              <Self as
                                                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                                    Vec<T::AccountId>>>::key_for(x));
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
         Option<(Vec<T::AccountId>,
                 self::__linked_map_details_for_nominators_do_not_use::Linkage<T::AccountId>)> {
        storage.get(key)
    }
    fn new_head_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                &S,
                                                                                                                                                                            key:
                                                                                                                                                                                &T::AccountId)
     ->
         self::__linked_map_details_for_nominators_do_not_use::Linkage<T::AccountId> {
        use self::__linked_map_details_for_nominators_do_not_use::Utils;
        if let Some(head) = Self::read_head(storage) {
            {
                let head_key =
                    <Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                          Vec<T::AccountId>>>::key_for(&head);
                let (data, linkage) =
                    Self::read_with_linkage(storage,
                                            &*head_key).expect(r#"
								head is set when first element is inserted and unset when last element is removed;
								if head is Some then it points to existing key; qed
							"#);
                storage.put(&*head_key,
                            &(data,
                              self::__linked_map_details_for_nominators_do_not_use::Linkage{next:
                                                                                                linkage.next.as_ref(),
                                                                                            previous:
                                                                                                Some(key),}));
            }
            Self::write_head(storage, Some(key));
            let mut linkage =
                self::__linked_map_details_for_nominators_do_not_use::Linkage::default();
            linkage.next = Some(head);
            linkage
        } else {
            Self::write_head(storage, Some(key));
            self::__linked_map_details_for_nominators_do_not_use::Linkage::default()
        }
    }
    fn read_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                         &S)
     -> Option<T::AccountId> {
        storage.get("head of Staking Nominators".as_bytes())
    }
    fn write_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                          &S,
                                                                                                                                                                      head:
                                                                                                                                                                          Option<&T::AccountId>) {
        match head {
            Some(head) =>
            storage.put("head of Staking Nominators".as_bytes(), head),
            None => storage.kill("head of Staking Nominators".as_bytes()),
        }
    }
}
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   Vec<T::AccountId>>
 for Nominators<T> {
    type
    Query
    =
    Vec<T::AccountId>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Staking Nominators".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(key: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key_for =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  Vec<T::AccountId>>>::prefix().to_vec();
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
                                                                                                                            Vec<T::AccountId>>>::key_for(key)).unwrap_or_else(||
                                                                                                                                                                                  Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &T::AccountId,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        use self::__linked_map_details_for_nominators_do_not_use::Utils;
        let res:
                Option<(Vec<T::AccountId>,
                        self::__linked_map_details_for_nominators_do_not_use::Linkage<T::AccountId>)> =
            storage.take(&*<Self as
                               self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                 Vec<T::AccountId>>>::key_for(key));
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
                                                                                                              Vec<T::AccountId>>>::take(key,
                                                                                                                                        storage);
    }
    #[doc =
          r" Store a value to be associated with the given key from the map."]
    fn insert<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::AccountId,
                                                                                                                                                                  val:
                                                                                                                                                                      &Vec<T::AccountId>,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S) {
        use self::__linked_map_details_for_nominators_do_not_use::Utils;
        let key_for =
            &*<Self as
                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                    Vec<T::AccountId>>>::key_for(key);
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
        use self::__linked_map_details_for_nominators_do_not_use::Utils;
        let key_for =
            &*<Self as
                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                    Vec<T::AccountId>>>::key_for(key);
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
                                                                                                                  Vec<T::AccountId>>>::insert(key,
                                                                                                                                              &val,
                                                                                                                                              storage),
        };
        ret
    }
}
impl <T: 'static + Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::EnumerableStorageMap<T::AccountId,
                                                                                                             Vec<T::AccountId>>
 for Nominators<T> {
    fn head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                    &S)
     -> Option<T::AccountId> {
        use self::__linked_map_details_for_nominators_do_not_use::Utils;
        Self::read_head(storage)
    }
    fn enumerate<'a,
                 S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                         &'a S)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::boxed::Box<dyn Iterator<Item
                                                                                     =
                                                                                     (T::AccountId,
                                                                                      Vec<T::AccountId>)> +
                                                                                     'a>
     where T::AccountId: 'a, Vec<T::AccountId>: 'a {
        use self::__linked_map_details_for_nominators_do_not_use::{Utils,
                                                                   Enumerator};
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::boxed::Box::new(Enumerator{next:
                                                                                                        Self::read_head(storage),
                                                                                                    storage,
                                                                                                    _data:
                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData::<(Vec<T::AccountId>,
                                                                                                                                                                                                T)>::default(),})
    }
}
#[doc =
      " Nominators for a particular account that is in action right now. You can\'t iterate through validators here,"]
#[doc = " but you can find them in the Session module."]
#[doc = ""]
#[doc = " This is keyed by the stash account."]
pub struct Stakers<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   Exposure<T::AccountId,
                                                                                                            BalanceOf<T>>>
 for Stakers<T> {
    type
    Query
    =
    Exposure<T::AccountId, BalanceOf<T>>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Staking Stakers".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  Exposure<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::prefix().to_vec();
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
                                                                                                                  Exposure<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::key_for(key);
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
                                                                                                                  Exposure<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::key_for(key);
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
                                                                                                                  Exposure<T::AccountId,
                                                                                                                           BalanceOf<T>>>>::get(key,
                                                                                                                                                storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              Exposure<T::AccountId,
                                                                                                                       BalanceOf<T>>>>::insert(key,
                                                                                                                                               &val,
                                                                                                                                               storage);
        ret
    }
}
#[doc = " The currently elected validator set keyed by stash account ID."]
pub struct CurrentElected<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>
 for CurrentElected<T> {
    type
    Query
    =
    Vec<T::AccountId>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking CurrentElected".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::key()).unwrap_or_else(||
                                                                                                                                                                           Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::put(&val,
                                                                                                                                         storage);
        ret
    }
}
#[doc = " The current era index."]
pub struct CurrentEra<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for CurrentEra<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking CurrentEra".as_bytes() }
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
      " Maximum reward, per validator, that is provided per acceptable session."]
pub struct CurrentSessionReward<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for CurrentSessionReward<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking CurrentSessionReward".as_bytes() }
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
      " The accumulated reward for the current era. Reset to zero at the beginning of the era and"]
#[doc = " increased for every successfully finished session."]
pub struct CurrentEraReward<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for CurrentEraReward<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking CurrentEraReward".as_bytes() }
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
#[doc = " The next value of sessions per era."]
pub struct NextSessionsPerEra<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for NextSessionsPerEra<T> {
    type
    Query
    =
    Option<T::BlockNumber>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking NextSessionsPerEra".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).or_else(||
                                                                                                                                                                 Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).or_else(||
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
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&val,
                                                                                                                                          storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::kill(storage),
        };
        ret
    }
}
#[doc = " The session index at which the era length last changed."]
pub struct LastEraLengthChange<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for LastEraLengthChange<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking LastEraLengthChange".as_bytes() }
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
      " The amount of balance actively at stake for each validator slot, currently."]
#[doc = ""]
#[doc = " This is used to derive rewards and punishments."]
pub struct SlotStake<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>
 for SlotStake<T> {
    type
    Query
    =
    BalanceOf<T>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking SlotStake".as_bytes() }
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
      " The number of times a given validator has been reported offline. This gets decremented by one each era that passes."]
pub struct SlashCount<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   u32>
 for SlashCount<T> {
    type
    Query
    =
    u32;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Staking SlashCount".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  u32>>::prefix().to_vec();
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
                                                                                                                  u32>>::key_for(key);
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
                                                                                                                  u32>>::key_for(key);
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
                                                                                                                  u32>>::get(key,
                                                                                                                             storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              u32>>::insert(key,
                                                                                                                            &val,
                                                                                                                            storage);
        ret
    }
}
#[doc = " We are forcing a new era."]
pub struct ForcingNewEra<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<()>
 for ForcingNewEra<T> {
    type
    Query
    =
    Option<()>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking ForcingNewEra".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<()>>::key()).or_else(||
                                                                                                                                                     Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<()>>::key()).or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<()>>::get(storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<()>>::put(&val,
                                                                                                                              storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<()>>::kill(storage),
        };
        ret
    }
}
#[doc =
      " Most recent `RECENT_OFFLINE_COUNT` instances. (Who it was, when it was reported, how many instances they were offline for)."]
pub struct RecentlyOffline<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                          T::BlockNumber,
                                                                                                          u32)>>
 for RecentlyOffline<T> {
    type
    Query
    =
    Vec<(T::AccountId, T::BlockNumber, u32)>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Staking RecentlyOffline".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                                 T::BlockNumber,
                                                                                                                                 u32)>>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                                  T::BlockNumber,
                                                                                                                                  u32)>>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                         T::BlockNumber,
                                                                                                                         u32)>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                     T::BlockNumber,
                                                                                                                     u32)>>>::put(&val,
                                                                                                                                  storage);
        ret
    }
}
trait Store {
    type
    ValidatorCount;
    type
    MinimumValidatorCount;
    type
    SessionsPerEra;
    type
    SessionReward;
    type
    OfflineSlash;
    type
    OfflineSlashGrace;
    type
    BondingDuration;
    type
    Invulnerables;
    type
    Bonded;
    type
    Ledger;
    type
    Payee;
    type
    Validators;
    type
    Nominators;
    type
    Stakers;
    type
    CurrentElected;
    type
    CurrentEra;
    type
    CurrentSessionReward;
    type
    CurrentEraReward;
    type
    NextSessionsPerEra;
    type
    LastEraLengthChange;
    type
    SlotStake;
    type
    SlashCount;
    type
    ForcingNewEra;
    type
    RecentlyOffline;
}
#[doc(hidden)]
pub struct __GetByteStructValidatorCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ValidatorCount:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructValidatorCount<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ValidatorCount.get_or_init(||
                                                               {
                                                                   let def_val:
                                                                           u32 =
                                                                       Default::default();
                                                                   <u32 as
                                                                       Encode>::encode(&def_val)
                                                               }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructMinimumValidatorCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_MinimumValidatorCount:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructMinimumValidatorCount<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_MinimumValidatorCount.get_or_init(||
                                                                      {
                                                                          let def_val:
                                                                                  u32 =
                                                                              DEFAULT_MINIMUM_VALIDATOR_COUNT;
                                                                          <u32
                                                                              as
                                                                              Encode>::encode(&def_val)
                                                                      }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructSessionsPerEra<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_SessionsPerEra:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructSessionsPerEra<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_SessionsPerEra.get_or_init(||
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
pub struct __GetByteStructSessionReward<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_SessionReward:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructSessionReward<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_SessionReward.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          Perbill =
                                                                      Perbill::from_parts(60);
                                                                  <Perbill as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructOfflineSlash<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OfflineSlash:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructOfflineSlash<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OfflineSlash.get_or_init(||
                                                             {
                                                                 let def_val:
                                                                         Perbill =
                                                                     Perbill::from_millionths(1000);
                                                                 <Perbill as
                                                                     Encode>::encode(&def_val)
                                                             }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructOfflineSlashGrace<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OfflineSlashGrace:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructOfflineSlashGrace<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OfflineSlashGrace.get_or_init(||
                                                                  {
                                                                      let def_val:
                                                                              u32 =
                                                                          Default::default();
                                                                      <u32 as
                                                                          Encode>::encode(&def_val)
                                                                  }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructBondingDuration<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_BondingDuration:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructBondingDuration<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_BondingDuration.get_or_init(||
                                                                {
                                                                    let def_val:
                                                                            T::BlockNumber =
                                                                        T::BlockNumber::sa(12);
                                                                    <T::BlockNumber
                                                                        as
                                                                        Encode>::encode(&def_val)
                                                                }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructInvulnerables<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Invulnerables:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructInvulnerables<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Invulnerables.get_or_init(||
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
pub struct __GetByteStructBonded<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Bonded:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructBonded<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Bonded.get_or_init(||
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
pub struct __GetByteStructLedger<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Ledger:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructLedger<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Ledger.get_or_init(||
                                                       {
                                                           let def_val:
                                                                   Option<StakingLedger<T::AccountId,
                                                                                        BalanceOf<T>,
                                                                                        T::BlockNumber>> =
                                                               Default::default();
                                                           <Option<StakingLedger<T::AccountId,
                                                                                 BalanceOf<T>,
                                                                                 T::BlockNumber>>
                                                               as
                                                               Encode>::encode(&def_val)
                                                       }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructPayee<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Payee:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructPayee<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Payee.get_or_init(||
                                                      {
                                                          let def_val:
                                                                  RewardDestination =
                                                              Default::default();
                                                          <RewardDestination
                                                              as
                                                              Encode>::encode(&def_val)
                                                      }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructValidators<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Validators:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructValidators<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Validators.get_or_init(||
                                                           {
                                                               let def_val:
                                                                       ValidatorPrefs<BalanceOf<T>> =
                                                                   Default::default();
                                                               <ValidatorPrefs<BalanceOf<T>>
                                                                   as
                                                                   Encode>::encode(&def_val)
                                                           }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructNominators<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Nominators:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNominators<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Nominators.get_or_init(||
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
pub struct __GetByteStructStakers<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Stakers:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructStakers<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Stakers.get_or_init(||
                                                        {
                                                            let def_val:
                                                                    Exposure<T::AccountId,
                                                                             BalanceOf<T>> =
                                                                Default::default();
                                                            <Exposure<T::AccountId,
                                                                      BalanceOf<T>>
                                                                as
                                                                Encode>::encode(&def_val)
                                                        }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructCurrentElected<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CurrentElected:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCurrentElected<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CurrentElected.get_or_init(||
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
pub struct __GetByteStructCurrentEra<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CurrentEra:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCurrentEra<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CurrentEra.get_or_init(||
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
pub struct __GetByteStructCurrentSessionReward<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CurrentSessionReward:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCurrentSessionReward<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CurrentSessionReward.get_or_init(||
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
pub struct __GetByteStructCurrentEraReward<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CurrentEraReward:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCurrentEraReward<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CurrentEraReward.get_or_init(||
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
pub struct __GetByteStructNextSessionsPerEra<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_NextSessionsPerEra:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNextSessionsPerEra<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_NextSessionsPerEra.get_or_init(||
                                                                   {
                                                                       let def_val:
                                                                               Option<T::BlockNumber> =
                                                                           Default::default();
                                                                       <Option<T::BlockNumber>
                                                                           as
                                                                           Encode>::encode(&def_val)
                                                                   }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructLastEraLengthChange<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_LastEraLengthChange:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructLastEraLengthChange<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_LastEraLengthChange.get_or_init(||
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
pub struct __GetByteStructSlotStake<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_SlotStake:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructSlotStake<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_SlotStake.get_or_init(||
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
pub struct __GetByteStructSlashCount<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_SlashCount:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructSlashCount<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_SlashCount.get_or_init(||
                                                           {
                                                               let def_val:
                                                                       u32 =
                                                                   Default::default();
                                                               <u32 as
                                                                   Encode>::encode(&def_val)
                                                           }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructForcingNewEra<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ForcingNewEra:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructForcingNewEra<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ForcingNewEra.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          Option<()> =
                                                                      Default::default();
                                                                  <Option<()>
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructRecentlyOffline<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_RecentlyOffline:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructRecentlyOffline<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_RecentlyOffline.get_or_init(||
                                                                {
                                                                    let def_val:
                                                                            Vec<(T::AccountId,
                                                                                 T::BlockNumber,
                                                                                 u32)> =
                                                                        Default::default();
                                                                    <Vec<(T::AccountId,
                                                                          T::BlockNumber,
                                                                          u32)>
                                                                        as
                                                                        Encode>::encode(&def_val)
                                                                }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    ValidatorCount
    =
    ValidatorCount<T>;
    type
    MinimumValidatorCount
    =
    MinimumValidatorCount<T>;
    type
    SessionsPerEra
    =
    SessionsPerEra<T>;
    type
    SessionReward
    =
    SessionReward<T>;
    type
    OfflineSlash
    =
    OfflineSlash<T>;
    type
    OfflineSlashGrace
    =
    OfflineSlashGrace<T>;
    type
    BondingDuration
    =
    BondingDuration<T>;
    type
    Invulnerables
    =
    Invulnerables<T>;
    type
    Bonded
    =
    Bonded<T>;
    type
    Ledger
    =
    Ledger<T>;
    type
    Payee
    =
    Payee<T>;
    type
    Validators
    =
    Validators<T>;
    type
    Nominators
    =
    Nominators<T>;
    type
    Stakers
    =
    Stakers<T>;
    type
    CurrentElected
    =
    CurrentElected<T>;
    type
    CurrentEra
    =
    CurrentEra<T>;
    type
    CurrentSessionReward
    =
    CurrentSessionReward<T>;
    type
    CurrentEraReward
    =
    CurrentEraReward<T>;
    type
    NextSessionsPerEra
    =
    NextSessionsPerEra<T>;
    type
    LastEraLengthChange
    =
    LastEraLengthChange<T>;
    type
    SlotStake
    =
    SlotStake<T>;
    type
    SlashCount
    =
    SlashCount<T>;
    type
    ForcingNewEra
    =
    ForcingNewEra<T>;
    type
    RecentlyOffline
    =
    RecentlyOffline<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc = " The ideal number of staking participants."]
    pub fn validator_count() -> u32 {
        <ValidatorCount<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Minimum number of staking participants before emergency conditions are imposed."]
    pub fn minimum_validator_count() -> u32 {
        <MinimumValidatorCount<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The length of a staking era in sessions."]
    pub fn sessions_per_era() -> T::BlockNumber {
        <SessionsPerEra<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Maximum reward, per validator, that is provided per acceptable session."]
    pub fn session_reward() -> Perbill {
        <SessionReward<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Slash, per validator that is taken for the first time they are found to be offline."]
    pub fn offline_slash() -> Perbill {
        <OfflineSlash<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Number of instances of offline reports before slashing begins for validators."]
    pub fn offline_slash_grace() -> u32 {
        <OfflineSlashGrace<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<u32>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The length of the bonding duration in eras."]
    pub fn bonding_duration() -> T::BlockNumber {
        <BondingDuration<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Any validators that may never be slashed or forcibly kicked. It\'s a Vec since they\'re easy to initialize"]
    #[doc =
          " and the performance hit is minimal (we expect no more than four invulnerables) and restricted to testnets."]
    pub fn invulnerables() -> Vec<T::AccountId> {
        <Invulnerables<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Map from all locked \"stash\" accounts to the controller account."]
    pub fn bonded<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                        K)
     -> Option<T::AccountId> {
        <Bonded<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::AccountId>>::get(key.borrow(),
                                                                                                                                  &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]
    pub fn ledger<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                        K)
     -> Option<StakingLedger<T::AccountId, BalanceOf<T>, T::BlockNumber>> {
        <Ledger<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              StakingLedger<T::AccountId,
                                                                                                                            BalanceOf<T>,
                                                                                                                            T::BlockNumber>>>::get(key.borrow(),
                                                                                                                                                   &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Where the reward payment should be made. Keyed by stash."]
    pub fn payee<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                       K)
     -> RewardDestination {
        <Payee<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              RewardDestination>>::get(key.borrow(),
                                                                                                                                       &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The map from (wannabe) validator stash key to the preferences of that validator."]
    pub fn validators<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                            K)
     -> ValidatorPrefs<BalanceOf<T>> {
        <Validators<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              ValidatorPrefs<BalanceOf<T>>>>::get(key.borrow(),
                                                                                                                                                  &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The map from nominator stash key to the set of stash keys of all validators to nominate."]
    pub fn nominators<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                            K)
     -> Vec<T::AccountId> {
        <Nominators<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              Vec<T::AccountId>>>::get(key.borrow(),
                                                                                                                                       &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Nominators for a particular account that is in action right now. You can\'t iterate through validators here,"]
    #[doc = " but you can find them in the Session module."]
    #[doc = ""]
    #[doc = " This is keyed by the stash account."]
    pub fn stakers<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                         K)
     -> Exposure<T::AccountId, BalanceOf<T>> {
        <Stakers<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              Exposure<T::AccountId,
                                                                                                                       BalanceOf<T>>>>::get(key.borrow(),
                                                                                                                                            &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The currently elected validator set keyed by stash account ID."]
    pub fn current_elected() -> Vec<T::AccountId> {
        <CurrentElected<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The current era index."]
    pub fn current_era() -> T::BlockNumber {
        <CurrentEra<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Maximum reward, per validator, that is provided per acceptable session."]
    pub fn current_session_reward() -> BalanceOf<T> {
        <CurrentSessionReward<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The accumulated reward for the current era. Reset to zero at the beginning of the era and"]
    #[doc = " increased for every successfully finished session."]
    pub fn current_era_reward() -> BalanceOf<T> {
        <CurrentEraReward<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The next value of sessions per era."]
    pub fn next_sessions_per_era() -> Option<T::BlockNumber> {
        <NextSessionsPerEra<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The session index at which the era length last changed."]
    pub fn last_era_length_change() -> T::BlockNumber {
        <LastEraLengthChange<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The amount of balance actively at stake for each validator slot, currently."]
    #[doc = ""]
    #[doc = " This is used to derive rewards and punishments."]
    pub fn slot_stake() -> BalanceOf<T> {
        <SlotStake<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The number of times a given validator has been reported offline. This gets decremented by one each era that passes."]
    pub fn slash_count<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                             K)
     -> u32 {
        <SlashCount<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              u32>>::get(key.borrow(),
                                                                                                                         &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " We are forcing a new era."]
    pub fn forcing_new_era() -> Option<()> {
        <ForcingNewEra<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<()>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " Most recent `RECENT_OFFLINE_COUNT` instances. (Who it was, when it was reported, how many instances they were offline for)."]
    pub fn recently_offline() -> Vec<(T::AccountId, T::BlockNumber, u32)> {
        <RecentlyOffline<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<(T::AccountId,
                                                                                                                     T::BlockNumber,
                                                                                                                     u32)>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ValidatorCount"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructValidatorCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The ideal number of staking participants."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MinimumValidatorCount"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMinimumValidatorCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Minimum number of staking participants before emergency conditions are imposed."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SessionsPerEra"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSessionsPerEra::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The length of a staking era in sessions."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SessionReward"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Perbill")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSessionReward::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Maximum reward, per validator, that is provided per acceptable session."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("OfflineSlash"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Perbill")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructOfflineSlash::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Slash, per validator that is taken for the first time they are found to be offline."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("OfflineSlashGrace"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructOfflineSlashGrace::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of instances of offline reports before slashing begins for validators."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BondingDuration"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBondingDuration::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The length of the bonding duration in eras."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Invulnerables"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructInvulnerables::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Any validators that may never be slashed or forcibly kicked. It\'s a Vec since they\'re easy to initialize",
                                                                                                                                                                                                                                                                                                                                                                                                    " and the performance hit is minimal (we expect no more than four invulnerables) and restricted to testnets."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Bonded"),
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
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBonded::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Map from all locked \"stash\" accounts to the controller account."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Ledger"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("StakingLedger<T::AccountId, BalanceOf<T>, T::BlockNumber>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLedger::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Payee"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RewardDestination"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPayee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Where the reward payment should be made. Keyed by stash."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Validators"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ValidatorPrefs<BalanceOf<T>>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       true,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructValidators::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The map from (wannabe) validator stash key to the preferences of that validator."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Nominators"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       true,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNominators::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The map from nominator stash key to the set of stash keys of all validators to nominate."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Stakers"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Exposure<T::AccountId, BalanceOf<T>>"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructStakers::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Nominators for a particular account that is in action right now. You can\'t iterate through validators here,",
                                                                                                                                                                                                                                                                                                                                                                                                    " but you can find them in the Session module.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " This is keyed by the stash account."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentElected"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentElected::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The currently elected validator set keyed by stash account ID."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentEra"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentEra::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The current era index."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentSessionReward"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentSessionReward::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Maximum reward, per validator, that is provided per acceptable session."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentEraReward"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentEraReward::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The accumulated reward for the current era. Reset to zero at the beginning of the era and",
                                                                                                                                                                                                                                                                                                                                                                                                    " increased for every successfully finished session."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextSessionsPerEra"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextSessionsPerEra::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next value of sessions per era."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LastEraLengthChange"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLastEraLengthChange::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The session index at which the era length last changed."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SlotStake"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSlotStake::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The amount of balance actively at stake for each validator slot, currently.",
                                                                                                                                                                                                                                                                                                                                                                                                    "",
                                                                                                                                                                                                                                                                                                                                                                                                    " This is used to derive rewards and punishments."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SlashCount"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSlashCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of times a given validator has been reported offline. This gets decremented by one each era that passes."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ForcingNewEra"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("()")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructForcingNewEra::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" We are forcing a new era."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RecentlyOffline"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(T::AccountId, T::BlockNumber, u32)>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRecentlyOffline::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Most recent `RECENT_OFFLINE_COUNT` instances. (Who it was, when it was reported, how many instances they were offline for)."]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ValidatorCount"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructValidatorCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The ideal number of staking participants."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MinimumValidatorCount"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMinimumValidatorCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Minimum number of staking participants before emergency conditions are imposed."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SessionsPerEra"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSessionsPerEra::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The length of a staking era in sessions."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SessionReward"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Perbill")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSessionReward::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Maximum reward, per validator, that is provided per acceptable session."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("OfflineSlash"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Perbill")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructOfflineSlash::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Slash, per validator that is taken for the first time they are found to be offline."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("OfflineSlashGrace"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructOfflineSlashGrace::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Number of instances of offline reports before slashing begins for validators."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BondingDuration"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBondingDuration::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The length of the bonding duration in eras."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Invulnerables"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructInvulnerables::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Any validators that may never be slashed or forcibly kicked. It\'s a Vec since they\'re easy to initialize",
                                                                                                                                                                                                              " and the performance hit is minimal (we expect no more than four invulnerables) and restricted to testnets."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Bonded"),
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
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBonded::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Map from all locked \"stash\" accounts to the controller account."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Ledger"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("StakingLedger<T::AccountId, BalanceOf<T>, T::BlockNumber>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLedger::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Map from all (unlocked) \"controller\" accounts to the info regarding the staking."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Payee"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RewardDestination"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPayee::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Where the reward payment should be made. Keyed by stash."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Validators"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ValidatorPrefs<BalanceOf<T>>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 true,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructValidators::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The map from (wannabe) validator stash key to the preferences of that validator."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Nominators"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 true,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNominators::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The map from nominator stash key to the set of stash keys of all validators to nominate."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Stakers"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Exposure<T::AccountId, BalanceOf<T>>"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructStakers::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Nominators for a particular account that is in action right now. You can\'t iterate through validators here,",
                                                                                                                                                                                                              " but you can find them in the Session module.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " This is keyed by the stash account."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentElected"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentElected::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The currently elected validator set keyed by stash account ID."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentEra"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentEra::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The current era index."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentSessionReward"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentSessionReward::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Maximum reward, per validator, that is provided per acceptable session."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentEraReward"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentEraReward::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The accumulated reward for the current era. Reset to zero at the beginning of the era and",
                                                                                                                                                                                                              " increased for every successfully finished session."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextSessionsPerEra"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextSessionsPerEra::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next value of sessions per era."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LastEraLengthChange"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLastEraLengthChange::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The session index at which the era length last changed."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SlotStake"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BalanceOf<T>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSlotStake::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The amount of balance actively at stake for each validator slot, currently.",
                                                                                                                                                                                                              "",
                                                                                                                                                                                                              " This is used to derive rewards and punishments."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SlashCount"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("u32"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSlashCount::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of times a given validator has been reported offline. This gets decremented by one each era that passes."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ForcingNewEra"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("()")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructForcingNewEra::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" We are forcing a new era."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RecentlyOffline"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<(T::AccountId, T::BlockNumber, u32)>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRecentlyOffline::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Most recent `RECENT_OFFLINE_COUNT` instances. (Who it was, when it was reported, how many instances they were offline for)."]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Staking" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Perbill : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Perbill : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Vec < T :: AccountId > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Vec < (\nT :: AccountId , T :: AccountId , BalanceOf < T > , StakerStatus < T ::\nAccountId > ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Perbill : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Perbill : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, u32 : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Vec < T :: AccountId > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, BalanceOf < T > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Vec < (\nT :: AccountId , T :: AccountId , BalanceOf < T > , StakerStatus < T ::\nAccountId > ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    #[doc = " The ideal number of staking participants."]
    pub validator_count: u32,
    #[doc =
          " Minimum number of staking participants before emergency conditions are imposed."]
    pub minimum_validator_count: u32,
    #[doc = " The length of a staking era in sessions."]
    pub sessions_per_era: T::BlockNumber,
    #[doc =
          " Maximum reward, per validator, that is provided per acceptable session."]
    pub session_reward: Perbill,
    #[doc =
          " Slash, per validator that is taken for the first time they are found to be offline."]
    pub offline_slash: Perbill,
    #[doc =
          " Number of instances of offline reports before slashing begins for validators."]
    pub offline_slash_grace: u32,
    #[doc = " The length of the bonding duration in eras."]
    pub bonding_duration: T::BlockNumber,
    #[doc =
          " Any validators that may never be slashed or forcibly kicked. It\'s a Vec since they\'re easy to initialize"]
    #[doc =
          " and the performance hit is minimal (we expect no more than four invulnerables) and restricted to testnets."]
    pub invulnerables: Vec<T::AccountId>,
    #[doc = " The current era index."]
    pub current_era: T::BlockNumber,
    #[doc =
          " Maximum reward, per validator, that is provided per acceptable session."]
    pub current_session_reward: BalanceOf<T>,
    pub stakers: Vec<(T::AccountId, T::AccountId, BalanceOf<T>,
                      StakerStatus<T::AccountId>)>,
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
         u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Perbill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Perbill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Vec<(T::AccountId, T::AccountId, BalanceOf<T>,
              StakerStatus<T::AccountId>)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
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
                                                                   + 1 + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "validatorCount",
                                                                    &self.validator_count)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "minimumValidatorCount",
                                                                    &self.minimum_validator_count)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "sessionsPerEra",
                                                                    &self.sessions_per_era)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "sessionReward",
                                                                    &self.session_reward)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "offlineSlash",
                                                                    &self.offline_slash)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "offlineSlashGrace",
                                                                    &self.offline_slash_grace)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "bondingDuration",
                                                                    &self.bonding_duration)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "invulnerables",
                                                                    &self.invulnerables)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "currentEra",
                                                                    &self.current_era)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "currentSessionReward",
                                                                    &self.current_session_reward)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "stakers",
                                                                    &self.stakers)
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
         u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Perbill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Perbill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Vec<(T::AccountId, T::AccountId, BalanceOf<T>,
              StakerStatus<T::AccountId>)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 11")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "validatorCount" =>
                            _serde::export::Ok(__Field::__field0),
                            "minimumValidatorCount" =>
                            _serde::export::Ok(__Field::__field1),
                            "sessionsPerEra" =>
                            _serde::export::Ok(__Field::__field2),
                            "sessionReward" =>
                            _serde::export::Ok(__Field::__field3),
                            "offlineSlash" =>
                            _serde::export::Ok(__Field::__field4),
                            "offlineSlashGrace" =>
                            _serde::export::Ok(__Field::__field5),
                            "bondingDuration" =>
                            _serde::export::Ok(__Field::__field6),
                            "invulnerables" =>
                            _serde::export::Ok(__Field::__field7),
                            "currentEra" =>
                            _serde::export::Ok(__Field::__field8),
                            "currentSessionReward" =>
                            _serde::export::Ok(__Field::__field9),
                            "stakers" =>
                            _serde::export::Ok(__Field::__field10),
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
                            b"validatorCount" =>
                            _serde::export::Ok(__Field::__field0),
                            b"minimumValidatorCount" =>
                            _serde::export::Ok(__Field::__field1),
                            b"sessionsPerEra" =>
                            _serde::export::Ok(__Field::__field2),
                            b"sessionReward" =>
                            _serde::export::Ok(__Field::__field3),
                            b"offlineSlash" =>
                            _serde::export::Ok(__Field::__field4),
                            b"offlineSlashGrace" =>
                            _serde::export::Ok(__Field::__field5),
                            b"bondingDuration" =>
                            _serde::export::Ok(__Field::__field6),
                            b"invulnerables" =>
                            _serde::export::Ok(__Field::__field7),
                            b"currentEra" =>
                            _serde::export::Ok(__Field::__field8),
                            b"currentSessionReward" =>
                            _serde::export::Ok(__Field::__field9),
                            b"stakers" =>
                            _serde::export::Ok(__Field::__field10),
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
                       u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Perbill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Perbill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Vec<(T::AccountId, T::AccountId, BalanceOf<T>,
                            StakerStatus<T::AccountId>)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Perbill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Perbill: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 u32: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 BalanceOf<T>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Vec<(T::AccountId, T::AccountId, BalanceOf<T>,
                      StakerStatus<T::AccountId>)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                                                                                                 &"struct GenesisConfig with 11 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct GenesisConfig with 11 elements"));
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
                                                                                                 &"struct GenesisConfig with 11 elements"));
                                }
                            };
                        let __field3 =
                            match match _serde::de::SeqAccess::next_element::<Perbill>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(3usize,
                                                                                                 &"struct GenesisConfig with 11 elements"));
                                }
                            };
                        let __field4 =
                            match match _serde::de::SeqAccess::next_element::<Perbill>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(4usize,
                                                                                                 &"struct GenesisConfig with 11 elements"));
                                }
                            };
                        let __field5 =
                            match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(5usize,
                                                                                                 &"struct GenesisConfig with 11 elements"));
                                }
                            };
                        let __field6 =
                            match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(6usize,
                                                                                                 &"struct GenesisConfig with 11 elements"));
                                }
                            };
                        let __field7 =
                            match match _serde::de::SeqAccess::next_element::<Vec<T::AccountId>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(7usize,
                                                                                                 &"struct GenesisConfig with 11 elements"));
                                }
                            };
                        let __field8 =
                            match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(8usize,
                                                                                                 &"struct GenesisConfig with 11 elements"));
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
                                                                                                 &"struct GenesisConfig with 11 elements"));
                                }
                            };
                        let __field10 =
                            match match _serde::de::SeqAccess::next_element::<Vec<(T::AccountId,
                                                                                   T::AccountId,
                                                                                   BalanceOf<T>,
                                                                                   StakerStatus<T::AccountId>)>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(10usize,
                                                                                                 &"struct GenesisConfig with 11 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{validator_count:
                                                             __field0,
                                                         minimum_validator_count:
                                                             __field1,
                                                         sessions_per_era:
                                                             __field2,
                                                         session_reward:
                                                             __field3,
                                                         offline_slash:
                                                             __field4,
                                                         offline_slash_grace:
                                                             __field5,
                                                         bonding_duration:
                                                             __field6,
                                                         invulnerables:
                                                             __field7,
                                                         current_era:
                                                             __field8,
                                                         current_session_reward:
                                                             __field9,
                                                         stakers: __field10,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0: _serde::export::Option<u32> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<u32> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<T::BlockNumber> =
                            _serde::export::None;
                        let mut __field3: _serde::export::Option<Perbill> =
                            _serde::export::None;
                        let mut __field4: _serde::export::Option<Perbill> =
                            _serde::export::None;
                        let mut __field5: _serde::export::Option<u32> =
                            _serde::export::None;
                        let mut __field6:
                                _serde::export::Option<T::BlockNumber> =
                            _serde::export::None;
                        let mut __field7:
                                _serde::export::Option<Vec<T::AccountId>> =
                            _serde::export::None;
                        let mut __field8:
                                _serde::export::Option<T::BlockNumber> =
                            _serde::export::None;
                        let mut __field9:
                                _serde::export::Option<BalanceOf<T>> =
                            _serde::export::None;
                        let mut __field10:
                                _serde::export::Option<Vec<(T::AccountId,
                                                            T::AccountId,
                                                            BalanceOf<T>,
                                                            StakerStatus<T::AccountId>)>> =
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
                                                                       _serde::de::Error>::duplicate_field("validatorCount"));
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
                                                                       _serde::de::Error>::duplicate_field("minimumValidatorCount"));
                                    }
                                    __field1 =
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
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("sessionsPerEra"));
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
                                                                       _serde::de::Error>::duplicate_field("sessionReward"));
                                    }
                                    __field3 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Perbill>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("offlineSlash"));
                                    }
                                    __field4 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Perbill>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("offlineSlashGrace"));
                                    }
                                    __field5 =
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
                                __Field::__field6 => {
                                    if _serde::export::Option::is_some(&__field6)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("bondingDuration"));
                                    }
                                    __field6 =
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
                                __Field::__field7 => {
                                    if _serde::export::Option::is_some(&__field7)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("invulnerables"));
                                    }
                                    __field7 =
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
                                __Field::__field8 => {
                                    if _serde::export::Option::is_some(&__field8)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("currentEra"));
                                    }
                                    __field8 =
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
                                __Field::__field9 => {
                                    if _serde::export::Option::is_some(&__field9)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("currentSessionReward"));
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
                                                                       _serde::de::Error>::duplicate_field("stakers"));
                                    }
                                    __field10 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<(T::AccountId,
                                                                                                            T::AccountId,
                                                                                                            BalanceOf<T>,
                                                                                                            StakerStatus<T::AccountId>)>>(&mut __map)
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
                                match _serde::private::de::missing_field("validatorCount")
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
                                match _serde::private::de::missing_field("minimumValidatorCount")
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
                                match _serde::private::de::missing_field("sessionsPerEra")
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
                                match _serde::private::de::missing_field("sessionReward")
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
                                match _serde::private::de::missing_field("offlineSlash")
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
                                match _serde::private::de::missing_field("offlineSlashGrace")
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
                                match _serde::private::de::missing_field("bondingDuration")
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
                                match _serde::private::de::missing_field("invulnerables")
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
                                match _serde::private::de::missing_field("currentEra")
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
                                match _serde::private::de::missing_field("currentSessionReward")
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
                                match _serde::private::de::missing_field("stakers")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{validator_count:
                                                             __field0,
                                                         minimum_validator_count:
                                                             __field1,
                                                         sessions_per_era:
                                                             __field2,
                                                         session_reward:
                                                             __field3,
                                                         offline_slash:
                                                             __field4,
                                                         offline_slash_grace:
                                                             __field5,
                                                         bonding_duration:
                                                             __field6,
                                                         invulnerables:
                                                             __field7,
                                                         current_era:
                                                             __field8,
                                                         current_session_reward:
                                                             __field9,
                                                         stakers: __field10,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["validatorCount", "minimumValidatorCount",
                      "sessionsPerEra", "sessionReward", "offlineSlash",
                      "offlineSlashGrace", "bondingDuration", "invulnerables",
                      "currentEra", "currentSessionReward", "stakers"];
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
        GenesisConfig{validator_count: Default::default(),
                      minimum_validator_count:
                          DEFAULT_MINIMUM_VALIDATOR_COUNT,
                      sessions_per_era: T::BlockNumber::sa(1000),
                      session_reward: Perbill::from_parts(60),
                      offline_slash: Perbill::from_millionths(1000),
                      offline_slash_grace: Default::default(),
                      bonding_duration: T::BlockNumber::sa(12),
                      invulnerables: Default::default(),
                      current_era: Default::default(),
                      current_session_reward: Default::default(),
                      stakers: Default::default(),}
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
                      config.validator_count.clone()))(&self);
            <ValidatorCount<T> as
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
                      config.minimum_validator_count.clone()))(&self);
            <MinimumValidatorCount<T> as
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
                      config.sessions_per_era.clone()))(&self);
            <SessionsPerEra<T> as
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
                      config.session_reward.clone()))(&self);
            <SessionReward<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::put(&v,
                                                                                                                                   &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.offline_slash.clone()))(&self);
            <OfflineSlash<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Perbill>>::put(&v,
                                                                                                                                   &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.offline_slash_grace.clone()))(&self);
            <OfflineSlashGrace<T> as
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
                      config.bonding_duration.clone()))(&self);
            <BondingDuration<T> as
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
                      config.invulnerables.clone()))(&self);
            <Invulnerables<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::put(&v,
                                                                                                                                             &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.current_era.clone()))(&self);
            <CurrentEra<T> as
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
                      config.current_session_reward.clone()))(&self);
            <CurrentSessionReward<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&v,
                                                                                                                                        &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                (|config: &GenesisConfig<T>|
                     {
                         config.stakers.iter().map(|&(_, _, value, _)|
                                                       value).min().unwrap_or_default()
                     })(&self);
            <SlotStake<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<BalanceOf<T>>>::put(&v,
                                                                                                                                        &storage);
        }
        let r = storage.into_inner();
        (|storage: &mut primitives::StorageOverlay,
          _: &mut primitives::ChildrenStorageOverlay,
          config: &GenesisConfig<T>|
             {
                 with_storage(storage,
                              ||
                                  {
                                      for &(ref stash, ref controller,
                                            balance, ref status) in
                                          &config.stakers {
                                          if !(T::Currency::free_balance(&stash)
                                                   >= balance) {
                                              {
                                                  ::std::rt::begin_panic("assertion failed: T::Currency::free_balance(&stash) >= balance",
                                                                         &("srml/staking/src/lib.rs",
                                                                           516u32,
                                                                           6u32))
                                              }
                                          };
                                          let _ =
                                              <Module<T>>::bond(T::Origin::from(Some(stash.clone()).into()),
                                                                T::Lookup::unlookup(controller.clone()),
                                                                balance,
                                                                RewardDestination::Staked);
                                          let _ =
                                              match status {
                                                  StakerStatus::Validator => {
                                                      <Module<T>>::validate(T::Origin::from(Some(controller.clone()).into()),
                                                                            Default::default())
                                                  }
                                                  StakerStatus::Nominator(votes)
                                                  => {
                                                      <Module<T>>::nominate(T::Origin::from(Some(controller.clone()).into()),
                                                                            votes.iter().map(|l|
                                                                                                 {
                                                                                                     T::Lookup::unlookup(l.clone())
                                                                                                 }).collect())
                                                  }
                                                  _ => Ok(()),
                                              };
                                      }
                                      <Module<T>>::select_validators();
                                  });
             })(r, c, &self);
        Ok(())
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
          r" Take the origin account as a stash and lock up `value` of its balance. `controller` will be the"]
    #[doc = r" account that controls it."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the stash account."]
    fn bond(origin: T::Origin,
            controller: <T::Lookup as StaticLookup>::Source,
            value: BalanceOf<T>, payee: RewardDestination)
     -> ::srml_support::dispatch::Result {
        {
            let stash = ensure_signed(origin)?;
            if <Bonded<T>>::exists(&stash) {
                return Err("stash already bonded")
            }
            let controller = T::Lookup::lookup(controller)?;
            if <Ledger<T>>::exists(&controller) {
                return Err("controller already paired")
            }
            <Bonded<T>>::insert(&stash, controller.clone());
            <Payee<T>>::insert(&stash, payee);
            let stash_balance = T::Currency::free_balance(&stash);
            let value = value.min(stash_balance);
            Self::update_ledger(&controller,
                                &StakingLedger{stash,
                                               total: value,
                                               active: value,
                                               unlocking:
                                                   <[_]>::into_vec(box []),});
        }
        Ok(())
    }
    #[doc =
          r" Add some extra amount that have appeared in the stash `free_balance` into the balance up for"]
    #[doc = r" staking."]
    #[doc = r""]
    #[doc =
          r" Use this if there are additional funds in your stash account that you wish to bond."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
    fn bond_extra(origin: T::Origin, max_additional: BalanceOf<T>)
     -> ::srml_support::dispatch::Result {
        {
            let stash = ensure_signed(origin)?;
            let controller = Self::bonded(&stash).ok_or("not a stash")?;
            let mut ledger =
                Self::ledger(&controller).ok_or("not a controller")?;
            let stash_balance = T::Currency::free_balance(&stash);
            if let Some(extra) = stash_balance.checked_sub(&ledger.total) {
                let extra = extra.min(max_additional);
                ledger.total += extra;
                ledger.active += extra;
                Self::update_ledger(&controller, &ledger);
            }
        }
        Ok(())
    }
    #[doc =
          r" Schedule a portion of the stash to be unlocked ready for transfer out after the bond"]
    #[doc =
          r" period ends. If this leaves an amount actively bonded less than"]
    #[doc =
          r" T::Currency::existential_deposit(), then it is increased to the full amount."]
    #[doc = r""]
    #[doc =
          r" Once the unlock period is done, you can call `withdraw_unbonded` to actually move"]
    #[doc = r" the funds out of management ready for transfer."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
    #[doc = r""]
    #[doc = r" See also [`Call::withdraw_unbonded`]."]
    fn unbond(origin: T::Origin, value: BalanceOf<T>)
     -> ::srml_support::dispatch::Result {
        {
            let controller = ensure_signed(origin)?;
            let mut ledger =
                Self::ledger(&controller).ok_or("not a controller")?;
            let mut value = value.min(ledger.active);
            if !value.is_zero() {
                ledger.active -= value;
                if ledger.active < T::Currency::minimum_balance() {
                    value += ledger.active;
                    ledger.active = Zero::zero();
                }
                let era = Self::current_era() + Self::bonding_duration();
                ledger.unlocking.push(UnlockChunk{value, era,});
                Self::update_ledger(&controller, &ledger);
            }
        }
        Ok(())
    }
    #[doc =
          r" Remove any unlocked chunks from the `unlocking` queue from our management."]
    #[doc = r""]
    #[doc =
          r" This essentially frees up that balance to be used by the stash account to do"]
    #[doc = r" whatever it wants."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
    #[doc = r""]
    #[doc = r" See also [`Call::unbond`]."]
    fn withdraw_unbonded(origin: T::Origin)
     -> ::srml_support::dispatch::Result {
        {
            let controller = ensure_signed(origin)?;
            let ledger = Self::ledger(&controller).ok_or("not a controller")?;
            let ledger = ledger.consolidate_unlocked(Self::current_era());
            Self::update_ledger(&controller, &ledger);
        }
        Ok(())
    }
    #[doc = r" Declare the desire to validate for the origin controller."]
    #[doc = r""]
    #[doc = r" Effects will be felt at the beginning of the next era."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
    fn validate(origin: T::Origin, prefs: ValidatorPrefs<BalanceOf<T>>)
     -> ::srml_support::dispatch::Result {
        {
            let controller = ensure_signed(origin)?;
            let ledger = Self::ledger(&controller).ok_or("not a controller")?;
            let stash = &ledger.stash;
            {
                if !(prefs.unstake_threshold <= MAX_UNSTAKE_THRESHOLD) {
                    { return Err("unstake threshold too large"); };
                }
            };
            <Nominators<T>>::remove(stash);
            <Validators<T>>::insert(stash, prefs);
        }
        Ok(())
    }
    #[doc =
          r" Declare the desire to nominate `targets` for the origin controller."]
    #[doc = r""]
    #[doc = r" Effects will be felt at the beginning of the next era."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
    fn nominate(origin: T::Origin,
                targets: Vec<<T::Lookup as StaticLookup>::Source>)
     -> ::srml_support::dispatch::Result {
        {
            let controller = ensure_signed(origin)?;
            let ledger = Self::ledger(&controller).ok_or("not a controller")?;
            let stash = &ledger.stash;
            {
                if !!targets.is_empty() {
                    { return Err("targets cannot be empty"); };
                }
            };
            let targets =
                targets.into_iter().take(MAX_NOMINATIONS).map(T::Lookup::lookup).collect::<result::Result<Vec<T::AccountId>,
                                                                                                          &'static str>>()?;
            <Validators<T>>::remove(stash);
            <Nominators<T>>::insert(stash, targets);
        }
        Ok(())
    }
    #[doc = r" Declare no desire to either validate or nominate."]
    #[doc = r""]
    #[doc = r" Effects will be felt at the beginning of the next era."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
    fn chill(origin: T::Origin) -> ::srml_support::dispatch::Result {
        {
            let controller = ensure_signed(origin)?;
            let ledger = Self::ledger(&controller).ok_or("not a controller")?;
            let stash = &ledger.stash;
            <Validators<T>>::remove(stash);
            <Nominators<T>>::remove(stash);
        }
        Ok(())
    }
    #[doc = r" (Re-)set the payment target for a controller."]
    #[doc = r""]
    #[doc = r" Effects will be felt at the beginning of the next era."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
    fn set_payee(origin: T::Origin, payee: RewardDestination)
     -> ::srml_support::dispatch::Result {
        {
            let controller = ensure_signed(origin)?;
            let ledger = Self::ledger(&controller).ok_or("not a controller")?;
            let stash = &ledger.stash;
            <Payee<T>>::insert(stash, payee);
        }
        Ok(())
    }
    #[doc = r" (Re-)set the payment target for a controller."]
    #[doc = r""]
    #[doc = r" Effects will be felt at the beginning of the next era."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the stash, not the controller."]
    fn set_controller(origin: T::Origin,
                      controller: <T::Lookup as StaticLookup>::Source)
     -> ::srml_support::dispatch::Result {
        {
            let stash = ensure_signed(origin)?;
            let old_controller = Self::bonded(&stash).ok_or("not a stash")?;
            let controller = T::Lookup::lookup(controller)?;
            if <Ledger<T>>::exists(&controller) {
                return Err("controller already paired")
            }
            if controller != old_controller {
                <Bonded<T>>::insert(&stash, &controller);
                if let Some(l) = <Ledger<T>>::take(&old_controller) {
                    <Ledger<T>>::insert(&controller, l)
                };
            }
        }
        Ok(())
    }
    #[doc = r" Set the number of sessions in an era."]
    fn set_sessions_per_era(new: T::BlockNumber)
     -> ::srml_support::dispatch::Result {
        { <NextSessionsPerEra<T>>::put(new); }
        Ok(())
    }
    #[doc = r" The length of the bonding duration in eras."]
    fn set_bonding_duration(new: T::BlockNumber)
     -> ::srml_support::dispatch::Result {
        { <BondingDuration<T>>::put(new); }
        Ok(())
    }
    #[doc = r" The ideal number of validators."]
    fn set_validator_count(new: u32) -> ::srml_support::dispatch::Result {
        { <ValidatorCount<T>>::put(new); }
        Ok(())
    }
    #[doc =
          r" Force there to be a new era. This also forces a new session immediately after."]
    #[doc =
          r" `apply_rewards` should be true for validators to get the session reward."]
    fn force_new_era(apply_rewards: bool) -> Result {
        Self::apply_force_new_era(apply_rewards)
    }
    #[doc = r" Set the offline slash grace period."]
    fn set_offline_slash_grace(new: u32) -> ::srml_support::dispatch::Result {
        { <OfflineSlashGrace<T>>::put(new); }
        Ok(())
    }
    #[doc = r" Set the validators who cannot be slashed (if any)."]
    fn set_invulnerables(validators: Vec<T::AccountId>)
     -> ::srml_support::dispatch::Result {
        { <Invulnerables<T>>::put(validators); }
        Ok(())
    }
}
pub enum Call<T: Trait> {

    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                  ::srml_support::dispatch::Never),

    #[allow(non_camel_case_types)]
    bond(<T::Lookup as StaticLookup>::Source,
         #[codec(compact)]
         BalanceOf<T>, RewardDestination),

    #[allow(non_camel_case_types)]
    bond_extra(
               #[codec(compact)]
               BalanceOf<T>),

    #[allow(non_camel_case_types)]
    unbond(
           #[codec(compact)]
           BalanceOf<T>),

    #[allow(non_camel_case_types)]
    #[doc =
          r" Remove any unlocked chunks from the `unlocking` queue from our management."]
    #[doc = r""]
    #[doc =
          r" This essentially frees up that balance to be used by the stash account to do"]
    #[doc = r" whatever it wants."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
    #[doc = r""]
    #[doc = r" See also [`Call::unbond`]."]
    withdraw_unbonded(),

    #[allow(non_camel_case_types)]
    validate(ValidatorPrefs<BalanceOf<T>>),

    #[allow(non_camel_case_types)]
    nominate(Vec<<T::Lookup as StaticLookup>::Source>),

    #[allow(non_camel_case_types)]
    #[doc = r" Declare no desire to either validate or nominate."]
    #[doc = r""]
    #[doc = r" Effects will be felt at the beginning of the next era."]
    #[doc = r""]
    #[doc =
          r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]
    chill(),

    #[allow(non_camel_case_types)]
    set_payee(RewardDestination),

    #[allow(non_camel_case_types)]
    set_controller(<T::Lookup as StaticLookup>::Source),

    #[allow(non_camel_case_types)]
    set_sessions_per_era(
                         #[codec(compact)]
                         T::BlockNumber),

    #[allow(non_camel_case_types)]
    set_bonding_duration(
                         #[codec(compact)]
                         T::BlockNumber),

    #[allow(non_camel_case_types)]
    set_validator_count(
                        #[codec(compact)]
                        u32),

    #[allow(non_camel_case_types)]
    force_new_era(bool),

    #[allow(non_camel_case_types)]
    set_offline_slash_grace(
                            #[codec(compact)]
                            u32),

    #[allow(non_camel_case_types)]
    set_invulnerables(Vec<T::AccountId>),
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
         ValidatorPrefs<BalanceOf<T>>: _parity_codec::Encode,
         ValidatorPrefs<BalanceOf<T>>: _parity_codec::Encode,
         Vec<<T::Lookup as StaticLookup>::Source>: _parity_codec::Encode,
         Vec<<T::Lookup as StaticLookup>::Source>: _parity_codec::Encode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Encode,
         Vec<T::AccountId>: _parity_codec::Encode,
         Vec<T::AccountId>: _parity_codec::Encode,
         BalanceOf<T>: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::BlockNumber: _parity_codec::HasCompact,
         T::BlockNumber: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::bond(ref aa, ref ba, ref ca) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        {
                            dest.push(&<<BalanceOf<T> as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      BalanceOf<T>>>::from(ba));
                        }
                        dest.push(ca);
                    }
                    Call::bond_extra(ref aa) => {
                        dest.push_byte(1usize as u8);
                        {
                            dest.push(&<<BalanceOf<T> as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      BalanceOf<T>>>::from(aa));
                        }
                    }
                    Call::unbond(ref aa) => {
                        dest.push_byte(2usize as u8);
                        {
                            dest.push(&<<BalanceOf<T> as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      BalanceOf<T>>>::from(aa));
                        }
                    }
                    Call::withdraw_unbonded() => {
                        dest.push_byte(3usize as u8);
                    }
                    Call::validate(ref aa) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                    }
                    Call::nominate(ref aa) => {
                        dest.push_byte(5usize as u8);
                        dest.push(aa);
                    }
                    Call::chill() => { dest.push_byte(6usize as u8); }
                    Call::set_payee(ref aa) => {
                        dest.push_byte(7usize as u8);
                        dest.push(aa);
                    }
                    Call::set_controller(ref aa) => {
                        dest.push_byte(8usize as u8);
                        dest.push(aa);
                    }
                    Call::set_sessions_per_era(ref aa) => {
                        dest.push_byte(9usize as u8);
                        {
                            dest.push(&<<T::BlockNumber as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::BlockNumber>>::from(aa));
                        }
                    }
                    Call::set_bonding_duration(ref aa) => {
                        dest.push_byte(10usize as u8);
                        {
                            dest.push(&<<T::BlockNumber as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::BlockNumber>>::from(aa));
                        }
                    }
                    Call::set_validator_count(ref aa) => {
                        dest.push_byte(11usize as u8);
                        {
                            dest.push(&<<u32 as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      u32>>::from(aa));
                        }
                    }
                    Call::force_new_era(ref aa) => {
                        dest.push_byte(12usize as u8);
                        dest.push(aa);
                    }
                    Call::set_offline_slash_grace(ref aa) => {
                        dest.push_byte(13usize as u8);
                        {
                            dest.push(&<<u32 as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      u32>>::from(aa));
                        }
                    }
                    Call::set_invulnerables(ref aa) => {
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
        impl <T: Trait> _parity_codec::Decode for Call<T> where
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         ValidatorPrefs<BalanceOf<T>>: _parity_codec::Decode,
         ValidatorPrefs<BalanceOf<T>>: _parity_codec::Decode,
         Vec<<T::Lookup as StaticLookup>::Source>: _parity_codec::Decode,
         Vec<<T::Lookup as StaticLookup>::Source>: _parity_codec::Decode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         <T::Lookup as StaticLookup>::Source: _parity_codec::Decode,
         Vec<T::AccountId>: _parity_codec::Decode,
         Vec<T::AccountId>: _parity_codec::Decode,
         BalanceOf<T>: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         BalanceOf<T>: _parity_codec::HasCompact,
         T::BlockNumber: _parity_codec::HasCompact,
         T::BlockNumber: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::bond(_parity_codec::Decode::decode(input)?,
                                        <<BalanceOf<T> as
                                         _parity_codec::HasCompact>::Type as
                                            _parity_codec::Decode>::decode(input)?.into(),
                                        _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::bond_extra(<<BalanceOf<T> as
                                               _parity_codec::HasCompact>::Type
                                                  as
                                                  _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 2usize as u8 => {
                        Some(Call::unbond(<<BalanceOf<T> as
                                           _parity_codec::HasCompact>::Type as
                                              _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 3usize as u8 => {
                        Some(Call::withdraw_unbonded())
                    }
                    x if x == 4usize as u8 => {
                        Some(Call::validate(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 5usize as u8 => {
                        Some(Call::nominate(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 6usize as u8 => { Some(Call::chill()) }
                    x if x == 7usize as u8 => {
                        Some(Call::set_payee(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 8usize as u8 => {
                        Some(Call::set_controller(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 9usize as u8 => {
                        Some(Call::set_sessions_per_era(<<T::BlockNumber as
                                                         _parity_codec::HasCompact>::Type
                                                            as
                                                            _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 10usize as u8 => {
                        Some(Call::set_bonding_duration(<<T::BlockNumber as
                                                         _parity_codec::HasCompact>::Type
                                                            as
                                                            _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 11usize as u8 => {
                        Some(Call::set_validator_count(<<u32 as
                                                        _parity_codec::HasCompact>::Type
                                                           as
                                                           _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 12usize as u8 => {
                        Some(Call::force_new_era(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 13usize as u8 => {
                        Some(Call::set_offline_slash_grace(<<u32 as
                                                            _parity_codec::HasCompact>::Type
                                                               as
                                                               _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 14usize as u8 => {
                        Some(Call::set_invulnerables(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::bond(ref controller, ref value, ref payee) =>
            Call::bond((*controller).clone(), (*value).clone(),
                       (*payee).clone()),
            Call::bond_extra(ref max_additional) =>
            Call::bond_extra((*max_additional).clone()),
            Call::unbond(ref value) => Call::unbond((*value).clone()),
            Call::withdraw_unbonded() => Call::withdraw_unbonded(),
            Call::validate(ref prefs) => Call::validate((*prefs).clone()),
            Call::nominate(ref targets) => Call::nominate((*targets).clone()),
            Call::chill() => Call::chill(),
            Call::set_payee(ref payee) => Call::set_payee((*payee).clone()),
            Call::set_controller(ref controller) =>
            Call::set_controller((*controller).clone()),
            Call::set_sessions_per_era(ref new) =>
            Call::set_sessions_per_era((*new).clone()),
            Call::set_bonding_duration(ref new) =>
            Call::set_bonding_duration((*new).clone()),
            Call::set_validator_count(ref new) =>
            Call::set_validator_count((*new).clone()),
            Call::force_new_era(ref apply_rewards) =>
            Call::force_new_era((*apply_rewards).clone()),
            Call::set_offline_slash_grace(ref new) =>
            Call::set_offline_slash_grace((*new).clone()),
            Call::set_invulnerables(ref validators) =>
            Call::set_invulnerables((*validators).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/staking/src/lib.rs",
                                             544u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::bond(ref controller, ref value, ref payee) => {
                let self_params = (controller, value, payee);
                if let Call::bond(ref controller, ref value, ref payee) =
                       *_other {
                    self_params == (controller, value, payee)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::bond_extra(ref max_additional) => {
                let self_params = (max_additional,);
                if let Call::bond_extra(ref max_additional) = *_other {
                    self_params == (max_additional,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::unbond(ref value) => {
                let self_params = (value,);
                if let Call::unbond(ref value) = *_other {
                    self_params == (value,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::withdraw_unbonded() => {
                let self_params = ();
                if let Call::withdraw_unbonded() = *_other {
                    self_params == ()
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::validate(ref prefs) => {
                let self_params = (prefs,);
                if let Call::validate(ref prefs) = *_other {
                    self_params == (prefs,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::nominate(ref targets) => {
                let self_params = (targets,);
                if let Call::nominate(ref targets) = *_other {
                    self_params == (targets,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::chill() => {
                let self_params = ();
                if let Call::chill() = *_other {
                    self_params == ()
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_payee(ref payee) => {
                let self_params = (payee,);
                if let Call::set_payee(ref payee) = *_other {
                    self_params == (payee,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_controller(ref controller) => {
                let self_params = (controller,);
                if let Call::set_controller(ref controller) = *_other {
                    self_params == (controller,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_sessions_per_era(ref new) => {
                let self_params = (new,);
                if let Call::set_sessions_per_era(ref new) = *_other {
                    self_params == (new,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_bonding_duration(ref new) => {
                let self_params = (new,);
                if let Call::set_bonding_duration(ref new) = *_other {
                    self_params == (new,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_validator_count(ref new) => {
                let self_params = (new,);
                if let Call::set_validator_count(ref new) = *_other {
                    self_params == (new,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::force_new_era(ref apply_rewards) => {
                let self_params = (apply_rewards,);
                if let Call::force_new_era(ref apply_rewards) = *_other {
                    self_params == (apply_rewards,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_offline_slash_grace(ref new) => {
                let self_params = (new,);
                if let Call::set_offline_slash_grace(ref new) = *_other {
                    self_params == (new,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_invulnerables(ref validators) => {
                let self_params = (validators,);
                if let Call::set_invulnerables(ref validators) = *_other {
                    self_params == (validators,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/staking/src/lib.rs",
                                             544u32, 1u32))
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
            Call::bond(ref controller, ref value, ref payee) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"bond",
                                                               &(controller.clone(),
                                                                 value.clone(),
                                                                 payee.clone()))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::bond_extra(ref max_additional) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"bond_extra",
                                                               &(max_additional.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::unbond(ref value) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"unbond",
                                                               &(value.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::withdraw_unbonded() =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"withdraw_unbonded",
                                                               &()) {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::validate(ref prefs) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"validate",
                                                               &(prefs.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::nominate(ref targets) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"nominate",
                                                               &(targets.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::chill() =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"chill", &())
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_payee(ref payee) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_payee",
                                                               &(payee.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_controller(ref controller) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_controller",
                                                               &(controller.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_sessions_per_era(ref new) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_sessions_per_era",
                                                               &(new.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_bonding_duration(ref new) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_bonding_duration",
                                                               &(new.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_validator_count(ref new) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_validator_count",
                                                               &(new.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::force_new_era(ref apply_rewards) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"force_new_era",
                                                               &(apply_rewards.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_offline_slash_grace(ref new) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_offline_slash_grace",
                                                               &(new.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_invulnerables(ref validators) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_invulnerables",
                                                               &(validators.clone(),))
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
                                           &("srml/staking/src/lib.rs",
                                             544u32, 1u32))
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
            Call::bond(controller, value, payee) => {
                <Module<T>>::bond(_origin, controller, value, payee)
            }
            Call::bond_extra(max_additional) => {
                <Module<T>>::bond_extra(_origin, max_additional)
            }
            Call::unbond(value) => { <Module<T>>::unbond(_origin, value) }
            Call::withdraw_unbonded() => {
                <Module<T>>::withdraw_unbonded(_origin)
            }
            Call::validate(prefs) => { <Module<T>>::validate(_origin, prefs) }
            Call::nominate(targets) => {
                <Module<T>>::nominate(_origin, targets)
            }
            Call::chill() => { <Module<T>>::chill(_origin) }
            Call::set_payee(payee) => {
                <Module<T>>::set_payee(_origin, payee)
            }
            Call::set_controller(controller) => {
                <Module<T>>::set_controller(_origin, controller)
            }
            Call::set_sessions_per_era(new) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_sessions_per_era(new)
                }
            }
            Call::set_bonding_duration(new) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_bonding_duration(new)
                }
            }
            Call::set_validator_count(new) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_validator_count(new)
                }
            }
            Call::force_new_era(apply_rewards) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::force_new_era(apply_rewards)
                }
            }
            Call::set_offline_slash_grace(new) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_offline_slash_grace(new)
                }
            }
            Call::set_invulnerables(validators) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_invulnerables(validators)
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
                                                       &("srml/staking/src/lib.rs",
                                                         544u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("bond"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("controller"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("value"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),},
                                                                                                             ::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("payee"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("RewardDestination"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Take the origin account as a stash and lock up `value` of its balance. `controller` will be the",
                                                                                                             r" account that controls it.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be _Signed_ by the stash account."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("bond_extra"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("max_additional"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Add some extra amount that have appeared in the stash `free_balance` into the balance up for",
                                                                                                             r" staking.",
                                                                                                             r"",
                                                                                                             r" Use this if there are additional funds in your stash account that you wish to bond.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be _Signed_ by the stash, not the controller."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("unbond"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("value"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<BalanceOf<T>>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Schedule a portion of the stash to be unlocked ready for transfer out after the bond",
                                                                                                             r" period ends. If this leaves an amount actively bonded less than",
                                                                                                             r" T::Currency::existential_deposit(), then it is increased to the full amount.",
                                                                                                             r"",
                                                                                                             r" Once the unlock period is done, you can call `withdraw_unbonded` to actually move",
                                                                                                             r" the funds out of management ready for transfer.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be _Signed_ by the controller, not the stash.",
                                                                                                             r"",
                                                                                                             r" See also [`Call::withdraw_unbonded`]."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("withdraw_unbonded"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Remove any unlocked chunks from the `unlocking` queue from our management.",
                                                                                                             r"",
                                                                                                             r" This essentially frees up that balance to be used by the stash account to do",
                                                                                                             r" whatever it wants.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be _Signed_ by the controller, not the stash.",
                                                                                                             r"",
                                                                                                             r" See also [`Call::unbond`]."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("validate"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("prefs"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("ValidatorPrefs<BalanceOf<T>>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Declare the desire to validate for the origin controller.",
                                                                                                             r"",
                                                                                                             r" Effects will be felt at the beginning of the next era.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("nominate"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("targets"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<<T::Lookup as StaticLookup>::Source>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Declare the desire to nominate `targets` for the origin controller.",
                                                                                                             r"",
                                                                                                             r" Effects will be felt at the beginning of the next era.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("chill"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Declare no desire to either validate or nominate.",
                                                                                                             r"",
                                                                                                             r" Effects will be felt at the beginning of the next era.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_payee"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("payee"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("RewardDestination"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" (Re-)set the payment target for a controller.",
                                                                                                             r"",
                                                                                                             r" Effects will be felt at the beginning of the next era.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be _Signed_ by the controller, not the stash."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_controller"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("controller"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("<T::Lookup as StaticLookup>::Source"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" (Re-)set the payment target for a controller.",
                                                                                                             r"",
                                                                                                             r" Effects will be felt at the beginning of the next era.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be _Signed_ by the stash, not the controller."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_sessions_per_era"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("new"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the number of sessions in an era."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_bonding_duration"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("new"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" The length of the bonding duration in eras."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_validator_count"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("new"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<u32>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" The ideal number of validators."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("force_new_era"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("apply_rewards"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("bool"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Force there to be a new era. This also forces a new session immediately after.",
                                                                                                             r" `apply_rewards` should be true for validators to get the session reward."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_offline_slash_grace"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("new"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<u32>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the offline slash grace period."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_invulnerables"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("validators"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<T::AccountId>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the validators who cannot be slashed (if any)."]),}]
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

    #[doc = r" All validators have been rewarded by the given balance."]
    Reward(Balance),

    #[doc =
          r" One validator (and its nominators) has been given an offline-warning (it is still"]
    #[doc =
          r" within its grace). The accrued number of slashes is recorded, too."]
    OfflineWarning(AccountId, u32),

    #[doc =
          r" One validator (and its nominators) has been slashed by the given amount."]
    OfflineSlash(AccountId, Balance),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <Balance: ::std::clone::Clone, AccountId: ::std::clone::Clone>
 ::std::clone::Clone for RawEvent<Balance, AccountId> {
    #[inline]
    fn clone(&self) -> RawEvent<Balance, AccountId> {
        match (&*self,) {
            (&RawEvent::Reward(ref __self_0),) =>
            RawEvent::Reward(::std::clone::Clone::clone(&(*__self_0))),
            (&RawEvent::OfflineWarning(ref __self_0, ref __self_1),) =>
            RawEvent::OfflineWarning(::std::clone::Clone::clone(&(*__self_0)),
                                     ::std::clone::Clone::clone(&(*__self_1))),
            (&RawEvent::OfflineSlash(ref __self_0, ref __self_1),) =>
            RawEvent::OfflineSlash(::std::clone::Clone::clone(&(*__self_0)),
                                   ::std::clone::Clone::clone(&(*__self_1))),
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
                    (&RawEvent::Reward(ref __self_0),
                     &RawEvent::Reward(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RawEvent::OfflineWarning(ref __self_0, ref __self_1),
                     &RawEvent::OfflineWarning(ref __arg_1_0, ref __arg_1_1))
                    =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    (&RawEvent::OfflineSlash(ref __self_0, ref __self_1),
                     &RawEvent::OfflineSlash(ref __arg_1_0, ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
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
                    (&RawEvent::Reward(ref __self_0),
                     &RawEvent::Reward(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RawEvent::OfflineWarning(ref __self_0, ref __self_1),
                     &RawEvent::OfflineWarning(ref __arg_1_0, ref __arg_1_1))
                    =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    (&RawEvent::OfflineSlash(ref __self_0, ref __self_1),
                     &RawEvent::OfflineSlash(ref __arg_1_0, ref __arg_1_1)) =>
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
impl <Balance: ::std::cmp::Eq, AccountId: ::std::cmp::Eq> ::std::cmp::Eq for
 RawEvent<Balance, AccountId> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<Balance>;
            let _: ::std::cmp::AssertParamIsEq<AccountId>;
            let _: ::std::cmp::AssertParamIsEq<u32>;
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
        impl <Balance, AccountId> _parity_codec::Encode for
         RawEvent<Balance, AccountId> where Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, AccountId: _parity_codec::Encode,
         AccountId: _parity_codec::Encode, Balance: _parity_codec::Encode,
         Balance: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::Reward(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    RawEvent::OfflineWarning(ref aa, ref ba) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawEvent::OfflineSlash(ref aa, ref ba) => {
                        dest.push_byte(2usize as u8);
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
        impl <Balance, AccountId> _parity_codec::Decode for
         RawEvent<Balance, AccountId> where Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, AccountId: _parity_codec::Decode,
         AccountId: _parity_codec::Decode, Balance: _parity_codec::Decode,
         Balance: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::Reward(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(RawEvent::OfflineWarning(_parity_codec::Decode::decode(input)?,
                                                      _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(RawEvent::OfflineSlash(_parity_codec::Decode::decode(input)?,
                                                    _parity_codec::Decode::decode(input)?))
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
            (&RawEvent::Reward(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Reward");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RawEvent::OfflineWarning(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("OfflineWarning");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
            (&RawEvent::OfflineSlash(ref __self_0, ref __self_1),) => {
                let mut debug_trait_builder = f.debug_tuple("OfflineSlash");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
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
                                                   ::srml_support::event::DecodeDifferent::Encode("Reward"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" All validators have been rewarded by the given balance."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("OfflineWarning"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "u32"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" One validator (and its nominators) has been given an offline-warning (it is still",
                                                                                                    r" within its grace). The accrued number of slashes is recorded, too."]),},
          ::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("OfflineSlash"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["AccountId",
                                                                                                    "Balance"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" One validator (and its nominators) has been slashed by the given amount."]),}]
    }
}
impl <T: Trait> Module<T> {
    /// Just force_new_era without origin check.
    fn apply_force_new_era(apply_rewards: bool) -> Result {
        <ForcingNewEra<T>>::put(());
        <session::Module<T>>::apply_force_new_session(apply_rewards)
    }
    /// The length of a staking era in blocks.
    pub fn era_length() -> T::BlockNumber {
        Self::sessions_per_era() * <session::Module<T>>::length()
    }
    /// The total balance that can be slashed from a validator controller account as of
    /// right now.
    pub fn slashable_balance(who: &T::AccountId) -> BalanceOf<T> {
        Self::stakers(who).total
    }
    /// Update the ledger for a controller. This will also update the stash lock.
    fn update_ledger(controller: &T::AccountId,
                     ledger:
                         &StakingLedger<T::AccountId, BalanceOf<T>,
                                        T::BlockNumber>) {
        T::Currency::set_lock(STAKING_ID, &ledger.stash, ledger.total,
                              T::BlockNumber::max_value(),
                              WithdrawReasons::all());
        <Ledger<T>>::insert(controller, ledger);
    }
    /// Slash a given validator by a specific amount. Removes the slash from the validator's balance by preference,
    /// and reduces the nominators' balance if needed.
    fn slash_validator(stash: &T::AccountId, slash: BalanceOf<T>) {
        let exposure = Self::stakers(stash);
        let slash = slash.min(exposure.total);
        let own_slash = exposure.own.min(slash);
        let (mut imbalance, missing) = T::Currency::slash(stash, own_slash);
        let own_slash = own_slash - missing;
        let rest_slash = slash - own_slash;
        if !rest_slash.is_zero() {
            let total = exposure.total - exposure.own;
            if !total.is_zero() {
                let safe_mul_rational = |b| b * rest_slash / total;
                for i in exposure.others.iter() {
                    imbalance.subsume(T::Currency::slash(&i.who,
                                                         safe_mul_rational(i.value)).0)
                }
            }
        }
        T::Slash::on_unbalanced(imbalance);
    }
    /// Actually make a payment to a staker. This uses the currency's reward function
    /// to pay the right payee for the given staker account.
    fn make_payout(stash: &T::AccountId, amount: BalanceOf<T>)
     -> Option<PositiveImbalanceOf<T>> {
        let dest = Self::payee(stash);
        match dest {
            RewardDestination::Controller =>
            Self::bonded(stash).and_then(|controller|
                                             T::Currency::deposit_into_existing(&controller,
                                                                                amount).ok()),
            RewardDestination::Stash =>
            T::Currency::deposit_into_existing(stash, amount).ok(),
            RewardDestination::Staked =>
            Self::bonded(stash).and_then(|c|
                                             Self::ledger(&c).map(|l|
                                                                      (c,
                                                                       l))).and_then(|(controller,
                                                                                       mut l)|
                                                                                         {
                                                                                             l.active
                                                                                                 +=
                                                                                                 amount;
                                                                                             l.total
                                                                                                 +=
                                                                                                 amount;
                                                                                             let r =
                                                                                                 T::Currency::deposit_into_existing(stash,
                                                                                                                                    amount).ok();
                                                                                             Self::update_ledger(&controller,
                                                                                                                 &l);
                                                                                             r
                                                                                         }),
        }
    }
    /// Reward a given validator by a specific amount. Add the reward to the validator's, and its nominators'
    /// balance, pro-rata based on their exposure, after having removed the validator's pre-payout cut.
    fn reward_validator(stash: &T::AccountId, reward: BalanceOf<T>) {
        let off_the_table =
            reward.min(Self::validators(stash).validator_payment);
        let reward = reward - off_the_table;
        let mut imbalance = <PositiveImbalanceOf<T>>::zero();
        let validator_cut =
            if reward.is_zero() {
                Zero::zero()
            } else {
                let exposure = Self::stakers(stash);
                let total = exposure.total.max(One::one());
                let safe_mul_rational = |b| b * reward / total;
                for i in &exposure.others {
                    let nom_payout = safe_mul_rational(i.value);
                    imbalance.maybe_subsume(Self::make_payout(&i.who,
                                                              nom_payout));
                }
                safe_mul_rational(exposure.own)
            };
        imbalance.maybe_subsume(Self::make_payout(stash,
                                                  validator_cut +
                                                      off_the_table));
        T::Reward::on_unbalanced(imbalance);
    }
    /// Get the reward for the session, assuming it ends with this block.
    fn this_session_reward(actual_elapsed: T::Moment) -> BalanceOf<T> {
        let ideal_elapsed = <session::Module<T>>::ideal_session_duration();
        if ideal_elapsed.is_zero() { return Self::current_session_reward(); }
        let per65536: u64 =
            (T::Moment::sa(65536u64) * ideal_elapsed.clone() /
                 actual_elapsed.max(ideal_elapsed)).as_();
        Self::current_session_reward() * <BalanceOf<T>>::sa(per65536) /
            <BalanceOf<T>>::sa(65536u64)
    }
    /// Session has just changed. We need to determine whether we pay a reward, slash and/or
    /// move to a new era.
    fn new_session(actual_elapsed: T::Moment, should_reward: bool) {
        if should_reward {
            let reward = Self::this_session_reward(actual_elapsed);
            <CurrentEraReward<T>>::mutate(|r| *r += reward);
        }
        let session_index = <session::Module<T>>::current_index();
        if <ForcingNewEra<T>>::take().is_some() ||
               ((session_index - Self::last_era_length_change()) %
                    Self::sessions_per_era()).is_zero() {
            Self::new_era();
        }
    }
    /// The era has changed - enact new staking set.
    ///
    /// NOTE: This always happens immediately before a session change to ensure that new validators
    /// get a chance to set their session keys.
    fn new_era() {
        let reward = <CurrentEraReward<T>>::take();
        if !reward.is_zero() {
            let validators = Self::current_elected();
            for v in validators.iter() { Self::reward_validator(v, reward); }
            Self::deposit_event(RawEvent::Reward(reward));
            let len = validators.len() as u64;
            let len = BalanceOf::<T>::sa(len);
            let total_minted = reward * len;
            let total_rewarded_stake = Self::slot_stake() * len;
            T::OnRewardMinted::on_dilution(total_minted,
                                           total_rewarded_stake);
        }
        <CurrentEra<T>>::put(&(<CurrentEra<T>>::get() + One::one()));
        if let Some(next_spe) = Self::next_sessions_per_era() {
            if next_spe != Self::sessions_per_era() {
                <SessionsPerEra<T>>::put(&next_spe);
                <LastEraLengthChange<T>>::put(&<session::Module<T>>::current_index());
            }
        }
        let slot_stake = Self::select_validators();
        <CurrentSessionReward<T>>::put(Self::session_reward() * slot_stake);
    }
    fn slashable_balance_of(stash: &T::AccountId) -> BalanceOf<T> {
        Self::bonded(stash).and_then(Self::ledger).map(|l|
                                                           l.total).unwrap_or_default()
    }
    /// Select a new validator set from the assembled stakers and their role preferences.
    ///
    /// Returns the new `SlotStake` value.
    fn select_validators() -> BalanceOf<T> {
        let maybe_elected_set =
            elect::<T, _, _,
                    _>(Self::validator_count() as usize,
                       Self::minimum_validator_count().max(1) as usize,
                       <Validators<T>>::enumerate(),
                       <Nominators<T>>::enumerate(),
                       Self::slashable_balance_of);
        if let Some(elected_set) = maybe_elected_set {
            let elected_stashes = elected_set.0;
            let assignments = elected_set.1;
            let to_balance =
                |b: ExtendedBalance|
                    <T::CurrencyToVote as
                        Convert<ExtendedBalance, BalanceOf<T>>>::convert(b);
            let to_votes =
                |b: BalanceOf<T>|
                    <T::CurrencyToVote as
                        Convert<BalanceOf<T>, u64>>::convert(b) as
                        ExtendedBalance;
            let ratio_of =
                |b, p|
                    (p as ExtendedBalance).saturating_mul(to_votes(b)) /
                        ACCURACY;
            let mut assignments_with_stakes =
                assignments.iter().map(|(n, a)|
                                           (n.clone(),
                                            Self::slashable_balance_of(n),
                                            a.iter().map(|(acc, r)|
                                                             (acc.clone(), *r,
                                                              to_balance(ratio_of(Self::slashable_balance_of(n),
                                                                                  *r)))).collect::<Vec<Assignment<T>>>())).collect::<Vec<(T::AccountId,
                                                                                                                                          BalanceOf<T>,
                                                                                                                                          Vec<Assignment<T>>)>>();
            let mut exposures = <ExpoMap<T>>::new();
            elected_stashes.iter().map(|e|
                                           (e,
                                            Self::slashable_balance_of(e))).for_each(|(e,
                                                                                       s)|
                                                                                         {
                                                                                             exposures.insert(e.clone(),
                                                                                                              Exposure{own:
                                                                                                                           s,
                                                                                                                       total:
                                                                                                                           s,
                                                                                                                                ..Default::default()});
                                                                                         });
            for (n, _, assignment) in &assignments_with_stakes {
                for (c, _, s) in assignment {
                    if let Some(expo) = exposures.get_mut(c) {
                        expo.total = expo.total.saturating_add(*s);
                        expo.others.push(IndividualExposure{who: n.clone(),
                                                            value: *s,});
                    }
                }
            }
            let do_equalise = false;
            if do_equalise {
                let tolerance = 10 as u128;
                let iterations = 10 as usize;
                phragmen::equalize::<T>(&mut assignments_with_stakes,
                                        &mut exposures, tolerance,
                                        iterations);
            }
            for v in Self::current_elected().iter() {
                <Stakers<T>>::remove(v);
                let slash_count = <SlashCount<T>>::take(v);
                if slash_count > 1 {
                    <SlashCount<T>>::insert(v, slash_count - 1);
                }
            }
            let mut slot_stake = BalanceOf::<T>::max_value();
            for (c, e) in exposures.iter() {
                if e.total < slot_stake { slot_stake = e.total; }
                <Stakers<T>>::insert(c.clone(), e.clone());
            }
            <SlotStake<T>>::put(&slot_stake);
            <CurrentElected<T>>::put(&elected_stashes);
            <session::Module<T>>::set_validators(&elected_stashes.into_iter().map(|s|
                                                                                      Self::bonded(s).unwrap_or_default()).collect::<Vec<_>>());
            slot_stake
        } else { Self::slot_stake() }
    }
    /// Call when a validator is determined to be offline. `count` is the
    /// number of offenses the validator has committed.
    ///
    /// NOTE: This is called with the controller (not the stash) account id.
    pub fn on_offline_validator(controller: T::AccountId, count: usize) {
        if let Some(l) = Self::ledger(&controller) {
            let stash = l.stash;
            if Self::invulnerables().contains(&stash) { return }
            let slash_count = Self::slash_count(&stash);
            let new_slash_count = slash_count + count as u32;
            <SlashCount<T>>::insert(&stash, new_slash_count);
            let grace = Self::offline_slash_grace();
            if RECENT_OFFLINE_COUNT > 0 {
                let item =
                    (stash.clone(), <system::Module<T>>::block_number(),
                     count as u32);
                <RecentlyOffline<T>>::mutate(|v|
                                                 if v.len() >=
                                                        RECENT_OFFLINE_COUNT {
                                                     let index =
                                                         v.iter().enumerate().min_by_key(|(_,
                                                                                           (_,
                                                                                            block,
                                                                                            _))|
                                                                                             block).expect("v is non-empty; qed").0;
                                                     v[index] = item;
                                                 } else { v.push(item); });
            }
            let prefs = Self::validators(&stash);
            let unstake_threshold =
                prefs.unstake_threshold.min(MAX_UNSTAKE_THRESHOLD);
            let max_slashes = grace + unstake_threshold;
            let event =
                if new_slash_count > max_slashes {
                    let slash_exposure = Self::stakers(&stash).total;
                    let offline_slash_base =
                        Self::offline_slash() * slash_exposure;
                    let slash =
                        offline_slash_base.checked_shl(unstake_threshold).map(|x|
                                                                                  x.min(slash_exposure)).unwrap_or(slash_exposure);
                    let _ = Self::slash_validator(&stash, slash);
                    <Validators<T>>::remove(&stash);
                    let _ = Self::apply_force_new_era(false);
                    RawEvent::OfflineSlash(stash.clone(), slash)
                } else {
                    RawEvent::OfflineWarning(stash.clone(), slash_count)
                };
            Self::deposit_event(event);
        }
    }
}
impl <T: Trait> OnSessionChange<T::Moment> for Module<T> {
    fn on_session_change(elapsed: T::Moment, should_reward: bool) {
        Self::new_session(elapsed, should_reward);
    }
}
impl <T: Trait> OnFreeBalanceZero<T::AccountId> for Module<T> {
    fn on_free_balance_zero(stash: &T::AccountId) {
        if let Some(controller) = <Bonded<T>>::take(stash) {
            <Ledger<T>>::remove(&controller);
        }
        <Payee<T>>::remove(stash);
        <SlashCount<T>>::remove(stash);
        <Validators<T>>::remove(stash);
        <Nominators<T>>::remove(stash);
    }
}
impl <T: Trait> consensus::OnOfflineReport<Vec<u32>> for Module<T> {
    fn handle_report(reported_indices: Vec<u32>) {
        for validator_index in reported_indices {
            let v =
                <session::Module<T>>::validators()[validator_index as
                                                       usize].clone();
            Self::on_offline_validator(v, 1);
        }
    }
}
