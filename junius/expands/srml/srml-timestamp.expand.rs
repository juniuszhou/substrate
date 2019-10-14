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

//! # Timestamp Module
//!
//! The Timestamp module provides functionality to get and set the on-chain time.
//!
//! - [`timestamp::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! The Timestamp module allows the validators to set and validate a timestamp with each block.
//!
//! It uses inherents for timestamp data, which is provided by the block author and validated/verified
//! by other validators. The timestamp can be set only once per block and must be set each block.
//! There could be a constraint on how much time must pass before setting the new timestamp.
//!
//! **NOTE:** The Timestamp module is the recommended way to query the on-chain time instead of using
//! an approach based on block numbers. The block number based time measurement can cause issues
//! because of cumulative calculation errors and hence should be avoided.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `set` - Sets the current time.
//!
//! ### Public functions
//!
//! * `get` - Gets the current time for the current block. If this function is called prior to
//! setting the timestamp, it will return the timestamp of the previous block.
//! * `minimum_period` - Gets the minimum (and advised) period between blocks for the chain.
//!
//! ## Usage
//!
//! The following example shows how to use the Timestamp module in your custom module to query the current timestamp.
//!
//! ### Prerequisites
//!
//! Import the Timestamp module into your custom module and derive the module configuration
//! trait from the timestamp trait.
//!
//! ### Get current timestamp
//!
//! ```
//! use srml_support::{decl_module, dispatch::Result};
//! # use srml_timestamp as timestamp;
//! use system::ensure_signed;
//!
//! pub trait Trait: timestamp::Trait {}
//!
//! decl_module! {
//! 	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
//! 		pub fn get_time(origin) -> Result {
//! 			let _sender = ensure_signed(origin)?;
//! 			let _now = <timestamp::Module<T>>::get();
//! 			Ok(())
//! 		}
//! 	}
//! }
//! # fn main() {}
//! ```
//!
//! ### Example from the SRML
//!
//! The [Session module](https://github.com/paritytech/substrate/blob/master/srml/session/src/lib.rs) uses
//! the Timestamp module for session management.
//!
//! ## Related Modules
//!
//! * [Session](../srml_session/index.html)
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


use parity_codec::Encode;
#[cfg(feature = "std")]
use parity_codec::Decode;
#[cfg(feature = "std")]
use inherents::ProvideInherentData;
use srml_support::{StorageValue, Parameter, decl_storage, decl_module};
use srml_support::for_each_tuple;
use runtime_primitives::traits::{As, SimpleArithmetic, Zero};
use system::ensure_none;
use rstd::{result, ops::{Mul, Div}, cmp};
use inherents::{RuntimeString, InherentIdentifier, ProvideInherent,
                IsFatalError, InherentData};

/// The identifier for the `timestamp` inherent.
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"timstap0";
/// The type of the inherent.
pub type InherentType = u64;

/// Errors that can occur while checking the timestamp inherent.
pub enum InherentError {

    /// The timestamp is valid in the future.
    /// This is a non-fatal-error and will not stop checking the inherents.
    ValidAtTimestamp(InherentType),

    /// Some other error.
    Other(RuntimeString),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_InherentError: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for InherentError {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    InherentError::ValidAtTimestamp(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    InherentError::Other(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for InherentError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&InherentError::ValidAtTimestamp(ref __self_0),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("ValidAtTimestamp");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&InherentError::Other(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Other");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_InherentError: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for InherentError {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(InherentError::ValidAtTimestamp(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(InherentError::Other(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };

impl IsFatalError for InherentError {
    fn is_fatal_error(&self) -> bool {
        match self {
            InherentError::ValidAtTimestamp(_) => false,
            InherentError::Other(_) => true,
        }
    }
}

impl InherentError {
    /// Try to create an instance ouf of the given identifier and data.
    #[cfg(feature = "std")]
    pub fn try_from(id: &InherentIdentifier, data: &[u8]) -> Option<Self> {
        if id == &INHERENT_IDENTIFIER {
            <InherentError as parity_codec::Decode>::decode(&mut &data[..])
        } else { None }
    }
}

/// Auxiliary trait to extract timestamp inherent data.
pub trait TimestampInherentData {
    /// Get timestamp inherent data.
    fn timestamp_inherent_data(&self)
    -> Result<InherentType, RuntimeString>;
}

impl TimestampInherentData for InherentData {
    fn timestamp_inherent_data(&self) -> Result<InherentType, RuntimeString> {
        self.get_data(&INHERENT_IDENTIFIER).and_then(|r|
                                                         r.ok_or_else(||
                                                                          "Timestamp inherent data not found".into()))
    }
}

#[cfg(feature = "std")]
pub struct InherentDataProvider;

#[cfg(feature = "std")]
impl ProvideInherentData for InherentDataProvider {
    fn inherent_identifier(&self) -> &'static InherentIdentifier {
        &INHERENT_IDENTIFIER
    }

    fn provide_inherent_data(&self, inherent_data: &mut InherentData)
     -> Result<(), RuntimeString> {
        use std::time::SystemTime;

        let now = SystemTime::now();
        now.duration_since(SystemTime::UNIX_EPOCH).map_err(|_|
                                                               {
                                                                   "Current time is before unix epoch".into()
                                                               }).and_then(|d|
                                                                               {
                                                                                   let duration:
                                                                                           InherentType =
                                                                                       d.as_secs();
                                                                                   inherent_data.put_data(INHERENT_IDENTIFIER,
                                                                                                          &duration)
                                                                               })
    }

    fn error_to_string(&self, error: &[u8]) -> Option<String> {
        InherentError::try_from(&INHERENT_IDENTIFIER,
                                error).map(|e|








                                               // Manage upgrade. Remove after all networks upgraded.
                                               // TODO: #2133



                                               // TODO: #2133






















                                               ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                  &match (&e,)
                                                                                                       {
                                                                                                       (arg0,)
                                                                                                       =>
                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                                   })))
    }
}
/// A trait which is called when the timestamp is set.
pub trait OnTimestampSet<Moment> {
    fn on_timestamp_set(moment: Moment);
}
macro_rules! impl_timestamp_set((  ) => (
                                impl < Moment > OnTimestampSet < Moment > for
                                (  ) { fn on_timestamp_set ( _ : Moment ) {  }
                                } ) ; ( $ ( $ t : ident ) * ) => {
                                impl < Moment : Clone , $ (
                                $ t : OnTimestampSet < Moment > ) , * >
                                OnTimestampSet < Moment > for ( $ ( $ t , ) *
                                ) {
                                fn on_timestamp_set ( moment : Moment ) {
                                $ (
                                $ t :: on_timestamp_set ( moment . clone (  )
                                ) ; ) * } } });
impl <Moment: Clone, A: OnTimestampSet<Moment>, B: OnTimestampSet<Moment>,
      C: OnTimestampSet<Moment>, D: OnTimestampSet<Moment>,
      E: OnTimestampSet<Moment>, F: OnTimestampSet<Moment>,
      G: OnTimestampSet<Moment>, H: OnTimestampSet<Moment>,
      I: OnTimestampSet<Moment>, J: OnTimestampSet<Moment>,
      K: OnTimestampSet<Moment>, L: OnTimestampSet<Moment>,
      M: OnTimestampSet<Moment>, N: OnTimestampSet<Moment>,
      O: OnTimestampSet<Moment>, P: OnTimestampSet<Moment>,
      Q: OnTimestampSet<Moment>, R: OnTimestampSet<Moment>,
      S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for
 (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        A::on_timestamp_set(moment.clone());
        B::on_timestamp_set(moment.clone());
        C::on_timestamp_set(moment.clone());
        D::on_timestamp_set(moment.clone());
        E::on_timestamp_set(moment.clone());
        F::on_timestamp_set(moment.clone());
        G::on_timestamp_set(moment.clone());
        H::on_timestamp_set(moment.clone());
        I::on_timestamp_set(moment.clone());
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, B: OnTimestampSet<Moment>, C: OnTimestampSet<Moment>,
      D: OnTimestampSet<Moment>, E: OnTimestampSet<Moment>,
      F: OnTimestampSet<Moment>, G: OnTimestampSet<Moment>,
      H: OnTimestampSet<Moment>, I: OnTimestampSet<Moment>,
      J: OnTimestampSet<Moment>, K: OnTimestampSet<Moment>,
      L: OnTimestampSet<Moment>, M: OnTimestampSet<Moment>,
      N: OnTimestampSet<Moment>, O: OnTimestampSet<Moment>,
      P: OnTimestampSet<Moment>, Q: OnTimestampSet<Moment>,
      R: OnTimestampSet<Moment>, S: OnTimestampSet<Moment>>
 OnTimestampSet<Moment> for
 (B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        B::on_timestamp_set(moment.clone());
        C::on_timestamp_set(moment.clone());
        D::on_timestamp_set(moment.clone());
        E::on_timestamp_set(moment.clone());
        F::on_timestamp_set(moment.clone());
        G::on_timestamp_set(moment.clone());
        H::on_timestamp_set(moment.clone());
        I::on_timestamp_set(moment.clone());
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, C: OnTimestampSet<Moment>, D: OnTimestampSet<Moment>,
      E: OnTimestampSet<Moment>, F: OnTimestampSet<Moment>,
      G: OnTimestampSet<Moment>, H: OnTimestampSet<Moment>,
      I: OnTimestampSet<Moment>, J: OnTimestampSet<Moment>,
      K: OnTimestampSet<Moment>, L: OnTimestampSet<Moment>,
      M: OnTimestampSet<Moment>, N: OnTimestampSet<Moment>,
      O: OnTimestampSet<Moment>, P: OnTimestampSet<Moment>,
      Q: OnTimestampSet<Moment>, R: OnTimestampSet<Moment>,
      S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for
 (C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        C::on_timestamp_set(moment.clone());
        D::on_timestamp_set(moment.clone());
        E::on_timestamp_set(moment.clone());
        F::on_timestamp_set(moment.clone());
        G::on_timestamp_set(moment.clone());
        H::on_timestamp_set(moment.clone());
        I::on_timestamp_set(moment.clone());
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, D: OnTimestampSet<Moment>, E: OnTimestampSet<Moment>,
      F: OnTimestampSet<Moment>, G: OnTimestampSet<Moment>,
      H: OnTimestampSet<Moment>, I: OnTimestampSet<Moment>,
      J: OnTimestampSet<Moment>, K: OnTimestampSet<Moment>,
      L: OnTimestampSet<Moment>, M: OnTimestampSet<Moment>,
      N: OnTimestampSet<Moment>, O: OnTimestampSet<Moment>,
      P: OnTimestampSet<Moment>, Q: OnTimestampSet<Moment>,
      R: OnTimestampSet<Moment>, S: OnTimestampSet<Moment>>
 OnTimestampSet<Moment> for (D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        D::on_timestamp_set(moment.clone());
        E::on_timestamp_set(moment.clone());
        F::on_timestamp_set(moment.clone());
        G::on_timestamp_set(moment.clone());
        H::on_timestamp_set(moment.clone());
        I::on_timestamp_set(moment.clone());
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, E: OnTimestampSet<Moment>, F: OnTimestampSet<Moment>,
      G: OnTimestampSet<Moment>, H: OnTimestampSet<Moment>,
      I: OnTimestampSet<Moment>, J: OnTimestampSet<Moment>,
      K: OnTimestampSet<Moment>, L: OnTimestampSet<Moment>,
      M: OnTimestampSet<Moment>, N: OnTimestampSet<Moment>,
      O: OnTimestampSet<Moment>, P: OnTimestampSet<Moment>,
      Q: OnTimestampSet<Moment>, R: OnTimestampSet<Moment>,
      S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for
 (E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        E::on_timestamp_set(moment.clone());
        F::on_timestamp_set(moment.clone());
        G::on_timestamp_set(moment.clone());
        H::on_timestamp_set(moment.clone());
        I::on_timestamp_set(moment.clone());
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, F: OnTimestampSet<Moment>, G: OnTimestampSet<Moment>,
      H: OnTimestampSet<Moment>, I: OnTimestampSet<Moment>,
      J: OnTimestampSet<Moment>, K: OnTimestampSet<Moment>,
      L: OnTimestampSet<Moment>, M: OnTimestampSet<Moment>,
      N: OnTimestampSet<Moment>, O: OnTimestampSet<Moment>,
      P: OnTimestampSet<Moment>, Q: OnTimestampSet<Moment>,
      R: OnTimestampSet<Moment>, S: OnTimestampSet<Moment>>
 OnTimestampSet<Moment> for (F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        F::on_timestamp_set(moment.clone());
        G::on_timestamp_set(moment.clone());
        H::on_timestamp_set(moment.clone());
        I::on_timestamp_set(moment.clone());
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, G: OnTimestampSet<Moment>, H: OnTimestampSet<Moment>,
      I: OnTimestampSet<Moment>, J: OnTimestampSet<Moment>,
      K: OnTimestampSet<Moment>, L: OnTimestampSet<Moment>,
      M: OnTimestampSet<Moment>, N: OnTimestampSet<Moment>,
      O: OnTimestampSet<Moment>, P: OnTimestampSet<Moment>,
      Q: OnTimestampSet<Moment>, R: OnTimestampSet<Moment>,
      S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for
 (G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        G::on_timestamp_set(moment.clone());
        H::on_timestamp_set(moment.clone());
        I::on_timestamp_set(moment.clone());
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, H: OnTimestampSet<Moment>, I: OnTimestampSet<Moment>,
      J: OnTimestampSet<Moment>, K: OnTimestampSet<Moment>,
      L: OnTimestampSet<Moment>, M: OnTimestampSet<Moment>,
      N: OnTimestampSet<Moment>, O: OnTimestampSet<Moment>,
      P: OnTimestampSet<Moment>, Q: OnTimestampSet<Moment>,
      R: OnTimestampSet<Moment>, S: OnTimestampSet<Moment>>
 OnTimestampSet<Moment> for (H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        H::on_timestamp_set(moment.clone());
        I::on_timestamp_set(moment.clone());
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, I: OnTimestampSet<Moment>, J: OnTimestampSet<Moment>,
      K: OnTimestampSet<Moment>, L: OnTimestampSet<Moment>,
      M: OnTimestampSet<Moment>, N: OnTimestampSet<Moment>,
      O: OnTimestampSet<Moment>, P: OnTimestampSet<Moment>,
      Q: OnTimestampSet<Moment>, R: OnTimestampSet<Moment>,
      S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for
 (I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        I::on_timestamp_set(moment.clone());
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, J: OnTimestampSet<Moment>, K: OnTimestampSet<Moment>,
      L: OnTimestampSet<Moment>, M: OnTimestampSet<Moment>,
      N: OnTimestampSet<Moment>, O: OnTimestampSet<Moment>,
      P: OnTimestampSet<Moment>, Q: OnTimestampSet<Moment>,
      R: OnTimestampSet<Moment>, S: OnTimestampSet<Moment>>
 OnTimestampSet<Moment> for (J, K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        J::on_timestamp_set(moment.clone());
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, K: OnTimestampSet<Moment>, L: OnTimestampSet<Moment>,
      M: OnTimestampSet<Moment>, N: OnTimestampSet<Moment>,
      O: OnTimestampSet<Moment>, P: OnTimestampSet<Moment>,
      Q: OnTimestampSet<Moment>, R: OnTimestampSet<Moment>,
      S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for
 (K, L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        K::on_timestamp_set(moment.clone());
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, L: OnTimestampSet<Moment>, M: OnTimestampSet<Moment>,
      N: OnTimestampSet<Moment>, O: OnTimestampSet<Moment>,
      P: OnTimestampSet<Moment>, Q: OnTimestampSet<Moment>,
      R: OnTimestampSet<Moment>, S: OnTimestampSet<Moment>>
 OnTimestampSet<Moment> for (L, M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        L::on_timestamp_set(moment.clone());
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, M: OnTimestampSet<Moment>, N: OnTimestampSet<Moment>,
      O: OnTimestampSet<Moment>, P: OnTimestampSet<Moment>,
      Q: OnTimestampSet<Moment>, R: OnTimestampSet<Moment>,
      S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for
 (M, N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        M::on_timestamp_set(moment.clone());
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, N: OnTimestampSet<Moment>, O: OnTimestampSet<Moment>,
      P: OnTimestampSet<Moment>, Q: OnTimestampSet<Moment>,
      R: OnTimestampSet<Moment>, S: OnTimestampSet<Moment>>
 OnTimestampSet<Moment> for (N, O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        N::on_timestamp_set(moment.clone());
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, O: OnTimestampSet<Moment>, P: OnTimestampSet<Moment>,
      Q: OnTimestampSet<Moment>, R: OnTimestampSet<Moment>,
      S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for (O, P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        O::on_timestamp_set(moment.clone());
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, P: OnTimestampSet<Moment>, Q: OnTimestampSet<Moment>,
      R: OnTimestampSet<Moment>, S: OnTimestampSet<Moment>>
 OnTimestampSet<Moment> for (P, Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        P::on_timestamp_set(moment.clone());
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, Q: OnTimestampSet<Moment>, R: OnTimestampSet<Moment>,
      S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for (Q, R, S) {
    fn on_timestamp_set(moment: Moment) {
        Q::on_timestamp_set(moment.clone());
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, R: OnTimestampSet<Moment>, S: OnTimestampSet<Moment>>
 OnTimestampSet<Moment> for (R, S) {
    fn on_timestamp_set(moment: Moment) {
        R::on_timestamp_set(moment.clone());
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment: Clone, S: OnTimestampSet<Moment>> OnTimestampSet<Moment> for
 (S,) {
    fn on_timestamp_set(moment: Moment) {
        S::on_timestamp_set(moment.clone());
    }
}
impl <Moment> OnTimestampSet<Moment> for () {
    fn on_timestamp_set(_: Moment) { }
}
/// The module configuration trait
pub trait Trait: system::Trait {
    /// Type used for expressing timestamp.
    type
    Moment: Parameter +
    Default +
    SimpleArithmetic +
    Mul<Self::BlockNumber,
    Output
    =
    Self::Moment> +
    Div<Self::BlockNumber,
    Output
    =
    Self::Moment>;
    /// Something which can be notified when the timestamp is set. Set this to `()` if not needed.
    type
    OnTimestampSet: OnTimestampSet<Self::Moment>;
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
    fn on_initialize(_block_number_not_used: T::BlockNumber) {
        if let Some(period) = <BlockPeriod<T>>::take() {
            if !<MinimumPeriod<T>>::exists() {
                <MinimumPeriod<T>>::put(period)
            }
        }
    }
}
impl <T: Trait>
 ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber> for
 Module<T> {
    fn on_finalize(_block_number_not_used: T::BlockNumber) {
        if !<Self as Store>::take() {
            {
                ::std::rt::begin_panic("Timestamp must be updated once in the block",
                                       &("srml/timestamp/src/lib.rs", 247u32,
                                         4u32))
            }
        };
    }
}
impl <T: Trait>
 ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
 for Module<T> {
}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl <T: Trait> Module<T> {
    #[doc = r" Set the current time."]
    #[doc = r""]
    #[doc =
          r" This call should be invoked exactly once per block. It will panic at the finalization phase,"]
    #[doc = r" if this call hasn't been invoked by that time."]
    #[doc = r""]
    #[doc =
          r" The timestamp should be greater than the previous one by the amount specified by `minimum_period`."]
    #[doc = r""]
    #[doc = r" The dispatch origin for this call must be `Inherent`."]
    fn set(origin: T::Origin, now: T::Moment)
     -> ::srml_support::dispatch::Result {
        {
            ensure_none(origin)?;
            if !!<Self as Store>::exists() {
                {
                    ::std::rt::begin_panic("Timestamp must be updated only once in the block",
                                           &("srml/timestamp/src/lib.rs",
                                             225u32, 4u32))
                }
            };
            if !(Self::now().is_zero() ||
                     now >= Self::now() + <MinimumPeriod<T>>::get()) {
                {
                    ::std::rt::begin_panic("Timestamp must increment by at least <MinimumPeriod> between sequential blocks",
                                           &("srml/timestamp/src/lib.rs",
                                             226u32, 4u32))
                }
            };
            <Self as Store>::put(now.clone());
            <Self as Store>::put(true);
            <T::OnTimestampSet as OnTimestampSet<_>>::on_timestamp_set(now);
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
    set(
        #[codec(compact)]
        T::Moment),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for Call<T> where
         T::Moment: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::set(ref aa) => {
                        dest.push_byte(0usize as u8);
                        {
                            dest.push(&<<T::Moment as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Moment>>::from(aa));
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
         T::Moment: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::set(<<T::Moment as
                                        _parity_codec::HasCompact>::Type as
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
            Call::set(ref now) => Call::set((*now).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/timestamp/src/lib.rs",
                                             213u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::set(ref now) => {
                let self_params = (now,);
                if let Call::set(ref now) = *_other {
                    self_params == (now,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/timestamp/src/lib.rs",
                                                         213u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/timestamp/src/lib.rs",
                                             213u32, 1u32))
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
            Call::set(ref now) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set",
                                                               &(now.clone(),))
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
                                           &("srml/timestamp/src/lib.rs",
                                             213u32, 1u32))
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
            Call::set(now) => { <Module<T>>::set(_origin, now) }
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
                                                       &("srml/timestamp/src/lib.rs",
                                                         213u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("now"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Moment>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the current time.",
                                                                                                             r"",
                                                                                                             r" This call should be invoked exactly once per block. It will panic at the finalization phase,",
                                                                                                             r" if this call hasn't been invoked by that time.",
                                                                                                             r"",
                                                                                                             r" The timestamp should be greater than the previous one by the amount specified by `minimum_period`.",
                                                                                                             r"",
                                                                                                             r" The dispatch origin for this call must be `Inherent`."]),}]
    }
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = " Current time for the current block."]
pub struct Now<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>
 for Now<T> {
    type
    Query
    =
    T::Moment;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp Now".as_bytes() }
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
      " Old storage item provided for compatibility. Remove after all networks upgraded."]
pub struct BlockPeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>
 for BlockPeriod<T> {
    type
    Query
    =
    Option<T::Moment>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp BlockPeriod".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::key()).or_else(||
                                                                                                                                                            Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::key()).or_else(||
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
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::put(&val,
                                                                                                                                     storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::kill(storage),
        };
        ret
    }
}
#[doc =
      " The minimum period between blocks. Beware that this is different to the *expected* period"]
#[doc =
      " that the block production apparatus provides. Your chosen consensus system will generally"]
#[doc =
      " work with this to determine a sensible block time. e.g. For Aura, it will be double this"]
#[doc = " period on default settings."]
pub struct MinimumPeriod<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>
 for MinimumPeriod<T> {
    type
    Query
    =
    T::Moment;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp MinimumPeriod".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::key()).unwrap_or_else(||
                                                                                                                                                                   T::Moment::sa(3))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::key()).unwrap_or_else(||
                                                                                                                                                                    T::Moment::sa(3))
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
#[doc = " Did the timestamp get updated in this block?"]
struct DidUpdate<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>
 for DidUpdate<T> {
    type
    Query
    =
    bool;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp DidUpdate".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::key()).unwrap_or_else(||
                                                                                                                                                              Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::key()).unwrap_or_else(||
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
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::put(&val,
                                                                                                                            storage);
        ret
    }
}
trait Store {
    type
    Now;
    type
    BlockPeriod;
    type
    MinimumPeriod;
    type
    DidUpdate;
}
#[doc(hidden)]
pub struct __GetByteStructNow<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Now:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNow<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Now.get_or_init(||
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
pub struct __GetByteStructBlockPeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_BlockPeriod:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructBlockPeriod<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_BlockPeriod.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        Option<T::Moment> =
                                                                    Default::default();
                                                                <Option<T::Moment>
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructMinimumPeriod<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_MinimumPeriod:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructMinimumPeriod<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_MinimumPeriod.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          T::Moment =
                                                                      T::Moment::sa(3);
                                                                  <T::Moment
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructDidUpdate<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_DidUpdate:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructDidUpdate<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_DidUpdate.get_or_init(||
                                                          {
                                                              let def_val:
                                                                      bool =
                                                                  Default::default();
                                                              <bool as
                                                                  Encode>::encode(&def_val)
                                                          }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    Now
    =
    Now<T>;
    type
    BlockPeriod
    =
    BlockPeriod<T>;
    type
    MinimumPeriod
    =
    MinimumPeriod<T>;
    type
    DidUpdate
    =
    DidUpdate<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc = " Current time for the current block."]
    pub fn now() -> T::Moment {
        <Now<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The minimum period between blocks. Beware that this is different to the *expected* period"]
    #[doc =
          " that the block production apparatus provides. Your chosen consensus system will generally"]
    #[doc =
          " work with this to determine a sensible block time. e.g. For Aura, it will be double this"]
    #[doc = " period on default settings."]
    pub fn minimum_period() -> T::Moment {
        <MinimumPeriod<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Now"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Moment")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNow::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Current time for the current block."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BlockPeriod"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Moment")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBlockPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Old storage item provided for compatibility. Remove after all networks upgraded."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MinimumPeriod"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Moment")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMinimumPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The minimum period between blocks. Beware that this is different to the *expected* period",
                                                                                                                                                                                                                                                                                                                                                                                                    " that the block production apparatus provides. Your chosen consensus system will generally",
                                                                                                                                                                                                                                                                                                                                                                                                    " work with this to determine a sensible block time. e.g. For Aura, it will be double this",
                                                                                                                                                                                                                                                                                                                                                                                                    " period on default settings."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("DidUpdate"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("bool")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDidUpdate::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Did the timestamp get updated in this block?"]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Now"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Moment")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNow::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Current time for the current block."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("BlockPeriod"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Moment")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBlockPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Old storage item provided for compatibility. Remove after all networks upgraded."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("MinimumPeriod"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Moment")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMinimumPeriod::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The minimum period between blocks. Beware that this is different to the *expected* period",
                                                                                                                                                                                                              " that the block production apparatus provides. Your chosen consensus system will generally",
                                                                                                                                                                                                              " work with this to determine a sensible block time. e.g. For Aura, it will be double this",
                                                                                                                                                                                                              " period on default settings."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("DidUpdate"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("bool")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDidUpdate::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Did the timestamp get updated in this block?"]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Timestamp" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "T :: Moment : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "T :: Moment : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    #[doc =
          " The minimum period between blocks. Beware that this is different to the *expected* period"]
    #[doc =
          " that the block production apparatus provides. Your chosen consensus system will generally"]
    #[doc =
          " work with this to determine a sensible block time. e.g. For Aura, it will be double this"]
    #[doc = " period on default settings."]
    pub minimum_period: T::Moment,
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
         T::Moment: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
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
                                                                    "minimumPeriod",
                                                                    &self.minimum_period)
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
         T::Moment: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            "minimumPeriod" =>
                            _serde::export::Ok(__Field::__field0),
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
                            b"minimumPeriod" =>
                            _serde::export::Ok(__Field::__field0),
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
                       T::Moment: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 T::Moment: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            match match _serde::de::SeqAccess::next_element::<T::Moment>(&mut __seq)
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
                        _serde::export::Ok(GenesisConfig{minimum_period:
                                                             __field0,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0: _serde::export::Option<T::Moment> =
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
                                                                       _serde::de::Error>::duplicate_field("minimumPeriod"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Moment>(&mut __map)
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
                                match _serde::private::de::missing_field("minimumPeriod")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{minimum_period:
                                                             __field0,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["minimumPeriod"];
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
    fn default() -> Self { GenesisConfig{minimum_period: T::Moment::sa(3),} }
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
            let v = (|_| T::Moment::sa(0))(&self);
            <Now<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::put(&v,
                                                                                                                                     &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                ((|config: &GenesisConfig<T>|
                      config.minimum_period.clone()))(&self);
            <MinimumPeriod<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Moment>>::put(&v,
                                                                                                                                     &storage);
        }
        let r = storage.into_inner();
        (|_, _, _| { })(r, c, &self);
        Ok(())
    }
}
impl <T: Trait> Module<T> {
    /// Get the current time for the current block.
    ///
    /// NOTE: if this function is called prior to setting the timestamp,
    /// it will return the timestamp of the previous block.
    pub fn get() -> T::Moment { Self::now() }
    /// Set the timestamp to something in particular. Only used for tests.
    #[cfg(feature = "std")]
    pub fn set_timestamp(now: T::Moment) { <Self as Store>::put(now); }
}
fn extract_inherent_data(data: &InherentData)
 -> Result<InherentType, RuntimeString> {
    data.get_data::<InherentType>(&INHERENT_IDENTIFIER).map_err(|_|
                                                                    RuntimeString::from("Invalid timestamp inherent data encoding."))?.ok_or_else(||
                                                                                                                                                      "Timestamp inherent data is not provided.".into())
}
impl <T: Trait> ProvideInherent for Module<T> {
    type
    Call
    =
    Call<T>;
    type
    Error
    =
    InherentError;
    const
    INHERENT_IDENTIFIER:
    InherentIdentifier
    =
    INHERENT_IDENTIFIER;
    fn create_inherent(data: &InherentData) -> Option<Self::Call> {
        let data =
            extract_inherent_data(data).expect("Gets and decodes timestamp inherent data");
        let next_time =
            cmp::max(As::sa(data), Self::now() + <MinimumPeriod<T>>::get());
        Some(Call::set(next_time.into()))
    }
    fn check_inherent(call: &Self::Call, data: &InherentData)
     -> result::Result<(), Self::Error> {
        const MAX_TIMESTAMP_DRIFT: u64 = 60;
        let t =
            match call {
                Call::set(ref t) => t.clone(),
                _ => return Ok(()),
            }.as_();
        let data =
            extract_inherent_data(data).map_err(|e| InherentError::Other(e))?;
        let minimum = (Self::now() + <MinimumPeriod<T>>::get()).as_();
        if t > data + MAX_TIMESTAMP_DRIFT {
            Err(InherentError::Other("Timestamp too far in future to accept".into()))
        } else if t < minimum {
            Err(InherentError::ValidAtTimestamp(minimum))
        } else { Ok(()) }
    }
}
