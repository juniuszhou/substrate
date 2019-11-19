#![feature(prelude_import)]
#![no_std]
// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate Consensus Common.

// Substrate Demo is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate Consensus Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate Consensus Common.  If not, see <http://www.gnu.org/licenses/>.

//! Common utilities for building and using consensus engines in substrate.
//!
//! Much of this crate is _unstable_ and thus the API is likely to undergo
//! change. Implementors of traits should not rely on the interfaces to remain
//! the same.

// This provides "unused" building blocks to other crates
#![allow(dead_code)]

// our error-chain could potentially blow up otherwise
#![recursion_limit = "128"]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

#[macro_use]
extern crate crossbeam_channel;
#[macro_use]
extern crate log;

use std::sync::Arc;
use std::time::Duration;

use runtime_primitives::generic::BlockId;
use runtime_primitives::traits::{AuthorityIdFor, Block};
use futures::prelude::*;
pub use inherents::InherentData;

pub mod offline_tracker {

    // block size limit.














    //! Tracks offline validators.
    use std::collections::HashMap;
    use std::time::{Instant, Duration};
    const REPORT_TIME: Duration = Duration::from_secs(60 * 5);
    struct Observed {
        last_round_end: Instant,
        offline_since: Instant,
    }
    impl Observed {
        fn new() -> Observed {
            let now = Instant::now();
            Observed{last_round_end: now, offline_since: now,}
        }
        fn note_round_end(&mut self, was_online: bool) {
            let now = Instant::now();
            self.last_round_end = now;
            if was_online { self.offline_since = now; }
        }
        fn is_active(&self) -> bool {
            if self.offline_since > self.last_round_end { return true }
            self.last_round_end.duration_since(self.offline_since) <
                REPORT_TIME
        }
    }
    /// Tracks offline validators and can issue a report for those offline.
    pub struct OfflineTracker<AuthorityId> {
        observed: HashMap<AuthorityId, Observed>,
    }
    impl <AuthorityId: Eq + Clone + std::hash::Hash>
     OfflineTracker<AuthorityId> {
        /// Create a new tracker.
        pub fn new() -> Self { OfflineTracker{observed: HashMap::new(),} }
        /// Note new consensus is starting with the given set of validators.
        pub fn note_new_block(&mut self, validators: &[AuthorityId]) {
            use std::collections::HashSet;
            let set: HashSet<_> = validators.iter().cloned().collect();
            self.observed.retain(|k, _| set.contains(k));
        }
        /// Note that a round has ended.
        pub fn note_round_end(&mut self, validator: AuthorityId,
                              was_online: bool) {
            self.observed.entry(validator).or_insert_with(Observed::new).note_round_end(was_online);
        }
        /// Generate a vector of indices for offline account IDs.
        pub fn reports(&self, validators: &[AuthorityId]) -> Vec<u32> {
            validators.iter().enumerate().filter_map(|(i, v)|
                                                         if self.is_online(v)
                                                            {
                                                             None
                                                         } else {
                                                             Some(i as u32)
                                                         }).collect()
        }
        /// Whether reports on a validator set are consistent with our view of things.
        pub fn check_consistency(&self, validators: &[AuthorityId],
                                 reports: &[u32]) -> bool {
            reports.iter().cloned().all(|r|
                                            {
                                                let v =
                                                    match validators.get(r as
                                                                             usize)
                                                        {
                                                        Some(v) => v,
                                                        None => return false,
                                                    };
                                                let thinks_online =
                                                    self.is_online(v);
                                                !thinks_online
                                            })
        }
        fn is_online(&self, v: &AuthorityId) -> bool {
            self.observed.get(v).map(Observed::is_active).unwrap_or(true)
        }
    }
}
pub mod error {
    //! Error types in Consensus
    use runtime_version::RuntimeVersion;
    use error_chain::{error_chain, error_chain_processing,
                      impl_error_chain_processed, impl_extract_backtrace,
                      impl_error_chain_kind};
    use primitives::ed25519::{Public, Signature};
    /// The Error type.
    ///
    /// This tuple struct is made of two elements:
    ///
    /// - an `ErrorKind` which is used to determine the type of the error.
    /// - An internal `State`, not meant for direct use outside of `error_chain`
    ///   internals, containing:
    ///   - a backtrace, generated when the error is created.
    ///   - an error chain, used for the implementation of `Error::cause()`.
    pub struct Error(
                     /// The kind of the error.
                     pub ErrorKind,
                     /// Contains the error chain and the backtrace.
                     #[doc(hidden)]
                     pub ::error_chain::State);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Error(ref __self_0_0, ref __self_0_1) => {
                    let mut debug_trait_builder = f.debug_tuple("Error");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    let _ = debug_trait_builder.field(&&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl ::error_chain::ChainedError for Error {
        type
        ErrorKind
        =
        ErrorKind;
        fn new(kind: ErrorKind, state: ::error_chain::State) -> Error {
            Error(kind, state)
        }
        fn from_kind(kind: Self::ErrorKind) -> Self { Self::from_kind(kind) }
        fn with_chain<E, K>(error: E, kind: K) -> Self where
         E: ::std::error::Error + Send + 'static, K: Into<Self::ErrorKind> {
            Self::with_chain(error, kind)
        }
        fn kind(&self) -> &Self::ErrorKind { self.kind() }
        fn iter(&self) -> ::error_chain::Iter {
            ::error_chain::Iter::new(Some(self))
        }
        fn chain_err<F, EK>(self, error: F) -> Self where F: FnOnce() -> EK,
         EK: Into<ErrorKind> {
            self.chain_err(error)
        }
        fn backtrace(&self) -> Option<&::error_chain::Backtrace> {
            self.backtrace()
        }
        #[allow(unknown_lints,
                renamed_and_removed_lints,
                unused_doc_comment,
                unused_doc_comments)]
        fn extract_backtrace(e: &(::std::error::Error + Send + 'static))
         -> Option<::error_chain::InternalBacktrace> {
            if let Some(e) = e.downcast_ref::<Error>() {
                return Some(e.1.backtrace.clone());
            }
            None
        }
    }
    #[allow(dead_code)]
    impl Error {
        /// Constructs an error from a kind, and generates a backtrace.
        pub fn from_kind(kind: ErrorKind) -> Error {
            Error(kind, ::error_chain::State::default())
        }
        /// Constructs a chained error from another error and a kind, and generates a backtrace.
        pub fn with_chain<E, K>(error: E, kind: K) -> Error where
         E: ::std::error::Error + Send + 'static, K: Into<ErrorKind> {
            Error::with_boxed_chain(Box::new(error), kind)
        }
        /// Construct a chained error from another boxed error and a kind, and generates a backtrace
        pub fn with_boxed_chain<K>(error: Box<::std::error::Error + Send>,
                                   kind: K) -> Error where
         K: Into<ErrorKind> {
            Error(kind.into(), ::error_chain::State::new::<Error>(error))
        }
        /// Returns the kind of the error.
        pub fn kind(&self) -> &ErrorKind { &self.0 }
        /// Iterates over the error chain.
        pub fn iter(&self) -> ::error_chain::Iter {
            ::error_chain::ChainedError::iter(self)
        }
        /// Returns the backtrace associated with this error.
        pub fn backtrace(&self) -> Option<&::error_chain::Backtrace> {
            self.1.backtrace()
        }
        /// Extends the error chain with a new entry.
        pub fn chain_err<F, EK>(self, error: F) -> Error where F: FnOnce() ->
         EK, EK: Into<ErrorKind> {
            Error::with_chain(self, Self::from_kind(error().into()))
        }
        /// A short description of the error.
        /// This method is identical to [`Error::description()`](https://doc.rust-lang.org/nightly/std/error/trait.Error.html#tymethod.description)
        pub fn description(&self) -> &str { self.0.description() }
    }
    impl ::std::error::Error for Error {
        fn description(&self) -> &str { self.description() }
        #[allow(unknown_lints,
                renamed_and_removed_lints,
                unused_doc_comment,
                unused_doc_comments)]
        fn cause(&self) -> Option<&::std::error::Error> {
            match self.1.next_error {
                Some(ref c) => Some(&**c),
                None => { match self.0 { _ => None, } }
            }
        }
    }
    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl From<ErrorKind> for Error {
        fn from(e: ErrorKind) -> Self { Error::from_kind(e) }
    }
    impl <'a> From<&'a str> for Error {
        fn from(s: &'a str) -> Self { Error::from_kind(s.into()) }
    }
    impl From<String> for Error {
        fn from(s: String) -> Self { Error::from_kind(s.into()) }
    }
    #[doc = r" The kind of an error."]
    pub enum ErrorKind {

        #[doc = r" A convenient variant for String."]
        Msg(String),

        #[doc = r" Missing state at block with given descriptor."]
        StateUnavailable(String),

        #[doc = r" I/O terminated unexpectedly"]
        IoTerminated,

        #[doc = r" Unable to schedule wakeup."]
        FaultyTimer(::tokio_timer::Error),

        #[doc = r" Error while working with inherent data."]
        InherentData(String),

        #[doc = r" Unable to propose a block."]
        CannotPropose,

        #[doc = r" Error checking signature"]
        InvalidSignature(Signature, Public),

        #[doc = r" Invalid authorities set received from the runtime."]
        InvalidAuthoritiesSet,

        #[doc = r" Account is not an authority."]
        InvalidAuthority(Public),

        #[doc = r" Authoring interface does not match the runtime."]
        IncompatibleAuthoringRuntime(RuntimeVersion, RuntimeVersion),

        #[doc = r" Authoring interface does not match the runtime."]
        RuntimeVersionMissing,

        #[doc = r" Authoring interface does not match the runtime."]
        NativeRuntimeMissing,

        #[doc = r" Justification requirements not met."]
        InvalidJustification,

        #[doc = r" Some other error."]
        Other(Box<::std::error::Error + Send>),

        #[doc = r" Error from the client while importing"]
        ClientImport(String),

        #[doc = r" Error from the client while importing"]
        ChainLookup(String),

        #[doc(hidden)]
        __Nonexhaustive {
        },
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ErrorKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&ErrorKind::Msg(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Msg");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::StateUnavailable(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("StateUnavailable");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::IoTerminated,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("IoTerminated");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::FaultyTimer(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("FaultyTimer");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::InherentData(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InherentData");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::CannotPropose,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("CannotPropose");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::InvalidSignature(ref __self_0, ref __self_1),) =>
                {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidSignature");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::InvalidAuthoritiesSet,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidAuthoritiesSet");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::InvalidAuthority(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidAuthority");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::IncompatibleAuthoringRuntime(ref __self_0,
                                                          ref __self_1),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("IncompatibleAuthoringRuntime");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::RuntimeVersionMissing,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("RuntimeVersionMissing");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::NativeRuntimeMissing,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NativeRuntimeMissing");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::InvalidJustification,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidJustification");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Other(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Other");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::ClientImport(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ClientImport");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::ChainLookup(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ChainLookup");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::__Nonexhaustive {  },) => {
                    let mut debug_trait_builder =
                        f.debug_struct("__Nonexhaustive");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(unknown_lints,
            unused,
            renamed_and_removed_lints,
            unused_doc_comment,
            unused_doc_comments)]
    impl ::std::fmt::Display for ErrorKind {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self
                {
                 #[doc = r" A convenient variant for String."]
                 ErrorKind::Msg(ref s) => {
                     let display_fn =
                         |_, f: &mut ::std::fmt::Formatter|
                             {
                                 f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                           &match (&s,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }))
                             };
                     display_fn(self, fmt)
                 }
                  #[doc = r" Missing state at block with given descriptor."]
                  ErrorKind::StateUnavailable(ref b) => {
                      let display_fn =
                          |_, f: &mut ::std::fmt::Formatter|
                              {
                                  f.write_fmt(::std::fmt::Arguments::new_v1(&["State unavailable at block "],
                                                                            &match (&b,)
                                                                                 {
                                                                                 (arg0,)
                                                                                 =>
                                                                                 [::std::fmt::ArgumentV1::new(arg0,
                                                                                                              ::std::fmt::Display::fmt)],
                                                                             }))
                              };
                      display_fn(self, fmt)
                  }
                   #[doc = r" I/O terminated unexpectedly"]
                   ErrorKind::IoTerminated => {
                       let display_fn =
                           |_, f: &mut ::std::fmt::Formatter|
                               {
                                   f.write_fmt(::std::fmt::Arguments::new_v1(&["I/O terminated unexpectedly."],
                                                                             &match ()
                                                                                  {
                                                                                  ()
                                                                                  =>
                                                                                  [],
                                                                              }))
                               };
                       display_fn(self, fmt)
                   }
                    #[doc = r" Unable to schedule wakeup."]
                    ErrorKind::FaultyTimer(ref e) => {
                        let display_fn =
                            |_, f: &mut ::std::fmt::Formatter|
                                {
                                    f.write_fmt(::std::fmt::Arguments::new_v1(&["Timer error: "],
                                                                              &match (&e,)
                                                                                   {
                                                                                   (arg0,)
                                                                                   =>
                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                ::std::fmt::Display::fmt)],
                                                                               }))
                                };
                        display_fn(self, fmt)
                    }
                     #[doc = r" Error while working with inherent data."]
                     ErrorKind::InherentData(ref e) => {
                         let display_fn =
                             |_, f: &mut ::std::fmt::Formatter|
                                 {
                                     f.write_fmt(::std::fmt::Arguments::new_v1(&["InherentData error: "],
                                                                               &match (&e,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                }))
                                 };
                         display_fn(self, fmt)
                     }
                      #[doc = r" Unable to propose a block."]
                      ErrorKind::CannotPropose => {
                          let display_fn =
                              |_, f: &mut ::std::fmt::Formatter|
                                  {
                                      f.write_fmt(::std::fmt::Arguments::new_v1(&["Unable to create block proposal."],
                                                                                &match ()
                                                                                     {
                                                                                     ()
                                                                                     =>
                                                                                     [],
                                                                                 }))
                                  };
                          display_fn(self, fmt)
                      }
                       #[doc = r" Error checking signature"]
                       ErrorKind::InvalidSignature(ref s, ref a) => {
                           let display_fn =
                               |_, f: &mut ::std::fmt::Formatter|
                                   {
                                       f.write_fmt(::std::fmt::Arguments::new_v1(&["Message signature ",
                                                                                   " by ",
                                                                                   " is invalid."],
                                                                                 &match (&s,
                                                                                         &a)
                                                                                      {
                                                                                      (arg0,
                                                                                       arg1)
                                                                                      =>
                                                                                      [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                   ::std::fmt::Debug::fmt),
                                                                                       ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                   ::std::fmt::Debug::fmt)],
                                                                                  }))
                                   };
                           display_fn(self, fmt)
                       }
                        #[doc =
                              r" Invalid authorities set received from the runtime."]
                        ErrorKind::InvalidAuthoritiesSet => {
                            let display_fn =
                                |_, f: &mut ::std::fmt::Formatter|
                                    {
                                        f.write_fmt(::std::fmt::Arguments::new_v1(&["Current state of blockchain has invalid authorities set"],
                                                                                  &match ()
                                                                                       {
                                                                                       ()
                                                                                       =>
                                                                                       [],
                                                                                   }))
                                    };
                            display_fn(self, fmt)
                        }
                         #[doc = r" Account is not an authority."]
                         ErrorKind::InvalidAuthority(ref a) => {
                             let display_fn =
                                 |_, f: &mut ::std::fmt::Formatter|
                                     {
                                         f.write_fmt(::std::fmt::Arguments::new_v1(&["Message sender ",
                                                                                     " is not a valid authority."],
                                                                                   &match (&a,)
                                                                                        {
                                                                                        (arg0,)
                                                                                        =>
                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                    }))
                                     };
                             display_fn(self, fmt)
                         }
                          #[doc =
                                r" Authoring interface does not match the runtime."]
                          ErrorKind::IncompatibleAuthoringRuntime(ref native,
                                                                  ref on_chain)
                          => {
                              let display_fn =
                                  |_, f: &mut ::std::fmt::Formatter|
                                      {
                                          f.write_fmt(::std::fmt::Arguments::new_v1(&["Authoring for current runtime is not supported. Native (",
                                                                                      ") cannot author for on-chain (",
                                                                                      ")."],
                                                                                    &match (&native,
                                                                                            &on_chain)
                                                                                         {
                                                                                         (arg0,
                                                                                          arg1)
                                                                                         =>
                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                      ::std::fmt::Display::fmt),
                                                                                          ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                      ::std::fmt::Display::fmt)],
                                                                                     }))
                                      };
                              display_fn(self, fmt)
                          }
                           #[doc =
                                 r" Authoring interface does not match the runtime."]
                           ErrorKind::RuntimeVersionMissing => {
                               let display_fn =
                                   |_, f: &mut ::std::fmt::Formatter|
                                       {
                                           f.write_fmt(::std::fmt::Arguments::new_v1(&["Authoring for current runtime is not supported since it has no version."],
                                                                                     &match ()
                                                                                          {
                                                                                          ()
                                                                                          =>
                                                                                          [],
                                                                                      }))
                                       };
                               display_fn(self, fmt)
                           }
                            #[doc =
                                  r" Authoring interface does not match the runtime."]
                            ErrorKind::NativeRuntimeMissing => {
                                let display_fn =
                                    |_, f: &mut ::std::fmt::Formatter|
                                        {
                                            f.write_fmt(::std::fmt::Arguments::new_v1(&["Authoring in current build is not supported since it has no runtime."],
                                                                                      &match ()
                                                                                           {
                                                                                           ()
                                                                                           =>
                                                                                           [],
                                                                                       }))
                                        };
                                display_fn(self, fmt)
                            }
                             #[doc = r" Justification requirements not met."]
                             ErrorKind::InvalidJustification => {
                                 let display_fn =
                                     |_, f: &mut ::std::fmt::Formatter|
                                         {
                                             f.write_fmt(::std::fmt::Arguments::new_v1(&["Invalid justification."],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        }))
                                         };
                                 display_fn(self, fmt)
                             }
                              #[doc = r" Some other error."]
                              ErrorKind::Other(ref e) => {
                                  let display_fn =
                                      |_, f: &mut ::std::fmt::Formatter|
                                          {
                                              f.write_fmt(::std::fmt::Arguments::new_v1(&["Other error: "],
                                                                                        &match (&e.description(),)
                                                                                             {
                                                                                             (arg0,)
                                                                                             =>
                                                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                          ::std::fmt::Display::fmt)],
                                                                                         }))
                                          };
                                  display_fn(self, fmt)
                              }
                               #[doc =
                                     r" Error from the client while importing"]
                               ErrorKind::ClientImport(ref reason) => {
                                   let display_fn =
                                       |_, f: &mut ::std::fmt::Formatter|
                                           {
                                               f.write_fmt(::std::fmt::Arguments::new_v1(&["Import failed: "],
                                                                                         &match (&reason,)
                                                                                              {
                                                                                              (arg0,)
                                                                                              =>
                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                           ::std::fmt::Display::fmt)],
                                                                                          }))
                                           };
                                   display_fn(self, fmt)
                               }
                                #[doc =
                                      r" Error from the client while importing"]
                                ErrorKind::ChainLookup(ref reason) => {
                                    let display_fn =
                                        |_, f: &mut ::std::fmt::Formatter|
                                            {
                                                f.write_fmt(::std::fmt::Arguments::new_v1(&["Chain lookup failed: "],
                                                                                          &match (&reason,)
                                                                                               {
                                                                                               (arg0,)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Display::fmt)],
                                                                                           }))
                                            };
                                    display_fn(self, fmt)
                                }
                _ => Ok(()),
            }
        }
    }
    #[allow(unknown_lints,
            unused,
            renamed_and_removed_lints,
            unused_doc_comment,
            unused_doc_comments)]
    impl ErrorKind {
        /// A string describing the error kind.
        pub fn description(&self) -> &str {
            match *self
                {
                 #[doc = r" A convenient variant for String."]
                 ErrorKind::Msg(ref s) => {
                     &s
                 }
                  #[doc = r" Missing state at block with given descriptor."]
                  ErrorKind::StateUnavailable(ref b) => {
                      "State missing at given block."
                  }
                   #[doc = r" I/O terminated unexpectedly"]
                   ErrorKind::IoTerminated => {
                       "I/O terminated unexpectedly."
                   }
                    #[doc = r" Unable to schedule wakeup."]
                    ErrorKind::FaultyTimer(ref e) => {
                        "Timer error"
                    }
                     #[doc = r" Error while working with inherent data."]
                     ErrorKind::InherentData(ref e) => {
                         "InherentData error"
                     }
                      #[doc = r" Unable to propose a block."]
                      ErrorKind::CannotPropose => {
                          "Unable to create block proposal."
                      }
                       #[doc = r" Error checking signature"]
                       ErrorKind::InvalidSignature(ref s, ref a) => {
                           "Message signature is invalid"
                       }
                        #[doc =
                              r" Invalid authorities set received from the runtime."]
                        ErrorKind::InvalidAuthoritiesSet => {
                            "authorities set is invalid"
                        }
                         #[doc = r" Account is not an authority."]
                         ErrorKind::InvalidAuthority(ref a) => {
                             "Message sender is not a valid authority"
                         }
                          #[doc =
                                r" Authoring interface does not match the runtime."]
                          ErrorKind::IncompatibleAuthoringRuntime(ref native,
                                                                  ref on_chain)
                          => {
                              "Authoring for current runtime is not supported"
                          }
                           #[doc =
                                 r" Authoring interface does not match the runtime."]
                           ErrorKind::RuntimeVersionMissing => {
                               "Current runtime has no version"
                           }
                            #[doc =
                                  r" Authoring interface does not match the runtime."]
                            ErrorKind::NativeRuntimeMissing => {
                                "This build has no native runtime"
                            }
                             #[doc = r" Justification requirements not met."]
                             ErrorKind::InvalidJustification => {
                                 "Invalid justification"
                             }
                              #[doc = r" Some other error."]
                              ErrorKind::Other(ref e) => {
                                  "Other error"
                              }
                               #[doc =
                                     r" Error from the client while importing"]
                               ErrorKind::ClientImport(ref reason) => {
                                   "Import failed"
                               }
                                #[doc =
                                      r" Error from the client while importing"]
                                ErrorKind::ChainLookup(ref reason) => {
                                    "Looking up chain failed"
                                }
                _ => "",
            }
        }
    }
    impl <'a> From<&'a str> for ErrorKind {
        fn from(s: &'a str) -> Self { ErrorKind::Msg(s.to_string()) }
    }
    impl From<String> for ErrorKind {
        fn from(s: String) -> Self { ErrorKind::Msg(s) }
    }
    impl From<Error> for ErrorKind {
        fn from(e: Error) -> Self { e.0 }
    }
    /// Additional methods for `Result`, for easy interaction with this crate.
    pub trait ResultExt<T> {
        /// If the `Result` is an `Err` then `chain_err` evaluates the closure,
        /// which returns *some type that can be converted to `ErrorKind`*, boxes
        /// the original error to store as the cause, then returns a new error
        /// containing the original error.
        fn chain_err<F, EK>(self, callback: F)
        -> ::std::result::Result<T, Error>
        where
        F: FnOnce()
        ->
        EK,
        EK: Into<ErrorKind>;
    }
    impl <T, E> ResultExt<T> for ::std::result::Result<T, E> where
     E: ::std::error::Error + Send + 'static {
        fn chain_err<F, EK>(self, callback: F)
         -> ::std::result::Result<T, Error> where F: FnOnce() -> EK,
         EK: Into<ErrorKind> {
            self.map_err(move |e|
                             {
                                 let state =
                                     ::error_chain::State::new::<Error>(Box::new(e));
                                 ::error_chain::ChainedError::new(callback().into(),
                                                                  state)
                             })
        }
    }
    impl <T> ResultExt<T> for ::std::option::Option<T> {
        fn chain_err<F, EK>(self, callback: F)
         -> ::std::result::Result<T, Error> where F: FnOnce() -> EK,
         EK: Into<ErrorKind> {
            self.ok_or_else(move ||
                                {
                                    ::error_chain::ChainedError::from_kind(callback().into())
                                })
        }
    }
    /// Convenient wrapper around `std::Result`.
    #[allow(unused)]
    pub type Result<T> = ::std::result::Result<T, Error>;
}
mod block_import {
    //! Block import helpers.
    use runtime_primitives::traits::{Block as BlockT, DigestItemFor, Header as
                                     HeaderT, NumberFor};
    use runtime_primitives::Justification;
    use std::borrow::Cow;
    use std::collections::HashMap;
    use crate::well_known_cache_keys;
    use crate::import_queue::Verifier;
    /// Block import result.
    #[structural_match]
    pub enum ImportResult {

        /// Block imported.
        Imported(ImportedAux),

        /// Already in the blockchain.
        AlreadyInChain,

        /// Block or parent is known to be bad.
        KnownBad,

        /// Block parent is not in the chain.
        UnknownParent,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ImportResult {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&ImportResult::Imported(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Imported");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ImportResult::AlreadyInChain,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("AlreadyInChain");
                    debug_trait_builder.finish()
                }
                (&ImportResult::KnownBad,) => {
                    let mut debug_trait_builder = f.debug_tuple("KnownBad");
                    debug_trait_builder.finish()
                }
                (&ImportResult::UnknownParent,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UnknownParent");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ImportResult {
        #[inline]
        fn eq(&self, other: &ImportResult) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ImportResult::Imported(ref __self_0),
                         &ImportResult::Imported(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => true,
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &ImportResult) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ImportResult::Imported(ref __self_0),
                         &ImportResult::Imported(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => false,
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ImportResult {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<ImportedAux>; }
        }
    }
    /// Auxiliary data associated with an imported block result.
    #[structural_match]
    pub struct ImportedAux {
        /// Clear all pending justification requests.
        pub clear_justification_requests: bool,
        /// Request a justification for the given block.
        pub needs_justification: bool,
        /// Received a bad justification.
        pub bad_justification: bool,
        /// Request a finality proof for the given block.
        pub needs_finality_proof: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ImportedAux {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ImportedAux {
                clear_justification_requests: ref __self_0_0,
                needs_justification: ref __self_0_1,
                bad_justification: ref __self_0_2,
                needs_finality_proof: ref __self_0_3 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("ImportedAux");
                    let _ =
                        debug_trait_builder.field("clear_justification_requests",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("needs_justification",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("bad_justification",
                                                  &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("needs_finality_proof",
                                                  &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ImportedAux {
        #[inline]
        fn eq(&self, other: &ImportedAux) -> bool {
            match *other {
                ImportedAux {
                clear_justification_requests: ref __self_1_0,
                needs_justification: ref __self_1_1,
                bad_justification: ref __self_1_2,
                needs_finality_proof: ref __self_1_3 } =>
                match *self {
                    ImportedAux {
                    clear_justification_requests: ref __self_0_0,
                    needs_justification: ref __self_0_1,
                    bad_justification: ref __self_0_2,
                    needs_finality_proof: ref __self_0_3 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ImportedAux) -> bool {
            match *other {
                ImportedAux {
                clear_justification_requests: ref __self_1_0,
                needs_justification: ref __self_1_1,
                bad_justification: ref __self_1_2,
                needs_finality_proof: ref __self_1_3 } =>
                match *self {
                    ImportedAux {
                    clear_justification_requests: ref __self_0_0,
                    needs_justification: ref __self_0_1,
                    bad_justification: ref __self_0_2,
                    needs_finality_proof: ref __self_0_3 } =>
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
    impl ::std::cmp::Eq for ImportedAux {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
                let _: ::std::cmp::AssertParamIsEq<bool>;
            }
        }
    }
    impl Default for ImportedAux {
        fn default() -> ImportedAux {
            ImportedAux{clear_justification_requests: false,
                        needs_justification: false,
                        bad_justification: false,
                        needs_finality_proof: false,}
        }
    }
    impl ImportResult {
        /// Returns default value for `ImportResult::Imported` with both
        /// `clear_justification_requests` and `needs_justification` set to false.
        pub fn imported() -> ImportResult {
            ImportResult::Imported(ImportedAux::default())
        }
    }
    /// Block data origin.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum BlockOrigin {

        /// Genesis block built into the client.
        Genesis,

        /// Block is part of the initial sync with the network.
        NetworkInitialSync,

        /// Block was broadcasted on the network.
        NetworkBroadcast,

        /// Block that was received from the network and validated in the consensus process.
        ConsensusBroadcast,

        /// Block that was collated by this node.
        Own,

        /// Block was imported from a file.
        File,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlockOrigin {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&BlockOrigin::Genesis,) => {
                    let mut debug_trait_builder = f.debug_tuple("Genesis");
                    debug_trait_builder.finish()
                }
                (&BlockOrigin::NetworkInitialSync,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NetworkInitialSync");
                    debug_trait_builder.finish()
                }
                (&BlockOrigin::NetworkBroadcast,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NetworkBroadcast");
                    debug_trait_builder.finish()
                }
                (&BlockOrigin::ConsensusBroadcast,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ConsensusBroadcast");
                    debug_trait_builder.finish()
                }
                (&BlockOrigin::Own,) => {
                    let mut debug_trait_builder = f.debug_tuple("Own");
                    debug_trait_builder.finish()
                }
                (&BlockOrigin::File,) => {
                    let mut debug_trait_builder = f.debug_tuple("File");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlockOrigin {
        #[inline]
        fn eq(&self, other: &BlockOrigin) -> bool {
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
    impl ::std::cmp::Eq for BlockOrigin {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BlockOrigin {
        #[inline]
        fn clone(&self) -> BlockOrigin { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for BlockOrigin { }
    /// Fork choice strategy.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum ForkChoiceStrategy {

        /// Longest chain fork choice.
        LongestChain,

        /// Custom fork choice rule, where true indicates the new block should be the best block.
        Custom(bool),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ForkChoiceStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&ForkChoiceStrategy::LongestChain,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("LongestChain");
                    debug_trait_builder.finish()
                }
                (&ForkChoiceStrategy::Custom(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Custom");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ForkChoiceStrategy {
        #[inline]
        fn eq(&self, other: &ForkChoiceStrategy) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ForkChoiceStrategy::Custom(ref __self_0),
                         &ForkChoiceStrategy::Custom(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => true,
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &ForkChoiceStrategy) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ForkChoiceStrategy::Custom(ref __self_0),
                         &ForkChoiceStrategy::Custom(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => false,
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ForkChoiceStrategy {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<bool>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ForkChoiceStrategy {
        #[inline]
        fn clone(&self) -> ForkChoiceStrategy {
            { let _: ::std::clone::AssertParamIsClone<bool>; *self }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for ForkChoiceStrategy { }
    /// Data required to import a Block
    pub struct ImportBlock<Block: BlockT> {
        /// Origin of the Block
        pub origin: BlockOrigin,
        /// The header, without consensus post-digests applied. This should be in the same
        /// state as it comes out of the runtime.
        ///
        /// Consensus engines which alter the header (by adding post-runtime digests)
        /// should strip those off in the initial verification process and pass them
        /// via the `post_digests` field. During block authorship, they should
        /// not be pushed to the header directly.
        ///
        /// The reason for this distinction is so the header can be directly
        /// re-executed in a runtime that checks digest equivalence -- the
        /// post-runtime digests are pushed back on after.
        pub header: Block::Header,
        /// Justification provided for this block from the outside.
        pub justification: Option<Justification>,
        /// Digest items that have been added after the runtime for external
        /// work, like a consensus signature.
        pub post_digests: Vec<DigestItemFor<Block>>,
        /// Block's body
        pub body: Option<Vec<Block::Extrinsic>>,
        /// Is this block finalized already?
        /// `true` implies instant finality.
        pub finalized: bool,
        /// Auxiliary consensus data produced by the block.
        /// Contains a list of key-value pairs. If values are `None`, the keys
        /// will be deleted.
        pub auxiliary: Vec<(Vec<u8>, Option<Vec<u8>>)>,
        /// Fork choice strategy of this import.
        pub fork_choice: ForkChoiceStrategy,
    }
    impl <Block: BlockT> ImportBlock<Block> {
        /// Deconstruct the justified header into parts.
        pub fn into_inner(self)
         ->
             (BlockOrigin, <Block as BlockT>::Header, Option<Justification>,
              Vec<DigestItemFor<Block>>,
              Option<Vec<<Block as BlockT>::Extrinsic>>, bool,
              Vec<(Vec<u8>, Option<Vec<u8>>)>) {
            (self.origin, self.header, self.justification, self.post_digests,
             self.body, self.finalized, self.auxiliary)
        }
        /// Get a handle to full header (with post-digests applied).
        pub fn post_header(&self) -> Cow<Block::Header> {
            use runtime_primitives::traits::Digest;
            if self.post_digests.is_empty() {
                Cow::Borrowed(&self.header)
            } else {
                Cow::Owned({
                               let mut hdr = self.header.clone();
                               for digest_item in &self.post_digests {
                                   hdr.digest_mut().push(digest_item.clone());
                               }
                               hdr
                           })
            }
        }
    }
    /// Block import trait.
    pub trait BlockImport<B: BlockT> {
        type
        Error: ::std::error::Error +
        Send +
        'static;
        /// Check block preconditions.
        fn check_block(&self, hash: B::Hash, parent_hash: B::Hash)
        -> Result<ImportResult, Self::Error>;
        /// Import a block.
        ///
        /// Cached data can be accessed through the blockchain cache.
        fn import_block(&self, block: ImportBlock<B>,
                        cache: HashMap<well_known_cache_keys::Id, Vec<u8>>)
        -> Result<ImportResult, Self::Error>;
    }
    /// Justification import trait
    pub trait JustificationImport<B: BlockT> {
        type
        Error: ::std::error::Error +
        Send +
        'static;
        /// Called by the import queue when it is started.
        fn on_start(&self, _link: &crate::import_queue::Link<B>) { }
        /// Import a Block justification and finalize the given block.
        fn import_justification(&self, hash: B::Hash, number: NumberFor<B>,
                                justification: Justification)
        -> Result<(), Self::Error>;
    }
    /// Finality proof import trait.
    pub trait FinalityProofImport<B: BlockT> {
        type
        Error: ::std::error::Error +
        Send +
        'static;
        /// Called by the import queue when it is started.
        fn on_start(&self, _link: &crate::import_queue::Link<B>) { }
        /// Import a Block justification and finalize the given block. Returns finalized block or error.
        fn import_finality_proof(&self, hash: B::Hash, number: NumberFor<B>,
                                 finality_proof: Vec<u8>,
                                 verifier: &Verifier<B>)
        -> Result<(B::Hash, NumberFor<B>), Self::Error>;
    }
    /// Finality proof request builder.
    pub trait FinalityProofRequestBuilder<B: BlockT>: Send {
        /// Build data blob, associated with the request.
        fn build_request_data(&self, hash: &B::Hash)
        -> Vec<u8>;
    }
}
mod select_chain {
    use crate::error::Error;
    use runtime_primitives::traits::{Block as BlockT, NumberFor};
    /// The SelectChain trait defines the strategy upon which the head is chosen
    /// if multiple forks are present for an opaque definition of "best" in the 
    /// specific chain build.
    ///
    /// The Strategy can be customised for the two use cases of authoring new blocks
    /// upon the best chain or which fork to finalise. Unless implemented differently
    /// by default finalisation methods fall back to use authoring, so as a minimum
    /// `_authoring`-functions must be implemented. 
    ///
    /// Any particular user must make explicit, however, whether they intend to finalise
    /// or author through the using the right function call, as these might differ in
    /// some implementations.
    ///
    /// Non-deterministicly finalising chains may only use the `_authoring` functions.
    pub trait SelectChain<Block: BlockT>: Sync + Send + Clone {
        /// Get all leaves of the chain: block hashes that have no children currently.
        /// Leaves that can never be finalized will not be returned.
        fn leaves(&self)
        -> Result<Vec<<Block as BlockT>::Hash>, Error>;
        /// Among those `leaves` deterministically pick one chain as the generally
        /// best chain to author new blocks upon and probably finalize.
        fn best_chain(&self)
        -> Result<<Block as BlockT>::Header, Error>;
        /// Get the best ancestor of `target_hash` that we should attempt
        /// to finalize next.
        fn finality_target(&self, target_hash: <Block as BlockT>::Hash,
                           _maybe_max_number: Option<NumberFor<Block>>)
         -> Result<Option<<Block as BlockT>::Hash>, Error> {
            Ok(Some(target_hash))
        }
    }
}
pub mod import_queue {
    //! Import Queue primitive: something which can verify and import blocks.
    //!
    //! This serves as an intermediate and abstracted step between synchronization
    //! and import. Each mode of consensus will have its own requirements for block
    //! verification. Some algorithms can verify in parallel, while others only
    //! sequentially.
    //!
    //! The `ImportQueue` trait allows such verification strategies to be
    //! instantiated. The `BasicQueue` and `BasicVerifier` traits allow serial
    //! queues to be instantiated simply.
    use crate::block_import::{BlockImport, BlockOrigin, ImportBlock,
                              ImportedAux, ImportResult, JustificationImport,
                              FinalityProofImport,
                              FinalityProofRequestBuilder};
    use crossbeam_channel::{self as channel, Receiver, Sender};
    use parity_codec::Encode;
    use std::sync::Arc;
    use std::thread;
    use runtime_primitives::traits::{AuthorityIdFor, Block as BlockT, Header
                                     as HeaderT, NumberFor};
    use runtime_primitives::Justification;
    use crate::error::Error as ConsensusError;
    use parity_codec::alloc::collections::hash_map::HashMap;
    /// Reputation change for peers which send us a block with an incomplete header.
    const INCOMPLETE_HEADER_REPUTATION_CHANGE: i32 = -(1 << 20);
    /// Reputation change for peers which send us a block which we fail to verify.
    const VERIFICATION_FAIL_REPUTATION_CHANGE: i32 = -(1 << 20);
    /// Reputation change for peers which send us a bad block.
    const BAD_BLOCK_REPUTATION_CHANGE: i32 = -(1 << 29);
    /// Reputation change for peers which send us a block with bad justifications.
    const BAD_JUSTIFICATION_REPUTATION_CHANGE: i32 = -(1 << 16);
    /// Shared block import struct used by the queue.
    pub type SharedBlockImport<B>
        =
        Arc<dyn BlockImport<B, Error = ConsensusError> + Send + Sync>;
    /// Shared justification import struct used by the queue.
    pub type SharedJustificationImport<B>
        =
        Arc<dyn JustificationImport<B, Error = ConsensusError> + Send + Sync>;
    /// Shared finality proof import struct used by the queue.
    pub type SharedFinalityProofImport<B>
        =
        Arc<dyn FinalityProofImport<B, Error = ConsensusError> + Send + Sync>;
    /// Shared finality proof request builder struct used by the queue.
    pub type SharedFinalityProofRequestBuilder<B>
        =
        Arc<dyn FinalityProofRequestBuilder<B> + Send + Sync>;
    /// Maps to the Origin used by the network.
    pub type Origin = libp2p::PeerId;
    /// Block data used by the queue.
    #[structural_match]
    pub struct IncomingBlock<B: BlockT> {
        /// Block header hash.
        pub hash: <B as BlockT>::Hash,
        /// Block header if requested.
        pub header: Option<<B as BlockT>::Header>,
        /// Block body if requested.
        pub body: Option<Vec<<B as BlockT>::Extrinsic>>,
        /// Justification if requested.
        pub justification: Option<Justification>,
        /// The peer, we received this from
        pub origin: Option<Origin>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <B: ::std::fmt::Debug + BlockT> ::std::fmt::Debug for
     IncomingBlock<B> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                IncomingBlock {
                hash: ref __self_0_0,
                header: ref __self_0_1,
                body: ref __self_0_2,
                justification: ref __self_0_3,
                origin: ref __self_0_4 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("IncomingBlock");
                    let _ =
                        debug_trait_builder.field("hash", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("header", &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("body", &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("justification",
                                                  &&(*__self_0_3));
                    let _ =
                        debug_trait_builder.field("origin", &&(*__self_0_4));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <B: ::std::cmp::PartialEq + BlockT> ::std::cmp::PartialEq for
     IncomingBlock<B> {
        #[inline]
        fn eq(&self, other: &IncomingBlock<B>) -> bool {
            match *other {
                IncomingBlock {
                hash: ref __self_1_0,
                header: ref __self_1_1,
                body: ref __self_1_2,
                justification: ref __self_1_3,
                origin: ref __self_1_4 } =>
                match *self {
                    IncomingBlock {
                    hash: ref __self_0_0,
                    header: ref __self_0_1,
                    body: ref __self_0_2,
                    justification: ref __self_0_3,
                    origin: ref __self_0_4 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3) &&
                        (*__self_0_4) == (*__self_1_4),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &IncomingBlock<B>) -> bool {
            match *other {
                IncomingBlock {
                hash: ref __self_1_0,
                header: ref __self_1_1,
                body: ref __self_1_2,
                justification: ref __self_1_3,
                origin: ref __self_1_4 } =>
                match *self {
                    IncomingBlock {
                    hash: ref __self_0_0,
                    header: ref __self_0_1,
                    body: ref __self_0_2,
                    justification: ref __self_0_3,
                    origin: ref __self_0_4 } =>
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
    impl <B: ::std::cmp::Eq + BlockT> ::std::cmp::Eq for IncomingBlock<B> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<<B as BlockT>::Hash>;
                let _:
                        ::std::cmp::AssertParamIsEq<Option<<B as
                                                           BlockT>::Header>>;
                let _:
                        ::std::cmp::AssertParamIsEq<Option<Vec<<B as
                                                               BlockT>::Extrinsic>>>;
                let _: ::std::cmp::AssertParamIsEq<Option<Justification>>;
                let _: ::std::cmp::AssertParamIsEq<Option<Origin>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <B: ::std::clone::Clone + BlockT> ::std::clone::Clone for
     IncomingBlock<B> {
        #[inline]
        fn clone(&self) -> IncomingBlock<B> {
            match *self {
                IncomingBlock {
                hash: ref __self_0_0,
                header: ref __self_0_1,
                body: ref __self_0_2,
                justification: ref __self_0_3,
                origin: ref __self_0_4 } =>
                IncomingBlock{hash:
                                  ::std::clone::Clone::clone(&(*__self_0_0)),
                              header:
                                  ::std::clone::Clone::clone(&(*__self_0_1)),
                              body:
                                  ::std::clone::Clone::clone(&(*__self_0_2)),
                              justification:
                                  ::std::clone::Clone::clone(&(*__self_0_3)),
                              origin:
                                  ::std::clone::Clone::clone(&(*__self_0_4)),},
            }
        }
    }
    /// Verify a justification of a block
    pub trait Verifier<B: BlockT>: Send + Sync {
        /// Verify the given data and return the ImportBlock and an optional
        /// new set of validators to import. If not, err with an Error-Message
        /// presented to the User in the logs.
        fn verify(&self, origin: BlockOrigin, header: B::Header,
                  justification: Option<Justification>,
                  body: Option<Vec<B::Extrinsic>>)
        -> Result<(ImportBlock<B>, Option<Vec<AuthorityIdFor<B>>>), String>;
    }
    /// Blocks import queue API.
    pub trait ImportQueue<B: BlockT>: Send + Sync + ImportQueueClone<B> {
        /// Start background work for the queue as necessary.
        ///
        /// This is called automatically by the network service when synchronization
        /// begins.
        fn start(&self, _link: Box<Link<B>>) -> Result<(), std::io::Error> {
            Ok(())
        }
        /// Clears the import queue and stops importing.
        fn stop(&self);
        /// Import bunch of blocks.
        fn import_blocks(&self, origin: BlockOrigin,
                         blocks: Vec<IncomingBlock<B>>);
        /// Import a block justification.
        fn import_justification(&self, who: Origin, hash: B::Hash,
                                number: NumberFor<B>,
                                justification: Justification);
        /// Import block finality proof.
        fn import_finality_proof(&self, who: Origin, hash: B::Hash,
                                 number: NumberFor<B>,
                                 finality_proof: Vec<u8>);
    }
    pub trait ImportQueueClone<B: BlockT> {
        fn clone_box(&self)
        -> Box<ImportQueue<B>>;
    }
    impl <B: BlockT> Clone for Box<ImportQueue<B>> {
        fn clone(&self) -> Box<ImportQueue<B>> { self.clone_box() }
    }
    /// Interface to a basic block import queue that is importing blocks
    /// sequentially in a separate thread, with pluggable verification.
    pub struct BasicQueue<B: BlockT> {
        sender: Sender<BlockImportMsg<B>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <B: ::std::clone::Clone + BlockT> ::std::clone::Clone for
     BasicQueue<B> {
        #[inline]
        fn clone(&self) -> BasicQueue<B> {
            match *self {
                BasicQueue { sender: ref __self_0_0 } =>
                BasicQueue{sender:
                               ::std::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    impl <B: BlockT> ImportQueueClone<B> for BasicQueue<B> {
        fn clone_box(&self) -> Box<ImportQueue<B>> { Box::new(self.clone()) }
    }
    /// "BasicQueue" is a wrapper around a channel sender to the "BlockImporter".
    /// "BasicQueue" itself does not keep any state or do any importing work, and
    /// can therefore be send to other threads.
    ///
    /// "BasicQueue" implements "ImportQueue" by sending messages to the
    /// "BlockImporter", which runs in it's own thread.
    ///
    /// The "BlockImporter" is responsible for handling incoming requests from the
    /// "BasicQueue". Some of these requests are handled by the "BlockImporter"
    /// itself, such as "is_importing", "status", and justifications.
    ///
    /// The "import block" work will be offloaded to a single "BlockImportWorker",
    /// running in another thread. Offloading the work is done via a channel,
    /// ensuring blocks in this implementation are imported sequentially and in
    /// order (as received by the "BlockImporter").
    ///
    /// As long as the "BasicQueue" is not dropped, the "BlockImporter" will keep
    /// running. The "BlockImporter" owns a sender to the "BlockImportWorker",
    /// ensuring that the worker is kept alive until that sender is dropped.
    impl <B: BlockT> BasicQueue<B> {
        /// Instantiate a new basic queue, with given verifier.
        pub fn new<V: 'static +
                   Verifier<B>>(verifier: Arc<V>,
                                block_import: SharedBlockImport<B>,
                                justification_import:
                                    Option<SharedJustificationImport<B>>,
                                finality_proof_import:
                                    Option<SharedFinalityProofImport<B>>,
                                finality_proof_request_builder:
                                    Option<SharedFinalityProofRequestBuilder<B>>)
         -> Self {
            let (result_sender, result_port) = channel::unbounded();
            let worker_sender =
                BlockImportWorker::new(result_sender, verifier.clone(),
                                       block_import,
                                       finality_proof_import.clone());
            let importer_sender =
                BlockImporter::new(result_port, worker_sender, verifier,
                                   justification_import,
                                   finality_proof_import,
                                   finality_proof_request_builder);
            Self{sender: importer_sender,}
        }
    }
    impl <B: BlockT> ImportQueue<B> for BasicQueue<B> {
        fn start(&self, link: Box<Link<B>>) -> Result<(), std::io::Error> {
            let (sender, port) = channel::unbounded();
            let _ =
                self.sender.send(BlockImportMsg::Start(link,
                                                       sender)).expect("1. self is holding a sender to the Importer, 2. Importer should handle messages while there are senders around; qed");
            port.recv().expect("1. self is holding a sender to the Importer, 2. Importer should handle messages while there are senders around; qed")
        }
        fn stop(&self) {
            let _ =
                self.sender.send(BlockImportMsg::Stop).expect("1. self is holding a sender to the Importer, 2. Importer should handle messages while there are senders around; qed");
        }
        fn import_blocks(&self, origin: BlockOrigin,
                         blocks: Vec<IncomingBlock<B>>) {
            if blocks.is_empty() { return; }
            let _ =
                self.sender.send(BlockImportMsg::ImportBlocks(origin,
                                                              blocks)).expect("1. self is holding a sender to the Importer, 2. Importer should handle messages while there are senders around; qed");
        }
        fn import_justification(&self, who: Origin, hash: B::Hash,
                                number: NumberFor<B>,
                                justification: Justification) {
            let _ =
                self.sender.send(BlockImportMsg::ImportJustification(who.clone(),
                                                                     hash,
                                                                     number,
                                                                     justification)).expect("1. self is holding a sender to the Importer, 2. Importer should handle messages while there are senders around; qed");
        }
        fn import_finality_proof(&self, who: Origin, hash: B::Hash,
                                 number: NumberFor<B>,
                                 finality_proof: Vec<u8>) {
            let _ =
                self.sender.send(BlockImportMsg::ImportFinalityProof(who,
                                                                     hash,
                                                                     number,
                                                                     finality_proof)).expect("1. self is holding a sender to the Importer, 2. Importer should handle messages while there are senders around; qed");
        }
    }
    pub enum BlockImportMsg<B: BlockT> {
        ImportBlocks(BlockOrigin, Vec<IncomingBlock<B>>),
        ImportJustification(Origin, B::Hash, NumberFor<B>, Justification),
        ImportFinalityProof(Origin, B::Hash, NumberFor<B>, Vec<u8>),
        Start(Box<Link<B>>, Sender<Result<(), std::io::Error>>),
        Stop,
    }
    pub enum BlockImportWorkerMsg<B: BlockT> {
        ImportBlocks(BlockOrigin, Vec<IncomingBlock<B>>),
        ImportedBlocks(Vec<(Result<BlockImportResult<NumberFor<B>>,
                                   BlockImportError>, B::Hash)>),
        ImportFinalityProof(Origin, B::Hash, NumberFor<B>, Vec<u8>),
        ImportedFinalityProof(Origin, (B::Hash, NumberFor<B>),
                              Result<(B::Hash, NumberFor<B>), ()>),
    }
    enum ImportMsgType<B: BlockT> {
        FromWorker(BlockImportWorkerMsg<B>),
        FromNetwork(BlockImportMsg<B>),
    }
    struct BlockImporter<B: BlockT> {
        port: Receiver<BlockImportMsg<B>>,
        result_port: Receiver<BlockImportWorkerMsg<B>>,
        worker_sender: Sender<BlockImportWorkerMsg<B>>,
        link: Option<Box<dyn Link<B>>>,
        verifier: Arc<Verifier<B>>,
        justification_import: Option<SharedJustificationImport<B>>,
        finality_proof_import: Option<SharedFinalityProofImport<B>>,
        finality_proof_request_builder: Option<SharedFinalityProofRequestBuilder<B>>,
    }
    impl <B: BlockT> BlockImporter<B> {
        fn new(result_port: Receiver<BlockImportWorkerMsg<B>>,
               worker_sender: Sender<BlockImportWorkerMsg<B>>,
               verifier: Arc<Verifier<B>>,
               justification_import: Option<SharedJustificationImport<B>>,
               finality_proof_import: Option<SharedFinalityProofImport<B>>,
               finality_proof_request_builder:
                   Option<SharedFinalityProofRequestBuilder<B>>)
         -> Sender<BlockImportMsg<B>> {
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Creating new Block Importer!"],
                                                                           &match ()
                                                                                {
                                                                                ()
                                                                                =>
                                                                                [],
                                                                            }),
                                             lvl,
                                             &("block_import",
                                               "substrate_consensus_common::import_queue",
                                               "core/consensus/common/src/import_queue.rs",
                                               296u32));
                }
            };
            let (sender, port) = channel::bounded(4);
            let _ =
                thread::Builder::new().name("ImportQueue".into()).spawn(move
                                                                            ||
                                                                            {
                                                                                let mut importer =
                                                                                    BlockImporter{port,
                                                                                                  result_port,
                                                                                                  worker_sender,
                                                                                                  link:
                                                                                                      None,
                                                                                                  verifier,
                                                                                                  justification_import,
                                                                                                  finality_proof_import,
                                                                                                  finality_proof_request_builder,};
                                                                                while importer.run()
                                                                                      {
                                                                                }
                                                                            }).expect("ImportQueue thread spawning failed");
            sender
        }
        fn run(&mut self) -> bool {
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Running import queue"],
                                                                           &match ()
                                                                                {
                                                                                ()
                                                                                =>
                                                                                [],
                                                                            }),
                                             lvl,
                                             &("import_queue",
                                               "substrate_consensus_common::import_queue",
                                               "core/consensus/common/src/import_queue.rs",
                                               320u32));
                }
            };
            let msg =
                {
                    #[allow(unused_mut)]
                    let mut _sel = ::crossbeam_channel::Select::new();
                    {
                        match self.port {
                            ref _r => {
                                #[allow(unsafe_code)]
                                let _oper0:
                                        &::crossbeam_channel::Receiver<_> =
                                    unsafe {
                                        let _r:
                                                &::crossbeam_channel::Receiver<_> =
                                            _r;
                                        unsafe fn unbind<'a, T>(x: &T)
                                         -> &'a T {
                                            ::std::mem::transmute(x)
                                        }
                                        unbind(_r)
                                    };
                                _sel.recv(_oper0);
                                {
                                    match self.result_port {
                                        ref _r => {
                                            #[allow(unsafe_code)]
                                            let _oper1:
                                                    &::crossbeam_channel::Receiver<_> =
                                                unsafe {
                                                    let _r:
                                                            &::crossbeam_channel::Receiver<_> =
                                                        _r;
                                                    unsafe fn unbind<'a,
                                                                     T>(x: &T)
                                                     -> &'a T {
                                                        ::std::mem::transmute(x)
                                                    }
                                                    unbind(_r)
                                                };
                                            _sel.recv(_oper1);
                                            {
                                                let _oper:
                                                        ::crossbeam_channel::SelectedOperation<'_> =
                                                    {
                                                        let _oper =
                                                            _sel.select();

                                                        #[allow(unsafe_code)]
                                                        unsafe {
                                                            ::std::mem::transmute(_oper)
                                                        }
                                                    };
                                                {
                                                    if _oper.index() == 0usize
                                                       {
                                                        let _res =
                                                            _oper.recv(_oper0);
                                                        ::std::mem::drop(_sel);
                                                        let msg = _res;
                                                        {
                                                            {
                                                                match msg {
                                                                    Err(_) =>
                                                                    return false,
                                                                    Ok(msg) =>
                                                                    ImportMsgType::FromNetwork(msg),
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            if _oper.index()
                                                                   == 1usize {
                                                                let _res =
                                                                    _oper.recv(_oper1);
                                                                ::std::mem::drop(_sel);
                                                                let msg =
                                                                    _res;
                                                                {
                                                                    {
                                                                        match msg
                                                                            {
                                                                            Err(_)
                                                                            =>
                                                                            {
                                                                                {
                                                                                    {
                                                                                        ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                                                                                  &match (&"1. We hold a sender to the Worker, 2. it should not quit until that sender is dropped; qed",)
                                                                                                                                                       {
                                                                                                                                                       (arg0,)
                                                                                                                                                       =>
                                                                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                    ::std::fmt::Display::fmt)],
                                                                                                                                                   }),
                                                                                                                   &("core/consensus/common/src/import_queue.rs",
                                                                                                                     331u32,
                                                                                                                     16u32))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Ok(msg)
                                                                            =>
                                                                            ImportMsgType::FromWorker(msg),
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    {
                                                                        {
                                                                            {
                                                                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                                                                          &match (&"internal error in crossbeam-channel: invalid case",)
                                                                                                                                               {
                                                                                                                                               (arg0,)
                                                                                                                                               =>
                                                                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                            ::std::fmt::Display::fmt)],
                                                                                                                                           }),
                                                                                                           &("core/consensus/common/src/import_queue.rs",
                                                                                                             321u32,
                                                                                                             13u32))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                };
            match msg {
                ImportMsgType::FromNetwork(msg) =>
                self.handle_network_msg(msg),
                ImportMsgType::FromWorker(msg) => self.handle_worker_msg(msg),
            }
        }
        fn handle_network_msg(&mut self, msg: BlockImportMsg<B>) -> bool {
            match msg {
                BlockImportMsg::ImportBlocks(origin, incoming_blocks) => {
                    self.handle_import_blocks(origin, incoming_blocks)
                }
                BlockImportMsg::ImportJustification(who, hash, number,
                                                    justification) => {
                    self.handle_import_justification(who, hash, number,
                                                     justification)
                }
                BlockImportMsg::ImportFinalityProof(who, hash, number,
                                                    finality_proof) => {
                    self.handle_import_finality_proof(who, hash, number,
                                                      finality_proof)
                }
                BlockImportMsg::Start(link, sender) => {
                    if let Some(finality_proof_request_builder) =
                           self.finality_proof_request_builder.take() {
                        link.set_finality_proof_request_builder(finality_proof_request_builder);
                    }
                    if let Some(justification_import) =
                           self.justification_import.as_ref() {
                        justification_import.on_start(&*link);
                    }
                    if let Some(finality_proof_import) =
                           self.finality_proof_import.as_ref() {
                        finality_proof_import.on_start(&*link);
                    }
                    self.link = Some(link);
                    let _ = sender.send(Ok(()));
                }
                BlockImportMsg::Stop => return false,
            }
            true
        }
        fn handle_worker_msg(&mut self, msg: BlockImportWorkerMsg<B>)
         -> bool {
            let link =
                match self.link.as_ref() {
                    Some(link) => link,
                    None => {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL &&
                                   lvl <= ::log::max_level() {
                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Received import result while import-queue has no link"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        }),
                                                         lvl,
                                                         &("sync",
                                                           "substrate_consensus_common::import_queue",
                                                           "core/consensus/common/src/import_queue.rs",
                                                           382u32));
                            }
                        };
                        return true;
                    }
                };
            let results =
                match msg {
                    BlockImportWorkerMsg::ImportedBlocks(results) =>
                    (results),
                    BlockImportWorkerMsg::ImportedFinalityProof(who,
                                                                request_block,
                                                                finalization_result)
                    => {
                        link.finality_proof_imported(who, request_block,
                                                     finalization_result);
                        return true;
                    }
                    BlockImportWorkerMsg::ImportBlocks(_, _) |
                    BlockImportWorkerMsg::ImportFinalityProof(_, _, _, _) => {
                        {
                            {
                                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                          &match (&"Import Worker does not send Import* message; qed",)
                                                                                               {
                                                                                               (arg0,)
                                                                                               =>
                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                            ::std::fmt::Display::fmt)],
                                                                                           }),
                                                           &("core/consensus/common/src/import_queue.rs",
                                                             401u32, 9u32))
                            }
                        }
                    }
                };
            let mut has_error = false;
            let mut hashes = <[_]>::into_vec(box []);
            for (result, hash) in results {
                hashes.push(hash);
                if has_error { continue ; }
                if result.is_err() { has_error = true; }
                match result {
                    Ok(BlockImportResult::ImportedKnown(number)) =>
                    link.block_imported(&hash, number),
                    Ok(BlockImportResult::ImportedUnknown(number, aux, who))
                    => {
                        link.block_imported(&hash, number);
                        if aux.clear_justification_requests {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Block imported clears all pending justification requests ",
                                                                                             ": "],
                                                                                           &match (&number,
                                                                                                   &hash)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("sync",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               422u32));
                                }
                            };
                            link.clear_justification_requests();
                        }
                        if aux.needs_justification {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Block imported but requires justification ",
                                                                                             ": "],
                                                                                           &match (&number,
                                                                                                   &hash)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("sync",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               427u32));
                                }
                            };
                            link.request_justification(&hash, number);
                        }
                        if aux.bad_justification {
                            if let Some(peer) = who {
                                {
                                    let lvl = ::log::Level::Info;
                                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                                           lvl <= ::log::max_level() {
                                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Sent block with bad justification to import"],
                                                                                               &match ()
                                                                                                    {
                                                                                                    ()
                                                                                                    =>
                                                                                                    [],
                                                                                                }),
                                                                 lvl,
                                                                 &("substrate_consensus_common::import_queue",
                                                                   "substrate_consensus_common::import_queue",
                                                                   "core/consensus/common/src/import_queue.rs",
                                                                   433u32));
                                    }
                                };
                                link.report_peer(peer,
                                                 BAD_JUSTIFICATION_REPUTATION_CHANGE);
                            }
                        }
                        if aux.needs_finality_proof {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Block imported but requires finality proof ",
                                                                                             ": "],
                                                                                           &match (&number,
                                                                                                   &hash)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("sync",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               439u32));
                                }
                            };
                            link.request_finality_proof(&hash, number);
                        }
                    }
                    Err(BlockImportError::IncompleteHeader(who)) => {
                        if let Some(peer) = who {
                            {
                                let lvl = ::log::Level::Info;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Peer sent block with incomplete header to import"],
                                                                                           &match ()
                                                                                                {
                                                                                                ()
                                                                                                =>
                                                                                                [],
                                                                                            }),
                                                             lvl,
                                                             &("substrate_consensus_common::import_queue",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               445u32));
                                }
                            };
                            link.report_peer(peer,
                                             INCOMPLETE_HEADER_REPUTATION_CHANGE);
                            link.restart();
                        }
                    }
                    Err(BlockImportError::VerificationFailed(who, e)) => {
                        if let Some(peer) = who {
                            {
                                let lvl = ::log::Level::Info;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Verification failed from peer: "],
                                                                                           &match (&e,)
                                                                                                {
                                                                                                (arg0,)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("substrate_consensus_common::import_queue",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               452u32));
                                }
                            };
                            link.report_peer(peer,
                                             VERIFICATION_FAIL_REPUTATION_CHANGE);
                            link.restart();
                        }
                    }
                    Err(BlockImportError::BadBlock(who)) => {
                        if let Some(peer) = who {
                            {
                                let lvl = ::log::Level::Info;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Bad block"],
                                                                                           &match ()
                                                                                                {
                                                                                                ()
                                                                                                =>
                                                                                                [],
                                                                                            }),
                                                             lvl,
                                                             &("substrate_consensus_common::import_queue",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               459u32));
                                }
                            };
                            link.report_peer(peer,
                                             BAD_BLOCK_REPUTATION_CHANGE);
                            link.restart();
                        }
                    }
                    Err(BlockImportError::UnknownParent) |
                    Err(BlockImportError::Error) => {
                        link.restart();
                    }
                };
            }
            if let Some(link) = self.link.as_ref() {
                link.blocks_processed(hashes, has_error);
            }
            true
        }
        fn handle_import_justification(&self, who: Origin, hash: B::Hash,
                                       number: NumberFor<B>,
                                       justification: Justification) {
            let success =
                self.justification_import.as_ref().map(|justification_import|
                                                           {
                                                               justification_import.import_justification(hash,
                                                                                                         number,
                                                                                                         justification).map_err(|e|
                                                                                                                                    {
                                                                                                                                        {
                                                                                                                                            let lvl =
                                                                                                                                                ::log::Level::Debug;
                                                                                                                                            if lvl
                                                                                                                                                   <=
                                                                                                                                                   ::log::STATIC_MAX_LEVEL
                                                                                                                                                   &&
                                                                                                                                                   lvl
                                                                                                                                                       <=
                                                                                                                                                       ::log::max_level()
                                                                                                                                               {
                                                                                                                                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Justification import failed with ",
                                                                                                                                                                                                         " for hash: ",
                                                                                                                                                                                                         " number: ",
                                                                                                                                                                                                         " coming from node: "],
                                                                                                                                                                                                       &match (&e,
                                                                                                                                                                                                               &hash,
                                                                                                                                                                                                               &number,
                                                                                                                                                                                                               &who)
                                                                                                                                                                                                            {
                                                                                                                                                                                                            (arg0,
                                                                                                                                                                                                             arg1,
                                                                                                                                                                                                             arg2,
                                                                                                                                                                                                             arg3)
                                                                                                                                                                                                            =>
                                                                                                                                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                         ::std::fmt::Debug::fmt),
                                                                                                                                                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                                                                         ::std::fmt::Debug::fmt),
                                                                                                                                                                                                             ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                                                                                                                                         ::std::fmt::Debug::fmt),
                                                                                                                                                                                                             ::std::fmt::ArgumentV1::new(arg3,
                                                                                                                                                                                                                                         ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                        }),
                                                                                                                                                                         lvl,
                                                                                                                                                                         &("sync",
                                                                                                                                                                           "substrate_consensus_common::import_queue",
                                                                                                                                                                           "core/consensus/common/src/import_queue.rs",
                                                                                                                                                                           479u32));
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        e
                                                                                                                                    }).is_ok()
                                                           }).unwrap_or(false);
            if let Some(link) = self.link.as_ref() {
                link.justification_imported(who, &hash, number, success);
            }
        }
        fn handle_import_finality_proof(&self, who: Origin, hash: B::Hash,
                                        number: NumberFor<B>,
                                        finality_proof: Vec<u8>) {
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Scheduling finality proof of ",
                                                                             "/",
                                                                             " for import"],
                                                                           &match (&number,
                                                                                   &hash)
                                                                                {
                                                                                (arg0,
                                                                                 arg1)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }),
                                             lvl,
                                             &("sync",
                                               "substrate_consensus_common::import_queue",
                                               "core/consensus/common/src/import_queue.rs",
                                               490u32));
                }
            };
            self.worker_sender.send(BlockImportWorkerMsg::ImportFinalityProof(who,
                                                                              hash,
                                                                              number,
                                                                              finality_proof)).expect("1. This is holding a sender to the worker, 2. the worker should not quit while a sender is still held; qed");
        }
        fn handle_import_blocks(&mut self, origin: BlockOrigin,
                                blocks: Vec<IncomingBlock<B>>) {
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Scheduling ",
                                                                             " blocks for import"],
                                                                           &match (&blocks.len(),)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }),
                                             lvl,
                                             &("sync",
                                               "substrate_consensus_common::import_queue",
                                               "core/consensus/common/src/import_queue.rs",
                                               497u32));
                }
            };
            self.worker_sender.send(BlockImportWorkerMsg::ImportBlocks(origin,
                                                                       blocks)).expect("1. This is holding a sender to the worker, 2. the worker should not quit while a sender is still held; qed");
        }
    }
    struct BlockImportWorker<B: BlockT, V: Verifier<B>> {
        result_sender: Sender<BlockImportWorkerMsg<B>>,
        block_import: SharedBlockImport<B>,
        finality_proof_import: Option<SharedFinalityProofImport<B>>,
        verifier: Arc<V>,
    }
    impl <B: BlockT, V: 'static + Verifier<B>> BlockImportWorker<B, V> {
        pub fn new(result_sender: Sender<BlockImportWorkerMsg<B>>,
                   verifier: Arc<V>, block_import: SharedBlockImport<B>,
                   finality_proof_import:
                       Option<SharedFinalityProofImport<B>>)
         -> Sender<BlockImportWorkerMsg<B>> {
            let (sender, port) = channel::bounded(4);
            let _ =
                thread::Builder::new().name("ImportQueueWorker".into()).spawn(move
                                                                                  ||
                                                                                  {
                                                                                      let worker =
                                                                                          BlockImportWorker{result_sender,
                                                                                                            verifier,
                                                                                                            block_import,
                                                                                                            finality_proof_import,};
                                                                                      for msg
                                                                                          in
                                                                                          port.iter()
                                                                                          {
                                                                                          match msg
                                                                                              {
                                                                                              BlockImportWorkerMsg::ImportBlocks(origin,
                                                                                                                                 blocks)
                                                                                              =>
                                                                                              {
                                                                                                  worker.import_a_batch_of_blocks(origin,
                                                                                                                                  blocks);
                                                                                              }
                                                                                              BlockImportWorkerMsg::ImportFinalityProof(who,
                                                                                                                                        hash,
                                                                                                                                        number,
                                                                                                                                        proof)
                                                                                              =>
                                                                                              {
                                                                                                  worker.import_finality_proof(who,
                                                                                                                               hash,
                                                                                                                               number,
                                                                                                                               proof);
                                                                                              }
                                                                                              BlockImportWorkerMsg::ImportedBlocks(_)
                                                                                              |
                                                                                              BlockImportWorkerMsg::ImportedFinalityProof(_,
                                                                                                                                          _,
                                                                                                                                          _)
                                                                                              =>
                                                                                              {
                                                                                                  {
                                                                                                      {
                                                                                                          ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["internal error: entered unreachable code: "],
                                                                                                                                                                    &match (&"Import Worker does not receive the Imported* messages; qed",)
                                                                                                                                                                         {
                                                                                                                                                                         (arg0,)
                                                                                                                                                                         =>
                                                                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                      ::std::fmt::Display::fmt)],
                                                                                                                                                                     }),
                                                                                                                                     &("core/consensus/common/src/import_queue.rs",
                                                                                                                                       544u32,
                                                                                                                                       12u32))
                                                                                                      }
                                                                                                  }
                                                                                              }
                                                                                          }
                                                                                      }
                                                                                  }).expect("ImportQueueWorker thread spawning failed");
            sender
        }
        fn import_a_batch_of_blocks(&self, origin: BlockOrigin,
                                    blocks: Vec<IncomingBlock<B>>) {
            let count = blocks.len();
            let mut imported = 0;
            let blocks_range =
                match (blocks.first().and_then(|b|
                                                   b.header.as_ref().map(|h|
                                                                             h.number())),
                       blocks.last().and_then(|b|
                                                  b.header.as_ref().map(|h|
                                                                            h.number())))
                    {
                    (Some(first), Some(last)) if first != last =>
                    ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[" (",
                                                                         "..",
                                                                         ")"],
                                                                       &match (&first,
                                                                               &last)
                                                                            {
                                                                            (arg0,
                                                                             arg1)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Display::fmt),
                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                         ::std::fmt::Display::fmt)],
                                                                        })),
                    (Some(first), Some(_)) =>
                    ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[" (",
                                                                         ")"],
                                                                       &match (&first,)
                                                                            {
                                                                            (arg0,)
                                                                            =>
                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                         ::std::fmt::Display::fmt)],
                                                                        })),
                    _ => Default::default(),
                };
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Starting import of ",
                                                                             " blocks "],
                                                                           &match (&count,
                                                                                   &blocks_range)
                                                                                {
                                                                                (arg0,
                                                                                 arg1)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }),
                                             lvl,
                                             &("sync",
                                               "substrate_consensus_common::import_queue",
                                               "core/consensus/common/src/import_queue.rs",
                                               565u32));
                }
            };
            let mut results = <[_]>::into_vec(box []);
            let mut has_error = false;
            for block in blocks {
                let import_result =
                    if has_error {
                        Err(BlockImportError::Error)
                    } else {
                        import_single_block(&*self.block_import,
                                            origin.clone(), block.clone(),
                                            self.verifier.clone())
                    };
                let was_ok = import_result.is_ok();
                results.push((import_result, block.hash));
                if was_ok { imported += 1; } else { has_error = true; }
            }
            let _ =
                self.result_sender.send(BlockImportWorkerMsg::ImportedBlocks(results));
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Imported ",
                                                                             " of "],
                                                                           &match (&imported,
                                                                                   &count)
                                                                                {
                                                                                (arg0,
                                                                                 arg1)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }),
                                             lvl,
                                             &("sync",
                                               "substrate_consensus_common::import_queue",
                                               "core/consensus/common/src/import_queue.rs",
                                               596u32));
                }
            };
        }
        fn import_finality_proof(&self, who: Origin, hash: B::Hash,
                                 number: NumberFor<B>,
                                 finality_proof: Vec<u8>) {
            let result =
                self.finality_proof_import.as_ref().map(|finality_proof_import|
                                                            {
                                                                finality_proof_import.import_finality_proof(hash,
                                                                                                            number,
                                                                                                            finality_proof,
                                                                                                            &*self.verifier).map_err(|e|
                                                                                                                                         {
                                                                                                                                             {
                                                                                                                                                 let lvl =
                                                                                                                                                     ::log::Level::Debug;
                                                                                                                                                 if lvl
                                                                                                                                                        <=
                                                                                                                                                        ::log::STATIC_MAX_LEVEL
                                                                                                                                                        &&
                                                                                                                                                        lvl
                                                                                                                                                            <=
                                                                                                                                                            ::log::max_level()
                                                                                                                                                    {
                                                                                                                                                     ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Finality proof import failed with ",
                                                                                                                                                                                                              " for hash: ",
                                                                                                                                                                                                              " number: ",
                                                                                                                                                                                                              " coming from node: "],
                                                                                                                                                                                                            &match (&e,
                                                                                                                                                                                                                    &hash,
                                                                                                                                                                                                                    &number,
                                                                                                                                                                                                                    &who)
                                                                                                                                                                                                                 {
                                                                                                                                                                                                                 (arg0,
                                                                                                                                                                                                                  arg1,
                                                                                                                                                                                                                  arg2,
                                                                                                                                                                                                                  arg3)
                                                                                                                                                                                                                 =>
                                                                                                                                                                                                                 [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                              ::std::fmt::Debug::fmt),
                                                                                                                                                                                                                  ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                                                                              ::std::fmt::Debug::fmt),
                                                                                                                                                                                                                  ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                                                                                                                                              ::std::fmt::Debug::fmt),
                                                                                                                                                                                                                  ::std::fmt::ArgumentV1::new(arg3,
                                                                                                                                                                                                                                              ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                             }),
                                                                                                                                                                              lvl,
                                                                                                                                                                              &("substrate_consensus_common::import_queue",
                                                                                                                                                                                "substrate_consensus_common::import_queue",
                                                                                                                                                                                "core/consensus/common/src/import_queue.rs",
                                                                                                                                                                                603u32));
                                                                                                                                                 }
                                                                                                                                             };
                                                                                                                                         })
                                                            }).unwrap_or(Err(()));
            let _ =
                self.result_sender.send(BlockImportWorkerMsg::ImportedFinalityProof(who,
                                                                                    (hash,
                                                                                     number),
                                                                                    result));
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Imported finality proof for ",
                                                                             "/"],
                                                                           &match (&number,
                                                                                   &hash)
                                                                                {
                                                                                (arg0,
                                                                                 arg1)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }),
                                             lvl,
                                             &("sync",
                                               "substrate_consensus_common::import_queue",
                                               "core/consensus/common/src/import_queue.rs",
                                               617u32));
                }
            };
        }
    }
    /// Hooks that the verification queue can use to influence the synchronization
    /// algorithm.
    pub trait Link<B: BlockT>: Send {
        /// Block imported.
        fn block_imported(&self, _hash: &B::Hash, _number: NumberFor<B>) { }
        /// Batch of blocks imported, with or without error.
        fn blocks_processed(&self, _processed_blocks: Vec<B::Hash>,
                            _has_error: bool) {
        }
        /// Justification import result.
        fn justification_imported(&self, _who: Origin, _hash: &B::Hash,
                                  _number: NumberFor<B>, _success: bool) {
        }
        /// Clear all pending justification requests.
        fn clear_justification_requests(&self) { }
        /// Request a justification for the given block.
        fn request_justification(&self, _hash: &B::Hash,
                                 _number: NumberFor<B>) {
        }
        /// Finality proof import result.
        ///
        /// Even though we have asked for finality proof of block A, provider could return proof of
        /// some earlier block B, if the proof for A was too large. The sync module should continue
        /// asking for proof of A in this case.
        fn finality_proof_imported(&self, _who: Origin,
                                   _request_block: (B::Hash, NumberFor<B>),
                                   _finalization_result:
                                       Result<(B::Hash, NumberFor<B>), ()>) {
        }
        /// Request a finality proof for the given block.
        fn request_finality_proof(&self, _hash: &B::Hash,
                                  _number: NumberFor<B>) {
        }
        /// Remember finality proof request builder on start.
        fn set_finality_proof_request_builder(&self,
                                              _request_builder:
                                                  SharedFinalityProofRequestBuilder<B>) {
        }
        /// Adjusts the reputation of the given peer.
        fn report_peer(&self, _who: Origin, _reputation_change: i32) { }
        /// Restart sync.
        fn restart(&self) { }
    }
    /// Block import successful result.
    pub enum BlockImportResult<N: ::std::fmt::Debug + PartialEq> {

        /// Imported known block.
        ImportedKnown(N),

        /// Imported unknown block.
        ImportedUnknown(N, ImportedAux, Option<Origin>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <N: ::std::fmt::Debug + ::std::fmt::Debug + PartialEq>
     ::std::fmt::Debug for BlockImportResult<N> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&BlockImportResult::ImportedKnown(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ImportedKnown");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&BlockImportResult::ImportedUnknown(ref __self_0,
                                                     ref __self_1,
                                                     ref __self_2),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ImportedUnknown");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <N: ::std::cmp::PartialEq + ::std::fmt::Debug + PartialEq>
     ::std::cmp::PartialEq for BlockImportResult<N> {
        #[inline]
        fn eq(&self, other: &BlockImportResult<N>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&BlockImportResult::ImportedKnown(ref __self_0),
                         &BlockImportResult::ImportedKnown(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&BlockImportResult::ImportedUnknown(ref __self_0,
                                                             ref __self_1,
                                                             ref __self_2),
                         &BlockImportResult::ImportedUnknown(ref __arg_1_0,
                                                             ref __arg_1_1,
                                                             ref __arg_1_2))
                        =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1) &&
                            (*__self_2) == (*__arg_1_2),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &BlockImportResult<N>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&BlockImportResult::ImportedKnown(ref __self_0),
                         &BlockImportResult::ImportedKnown(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&BlockImportResult::ImportedUnknown(ref __self_0,
                                                             ref __self_1,
                                                             ref __self_2),
                         &BlockImportResult::ImportedUnknown(ref __arg_1_0,
                                                             ref __arg_1_1,
                                                             ref __arg_1_2))
                        =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1) ||
                            (*__self_2) != (*__arg_1_2),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    /// Block import error.
    pub enum BlockImportError {

        /// Block missed header, can't be imported
        IncompleteHeader(Option<Origin>),

        /// Block verification failed, can't be imported
        VerificationFailed(Option<Origin>, String),

        /// Block is known to be Bad
        BadBlock(Option<Origin>),

        /// Block has an unknown parent
        UnknownParent,

        /// Other Error.
        Error,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlockImportError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&BlockImportError::IncompleteHeader(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("IncompleteHeader");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&BlockImportError::VerificationFailed(ref __self_0,
                                                       ref __self_1),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("VerificationFailed");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&BlockImportError::BadBlock(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("BadBlock");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&BlockImportError::UnknownParent,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UnknownParent");
                    debug_trait_builder.finish()
                }
                (&BlockImportError::Error,) => {
                    let mut debug_trait_builder = f.debug_tuple("Error");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlockImportError {
        #[inline]
        fn eq(&self, other: &BlockImportError) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&BlockImportError::IncompleteHeader(ref __self_0),
                         &BlockImportError::IncompleteHeader(ref __arg_1_0))
                        => (*__self_0) == (*__arg_1_0),
                        (&BlockImportError::VerificationFailed(ref __self_0,
                                                               ref __self_1),
                         &BlockImportError::VerificationFailed(ref __arg_1_0,
                                                               ref __arg_1_1))
                        =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1),
                        (&BlockImportError::BadBlock(ref __self_0),
                         &BlockImportError::BadBlock(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => true,
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &BlockImportError) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&BlockImportError::IncompleteHeader(ref __self_0),
                         &BlockImportError::IncompleteHeader(ref __arg_1_0))
                        => (*__self_0) != (*__arg_1_0),
                        (&BlockImportError::VerificationFailed(ref __self_0,
                                                               ref __self_1),
                         &BlockImportError::VerificationFailed(ref __arg_1_0,
                                                               ref __arg_1_1))
                        =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1),
                        (&BlockImportError::BadBlock(ref __self_0),
                         &BlockImportError::BadBlock(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => false,
                    }
                } else { true }
            }
        }
    }
    /// Single block import function.
    pub fn import_single_block<B: BlockT,
                               V: Verifier<B>>(import_handle:
                                                   &BlockImport<B, Error =
                                                    ConsensusError>,
                                               block_origin: BlockOrigin,
                                               block: IncomingBlock<B>,
                                               verifier: Arc<V>)
     -> Result<BlockImportResult<NumberFor<B>>, BlockImportError> {
        let peer = block.origin;
        let (header, justification) =
            match (block.header, block.justification) {
                (Some(header), justification) => (header, justification),
                (None, _) => {
                    if let Some(ref peer) = peer {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL &&
                                   lvl <= ::log::max_level() {
                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Header ",
                                                                                         " was not provided by ",
                                                                                         " "],
                                                                                       &match (&block.hash,
                                                                                               &peer)
                                                                                            {
                                                                                            (arg0,
                                                                                             arg1)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt),
                                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        }),
                                                         lvl,
                                                         &("sync",
                                                           "substrate_consensus_common::import_queue",
                                                           "core/consensus/common/src/import_queue.rs",
                                                           695u32));
                            }
                        };
                    } else {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL &&
                                   lvl <= ::log::max_level() {
                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Header ",
                                                                                         " was not provided "],
                                                                                       &match (&block.hash,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        }),
                                                         lvl,
                                                         &("sync",
                                                           "substrate_consensus_common::import_queue",
                                                           "core/consensus/common/src/import_queue.rs",
                                                           697u32));
                            }
                        };
                    }
                    return Err(BlockImportError::IncompleteHeader(peer))
                }
            };
        let number = header.number().clone();
        let hash = header.hash();
        let parent = header.parent_hash().clone();
        let import_error =
            |e|
                {
                    match e {
                        Ok(ImportResult::AlreadyInChain) => {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Block already in chain ",
                                                                                             ": "],
                                                                                           &match (&number,
                                                                                                   &hash)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("sync",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               710u32));
                                }
                            };
                            Ok(BlockImportResult::ImportedKnown(number))
                        }
                        Ok(ImportResult::Imported(aux)) =>
                        Ok(BlockImportResult::ImportedUnknown(number, aux,
                                                              peer.clone())),
                        Ok(ImportResult::UnknownParent) => {
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Block with unknown parent ",
                                                                                             ": ",
                                                                                             ", parent: "],
                                                                                           &match (&number,
                                                                                                   &hash,
                                                                                                   &parent)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1,
                                                                                                 arg2)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                             ::std::fmt::Debug::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("sync",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               715u32));
                                }
                            };
                            Err(BlockImportError::UnknownParent)
                        }
                        Ok(ImportResult::KnownBad) => {
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Peer gave us a bad block ",
                                                                                             ": "],
                                                                                           &match (&number,
                                                                                                   &hash)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("sync",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               719u32));
                                }
                            };
                            Err(BlockImportError::BadBlock(peer.clone()))
                        }
                        Err(e) => {
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL &&
                                       lvl <= ::log::max_level() {
                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Error importing block ",
                                                                                             ": ",
                                                                                             ": "],
                                                                                           &match (&number,
                                                                                                   &hash,
                                                                                                   &e)
                                                                                                {
                                                                                                (arg0,
                                                                                                 arg1,
                                                                                                 arg2)
                                                                                                =>
                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                             ::std::fmt::Debug::fmt),
                                                                                                 ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                             ::std::fmt::Debug::fmt)],
                                                                                            }),
                                                             lvl,
                                                             &("sync",
                                                               "substrate_consensus_common::import_queue",
                                                               "core/consensus/common/src/import_queue.rs",
                                                               723u32));
                                }
                            };
                            Err(BlockImportError::Error)
                        }
                    }
                };
        match import_error(import_handle.check_block(hash, parent))? {
            BlockImportResult::ImportedUnknown { .. } => (),
            r => return Ok(r),
        }
        let (import_block, new_authorities) =
            verifier.verify(block_origin, header, justification,
                            block.body).map_err(|msg|
                                                    {
                                                        if let Some(ref peer)
                                                               = peer {
                                                            {
                                                                let lvl =
                                                                    ::log::Level::Trace;
                                                                if lvl <=
                                                                       ::log::STATIC_MAX_LEVEL
                                                                       &&
                                                                       lvl <=
                                                                           ::log::max_level()
                                                                   {
                                                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Verifying ",
                                                                                                                             "(",
                                                                                                                             ") from ",
                                                                                                                             " failed: "],
                                                                                                                           &match (&number,
                                                                                                                                   &hash,
                                                                                                                                   &peer,
                                                                                                                                   &msg)
                                                                                                                                {
                                                                                                                                (arg0,
                                                                                                                                 arg1,
                                                                                                                                 arg2,
                                                                                                                                 arg3)
                                                                                                                                =>
                                                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                                                 ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                                                 ::std::fmt::ArgumentV1::new(arg3,
                                                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                                                            }),
                                                                                             lvl,
                                                                                             &("sync",
                                                                                               "substrate_consensus_common::import_queue",
                                                                                               "core/consensus/common/src/import_queue.rs",
                                                                                               737u32));
                                                                }
                                                            };
                                                        } else {
                                                            {
                                                                let lvl =
                                                                    ::log::Level::Trace;
                                                                if lvl <=
                                                                       ::log::STATIC_MAX_LEVEL
                                                                       &&
                                                                       lvl <=
                                                                           ::log::max_level()
                                                                   {
                                                                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Verifying ",
                                                                                                                             "(",
                                                                                                                             ") failed: "],
                                                                                                                           &match (&number,
                                                                                                                                   &hash,
                                                                                                                                   &msg)
                                                                                                                                {
                                                                                                                                (arg0,
                                                                                                                                 arg1,
                                                                                                                                 arg2)
                                                                                                                                =>
                                                                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                             ::std::fmt::Display::fmt),
                                                                                                                                 ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                                                             ::std::fmt::Display::fmt)],
                                                                                                                            }),
                                                                                             lvl,
                                                                                             &("sync",
                                                                                               "substrate_consensus_common::import_queue",
                                                                                               "core/consensus/common/src/import_queue.rs",
                                                                                               739u32));
                                                                }
                                                            };
                                                        }
                                                        BlockImportError::VerificationFailed(peer.clone(),
                                                                                             msg)
                                                    })?;
        let mut cache = HashMap::new();
        if let Some(authorities) = new_authorities {
            cache.insert(crate::well_known_cache_keys::AUTHORITIES,
                         authorities.encode());
        }
        import_error(import_handle.import_block(import_block, cache))
    }
}
pub mod evaluation {
    //! Block evaluation and evaluation errors.
    use super::MAX_BLOCK_SIZE;
    use parity_codec::Encode;
    use runtime_primitives::traits::{Block as BlockT, Header as HeaderT, As};
    use error_chain::{error_chain, error_chain_processing,
                      impl_error_chain_processed, impl_extract_backtrace,
                      impl_error_chain_kind, bail};
    type BlockNumber = u64;
    /// The Error type.
    ///
    /// This tuple struct is made of two elements:
    ///
    /// - an `ErrorKind` which is used to determine the type of the error.
    /// - An internal `State`, not meant for direct use outside of `error_chain`
    ///   internals, containing:
    ///   - a backtrace, generated when the error is created.
    ///   - an error chain, used for the implementation of `Error::cause()`.
    pub struct Error(
                     /// The kind of the error.
                     pub ErrorKind,
                     /// Contains the error chain and the backtrace.
                     #[doc(hidden)]
                     pub ::error_chain::State);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Error(ref __self_0_0, ref __self_0_1) => {
                    let mut debug_trait_builder = f.debug_tuple("Error");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    let _ = debug_trait_builder.field(&&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl ::error_chain::ChainedError for Error {
        type
        ErrorKind
        =
        ErrorKind;
        fn new(kind: ErrorKind, state: ::error_chain::State) -> Error {
            Error(kind, state)
        }
        fn from_kind(kind: Self::ErrorKind) -> Self { Self::from_kind(kind) }
        fn with_chain<E, K>(error: E, kind: K) -> Self where
         E: ::std::error::Error + Send + 'static, K: Into<Self::ErrorKind> {
            Self::with_chain(error, kind)
        }
        fn kind(&self) -> &Self::ErrorKind { self.kind() }
        fn iter(&self) -> ::error_chain::Iter {
            ::error_chain::Iter::new(Some(self))
        }
        fn chain_err<F, EK>(self, error: F) -> Self where F: FnOnce() -> EK,
         EK: Into<ErrorKind> {
            self.chain_err(error)
        }
        fn backtrace(&self) -> Option<&::error_chain::Backtrace> {
            self.backtrace()
        }
        #[allow(unknown_lints,
                renamed_and_removed_lints,
                unused_doc_comment,
                unused_doc_comments)]
        fn extract_backtrace(e: &(::std::error::Error + Send + 'static))
         -> Option<::error_chain::InternalBacktrace> {
            if let Some(e) = e.downcast_ref::<Error>() {
                return Some(e.1.backtrace.clone());
            }
            None
        }
    }
    #[allow(dead_code)]
    impl Error {
        /// Constructs an error from a kind, and generates a backtrace.
        pub fn from_kind(kind: ErrorKind) -> Error {
            Error(kind, ::error_chain::State::default())
        }
        /// Constructs a chained error from another error and a kind, and generates a backtrace.
        pub fn with_chain<E, K>(error: E, kind: K) -> Error where
         E: ::std::error::Error + Send + 'static, K: Into<ErrorKind> {
            Error::with_boxed_chain(Box::new(error), kind)
        }
        /// Construct a chained error from another boxed error and a kind, and generates a backtrace
        pub fn with_boxed_chain<K>(error: Box<::std::error::Error + Send>,
                                   kind: K) -> Error where
         K: Into<ErrorKind> {
            Error(kind.into(), ::error_chain::State::new::<Error>(error))
        }
        /// Returns the kind of the error.
        pub fn kind(&self) -> &ErrorKind { &self.0 }
        /// Iterates over the error chain.
        pub fn iter(&self) -> ::error_chain::Iter {
            ::error_chain::ChainedError::iter(self)
        }
        /// Returns the backtrace associated with this error.
        pub fn backtrace(&self) -> Option<&::error_chain::Backtrace> {
            self.1.backtrace()
        }
        /// Extends the error chain with a new entry.
        pub fn chain_err<F, EK>(self, error: F) -> Error where F: FnOnce() ->
         EK, EK: Into<ErrorKind> {
            Error::with_chain(self, Self::from_kind(error().into()))
        }
        /// A short description of the error.
        /// This method is identical to [`Error::description()`](https://doc.rust-lang.org/nightly/std/error/trait.Error.html#tymethod.description)
        pub fn description(&self) -> &str { self.0.description() }
    }
    impl ::std::error::Error for Error {
        fn description(&self) -> &str { self.description() }
        #[allow(unknown_lints,
                renamed_and_removed_lints,
                unused_doc_comment,
                unused_doc_comments)]
        fn cause(&self) -> Option<&::std::error::Error> {
            match self.1.next_error {
                Some(ref c) => Some(&**c),
                None => { match self.0 { _ => None, } }
            }
        }
    }
    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl From<ErrorKind> for Error {
        fn from(e: ErrorKind) -> Self { Error::from_kind(e) }
    }
    impl <'a> From<&'a str> for Error {
        fn from(s: &'a str) -> Self { Error::from_kind(s.into()) }
    }
    impl From<String> for Error {
        fn from(s: String) -> Self { Error::from_kind(s.into()) }
    }
    #[doc = r" The kind of an error."]
    pub enum ErrorKind {

        #[doc = r" A convenient variant for String."]
        Msg(String),
        BadProposalFormat,
        WrongParentHash(String, String),
        WrongNumber(BlockNumber, BlockNumber),
        ProposalTooLarge(usize),

        #[doc(hidden)]
        __Nonexhaustive {
        },
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ErrorKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&ErrorKind::Msg(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Msg");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::BadProposalFormat,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("BadProposalFormat");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::WrongParentHash(ref __self_0, ref __self_1),) =>
                {
                    let mut debug_trait_builder =
                        f.debug_tuple("WrongParentHash");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::WrongNumber(ref __self_0, ref __self_1),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("WrongNumber");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::ProposalTooLarge(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ProposalTooLarge");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::__Nonexhaustive {  },) => {
                    let mut debug_trait_builder =
                        f.debug_struct("__Nonexhaustive");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(unknown_lints,
            unused,
            renamed_and_removed_lints,
            unused_doc_comment,
            unused_doc_comments)]
    impl ::std::fmt::Display for ErrorKind {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self
                {
                 #[doc = r" A convenient variant for String."]
                 ErrorKind::Msg(ref s) => {
                     let display_fn =
                         |_, f: &mut ::std::fmt::Formatter|
                             {
                                 f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                           &match (&s,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }))
                             };
                     display_fn(self, fmt)
                 }
                ErrorKind::BadProposalFormat => {
                    let display_fn =
                        |_, f: &mut ::std::fmt::Formatter|
                            {
                                f.write_fmt(::std::fmt::Arguments::new_v1(&["Proposal provided not a block."],
                                                                          &match ()
                                                                               {
                                                                               ()
                                                                               =>
                                                                               [],
                                                                           }))
                            };
                    display_fn(self, fmt)
                }
                ErrorKind::WrongParentHash(ref expected, ref got) => {
                    let display_fn =
                        |_, f: &mut ::std::fmt::Formatter|
                            {
                                f.write_fmt(::std::fmt::Arguments::new_v1(&["Proposal had wrong parent hash. Expected ",
                                                                            ", got "],
                                                                          &match (&expected,
                                                                                  &got)
                                                                               {
                                                                               (arg0,
                                                                                arg1)
                                                                               =>
                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                            ::std::fmt::Debug::fmt),
                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                            ::std::fmt::Debug::fmt)],
                                                                           }))
                            };
                    display_fn(self, fmt)
                }
                ErrorKind::WrongNumber(ref expected, ref got) => {
                    let display_fn =
                        |_, f: &mut ::std::fmt::Formatter|
                            {
                                f.write_fmt(::std::fmt::Arguments::new_v1(&["Proposal had wrong number. Expected ",
                                                                            ", got "],
                                                                          &match (&expected,
                                                                                  &got)
                                                                               {
                                                                               (arg0,
                                                                                arg1)
                                                                               =>
                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                            ::std::fmt::Display::fmt),
                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                            ::std::fmt::Display::fmt)],
                                                                           }))
                            };
                    display_fn(self, fmt)
                }
                ErrorKind::ProposalTooLarge(ref size) => {
                    let display_fn =
                        |_, f: &mut ::std::fmt::Formatter|
                            {
                                f.write_fmt(::std::fmt::Arguments::new_v1(&["Proposal exceeded the maximum size of ",
                                                                            " by ",
                                                                            " bytes."],
                                                                          &match (&MAX_BLOCK_SIZE,
                                                                                  &size.saturating_sub(MAX_BLOCK_SIZE))
                                                                               {
                                                                               (arg0,
                                                                                arg1)
                                                                               =>
                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                            ::std::fmt::Display::fmt),
                                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                                            ::std::fmt::Display::fmt)],
                                                                           }))
                            };
                    display_fn(self, fmt)
                }
                _ => Ok(()),
            }
        }
    }
    #[allow(unknown_lints,
            unused,
            renamed_and_removed_lints,
            unused_doc_comment,
            unused_doc_comments)]
    impl ErrorKind {
        /// A string describing the error kind.
        pub fn description(&self) -> &str {
            match *self
                {
                 #[doc = r" A convenient variant for String."]
                 ErrorKind::Msg(ref s) => {
                     &s
                 }
                ErrorKind::BadProposalFormat => {
                    "Proposal provided not a block."
                }
                ErrorKind::WrongParentHash(ref expected, ref got) => {
                    "Proposal had wrong parent hash."
                }
                ErrorKind::WrongNumber(ref expected, ref got) => {
                    "Proposal had wrong number."
                }
                ErrorKind::ProposalTooLarge(ref size) => {
                    "Proposal exceeded the maximum size."
                }
                _ => "",
            }
        }
    }
    impl <'a> From<&'a str> for ErrorKind {
        fn from(s: &'a str) -> Self { ErrorKind::Msg(s.to_string()) }
    }
    impl From<String> for ErrorKind {
        fn from(s: String) -> Self { ErrorKind::Msg(s) }
    }
    impl From<Error> for ErrorKind {
        fn from(e: Error) -> Self { e.0 }
    }
    /// Additional methods for `Result`, for easy interaction with this crate.
    pub trait ResultExt<T> {
        /// If the `Result` is an `Err` then `chain_err` evaluates the closure,
        /// which returns *some type that can be converted to `ErrorKind`*, boxes
        /// the original error to store as the cause, then returns a new error
        /// containing the original error.
        fn chain_err<F, EK>(self, callback: F)
        -> ::std::result::Result<T, Error>
        where
        F: FnOnce()
        ->
        EK,
        EK: Into<ErrorKind>;
    }
    impl <T, E> ResultExt<T> for ::std::result::Result<T, E> where
     E: ::std::error::Error + Send + 'static {
        fn chain_err<F, EK>(self, callback: F)
         -> ::std::result::Result<T, Error> where F: FnOnce() -> EK,
         EK: Into<ErrorKind> {
            self.map_err(move |e|
                             {
                                 let state =
                                     ::error_chain::State::new::<Error>(Box::new(e));
                                 ::error_chain::ChainedError::new(callback().into(),
                                                                  state)
                             })
        }
    }
    impl <T> ResultExt<T> for ::std::option::Option<T> {
        fn chain_err<F, EK>(self, callback: F)
         -> ::std::result::Result<T, Error> where F: FnOnce() -> EK,
         EK: Into<ErrorKind> {
            self.ok_or_else(move ||
                                {
                                    ::error_chain::ChainedError::from_kind(callback().into())
                                })
        }
    }
    /// Convenient wrapper around `std::Result`.
    #[allow(unused)]
    pub type Result<T> = ::std::result::Result<T, Error>;
    /// Attempt to evaluate a substrate block as a node block, returning error
    /// upon any initial validity checks failing.
    pub fn evaluate_initial<Block: BlockT>(proposal: &Block,
                                           parent_hash:
                                               &<Block as BlockT>::Hash,
                                           parent_number:
                                               <<Block as BlockT>::Header as
                                               HeaderT>::Number)
     -> Result<()> {
        let encoded = Encode::encode(proposal);
        let proposal =
            Block::decode(&mut &encoded[..]).ok_or_else(||
                                                            ErrorKind::BadProposalFormat)?;
        if encoded.len() > MAX_BLOCK_SIZE {
            return Err(ErrorKind::ProposalTooLarge(encoded.len()).into())
        }
        if *parent_hash != *proposal.header().parent_hash() {
            return Err(ErrorKind::WrongParentHash(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                     &match (&*parent_hash,)
                                                                                                          {
                                                                                                          (arg0,)
                                                                                                          =>
                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                       ::std::fmt::Debug::fmt)],
                                                                                                      })),
                                                  ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                     &match (&proposal.header().parent_hash(),)
                                                                                                          {
                                                                                                          (arg0,)
                                                                                                          =>
                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                       ::std::fmt::Debug::fmt)],
                                                                                                      }))).into());
        }
        if parent_number.as_() + 1 != proposal.header().number().as_() {
            return Err(ErrorKind::WrongNumber(parent_number.as_() + 1,
                                              proposal.header().number().as_()).into());
        }
        Ok(())
    }
}
const MAX_BLOCK_SIZE: usize = 4 * 1024 * 1024 + 512;
pub use self::error::{Error, ErrorKind};
pub use block_import::{BlockImport, BlockOrigin, ForkChoiceStrategy,
                       ImportedAux, ImportBlock, ImportResult,
                       JustificationImport, FinalityProofImport,
                       FinalityProofRequestBuilder};
pub use select_chain::SelectChain;
/// Trait for getting the authorities at a given block.
pub trait Authorities<B: Block> {
    type
    Error: std::error::Error +
    Send +
    'static;
    /// Get the authorities at the given block.
    fn authorities(&self, at: &BlockId<B>)
    -> Result<Vec<AuthorityIdFor<B>>, Self::Error>;
}
/// Environment producer for a Consensus instance. Creates proposer instance and communication streams.
pub trait Environment<B: Block> {
    /// The proposer type this creates.
    type
    Proposer: Proposer<B>;
    /// Error which can occur upon creation.
    type
    Error: From<Error>;
    /// Initialize the proposal logic on top of a specific header. Provide
    /// the authorities at that header.
    fn init(&self, parent_header: &B::Header,
            authorities: &[AuthorityIdFor<B>])
    -> Result<Self::Proposer, Self::Error>;
}
/// Logic for a proposer.
///
/// This will encapsulate creation and evaluation of proposals at a specific
/// block.
///
/// Proposers are generic over bits of "consensus data" which are engine-specific.
pub trait Proposer<B: Block> {
    /// Error type which can occur when proposing or evaluating.
    type
    Error: From<Error> +
    ::std::fmt::Debug +
    'static;
    /// Future that resolves to a committed proposal.
    type
    Create: IntoFuture<Item
    =
    B,
    Error
    =
    Self::Error>;
    /// Create a proposal.
    fn propose(&self, inherent_data: InherentData, max_duration: Duration)
    -> Self::Create;
}
/// An oracle for when major synchronization work is being undertaken.
///
/// Generally, consensus authoring work isn't undertaken while well behind
/// the head of the chain.
pub trait SyncOracle {
    /// Whether the synchronization service is undergoing major sync.
    /// Returns true if so.
    fn is_major_syncing(&self)
    -> bool;
    /// Whether the synchronization service is offline.
    /// Returns true if so.
    fn is_offline(&self)
    -> bool;
}
/// A synchronization oracle for when there is no network.
#[rustc_copy_clone_marker]
pub struct NoNetwork;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for NoNetwork {
    #[inline]
    fn clone(&self) -> NoNetwork { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for NoNetwork { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for NoNetwork {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NoNetwork => {
                let mut debug_trait_builder = f.debug_tuple("NoNetwork");
                debug_trait_builder.finish()
            }
        }
    }
}
impl SyncOracle for NoNetwork {
    fn is_major_syncing(&self) -> bool { false }
    fn is_offline(&self) -> bool { false }
}
impl <T: SyncOracle> SyncOracle for Arc<T> {
    fn is_major_syncing(&self) -> bool { T::is_major_syncing(&*self) }
    fn is_offline(&self) -> bool { T::is_offline(&*self) }
}
/// Extra verification for blocks.
pub trait ExtraVerification<B: Block>: Send + Sync {
    /// Future that resolves when the block is verified, or fails with error if
    /// not.
    type
    Verified: IntoFuture<Item
    =
    (),
    Error
    =
    String>;
    /// Do additional verification for this block.
    fn verify(&self, header: &B::Header, body: Option<&[B::Extrinsic]>)
    -> Self::Verified;
}
/// A list of all well known keys in the cache.
pub mod well_known_cache_keys {
    /// The type representing cache keys.
    pub type Id = [u8; 4];
    /// A list of authorities.
    pub const AUTHORITIES: Id = *b"auth";
}
