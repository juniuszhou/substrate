#![feature(prelude_import)]
#![no_std]
// Copyright 2019 Parity Technologies (UK) Ltd.
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

//! SRML module that tracks the last finalized block, as perceived by block authors.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[macro_use]
extern crate srml_support;

use inherents::{RuntimeString, InherentIdentifier, ProvideInherent,
                InherentData, MakeFatalError};
use srml_support::StorageValue;
use primitives::traits::{As, One, Zero};
use rstd::{prelude::*, result, cmp, vec};
use parity_codec::Decode;
use srml_system::{ensure_none, Trait as SystemTrait};

#[cfg(feature = "std")]
use parity_codec::Encode;

const DEFAULT_WINDOW_SIZE: u64 = 101;
const DEFAULT_DELAY: u64 = 1000;

/// The identifier for the `finalnum` inherent.
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"finalnum";

/// Auxiliary trait to extract finalized inherent data.
pub trait FinalizedInherentData<N: Decode> {
    /// Get finalized inherent data.
    fn finalized_number(&self)
    -> Result<N, RuntimeString>;
}

impl <N: Decode> FinalizedInherentData<N> for InherentData {
    fn finalized_number(&self) -> Result<N, RuntimeString> {
        self.get_data(&INHERENT_IDENTIFIER).and_then(|r|
                                                         r.ok_or_else(||
                                                                          "Finalized number inherent data not found".into()))
    }
}

/// Provider for inherent data.
#[cfg(feature = "std")]
pub struct InherentDataProvider<F, N> {
    inner: F,
    _marker: std::marker::PhantomData<N>,
}

#[cfg(feature = "std")]
impl <F, N> InherentDataProvider<F, N> {
    pub fn new(final_oracle: F) -> Self {
        InherentDataProvider{inner: final_oracle,
                             _marker: Default::default(),}
    }
}

#[cfg(feature = "std")]
impl <F, N: Encode> inherents::ProvideInherentData for
 InherentDataProvider<F, N> where F: Fn() -> Result<N, RuntimeString> {
    fn inherent_identifier(&self) -> &'static InherentIdentifier {
        &INHERENT_IDENTIFIER
    }

    fn provide_inherent_data(&self, inherent_data: &mut InherentData)
     -> Result<(), RuntimeString> {
        (self.inner)().and_then(|n|
                                    inherent_data.put_data(INHERENT_IDENTIFIER,
                                                           &n))
    }

    fn error_to_string(&self, _error: &[u8]) -> Option<String> {
        Some(





             // when initialized through config this is set in the beginning.







             // prune off the front of the list -- typically 1 except for when
             // the sample size has just been shrunk.
             // take into account the item we haven't pushed yet.



             // find the position in the ordered list where the new item goes.





             // compute average.




             // the delay is the latency plus half the window size.
             // median may be at most n - delay







             // make hint only when not same as last to avoid bloat.




















             ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["no further information"],
                                                                &match () {
                                                                     () => [],
                                                                 })))
    }
}
pub trait Trait: SystemTrait {
    /// Something which can be notified when the timestamp is set. Set this to `()` if not needed.
    type
    OnFinalizationStalled: OnFinalizationStalled<Self::BlockNumber>;
}
#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
}
#[doc = " Recent hints."]
struct RecentHints<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>
 for RecentHints<T> {
    type
    Query
    =
    Vec<T::BlockNumber>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp RecentHints".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::key()).unwrap_or_else(||
                                                                                                                                                                             Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::put(&val,
                                                                                                                                           storage);
        ret
    }
}
#[doc = " Ordered recent hints."]
struct OrderedHints<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>
 for OrderedHints<T> {
    type
    Query
    =
    Vec<T::BlockNumber>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp OrderedHints".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::key()).unwrap_or_else(||
                                                                                                                                                                             Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::put(&val,
                                                                                                                                           storage);
        ret
    }
}
#[doc = " The median."]
struct Median<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for Median<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp Median".as_bytes() }
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
      " The number of recent samples to keep from this chain. Default is n-100"]
pub struct WindowSize<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for WindowSize<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp WindowSize".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                        T::BlockNumber::sa(DEFAULT_WINDOW_SIZE))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                         T::BlockNumber::sa(DEFAULT_WINDOW_SIZE))
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
#[doc = " The delay after which point things become suspicious."]
pub struct ReportLatency<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for ReportLatency<T> {
    type
    Query
    =
    T::BlockNumber;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp ReportLatency".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                        T::BlockNumber::sa(DEFAULT_DELAY))
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::key()).unwrap_or_else(||
                                                                                                                                                                         T::BlockNumber::sa(DEFAULT_DELAY))
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
#[doc = " Final hint to apply in the block. `None` means \"same as parent\"."]
struct Update<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>
 for Update<T> {
    type
    Query
    =
    Option<T::BlockNumber>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp Update".as_bytes() }
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
struct Initialized<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>
 for Initialized<T> {
    type
    Query
    =
    bool;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Timestamp Initialized".as_bytes() }
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
    RecentHints;
    type
    OrderedHints;
    type
    Median;
    type
    WindowSize;
    type
    ReportLatency;
    type
    Update;
    type
    Initialized;
}
#[doc(hidden)]
pub struct __GetByteStructRecentHints<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_RecentHints:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructRecentHints<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_RecentHints.get_or_init(||
                                                            {
                                                                let def_val:
                                                                        Vec<T::BlockNumber> =
                                                                    Default::default();
                                                                <Vec<T::BlockNumber>
                                                                    as
                                                                    Encode>::encode(&def_val)
                                                            }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructOrderedHints<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OrderedHints:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructOrderedHints<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OrderedHints.get_or_init(||
                                                             {
                                                                 let def_val:
                                                                         Vec<T::BlockNumber> =
                                                                     Default::default();
                                                                 <Vec<T::BlockNumber>
                                                                     as
                                                                     Encode>::encode(&def_val)
                                                             }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructMedian<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Median:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructMedian<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Median.get_or_init(||
                                                       {
                                                           let def_val:
                                                                   T::BlockNumber =
                                                               Default::default();
                                                           <T::BlockNumber as
                                                               Encode>::encode(&def_val)
                                                       }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructWindowSize<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_WindowSize:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructWindowSize<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_WindowSize.get_or_init(||
                                                           {
                                                               let def_val:
                                                                       T::BlockNumber =
                                                                   T::BlockNumber::sa(DEFAULT_WINDOW_SIZE);
                                                               <T::BlockNumber
                                                                   as
                                                                   Encode>::encode(&def_val)
                                                           }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructReportLatency<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ReportLatency:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructReportLatency<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ReportLatency.get_or_init(||
                                                              {
                                                                  let def_val:
                                                                          T::BlockNumber =
                                                                      T::BlockNumber::sa(DEFAULT_DELAY);
                                                                  <T::BlockNumber
                                                                      as
                                                                      Encode>::encode(&def_val)
                                                              }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructUpdate<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Update:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructUpdate<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Update.get_or_init(||
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
pub struct __GetByteStructInitialized<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Initialized:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructInitialized<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Initialized.get_or_init(||
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
    RecentHints
    =
    RecentHints<T>;
    type
    OrderedHints
    =
    OrderedHints<T>;
    type
    Median
    =
    Median<T>;
    type
    WindowSize
    =
    WindowSize<T>;
    type
    ReportLatency
    =
    ReportLatency<T>;
    type
    Update
    =
    Update<T>;
    type
    Initialized
    =
    Initialized<T>;
}
impl <T: 'static + Trait> Module<T> {
    #[doc = " Recent hints."]
    pub fn recent_hints() -> Vec<T::BlockNumber> {
        <RecentHints<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " Ordered recent hints."]
    pub fn ordered_hints() -> Vec<T::BlockNumber> {
        <OrderedHints<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The median."]
    pub fn median() -> T::BlockNumber {
        <Median<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc =
          " The number of recent samples to keep from this chain. Default is n-100"]
    pub fn window_size() -> T::BlockNumber {
        <WindowSize<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc = " The delay after which point things become suspicious."]
    pub fn report_latency() -> T::BlockNumber {
        <ReportLatency<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    pub fn initialized() -> bool {
        <Initialized<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RecentHints"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::BlockNumber>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRecentHints::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Recent hints."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("OrderedHints"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::BlockNumber>")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructOrderedHints::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Ordered recent hints."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Median"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMedian::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The median."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("WindowSize"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructWindowSize::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of recent samples to keep from this chain. Default is n-100"]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReportLatency"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructReportLatency::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The delay after which point things become suspicious."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Update"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructUpdate::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Final hint to apply in the block. `None` means \"same as parent\"."]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Initialized"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("bool")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructInitialized::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
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
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("RecentHints"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::BlockNumber>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructRecentHints::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Recent hints."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("OrderedHints"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Vec<T::BlockNumber>")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructOrderedHints::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Ordered recent hints."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Median"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructMedian::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The median."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("WindowSize"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructWindowSize::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The number of recent samples to keep from this chain. Default is n-100"]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("ReportLatency"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructReportLatency::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" The delay after which point things become suspicious."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Update"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::BlockNumber")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructUpdate::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[" Final hint to apply in the block. `None` means \"same as parent\"."]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Initialized"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("bool")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructInitialized::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Timestamp" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: BlockNumber : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    #[doc =
          " The number of recent samples to keep from this chain. Default is n-100"]
    pub window_size: T::BlockNumber,
    #[doc = " The delay after which point things become suspicious."]
    pub report_latency: T::BlockNumber,
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
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
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
                                                                    "windowSize",
                                                                    &self.window_size)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "reportLatency",
                                                                    &self.report_latency)
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
         T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            "windowSize" =>
                            _serde::export::Ok(__Field::__field0),
                            "reportLatency" =>
                            _serde::export::Ok(__Field::__field1),
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
                            b"windowSize" =>
                            _serde::export::Ok(__Field::__field0),
                            b"reportLatency" =>
                            _serde::export::Ok(__Field::__field1),
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
                       T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::BlockNumber: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                                                                                                 &"struct GenesisConfig with 2 elements"));
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
                                                                                                 &"struct GenesisConfig with 2 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{window_size:
                                                             __field0,
                                                         report_latency:
                                                             __field1,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0:
                                _serde::export::Option<T::BlockNumber> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<T::BlockNumber> =
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
                                                                       _serde::de::Error>::duplicate_field("windowSize"));
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
                                                                       _serde::de::Error>::duplicate_field("reportLatency"));
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
                            }
                        }
                        let __field0 =
                            match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("windowSize")
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
                                match _serde::private::de::missing_field("reportLatency")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{window_size:
                                                             __field0,
                                                         report_latency:
                                                             __field1,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["windowSize", "reportLatency"];
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
        GenesisConfig{window_size: T::BlockNumber::sa(DEFAULT_WINDOW_SIZE),
                      report_latency: T::BlockNumber::sa(DEFAULT_DELAY),}
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
                (|_| <[_]>::into_vec(box [T::BlockNumber::zero()]))(&self);
            <RecentHints<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::put(&v,
                                                                                                                                               &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v =
                (|_| <[_]>::into_vec(box [T::BlockNumber::zero()]))(&self);
            <OrderedHints<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<Vec<T::BlockNumber>>>::put(&v,
                                                                                                                                               &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v = (|_| T::BlockNumber::zero())(&self);
            <Median<T> as
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
                      config.window_size.clone()))(&self);
            <WindowSize<T> as
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
                      config.report_latency.clone()))(&self);
            <ReportLatency<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::BlockNumber>>::put(&v,
                                                                                                                                          &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v = (|_| false)(&self);
            <Initialized<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<bool>>::put(&v,
                                                                                                                                &storage);
        }
        let r = storage.into_inner();
        (|_, _, _| { })(r, c, &self);
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
    fn on_finalize(_block_number_not_used: T::BlockNumber) {
        Self::update_hint(<Self as Store>::take())
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
    #[doc = r" Hint that the author of this block thinks the best finalized"]
    #[doc = r" block is the given number."]
    fn final_hint(origin: T::Origin, hint: T::BlockNumber)
     -> ::srml_support::dispatch::Result {
        {
            ensure_none(origin)?;
            if !!<Self as Store>::exists() {
                {
                    ::std::rt::begin_panic("Final hint must be updated only once in the block",
                                           &("srml/finality-tracker/src/lib.rs",
                                             121u32, 4u32))
                }
            };
            if !(srml_system::Module::<T>::block_number() >= hint) {
                {
                    {
                        ::std::rt::begin_panic("Finalized height above block number",
                                               &("srml/finality-tracker/src/lib.rs",
                                                 122u32, 4u32))
                    }
                }
            };
            <Self as Store>::put(hint);
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
    final_hint(
               #[codec(compact)]
               T::BlockNumber),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for Call<T> where
         T::BlockNumber: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::final_hint(ref aa) => {
                        dest.push_byte(0usize as u8);
                        {
                            dest.push(&<<T::BlockNumber as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::BlockNumber>>::from(aa));
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
         T::BlockNumber: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::final_hint(<<T::BlockNumber as
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
            Call::final_hint(ref hint) => Call::final_hint((*hint).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/finality-tracker/src/lib.rs",
                                             115u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::final_hint(ref hint) => {
                let self_params = (hint,);
                if let Call::final_hint(ref hint) = *_other {
                    self_params == (hint,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/finality-tracker/src/lib.rs",
                                                         115u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/finality-tracker/src/lib.rs",
                                             115u32, 1u32))
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
            Call::final_hint(ref hint) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"final_hint",
                                                               &(hint.clone(),))
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
                                           &("srml/finality-tracker/src/lib.rs",
                                             115u32, 1u32))
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
            Call::final_hint(hint) => {
                <Module<T>>::final_hint(_origin, hint)
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
                                                       &("srml/finality-tracker/src/lib.rs",
                                                         115u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("final_hint"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("hint"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::BlockNumber>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" Hint that the author of this block thinks the best finalized",
                                                                                                             r" block is the given number."]),}]
    }
}
impl <T: Trait> Module<T> {
    fn update_hint(hint: Option<T::BlockNumber>) {
        if !Self::initialized() {
            <Self as
                Store>::put(<[_]>::into_vec(box [T::BlockNumber::zero()]));
            <Self as
                Store>::put(<[_]>::into_vec(box [T::BlockNumber::zero()]));
            <Self as Store>::put(T::BlockNumber::zero());
            <Self as Store>::put(true);
        }
        let mut recent = Self::recent_hints();
        let mut ordered = Self::ordered_hints();
        let window_size =
            cmp::max(T::BlockNumber::one(), Self::window_size());
        let hint =
            hint.unwrap_or_else(||
                                    recent.last().expect("always at least one recent sample; qed").clone());
        {
            let to_prune =
                (recent.len() + 1).saturating_sub(window_size.as_() as usize);
            for drained in recent.drain(..to_prune) {
                let idx =
                    ordered.binary_search(&drained).expect("recent and ordered contain the same items; qed");
                ordered.remove(idx);
            }
        }
        let ordered_idx =
            ordered.binary_search(&hint).unwrap_or_else(|idx| idx);
        ordered.insert(ordered_idx, hint);
        recent.push(hint);
        let two = T::BlockNumber::one() + T::BlockNumber::one();
        let median =
            {
                let len = ordered.len();
                if !(len > 0) {
                    {
                        ::std::rt::begin_panic("pruning dictated by window_size which is always saturated at 1; qed",
                                               &("srml/finality-tracker/src/lib.rs",
                                                 178u32, 4u32))
                    }
                };
                if len % 2 == 0 {
                    let a = ordered[len / 2];
                    let b = ordered[(len / 2) - 1];
                    (a + b) / two
                } else { ordered[len / 2] }
            };
        let our_window_size = recent.len();
        <Self as Store>::put(recent);
        <Self as Store>::put(ordered);
        <Self as Store>::put(median);
        if T::BlockNumber::sa(our_window_size as u64) == window_size {
            let now = srml_system::Module::<T>::block_number();
            let latency = Self::report_latency();
            let delay = latency + (window_size / two);
            if median + delay <= now {
                T::OnFinalizationStalled::on_stalled(window_size -
                                                         T::BlockNumber::one());
            }
        }
    }
}
/// Called when finalization stalled at a given number.
pub trait OnFinalizationStalled<N> {
    /// The parameter here is how many more blocks to wait before applying
    /// changes triggered by finality stalling.
    fn on_stalled(further_wait: N);
}
macro_rules! impl_on_stalled((  ) => (
                             impl < N > OnFinalizationStalled < N > for (  ) {
                             fn on_stalled ( _ : N ) {  } } ) ; (
                             $ ( $ t : ident ) * ) => {
                             impl < NUM : Clone , $ (
                             $ t : OnFinalizationStalled < NUM > ) , * >
                             OnFinalizationStalled < NUM > for ( $ ( $ t , ) *
                             ) {
                             fn on_stalled ( further_wait : NUM ) {
                             $ (
                             $ t :: on_stalled ( further_wait . clone (  ) ) ;
                             ) * } } });
impl <NUM: Clone, A: OnFinalizationStalled<NUM>,
      B: OnFinalizationStalled<NUM>, C: OnFinalizationStalled<NUM>,
      D: OnFinalizationStalled<NUM>, E: OnFinalizationStalled<NUM>,
      F: OnFinalizationStalled<NUM>, G: OnFinalizationStalled<NUM>,
      H: OnFinalizationStalled<NUM>, I: OnFinalizationStalled<NUM>,
      J: OnFinalizationStalled<NUM>, K: OnFinalizationStalled<NUM>,
      L: OnFinalizationStalled<NUM>, M: OnFinalizationStalled<NUM>,
      N: OnFinalizationStalled<NUM>, O: OnFinalizationStalled<NUM>,
      P: OnFinalizationStalled<NUM>, Q: OnFinalizationStalled<NUM>,
      R: OnFinalizationStalled<NUM>, S: OnFinalizationStalled<NUM>>
 OnFinalizationStalled<NUM> for
 (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        A::on_stalled(further_wait.clone());
        B::on_stalled(further_wait.clone());
        C::on_stalled(further_wait.clone());
        D::on_stalled(further_wait.clone());
        E::on_stalled(further_wait.clone());
        F::on_stalled(further_wait.clone());
        G::on_stalled(further_wait.clone());
        H::on_stalled(further_wait.clone());
        I::on_stalled(further_wait.clone());
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, B: OnFinalizationStalled<NUM>,
      C: OnFinalizationStalled<NUM>, D: OnFinalizationStalled<NUM>,
      E: OnFinalizationStalled<NUM>, F: OnFinalizationStalled<NUM>,
      G: OnFinalizationStalled<NUM>, H: OnFinalizationStalled<NUM>,
      I: OnFinalizationStalled<NUM>, J: OnFinalizationStalled<NUM>,
      K: OnFinalizationStalled<NUM>, L: OnFinalizationStalled<NUM>,
      M: OnFinalizationStalled<NUM>, N: OnFinalizationStalled<NUM>,
      O: OnFinalizationStalled<NUM>, P: OnFinalizationStalled<NUM>,
      Q: OnFinalizationStalled<NUM>, R: OnFinalizationStalled<NUM>,
      S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM> for
 (B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        B::on_stalled(further_wait.clone());
        C::on_stalled(further_wait.clone());
        D::on_stalled(further_wait.clone());
        E::on_stalled(further_wait.clone());
        F::on_stalled(further_wait.clone());
        G::on_stalled(further_wait.clone());
        H::on_stalled(further_wait.clone());
        I::on_stalled(further_wait.clone());
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, C: OnFinalizationStalled<NUM>,
      D: OnFinalizationStalled<NUM>, E: OnFinalizationStalled<NUM>,
      F: OnFinalizationStalled<NUM>, G: OnFinalizationStalled<NUM>,
      H: OnFinalizationStalled<NUM>, I: OnFinalizationStalled<NUM>,
      J: OnFinalizationStalled<NUM>, K: OnFinalizationStalled<NUM>,
      L: OnFinalizationStalled<NUM>, M: OnFinalizationStalled<NUM>,
      N: OnFinalizationStalled<NUM>, O: OnFinalizationStalled<NUM>,
      P: OnFinalizationStalled<NUM>, Q: OnFinalizationStalled<NUM>,
      R: OnFinalizationStalled<NUM>, S: OnFinalizationStalled<NUM>>
 OnFinalizationStalled<NUM> for
 (C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        C::on_stalled(further_wait.clone());
        D::on_stalled(further_wait.clone());
        E::on_stalled(further_wait.clone());
        F::on_stalled(further_wait.clone());
        G::on_stalled(further_wait.clone());
        H::on_stalled(further_wait.clone());
        I::on_stalled(further_wait.clone());
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, D: OnFinalizationStalled<NUM>,
      E: OnFinalizationStalled<NUM>, F: OnFinalizationStalled<NUM>,
      G: OnFinalizationStalled<NUM>, H: OnFinalizationStalled<NUM>,
      I: OnFinalizationStalled<NUM>, J: OnFinalizationStalled<NUM>,
      K: OnFinalizationStalled<NUM>, L: OnFinalizationStalled<NUM>,
      M: OnFinalizationStalled<NUM>, N: OnFinalizationStalled<NUM>,
      O: OnFinalizationStalled<NUM>, P: OnFinalizationStalled<NUM>,
      Q: OnFinalizationStalled<NUM>, R: OnFinalizationStalled<NUM>,
      S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM> for
 (D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        D::on_stalled(further_wait.clone());
        E::on_stalled(further_wait.clone());
        F::on_stalled(further_wait.clone());
        G::on_stalled(further_wait.clone());
        H::on_stalled(further_wait.clone());
        I::on_stalled(further_wait.clone());
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, E: OnFinalizationStalled<NUM>,
      F: OnFinalizationStalled<NUM>, G: OnFinalizationStalled<NUM>,
      H: OnFinalizationStalled<NUM>, I: OnFinalizationStalled<NUM>,
      J: OnFinalizationStalled<NUM>, K: OnFinalizationStalled<NUM>,
      L: OnFinalizationStalled<NUM>, M: OnFinalizationStalled<NUM>,
      N: OnFinalizationStalled<NUM>, O: OnFinalizationStalled<NUM>,
      P: OnFinalizationStalled<NUM>, Q: OnFinalizationStalled<NUM>,
      R: OnFinalizationStalled<NUM>, S: OnFinalizationStalled<NUM>>
 OnFinalizationStalled<NUM> for (E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)
 {
    fn on_stalled(further_wait: NUM) {
        E::on_stalled(further_wait.clone());
        F::on_stalled(further_wait.clone());
        G::on_stalled(further_wait.clone());
        H::on_stalled(further_wait.clone());
        I::on_stalled(further_wait.clone());
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, F: OnFinalizationStalled<NUM>,
      G: OnFinalizationStalled<NUM>, H: OnFinalizationStalled<NUM>,
      I: OnFinalizationStalled<NUM>, J: OnFinalizationStalled<NUM>,
      K: OnFinalizationStalled<NUM>, L: OnFinalizationStalled<NUM>,
      M: OnFinalizationStalled<NUM>, N: OnFinalizationStalled<NUM>,
      O: OnFinalizationStalled<NUM>, P: OnFinalizationStalled<NUM>,
      Q: OnFinalizationStalled<NUM>, R: OnFinalizationStalled<NUM>,
      S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM> for
 (F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        F::on_stalled(further_wait.clone());
        G::on_stalled(further_wait.clone());
        H::on_stalled(further_wait.clone());
        I::on_stalled(further_wait.clone());
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, G: OnFinalizationStalled<NUM>,
      H: OnFinalizationStalled<NUM>, I: OnFinalizationStalled<NUM>,
      J: OnFinalizationStalled<NUM>, K: OnFinalizationStalled<NUM>,
      L: OnFinalizationStalled<NUM>, M: OnFinalizationStalled<NUM>,
      N: OnFinalizationStalled<NUM>, O: OnFinalizationStalled<NUM>,
      P: OnFinalizationStalled<NUM>, Q: OnFinalizationStalled<NUM>,
      R: OnFinalizationStalled<NUM>, S: OnFinalizationStalled<NUM>>
 OnFinalizationStalled<NUM> for (G, H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        G::on_stalled(further_wait.clone());
        H::on_stalled(further_wait.clone());
        I::on_stalled(further_wait.clone());
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, H: OnFinalizationStalled<NUM>,
      I: OnFinalizationStalled<NUM>, J: OnFinalizationStalled<NUM>,
      K: OnFinalizationStalled<NUM>, L: OnFinalizationStalled<NUM>,
      M: OnFinalizationStalled<NUM>, N: OnFinalizationStalled<NUM>,
      O: OnFinalizationStalled<NUM>, P: OnFinalizationStalled<NUM>,
      Q: OnFinalizationStalled<NUM>, R: OnFinalizationStalled<NUM>,
      S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM> for
 (H, I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        H::on_stalled(further_wait.clone());
        I::on_stalled(further_wait.clone());
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, I: OnFinalizationStalled<NUM>,
      J: OnFinalizationStalled<NUM>, K: OnFinalizationStalled<NUM>,
      L: OnFinalizationStalled<NUM>, M: OnFinalizationStalled<NUM>,
      N: OnFinalizationStalled<NUM>, O: OnFinalizationStalled<NUM>,
      P: OnFinalizationStalled<NUM>, Q: OnFinalizationStalled<NUM>,
      R: OnFinalizationStalled<NUM>, S: OnFinalizationStalled<NUM>>
 OnFinalizationStalled<NUM> for (I, J, K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        I::on_stalled(further_wait.clone());
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, J: OnFinalizationStalled<NUM>,
      K: OnFinalizationStalled<NUM>, L: OnFinalizationStalled<NUM>,
      M: OnFinalizationStalled<NUM>, N: OnFinalizationStalled<NUM>,
      O: OnFinalizationStalled<NUM>, P: OnFinalizationStalled<NUM>,
      Q: OnFinalizationStalled<NUM>, R: OnFinalizationStalled<NUM>,
      S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM> for
 (J, K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        J::on_stalled(further_wait.clone());
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, K: OnFinalizationStalled<NUM>,
      L: OnFinalizationStalled<NUM>, M: OnFinalizationStalled<NUM>,
      N: OnFinalizationStalled<NUM>, O: OnFinalizationStalled<NUM>,
      P: OnFinalizationStalled<NUM>, Q: OnFinalizationStalled<NUM>,
      R: OnFinalizationStalled<NUM>, S: OnFinalizationStalled<NUM>>
 OnFinalizationStalled<NUM> for (K, L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        K::on_stalled(further_wait.clone());
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, L: OnFinalizationStalled<NUM>,
      M: OnFinalizationStalled<NUM>, N: OnFinalizationStalled<NUM>,
      O: OnFinalizationStalled<NUM>, P: OnFinalizationStalled<NUM>,
      Q: OnFinalizationStalled<NUM>, R: OnFinalizationStalled<NUM>,
      S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM> for
 (L, M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        L::on_stalled(further_wait.clone());
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, M: OnFinalizationStalled<NUM>,
      N: OnFinalizationStalled<NUM>, O: OnFinalizationStalled<NUM>,
      P: OnFinalizationStalled<NUM>, Q: OnFinalizationStalled<NUM>,
      R: OnFinalizationStalled<NUM>, S: OnFinalizationStalled<NUM>>
 OnFinalizationStalled<NUM> for (M, N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        M::on_stalled(further_wait.clone());
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, N: OnFinalizationStalled<NUM>,
      O: OnFinalizationStalled<NUM>, P: OnFinalizationStalled<NUM>,
      Q: OnFinalizationStalled<NUM>, R: OnFinalizationStalled<NUM>,
      S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM> for
 (N, O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        N::on_stalled(further_wait.clone());
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, O: OnFinalizationStalled<NUM>,
      P: OnFinalizationStalled<NUM>, Q: OnFinalizationStalled<NUM>,
      R: OnFinalizationStalled<NUM>, S: OnFinalizationStalled<NUM>>
 OnFinalizationStalled<NUM> for (O, P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        O::on_stalled(further_wait.clone());
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, P: OnFinalizationStalled<NUM>,
      Q: OnFinalizationStalled<NUM>, R: OnFinalizationStalled<NUM>,
      S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM> for
 (P, Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        P::on_stalled(further_wait.clone());
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, Q: OnFinalizationStalled<NUM>,
      R: OnFinalizationStalled<NUM>, S: OnFinalizationStalled<NUM>>
 OnFinalizationStalled<NUM> for (Q, R, S) {
    fn on_stalled(further_wait: NUM) {
        Q::on_stalled(further_wait.clone());
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, R: OnFinalizationStalled<NUM>,
      S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM> for (R, S) {
    fn on_stalled(further_wait: NUM) {
        R::on_stalled(further_wait.clone());
        S::on_stalled(further_wait.clone());
    }
}
impl <NUM: Clone, S: OnFinalizationStalled<NUM>> OnFinalizationStalled<NUM>
 for (S,) {
    fn on_stalled(further_wait: NUM) { S::on_stalled(further_wait.clone()); }
}
impl <N> OnFinalizationStalled<N> for () {
    fn on_stalled(_: N) { }
}
impl <T: Trait> ProvideInherent for Module<T> {
    type
    Call
    =
    Call<T>;
    type
    Error
    =
    MakeFatalError<()>;
    const
    INHERENT_IDENTIFIER:
    InherentIdentifier
    =
    INHERENT_IDENTIFIER;
    fn create_inherent(data: &InherentData) -> Option<Self::Call> {
        let final_num =
            data.finalized_number().expect("Gets and decodes final number inherent data");
        Self::recent_hints().last().and_then(|last|
                                                 if last == &final_num {
                                                     None
                                                 } else {
                                                     Some(Call::final_hint(final_num))
                                                 })
    }
    fn check_inherent(_call: &Self::Call, _data: &InherentData)
     -> result::Result<(), Self::Error> {
        Ok(())
    }
}
