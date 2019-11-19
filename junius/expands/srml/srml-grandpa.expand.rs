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

//! GRANDPA Consensus module for runtime.
//!
//! This manages the GRANDPA authority set ready for the native code.
//! These authorities are only for GRANDPA finality, not for consensus overall.
//!
//! In the future, it will also handle misbehavior reports, and on-chain
//! finality notifications.
//!
//! For full integration with GRANDPA, the `GrandpaApi` should be implemented.
//! The necessary items are re-exported via the `fg_primitives` crate.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


// re-export since this is necessary for `impl_apis` in runtime.
pub use substrate_finality_grandpa_primitives as fg_primitives;

#[cfg(feature = "std")]
use serde::Serialize;
use rstd::prelude::*;
use parity_codec as codec;
use codec::{Encode, Decode};
use fg_primitives::ScheduledChange;
use srml_support::{Parameter, decl_event, decl_storage, decl_module};
use srml_support::dispatch::Result;
use srml_support::storage::StorageValue;
use srml_support::storage::unhashed::StorageVec;
use primitives::traits::CurrentHeight;
use substrate_primitives::ed25519;
use system::ensure_signed;
use primitives::traits::MaybeSerializeDebug;
use ed25519::Public as AuthorityId;


struct AuthorityStorageVec<S: codec::Codec +
                           Default>(rstd::marker::PhantomData<S>);
impl <S: codec::Codec + Default> StorageVec for AuthorityStorageVec<S> {
    type
    Item
    =
    (S, u64);
    const
    PREFIX:
    &'static [u8]
    =
    crate::fg_primitives::well_known_keys::AUTHORITY_PREFIX;
}

/// The log type of this crate, projected from module trait type.
pub type Log<T>
    =
    RawLog<<T as system::Trait>::BlockNumber, <T as Trait>::SessionKey>;

/// Logs which can be scanned by GRANDPA for authorities change events.
pub trait GrandpaChangeSignal<N> {
    /// Try to cast the log entry as a contained signal.
    fn as_signal(&self)
    -> Option<ScheduledChange<N>>;
    /// Try to cast the log entry as a contained forced signal.
    fn as_forced_signal(&self)
    -> Option<(N, ScheduledChange<N>)>;
}

/// A logs in this module.
#[structural_match]
pub enum RawLog<N, SessionKey> {

    /// Authorities set change has been signaled. Contains the new set of authorities
    /// and the delay in blocks _to finalize_ before applying.
    AuthoritiesChangeSignal(N, Vec<(SessionKey, u64)>),

    /// A forced authorities set change. Contains in this order: the median last
    /// finalized block when the change was signaled, the delay in blocks _to import_
    /// before applying and the new set of authorities.
    ForcedAuthoritiesChangeSignal(N, N, Vec<(SessionKey, u64)>),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_RawLog: () =
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
        impl <N, SessionKey> _serde::Serialize for RawLog<N, SessionKey> where
         N: _serde::Serialize, SessionKey: _serde::Serialize {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    RawLog::AuthoritiesChangeSignal(ref __field0,
                                                    ref __field1) => {
                        let mut __serde_state =
                            match _serde::Serializer::serialize_tuple_variant(__serializer,
                                                                              "RawLog",
                                                                              0u32,
                                                                              "AuthoritiesChangeSignal",
                                                                              0
                                                                                  +
                                                                                  1
                                                                                  +
                                                                                  1)
                                {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeTupleVariant::serialize_field(&mut __serde_state,
                                                                                  __field0)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeTupleVariant::serialize_field(&mut __serde_state,
                                                                                  __field1)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeTupleVariant::end(__serde_state)
                    }
                    RawLog::ForcedAuthoritiesChangeSignal(ref __field0,
                                                          ref __field1,
                                                          ref __field2) => {
                        let mut __serde_state =
                            match _serde::Serializer::serialize_tuple_variant(__serializer,
                                                                              "RawLog",
                                                                              1u32,
                                                                              "ForcedAuthoritiesChangeSignal",
                                                                              0
                                                                                  +
                                                                                  1
                                                                                  +
                                                                                  1
                                                                                  +
                                                                                  1)
                                {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeTupleVariant::serialize_field(&mut __serde_state,
                                                                                  __field0)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeTupleVariant::serialize_field(&mut __serde_state,
                                                                                  __field1)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeTupleVariant::serialize_field(&mut __serde_state,
                                                                                  __field2)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeTupleVariant::end(__serde_state)
                    }
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <N: ::std::fmt::Debug, SessionKey: ::std::fmt::Debug> ::std::fmt::Debug
 for RawLog<N, SessionKey> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawLog::AuthoritiesChangeSignal(ref __self_0, ref __self_1),) =>
            {
                let mut debug_trait_builder =
                    f.debug_tuple("AuthoritiesChangeSignal");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                debug_trait_builder.finish()
            }
            (&RawLog::ForcedAuthoritiesChangeSignal(ref __self_0,
                                                    ref __self_1,
                                                    ref __self_2),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("ForcedAuthoritiesChangeSignal");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawLog: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <N, SessionKey> _parity_codec::Encode for RawLog<N, SessionKey>
         where N: _parity_codec::Encode, N: _parity_codec::Encode,
         Vec<(SessionKey, u64)>: _parity_codec::Encode,
         Vec<(SessionKey, u64)>: _parity_codec::Encode,
         N: _parity_codec::Encode, N: _parity_codec::Encode,
         N: _parity_codec::Encode, N: _parity_codec::Encode,
         Vec<(SessionKey, u64)>: _parity_codec::Encode,
         Vec<(SessionKey, u64)>: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawLog::AuthoritiesChangeSignal(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    RawLog::ForcedAuthoritiesChangeSignal(ref aa, ref ba,
                                                          ref ca) => {
                        dest.push_byte(1usize as u8);
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
const _IMPL_DECODE_FOR_RawLog: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <N, SessionKey> _parity_codec::Decode for RawLog<N, SessionKey>
         where N: _parity_codec::Decode, N: _parity_codec::Decode,
         Vec<(SessionKey, u64)>: _parity_codec::Decode,
         Vec<(SessionKey, u64)>: _parity_codec::Decode,
         N: _parity_codec::Decode, N: _parity_codec::Decode,
         N: _parity_codec::Decode, N: _parity_codec::Decode,
         Vec<(SessionKey, u64)>: _parity_codec::Decode,
         Vec<(SessionKey, u64)>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawLog::AuthoritiesChangeSignal(_parity_codec::Decode::decode(input)?,
                                                             _parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(RawLog::ForcedAuthoritiesChangeSignal(_parity_codec::Decode::decode(input)?,
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
impl <N: ::std::cmp::PartialEq, SessionKey: ::std::cmp::PartialEq>
 ::std::cmp::PartialEq for RawLog<N, SessionKey> {
    #[inline]
    fn eq(&self, other: &RawLog<N, SessionKey>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawLog::AuthoritiesChangeSignal(ref __self_0,
                                                      ref __self_1),
                     &RawLog::AuthoritiesChangeSignal(ref __arg_1_0,
                                                      ref __arg_1_1)) =>
                    (*__self_0) == (*__arg_1_0) &&
                        (*__self_1) == (*__arg_1_1),
                    (&RawLog::ForcedAuthoritiesChangeSignal(ref __self_0,
                                                            ref __self_1,
                                                            ref __self_2),
                     &RawLog::ForcedAuthoritiesChangeSignal(ref __arg_1_0,
                                                            ref __arg_1_1,
                                                            ref __arg_1_2)) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &RawLog<N, SessionKey>) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawLog::AuthoritiesChangeSignal(ref __self_0,
                                                      ref __self_1),
                     &RawLog::AuthoritiesChangeSignal(ref __arg_1_0,
                                                      ref __arg_1_1)) =>
                    (*__self_0) != (*__arg_1_0) ||
                        (*__self_1) != (*__arg_1_1),
                    (&RawLog::ForcedAuthoritiesChangeSignal(ref __self_0,
                                                            ref __self_1,
                                                            ref __self_2),
                     &RawLog::ForcedAuthoritiesChangeSignal(ref __arg_1_0,
                                                            ref __arg_1_1,
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
impl <N: ::std::cmp::Eq, SessionKey: ::std::cmp::Eq> ::std::cmp::Eq for
 RawLog<N, SessionKey> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<N>;
            let _: ::std::cmp::AssertParamIsEq<Vec<(SessionKey, u64)>>;
            let _: ::std::cmp::AssertParamIsEq<N>;
            let _: ::std::cmp::AssertParamIsEq<N>;
            let _: ::std::cmp::AssertParamIsEq<Vec<(SessionKey, u64)>>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <N: ::std::clone::Clone, SessionKey: ::std::clone::Clone>
 ::std::clone::Clone for RawLog<N, SessionKey> {
    #[inline]
    fn clone(&self) -> RawLog<N, SessionKey> {
        match (&*self,) {
            (&RawLog::AuthoritiesChangeSignal(ref __self_0, ref __self_1),) =>
            RawLog::AuthoritiesChangeSignal(::std::clone::Clone::clone(&(*__self_0)),
                                            ::std::clone::Clone::clone(&(*__self_1))),
            (&RawLog::ForcedAuthoritiesChangeSignal(ref __self_0,
                                                    ref __self_1,
                                                    ref __self_2),) =>
            RawLog::ForcedAuthoritiesChangeSignal(::std::clone::Clone::clone(&(*__self_0)),
                                                  ::std::clone::Clone::clone(&(*__self_1)),
                                                  ::std::clone::Clone::clone(&(*__self_2))),
        }
    }
}

impl <N: Clone, SessionKey> RawLog<N, SessionKey> {
    /// Try to cast the log entry as a contained signal.
    pub fn as_signal(&self) -> Option<(N, &[(SessionKey, u64)])> {
        match *self {
            RawLog::AuthoritiesChangeSignal(ref delay, ref signal) =>
            Some((delay.clone(), signal)),
            RawLog::ForcedAuthoritiesChangeSignal(_, _, _) => None,
        }
    }

    /// Try to cast the log entry as a contained forced signal.
    pub fn as_forced_signal(&self) -> Option<(N, N, &[(SessionKey, u64)])> {
        match *self {
            RawLog::ForcedAuthoritiesChangeSignal(ref median, ref delay,
                                                  ref signal) =>
            Some((median.clone(), delay.clone(), signal)),
            RawLog::AuthoritiesChangeSignal(_, _) => None,
        }
    }
}

impl <N, SessionKey> GrandpaChangeSignal<N> for RawLog<N, SessionKey> where
 N: Clone, SessionKey: Clone + Into<AuthorityId> {
    fn as_signal(&self) -> Option<ScheduledChange<N>> {
        RawLog::as_signal(self).map(|(delay, next_authorities)|
                                        ScheduledChange{delay,
                                                        next_authorities:
                                                            next_authorities.iter().cloned().map(|(k,
                                                                                                   w)|
                                                                                                     (k.into(),
                                                                                                      w)).collect(),})
    }

    fn as_forced_signal(&self) -> Option<(N, ScheduledChange<N>)> {
        RawLog::as_forced_signal(self).map(|(median, delay, next_authorities)|
                                               (median,
                                                ScheduledChange{delay,
                                                                next_authorities:
                                                                    next_authorities.iter().cloned().map(|(k,
                                                                                                           w)|
                                                                                                             (k.into(),
                                                                                                              w)).collect(),}))
    }
}

pub trait Trait: system::Trait {
    /// Type for all log entries of this module.
    type
    Log: From<Log<Self>> +
    Into<system::DigestItemOf<Self>>;

    /// The session key type used by authorities.
    type
    SessionKey: Parameter +
    Default +
    MaybeSerializeDebug;

    /// The event type of this module.
    type
    Event: From<Event<Self>> +
    Into<<Self as system::Trait>::Event>;
}

// TODO: remove shim
// https://github.com/paritytech/substrate/issues/1614
/// A stored pending change, old format.
pub struct OldStoredPendingChange<N, SessionKey> {
    /// The block number this was scheduled at.
    pub scheduled_at: N,
    /// The delay in blocks until it will be applied.
    pub delay: N,
    /// The next authority set.
    pub next_authorities: Vec<(SessionKey, u64)>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_OldStoredPendingChange: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <N, SessionKey> _parity_codec::Encode for
         OldStoredPendingChange<N, SessionKey> where N: _parity_codec::Encode,
         N: _parity_codec::Encode, N: _parity_codec::Encode,
         N: _parity_codec::Encode,
         Vec<(SessionKey, u64)>: _parity_codec::Encode,
         Vec<(SessionKey, u64)>: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.scheduled_at);
                dest.push(&self.delay);
                dest.push(&self.next_authorities);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_OldStoredPendingChange: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <N, SessionKey> _parity_codec::Decode for
         OldStoredPendingChange<N, SessionKey> where N: _parity_codec::Decode,
         N: _parity_codec::Decode, N: _parity_codec::Decode,
         N: _parity_codec::Decode,
         Vec<(SessionKey, u64)>: _parity_codec::Decode,
         Vec<(SessionKey, u64)>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(OldStoredPendingChange{scheduled_at:
                                                _parity_codec::Decode::decode(input)?,
                                            delay:
                                                _parity_codec::Decode::decode(input)?,
                                            next_authorities:
                                                _parity_codec::Decode::decode(input)?,})
            }
        }
    };

/// A stored pending change.
pub struct StoredPendingChange<N, SessionKey> {
    /// The block number this was scheduled at.
    pub scheduled_at: N,
    /// The delay in blocks until it will be applied.
    pub delay: N,
    /// The next authority set.
    pub next_authorities: Vec<(SessionKey, u64)>,
    /// If defined it means the change was forced and the given block number
    /// indicates the median last finalized block when the change was signaled.
    pub forced: Option<N>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_StoredPendingChange: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <N, SessionKey> _parity_codec::Encode for
         StoredPendingChange<N, SessionKey> where N: _parity_codec::Encode,
         N: _parity_codec::Encode, N: _parity_codec::Encode,
         N: _parity_codec::Encode,
         Vec<(SessionKey, u64)>: _parity_codec::Encode,
         Vec<(SessionKey, u64)>: _parity_codec::Encode,
         Option<N>: _parity_codec::Encode, Option<N>: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.scheduled_at);
                dest.push(&self.delay);
                dest.push(&self.next_authorities);
                dest.push(&self.forced);
            }
        }
    };

impl <N: Decode, SessionKey: Decode> Decode for
 StoredPendingChange<N, SessionKey> {
    fn decode<I: codec::Input>(value: &mut I) -> Option<Self> {
        let old = OldStoredPendingChange::decode(value)?;
        let forced = <Option<N>>::decode(value).unwrap_or(None);

        Some(StoredPendingChange{scheduled_at: old.scheduled_at,
                                 delay: old.delay,
                                 next_authorities: old.next_authorities,
                                 forced,})
    }
}


// Pending change: (signaled at, scheduled change).
// next block number where we can force a change.




// FIXME: https://github.com/paritytech/substrate/issues/1112







// only allow the next forced change when twice the window has passed since
// this one.







// FIXME: remove when https://github.com/rust-lang/rust/issues/26925 is fixed


// evenly-weighted.

// instant changes

// when we record old authority sets, we can use `finality_tracker::median`
// to figure out _who_ failed. until then, we can't meaningfully guard
// against `next == last` the way that normal session changes do.

// evenly-weighted.


// schedule a change for `further_wait` blocks.
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T> = RawEvent<<T as Trait>::SessionKey>;
/// Events for this module.
///
#[structural_match]
pub enum RawEvent<SessionKey> {

    #[doc = r" New authority set has been applied."]
    NewAuthorities(Vec<(SessionKey, u64)>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <SessionKey: ::std::clone::Clone> ::std::clone::Clone for
 RawEvent<SessionKey> {
    #[inline]
    fn clone(&self) -> RawEvent<SessionKey> {
        match (&*self,) {
            (&RawEvent::NewAuthorities(ref __self_0),) =>
            RawEvent::NewAuthorities(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <SessionKey: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
 RawEvent<SessionKey> {
    #[inline]
    fn eq(&self, other: &RawEvent<SessionKey>) -> bool {
        match (&*self, &*other) {
            (&RawEvent::NewAuthorities(ref __self_0),
             &RawEvent::NewAuthorities(ref __arg_1_0)) =>
            (*__self_0) == (*__arg_1_0),
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<SessionKey>) -> bool {
        match (&*self, &*other) {
            (&RawEvent::NewAuthorities(ref __self_0),
             &RawEvent::NewAuthorities(ref __arg_1_0)) =>
            (*__self_0) != (*__arg_1_0),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <SessionKey: ::std::cmp::Eq> ::std::cmp::Eq for RawEvent<SessionKey> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<Vec<(SessionKey, u64)>>; }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawEvent: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <SessionKey> _parity_codec::Encode for RawEvent<SessionKey> where
         Vec<(SessionKey, u64)>: _parity_codec::Encode,
         Vec<(SessionKey, u64)>: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::NewAuthorities(ref aa) => {
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
        impl <SessionKey> _parity_codec::Decode for RawEvent<SessionKey> where
         Vec<(SessionKey, u64)>: _parity_codec::Decode,
         Vec<(SessionKey, u64)>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::NewAuthorities(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <SessionKey: ::std::fmt::Debug> ::std::fmt::Debug for
 RawEvent<SessionKey> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawEvent::NewAuthorities(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("NewAuthorities");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <SessionKey> From<RawEvent<SessionKey>> for () {
    fn from(_: RawEvent<SessionKey>) -> () { () }
}
impl <SessionKey> RawEvent<SessionKey> {
    #[allow(dead_code)]
    pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
        &[::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("NewAuthorities"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["Vec<(SessionKey, u64)>"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" New authority set has been applied."]),}]
    }
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
struct PendingChange<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<StoredPendingChange<T::BlockNumber,
                                                                                                                         T::SessionKey>>
 for PendingChange<T> {
    type
    Query
    =
    Option<StoredPendingChange<T::BlockNumber, T::SessionKey>>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "GrandpaFinality PendingChange".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<StoredPendingChange<T::BlockNumber,
                                                                                                                                                T::SessionKey>>>::key()).or_else(||
                                                                                                                                                                                     Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<StoredPendingChange<T::BlockNumber,
                                                                                                                                                 T::SessionKey>>>::key()).or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<StoredPendingChange<T::BlockNumber,
                                                                                                                                        T::SessionKey>>>::get(storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<StoredPendingChange<T::BlockNumber,
                                                                                                                                        T::SessionKey>>>::put(&val,
                                                                                                                                                              storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<StoredPendingChange<T::BlockNumber,
                                                                                                                                        T::SessionKey>>>::kill(storage),
        };
        ret
    }
}
struct NextForced<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for NextForced<T> {
    type
    Query
    =
    Option<T::BlockNumber>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "GrandpaFinality NextForced".as_bytes() }
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
    PendingChange;
    type
    NextForced;
}
#[doc(hidden)]
pub struct __GetByteStructPendingChange<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_PendingChange:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructPendingChange<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_PendingChange.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          Option<StoredPendingChange<T::BlockNumber,
                                                                                                     T::SessionKey>> =
                                                                      Default::default();
                                                                  <Option<StoredPendingChange<T::BlockNumber,
                                                                                              T::SessionKey>>
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructNextForced<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_NextForced:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructNextForced<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_NextForced.get_or_init(||
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
    PendingChange
    =
    PendingChange<T>;
    type
    NextForced
    =
    NextForced<T>;
}
impl <T: 'static + Trait> Module<T> {
    pub fn pending_change()
     -> Option<StoredPendingChange<T::BlockNumber, T::SessionKey>> {
        <PendingChange<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<StoredPendingChange<T::BlockNumber,
                                                                                                                                    T::SessionKey>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    pub fn next_forced() -> Option<T::BlockNumber> {
        <NextForced<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PendingChange"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("StoredPendingChange<T::BlockNumber, T::SessionKey>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPendingChange::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextForced"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextForced::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),}]
                                                                                                                                                                                              }),}
    }
    #[doc(hidden)]
    pub fn store_metadata_functions()
     ->
         &'static [self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata] {
        {
            &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("PendingChange"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("StoredPendingChange<T::BlockNumber, T::SessionKey>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructPendingChange::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("NextForced"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructNextForced::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "GrandpaFinality" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "Vec < ( T :: SessionKey , u64 ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "Vec < ( T :: SessionKey , u64 ) > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    pub authorities: Vec<(T::SessionKey, u64)>,
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
         Vec<(T::SessionKey,
              u64)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
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
                                                                    "authorities",
                                                                    &self.authorities)
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
         Vec<(T::SessionKey,
              u64)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            "authorities" =>
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
                            b"authorities" =>
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
                       Vec<(T::SessionKey,
                            u64)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 Vec<(T::SessionKey,
                      u64)>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            match match _serde::de::SeqAccess::next_element::<Vec<(T::SessionKey,
                                                                                   u64)>>(&mut __seq)
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
                        _serde::export::Ok(GenesisConfig{authorities:
                                                             __field0,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Vec<(T::SessionKey,
                                                            u64)>> =
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
                                                                       _serde::de::Error>::duplicate_field("authorities"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<(T::SessionKey,
                                                                                                            u64)>>(&mut __map)
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
                                match _serde::private::de::missing_field("authorities")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{authorities:
                                                             __field0,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["authorities"];
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
    fn default() -> Self { GenesisConfig{authorities: Default::default(),} }
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
        let r = storage.into_inner();
        (|storage: &mut primitives::StorageOverlay,
          _: &mut primitives::ChildrenStorageOverlay,
          config: &GenesisConfig<T>|
             {
                 use codec::{Encode, KeyedVec};
                 let auth_count = config.authorities.len() as u32;
                 config.authorities.iter().enumerate().for_each(|(i, v)|
                                                                    {
                                                                        storage.insert((i
                                                                                            as
                                                                                            u32).to_keyed_vec(crate::fg_primitives::well_known_keys::AUTHORITY_PREFIX),
                                                                                       v.encode());
                                                                    });
                 storage.insert(crate::fg_primitives::well_known_keys::AUTHORITY_COUNT.to_vec(),
                                auth_count.encode());
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
    fn on_finalize(block_number: T::BlockNumber) {
        if let Some(pending_change) = <PendingChange<T>>::get() {
            if block_number == pending_change.scheduled_at {
                if let Some(median) = pending_change.forced {
                    Self::deposit_log(RawLog::ForcedAuthoritiesChangeSignal(median,
                                                                            pending_change.delay,
                                                                            pending_change.next_authorities.clone()));
                } else {
                    Self::deposit_log(RawLog::AuthoritiesChangeSignal(pending_change.delay,
                                                                      pending_change.next_authorities.clone()));
                }
            }
            if block_number ==
                   pending_change.scheduled_at + pending_change.delay {
                Self::deposit_event(RawEvent::NewAuthorities(pending_change.next_authorities.clone()));
                <AuthorityStorageVec<T::SessionKey>>::set_items(pending_change.next_authorities);
                <PendingChange<T>>::kill();
            }
        }
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
    #[doc = r" Report some misbehavior."]
    fn report_misbehavior(origin: T::Origin, _report: Vec<u8>)
     -> ::srml_support::dispatch::Result {
        { ensure_signed(origin)?; }
        Ok(())
    }
}
pub enum Call<T: Trait> {

    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                  ::srml_support::dispatch::Never),

    #[allow(non_camel_case_types)]
    report_misbehavior(Vec<u8>),
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
                match *self {
                    Call::report_misbehavior(ref aa) => {
                        dest.push_byte(0usize as u8);
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
        impl <T: Trait> _parity_codec::Decode for Call<T> {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::report_misbehavior(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
impl <T: Trait> ::srml_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::report_misbehavior(ref _report) =>
            Call::report_misbehavior((*_report).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/grandpa/src/lib.rs",
                                             214u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::report_misbehavior(ref _report) => {
                let self_params = (_report,);
                if let Call::report_misbehavior(ref _report) = *_other {
                    self_params == (_report,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/grandpa/src/lib.rs",
                                                         214u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/grandpa/src/lib.rs",
                                             214u32, 1u32))
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
            Call::report_misbehavior(ref _report) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"report_misbehavior",
                                                               &(_report.clone(),))
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
                                           &("srml/grandpa/src/lib.rs",
                                             214u32, 1u32))
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
            Call::report_misbehavior(_report) => {
                <Module<T>>::report_misbehavior(_origin, _report)
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
                                                       &("srml/grandpa/src/lib.rs",
                                                         214u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("report_misbehavior"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("_report"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Report some misbehavior."]),}]
    }
}
impl <T: Trait> Module<T> {
    /// Get the current set of authorities, along with their respective weights.
    pub fn grandpa_authorities() -> Vec<(T::SessionKey, u64)> {
        <AuthorityStorageVec<T::SessionKey>>::items()
    }
    /// Schedule a change in the authorities.
    ///
    /// The change will be applied at the end of execution of the block
    /// `in_blocks` after the current block. This value may be 0, in which
    /// case the change is applied at the end of the current block.
    ///
    /// If the `forced` parameter is defined, this indicates that the current
    /// set has been synchronously determined to be offline and that after
    /// `in_blocks` the given change should be applied. The given block number
    /// indicates the median last finalized block number and it should be used
    /// as the canon block when starting the new grandpa voter.
    ///
    /// No change should be signaled while any change is pending. Returns
    /// an error if a change is already pending.
    pub fn schedule_change(next_authorities: Vec<(T::SessionKey, u64)>,
                           in_blocks: T::BlockNumber,
                           forced: Option<T::BlockNumber>) -> Result {
        use primitives::traits::As;
        if Self::pending_change().is_none() {
            let scheduled_at =
                system::ChainContext::<T>::default().current_height();
            if let Some(_) = forced {
                if Self::next_forced().map_or(false,
                                              |next| next > scheduled_at) {
                    return Err("Cannot signal forced change so soon after last.");
                }
                <NextForced<T>>::put(scheduled_at +
                                         in_blocks * T::BlockNumber::sa(2));
            }
            <PendingChange<T>>::put(StoredPendingChange{delay: in_blocks,
                                                        scheduled_at,
                                                        next_authorities,
                                                        forced,});
            Ok(())
        } else {
            Err("Attempt to signal GRANDPA change with one already pending.")
        }
    }
    /// Deposit one of this module's logs.
    fn deposit_log(log: Log<T>) {
        <system::Module<T>>::deposit_log(<T as Trait>::from(log).into());
    }
}
impl <T: Trait> Module<T> where
 AuthorityId: core::convert::From<<T as Trait>::SessionKey> {
    /// See if the digest contains any standard scheduled change.
    pub fn scrape_digest_change(log: &Log<T>)
     -> Option<ScheduledChange<T::BlockNumber>> {
        <Log<T> as GrandpaChangeSignal<T::BlockNumber>>::as_signal(log)
    }
    /// See if the digest contains any forced scheduled change.
    pub fn scrape_digest_forced_change(log: &Log<T>)
     -> Option<(T::BlockNumber, ScheduledChange<T::BlockNumber>)> {
        <Log<T> as GrandpaChangeSignal<T::BlockNumber>>::as_forced_signal(log)
    }
}
/// Helper for authorities being synchronized with the general session authorities.
///
/// This is not the only way to manage an authority set for GRANDPA, but it is
/// a convenient one. When this is used, no other mechanism for altering authority
/// sets should be.
pub struct SyncedAuthorities<T>(::rstd::marker::PhantomData<T>);
impl <T> Default for SyncedAuthorities<T> {
    fn default() -> Self { SyncedAuthorities(::rstd::marker::PhantomData) }
}
impl <X, T> session::OnSessionChange<X> for SyncedAuthorities<T> where
 T: Trait + consensus::Trait<SessionKey = <T as Trait>::SessionKey>,
 <T as
 consensus::Trait>::Log: From<consensus::RawLog<<T as Trait>::SessionKey>> {
    fn on_session_change(_: X, _: bool) {
        use primitives::traits::Zero;
        let next_authorities =
            <consensus::Module<T>>::authorities().into_iter().map(|key|
                                                                      (key,
                                                                       1)).collect::<Vec<(<T
                                                                                          as
                                                                                          Trait>::SessionKey,
                                                                                          u64)>>();
        let last_authorities = <Module<T>>::grandpa_authorities();
        if next_authorities != last_authorities {
            let _ =
                <Module<T>>::schedule_change(next_authorities, Zero::zero(),
                                             None);
        }
    }
}
impl <T> finality_tracker::OnFinalizationStalled<T::BlockNumber> for
 SyncedAuthorities<T> where T: Trait + consensus::Trait<SessionKey =
 <T as Trait>::SessionKey>,
 <T as
 consensus::Trait>::Log: From<consensus::RawLog<<T as Trait>::SessionKey>>,
 T: finality_tracker::Trait {
    fn on_stalled(further_wait: T::BlockNumber) {
        let next_authorities =
            <consensus::Module<T>>::authorities().into_iter().map(|key|
                                                                      (key,
                                                                       1)).collect::<Vec<(<T
                                                                                          as
                                                                                          Trait>::SessionKey,
                                                                                          u64)>>();
        let median = <finality_tracker::Module<T>>::median();
        let _ =
            <Module<T>>::schedule_change(next_authorities, further_wait,
                                         Some(median));
    }
}
