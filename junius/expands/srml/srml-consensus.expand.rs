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

//! # Consensus Module
//!
//! - [`consensus::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! The consensus module manages the authority set for the native code. It provides support for reporting offline
//! behavior among validators and logging changes in the validator authority set.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `report_misbehavior` - Report some misbehavior. The origin of this call must be signed.
//! - `note_offline` - Note that the previous block's validator missed its opportunity to propose a block.
//!  The origin of this call must be an inherent.
//! - `remark` - Make some on-chain remark. The origin of this call must be signed.
//! - `set_heap_pages` - Set the number of pages in the WebAssembly environment's heap.
//! - `set_code` - Set the new code.
//! - `set_storage` - Set some items of storage.
//!
//! ### Public Functions
//!
//! - `authorities` - Get the current set of authorities. These are the session keys.
//! - `set_authorities` - Set the current set of authorities' session keys.
//! - `set_authority_count` - Set the total number of authorities.
//! - `set_authority` - Set a single authority by index.
//!
//! ## Usage
//!
//! ### Simple Code Snippet
//!
//! Set authorities:
//!
//! ```
//! # use srml_consensus as consensus;
//! # fn not_executed<T: consensus::Trait>() {
//! # let authority1 = T::SessionKey::default();
//! # let authority2 = T::SessionKey::default();
//! <consensus::Module<T>>::set_authorities(&[authority1, authority2])
//! # }
//! ```
//!
//! Log changes in the authorities set:
//!
//! ```
//! # use srml_consensus as consensus;
//! # use primitives::traits::Zero;
//! # use primitives::traits::OnFinalize;
//! # fn not_executed<T: consensus::Trait>() {
//! <consensus::Module<T>>::on_finalize(T::BlockNumber::zero());
//! # }
//! ```
//!
//! ### Example from SRML
//!
//! In the staking module, the `consensus::OnOfflineReport` is implemented to monitor offline
//! reporting among validators:
//!
//! ```
//! # use srml_consensus as consensus;
//! # trait Trait: consensus::Trait {
//! # }
//! #
//! # srml_support::decl_module! {
//! #     pub struct Module<T: Trait> for enum Call where origin: T::Origin {
//! #     }
//! # }
//! #
//! impl<T: Trait> consensus::OnOfflineReport<Vec<u32>> for Module<T> {
//! 	fn handle_report(reported_indices: Vec<u32>) {
//! 		for validator_index in reported_indices {
//! 			// Get validator from session module
//! 			// Process validator
//! 		}
//! 	}
//! }
//! ```
//!
//! In the GRANDPA module, we use `srml-consensus` to get the set of `next_authorities` before changing
//! this set according to the consensus algorithm (which does not rotate sessions in the *normal* way):
//!
//! ```
//! # use srml_consensus as consensus;
//! # use consensus::Trait;
//! # fn not_executed<T: consensus::Trait>() {
//! let next_authorities = <consensus::Module<T>>::authorities()
//! 			.into_iter()
//! 			.map(|key| (key, 1)) // evenly-weighted.
//! 			.collect::<Vec<(<T as Trait>::SessionKey, u64)>>();
//! # }
//! ```
//!
//! ## Related Modules
//!
//! - [Staking](../srml_staking/index.html): This module uses `srml-consensus` to monitor offline
//! reporting among validators.
//! - [Aura](../srml_aura/index.html): This module does not relate directly to `srml-consensus`,
//! but serves to manage offline reporting for the Aura consensus algorithm with its own `handle_report` method.
//! - [Grandpa](../srml_grandpa/index.html): Although GRANDPA does its own voter-set management,
//!  it has a mode where it can track `consensus`, if desired.
//!
//! ## References
//!
//! If you're interested in hacking on this module, it is useful to understand the interaction with
//! `substrate/core/inherents/src/lib.rs` and, specifically, the required implementation of `ProvideInherent`
//! to create and check inherents.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[cfg(feature = "std")]
use serde::Serialize;
use rstd::prelude::*;
use parity_codec as codec;
use codec::{Encode, Decode};
use srml_support::{storage, Parameter, decl_storage, decl_module};
use srml_support::storage::StorageValue;
use srml_support::storage::unhashed::StorageVec;
use primitives::traits::{MaybeSerializeDebug, Member};
use substrate_primitives::storage::well_known_keys;
use system::{ensure_signed, ensure_none};
use inherents::{ProvideInherent, InherentData, InherentIdentifier,
                RuntimeString, MakeFatalError};

#[cfg(any(feature = "std", test))]
use substrate_primitives::sr25519::Public as AuthorityId;


/// The identifier for consensus inherents.
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"offlrep0";

/// The error type used by this inherent.
pub type InherentError = RuntimeString;

struct AuthorityStorageVec<S: codec::Codec +
                           Default>(rstd::marker::PhantomData<S>);
impl <S: codec::Codec + Default> StorageVec for AuthorityStorageVec<S> {
    type
    Item
    =
    S;
    const
    PREFIX:
    &'static [u8]
    =
    well_known_keys::AUTHORITY_PREFIX;
}

pub type Key = Vec<u8>;
pub type KeyValue = (Vec<u8>, Vec<u8>);

/// Handling offline validator reports in a generic way.
pub trait OnOfflineReport<Offline> {
    fn handle_report(offline: Offline);
}

impl <T> OnOfflineReport<T> for () {
    fn handle_report(_: T) { }
}

/// Describes the offline-reporting extrinsic.
pub trait InherentOfflineReport {
    /// The report data type passed to the runtime during block authorship.
    type
    Inherent: codec::Codec +
    Parameter;

    /// Whether an inherent is empty and doesn't need to be included.
    fn is_empty(inherent: &Self::Inherent)
    -> bool;

    /// Handle the report.
    fn handle_report(report: Self::Inherent);

    /// Whether two reports are compatible.
    fn check_inherent(contained: &Self::Inherent, expected: &Self::Inherent)
    -> Result<(), &'static str>;
}

impl InherentOfflineReport for () {
    type
    Inherent
    =
    ();

    fn is_empty(_inherent: &()) -> bool { true }
    fn handle_report(_: ()) { }
    fn check_inherent(_: &(), _: &()) -> Result<(), &'static str> {
        Err("Explicit reporting not allowed")
    }
}

/// A variant of the `OfflineReport` that is useful for instant-finality blocks.
///
/// This assumes blocks are only finalized.
pub struct InstantFinalityReportVec<T>(::rstd::marker::PhantomData<T>);

impl <T: OnOfflineReport<Vec<u32>>> InherentOfflineReport for
 InstantFinalityReportVec<T> {
    type
    Inherent
    =
    Vec<u32>;

    fn is_empty(inherent: &Self::Inherent) -> bool { inherent.is_empty() }

    fn handle_report(report: Vec<u32>) { T::handle_report(report) }

    fn check_inherent(contained: &Self::Inherent, expected: &Self::Inherent)
     -> Result<(), &'static str> {
        contained.iter().try_for_each(|n|
                                          if !expected.contains(n) {
                                              Err("Node we believe online marked offline")
                                          } else { Ok(()) })
    }
}

pub type Log<T> = RawLog<<T as Trait>::SessionKey>;

/// Logs in this module.
#[structural_match]
pub enum RawLog<SessionKey> {

    /// Authorities set has been changed. Contains the new set of authorities.
    AuthoritiesChange(Vec<SessionKey>),
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
        impl <SessionKey> _serde::Serialize for RawLog<SessionKey> where
         SessionKey: _serde::Serialize {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    RawLog::AuthoritiesChange(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "RawLog",
                                                                  0u32,
                                                                  "AuthoritiesChange",
                                                                  __field0),
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <SessionKey: ::std::fmt::Debug> ::std::fmt::Debug for RawLog<SessionKey>
 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawLog::AuthoritiesChange(ref __self_0),) => {
                let mut debug_trait_builder =
                    f.debug_tuple("AuthoritiesChange");
                let _ = debug_trait_builder.field(&&(*__self_0));
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
        impl <SessionKey> _parity_codec::Encode for RawLog<SessionKey> where
         Vec<SessionKey>: _parity_codec::Encode,
         Vec<SessionKey>: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawLog::AuthoritiesChange(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
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
        impl <SessionKey> _parity_codec::Decode for RawLog<SessionKey> where
         Vec<SessionKey>: _parity_codec::Decode,
         Vec<SessionKey>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawLog::AuthoritiesChange(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <SessionKey: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
 RawLog<SessionKey> {
    #[inline]
    fn eq(&self, other: &RawLog<SessionKey>) -> bool {
        match (&*self, &*other) {
            (&RawLog::AuthoritiesChange(ref __self_0),
             &RawLog::AuthoritiesChange(ref __arg_1_0)) =>
            (*__self_0) == (*__arg_1_0),
        }
    }
    #[inline]
    fn ne(&self, other: &RawLog<SessionKey>) -> bool {
        match (&*self, &*other) {
            (&RawLog::AuthoritiesChange(ref __self_0),
             &RawLog::AuthoritiesChange(ref __arg_1_0)) =>
            (*__self_0) != (*__arg_1_0),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <SessionKey: ::std::cmp::Eq> ::std::cmp::Eq for RawLog<SessionKey> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<Vec<SessionKey>>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <SessionKey: ::std::clone::Clone> ::std::clone::Clone for
 RawLog<SessionKey> {
    #[inline]
    fn clone(&self) -> RawLog<SessionKey> {
        match (&*self,) {
            (&RawLog::AuthoritiesChange(ref __self_0),) =>
            RawLog::AuthoritiesChange(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}

impl <SessionKey: Member> RawLog<SessionKey> {
    /// Try to cast the log entry as AuthoritiesChange log entry.
    pub fn as_authorities_change(&self) -> Option<&[SessionKey]> {
        match *self { RawLog::AuthoritiesChange(ref item) => Some(item), }
    }
}

// Implementation for tests outside of this crate.
#[cfg(any(feature = "std", test))]
impl <N> From<RawLog<N>> for primitives::testing::DigestItem where
 N: Into<AuthorityId> {
    fn from(log: RawLog<N>) -> primitives::testing::DigestItem {
        match log {
            RawLog::AuthoritiesChange(authorities) =>
            primitives::generic::DigestItem::AuthoritiesChange(authorities.into_iter().map(Into::into).collect()),
        }
    }
}

pub trait Trait: system::Trait {
    /// Type for all log entries of this module.
    type
    Log: From<Log<Self>> +
    Into<system::DigestItemOf<Self>>;

    type
    SessionKey: Parameter +
    Default +
    MaybeSerializeDebug;
    /// Defines the offline-report type of the trait.
    /// Set to `()` if offline-reports aren't needed for this runtime.
    type
    InherentOfflineReport: InherentOfflineReport;
}

#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
    // Actual authorities set at the block execution start. Is `Some` iff
    // the set has been changed.


}
struct OriginalAuthorities<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::SessionKey>>
 for OriginalAuthorities<T> {
    type
    Query
    =
    Option<Vec<T::SessionKey>>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Consensus OriginalAuthorities".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::SessionKey>>>::key()).or_else(||
                                                                                                                                                                     Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::SessionKey>>>::key()).or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::SessionKey>>>::get(storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::SessionKey>>>::put(&val,
                                                                                                                                              storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::SessionKey>>>::kill(storage),
        };
        ret
    }
}
trait Store {
    type
    OriginalAuthorities;
}
#[doc(hidden)]
pub struct __GetByteStructOriginalAuthorities<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OriginalAuthorities:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructOriginalAuthorities<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OriginalAuthorities.get_or_init(||
                                                                    {
                                                                        let def_val:
                                                                                Option<Vec<T::SessionKey>> =
                                                                            Default::default();
                                                                        <Option<Vec<T::SessionKey>>
                                                                            as
                                                                            Encode>::encode(&def_val)
                                                                    }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    OriginalAuthorities
    =
    OriginalAuthorities<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("OriginalAuthorities"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::SessionKey>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructOriginalAuthorities::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
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
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("OriginalAuthorities"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::SessionKey>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructOriginalAuthorities::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Consensus" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "Vec < T :: SessionKey > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, Vec < u8 > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "Vec < T :: SessionKey > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, Vec < u8 > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    pub authorities: Vec<T::SessionKey>,
    #[serde(with = "substrate_primitives::bytes")]
    pub code: Vec<u8>,
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
         Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
         {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "GenesisConfig",
                                                               false as usize
                                                                   + 1 + 1) {
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
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "code",
                                                                    {
                                                                        struct __SerializeWith<'__a,
                                                                                               T: Trait +
                                                                                               '__a>
                                                                               where
                                                                               Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
                                                                               Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize {
                                                                            values: (&'__a Vec<u8>,),
                                                                            phantom: _serde::export::PhantomData<GenesisConfig<T>>,
                                                                        }
                                                                        impl <'__a,
                                                                              T: Trait +
                                                                              '__a>
                                                                         _serde::Serialize
                                                                         for
                                                                         __SerializeWith<'__a,
                                                                                         T>
                                                                         where
                                                                         Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
                                                                         Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
                                                                         {
                                                                            fn serialize<__S>(&self,
                                                                                              __s:
                                                                                                  __S)
                                                                             ->
                                                                                 _serde::export::Result<__S::Ok,
                                                                                                        __S::Error>
                                                                             where
                                                                             __S: _serde::Serializer {
                                                                                substrate_primitives::bytes::serialize(self.values.0,
                                                                                                                       __s)
                                                                            }
                                                                        }
                                                                        &__SerializeWith{values:
                                                                                             (&self.code,),
                                                                                         phantom:
                                                                                             _serde::export::PhantomData::<GenesisConfig<T>>,}
                                                                    }) {
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
         Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
         {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, }
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
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 2")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "authorities" =>
                            _serde::export::Ok(__Field::__field0),
                            "code" => _serde::export::Ok(__Field::__field1),
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
                            b"code" => _serde::export::Ok(__Field::__field1),
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
                       Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            match match _serde::de::SeqAccess::next_element::<Vec<T::SessionKey>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct GenesisConfig with 2 elements"));
                                }
                            };
                        let __field1 =
                            match {
                                      struct __DeserializeWith<'de, T: Trait>
                                             where
                                             Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                                             Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                                          value: Vec<u8>,
                                          phantom: _serde::export::PhantomData<GenesisConfig<T>>,
                                          lifetime: _serde::export::PhantomData<&'de ()>,
                                      }
                                      impl <'de, T: Trait>
                                       _serde::Deserialize<'de> for
                                       __DeserializeWith<'de, T> where
                                       Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                                       Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
                                       {
                                          fn deserialize<__D>(__deserializer:
                                                                  __D)
                                           ->
                                               _serde::export::Result<Self,
                                                                      __D::Error>
                                           where
                                           __D: _serde::Deserializer<'de> {
                                              _serde::export::Ok(__DeserializeWith{value:
                                                                                       match substrate_primitives::bytes::deserialize(__deserializer)
                                                                                           {
                                                                                           _serde::export::Ok(__val)
                                                                                           =>
                                                                                           __val,
                                                                                           _serde::export::Err(__err)
                                                                                           =>
                                                                                           {
                                                                                               return _serde::export::Err(__err);
                                                                                           }
                                                                                       },
                                                                                   phantom:
                                                                                       _serde::export::PhantomData,
                                                                                   lifetime:
                                                                                       _serde::export::PhantomData,})
                                          }
                                      }
                                      _serde::export::Option::map(match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de,
                                                                                                                                T>>(&mut __seq)
                                                                      {
                                                                      _serde::export::Ok(__val)
                                                                      =>
                                                                      __val,
                                                                      _serde::export::Err(__err)
                                                                      => {
                                                                          return _serde::export::Err(__err);
                                                                      }
                                                                  },
                                                                  |__wrap|
                                                                      __wrap.value)
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct GenesisConfig with 2 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{authorities:
                                                             __field0,
                                                         code: __field1,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<Vec<T::SessionKey>> =
                            _serde::export::None;
                        let mut __field1: _serde::export::Option<Vec<u8>> =
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
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<T::SessionKey>>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("code"));
                                    }
                                    __field1 =
                                        _serde::export::Some({
                                                                 struct __DeserializeWith<'de,
                                                                                          T: Trait>
                                                                        where
                                                                        Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                                                                        Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                                                                     value: Vec<u8>,
                                                                     phantom: _serde::export::PhantomData<GenesisConfig<T>>,
                                                                     lifetime: _serde::export::PhantomData<&'de ()>,
                                                                 }
                                                                 impl <'de,
                                                                       T: Trait>
                                                                  _serde::Deserialize<'de>
                                                                  for
                                                                  __DeserializeWith<'de,
                                                                                    T>
                                                                  where
                                                                  Vec<T::SessionKey>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                                                                  Vec<u8>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
                                                                  {
                                                                     fn deserialize<__D>(__deserializer:
                                                                                             __D)
                                                                      ->
                                                                          _serde::export::Result<Self,
                                                                                                 __D::Error>
                                                                      where
                                                                      __D: _serde::Deserializer<'de> {
                                                                         _serde::export::Ok(__DeserializeWith{value:
                                                                                                                  match substrate_primitives::bytes::deserialize(__deserializer)
                                                                                                                      {
                                                                                                                      _serde::export::Ok(__val)
                                                                                                                      =>
                                                                                                                      __val,
                                                                                                                      _serde::export::Err(__err)
                                                                                                                      =>
                                                                                                                      {
                                                                                                                          return _serde::export::Err(__err);
                                                                                                                      }
                                                                                                                  },
                                                                                                              phantom:
                                                                                                                  _serde::export::PhantomData,
                                                                                                              lifetime:
                                                                                                                  _serde::export::PhantomData,})
                                                                     }
                                                                 }
                                                                 match _serde::de::MapAccess::next_value::<__DeserializeWith<'de,
                                                                                                                             T>>(&mut __map)
                                                                     {
                                                                     _serde::export::Ok(__wrapper)
                                                                     =>
                                                                     __wrapper.value,
                                                                     _serde::export::Err(__err)
                                                                     => {
                                                                         return _serde::export::Err(__err);
                                                                     }
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
                        let __field1 =
                            match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None =>
                                return _serde::export::Err(<__A::Error as
                                                               _serde::de::Error>::missing_field("code")),
                            };
                        _serde::export::Ok(GenesisConfig{authorities:
                                                             __field0,
                                                         code: __field1,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["authorities", "code"];
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
        GenesisConfig{authorities: Default::default(),
                      code: Default::default(),}
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
                                                                                            u32).to_keyed_vec(well_known_keys::AUTHORITY_PREFIX),
                                                                                       v.encode());
                                                                    });
                 storage.insert(well_known_keys::AUTHORITY_COUNT.to_vec(),
                                auth_count.encode());
                 storage.insert(well_known_keys::CODE.to_vec(),
                                config.code.clone());
             })(r, c, &self);
        Ok(())
    }
}














// if we have already saved original set before, do not overwrite







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
        if let Some(original_authorities) = <OriginalAuthorities<T>>::take() {
            let current_authorities =
                AuthorityStorageVec::<T::SessionKey>::items();
            if current_authorities != original_authorities {
                Self::deposit_log(RawLog::AuthoritiesChange(current_authorities));
            }
        }
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
    #[doc = r" Report some misbehavior."]
    fn report_misbehavior(origin: T::Origin, _report: Vec<u8>)
     -> ::srml_support::dispatch::Result {
        { ensure_signed(origin)?; }
        Ok(())
    }
    #[doc =
          r" Note that the previous block's validator missed its opportunity to propose a block."]
    fn note_offline(origin: T::Origin,
                    offline:
                        <T::InherentOfflineReport as
                        InherentOfflineReport>::Inherent)
     -> ::srml_support::dispatch::Result {
        {
            ensure_none(origin)?;
            T::InherentOfflineReport::handle_report(offline);
        }
        Ok(())
    }
    #[doc = r" Make some on-chain remark."]
    fn remark(origin: T::Origin, _remark: Vec<u8>)
     -> ::srml_support::dispatch::Result {
        { ensure_signed(origin)?; }
        Ok(())
    }
    #[doc =
          r" Set the number of pages in the WebAssembly environment's heap."]
    fn set_heap_pages(pages: u64) -> ::srml_support::dispatch::Result {
        {
            storage::unhashed::put_raw(well_known_keys::HEAP_PAGES,
                                       &pages.encode());
        }
        Ok(())
    }
    #[doc = r" Set the new code."]
    pub fn set_code(new: Vec<u8>) -> ::srml_support::dispatch::Result {
        { storage::unhashed::put_raw(well_known_keys::CODE, &new); }
        Ok(())
    }
    #[doc = r" Set some items of storage."]
    fn set_storage(items: Vec<KeyValue>) -> ::srml_support::dispatch::Result {
        { for i in &items { storage::unhashed::put_raw(&i.0, &i.1); } }
        Ok(())
    }
    #[doc = r" Kill some items from storage."]
    fn kill_storage(keys: Vec<Key>) -> ::srml_support::dispatch::Result {
        { for key in &keys { storage::unhashed::kill(&key); } }
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

    #[allow(non_camel_case_types)]
    note_offline(<T::InherentOfflineReport as
                 InherentOfflineReport>::Inherent),

    #[allow(non_camel_case_types)]
    remark(Vec<u8>),

    #[allow(non_camel_case_types)]
    set_heap_pages(u64),

    #[allow(non_camel_case_types)]
    set_code(Vec<u8>),

    #[allow(non_camel_case_types)]
    set_storage(Vec<KeyValue>),

    #[allow(non_camel_case_types)]
    kill_storage(Vec<Key>),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for Call<T> where
         <T::InherentOfflineReport as
         InherentOfflineReport>::Inherent: _parity_codec::Encode,
         <T::InherentOfflineReport as
         InherentOfflineReport>::Inherent: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::report_misbehavior(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    Call::note_offline(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    Call::remark(ref aa) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                    }
                    Call::set_heap_pages(ref aa) => {
                        dest.push_byte(3usize as u8);
                        dest.push(aa);
                    }
                    Call::set_code(ref aa) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                    }
                    Call::set_storage(ref aa) => {
                        dest.push_byte(5usize as u8);
                        dest.push(aa);
                    }
                    Call::kill_storage(ref aa) => {
                        dest.push_byte(6usize as u8);
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
         <T::InherentOfflineReport as
         InherentOfflineReport>::Inherent: _parity_codec::Decode,
         <T::InherentOfflineReport as
         InherentOfflineReport>::Inherent: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::report_misbehavior(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::note_offline(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(Call::remark(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 3usize as u8 => {
                        Some(Call::set_heap_pages(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 4usize as u8 => {
                        Some(Call::set_code(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 5usize as u8 => {
                        Some(Call::set_storage(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 6usize as u8 => {
                        Some(Call::kill_storage(_parity_codec::Decode::decode(input)?))
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
            Call::note_offline(ref offline) =>
            Call::note_offline((*offline).clone()),
            Call::remark(ref _remark) => Call::remark((*_remark).clone()),
            Call::set_heap_pages(ref pages) =>
            Call::set_heap_pages((*pages).clone()),
            Call::set_code(ref new) => Call::set_code((*new).clone()),
            Call::set_storage(ref items) =>
            Call::set_storage((*items).clone()),
            Call::kill_storage(ref keys) =>
            Call::kill_storage((*keys).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/consensus/src/lib.rs",
                                             292u32, 1u32))
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
                                                       &("srml/consensus/src/lib.rs",
                                                         292u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::note_offline(ref offline) => {
                let self_params = (offline,);
                if let Call::note_offline(ref offline) = *_other {
                    self_params == (offline,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/consensus/src/lib.rs",
                                                         292u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::remark(ref _remark) => {
                let self_params = (_remark,);
                if let Call::remark(ref _remark) = *_other {
                    self_params == (_remark,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/consensus/src/lib.rs",
                                                         292u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_heap_pages(ref pages) => {
                let self_params = (pages,);
                if let Call::set_heap_pages(ref pages) = *_other {
                    self_params == (pages,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/consensus/src/lib.rs",
                                                         292u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_code(ref new) => {
                let self_params = (new,);
                if let Call::set_code(ref new) = *_other {
                    self_params == (new,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/consensus/src/lib.rs",
                                                         292u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_storage(ref items) => {
                let self_params = (items,);
                if let Call::set_storage(ref items) = *_other {
                    self_params == (items,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/consensus/src/lib.rs",
                                                         292u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::kill_storage(ref keys) => {
                let self_params = (keys,);
                if let Call::kill_storage(ref keys) = *_other {
                    self_params == (keys,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/consensus/src/lib.rs",
                                                         292u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/consensus/src/lib.rs",
                                             292u32, 1u32))
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
            Call::note_offline(ref offline) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"note_offline",
                                                               &(offline.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::remark(ref _remark) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"remark",
                                                               &(_remark.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_heap_pages(ref pages) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_heap_pages",
                                                               &(pages.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_code(ref new) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_code",
                                                               &(new.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_storage(ref items) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_storage",
                                                               &(items.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::kill_storage(ref keys) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"kill_storage",
                                                               &(keys.clone(),))
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
                                           &("srml/consensus/src/lib.rs",
                                             292u32, 1u32))
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
            Call::note_offline(offline) => {
                <Module<T>>::note_offline(_origin, offline)
            }
            Call::remark(_remark) => { <Module<T>>::remark(_origin, _remark) }
            Call::set_heap_pages(pages) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_heap_pages(pages)
                }
            }
            Call::set_code(new) => {
                { system::ensure_root(_origin)?; <Module<T>>::set_code(new) }
            }
            Call::set_storage(items) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_storage(items)
                }
            }
            Call::kill_storage(keys) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::kill_storage(keys)
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
                                                       &("srml/consensus/src/lib.rs",
                                                         292u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Report some misbehavior."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("note_offline"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("offline"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("<T::InherentOfflineReport as InherentOfflineReport>::Inherent"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Note that the previous block's validator missed its opportunity to propose a block."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("remark"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("_remark"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Make some on-chain remark."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_heap_pages"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("pages"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("u64"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the number of pages in the WebAssembly environment's heap."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_code"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("new"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set the new code."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_storage"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("items"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<KeyValue>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Set some items of storage."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("kill_storage"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("keys"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Vec<Key>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Kill some items from storage."]),}]
    }
}
impl <T: Trait> Module<T> {
    /// Get the current set of authorities. These are the session keys.
    pub fn authorities() -> Vec<T::SessionKey> {
        AuthorityStorageVec::<T::SessionKey>::items()
    }
    /// Set the current set of authorities' session keys. Will not exceed the current
    /// authorities count, even if the given `authorities` is longer.
    ///
    /// Called by `rotate_session` only.
    pub fn set_authorities(authorities: &[T::SessionKey]) {
        let current_authorities =
            AuthorityStorageVec::<T::SessionKey>::items();
        if current_authorities != authorities {
            Self::save_original_authorities(Some(current_authorities));
            AuthorityStorageVec::<T::SessionKey>::set_items(authorities);
        }
    }
    /// Set the total number of authorities.
    pub fn set_authority_count(count: u32) {
        Self::save_original_authorities(None);
        AuthorityStorageVec::<T::SessionKey>::set_count(count);
    }
    /// Set a single authority by index.
    pub fn set_authority(index: u32, key: &T::SessionKey) {
        let current_authority =
            AuthorityStorageVec::<T::SessionKey>::item(index);
        if current_authority != *key {
            Self::save_original_authorities(None);
            AuthorityStorageVec::<T::SessionKey>::set_item(index, key);
        }
    }
    /// Save original authorities set.
    fn save_original_authorities(current_authorities:
                                     Option<Vec<T::SessionKey>>) {
        if OriginalAuthorities::<T>::get().is_some() { return; }
        <OriginalAuthorities<T>>::put(current_authorities.unwrap_or_else(||
                                                                             AuthorityStorageVec::<T::SessionKey>::items()));
    }
    /// Deposit one of this module's logs.
    fn deposit_log(log: Log<T>) {
        <system::Module<T>>::deposit_log(<T as Trait>::from(log).into());
    }
}
/// Implementing `ProvideInherent` enables this module to create and check inherents.
impl <T: Trait> ProvideInherent for Module<T> {
    /// The call type of the module.
    type
    Call
    =
    Call<T>;
    /// The error returned by `check_inherent`.
    type
    Error
    =
    MakeFatalError<RuntimeString>;
    /// The inherent identifier used by this inherent.
    const
    INHERENT_IDENTIFIER:
    InherentIdentifier
    =
    INHERENT_IDENTIFIER;
    /// Creates an inherent from the `InherentData`.
    fn create_inherent(data: &InherentData) -> Option<Self::Call> {
        if let Ok(Some(data)) =
               data.get_data::<<T::InherentOfflineReport as
                               InherentOfflineReport>::Inherent>(&INHERENT_IDENTIFIER)
               {
            if <T::InherentOfflineReport as
                   InherentOfflineReport>::is_empty(&data) {
                None
            } else { Some(Call::note_offline(data)) }
        } else { None }
    }
    /// Verify the validity of the given inherent.
    fn check_inherent(call: &Self::Call, data: &InherentData)
     -> Result<(), Self::Error> {
        let offline =
            match call {
                Call::note_offline(ref offline) => offline,
                _ => return Ok(()),
            };
        let expected =
            data.get_data::<<T::InherentOfflineReport as
                            InherentOfflineReport>::Inherent>(&INHERENT_IDENTIFIER)?.ok_or(RuntimeString::from("No `offline_report` found in the inherent data!"))?;
        <T::InherentOfflineReport as
            InherentOfflineReport>::check_inherent(&offline,
                                                   &expected).map_err(|e|
                                                                          RuntimeString::from(e).into())
    }
}
