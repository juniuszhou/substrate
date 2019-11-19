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

//! # Session Module
//!
//! The Session module allows validators to manage their session keys, provides a function for changing
//! the session length, and handles session rotation.
//!
//! - [`session::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! ### Terminology
//! <!-- Original author of paragraph: @gavofyork -->
//!
//! - **Session:** A session is a period of time that has a constant set of validators. Validators can only join
//! or exit the validator set at a session change. It is measured in block numbers and set with `set_length`
//! during a session for use in subsequent sessions.
//! - **Session key:** A session key is actually several keys kept together that provide the various signing
//! functions required by network authorities/validators in pursuit of their duties.
//! - **Session key configuration process:** A session key is set using `set_key` for use in the
//! next session. It is stored in `NextKeyFor`, a mapping between the caller's `AccountID` and the session
//! key provided. `set_key` allows users to set their session key prior to becoming a validator.
//! It is a public call since it uses `ensure_signed`, which checks that the origin is a signed account.
//! As such, the account ID of the origin stored in in `NextKeyFor` may not necessarily be associated with
//! a block author or a validator. The session keys of accounts are removed once their account balance is zero.
//! - **Validator set session key configuration process:** Each session we iterate through the current
//! set of validator account IDs to check if a session key was created for it in the previous session
//! using `set_key`. If it was then we call `set_authority` from the [Consensus module](../srml_consensus/index.html)
//! and pass it a set of session keys (each associated with an account ID) as the session keys for the new
//! validator set. Lastly, if the session key of the current authority does not match any session keys stored under
//! its validator index in the `AuthorityStorageVec` mapping, then we update the mapping with its session
//! key and update the saved list of original authorities if necessary
//! (see https://github.com/paritytech/substrate/issues/1290). Note: Authorities are stored in the Consensus module.
//! They are represented by a validator account ID index from the Session module and allocated with a session
//! key for the length of the session.
//! - **Session length change process:** At the start of the next session we allocate a session index and record the
//! timestamp when the session started. If a `NextSessionLength` was recorded in the previous session, we record
//! it as the new session length. Additionally, if the new session length differs from the length of the
//! next session then we record a `LastLengthChange`.
//! - **Session rotation configuration:** Configure as either a 'normal' (rewardable session where rewards are
//! applied) or 'exceptional' (slashable) session rotation.
//! - **Session rotation process:** The session is changed at the end of the final block of the current session
//! using the `on_finalize` method. It may be called by either an origin or internally from another runtime
//! module at the end of each block.
//!
//! ### Goals
//!
//! The Session module in Substrate is designed to make the following possible:
//!
//! - Set session keys of the validator set for the next session.
//! - Set the length of a session.
//! - Configure and switch between either normal or exceptional session rotations.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `set_key` - Set a validator's session key for the next session.
//! - `set_length` - Set a new session length to be applied upon the next session change.
//! - `force_new_session` - Force a new session that should be considered either a normal (rewardable)
//! or exceptional rotation.
//! - `on_finalize` - Called when a block is finalized. Will rotate session if it is the last
//! block of the current session.
//!
//! ### Public Functions
//!
//! - `validator_count` - Get the current number of validators.
//! - `last_length_change` - Get the block number when the session length last changed.
//! - `apply_force_new_session` - Force a new session. Can be called by other runtime modules.
//! - `set_validators` - Set the current set of validators. Can only be called by the Staking module.
//! - `check_rotate_session` - Rotate the session and apply rewards if necessary. Called after the Staking
//! module updates the authorities to the new validator set.
//! - `rotate_session` - Change to the next session. Register the new authority set. Update session keys.
//! Enact session length change if applicable.
//! - `ideal_session_duration` - Get the time of an ideal session.
//! - `blocks_remaining` - Get the number of blocks remaining in the current session,
//! excluding the current block.
//!
//! ## Usage
//!
//! ### Example from the SRML
//!
//! The [Staking module](../srml_staking/index.html) uses the Session module to get the validator set.
//!
//! ```
//! use srml_session as session;
//! # fn not_executed<T: session::Trait>() {
//!
//! let validators = <session::Module<T>>::validators();
//! # }
//! # fn main(){}
//! ```
//!
//! ## Related Modules
//!
//! - [Consensus](../srml_consensus/index.html)
//! - [Staking](../srml_staking/index.html)
//! - [Timestamp](../srml_timestamp/index.html)
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


use rstd::prelude::*;
use primitives::traits::{As, Zero, One, Convert};
use srml_support::{StorageValue, StorageMap, for_each_tuple, decl_module,
                   decl_event, decl_storage};
use srml_support::{dispatch::Result, traits::OnFreeBalanceZero};
use system::ensure_signed;
use rstd::ops::Mul;

/// A session has changed.
pub trait OnSessionChange<T> {
    /// Session has changed.
    fn on_session_change(time_elapsed: T, should_reward: bool);
}

macro_rules! impl_session_change((  ) => (
                                 impl < T > OnSessionChange < T > for (  ) {
                                 fn on_session_change ( _ : T , _ : bool ) {
                                 } } ) ; ( $ ( $ t : ident ) * ) => {
                                 impl < T : Clone , $ (
                                 $ t : OnSessionChange < T > ) , * >
                                 OnSessionChange < T > for ( $ ( $ t , ) * ) {
                                 fn on_session_change (
                                 time_elapsed : T , should_reward : bool ) {
                                 $ (
                                 $ t :: on_session_change (
                                 time_elapsed . clone (  ) , should_reward ) ;
                                 ) * } } });

impl <T: Clone, A: OnSessionChange<T>, B: OnSessionChange<T>,
      C: OnSessionChange<T>, D: OnSessionChange<T>, E: OnSessionChange<T>,
      F: OnSessionChange<T>, G: OnSessionChange<T>, H: OnSessionChange<T>,
      I: OnSessionChange<T>, J: OnSessionChange<T>, K: OnSessionChange<T>,
      L: OnSessionChange<T>, M: OnSessionChange<T>, N: OnSessionChange<T>,
      O: OnSessionChange<T>, P: OnSessionChange<T>, Q: OnSessionChange<T>,
      R: OnSessionChange<T>, S: OnSessionChange<T>> OnSessionChange<T> for
 (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        A::on_session_change(time_elapsed.clone(), should_reward);
        B::on_session_change(time_elapsed.clone(), should_reward);
        C::on_session_change(time_elapsed.clone(), should_reward);
        D::on_session_change(time_elapsed.clone(), should_reward);
        E::on_session_change(time_elapsed.clone(), should_reward);
        F::on_session_change(time_elapsed.clone(), should_reward);
        G::on_session_change(time_elapsed.clone(), should_reward);
        H::on_session_change(time_elapsed.clone(), should_reward);
        I::on_session_change(time_elapsed.clone(), should_reward);
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, B: OnSessionChange<T>, C: OnSessionChange<T>,
      D: OnSessionChange<T>, E: OnSessionChange<T>, F: OnSessionChange<T>,
      G: OnSessionChange<T>, H: OnSessionChange<T>, I: OnSessionChange<T>,
      J: OnSessionChange<T>, K: OnSessionChange<T>, L: OnSessionChange<T>,
      M: OnSessionChange<T>, N: OnSessionChange<T>, O: OnSessionChange<T>,
      P: OnSessionChange<T>, Q: OnSessionChange<T>, R: OnSessionChange<T>,
      S: OnSessionChange<T>> OnSessionChange<T> for
 (B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        B::on_session_change(time_elapsed.clone(), should_reward);
        C::on_session_change(time_elapsed.clone(), should_reward);
        D::on_session_change(time_elapsed.clone(), should_reward);
        E::on_session_change(time_elapsed.clone(), should_reward);
        F::on_session_change(time_elapsed.clone(), should_reward);
        G::on_session_change(time_elapsed.clone(), should_reward);
        H::on_session_change(time_elapsed.clone(), should_reward);
        I::on_session_change(time_elapsed.clone(), should_reward);
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, C: OnSessionChange<T>, D: OnSessionChange<T>,
      E: OnSessionChange<T>, F: OnSessionChange<T>, G: OnSessionChange<T>,
      H: OnSessionChange<T>, I: OnSessionChange<T>, J: OnSessionChange<T>,
      K: OnSessionChange<T>, L: OnSessionChange<T>, M: OnSessionChange<T>,
      N: OnSessionChange<T>, O: OnSessionChange<T>, P: OnSessionChange<T>,
      Q: OnSessionChange<T>, R: OnSessionChange<T>, S: OnSessionChange<T>>
 OnSessionChange<T> for (C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        C::on_session_change(time_elapsed.clone(), should_reward);
        D::on_session_change(time_elapsed.clone(), should_reward);
        E::on_session_change(time_elapsed.clone(), should_reward);
        F::on_session_change(time_elapsed.clone(), should_reward);
        G::on_session_change(time_elapsed.clone(), should_reward);
        H::on_session_change(time_elapsed.clone(), should_reward);
        I::on_session_change(time_elapsed.clone(), should_reward);
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, D: OnSessionChange<T>, E: OnSessionChange<T>,
      F: OnSessionChange<T>, G: OnSessionChange<T>, H: OnSessionChange<T>,
      I: OnSessionChange<T>, J: OnSessionChange<T>, K: OnSessionChange<T>,
      L: OnSessionChange<T>, M: OnSessionChange<T>, N: OnSessionChange<T>,
      O: OnSessionChange<T>, P: OnSessionChange<T>, Q: OnSessionChange<T>,
      R: OnSessionChange<T>, S: OnSessionChange<T>> OnSessionChange<T> for
 (D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        D::on_session_change(time_elapsed.clone(), should_reward);
        E::on_session_change(time_elapsed.clone(), should_reward);
        F::on_session_change(time_elapsed.clone(), should_reward);
        G::on_session_change(time_elapsed.clone(), should_reward);
        H::on_session_change(time_elapsed.clone(), should_reward);
        I::on_session_change(time_elapsed.clone(), should_reward);
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, E: OnSessionChange<T>, F: OnSessionChange<T>,
      G: OnSessionChange<T>, H: OnSessionChange<T>, I: OnSessionChange<T>,
      J: OnSessionChange<T>, K: OnSessionChange<T>, L: OnSessionChange<T>,
      M: OnSessionChange<T>, N: OnSessionChange<T>, O: OnSessionChange<T>,
      P: OnSessionChange<T>, Q: OnSessionChange<T>, R: OnSessionChange<T>,
      S: OnSessionChange<T>> OnSessionChange<T> for
 (E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        E::on_session_change(time_elapsed.clone(), should_reward);
        F::on_session_change(time_elapsed.clone(), should_reward);
        G::on_session_change(time_elapsed.clone(), should_reward);
        H::on_session_change(time_elapsed.clone(), should_reward);
        I::on_session_change(time_elapsed.clone(), should_reward);
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, F: OnSessionChange<T>, G: OnSessionChange<T>,
      H: OnSessionChange<T>, I: OnSessionChange<T>, J: OnSessionChange<T>,
      K: OnSessionChange<T>, L: OnSessionChange<T>, M: OnSessionChange<T>,
      N: OnSessionChange<T>, O: OnSessionChange<T>, P: OnSessionChange<T>,
      Q: OnSessionChange<T>, R: OnSessionChange<T>, S: OnSessionChange<T>>
 OnSessionChange<T> for (F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        F::on_session_change(time_elapsed.clone(), should_reward);
        G::on_session_change(time_elapsed.clone(), should_reward);
        H::on_session_change(time_elapsed.clone(), should_reward);
        I::on_session_change(time_elapsed.clone(), should_reward);
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, G: OnSessionChange<T>, H: OnSessionChange<T>,
      I: OnSessionChange<T>, J: OnSessionChange<T>, K: OnSessionChange<T>,
      L: OnSessionChange<T>, M: OnSessionChange<T>, N: OnSessionChange<T>,
      O: OnSessionChange<T>, P: OnSessionChange<T>, Q: OnSessionChange<T>,
      R: OnSessionChange<T>, S: OnSessionChange<T>> OnSessionChange<T> for
 (G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        G::on_session_change(time_elapsed.clone(), should_reward);
        H::on_session_change(time_elapsed.clone(), should_reward);
        I::on_session_change(time_elapsed.clone(), should_reward);
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, H: OnSessionChange<T>, I: OnSessionChange<T>,
      J: OnSessionChange<T>, K: OnSessionChange<T>, L: OnSessionChange<T>,
      M: OnSessionChange<T>, N: OnSessionChange<T>, O: OnSessionChange<T>,
      P: OnSessionChange<T>, Q: OnSessionChange<T>, R: OnSessionChange<T>,
      S: OnSessionChange<T>> OnSessionChange<T> for
 (H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        H::on_session_change(time_elapsed.clone(), should_reward);
        I::on_session_change(time_elapsed.clone(), should_reward);
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, I: OnSessionChange<T>, J: OnSessionChange<T>,
      K: OnSessionChange<T>, L: OnSessionChange<T>, M: OnSessionChange<T>,
      N: OnSessionChange<T>, O: OnSessionChange<T>, P: OnSessionChange<T>,
      Q: OnSessionChange<T>, R: OnSessionChange<T>, S: OnSessionChange<T>>
 OnSessionChange<T> for (I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        I::on_session_change(time_elapsed.clone(), should_reward);
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, J: OnSessionChange<T>, K: OnSessionChange<T>,
      L: OnSessionChange<T>, M: OnSessionChange<T>, N: OnSessionChange<T>,
      O: OnSessionChange<T>, P: OnSessionChange<T>, Q: OnSessionChange<T>,
      R: OnSessionChange<T>, S: OnSessionChange<T>> OnSessionChange<T> for
 (J, K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        J::on_session_change(time_elapsed.clone(), should_reward);
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, K: OnSessionChange<T>, L: OnSessionChange<T>,
      M: OnSessionChange<T>, N: OnSessionChange<T>, O: OnSessionChange<T>,
      P: OnSessionChange<T>, Q: OnSessionChange<T>, R: OnSessionChange<T>,
      S: OnSessionChange<T>> OnSessionChange<T> for
 (K, L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        K::on_session_change(time_elapsed.clone(), should_reward);
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, L: OnSessionChange<T>, M: OnSessionChange<T>,
      N: OnSessionChange<T>, O: OnSessionChange<T>, P: OnSessionChange<T>,
      Q: OnSessionChange<T>, R: OnSessionChange<T>, S: OnSessionChange<T>>
 OnSessionChange<T> for (L, M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        L::on_session_change(time_elapsed.clone(), should_reward);
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, M: OnSessionChange<T>, N: OnSessionChange<T>,
      O: OnSessionChange<T>, P: OnSessionChange<T>, Q: OnSessionChange<T>,
      R: OnSessionChange<T>, S: OnSessionChange<T>> OnSessionChange<T> for
 (M, N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        M::on_session_change(time_elapsed.clone(), should_reward);
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, N: OnSessionChange<T>, O: OnSessionChange<T>,
      P: OnSessionChange<T>, Q: OnSessionChange<T>, R: OnSessionChange<T>,
      S: OnSessionChange<T>> OnSessionChange<T> for (N, O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        N::on_session_change(time_elapsed.clone(), should_reward);
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, O: OnSessionChange<T>, P: OnSessionChange<T>,
      Q: OnSessionChange<T>, R: OnSessionChange<T>, S: OnSessionChange<T>>
 OnSessionChange<T> for (O, P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        O::on_session_change(time_elapsed.clone(), should_reward);
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, P: OnSessionChange<T>, Q: OnSessionChange<T>,
      R: OnSessionChange<T>, S: OnSessionChange<T>> OnSessionChange<T> for
 (P, Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        P::on_session_change(time_elapsed.clone(), should_reward);
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, Q: OnSessionChange<T>, R: OnSessionChange<T>,
      S: OnSessionChange<T>> OnSessionChange<T> for (Q, R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        Q::on_session_change(time_elapsed.clone(), should_reward);
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, R: OnSessionChange<T>, S: OnSessionChange<T>>
 OnSessionChange<T> for (R, S) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        R::on_session_change(time_elapsed.clone(), should_reward);
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T: Clone, S: OnSessionChange<T>> OnSessionChange<T> for (S,) {
    fn on_session_change(time_elapsed: T, should_reward: bool) {
        S::on_session_change(time_elapsed.clone(), should_reward);
    }
}
impl <T> OnSessionChange<T> for () {
    fn on_session_change(_: T, _: bool) { }
}


pub trait Trait: timestamp::Trait + consensus::Trait {
    /// Create a session key from an account key.
    type
    ConvertAccountIdToSessionKey: Convert<Self::AccountId,
                                          Option<Self::SessionKey>>;

    /// Handler when a session changes.
    type
    OnSessionChange: OnSessionChange<Self::Moment>;

    /// The overarching event type.
    type
    Event: From<Event<Self>> +
    Into<<Self as system::Trait>::Event>;
}


// set new value for next session









// INTERNAL API (available to other runtime modules)


// Do this last, after the staking system has had the chance to switch out the authorities for the
// new set.
// Check block number and call `rotate_session` if necessary.



// Increment current session index.

// Enact session length change.


// Update any changes in session keys.





















// Block 1: Change to length 3; no visible change.

// Block 2: Length now changed to 3. Index incremented.

// Block 3: Length now changed to 3. Index incremented.

// Block 4: Change to length 2; no visible change.

// Block 5: Length now changed to 2. Index incremented.

// Block 6: No change.

// Block 7: Next index.

// Block 1: No change

// Block 2: Session rollover, but no change.

// Block 3: Set new key for validator 2; no visible change.


// Block 4: Session rollover, authority 2 changes.
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
    fn on_finalize(n: T::BlockNumber) { Self::check_rotate_session(n); }
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
          r" Sets the session key of a validator (function caller) to `key`."]
    #[doc = r" This doesn't take effect until the next session."]
    fn set_key(origin: T::Origin, key: T::SessionKey)
     -> ::srml_support::dispatch::Result {
        {
            let who = ensure_signed(origin)?;
            <NextKeyFor<T>>::insert(who, key);
        }
        Ok(())
    }
    #[doc =
          r" Set a new session length. Won't kick in until the next session change (at current length)."]
    fn set_length(new: T::BlockNumber) -> ::srml_support::dispatch::Result {
        { <NextSessionLength<T>>::put(new); }
        Ok(())
    }
    #[doc = r" Forces a new session."]
    fn force_new_session(apply_rewards: bool) -> Result {
        Self::apply_force_new_session(apply_rewards)
    }
}
pub enum Call<T: Trait> {

    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                  ::srml_support::dispatch::Never),

    #[allow(non_camel_case_types)]
    set_key(T::SessionKey),

    #[allow(non_camel_case_types)]
    set_length(
               #[codec(compact)]
               T::BlockNumber),

    #[allow(non_camel_case_types)]
    force_new_session(bool),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for Call<T> where
         T::SessionKey: _parity_codec::Encode,
         T::SessionKey: _parity_codec::Encode,
         T::BlockNumber: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::set_key(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    Call::set_length(ref aa) => {
                        dest.push_byte(1usize as u8);
                        {
                            dest.push(&<<T::BlockNumber as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::BlockNumber>>::from(aa));
                        }
                    }
                    Call::force_new_session(ref aa) => {
                        dest.push_byte(2usize as u8);
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
         T::SessionKey: _parity_codec::Decode,
         T::SessionKey: _parity_codec::Decode,
         T::BlockNumber: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::set_key(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::set_length(<<T::BlockNumber as
                                               _parity_codec::HasCompact>::Type
                                                  as
                                                  _parity_codec::Decode>::decode(input)?.into()))
                    }
                    x if x == 2usize as u8 => {
                        Some(Call::force_new_session(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::set_key(ref key) => Call::set_key((*key).clone()),
            Call::set_length(ref new) => Call::set_length((*new).clone()),
            Call::force_new_session(ref apply_rewards) =>
            Call::force_new_session((*apply_rewards).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/session/src/lib.rs",
                                             160u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::set_key(ref key) => {
                let self_params = (key,);
                if let Call::set_key(ref key) = *_other {
                    self_params == (key,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/session/src/lib.rs",
                                                         160u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_length(ref new) => {
                let self_params = (new,);
                if let Call::set_length(ref new) = *_other {
                    self_params == (new,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/session/src/lib.rs",
                                                         160u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::force_new_session(ref apply_rewards) => {
                let self_params = (apply_rewards,);
                if let Call::force_new_session(ref apply_rewards) = *_other {
                    self_params == (apply_rewards,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/session/src/lib.rs",
                                                         160u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/session/src/lib.rs",
                                             160u32, 1u32))
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
            Call::set_key(ref key) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_key",
                                                               &(key.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_length(ref new) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_length",
                                                               &(new.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::force_new_session(ref apply_rewards) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"force_new_session",
                                                               &(apply_rewards.clone(),))
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
                                           &("srml/session/src/lib.rs",
                                             160u32, 1u32))
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
            Call::set_key(key) => { <Module<T>>::set_key(_origin, key) }
            Call::set_length(new) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_length(new)
                }
            }
            Call::force_new_session(apply_rewards) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::force_new_session(apply_rewards)
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
                                                       &("srml/session/src/lib.rs",
                                                         160u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_key"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("key"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("T::SessionKey"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Sets the session key of a validator (function caller) to `key`.",
                                                                                                             r" This doesn't take effect until the next session."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_length"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("new"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set a new session length. Won't kick in until the next session change (at current length)."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("force_new_session"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("apply_rewards"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("bool"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Forces a new session."]),}]
    }
}
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T> = RawEvent<<T as system::Trait>::BlockNumber>;
/// Events for this module.
///
#[structural_match]
pub enum RawEvent<BlockNumber> {

    #[doc =
          r" New session has happened. Note that the argument is the session index, not the block"]
    #[doc = r" number as the type might suggest."]
    NewSession(BlockNumber),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <BlockNumber: ::std::clone::Clone> ::std::clone::Clone for
 RawEvent<BlockNumber> {
    #[inline]
    fn clone(&self) -> RawEvent<BlockNumber> {
        match (&*self,) {
            (&RawEvent::NewSession(ref __self_0),) =>
            RawEvent::NewSession(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <BlockNumber: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
 RawEvent<BlockNumber> {
    #[inline]
    fn eq(&self, other: &RawEvent<BlockNumber>) -> bool {
        match (&*self, &*other) {
            (&RawEvent::NewSession(ref __self_0),
             &RawEvent::NewSession(ref __arg_1_0)) =>
            (*__self_0) == (*__arg_1_0),
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<BlockNumber>) -> bool {
        match (&*self, &*other) {
            (&RawEvent::NewSession(ref __self_0),
             &RawEvent::NewSession(ref __arg_1_0)) =>
            (*__self_0) != (*__arg_1_0),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <BlockNumber: ::std::cmp::Eq> ::std::cmp::Eq for RawEvent<BlockNumber> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<BlockNumber>; }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawEvent: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <BlockNumber> _parity_codec::Encode for RawEvent<BlockNumber>
         where BlockNumber: _parity_codec::Encode,
         BlockNumber: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::NewSession(ref aa) => {
                        dest.push_byte(0usize as u8);
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
        impl <BlockNumber> _parity_codec::Decode for RawEvent<BlockNumber>
         where BlockNumber: _parity_codec::Decode,
         BlockNumber: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::NewSession(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <BlockNumber: ::std::fmt::Debug> ::std::fmt::Debug for
 RawEvent<BlockNumber> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawEvent::NewSession(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("NewSession");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <BlockNumber> From<RawEvent<BlockNumber>> for () {
    fn from(_: RawEvent<BlockNumber>) -> () { () }
}
impl <BlockNumber> RawEvent<BlockNumber> {
    #[allow(dead_code)]
    pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
        &[::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("NewSession"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["BlockNumber"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" New session has happened. Note that the argument is the session index, not the block",
                                                                                                    r" number as the type might suggest."]),}]
    }
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = " The current set of validators."]
pub struct Validators<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>
 for Validators<T> {
    type
    Query
    =
    Vec<T::AccountId>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Session Validators".as_bytes() }
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
#[doc = " Current length of the session."]
pub struct SessionLength<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for SessionLength<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Session SessionLength".as_bytes() }
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
#[doc = " Current index of the session."]
pub struct CurrentIndex<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for CurrentIndex<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Session CurrentIndex".as_bytes() }
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
#[doc = " Timestamp when current session started."]
pub struct CurrentStart<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>
 for CurrentStart<T> {
    type
    Query
    =
    T::Moment;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Session CurrentStart".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::key()).unwrap_or_else(||
                                                                                                                                                                   Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::put(&val,
                                                                                                                                 storage);
        ret
    }
}
#[doc =
      " New session is being forced if this entry exists; in which case, the boolean value is true if"]
#[doc =
      " the new session should be considered a normal rotation (rewardable) and false if the new session"]
#[doc = " should be considered exceptional (slashable)."]
pub struct ForcingNewSession<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>
 for ForcingNewSession<T> {
    type
    Query
    =
    Option<bool>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Session ForcingNewSession".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::key()).or_else(||
                                                                                                                                                       Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::key()).or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::get(storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::put(&val,
                                                                                                                                storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::kill(storage),
        };
        ret
    }
}
#[doc = " Block at which the session length last changed."]
struct LastLengthChange<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for LastLengthChange<T> {
    type
    Query
    =
    Option<T::BlockNumber>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Session LastLengthChange".as_bytes() }
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
#[doc = " The next key for a given validator."]
struct NextKeyFor<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   T::SessionKey>
 for NextKeyFor<T> {
    type
    Query
    =
    Option<T::SessionKey>;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Session NextKeyFor".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(x: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::SessionKey>>::prefix().to_vec();
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
                                                                                                                  T::SessionKey>>::key_for(key);
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
                                                                                                                  T::SessionKey>>::key_for(key);
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
                                                                                                                  T::SessionKey>>::get(key,
                                                                                                                                       storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::SessionKey>>::insert(key,
                                                                                                                                          &val,
                                                                                                                                          storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::SessionKey>>::remove(key,
                                                                                                                                          storage),
        };
        ret
    }
}
#[doc = " The next session length."]
struct NextSessionLength<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for NextSessionLength<T> {
    type
    Query
    =
    Option<T::BlockNumber>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Session NextSessionLength".as_bytes() }
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
trait Store {
    type
    Validators;
    type
    SessionLength;
    type
    CurrentIndex;
    type
    CurrentStart;
    type
    ForcingNewSession;
    type
    LastLengthChange;
    type
    NextKeyFor;
    type
    NextSessionLength;
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
                                                                       Vec<T::AccountId> =
                                                                   Default::default();
                                                               <Vec<T::AccountId>
                                                                   as
                                                                   Encode>::encode(&def_val)
                                                           }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructSessionLength<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_SessionLength:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructSessionLength<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_SessionLength.get_or_init(||
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
pub struct __GetByteStructCurrentIndex<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CurrentIndex:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCurrentIndex<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CurrentIndex.get_or_init(||
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
pub struct __GetByteStructCurrentStart<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_CurrentStart:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructCurrentStart<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_CurrentStart.get_or_init(||
                                                             {
                                                                 let def_val:
                                                                         T::Moment =
                                                                     Default::default();
                                                                 <T::Moment as
                                                                     Encode>::encode(&def_val)
                                                             }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructForcingNewSession<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ForcingNewSession:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructForcingNewSession<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ForcingNewSession.get_or_init(||
                                                                  {
                                                                      let def_val:
                                                                              Option<bool> =
                                                                          Default::default();
                                                                      <Option<bool>
                                                                          as
                                                                          Encode>::encode(&def_val)
                                                                  }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructLastLengthChange<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_LastLengthChange:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructLastLengthChange<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_LastLengthChange.get_or_init(||
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
pub struct __GetByteStructNextKeyFor<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_NextKeyFor:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNextKeyFor<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_NextKeyFor.get_or_init(||
                                                           {
                                                               let def_val:
                                                                       Option<T::SessionKey> =
                                                                   Default::default();
                                                               <Option<T::SessionKey>
                                                                   as
                                                                   Encode>::encode(&def_val)
                                                           }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructNextSessionLength<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_NextSessionLength:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNextSessionLength<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_NextSessionLength.get_or_init(||
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
impl <T: Trait> Store for Module<T> {
    type
    Validators
    =
    Validators<T>;
    type
    SessionLength
    =
    SessionLength<T>;
    type
    CurrentIndex
    =
    CurrentIndex<T>;
    type
    CurrentStart
    =
    CurrentStart<T>;
    type
    ForcingNewSession
    =
    ForcingNewSession<T>;
    type
    LastLengthChange
    =
    LastLengthChange<T>;
    type
    NextKeyFor
    =
    NextKeyFor<T>;
    type
    NextSessionLength
    =
    NextSessionLength<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc = " The current set of validators."]
    pub fn validators() -> Vec<T::AccountId> {
        <Validators<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::AccountId>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Current length of the session."]
    pub fn length() -> T::BlockNumber {
        <SessionLength<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Current index of the session."]
    pub fn current_index() -> T::BlockNumber {
        <CurrentIndex<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Timestamp when current session started."]
    pub fn current_start() -> T::Moment {
        <CurrentStart<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " New session is being forced if this entry exists; in which case, the boolean value is true if"]
    #[doc =
          " the new session should be considered a normal rotation (rewardable) and false if the new session"]
    #[doc = " should be considered exceptional (slashable)."]
    pub fn forcing_new_session() -> Option<bool> {
        <ForcingNewSession<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Validators"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructValidators::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The current set of validators."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SessionLength"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSessionLength::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Current length of the session."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentIndex"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentIndex::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Current index of the session."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentStart"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Moment")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentStart::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Timestamp when current session started."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ForcingNewSession"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("bool")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructForcingNewSession::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" New session is being forced if this entry exists; in which case, the boolean value is true if",
                                                                                                                                                                                                                                                                                                                                                                                                    " the new session should be considered a normal rotation (rewardable) and false if the new session",
                                                                                                                                                                                                                                                                                                                                                                                                    " should be considered exceptional (slashable)."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LastLengthChange"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLastLengthChange::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Block at which the session length last changed."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextKeyFor"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::SessionKey"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       false,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextKeyFor::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next key for a given validator."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextSessionLength"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextSessionLength::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next session length."]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Validators"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::AccountId>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructValidators::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The current set of validators."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("SessionLength"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructSessionLength::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Current length of the session."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentIndex"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentIndex::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Current index of the session."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("CurrentStart"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Moment")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructCurrentStart::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Timestamp when current session started."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ForcingNewSession"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("bool")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructForcingNewSession::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" New session is being forced if this entry exists; in which case, the boolean value is true if",
                                                                                                                                                                                                              " the new session should be considered a normal rotation (rewardable) and false if the new session",
                                                                                                                                                                                                              " should be considered exceptional (slashable)."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("LastLengthChange"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructLastLengthChange::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Block at which the session length last changed."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextKeyFor"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::SessionKey"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 false,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextKeyFor::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next key for a given validator."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextSessionLength"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextSessionLength::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The next session length."]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Session" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "Vec < T :: AccountId > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Vec < ( T :: AccountId , T :: SessionKey ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "Vec < T :: AccountId > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Vec < ( T :: AccountId , T :: SessionKey ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    #[doc = " The current set of validators."]
    pub validators: Vec<T::AccountId>,
    #[doc = " Current length of the session."]
    pub session_length: T::BlockNumber,
    pub keys: Vec<(T::AccountId, T::SessionKey)>,
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
         Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Vec<(T::AccountId,
              T::SessionKey)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
         {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "GenesisConfig",
                                                               false as usize
                                                                   + 1 + 1 +
                                                                   1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "validators",
                                                                    &self.validators)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "sessionLength",
                                                                    &self.session_length)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "keys",
                                                                    &self.keys)
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
         Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Vec<(T::AccountId,
              T::SessionKey)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
         {
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
                                                             "field identifier")
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
                                                                                 &"field index 0 <= i < 3")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "validators" =>
                            _serde::export::Ok(__Field::__field0),
                            "sessionLength" =>
                            _serde::export::Ok(__Field::__field1),
                            "keys" => _serde::export::Ok(__Field::__field2),
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
                            b"validators" =>
                            _serde::export::Ok(__Field::__field0),
                            b"sessionLength" =>
                            _serde::export::Ok(__Field::__field1),
                            b"keys" => _serde::export::Ok(__Field::__field2),
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
                       Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Vec<(T::AccountId,
                            T::SessionKey)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 Vec<T::AccountId>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Vec<(T::AccountId,
                      T::SessionKey)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                                                                                                 &"struct GenesisConfig with 3 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<T::BlockNumber>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct GenesisConfig with 3 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<Vec<(T::AccountId,
                                                                                   T::SessionKey)>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct GenesisConfig with 3 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{validators: __field0,
                                                         session_length:
                                                             __field1,
                                                         keys: __field2,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Vec<T::AccountId>> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<T::BlockNumber> =
                            _serde::export::None;
                        let mut __field2:
                                _serde::export::Option<Vec<(T::AccountId,
                                                            T::SessionKey)>> =
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
                                                                       _serde::de::Error>::duplicate_field("validators"));
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
                                __Field::__field1 => {
                                    if _serde::export::Option::is_some(&__field1)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("sessionLength"));
                                    }
                                    __field1 =
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
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("keys"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<(T::AccountId,
                                                                                                            T::SessionKey)>>(&mut __map)
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
                                match _serde::private::de::missing_field("validators")
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
                                match _serde::private::de::missing_field("sessionLength")
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
                                match _serde::private::de::missing_field("keys")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{validators: __field0,
                                                         session_length:
                                                             __field1,
                                                         keys: __field2,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["validators", "sessionLength", "keys"];
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
        GenesisConfig{validators: Default::default(),
                      session_length: T::BlockNumber::sa(1000),
                      keys: Default::default(),}
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
                      config.validators.clone()))(&self);
            <Validators<T> as
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
                      config.session_length.clone()))(&self);
            <SessionLength<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                          &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v = (|_| T::BlockNumber::sa(0))(&self);
            <CurrentIndex<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                          &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v = (|_| T::Moment::zero())(&self);
            <CurrentStart<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::put(&v,
                                                                                                                                     &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let data =
                (|config: &GenesisConfig<T>| { config.keys.clone() })(&self);
            for (k, v) in data.into_iter() {
                <NextKeyFor<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      T::SessionKey>>::insert(&k,
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
    /// The current number of validators.
    pub fn validator_count() -> u32 { <Validators<T>>::get().len() as u32 }
    /// The last length change if there was one, zero if not.
    pub fn last_length_change() -> T::BlockNumber {
        <LastLengthChange<T>>::get().unwrap_or_else(T::BlockNumber::zero)
    }
    /// Forces a new session, no origin.
    pub fn apply_force_new_session(apply_rewards: bool) -> Result {
        <ForcingNewSession<T>>::put(apply_rewards);
        Ok(())
    }
    /// Set the current set of validators.
    ///
    /// Called by `staking::new_era` only. `rotate_session` must be called after this in order to
    /// update the session keys to the next validator set.
    pub fn set_validators(new: &[T::AccountId]) {
        <Validators<T>>::put(&new.to_vec());
    }
    /// Hook to be called after transaction processing.
    pub fn check_rotate_session(block_number: T::BlockNumber) {
        let is_final_block =
            ((block_number - Self::last_length_change()) %
                 Self::length()).is_zero();
        let (should_end_session, apply_rewards) =
            <ForcingNewSession<T>>::take().map_or((is_final_block,
                                                   is_final_block),
                                                  |apply_rewards|
                                                      (true, apply_rewards));
        if should_end_session {
            Self::rotate_session(is_final_block, apply_rewards);
        }
    }
    /// Move on to next session: register the new authority set.
    pub fn rotate_session(is_final_block: bool, apply_rewards: bool) {
        let now = <timestamp::Module<T>>::get();
        let time_elapsed = now.clone() - Self::current_start();
        let session_index = <CurrentIndex<T>>::get() + One::one();
        Self::deposit_event(RawEvent::NewSession(session_index));
        <CurrentIndex<T>>::put(session_index);
        <CurrentStart<T>>::put(now);
        let len_changed =
            if let Some(next_len) = <NextSessionLength<T>>::take() {
                <SessionLength<T>>::put(next_len);
                true
            } else { false };
        if len_changed || !is_final_block {
            let block_number = <system::Module<T>>::block_number();
            <LastLengthChange<T>>::put(block_number);
        }
        T::OnSessionChange::on_session_change(time_elapsed, apply_rewards);
        let v = Self::validators();
        <consensus::Module<T>>::set_authority_count(v.len() as u32);
        for (i, v) in v.into_iter().enumerate() {
            <consensus::Module<T>>::set_authority(i as u32,
                                                  &<NextKeyFor<T>>::get(&v).or_else(||
                                                                                        T::ConvertAccountIdToSessionKey::convert(v)).unwrap_or_default());
        };
    }
    /// Get the time that should elapse over a session if everything is working perfectly.
    pub fn ideal_session_duration() -> T::Moment {
        let block_period: T::Moment =
            <timestamp::Module<T>>::minimum_period();
        let session_length: T::BlockNumber = Self::length();
        Mul::<T::BlockNumber>::mul(block_period, session_length)
    }
    /// Number of blocks remaining in this session, not counting this one. If the session is
    /// due to rotate at the end of this block, then it will return 0. If the session just began, then
    /// it will return `Self::length() - 1`.
    pub fn blocks_remaining() -> T::BlockNumber {
        let length = Self::length();
        let length_minus_1 = length - One::one();
        let block_number = <system::Module<T>>::block_number();
        length_minus_1 -
            (block_number - Self::last_length_change() + length_minus_1) %
                length
    }
}
impl <T: Trait> OnFreeBalanceZero<T::AccountId> for Module<T> {
    fn on_free_balance_zero(who: &T::AccountId) {
        <NextKeyFor<T>>::remove(who);
    }
}
