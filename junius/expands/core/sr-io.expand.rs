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

//! This is part of the Substrate runtime.

#![warn(missing_docs)]


#![doc =
       "Substrate runtime standard library as compiled when linked with Rust's standard library."]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

use hash_db::Hasher;
use rstd::vec::Vec;

#[doc(hidden)]
pub use codec;

pub use primitives::Blake2Hasher;

/// Error verifying ECDSA signature
pub enum EcdsaVerifyError {

    /// Incorrect value of R or S
    BadRS,

    /// Incorrect value of V
    BadV,

    /// Invalid signature
    BadSignature,
}

/// Trait for things which can be printed.
pub trait Printable {
    /// Print the object.
    fn print(self);
}

/// Converts a public trait definition into a private trait and set of public functions
/// that assume the trait is implemented for `()` for ease of calling.
macro_rules! export_api((
                        $ ( # [ $ trait_attr : meta ] ) * pub ( crate ) trait
                        $ trait_name : ident {
                        $ (
                        $ ( # [ $ attr : meta ] ) * fn $ name : ident $ (
                        < $ ( $ g_name : ident $ ( : $ g_ty : path ) ? ) , + >
                        ) ? ( $ ( $ arg : ident : $ arg_ty : ty ) , * ) $ (
                        -> $ ret : ty ) ? $ (
                        where $ ( $ w_name : path : $ w_ty : path ) , + ) ? ;
                        ) * } ) => {
                        $ ( # [ $ trait_attr ] ) * pub ( crate ) trait $
                        trait_name {
                        $ (
                        $ ( # [ $ attr ] ) * fn $ name $ (
                        < $ ( $ g_name $ ( : $ g_ty ) ? ) , + > ) ? (
                        $ ( $ arg : $ arg_ty ) , * ) $ ( -> $ ret ) ? $ (
                        where $ ( $ w_name : $ w_ty ) , + ) ? ; ) * } $ (
                        $ ( # [ $ attr ] ) * pub fn $ name $ (
                        < $ ( $ g_name $ ( : $ g_ty ) ? ) , + > ) ? (
                        $ ( $ arg : $ arg_ty ) , * ) $ ( -> $ ret ) ? $ (
                        where $ ( $ w_name : $ w_ty ) , + ) ? {
                        # [ allow ( deprecated ) ] < (  ) > :: $ name $ (
                        :: < $ ( $ g_name ) , + > ) ? ( $ ( $ arg ) , * ) } )
                        * });
pub(crate) trait StorageApi {
    #[doc =

          r" Get `key` from storage and return a `Vec`, empty if there's a problem."]
    fn storage(key: &[u8])
    -> Option<Vec<u8>>;
    #[doc =
          r" Get `key` from child storage and return a `Vec`, empty if there's a problem."]
    fn child_storage(storage_key: &[u8], key: &[u8])
    -> Option<Vec<u8>>;
    #[doc =
          r" Get `key` from storage, placing the value into `value_out` (as much of it as possible) and return"]
    #[doc =
          r" the number of bytes that the entry in storage had beyond the offset or None if the storage entry"]
    #[doc =
          r" doesn't exist at all. Note that if the buffer is smaller than the storage entry length, the returned"]
    #[doc =
          r" number of bytes is not equal to the number of bytes written to the `value_out`."]
    fn read_storage(key: &[u8], value_out: &mut [u8], value_offset: usize)
    -> Option<usize>;
    #[doc =
          r" Get `key` from child storage, placing the value into `value_out` (as much of it as possible) and return"]
    #[doc =
          r" the number of bytes that the entry in storage had beyond the offset or None if the storage entry"]
    #[doc =
          r" doesn't exist at all. Note that if the buffer is smaller than the storage entry length, the returned"]
    #[doc =
          r" number of bytes is not equal to the number of bytes written to the `value_out`."]
    fn read_child_storage(storage_key: &[u8], key: &[u8],
                          value_out: &mut [u8], value_offset: usize)
    -> Option<usize>;
    #[doc =
          r" Set the storage of some particular key to Some value."]
    fn set_storage(key: &[u8], value: &[u8]);
    #[doc =
          r" Set the child storage of some particular key to Some value."]
    fn set_child_storage(storage_key: &[u8], key: &[u8], value: &[u8]);
    #[doc =
          r" Clear the storage of a key."]
    fn clear_storage(key: &[u8]);
    #[doc =
          r" Clear the storage of a key."]
    fn clear_child_storage(storage_key: &[u8], key: &[u8]);
    #[doc =
          r" Clear an entire child storage."]
    fn kill_child_storage(storage_key: &[u8]);
    #[doc =
          r" Check whether a given `key` exists in storage."]
    fn exists_storage(key: &[u8])
    -> bool;
    #[doc =
          r" Check whether a given `key` exists in storage."]
    fn exists_child_storage(storage_key: &[u8], key: &[u8])
    -> bool;
    #[doc =
          r" Clear the storage entries with a key that starts with the given prefix."]
    fn clear_prefix(prefix: &[u8]);
    #[doc =
          r#" "Commit" all existing operations and compute the resultant storage root."#]
    fn storage_root()
    -> [u8; 32];
    #[doc =
          r#" "Commit" all existing operations and compute the resultant child storage root."#]
    fn child_storage_root(storage_key: &[u8])
    -> Vec<u8>;
    #[doc =
          r#" "Commit" all existing operations and get the resultant storage change root."#]
    fn storage_changes_root(parent_hash: [u8; 32], parent_num: u64)
    -> Option<[u8; 32]>;
    #[doc =
          r" A trie root formed from the enumerated items."]
    #[doc =
          r" TODO [#2382] remove (just use `ordered_trie_root` (NOTE currently not implemented for without_std))"]
    fn enumerated_trie_root<H>(input: &[&[u8]])
    -> H::Out
    where
    H: Hasher,
    H: self::imp::HasherBounds,
    H::Out: Ord;
    #[doc =
          r" A trie root formed from the iterated items."]
    fn trie_root<H, I, A, B>(input: I)
    -> H::Out
    where
    I: IntoIterator<Item
    =
    (A, B)>,
    A: AsRef<[u8]>,
    A: Ord,
    B: AsRef<[u8]>,
    H: Hasher,
    H: self::imp::HasherBounds,
    H::Out: Ord;
    #[doc =
          r" A trie root formed from the enumerated items."]
    fn ordered_trie_root<H, I, A>(input: I)
    -> H::Out
    where
    I: IntoIterator<Item
    =
    A>,
    A: AsRef<[u8]>,
    H: Hasher,
    H: self::imp::HasherBounds,
    H::Out: Ord;
}
#[doc =
      r" Get `key` from storage and return a `Vec`, empty if there's a problem."]
pub fn storage(key: &[u8]) -> Option<Vec<u8>> {

    #[allow(deprecated)]
    <()>::storage(key)
}
#[doc =
      r" Get `key` from child storage and return a `Vec`, empty if there's a problem."]
pub fn child_storage(storage_key: &[u8], key: &[u8]) -> Option<Vec<u8>> {

    #[allow(deprecated)]
    <()>::child_storage(storage_key, key)
}
#[doc =
      r" Get `key` from storage, placing the value into `value_out` (as much of it as possible) and return"]
#[doc =
      r" the number of bytes that the entry in storage had beyond the offset or None if the storage entry"]
#[doc =
      r" doesn't exist at all. Note that if the buffer is smaller than the storage entry length, the returned"]
#[doc =
      r" number of bytes is not equal to the number of bytes written to the `value_out`."]
pub fn read_storage(key: &[u8], value_out: &mut [u8], value_offset: usize)
 -> Option<usize> {

    #[allow(deprecated)]
    <()>::read_storage(key, value_out, value_offset)
}
#[doc =
      r" Get `key` from child storage, placing the value into `value_out` (as much of it as possible) and return"]
#[doc =
      r" the number of bytes that the entry in storage had beyond the offset or None if the storage entry"]
#[doc =
      r" doesn't exist at all. Note that if the buffer is smaller than the storage entry length, the returned"]
#[doc =
      r" number of bytes is not equal to the number of bytes written to the `value_out`."]
pub fn read_child_storage(storage_key: &[u8], key: &[u8],
                          value_out: &mut [u8], value_offset: usize)
 -> Option<usize> {

    #[allow(deprecated)]
    <()>::read_child_storage(storage_key, key, value_out, value_offset)
}
#[doc = r" Set the storage of some particular key to Some value."]
pub fn set_storage(key: &[u8], value: &[u8]) {

    #[allow(deprecated)]
    <()>::set_storage(key, value)
}
#[doc = r" Set the child storage of some particular key to Some value."]
pub fn set_child_storage(storage_key: &[u8], key: &[u8], value: &[u8]) {

    #[allow(deprecated)]
    <()>::set_child_storage(storage_key, key, value)
}
#[doc = r" Clear the storage of a key."]
pub fn clear_storage(key: &[u8]) {

    #[allow(deprecated)]
    <()>::clear_storage(key)
}
#[doc = r" Clear the storage of a key."]
pub fn clear_child_storage(storage_key: &[u8], key: &[u8]) {

    #[allow(deprecated)]
    <()>::clear_child_storage(storage_key, key)
}
#[doc = r" Clear an entire child storage."]
pub fn kill_child_storage(storage_key: &[u8]) {

    #[allow(deprecated)]
    <()>::kill_child_storage(storage_key)
}
#[doc = r" Check whether a given `key` exists in storage."]
pub fn exists_storage(key: &[u8]) -> bool {

    #[allow(deprecated)]
    <()>::exists_storage(key)
}
#[doc = r" Check whether a given `key` exists in storage."]
pub fn exists_child_storage(storage_key: &[u8], key: &[u8]) -> bool {

    #[allow(deprecated)]
    <()>::exists_child_storage(storage_key, key)
}
#[doc =
      r" Clear the storage entries with a key that starts with the given prefix."]
pub fn clear_prefix(prefix: &[u8]) {

    #[allow(deprecated)]
    <()>::clear_prefix(prefix)
}
#[doc =
      r#" "Commit" all existing operations and compute the resultant storage root."#]
pub fn storage_root() -> [u8; 32] {

    #[allow(deprecated)]
    <()>::storage_root()
}
#[doc =
      r#" "Commit" all existing operations and compute the resultant child storage root."#]
pub fn child_storage_root(storage_key: &[u8]) -> Vec<u8> {

    #[allow(deprecated)]
    <()>::child_storage_root(storage_key)
}
#[doc =
      r#" "Commit" all existing operations and get the resultant storage change root."#]
pub fn storage_changes_root(parent_hash: [u8; 32], parent_num: u64)
 -> Option<[u8; 32]> {

    #[allow(deprecated)]
    <()>::storage_changes_root(parent_hash, parent_num)
}
#[doc = r" A trie root formed from the enumerated items."]
#[doc =
      r" TODO [#2382] remove (just use `ordered_trie_root` (NOTE currently not implemented for without_std))"]
pub fn enumerated_trie_root<H>(input: &[&[u8]]) -> H::Out where H: Hasher,
 H: self::imp::HasherBounds, H::Out: Ord {

    #[allow(deprecated)]
    <()>::enumerated_trie_root::<H>(input)
}
#[doc = r" A trie root formed from the iterated items."]
pub fn trie_root<H, I, A, B>(input: I) -> H::Out where I: IntoIterator<Item =
 (A, B)>, A: AsRef<[u8]>, A: Ord, B: AsRef<[u8]>, H: Hasher,
 H: self::imp::HasherBounds, H::Out: Ord {

    #[allow(deprecated)]
    <()>::trie_root::<H, I, A, B>(input)
}
#[doc = r" A trie root formed from the enumerated items."]
pub fn ordered_trie_root<H, I, A>(input: I) -> H::Out where
 I: IntoIterator<Item = A>, A: AsRef<[u8]>, H: Hasher,
 H: self::imp::HasherBounds, H::Out: Ord {

    #[allow(deprecated)]
    <()>::ordered_trie_root::<H, I, A>(input)
}
pub(crate) trait OtherApi {
    #[doc =
          r" The current relay chain identifier."]
    fn chain_id()
    -> u64;
    #[doc =
          r" Print a printable value."]
    fn print<T>(value: T)
    where
    T: Printable,
    T: Sized;
}
#[doc = r" The current relay chain identifier."]
pub fn chain_id() -> u64 {

    #[allow(deprecated)]
    <()>::chain_id()
}
#[doc = r" Print a printable value."]
pub fn print<T>(value: T) where T: Printable, T: Sized {

    #[allow(deprecated)]
    <()>::print::<T>(value)
}
pub(crate) trait CryptoApi {
    #[doc =
          r" Verify a ed25519 signature."]
    fn ed25519_verify<P: AsRef<[u8]>>(sig: &[u8; 64], msg: &[u8], pubkey: P)
    -> bool;
    #[doc =
          r" Verify an sr25519 signature."]
    fn sr25519_verify<P: AsRef<[u8]>>(sig: &[u8; 64], msg: &[u8], pubkey: P)
    -> bool;
    #[doc =
          r" Verify and recover a SECP256k1 ECDSA signature."]
    #[doc =
          r" - `sig` is passed in RSV format. V should be either 0/1 or 27/28."]
    #[doc =
          r" - returns `Err` if the signature is bad, otherwise the 64-byte pubkey (doesn't include the 0x04 prefix)."]
    fn secp256k1_ecdsa_recover(sig: &[u8; 65], msg: &[u8; 32])
    -> Result<[u8; 64], EcdsaVerifyError>;
}
#[doc = r" Verify a ed25519 signature."]
pub fn ed25519_verify<P: AsRef<[u8]>>(sig: &[u8; 64], msg: &[u8], pubkey: P)
 -> bool {

    #[allow(deprecated)]
    <()>::ed25519_verify::<P>(sig, msg, pubkey)
}
#[doc = r" Verify an sr25519 signature."]
pub fn sr25519_verify<P: AsRef<[u8]>>(sig: &[u8; 64], msg: &[u8], pubkey: P)
 -> bool {

    #[allow(deprecated)]
    <()>::sr25519_verify::<P>(sig, msg, pubkey)
}
#[doc = r" Verify and recover a SECP256k1 ECDSA signature."]
#[doc = r" - `sig` is passed in RSV format. V should be either 0/1 or 27/28."]
#[doc =
      r" - returns `Err` if the signature is bad, otherwise the 64-byte pubkey (doesn't include the 0x04 prefix)."]
pub fn secp256k1_ecdsa_recover(sig: &[u8; 65], msg: &[u8; 32])
 -> Result<[u8; 64], EcdsaVerifyError> {

    #[allow(deprecated)]
    <()>::secp256k1_ecdsa_recover(sig, msg)
}
pub(crate) trait HashingApi {
    #[doc =
          r" Conduct a 256-bit Keccak hash."]
    fn keccak_256(data: &[u8])
    -> [u8; 32];
    #[doc =
          r" Conduct a 128-bit Blake2 hash."]
    fn blake2_128(data: &[u8])
    -> [u8; 16];
    #[doc =
          r" Conduct a 256-bit Blake2 hash."]
    fn blake2_256(data: &[u8])
    -> [u8; 32];
    #[doc =
          r" Conduct four XX hashes to give a 256-bit result."]
    fn twox_256(data: &[u8])
    -> [u8; 32];
    #[doc =
          r" Conduct two XX hashes to give a 128-bit result."]
    fn twox_128(data: &[u8])
    -> [u8; 16];
    #[doc =
          r" Conduct two XX hashes to give a 64-bit result."]
    fn twox_64(data: &[u8])
    -> [u8; 8];
}
#[doc = r" Conduct a 256-bit Keccak hash."]
pub fn keccak_256(data: &[u8]) -> [u8; 32] {

    #[allow(deprecated)]
    <()>::keccak_256(data)
}
#[doc = r" Conduct a 128-bit Blake2 hash."]
pub fn blake2_128(data: &[u8]) -> [u8; 16] {

    #[allow(deprecated)]
    <()>::blake2_128(data)
}
#[doc = r" Conduct a 256-bit Blake2 hash."]
pub fn blake2_256(data: &[u8]) -> [u8; 32] {

    #[allow(deprecated)]
    <()>::blake2_256(data)
}
#[doc = r" Conduct four XX hashes to give a 256-bit result."]
pub fn twox_256(data: &[u8]) -> [u8; 32] {

    #[allow(deprecated)]
    <()>::twox_256(data)
}
#[doc = r" Conduct two XX hashes to give a 128-bit result."]
pub fn twox_128(data: &[u8]) -> [u8; 16] {

    #[allow(deprecated)]
    <()>::twox_128(data)
}
#[doc = r" Conduct two XX hashes to give a 64-bit result."]
pub fn twox_64(data: &[u8]) -> [u8; 8] {

    #[allow(deprecated)]
    <()>::twox_64(data)
}
pub(crate) trait OffchainApi {
    #[doc =
          r" Submit extrinsic from the runtime."]
    #[doc = r""]
    #[doc = r" Depending on the kind of extrinsic it will either be:"]
    #[doc =
          r" 1. scheduled to be included in the next produced block (inherent)"]
    #[doc = r" 2. added to the pool and propagated (transaction)"]
    fn submit_extrinsic<T: codec::Encode>(data: &T);
}
#[doc = r" Submit extrinsic from the runtime."]
#[doc = r""]
#[doc = r" Depending on the kind of extrinsic it will either be:"]
#[doc = r" 1. scheduled to be included in the next produced block (inherent)"]
#[doc = r" 2. added to the pool and propagated (transaction)"]
pub fn submit_extrinsic<T: codec::Encode>(data: &T) {

    #[allow(deprecated)]
    <()>::submit_extrinsic::<T>(data)
}

/// API trait that should cover all other APIs.
///
/// Implement this to make sure you implement all APIs.
trait Api: StorageApi + OtherApi + CryptoApi + HashingApi + OffchainApi { }

mod imp {
    use super::*;



    use primitives::{blake2_128, blake2_256, twox_128, twox_256, twox_64,
                     ed25519, Blake2Hasher, sr25519, Pair};
    pub use substrate_state_machine::{Externalities, BasicExternalities,
                                      TestExternalities, ChildStorageKey};
    use environmental::environmental;
    use primitives::{hexdisplay::HexDisplay, H256};
    #[cfg(feature = "std")]
    use std::collections::HashMap;
    #[allow(non_camel_case_types, dead_code)]
    struct ext {
        __private_field: (),
    }
    const GLOBAL:
          ::std::thread::LocalKey<::environmental::imp::RefCell<Option<*mut (Externalities<Blake2Hasher> +
                                                                             'static)>>>
          =
        {
            #[inline]
            fn __init()
             ->
                 ::environmental::imp::RefCell<Option<*mut (Externalities<Blake2Hasher> +
                                                            'static)>> {
                ::environmental::imp::RefCell::new(None)
            }
            unsafe fn __getit()
             ->
                 ::std::option::Option<&'static ::std::cell::UnsafeCell<::std::option::Option<::environmental::imp::RefCell<Option<*mut (Externalities<Blake2Hasher> +
                                                                                                                                         'static)>>>>> {
                #[thread_local]
                #[cfg(all(target_thread_local,
                          not(all(target_arch = "wasm32",
                                  not(target_feature = "atomics")))))]
                static __KEY:
                       ::std::thread::__FastLocalKeyInner<::environmental::imp::RefCell<Option<*mut (Externalities<Blake2Hasher> +
                                                                                                     'static)>>>
                       =
                    ::std::thread::__FastLocalKeyInner::new();
                __KEY.get()
            }
            unsafe { ::std::thread::LocalKey::new(__getit, __init) }
        };
    impl ext {
        #[allow(unused_imports)]
        pub fn using<R, F: FnOnce() ->
                     R>(protected: &mut Externalities<Blake2Hasher>, f: F)
         -> R {
            let lifetime_extended =
                unsafe {
                    ::environmental::imp::transmute::<&mut Externalities<Blake2Hasher>,
                                                      &mut (Externalities<Blake2Hasher> +
                                                            'static)>(protected)
                };
            ::environmental::using(&GLOBAL, lifetime_extended, f)
        }
        pub fn with<R,
                    F: for<'a> FnOnce(&'a mut (Externalities<Blake2Hasher> +
                                               'a)) -> R>(f: F) -> Option<R> {
            ::environmental::with(&GLOBAL, |x| f(x))
        }
    }
    /// Additional bounds for `Hasher` trait for with_std.
    pub trait HasherBounds { }
    impl <T: Hasher> HasherBounds for T { }
    /// Returns a `ChildStorageKey` if the given `storage_key` slice is a valid storage
    /// key or panics otherwise.
    ///
    /// Panicking here is aligned with what the `without_std` environment would do
    /// in the case of an invalid child storage key.
    fn child_storage_key_or_panic(storage_key: &[u8])
     -> ChildStorageKey<Blake2Hasher> {
        match ChildStorageKey::from_slice(storage_key) {
            Some(storage_key) => storage_key,
            None => {
                ::std::rt::begin_panic("child storage key is invalid",
                                       &("core/sr-io/src/../with_std.rs",
                                         50u32, 11u32))
            }
        }
    }
    impl StorageApi for () {
        fn storage(key: &[u8]) -> Option<Vec<u8>> {
            ext::with(|ext|
                          ext.storage(key).map(|s|
                                                   s.to_vec())).expect("storage cannot be called outside of an Externalities-provided environment.")
        }
        fn read_storage(key: &[u8], value_out: &mut [u8], value_offset: usize)
         -> Option<usize> {
            ext::with(|ext|
                          ext.storage(key).map(|value|
                                                   {
                                                       let value =
                                                           &value[value_offset..];
                                                       let written =
                                                           std::cmp::min(value.len(),
                                                                         value_out.len());
                                                       value_out[..written].copy_from_slice(&value[..written]);
                                                       value.len()
                                                   })).expect("read_storage cannot be called outside of an Externalities-provided environment.")
        }
        fn child_storage(storage_key: &[u8], key: &[u8]) -> Option<Vec<u8>> {
            ext::with(|ext|
                          {
                              let storage_key =
                                  child_storage_key_or_panic(storage_key);
                              ext.child_storage(storage_key,
                                                key).map(|s| s.to_vec())
                          }).expect("storage cannot be called outside of an Externalities-provided environment.")
        }
        fn set_storage(key: &[u8], value: &[u8]) {
            ext::with(|ext| ext.set_storage(key.to_vec(), value.to_vec()));
        }
        fn read_child_storage(storage_key: &[u8], key: &[u8],
                              value_out: &mut [u8], value_offset: usize)
         -> Option<usize> {
            ext::with(|ext|
                          {
                              let storage_key =
                                  child_storage_key_or_panic(storage_key);
                              ext.child_storage(storage_key,
                                                key).map(|value|
                                                             {
                                                                 let value =
                                                                     &value[value_offset..];
                                                                 let written =
                                                                     std::cmp::min(value.len(),
                                                                                   value_out.len());
                                                                 value_out[..written].copy_from_slice(&value[..written]);
                                                                 value.len()
                                                             })
                          }).expect("read_child_storage cannot be called outside of an Externalities-provided environment.")
        }
        fn set_child_storage(storage_key: &[u8], key: &[u8], value: &[u8]) {
            ext::with(|ext|
                          {
                              let storage_key =
                                  child_storage_key_or_panic(storage_key);
                              ext.set_child_storage(storage_key, key.to_vec(),
                                                    value.to_vec())
                          });
        }
        fn clear_storage(key: &[u8]) {
            ext::with(|ext| ext.clear_storage(key));
        }
        fn clear_child_storage(storage_key: &[u8], key: &[u8]) {
            ext::with(|ext|
                          {
                              let storage_key =
                                  child_storage_key_or_panic(storage_key);
                              ext.clear_child_storage(storage_key, key)
                          });
        }
        fn kill_child_storage(storage_key: &[u8]) {
            ext::with(|ext|
                          {
                              let storage_key =
                                  child_storage_key_or_panic(storage_key);
                              ext.kill_child_storage(storage_key)
                          });
        }
        fn exists_storage(key: &[u8]) -> bool {
            ext::with(|ext| ext.exists_storage(key)).unwrap_or(false)
        }
        fn exists_child_storage(storage_key: &[u8], key: &[u8]) -> bool {
            ext::with(|ext|
                          {
                              let storage_key =
                                  child_storage_key_or_panic(storage_key);
                              ext.exists_child_storage(storage_key, key)
                          }).unwrap_or(false)
        }
        fn clear_prefix(prefix: &[u8]) {
            ext::with(|ext| ext.clear_prefix(prefix));
        }
        fn storage_root() -> [u8; 32] {
            ext::with(|ext| ext.storage_root()).unwrap_or(H256::zero()).into()
        }
        fn child_storage_root(storage_key: &[u8]) -> Vec<u8> {
            ext::with(|ext|
                          {
                              let storage_key =
                                  child_storage_key_or_panic(storage_key);
                              ext.child_storage_root(storage_key)
                          }).expect("child_storage_root cannot be called outside of an Externalities-provided environment.")
        }
        fn storage_changes_root(parent_hash: [u8; 32], parent_num: u64)
         -> Option<[u8; 32]> {
            ext::with(|ext|
                          ext.storage_changes_root(parent_hash.into(),
                                                   parent_num).map(Into::into)).unwrap_or(None)
        }
        fn enumerated_trie_root<H>(input: &[&[u8]]) -> H::Out where H: Hasher,
         H::Out: Ord {
            trie::ordered_trie_root::<H, _, _>(input.iter())
        }
        fn trie_root<H, I, A, B>(input: I) -> H::Out where
         I: IntoIterator<Item = (A, B)>, A: AsRef<[u8]> + Ord, B: AsRef<[u8]>,
         H: Hasher, H::Out: Ord {
            trie::trie_root::<H, _, _, _>(input)
        }
        fn ordered_trie_root<H, I, A>(input: I) -> H::Out where
         I: IntoIterator<Item = A>, A: AsRef<[u8]>, H: Hasher, H::Out: Ord {
            trie::ordered_trie_root::<H, _, _>(input)
        }
    }
    impl OtherApi for () {
        fn chain_id() -> u64 { ext::with(|ext| ext.chain_id()).unwrap_or(0) }
        fn print<T: Printable + Sized>(value: T) { value.print() }
    }
    impl CryptoApi for () {
        fn ed25519_verify<P: AsRef<[u8]>>(sig: &[u8; 64], msg: &[u8],
                                          pubkey: P) -> bool {
            ed25519::Pair::verify_weak(sig, msg, pubkey)
        }
        fn sr25519_verify<P: AsRef<[u8]>>(sig: &[u8; 64], msg: &[u8],
                                          pubkey: P) -> bool {
            sr25519::Pair::verify_weak(sig, msg, pubkey)
        }
        fn secp256k1_ecdsa_recover(sig: &[u8; 65], msg: &[u8; 32])
         -> Result<[u8; 64], EcdsaVerifyError> {
            let rs =
                secp256k1::Signature::parse_slice(&sig[0..64]).map_err(|_|
                                                                           EcdsaVerifyError::BadRS)?;
            let v =
                secp256k1::RecoveryId::parse(if sig[64] > 26 {
                                                 sig[64] - 27
                                             } else { sig[64] } as
                                                 u8).map_err(|_|
                                                                 EcdsaVerifyError::BadV)?;
            let pubkey =
                secp256k1::recover(&secp256k1::Message::parse(msg), &rs,
                                   &v).map_err(|_|
                                                   EcdsaVerifyError::BadSignature)?;
            let mut res = [0u8; 64];
            res.copy_from_slice(&pubkey.serialize()[1..65]);
            Ok(res)
        }
    }
    impl HashingApi for () {
        fn keccak_256(data: &[u8]) -> [u8; 32] {
            tiny_keccak::keccak256(data)
        }
        fn blake2_128(data: &[u8]) -> [u8; 16] { blake2_128(data) }
        fn blake2_256(data: &[u8]) -> [u8; 32] { blake2_256(data) }
        fn twox_256(data: &[u8]) -> [u8; 32] { twox_256(data) }
        fn twox_128(data: &[u8]) -> [u8; 16] { twox_128(data) }
        fn twox_64(data: &[u8]) -> [u8; 8] { twox_64(data) }
    }
    impl OffchainApi for () {
        fn submit_extrinsic<T: codec::Encode>(data: &T) {
            ext::with(|ext|
                          ext.submit_extrinsic(codec::Encode::encode(data)).expect("submit_extrinsic can be called only in offchain worker context")).expect("submit_extrinsic cannot be called outside of an Externalities-provided environment.")
        }
    }
    impl Api for () { }
    /// Execute the given closure with global function available whose functionality routes into the
    /// externalities `ext`. Forwards the value that the closure returns.
    pub fn with_externalities<R, F: FnOnce() ->
                              R>(ext: &mut Externalities<Blake2Hasher>, f: F)
     -> R {
        ext::using(ext, f)
    }
    /// A set of key value pairs for storage.
    pub type StorageOverlay = HashMap<Vec<u8>, Vec<u8>>;
    /// A set of key value pairs for children storage;
    pub type ChildrenStorageOverlay = HashMap<Vec<u8>, StorageOverlay>;
    /// Execute the given closure with global functions available whose functionality routes into
    /// externalities that draw from and populate `storage`. Forwards the value that the closure returns.
    pub fn with_storage<R, F: FnOnce() ->
                        R>(storage: &mut StorageOverlay, f: F) -> R {
        let mut alt_storage = Default::default();
        rstd::mem::swap(&mut alt_storage, storage);
        let mut ext: BasicExternalities = alt_storage.into();
        let r = ext::using(&mut ext, f);
        *storage = ext.into();
        r
    }
    impl <'a> Printable for &'a [u8] {
        fn print(self) {
            {
                ::std::io::_print(::std::fmt::Arguments::new_v1(&["Runtime: ",
                                                                  "\n"],
                                                                &match (&HexDisplay::from(&self),)
                                                                     {
                                                                     (arg0,)
                                                                     =>
                                                                     [::std::fmt::ArgumentV1::new(arg0,
                                                                                                  ::std::fmt::Display::fmt)],
                                                                 }));
            };
        }
    }
    impl <'a> Printable for &'a str {
        fn print(self) {
            {
                ::std::io::_print(::std::fmt::Arguments::new_v1(&["Runtime: ",
                                                                  "\n"],
                                                                &match (&self,)
                                                                     {
                                                                     (arg0,)
                                                                     =>
                                                                     [::std::fmt::ArgumentV1::new(arg0,
                                                                                                  ::std::fmt::Display::fmt)],
                                                                 }));
            };
        }
    }
    impl Printable for u64 {
        fn print(self) {
            {
                ::std::io::_print(::std::fmt::Arguments::new_v1(&["Runtime: ",
                                                                  "\n"],
                                                                &match (&self,)
                                                                     {
                                                                     (arg0,)
                                                                     =>
                                                                     [::std::fmt::ArgumentV1::new(arg0,
                                                                                                  ::std::fmt::Display::fmt)],
                                                                 }));
            };
        }
    }
}
#[cfg(feature = "std")]
pub use self::imp::{StorageOverlay, ChildrenStorageOverlay, with_storage,
                    with_externalities, TestExternalities};
