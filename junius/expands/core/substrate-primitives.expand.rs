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

//! Shareable Substrate types.

#![warn(missing_docs)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


/// Initialize a key-value collection from array.
///
/// Creates a vector of given pairs and calls `collect` on the iterator from it.
/// Can be used to create a `HashMap`.
#[macro_export]
macro_rules! map(( $ ( $ name : expr => $ value : expr ) , * ) => (
                 vec ! [ $ ( ( $ name , $ value ) ) , * ] . into_iter (  ) .
                 collect (  ) ));

use rstd::prelude::*;
use rstd::ops::Deref;
use parity_codec::{Encode, Decode};
#[cfg(feature = "std")]
use std::borrow::Cow;
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "std")]
pub use impl_serde::serialize as bytes;

#[cfg(feature = "std")]
pub mod hashing {





    // Switch back to Blake after PoC-3 is out
    // pub use self::hasher::blake::BlakeHasher;

















    // The enum is not constructable, so this function should never be callable!

    //! Hashing functions.
    use blake2_rfc;
    use twox_hash;
    /// Do a Blake2 512-bit hash and place result in `dest`.
    pub fn blake2_512_into(data: &[u8], dest: &mut [u8; 64]) {
        dest.copy_from_slice(blake2_rfc::blake2b::blake2b(64, &[],
                                                          data).as_bytes());
    }
    /// Do a Blake2 512-bit hash and return result.
    pub fn blake2_512(data: &[u8]) -> [u8; 64] {
        let mut r = [0; 64];
        blake2_512_into(data, &mut r);
        r
    }
    /// Do a Blake2 256-bit hash and place result in `dest`.
    pub fn blake2_256_into(data: &[u8], dest: &mut [u8; 32]) {
        dest.copy_from_slice(blake2_rfc::blake2b::blake2b(32, &[],
                                                          data).as_bytes());
    }
    /// Do a Blake2 256-bit hash and return result.
    pub fn blake2_256(data: &[u8]) -> [u8; 32] {
        let mut r = [0; 32];
        blake2_256_into(data, &mut r);
        r
    }
    /// Do a Blake2 128-bit hash and place result in `dest`.
    pub fn blake2_128_into(data: &[u8], dest: &mut [u8; 16]) {
        dest.copy_from_slice(blake2_rfc::blake2b::blake2b(16, &[],
                                                          data).as_bytes());
    }
    /// Do a Blake2 128-bit hash and return result.
    pub fn blake2_128(data: &[u8]) -> [u8; 16] {
        let mut r = [0; 16];
        blake2_128_into(data, &mut r);
        r
    }
    /// Do a XX 64-bit hash and place result in `dest`.
    pub fn twox_64_into(data: &[u8], dest: &mut [u8; 8]) {
        use ::core::hash::Hasher;
        let mut h0 = twox_hash::XxHash::with_seed(0);
        h0.write(data);
        let r0 = h0.finish();
        use byteorder::{ByteOrder, LittleEndian};
        LittleEndian::write_u64(&mut dest[0..8], r0);
    }
    /// Do a XX 64-bit hash and return result.
    pub fn twox_64(data: &[u8]) -> [u8; 8] {
        let mut r: [u8; 8] = [0; 8];
        twox_64_into(data, &mut r);
        r
    }
    /// Do a XX 128-bit hash and place result in `dest`.
    pub fn twox_128_into(data: &[u8], dest: &mut [u8; 16]) {
        use ::core::hash::Hasher;
        let mut h0 = twox_hash::XxHash::with_seed(0);
        let mut h1 = twox_hash::XxHash::with_seed(1);
        h0.write(data);
        h1.write(data);
        let r0 = h0.finish();
        let r1 = h1.finish();
        use byteorder::{ByteOrder, LittleEndian};
        LittleEndian::write_u64(&mut dest[0..8], r0);
        LittleEndian::write_u64(&mut dest[8..16], r1);
    }
    /// Do a XX 128-bit hash and return result.
    pub fn twox_128(data: &[u8]) -> [u8; 16] {
        let mut r: [u8; 16] = [0; 16];
        twox_128_into(data, &mut r);
        r
    }
    /// Do a XX 256-bit hash and place result in `dest`.
    pub fn twox_256_into(data: &[u8], dest: &mut [u8; 32]) {
        use ::core::hash::Hasher;
        use byteorder::{ByteOrder, LittleEndian};
        let mut h0 = twox_hash::XxHash::with_seed(0);
        let mut h1 = twox_hash::XxHash::with_seed(1);
        let mut h2 = twox_hash::XxHash::with_seed(2);
        let mut h3 = twox_hash::XxHash::with_seed(3);
        h0.write(data);
        h1.write(data);
        h2.write(data);
        h3.write(data);
        let r0 = h0.finish();
        let r1 = h1.finish();
        let r2 = h2.finish();
        let r3 = h3.finish();
        LittleEndian::write_u64(&mut dest[0..8], r0);
        LittleEndian::write_u64(&mut dest[8..16], r1);
        LittleEndian::write_u64(&mut dest[16..24], r2);
        LittleEndian::write_u64(&mut dest[24..32], r3);
    }
    /// Do a XX 256-bit hash and return result.
    pub fn twox_256(data: &[u8]) -> [u8; 32] {
        let mut r: [u8; 32] = [0; 32];
        twox_256_into(data, &mut r);
        r
    }
}
#[cfg(feature = "std")]
pub use hashing::{blake2_128, blake2_256, twox_64, twox_128, twox_256};
#[cfg(feature = "std")]
pub mod hexdisplay {
    //! Wrapper type for byte collections that outputs hex.
    /// Simple wrapper to display hex representation of bytes.
    pub struct HexDisplay<'a>(&'a [u8]);
    impl <'a> HexDisplay<'a> {
        /// Create new instance that will display `d` as a hex string when displayed.
        pub fn from(d: &'a AsBytesRef) -> Self {
            HexDisplay(d.as_bytes_ref())
        }
    }
    impl <'a> ::core::fmt::Display for HexDisplay<'a> {
        fn fmt(&self, fmtr: &mut ::core::fmt::Formatter)
         -> Result<(), ::core::fmt::Error> {
            if self.0.len() < 1027 {
                for byte in self.0 {
                    fmtr.write_fmt(::std::fmt::Arguments::new_v1_formatted(&[""],
                                                                           &match (&byte,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::LowerHex::fmt)],
                                                                            },
                                                                           &[::std::fmt::rt::v1::Argument{position:
                                                                                                              ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                          format:
                                                                                                              ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                 ' ',
                                                                                                                                             align:
                                                                                                                                                 ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                             flags:
                                                                                                                                                 8u32,
                                                                                                                                             precision:
                                                                                                                                                 ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                             width:
                                                                                                                                                 ::std::fmt::rt::v1::Count::Is(2usize),},}]))?;
                }
            } else {
                for byte in &self.0[0..512] {
                    fmtr.write_fmt(::std::fmt::Arguments::new_v1_formatted(&[""],
                                                                           &match (&byte,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::LowerHex::fmt)],
                                                                            },
                                                                           &[::std::fmt::rt::v1::Argument{position:
                                                                                                              ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                          format:
                                                                                                              ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                 ' ',
                                                                                                                                             align:
                                                                                                                                                 ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                             flags:
                                                                                                                                                 8u32,
                                                                                                                                             precision:
                                                                                                                                                 ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                             width:
                                                                                                                                                 ::std::fmt::rt::v1::Count::Is(2usize),},}]))?;
                }
                fmtr.write_str("...")?;
                for byte in &self.0[self.0.len() - 512..] {
                    fmtr.write_fmt(::std::fmt::Arguments::new_v1_formatted(&[""],
                                                                           &match (&byte,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::LowerHex::fmt)],
                                                                            },
                                                                           &[::std::fmt::rt::v1::Argument{position:
                                                                                                              ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                          format:
                                                                                                              ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                 ' ',
                                                                                                                                             align:
                                                                                                                                                 ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                             flags:
                                                                                                                                                 8u32,
                                                                                                                                             precision:
                                                                                                                                                 ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                             width:
                                                                                                                                                 ::std::fmt::rt::v1::Count::Is(2usize),},}]))?;
                }
            }
            Ok(())
        }
    }
    /// Simple trait to transform various types to `&[u8]`
    pub trait AsBytesRef {
        /// Transform `self` into `&[u8]`.
        fn as_bytes_ref(&self)
        -> &[u8];
    }
    impl <'a> AsBytesRef for &'a [u8] {
        fn as_bytes_ref(&self) -> &[u8] { self }
    }
    impl AsBytesRef for [u8] {
        fn as_bytes_ref(&self) -> &[u8] { &self }
    }
    impl AsBytesRef for Vec<u8> {
        fn as_bytes_ref(&self) -> &[u8] { &self }
    }
    macro_rules! impl_non_endians(( $ ( $ t : ty ) , * ) => {
                                  $ (
                                  impl AsBytesRef for $ t {
                                  fn as_bytes_ref ( & self ) -> & [ u8 ] {
                                  & self [ .. ] } } ) * });
    impl AsBytesRef for [u8; 1] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 2] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 3] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 4] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 5] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 6] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 7] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 8] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 10] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 12] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 14] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 16] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 20] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 24] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 28] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 32] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 40] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 48] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 56] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 64] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 80] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 96] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 112] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    impl AsBytesRef for [u8; 128] {
        fn as_bytes_ref(&self) -> &[u8] { &self[..] }
    }
    /// Format into ASCII + # + hex, suitable for storage key preimages.
    pub fn ascii_format(asciish: &[u8]) -> String {
        let mut r = String::new();
        let mut latch = false;
        for c in asciish {
            match (latch, *c) {
                (false, 32 ...127) => r.push(*c as char),
                _ => {
                    if !latch { r.push('#'); latch = true; }
                    r.push_str(&::alloc::fmt::format(::std::fmt::Arguments::new_v1_formatted(&[""],
                                                                                             &match (&*c,)
                                                                                                  {
                                                                                                  (arg0,)
                                                                                                  =>
                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::std::fmt::LowerHex::fmt)],
                                                                                              },
                                                                                             &[::std::fmt::rt::v1::Argument{position:
                                                                                                                                ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                                            format:
                                                                                                                                ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                                   ' ',
                                                                                                                                                               align:
                                                                                                                                                                   ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                                               flags:
                                                                                                                                                                   8u32,
                                                                                                                                                               precision:
                                                                                                                                                                   ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                                               width:
                                                                                                                                                                   ::std::fmt::rt::v1::Count::Is(2usize),},}])));
                }
            }
        }
        r
    }
}
pub mod crypto {
    //! Cryptographic utilities.
    #[cfg(feature = "std")]
    use parity_codec::{Encode, Decode};
    #[cfg(feature = "std")]
    use regex::Regex;
    #[cfg(feature = "std")]
    use base58::{FromBase58, ToBase58};
    /// The root phrase for our publicly known keys.
    pub const DEV_PHRASE: &str =
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    /// The address of the associated root phrase for our publicly known keys.
    pub const DEV_ADDRESS: &str =
        "5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV";
    /// The infallible type.
    pub enum Infallible { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Infallible {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            unsafe { ::std::intrinsics::unreachable() }
        }
    }
    /// The length of the junction identifier. Note that this is also referred to as the
    /// `CHAIN_CODE_LENGTH` in the context of Schnorrkel.
    #[cfg(feature = "std")]
    pub const JUNCTION_ID_LEN: usize = 32;
    /// Similar to `From`, except that the onus is on the part of the caller to ensure
    /// that data passed in makes sense. Basically, you're not guaranteed to get anything
    /// sensible out.
    pub trait UncheckedFrom<T> {
        /// Convert from an instance of `T` to Self. This is not guaranteed to be
        /// whatever counts as a valid instance of `T` and it's up to the caller to
        /// ensure that it makes sense.
        fn unchecked_from(t: T)
        -> Self;
    }
    /// The counterpart to `UncheckedFrom`.
    pub trait UncheckedInto<T> {
        /// The counterpart to `unchecked_from`.
        fn unchecked_into(self)
        -> T;
    }
    impl <S, T: UncheckedFrom<S>> UncheckedInto<T> for S {
        fn unchecked_into(self) -> T { T::unchecked_from(self) }
    }
    /// An error with the interpretation of a secret.
    #[cfg(feature = "std")]
    #[structural_match]
    pub enum SecretStringError {

        /// The overall format was invalid (e.g. the seed phrase contained symbols).
        InvalidFormat,

        /// The seed phrase provided is not a valid BIP39 phrase.
        InvalidPhrase,

        /// The supplied password was invalid.
        InvalidPassword,

        /// The seed is invalid (bad content).
        InvalidSeed,

        /// The seed has an invalid length.
        InvalidSeedLength,

        /// The derivation path was invalid (e.g. contains soft junctions when they are not supported).
        InvalidPath,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for SecretStringError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&SecretStringError::InvalidFormat,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidFormat");
                    debug_trait_builder.finish()
                }
                (&SecretStringError::InvalidPhrase,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidPhrase");
                    debug_trait_builder.finish()
                }
                (&SecretStringError::InvalidPassword,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidPassword");
                    debug_trait_builder.finish()
                }
                (&SecretStringError::InvalidSeed,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidSeed");
                    debug_trait_builder.finish()
                }
                (&SecretStringError::InvalidSeedLength,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidSeedLength");
                    debug_trait_builder.finish()
                }
                (&SecretStringError::InvalidPath,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidPath");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for SecretStringError {
        #[inline]
        fn clone(&self) -> SecretStringError {
            match (&*self,) {
                (&SecretStringError::InvalidFormat,) =>
                SecretStringError::InvalidFormat,
                (&SecretStringError::InvalidPhrase,) =>
                SecretStringError::InvalidPhrase,
                (&SecretStringError::InvalidPassword,) =>
                SecretStringError::InvalidPassword,
                (&SecretStringError::InvalidSeed,) =>
                SecretStringError::InvalidSeed,
                (&SecretStringError::InvalidSeedLength,) =>
                SecretStringError::InvalidSeedLength,
                (&SecretStringError::InvalidPath,) =>
                SecretStringError::InvalidPath,
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for SecretStringError {
        #[inline]
        fn eq(&self, other: &SecretStringError) -> bool {
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
    impl ::std::cmp::Eq for SecretStringError {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    /// A since derivation junction description. It is the single parameter used when creating
    /// a new secret key from an existing secret key and, in the case of `SoftRaw` and `SoftIndex`
    /// a new public key from an existing public key.
    #[cfg(feature = "std")]
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum DeriveJunction {

        /// Soft (vanilla) derivation. Public keys have a correspondent derivation.
        Soft([u8; JUNCTION_ID_LEN]),

        /// Hard ("hardened") derivation. Public keys do not have a correspondent derivation.
        Hard([u8; JUNCTION_ID_LEN]),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for DeriveJunction { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for DeriveJunction {
        #[inline]
        fn clone(&self) -> DeriveJunction {
            {
                let _:
                        ::std::clone::AssertParamIsClone<[u8; JUNCTION_ID_LEN]>;
                let _:
                        ::std::clone::AssertParamIsClone<[u8; JUNCTION_ID_LEN]>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for DeriveJunction {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<[u8; JUNCTION_ID_LEN]>;
                let _: ::std::cmp::AssertParamIsEq<[u8; JUNCTION_ID_LEN]>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for DeriveJunction {
        #[inline]
        fn eq(&self, other: &DeriveJunction) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&DeriveJunction::Soft(ref __self_0),
                         &DeriveJunction::Soft(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&DeriveJunction::Hard(ref __self_0),
                         &DeriveJunction::Hard(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &DeriveJunction) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&DeriveJunction::Soft(ref __self_0),
                         &DeriveJunction::Soft(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&DeriveJunction::Hard(ref __self_0),
                         &DeriveJunction::Hard(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::hash::Hash for DeriveJunction {
        fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
            match (&*self,) {
                (&DeriveJunction::Soft(ref __self_0),) => {
                    ::std::hash::Hash::hash(&unsafe {
                                                 ::std::intrinsics::discriminant_value(self)
                                             }, state);
                    ::std::hash::Hash::hash(&(*__self_0), state)
                }
                (&DeriveJunction::Hard(ref __self_0),) => {
                    ::std::hash::Hash::hash(&unsafe {
                                                 ::std::intrinsics::discriminant_value(self)
                                             }, state);
                    ::std::hash::Hash::hash(&(*__self_0), state)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for DeriveJunction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&DeriveJunction::Soft(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Soft");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&DeriveJunction::Hard(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Hard");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_DeriveJunction: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for DeriveJunction {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        DeriveJunction::Soft(ref aa) => {
                            dest.push_byte(0usize as u8);
                            dest.push(aa);
                        }
                        DeriveJunction::Hard(ref aa) => {
                            dest.push_byte(1usize as u8);
                            dest.push(aa);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_DeriveJunction: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for DeriveJunction {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => {
                            Some(DeriveJunction::Soft(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 1usize as u8 => {
                            Some(DeriveJunction::Hard(_parity_codec::Decode::decode(input)?))
                        }
                        _ => None,
                    }
                }
            }
        };
    #[cfg(feature = "std")]
    impl DeriveJunction {
        /// Consume self to return a soft derive junction with the same chain code.
        pub fn soften(self) -> Self {
            DeriveJunction::Soft(self.unwrap_inner())
        }
        /// Consume self to return a hard derive junction with the same chain code.
        pub fn harden(self) -> Self {
            DeriveJunction::Hard(self.unwrap_inner())
        }
        /// Create a new soft (vanilla) DeriveJunction from a given, encodable, value.
        ///
        /// If you need a hard junction, use `hard()`.
        pub fn soft<T: Encode>(index: T) -> Self {
            let mut cc: [u8; JUNCTION_ID_LEN] = Default::default();
            index.using_encoded(|data|
                                    if data.len() > JUNCTION_ID_LEN {
                                        let hash_result =
                                            blake2_rfc::blake2b::blake2b(JUNCTION_ID_LEN,
                                                                         &[],
                                                                         data);
                                        let hash = hash_result.as_bytes();
                                        cc.copy_from_slice(hash);
                                    } else {
                                        cc[0..data.len()].copy_from_slice(data);
                                    });
            DeriveJunction::Soft(cc)
        }
        /// Create a new hard (hardened) DeriveJunction from a given, encodable, value.
        ///
        /// If you need a soft junction, use `soft()`.
        pub fn hard<T: Encode>(index: T) -> Self {
            Self::soft(index).harden()
        }
        /// Consume self to return the chain code.
        pub fn unwrap_inner(self) -> [u8; JUNCTION_ID_LEN] {
            match self {
                DeriveJunction::Hard(c) | DeriveJunction::Soft(c) => c,
            }
        }
        /// Get a reference to the inner junction id.
        pub fn inner(&self) -> &[u8; JUNCTION_ID_LEN] {
            match self {
                DeriveJunction::Hard(ref c) | DeriveJunction::Soft(ref c) =>
                c,
            }
        }
        /// Return `true` if the junction is soft.
        pub fn is_soft(&self) -> bool {
            match *self { DeriveJunction::Soft(_) => true, _ => false, }
        }
        /// Return `true` if the junction is hard.
        pub fn is_hard(&self) -> bool {
            match *self { DeriveJunction::Hard(_) => true, _ => false, }
        }
    }
    #[cfg(feature = "std")]
    impl <T: AsRef<str>> From<T> for DeriveJunction {
        fn from(j: T) -> DeriveJunction {
            let j = j.as_ref();
            let (code, hard) =
                if j.starts_with("/") { (&j[1..], true) } else { (j, false) };
            let res =
                if let Ok(n) = str::parse::<u64>(code) {
                    DeriveJunction::soft(n)
                } else { DeriveJunction::soft(code) };
            if hard { res.harden() } else { res }
        }
    }
    /// An error type for SS58 decoding.
    #[cfg(feature = "std")]
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum PublicError {

        /// Bad alphabet.
        BadBase58,

        /// Bad length.
        BadLength,

        /// Unknown version.
        UnknownVersion,

        /// Invalid checksum.
        InvalidChecksum,

        /// Invalid format.
        InvalidFormat,

        /// Invalid derivation path.
        InvalidPath,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for PublicError {
        #[inline]
        fn clone(&self) -> PublicError { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for PublicError { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for PublicError {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for PublicError {
        #[inline]
        fn eq(&self, other: &PublicError) -> bool {
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
    impl ::std::fmt::Debug for PublicError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&PublicError::BadBase58,) => {
                    let mut debug_trait_builder = f.debug_tuple("BadBase58");
                    debug_trait_builder.finish()
                }
                (&PublicError::BadLength,) => {
                    let mut debug_trait_builder = f.debug_tuple("BadLength");
                    debug_trait_builder.finish()
                }
                (&PublicError::UnknownVersion,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UnknownVersion");
                    debug_trait_builder.finish()
                }
                (&PublicError::InvalidChecksum,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidChecksum");
                    debug_trait_builder.finish()
                }
                (&PublicError::InvalidFormat,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidFormat");
                    debug_trait_builder.finish()
                }
                (&PublicError::InvalidPath,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidPath");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Key that can be encoded to/from SS58.
    #[cfg(feature = "std")]
    pub trait Ss58Codec: Sized {
        /// Some if the string is a properly encoded SS58Check address.
        fn from_ss58check(s: &str)
        -> Result<Self, PublicError>;
        /// Some if the string is a properly encoded SS58Check address, optionally with
        /// a derivation path following.
        fn from_string(s: &str) -> Result<Self, PublicError> {
            Self::from_ss58check(s)
        }
        /// Return the ss58-check string for this key.
        fn to_ss58check(&self)
        -> String;
    }
    #[cfg(feature = "std")]
    /// Derivable key trait.
    pub trait Derive: Sized {
        /// Derive a child key from a series of given junctions.
        ///
        /// Will be `None` for public keys if there are any hard junctions in there.
        fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, _path: Iter)
         -> Option<Self> {
            None
        }
    }
    #[cfg(feature = "std")]
    const PREFIX: &[u8] = b"SS58PRE";
    #[cfg(feature = "std")]
    fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
        let mut context = blake2_rfc::blake2b::Blake2b::new(64);
        context.update(PREFIX);
        context.update(data);
        context.finalize()
    }
    #[cfg(feature = "std")]
    impl <T: AsMut<[u8]> + AsRef<[u8]> + Default + Derive> Ss58Codec for T {
        fn from_ss58check(s: &str) -> Result<Self, PublicError> {
            let mut res = T::default();
            let len = res.as_mut().len();
            let d = s.from_base58().map_err(|_| PublicError::BadBase58)?;
            if d.len() != len + 3 { return Err(PublicError::BadLength); }
            if d[0] != 42 { return Err(PublicError::UnknownVersion); }
            if d[len + 1..len + 3] !=
                   ss58hash(&d[0..len + 1]).as_bytes()[0..2] {
                return Err(PublicError::InvalidChecksum);
            }
            res.as_mut().copy_from_slice(&d[1..len + 1]);
            Ok(res)
        }
        fn to_ss58check(&self) -> String {
            let mut v = <[_]>::into_vec(box [42u8]);
            v.extend(self.as_ref());
            let r = ss58hash(&v);
            v.extend(&r.as_bytes()[0..2]);
            v.to_base58()
        }
        fn from_string(s: &str) -> Result<Self, PublicError> {
            let re =
                Regex::new(r"^(?P<ss58>[\w\d]+)?(?P<path>(//?[^/]+)*)$").expect("constructed from known-good static value; qed");
            let cap = re.captures(s).ok_or(PublicError::InvalidFormat)?;
            let re_junction =
                Regex::new(r"/(/?[^/]+)").expect("constructed from known-good static value; qed");
            let addr =
                Self::from_ss58check(cap.name("ss58").map(|r|
                                                              r.as_str()).unwrap_or(DEV_ADDRESS))?;
            if cap["path"].is_empty() {
                Ok(addr)
            } else {
                let path =
                    re_junction.captures_iter(&cap["path"]).map(|f|
                                                                    DeriveJunction::from(&f[1]));
                addr.derive(path).ok_or(PublicError::InvalidPath)
            }
        }
    }
    /// Trait suitable for typical cryptographic PKI key pair type.
    ///
    /// For now it just specifies how to create a key from a phrase and derivation path.
    #[cfg(feature = "std")]
    pub trait Pair: Sized + 'static {
        /// TThe type which is used to encode a public key.
        type
        Public;
        /// The type used to (minimally) encode the data required to securely create
        /// a new key pair.
        type
        Seed;
        /// The type used to represent a signature. Can be created from a key pair and a message
        /// and verified with the message and a public key.
        type
        Signature;
        /// Error returned from the `derive` function.
        type
        DeriveError;
        /// Generate new secure (random) key pair.
        ///
        /// This is only for ephemeral keys really, since you won't have access to the secret key
        /// for storage. If you want a persistent key pair, use `generate_with_phrase` instead.
        fn generate()
        -> Self;
        /// Generate new secure (random) key pair and provide the recovery phrase.
        ///
        /// You can recover the same key later with `from_phrase`.
        ///
        /// This is generally slower than `generate()`, so prefer that unless you need to persist
        /// the key from the current session.
        fn generate_with_phrase(password: Option<&str>)
        -> (Self, String);
        /// Returns the KeyPair from the English BIP39 seed `phrase`, or `None` if it's invalid.
        fn from_phrase(phrase: &str, password: Option<&str>)
        -> Result<Self, SecretStringError>;
        /// Derive a child key from a series of given junctions.
        fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, path: Iter)
        -> Result<Self, Self::DeriveError>;
        /// Generate new key pair from the provided `seed`.
        ///
        /// @WARNING: THIS WILL ONLY BE SECURE IF THE `seed` IS SECURE. If it can be guessed
        /// by an attacker then they can also derive your key.
        fn from_seed(seed: Self::Seed)
        -> Self;
        /// Make a new key pair from secret seed material. The slice must be the correct size or
        /// it will return `None`.
        ///
        /// @WARNING: THIS WILL ONLY BE SECURE IF THE `seed` IS SECURE. If it can be guessed
        /// by an attacker then they can also derive your key.
        fn from_seed_slice(seed: &[u8])
        -> Result<Self, SecretStringError>;
        /// Construct a key from a phrase, password and path.
        fn from_standard_components<I: Iterator<Item =
                                    DeriveJunction>>(phrase: &str,
                                                     password: Option<&str>,
                                                     path: I)
        -> Result<Self, SecretStringError>;
        /// Sign a message.
        fn sign(&self, message: &[u8])
        -> Self::Signature;
        /// Verify a signature on a message. Returns true if the signature is good.
        fn verify<P: AsRef<Self::Public>,
                  M: AsRef<[u8]>>(sig: &Self::Signature, message: M,
                                  pubkey: P)
        -> bool;
        /// Verify a signature on a message. Returns true if the signature is good.
        fn verify_weak<P: AsRef<[u8]>,
                       M: AsRef<[u8]>>(sig: &[u8], message: M, pubkey: P)
        -> bool;
        /// Get the public key.
        fn public(&self)
        -> Self::Public;
        /// Interprets the string `s` in order to generate a key Pair.
        ///
        /// This takes a helper function to do the key generation from a phrase, password and
        /// junction iterator.
        ///
        /// - If `s` is a possibly `0x` prefixed 64-digit hex string, then it will be interpreted
        /// directly as a `MiniSecretKey` (aka "seed" in `subkey`).
        /// - If `s` is a valid BIP-39 key phrase of 12, 15, 18, 21 or 24 words, then the key will
        /// be derived from it. In this case:
        ///   - the phrase may be followed by one or more items delimited by `/` characters.
        ///   - the path may be followed by `///`, in which case everything after the `///` is treated
        /// as a password.
        /// - If `s` begins with a `/` character it is prefixed with the Substrate public `DEV_PHRASE` and
        /// interpreted as above.
        ///
        /// In this case they are interpreted as HDKD junctions; purely numeric items are interpreted as
        /// integers, non-numeric items as strings. Junctions prefixed with `/` are interpreted as soft
        /// junctions, and with `//` as hard junctions.
        ///
        /// There is no correspondence mapping between SURI strings and the keys they represent.
        /// Two different non-identical strings can actually lead to the same secret being derived.
        /// Notably, integer junction indices may be legally prefixed with arbitrary number of zeros.
        /// Similarly an empty password (ending the SURI with `///`) is perfectly valid and will generally
        /// be equivalent to no password at all.
        ///
        /// `None` is returned if no matches are found.
        fn from_string(s: &str, password_override: Option<&str>)
         -> Result<Self, SecretStringError> {
            let hex_seed = if s.starts_with("0x") { &s[2..] } else { s };
            if let Ok(d) = hex::decode(hex_seed) {
                if let Ok(r) = Self::from_seed_slice(&d) { return Ok(r) }
            }
            let re =
                Regex::new(r"^(?P<phrase>\w+( \w+)*)?(?P<path>(//?[^/]+)*)(///(?P<password>.*))?$").expect("constructed from known-good static value; qed");
            let cap = re.captures(s).ok_or(SecretStringError::InvalidFormat)?;
            let re_junction =
                Regex::new(r"/(/?[^/]+)").expect("constructed from known-good static value; qed");
            let path =
                re_junction.captures_iter(&cap["path"]).map(|f|
                                                                DeriveJunction::from(&f[1]));
            Self::from_standard_components(cap.name("phrase").map(|r|
                                                                      r.as_str()).unwrap_or(DEV_PHRASE),
                                           password_override.or_else(||
                                                                         cap.name("password").map(|m|
                                                                                                      m.as_str())),
                                           path)
        }
    }
}
pub mod u32_trait {
    //! An u32 trait with "values" as impl'd types.
    /// A u32 value, wrapped in a trait because we don't yet have const generics.
    pub trait Value {
        /// The actual value represented by the impl'ing type.
        const
        VALUE:
        u32;
    }
    /// Type representing the value 0 for the `Value` trait.
    pub struct _0;
    impl Value for _0 {
        const
        VALUE:
        u32
        =
        0;
    }
    /// Type representing the value 1 for the `Value` trait.
    pub struct _1;
    impl Value for _1 {
        const
        VALUE:
        u32
        =
        1;
    }
    /// Type representing the value 2 for the `Value` trait.
    pub struct _2;
    impl Value for _2 {
        const
        VALUE:
        u32
        =
        2;
    }
    /// Type representing the value 3 for the `Value` trait.
    pub struct _3;
    impl Value for _3 {
        const
        VALUE:
        u32
        =
        3;
    }
    /// Type representing the value 4 for the `Value` trait.
    pub struct _4;
    impl Value for _4 {
        const
        VALUE:
        u32
        =
        4;
    }
    /// Type representing the value 5 for the `Value` trait.
    pub struct _5;
    impl Value for _5 {
        const
        VALUE:
        u32
        =
        5;
    }
    /// Type representing the value 6 for the `Value` trait.
    pub struct _6;
    impl Value for _6 {
        const
        VALUE:
        u32
        =
        6;
    }
    /// Type representing the value 7 for the `Value` trait.
    pub struct _7;
    impl Value for _7 {
        const
        VALUE:
        u32
        =
        7;
    }
    /// Type representing the value 8 for the `Value` trait.
    pub struct _8;
    impl Value for _8 {
        const
        VALUE:
        u32
        =
        8;
    }
    /// Type representing the value 9 for the `Value` trait.
    pub struct _9;
    impl Value for _9 {
        const
        VALUE:
        u32
        =
        9;
    }
    /// Type representing the value 10 for the `Value` trait.
    pub struct _10;
    impl Value for _10 {
        const
        VALUE:
        u32
        =
        10;
    }
    /// Type representing the value 11 for the `Value` trait.
    pub struct _11;
    impl Value for _11 {
        const
        VALUE:
        u32
        =
        11;
    }
    /// Type representing the value 12 for the `Value` trait.
    pub struct _12;
    impl Value for _12 {
        const
        VALUE:
        u32
        =
        12;
    }
    /// Type representing the value 13 for the `Value` trait.
    pub struct _13;
    impl Value for _13 {
        const
        VALUE:
        u32
        =
        13;
    }
    /// Type representing the value 14 for the `Value` trait.
    pub struct _14;
    impl Value for _14 {
        const
        VALUE:
        u32
        =
        14;
    }
    /// Type representing the value 15 for the `Value` trait.
    pub struct _15;
    impl Value for _15 {
        const
        VALUE:
        u32
        =
        15;
    }
    /// Type representing the value 16 for the `Value` trait.
    pub struct _16;
    impl Value for _16 {
        const
        VALUE:
        u32
        =
        16;
    }
    /// Type representing the value 24 for the `Value` trait.
    pub struct _24;
    impl Value for _24 {
        const
        VALUE:
        u32
        =
        24;
    }
    /// Type representing the value 32 for the `Value` trait.
    pub struct _32;
    impl Value for _32 {
        const
        VALUE:
        u32
        =
        32;
    }
    /// Type representing the value 40 for the `Value` trait.
    pub struct _40;
    impl Value for _40 {
        const
        VALUE:
        u32
        =
        40;
    }
    /// Type representing the value 48 for the `Value` trait.
    pub struct _48;
    impl Value for _48 {
        const
        VALUE:
        u32
        =
        48;
    }
    /// Type representing the value 56 for the `Value` trait.
    pub struct _56;
    impl Value for _56 {
        const
        VALUE:
        u32
        =
        56;
    }
    /// Type representing the value 64 for the `Value` trait.
    pub struct _64;
    impl Value for _64 {
        const
        VALUE:
        u32
        =
        64;
    }
    /// Type representing the value 80 for the `Value` trait.
    pub struct _80;
    impl Value for _80 {
        const
        VALUE:
        u32
        =
        80;
    }
    /// Type representing the value 96 for the `Value` trait.
    pub struct _96;
    impl Value for _96 {
        const
        VALUE:
        u32
        =
        96;
    }
    /// Type representing the value 112 for the `Value` trait.
    pub struct _112;
    impl Value for _112 {
        const
        VALUE:
        u32
        =
        112;
    }
    /// Type representing the value 128 for the `Value` trait.
    pub struct _128;
    impl Value for _128 {
        const
        VALUE:
        u32
        =
        128;
    }
    /// Type representing the value 160 for the `Value` trait.
    pub struct _160;
    impl Value for _160 {
        const
        VALUE:
        u32
        =
        160;
    }
    /// Type representing the value 192 for the `Value` trait.
    pub struct _192;
    impl Value for _192 {
        const
        VALUE:
        u32
        =
        192;
    }
    /// Type representing the value 224 for the `Value` trait.
    pub struct _224;
    impl Value for _224 {
        const
        VALUE:
        u32
        =
        224;
    }
    /// Type representing the value 256 for the `Value` trait.
    pub struct _256;
    impl Value for _256 {
        const
        VALUE:
        u32
        =
        256;
    }
    /// Type representing the value 384 for the `Value` trait.
    pub struct _384;
    impl Value for _384 {
        const
        VALUE:
        u32
        =
        384;
    }
    /// Type representing the value 512 for the `Value` trait.
    pub struct _512;
    impl Value for _512 {
        const
        VALUE:
        u32
        =
        512;
    }
}
pub mod ed25519 {
    //! Simple Ed25519 API.
    use crate::{hash::H256, hash::H512};
    use parity_codec::{Encode, Decode};
    #[cfg(feature = "std")]
    use blake2_rfc;
    #[cfg(feature = "std")]
    use substrate_bip39::seed_from_entropy;
    #[cfg(feature = "std")]
    use bip39::{Mnemonic, Language, MnemonicType};
    #[cfg(feature = "std")]
    use rand::Rng;
    #[cfg(feature = "std")]
    use crate::crypto::{Pair as TraitPair, DeriveJunction, SecretStringError,
                        Derive, Ss58Codec};
    #[cfg(feature = "std")]
    use serde::{de, Serializer, Serialize, Deserializer, Deserialize};
    use crate::crypto::UncheckedFrom;
    /// A secret seed. It's not called a "secret key" because ring doesn't expose the secret keys
    /// of the key pair (yeah, dumb); as such we're forced to remember the seed manually if we
    /// will need it later (such as for HDKD).
    #[cfg(feature = "std")]
    type Seed = [u8; 32];
    /// A public key.
    #[structural_match]
    pub struct Public(pub [u8; 32]);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Public {
        #[inline]
        fn eq(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for Public {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<[u8; 32]>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialOrd for Public {
        #[inline]
        fn partial_cmp(&self, other: &Public)
         -> ::std::option::Option<::std::cmp::Ordering> {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                              &(*__self_1_0))
                        {
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                        =>
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        == ::std::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        != ::std::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        == ::std::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        != ::std::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Ord for Public {
        #[inline]
        fn cmp(&self, other: &Public) -> ::std::cmp::Ordering {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    match ::std::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0))
                        {
                        ::std::cmp::Ordering::Equal =>
                        ::std::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Public {
        #[inline]
        fn clone(&self) -> Public {
            match *self {
                Public(ref __self_0_0) =>
                Public(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Public: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for Public {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.0);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Public: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for Public {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Public(_parity_codec::Decode::decode(input)?))
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::default::Default for Public {
        #[inline]
        fn default() -> Public { Public(::std::default::Default::default()) }
    }
    /// A key pair.
    #[cfg(feature = "std")]
    pub struct Pair(ed25519_dalek::Keypair);
    #[cfg(feature = "std")]
    impl Clone for Pair {
        fn clone(&self) -> Self {
            Pair(ed25519_dalek::Keypair{public: self.0.public.clone(),
                                        secret:
                                            ed25519_dalek::SecretKey::from_bytes(self.0.secret.as_bytes()).expect("key is always the correct size; qed"),})
        }
    }
    impl AsRef<[u8; 32]> for Public {
        fn as_ref(&self) -> &[u8; 32] { &self.0 }
    }
    impl AsRef<[u8]> for Public {
        fn as_ref(&self) -> &[u8] { &self.0[..] }
    }
    impl AsMut<[u8]> for Public {
        fn as_mut(&mut self) -> &mut [u8] { &mut self.0[..] }
    }
    impl From<Public> for [u8; 32] {
        fn from(x: Public) -> Self { x.0 }
    }
    #[cfg(feature = "std")]
    impl From<Pair> for Public {
        fn from(x: Pair) -> Self { x.public() }
    }
    impl AsRef<Public> for Public {
        fn as_ref(&self) -> &Public { &self }
    }
    impl From<Public> for H256 {
        fn from(x: Public) -> Self { x.0.into() }
    }
    impl UncheckedFrom<[u8; 32]> for Public {
        fn unchecked_from(x: [u8; 32]) -> Self { Public::from_raw(x) }
    }
    impl UncheckedFrom<H256> for Public {
        fn unchecked_from(x: H256) -> Self { Public::from_h256(x) }
    }
    #[cfg(feature = "std")]
    impl ::std::fmt::Display for Public {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                      &match (&self.to_ss58check(),)
                                                           {
                                                           (arg0,) =>
                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                        ::std::fmt::Display::fmt)],
                                                       }))
        }
    }
    #[cfg(feature = "std")]
    impl ::std::fmt::Debug for Public {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let s = self.to_ss58check();
            f.write_fmt(::std::fmt::Arguments::new_v1(&["", " (", "...)"],
                                                      &match (&crate::hexdisplay::HexDisplay::from(&self.0),
                                                              &&s[0..8]) {
                                                           (arg0, arg1) =>
                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                        ::std::fmt::Display::fmt),
                                                            ::std::fmt::ArgumentV1::new(arg1,
                                                                                        ::std::fmt::Display::fmt)],
                                                       }))
        }
    }
    #[cfg(feature = "std")]
    impl Serialize for Public {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
         S: Serializer {
            serializer.serialize_str(&self.to_ss58check())
        }
    }
    #[cfg(feature = "std")]
    impl <'de> Deserialize<'de> for Public {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
         D: Deserializer<'de> {
            Public::from_ss58check(&String::deserialize(deserializer)?).map_err(|e|
                                                                                    de::Error::custom(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                         &match (&e,)
                                                                                                                                                              {
                                                                                                                                                              (arg0,)
                                                                                                                                                              =>
                                                                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                           ::std::fmt::Debug::fmt)],
                                                                                                                                                          }))))
        }
    }
    #[cfg(feature = "std")]
    impl ::std::hash::Hash for Public {
        fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    /// A signature (a 512-bit value).
    pub struct Signature(pub [u8; 64]);
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Signature: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for Signature {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.0);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Signature: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for Signature {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Signature(_parity_codec::Decode::decode(input)?))
                }
            }
        };
    impl Clone for Signature {
        fn clone(&self) -> Self {
            let mut r = [0u8; 64];
            r.copy_from_slice(&self.0[..]);
            Signature(r)
        }
    }
    impl Default for Signature {
        fn default() -> Self { Signature([0u8; 64]) }
    }
    impl PartialEq for Signature {
        fn eq(&self, b: &Self) -> bool { &self.0[..] == &b.0[..] }
    }
    impl Eq for Signature { }
    impl From<Signature> for H512 {
        fn from(v: Signature) -> H512 { H512::from(v.0) }
    }
    impl From<Signature> for [u8; 64] {
        fn from(v: Signature) -> [u8; 64] { v.0 }
    }
    impl AsRef<[u8; 64]> for Signature {
        fn as_ref(&self) -> &[u8; 64] { &self.0 }
    }
    impl AsRef<[u8]> for Signature {
        fn as_ref(&self) -> &[u8] { &self.0[..] }
    }
    impl AsMut<[u8]> for Signature {
        fn as_mut(&mut self) -> &mut [u8] { &mut self.0[..] }
    }
    #[cfg(feature = "std")]
    impl ::std::fmt::Debug for Signature {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                      &match (&crate::hexdisplay::HexDisplay::from(&self.0),)
                                                           {
                                                           (arg0,) =>
                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                        ::std::fmt::Display::fmt)],
                                                       }))
        }
    }
    #[cfg(feature = "std")]
    impl ::std::hash::Hash for Signature {
        fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
            ::std::hash::Hash::hash(&self.0[..], state);
        }
    }
    impl Signature {
        /// A new instance from the given 64-byte `data`.
        ///
        /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
        /// you are certain that the array actually is a signature. GIGO!
        pub fn from_raw(data: [u8; 64]) -> Signature { Signature(data) }
        /// A new instance from the given slice that should be 64 bytes long.
        ///
        /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
        /// you are certain that the array actually is a signature. GIGO!
        pub fn from_slice(data: &[u8]) -> Self {
            let mut r = [0u8; 64];
            r.copy_from_slice(data);
            Signature(r)
        }
        /// A new instance from an H512.
        ///
        /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
        /// you are certain that the array actually is a signature. GIGO!
        pub fn from_h512(v: H512) -> Signature { Signature(v.into()) }
    }
    /// A localized signature also contains sender information.
    #[cfg(feature = "std")]
    #[structural_match]
    pub struct LocalizedSignature {
        /// The signer of the signature.
        pub signer: Public,
        /// The signature itself.
        pub signature: Signature,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for LocalizedSignature {
        #[inline]
        fn eq(&self, other: &LocalizedSignature) -> bool {
            match *other {
                LocalizedSignature {
                signer: ref __self_1_0, signature: ref __self_1_1 } =>
                match *self {
                    LocalizedSignature {
                    signer: ref __self_0_0, signature: ref __self_0_1 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LocalizedSignature) -> bool {
            match *other {
                LocalizedSignature {
                signer: ref __self_1_0, signature: ref __self_1_1 } =>
                match *self {
                    LocalizedSignature {
                    signer: ref __self_0_0, signature: ref __self_0_1 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for LocalizedSignature {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<Public>;
                let _: ::std::cmp::AssertParamIsEq<Signature>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LocalizedSignature {
        #[inline]
        fn clone(&self) -> LocalizedSignature {
            match *self {
                LocalizedSignature {
                signer: ref __self_0_0, signature: ref __self_0_1 } =>
                LocalizedSignature{signer:
                                       ::std::clone::Clone::clone(&(*__self_0_0)),
                                   signature:
                                       ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for LocalizedSignature {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LocalizedSignature {
                signer: ref __self_0_0, signature: ref __self_0_1 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("LocalizedSignature");
                    let _ =
                        debug_trait_builder.field("signer", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("signature",
                                                  &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_LocalizedSignature: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for LocalizedSignature {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.signer);
                    dest.push(&self.signature);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_LocalizedSignature: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for LocalizedSignature {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(LocalizedSignature{signer:
                                                _parity_codec::Decode::decode(input)?,
                                            signature:
                                                _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    /// An error type for SS58 decoding.
    #[cfg(feature = "std")]
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum PublicError {

        /// Bad alphabet.
        BadBase58,

        /// Bad length.
        BadLength,

        /// Unknown version.
        UnknownVersion,

        /// Invalid checksum.
        InvalidChecksum,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for PublicError {
        #[inline]
        fn clone(&self) -> PublicError { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for PublicError { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for PublicError {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for PublicError {
        #[inline]
        fn eq(&self, other: &PublicError) -> bool {
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
    impl ::std::fmt::Debug for PublicError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&PublicError::BadBase58,) => {
                    let mut debug_trait_builder = f.debug_tuple("BadBase58");
                    debug_trait_builder.finish()
                }
                (&PublicError::BadLength,) => {
                    let mut debug_trait_builder = f.debug_tuple("BadLength");
                    debug_trait_builder.finish()
                }
                (&PublicError::UnknownVersion,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UnknownVersion");
                    debug_trait_builder.finish()
                }
                (&PublicError::InvalidChecksum,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidChecksum");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl Public {
        /// A new instance from the given 32-byte `data`.
        ///
        /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
        /// you are certain that the array actually is a pubkey. GIGO!
        pub fn from_raw(data: [u8; 32]) -> Self { Public(data) }
        /// A new instance from the given slice that should be 32 bytes long.
        ///
        /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
        /// you are certain that the array actually is a pubkey. GIGO!
        pub fn from_slice(data: &[u8]) -> Self {
            let mut r = [0u8; 32];
            r.copy_from_slice(data);
            Public(r)
        }
        /// A new instance from an H256.
        ///
        /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
        /// you are certain that the array actually is a pubkey. GIGO!
        pub fn from_h256(x: H256) -> Self { Public(x.into()) }
        /// Return a `Vec<u8>` filled with raw data.
        #[cfg(feature = "std")]
        pub fn to_raw_vec(self) -> Vec<u8> {
            let r: &[u8; 32] = self.as_ref();
            r.to_vec()
        }
        /// Return a slice filled with raw data.
        pub fn as_slice(&self) -> &[u8] {
            let r: &[u8; 32] = self.as_ref();
            &r[..]
        }
        /// Return a slice filled with raw data.
        pub fn as_array_ref(&self) -> &[u8; 32] { self.as_ref() }
    }
    #[cfg(feature = "std")]
    impl Derive for Public { }
    #[cfg(feature = "std")]
    impl AsRef<Pair> for Pair {
        fn as_ref(&self) -> &Pair { &self }
    }
    /// Derive a single hard junction.
    #[cfg(feature = "std")]
    fn derive_hard_junction(secret_seed: &Seed, cc: &[u8; 32]) -> Seed {
        ("Ed25519HDKD", secret_seed,
         cc).using_encoded(|data|
                               {
                                   let mut res = [0u8; 32];
                                   res.copy_from_slice(blake2_rfc::blake2b::blake2b(32,
                                                                                    &[],
                                                                                    data).as_bytes());
                                   res
                               })
    }
    /// An error when deriving a key.
    #[cfg(feature = "std")]
    pub enum DeriveError {

        /// A soft key was found in the path (and is unsupported).
        SoftKeyInPath,
    }
    #[cfg(feature = "std")]
    impl TraitPair for Pair {
        type
        Public
        =
        Public;
        type
        Seed
        =
        Seed;
        type
        Signature
        =
        Signature;
        type
        DeriveError
        =
        DeriveError;
        /// Generate new secure (random) key pair.
        ///
        /// This is only for ephemeral keys really, since you won't have access to the secret key
        /// for storage. If you want a persistent key pair, use `generate_with_phrase` instead.
        fn generate() -> Pair {
            let mut seed: Seed = Default::default();
            rand::rngs::EntropyRng::new().fill(seed.as_mut());
            Self::from_seed(seed)
        }
        /// Generate new secure (random) key pair and provide the recovery phrase.
        ///
        /// You can recover the same key later with `from_phrase`.
        fn generate_with_phrase(password: Option<&str>) -> (Pair, String) {
            let mnemonic =
                Mnemonic::new(MnemonicType::Words12, Language::English);
            let phrase = mnemonic.phrase();
            (Self::from_phrase(phrase,
                               password).expect("All phrases generated by Mnemonic are valid; qed"),
             phrase.to_owned())
        }
        /// Generate key pair from given recovery phrase and password.
        fn from_phrase(phrase: &str, password: Option<&str>)
         -> Result<Pair, SecretStringError> {
            let big_seed =
                seed_from_entropy(Mnemonic::from_phrase(phrase,
                                                        Language::English).map_err(|_|
                                                                                       SecretStringError::InvalidPhrase)?.entropy(),
                                  password.unwrap_or("")).map_err(|_|
                                                                      SecretStringError::InvalidSeed)?;
            Self::from_seed_slice(&big_seed[0..32])
        }
        /// Make a new key pair from secret seed material.
        ///
        /// You should never need to use this; generate(), generate_with_phrasee
        fn from_seed(seed: Seed) -> Pair {
            let secret =
                ed25519_dalek::SecretKey::from_bytes(&seed[..]).expect("seed has valid length; qed");
            let public = ed25519_dalek::PublicKey::from(&secret);
            Pair(ed25519_dalek::Keypair{secret, public,})
        }
        /// Make a new key pair from secret seed material. The slice must be 32 bytes long or it
        /// will return `None`.
        ///
        /// You should never need to use this; generate(), generate_with_phrase
        fn from_seed_slice(seed_slice: &[u8])
         -> Result<Pair, SecretStringError> {
            if seed_slice.len() != 32 {
                Err(SecretStringError::InvalidSeedLength)
            } else {
                let mut seed = [0u8; 32];
                seed.copy_from_slice(&seed_slice);
                Ok(Self::from_seed(seed))
            }
        }
        /// Derive a child key from a series of given junctions.
        fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, path: Iter)
         -> Result<Pair, DeriveError> {
            let mut acc = self.0.public.to_bytes();
            for j in path {
                match j {
                    DeriveJunction::Soft(_cc) =>
                    return Err(DeriveError::SoftKeyInPath),
                    DeriveJunction::Hard(cc) =>
                    acc = derive_hard_junction(&acc, &cc),
                }
            }
            Ok(Self::from_seed(acc))
        }
        /// Generate a key from the phrase, password and derivation path.
        fn from_standard_components<I: Iterator<Item =
                                    DeriveJunction>>(phrase: &str,
                                                     password: Option<&str>,
                                                     path: I)
         -> Result<Pair, SecretStringError> {
            Self::from_phrase(phrase,
                              password)?.derive(path).map_err(|_|
                                                                  SecretStringError::InvalidPath)
        }
        /// Get the public key.
        fn public(&self) -> Public {
            let mut r = [0u8; 32];
            let pk = self.0.public.as_bytes();
            r.copy_from_slice(pk);
            Public(r)
        }
        /// Sign a message.
        fn sign(&self, message: &[u8]) -> Signature {
            let r = self.0.sign(message).to_bytes();
            Signature::from_raw(r)
        }
        /// Verify a signature on a message. Returns true if the signature is good.
        fn verify<P: AsRef<Self::Public>,
                  M: AsRef<[u8]>>(sig: &Self::Signature, message: M,
                                  pubkey: P) -> bool {
            Self::verify_weak(&sig.0[..], message.as_ref(),
                              &pubkey.as_ref().0[..])
        }
        /// Verify a signature on a message. Returns true if the signature is good.
        ///
        /// This doesn't use the type system to ensure that `sig` and `pubkey` are the correct
        /// size. Use it only if you're coming from byte buffers and need the speed.
        fn verify_weak<P: AsRef<[u8]>,
                       M: AsRef<[u8]>>(sig: &[u8], message: M, pubkey: P)
         -> bool {
            let public_key =
                match ed25519_dalek::PublicKey::from_bytes(pubkey.as_ref()) {
                    Ok(pk) => pk,
                    Err(_) => return false,
                };
            let sig =
                match ed25519_dalek::Signature::from_bytes(sig) {
                    Ok(s) => s,
                    Err(_) => return false,
                };
            match public_key.verify(message.as_ref(), &sig) {
                Ok(_) => true,
                _ => false,
            }
        }
    }
    #[cfg(feature = "std")]
    impl Pair {
        /// Get the seed for this key.
        pub fn seed(&self) -> &Seed { self.0.public.as_bytes() }
        /// Exactly as `from_string` except that if no matches are found then, the the first 32
        /// characters are taken (padded with spaces as necessary) and used as the MiniSecretKey.
        pub fn from_legacy_string(s: &str, password_override: Option<&str>)
         -> Pair {
            Self::from_string(s,
                              password_override).unwrap_or_else(|_|
                                                                    {
                                                                        let mut padded_seed:
                                                                                Seed =
                                                                            [' '
                                                                                 as
                                                                                 u8;
                                                                                32];
                                                                        let len =
                                                                            s.len().min(32);
                                                                        padded_seed[..len].copy_from_slice(&s.as_bytes()[..len]);
                                                                        Self::from_seed(padded_seed)
                                                                    })
        }
    }
}
pub mod sr25519 {
    //! Simple sr25519 (Schnorr-Ristretto) API.
    //!
    //! Note: `CHAIN_CODE_LENGTH` must be equal to `crate::crypto::JUNCTION_ID_LEN`
    //! for this to work.
    #[cfg(feature = "std")]
    use rand::rngs::OsRng;
    #[cfg(feature = "std")]
    use schnorrkel::{signing_context, Keypair, SecretKey, MiniSecretKey,
                     PublicKey,
                     derive::{Derivation, ChainCode, CHAIN_CODE_LENGTH}};
    #[cfg(feature = "std")]
    use substrate_bip39::mini_secret_from_entropy;
    #[cfg(feature = "std")]
    use bip39::{Mnemonic, Language, MnemonicType};
    #[cfg(feature = "std")]
    use crate::crypto::{Pair as TraitPair, DeriveJunction, Infallible,
                        SecretStringError, Derive, Ss58Codec};
    use crate::{hash::{H256, H512}, crypto::UncheckedFrom};
    use parity_codec::{Encode, Decode};
    #[cfg(feature = "std")]
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
    #[cfg(feature = "std")]
    use schnorrkel::keys::MINI_SECRET_KEY_LENGTH;
    #[cfg(feature = "std")]
    const SIGNING_CTX: &[u8] = b"substrate";
    /// An Schnorrkel/Ristretto x25519 ("sr25519") public key.
    #[structural_match]
    pub struct Public(pub [u8; 32]);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Public {
        #[inline]
        fn eq(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for Public {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<[u8; 32]>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialOrd for Public {
        #[inline]
        fn partial_cmp(&self, other: &Public)
         -> ::std::option::Option<::std::cmp::Ordering> {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                              &(*__self_1_0))
                        {
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                        =>
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        == ::std::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        != ::std::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        == ::std::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &Public) -> bool {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        != ::std::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Ord for Public {
        #[inline]
        fn cmp(&self, other: &Public) -> ::std::cmp::Ordering {
            match *other {
                Public(ref __self_1_0) =>
                match *self {
                    Public(ref __self_0_0) =>
                    match ::std::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0))
                        {
                        ::std::cmp::Ordering::Equal =>
                        ::std::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Public {
        #[inline]
        fn clone(&self) -> Public {
            match *self {
                Public(ref __self_0_0) =>
                Public(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Public: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for Public {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.0);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Public: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for Public {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Public(_parity_codec::Decode::decode(input)?))
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::default::Default for Public {
        #[inline]
        fn default() -> Public { Public(::std::default::Default::default()) }
    }
    /// An Schnorrkel/Ristretto x25519 ("sr25519") key pair.
    #[cfg(feature = "std")]
    pub struct Pair(Keypair);
    impl AsRef<Public> for Public {
        fn as_ref(&self) -> &Public { &self }
    }
    impl AsRef<[u8; 32]> for Public {
        fn as_ref(&self) -> &[u8; 32] { &self.0 }
    }
    impl AsRef<[u8]> for Public {
        fn as_ref(&self) -> &[u8] { &self.0[..] }
    }
    impl AsMut<[u8]> for Public {
        fn as_mut(&mut self) -> &mut [u8] { &mut self.0[..] }
    }
    impl From<Public> for [u8; 32] {
        fn from(x: Public) -> [u8; 32] { x.0 }
    }
    impl From<Public> for H256 {
        fn from(x: Public) -> H256 { x.0.into() }
    }
    impl UncheckedFrom<[u8; 32]> for Public {
        fn unchecked_from(x: [u8; 32]) -> Self { Public::from_raw(x) }
    }
    impl UncheckedFrom<H256> for Public {
        fn unchecked_from(x: H256) -> Self { Public::from_h256(x) }
    }
    #[cfg(feature = "std")]
    impl ::std::fmt::Display for Public {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                      &match (&self.to_ss58check(),)
                                                           {
                                                           (arg0,) =>
                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                        ::std::fmt::Display::fmt)],
                                                       }))
        }
    }
    #[cfg(feature = "std")]
    impl ::std::fmt::Debug for Public {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let s = self.to_ss58check();
            f.write_fmt(::std::fmt::Arguments::new_v1(&["", " (", "...)"],
                                                      &match (&crate::hexdisplay::HexDisplay::from(&self.0),
                                                              &&s[0..8]) {
                                                           (arg0, arg1) =>
                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                        ::std::fmt::Display::fmt),
                                                            ::std::fmt::ArgumentV1::new(arg1,
                                                                                        ::std::fmt::Display::fmt)],
                                                       }))
        }
    }
    #[cfg(feature = "std")]
    impl Serialize for Public {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
         S: Serializer {
            serializer.serialize_str(&self.to_ss58check())
        }
    }
    #[cfg(feature = "std")]
    impl <'de> Deserialize<'de> for Public {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
         D: Deserializer<'de> {
            Public::from_ss58check(&String::deserialize(deserializer)?).map_err(|e|
                                                                                    de::Error::custom(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                         &match (&e,)
                                                                                                                                                              {
                                                                                                                                                              (arg0,)
                                                                                                                                                              =>
                                                                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                           ::std::fmt::Debug::fmt)],
                                                                                                                                                          }))))
        }
    }
    #[cfg(feature = "std")]
    impl ::std::hash::Hash for Public {
        fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    /// An Schnorrkel/Ristretto x25519 ("sr25519") signature.
    ///
    /// Instead of importing it for the local module, alias it to be available as a public type
    pub struct Signature(pub [u8; 64]);
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Signature: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for Signature {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.0);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Signature: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for Signature {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Signature(_parity_codec::Decode::decode(input)?))
                }
            }
        };
    impl Clone for Signature {
        fn clone(&self) -> Self {
            let mut r = [0u8; 64];
            r.copy_from_slice(&self.0[..]);
            Signature(r)
        }
    }
    impl Default for Signature {
        fn default() -> Self { Signature([0u8; 64]) }
    }
    impl PartialEq for Signature {
        fn eq(&self, b: &Self) -> bool { &self.0[..] == &b.0[..] }
    }
    impl Eq for Signature { }
    impl From<Signature> for [u8; 64] {
        fn from(v: Signature) -> [u8; 64] { v.0 }
    }
    impl From<Signature> for H512 {
        fn from(v: Signature) -> H512 { H512::from(v.0) }
    }
    impl AsRef<[u8; 64]> for Signature {
        fn as_ref(&self) -> &[u8; 64] { &self.0 }
    }
    impl AsRef<[u8]> for Signature {
        fn as_ref(&self) -> &[u8] { &self.0[..] }
    }
    impl AsMut<[u8]> for Signature {
        fn as_mut(&mut self) -> &mut [u8] { &mut self.0[..] }
    }
    #[cfg(feature = "std")]
    impl From<schnorrkel::Signature> for Signature {
        fn from(s: schnorrkel::Signature) -> Signature {
            Signature(s.to_bytes())
        }
    }
    #[cfg(feature = "std")]
    impl ::std::fmt::Debug for Signature {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                      &match (&crate::hexdisplay::HexDisplay::from(&self.0),)
                                                           {
                                                           (arg0,) =>
                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                        ::std::fmt::Display::fmt)],
                                                       }))
        }
    }
    #[cfg(feature = "std")]
    impl ::std::hash::Hash for Signature {
        fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
            ::std::hash::Hash::hash(&self.0[..], state);
        }
    }
    /// A localized signature also contains sender information.
    /// NOTE: Encode and Decode traits are supported in ed25519 but not possible for now here.
    #[cfg(feature = "std")]
    #[structural_match]
    pub struct LocalizedSignature {
        /// The signer of the signature.
        pub signer: Public,
        /// The signature itself.
        pub signature: Signature,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for LocalizedSignature {
        #[inline]
        fn eq(&self, other: &LocalizedSignature) -> bool {
            match *other {
                LocalizedSignature {
                signer: ref __self_1_0, signature: ref __self_1_1 } =>
                match *self {
                    LocalizedSignature {
                    signer: ref __self_0_0, signature: ref __self_0_1 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LocalizedSignature) -> bool {
            match *other {
                LocalizedSignature {
                signer: ref __self_1_0, signature: ref __self_1_1 } =>
                match *self {
                    LocalizedSignature {
                    signer: ref __self_0_0, signature: ref __self_0_1 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for LocalizedSignature {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<Public>;
                let _: ::std::cmp::AssertParamIsEq<Signature>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LocalizedSignature {
        #[inline]
        fn clone(&self) -> LocalizedSignature {
            match *self {
                LocalizedSignature {
                signer: ref __self_0_0, signature: ref __self_0_1 } =>
                LocalizedSignature{signer:
                                       ::std::clone::Clone::clone(&(*__self_0_0)),
                                   signature:
                                       ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for LocalizedSignature {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LocalizedSignature {
                signer: ref __self_0_0, signature: ref __self_0_1 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("LocalizedSignature");
                    let _ =
                        debug_trait_builder.field("signer", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("signature",
                                                  &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl Signature {
        /// A new instance from the given 64-byte `data`.
        ///
        /// NOTE: No checking goes on to ensure this is a real signature. Only use
        /// it if you are certain that the array actually is a signature, or if you
        /// immediately verify the signature.  All functions that verify signatures
        /// will fail if the `Signature` is not actually a valid signature.
        pub fn from_raw(data: [u8; 64]) -> Signature { Signature(data) }
        /// A new instance from the given slice that should be 64 bytes long.
        ///
        /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
        /// you are certain that the array actually is a signature. GIGO!
        pub fn from_slice(data: &[u8]) -> Self {
            let mut r = [0u8; 64];
            r.copy_from_slice(data);
            Signature(r)
        }
        /// A new instance from an H512.
        ///
        /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
        /// you are certain that the array actually is a signature. GIGO!
        pub fn from_h512(v: H512) -> Signature { Signature(v.into()) }
    }
    #[cfg(feature = "std")]
    impl Derive for Public {
        /// Derive a child key from a series of given junctions.
        ///
        /// `None` if there are any hard junctions in there.
        fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, path: Iter)
         -> Option<Public> {
            let mut acc = PublicKey::from_bytes(self.as_ref()).ok()?;
            for j in path {
                match j {
                    DeriveJunction::Soft(cc) =>
                    acc = acc.derived_key_simple(ChainCode(cc), &[]).0,
                    DeriveJunction::Hard(_cc) => return None,
                }
            }
            Some(Self(acc.to_bytes()))
        }
    }
    impl Public {
        /// A new instance from the given 32-byte `data`.
        ///
        /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
        /// you are certain that the array actually is a pubkey. GIGO!
        pub fn from_raw(data: [u8; 32]) -> Self { Public(data) }
        /// A new instance from the given slice that should be 32 bytes long.
        ///
        /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
        /// you are certain that the array actually is a pubkey. GIGO!
        pub fn from_slice(data: &[u8]) -> Self {
            let mut r = [0u8; 32];
            r.copy_from_slice(data);
            Public(r)
        }
        /// A new instance from an H256.
        ///
        /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
        /// you are certain that the array actually is a pubkey. GIGO!
        pub fn from_h256(x: H256) -> Self { Public(x.into()) }
        /// Return a `Vec<u8>` filled with raw data.
        #[cfg(feature = "std")]
        pub fn to_raw_vec(self) -> Vec<u8> {
            let r: &[u8; 32] = self.as_ref();
            r.to_vec()
        }
        /// Return a slice filled with raw data.
        pub fn as_slice(&self) -> &[u8] {
            let r: &[u8; 32] = self.as_ref();
            &r[..]
        }
        /// Return a slice filled with raw data.
        pub fn as_array_ref(&self) -> &[u8; 32] { self.as_ref() }
    }
    #[cfg(feature = "std")]
    impl AsRef<Pair> for Pair {
        fn as_ref(&self) -> &Pair { &self }
    }
    #[cfg(feature = "std")]
    impl From<MiniSecretKey> for Pair {
        fn from(sec: MiniSecretKey) -> Pair { Pair(sec.expand_to_keypair()) }
    }
    #[cfg(feature = "std")]
    impl From<SecretKey> for Pair {
        fn from(sec: SecretKey) -> Pair { Pair(Keypair::from(sec)) }
    }
    #[cfg(feature = "std")]
    impl From<schnorrkel::Keypair> for Pair {
        fn from(p: schnorrkel::Keypair) -> Pair { Pair(p) }
    }
    #[cfg(feature = "std")]
    impl From<Pair> for schnorrkel::Keypair {
        fn from(p: Pair) -> schnorrkel::Keypair { p.0 }
    }
    #[cfg(feature = "std")]
    impl AsRef<schnorrkel::Keypair> for Pair {
        fn as_ref(&self) -> &schnorrkel::Keypair { &self.0 }
    }
    /// Derive a single hard junction.
    #[cfg(feature = "std")]
    fn derive_hard_junction(secret: &SecretKey, cc: &[u8; CHAIN_CODE_LENGTH])
     -> SecretKey {
        secret.hard_derive_mini_secret_key(Some(ChainCode(cc.clone())),
                                           b"").0.expand()
    }
    #[cfg(feature = "std")]
    type Seed = [u8; MINI_SECRET_KEY_LENGTH];
    #[cfg(feature = "std")]
    impl TraitPair for Pair {
        type
        Public
        =
        Public;
        type
        Seed
        =
        Seed;
        type
        Signature
        =
        Signature;
        type
        DeriveError
        =
        Infallible;
        /// Generate new secure (random) key pair.
        fn generate() -> Pair {
            let mut csprng: OsRng =
                OsRng::new().expect("os random generator works; qed");
            let key_pair: Keypair = Keypair::generate(&mut csprng);
            Pair(key_pair)
        }
        /// Make a new key pair from raw secret seed material.
        ///
        /// This is generated using schnorrkel's Mini-Secret-Keys.
        ///
        /// A MiniSecretKey is literally what Ed25519 calls a SecretKey, which is just 32 random bytes.
        fn from_seed(seed: Seed) -> Pair {
            let mini_key: MiniSecretKey =
                MiniSecretKey::from_bytes(&seed[..]).expect("32 bytes can always build a key; qed");
            let kp = mini_key.expand_to_keypair();
            Pair(kp)
        }
        /// Get the public key.
        fn public(&self) -> Public {
            let mut pk = [0u8; 32];
            pk.copy_from_slice(&self.0.public.to_bytes());
            Public(pk)
        }
        /// Make a new key pair from secret seed material. The slice must be 32 bytes long or it
        /// will return `None`.
        ///
        /// You should never need to use this; generate(), generate_with_phrase(), from_phrase()
        fn from_seed_slice(seed: &[u8]) -> Result<Pair, SecretStringError> {
            if seed.len() != MINI_SECRET_KEY_LENGTH {
                Err(SecretStringError::InvalidSeedLength)
            } else {
                Ok(Pair(MiniSecretKey::from_bytes(seed).map_err(|_|
                                                                    SecretStringError::InvalidSeed)?.expand_to_keypair()))
            }
        }
        /// Generate a key from the phrase, password and derivation path.
        fn from_standard_components<I: Iterator<Item =
                                    DeriveJunction>>(phrase: &str,
                                                     password: Option<&str>,
                                                     path: I)
         -> Result<Pair, SecretStringError> {
            Self::from_phrase(phrase,
                              password)?.derive(path).map_err(|_|
                                                                  SecretStringError::InvalidPath)
        }
        fn generate_with_phrase(password: Option<&str>) -> (Pair, String) {
            let mnemonic =
                Mnemonic::new(MnemonicType::Words12, Language::English);
            let phrase = mnemonic.phrase();
            (Self::from_phrase(phrase,
                               password).expect("All phrases generated by Mnemonic are valid; qed"),
             phrase.to_owned())
        }
        fn from_phrase(phrase: &str, password: Option<&str>)
         -> Result<Pair, SecretStringError> {
            Mnemonic::from_phrase(phrase,
                                  Language::English).map_err(|_|
                                                                 SecretStringError::InvalidPhrase).map(|m|
                                                                                                           Self::from_entropy(m.entropy(),
                                                                                                                              password))
        }
        fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, path: Iter)
         -> Result<Pair, Self::DeriveError> {
            let init = self.0.secret.clone();
            let result =
                path.fold(init,
                          |acc, j|
                              match j {
                                  DeriveJunction::Soft(cc) =>
                                  acc.derived_key_simple(ChainCode(cc),
                                                         &[]).0,
                                  DeriveJunction::Hard(cc) =>
                                  derive_hard_junction(&acc, &cc),
                              });
            Ok(Self(result.into()))
        }
        fn sign(&self, message: &[u8]) -> Signature {
            let context = signing_context(SIGNING_CTX);
            self.0.sign(context.bytes(message)).into()
        }
        /// Verify a signature on a message. Returns true if the signature is good.
        fn verify<P: AsRef<Self::Public>,
                  M: AsRef<[u8]>>(sig: &Self::Signature, message: M,
                                  pubkey: P) -> bool {
            let signature: schnorrkel::Signature =
                match schnorrkel::Signature::from_bytes(&sig.as_ref()) {
                    Ok(some_signature) => some_signature,
                    Err(_) => return false,
                };
            match PublicKey::from_bytes(pubkey.as_ref().as_slice()) {
                Ok(pk) =>
                pk.verify(signing_context(SIGNING_CTX).bytes(message.as_ref()),
                          &signature),
                Err(_) => false,
            }
        }
        /// Verify a signature on a message. Returns true if the signature is good.
        fn verify_weak<P: AsRef<[u8]>,
                       M: AsRef<[u8]>>(sig: &[u8], message: M, pubkey: P)
         -> bool {
            let signature: schnorrkel::Signature =
                match schnorrkel::Signature::from_bytes(sig) {
                    Ok(some_signature) => some_signature,
                    Err(_) => return false,
                };
            match PublicKey::from_bytes(pubkey.as_ref()) {
                Ok(pk) =>
                pk.verify(signing_context(SIGNING_CTX).bytes(message.as_ref()),
                          &signature),
                Err(_) => false,
            }
        }
    }
    #[cfg(feature = "std")]
    impl Pair {
        /// Make a new key pair from binary data derived from a valid seed phrase.
        ///
        /// This uses a key derivation function to convert the entropy into a seed, then returns
        /// the pair generated from it.
        pub fn from_entropy(entropy: &[u8], password: Option<&str>) -> Pair {
            let mini_key: MiniSecretKey =
                mini_secret_from_entropy(entropy,
                                         password.unwrap_or("")).expect("32 bytes can always build a key; qed");
            let kp = mini_key.expand_to_keypair();
            Pair(kp)
        }
    }
}
pub mod hash {
    //! A fixed hash type.
    pub use primitive_types::{H160, H256, H512};
    /// Hash conversion. Used to convert between unbound associated hash types in traits,
    /// implemented by the same hash type.
    /// Panics if used to convert between different hash types.
    pub fn convert_hash<H1: Default + AsMut<[u8]>, H2: AsRef<[u8]>>(src: &H2)
     -> H1 {
        let mut dest = H1::default();
        {
            match (&dest.as_mut().len(), &src.as_ref().len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                        "`,\n right: `",
                                                                                        "`"],
                                                                                      &match (&&*left_val,
                                                                                              &&*right_val)
                                                                                           {
                                                                                           (arg0,
                                                                                            arg1)
                                                                                           =>
                                                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                        ::std::fmt::Debug::fmt),
                                                                                            ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                        ::std::fmt::Debug::fmt)],
                                                                                       }),
                                                       &("core/primitives/src/hash.rs",
                                                         26u32, 2u32))
                        }
                    }
                }
            }
        };
        dest.as_mut().copy_from_slice(src.as_ref());
        dest
    }
}
mod hasher {
    //! Substrate Blake2b Hasher implementation
    use hash_db::Hasher;
    use hash256_std_hasher::Hash256StdHasher;
    use crate::hash::H256;
    pub mod blake2 {
        use super::{Hasher, Hash256StdHasher, H256};
        #[cfg(feature = "std")]
        use crate::hashing::blake2_256;
        /// Concrete implementation of Hasher using Blake2b 256-bit hashes
        pub struct Blake2Hasher;
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Blake2Hasher {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Blake2Hasher => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Blake2Hasher");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl Hasher for Blake2Hasher {
            type
            Out
            =
            H256;
            type
            StdHasher
            =
            Hash256StdHasher;
            const
            LENGTH:
            usize
            =
            32;
            fn hash(x: &[u8]) -> Self::Out { blake2_256(x).into() }
        }
    }
}
pub mod sandbox {
    //! Definition of a sandbox environment.
    use parity_codec::{Encode, Decode};
    use rstd::vec::Vec;
    /// Error error that can be returned from host function.
    pub struct HostError;
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_HostError: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for HostError {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    drop(dest);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_HostError: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for HostError {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    drop(input);
                    Some(HostError)
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for HostError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                HostError => {
                    let mut debug_trait_builder = f.debug_tuple("HostError");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Representation of a typed wasm value.
    #[rustc_copy_clone_marker]
    pub enum TypedValue {

        /// Value of 32-bit signed or unsigned integer.
        #[codec(index = "1")]
        I32(i32),

        /// Value of 64-bit signed or unsigned integer.
        #[codec(index = "2")]
        I64(i64),

        /// Value of 32-bit IEEE 754-2008 floating point number represented as a bit pattern.
        #[codec(index = "3")]
        F32(i32),

        /// Value of 64-bit IEEE 754-2008 floating point number represented as a bit pattern.
        #[codec(index = "4")]
        F64(i64),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for TypedValue {
        #[inline]
        fn clone(&self) -> TypedValue {
            {
                let _: ::std::clone::AssertParamIsClone<i32>;
                let _: ::std::clone::AssertParamIsClone<i64>;
                let _: ::std::clone::AssertParamIsClone<i32>;
                let _: ::std::clone::AssertParamIsClone<i64>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for TypedValue { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for TypedValue {
        #[inline]
        fn eq(&self, other: &TypedValue) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TypedValue::I32(ref __self_0),
                         &TypedValue::I32(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&TypedValue::I64(ref __self_0),
                         &TypedValue::I64(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&TypedValue::F32(ref __self_0),
                         &TypedValue::F32(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&TypedValue::F64(ref __self_0),
                         &TypedValue::F64(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &TypedValue) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TypedValue::I32(ref __self_0),
                         &TypedValue::I32(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&TypedValue::I64(ref __self_0),
                         &TypedValue::I64(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&TypedValue::F32(ref __self_0),
                         &TypedValue::F32(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&TypedValue::F64(ref __self_0),
                         &TypedValue::F64(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_TypedValue: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for TypedValue {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        TypedValue::I32(ref aa) => {
                            dest.push_byte(1u8 as u8);
                            dest.push(aa);
                        }
                        TypedValue::I64(ref aa) => {
                            dest.push_byte(2u8 as u8);
                            dest.push(aa);
                        }
                        TypedValue::F32(ref aa) => {
                            dest.push_byte(3u8 as u8);
                            dest.push(aa);
                        }
                        TypedValue::F64(ref aa) => {
                            dest.push_byte(4u8 as u8);
                            dest.push(aa);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_TypedValue: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for TypedValue {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 1u8 as u8 => {
                            Some(TypedValue::I32(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 2u8 as u8 => {
                            Some(TypedValue::I64(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 3u8 as u8 => {
                            Some(TypedValue::F32(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 4u8 as u8 => {
                            Some(TypedValue::F64(_parity_codec::Decode::decode(input)?))
                        }
                        _ => None,
                    }
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for TypedValue {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&TypedValue::I32(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("I32");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&TypedValue::I64(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("I64");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&TypedValue::F32(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("F32");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&TypedValue::F64(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("F64");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl TypedValue {
        /// Returns `Some` if this value of type `I32`.
        pub fn as_i32(&self) -> Option<i32> {
            match *self { TypedValue::I32(v) => Some(v), _ => None, }
        }
    }
    #[cfg(feature = "std")]
    impl From<::wasmi::RuntimeValue> for TypedValue {
        fn from(val: ::wasmi::RuntimeValue) -> TypedValue {
            use ::wasmi::RuntimeValue;
            match val {
                RuntimeValue::I32(v) => TypedValue::I32(v),
                RuntimeValue::I64(v) => TypedValue::I64(v),
                RuntimeValue::F32(v) => TypedValue::F32(v.to_bits() as i32),
                RuntimeValue::F64(v) => TypedValue::F64(v.to_bits() as i64),
            }
        }
    }
    #[cfg(feature = "std")]
    impl From<TypedValue> for ::wasmi::RuntimeValue {
        fn from(val: TypedValue) -> ::wasmi::RuntimeValue {
            use ::wasmi::RuntimeValue;
            use ::wasmi::nan_preserving_float::{F32, F64};
            match val {
                TypedValue::I32(v) => RuntimeValue::I32(v),
                TypedValue::I64(v) => RuntimeValue::I64(v),
                TypedValue::F32(v_bits) =>
                RuntimeValue::F32(F32::from_bits(v_bits as u32)),
                TypedValue::F64(v_bits) =>
                RuntimeValue::F64(F64::from_bits(v_bits as u64)),
            }
        }
    }
    /// Typed value that can be returned from a function.
    ///
    /// Basically a `TypedValue` plus `Unit`, for functions which return nothing.
    #[rustc_copy_clone_marker]
    pub enum ReturnValue {

        /// For returning nothing.
        Unit,

        /// For returning some concrete value.
        Value(TypedValue),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ReturnValue {
        #[inline]
        fn clone(&self) -> ReturnValue {
            { let _: ::std::clone::AssertParamIsClone<TypedValue>; *self }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for ReturnValue { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ReturnValue {
        #[inline]
        fn eq(&self, other: &ReturnValue) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ReturnValue::Value(ref __self_0),
                         &ReturnValue::Value(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => true,
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &ReturnValue) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ReturnValue::Value(ref __self_0),
                         &ReturnValue::Value(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => false,
                    }
                } else { true }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_ReturnValue: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for ReturnValue {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        ReturnValue::Unit => { dest.push_byte(0usize as u8); }
                        ReturnValue::Value(ref aa) => {
                            dest.push_byte(1usize as u8);
                            dest.push(aa);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_ReturnValue: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for ReturnValue {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => { Some(ReturnValue::Unit) }
                        x if x == 1usize as u8 => {
                            Some(ReturnValue::Value(_parity_codec::Decode::decode(input)?))
                        }
                        _ => None,
                    }
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ReturnValue {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&ReturnValue::Unit,) => {
                    let mut debug_trait_builder = f.debug_tuple("Unit");
                    debug_trait_builder.finish()
                }
                (&ReturnValue::Value(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Value");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl From<TypedValue> for ReturnValue {
        fn from(v: TypedValue) -> ReturnValue { ReturnValue::Value(v) }
    }
    impl ReturnValue {
        /// Maximum number of bytes `ReturnValue` might occupy when serialized with
        /// `Codec`.
        ///
        /// Breakdown:
        ///  1 byte for encoding unit/value variant
        ///  1 byte for encoding value type
        ///  8 bytes for encoding the biggest value types available in wasm: f64, i64.
        pub const
        ENCODED_MAX_SIZE:
        usize
        =
        10;
    }
    /// Describes an entity to define or import into the environment.
    #[structural_match]
    pub enum ExternEntity {

        /// Function that is specified by an index in a default table of
        /// a module that creates the sandbox.
        #[codec(index = "1")]
        Function(u32),

        /// Linear memory that is specified by some identifier returned by sandbox
        /// module upon creation new sandboxed memory.
        #[codec(index = "2")]
        Memory(u32),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ExternEntity {
        #[inline]
        fn clone(&self) -> ExternEntity {
            match (&*self,) {
                (&ExternEntity::Function(ref __self_0),) =>
                ExternEntity::Function(::std::clone::Clone::clone(&(*__self_0))),
                (&ExternEntity::Memory(ref __self_0),) =>
                ExternEntity::Memory(::std::clone::Clone::clone(&(*__self_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ExternEntity {
        #[inline]
        fn eq(&self, other: &ExternEntity) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ExternEntity::Function(ref __self_0),
                         &ExternEntity::Function(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&ExternEntity::Memory(ref __self_0),
                         &ExternEntity::Memory(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &ExternEntity) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&ExternEntity::Function(ref __self_0),
                         &ExternEntity::Function(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&ExternEntity::Memory(ref __self_0),
                         &ExternEntity::Memory(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ExternEntity {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<u32>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_ExternEntity: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for ExternEntity {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        ExternEntity::Function(ref aa) => {
                            dest.push_byte(1u8 as u8);
                            dest.push(aa);
                        }
                        ExternEntity::Memory(ref aa) => {
                            dest.push_byte(2u8 as u8);
                            dest.push(aa);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_ExternEntity: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for ExternEntity {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 1u8 as u8 => {
                            Some(ExternEntity::Function(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 2u8 as u8 => {
                            Some(ExternEntity::Memory(_parity_codec::Decode::decode(input)?))
                        }
                        _ => None,
                    }
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ExternEntity {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&ExternEntity::Function(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Function");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ExternEntity::Memory(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Memory");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// An entry in a environment definition table.
    ///
    /// Each entry has a two-level name and description of an entity
    /// being defined.
    #[structural_match]
    pub struct Entry {
        /// Module name of which corresponding entity being defined.
        pub module_name: Vec<u8>,
        /// Field name in which corresponding entity being defined.
        pub field_name: Vec<u8>,
        /// External entity being defined.
        pub entity: ExternEntity,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Entry {
        #[inline]
        fn clone(&self) -> Entry {
            match *self {
                Entry {
                module_name: ref __self_0_0,
                field_name: ref __self_0_1,
                entity: ref __self_0_2 } =>
                Entry{module_name: ::std::clone::Clone::clone(&(*__self_0_0)),
                      field_name: ::std::clone::Clone::clone(&(*__self_0_1)),
                      entity: ::std::clone::Clone::clone(&(*__self_0_2)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Entry {
        #[inline]
        fn eq(&self, other: &Entry) -> bool {
            match *other {
                Entry {
                module_name: ref __self_1_0,
                field_name: ref __self_1_1,
                entity: ref __self_1_2 } =>
                match *self {
                    Entry {
                    module_name: ref __self_0_0,
                    field_name: ref __self_0_1,
                    entity: ref __self_0_2 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Entry) -> bool {
            match *other {
                Entry {
                module_name: ref __self_1_0,
                field_name: ref __self_1_1,
                entity: ref __self_1_2 } =>
                match *self {
                    Entry {
                    module_name: ref __self_0_0,
                    field_name: ref __self_0_1,
                    entity: ref __self_0_2 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for Entry {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
                let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
                let _: ::std::cmp::AssertParamIsEq<ExternEntity>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Entry: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for Entry {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.module_name);
                    dest.push(&self.field_name);
                    dest.push(&self.entity);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Entry: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for Entry {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Entry{module_name:
                                   _parity_codec::Decode::decode(input)?,
                               field_name:
                                   _parity_codec::Decode::decode(input)?,
                               entity:
                                   _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Entry {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Entry {
                module_name: ref __self_0_0,
                field_name: ref __self_0_1,
                entity: ref __self_0_2 } => {
                    let mut debug_trait_builder = f.debug_struct("Entry");
                    let _ =
                        debug_trait_builder.field("module_name",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("field_name",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("entity", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Definition of runtime that could be used by sandboxed code.
    #[structural_match]
    pub struct EnvironmentDefinition {
        /// Vector of all entries in the environment definition.
        pub entries: Vec<Entry>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for EnvironmentDefinition {
        #[inline]
        fn clone(&self) -> EnvironmentDefinition {
            match *self {
                EnvironmentDefinition { entries: ref __self_0_0 } =>
                EnvironmentDefinition{entries:
                                          ::std::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for EnvironmentDefinition {
        #[inline]
        fn eq(&self, other: &EnvironmentDefinition) -> bool {
            match *other {
                EnvironmentDefinition { entries: ref __self_1_0 } =>
                match *self {
                    EnvironmentDefinition { entries: ref __self_0_0 } =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &EnvironmentDefinition) -> bool {
            match *other {
                EnvironmentDefinition { entries: ref __self_1_0 } =>
                match *self {
                    EnvironmentDefinition { entries: ref __self_0_0 } =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for EnvironmentDefinition {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<Vec<Entry>>; }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_EnvironmentDefinition: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for EnvironmentDefinition {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.entries);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_EnvironmentDefinition: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for EnvironmentDefinition {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(EnvironmentDefinition{entries:
                                                   _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for EnvironmentDefinition {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                EnvironmentDefinition { entries: ref __self_0_0 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("EnvironmentDefinition");
                    let _ =
                        debug_trait_builder.field("entries", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Constant for specifying no limit when creating a sandboxed
    /// memory instance. For FFI purposes.
    pub const MEM_UNLIMITED: u32 = -1i32 as u32;
    /// No error happened.
    ///
    /// For FFI purposes.
    pub const ERR_OK: u32 = 0;
    /// Validation or instantiation error occurred when creating new
    /// sandboxed module instance.
    ///
    /// For FFI purposes.
    pub const ERR_MODULE: u32 = -1i32 as u32;
    /// Out-of-bounds access attempted with memory or table.
    ///
    /// For FFI purposes.
    pub const ERR_OUT_OF_BOUNDS: u32 = -2i32 as u32;
    /// Execution error occurred (typically trap).
    ///
    /// For FFI purposes.
    pub const ERR_EXECUTION: u32 = -3i32 as u32;
}
pub mod storage {
    //! Contract execution data.
    #[cfg(feature = "std")]
    use serde::{Serialize, Deserialize};
    #[cfg(feature = "std")]
    use crate::bytes;
    use rstd::vec::Vec;
    /// Contract storage key.
    #[structural_match]
    pub struct StorageKey(
                          #[serde(with = "bytes")]
                          pub Vec<u8>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for StorageKey {
        #[inline]
        fn eq(&self, other: &StorageKey) -> bool {
            match *other {
                StorageKey(ref __self_1_0) =>
                match *self {
                    StorageKey(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &StorageKey) -> bool {
            match *other {
                StorageKey(ref __self_1_0) =>
                match *self {
                    StorageKey(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for StorageKey {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<Vec<u8>>; }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_StorageKey: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl _serde::Serialize for StorageKey {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    _serde::Serializer::serialize_newtype_struct(__serializer,
                                                                 "StorageKey",
                                                                 {
                                                                     struct __SerializeWith<'__a> {
                                                                         values: (&'__a Vec<u8>,),
                                                                         phantom: _serde::export::PhantomData<StorageKey>,
                                                                     }
                                                                     impl <'__a>
                                                                      _serde::Serialize
                                                                      for
                                                                      __SerializeWith<'__a>
                                                                      {
                                                                         fn serialize<__S>(&self,
                                                                                           __s:
                                                                                               __S)
                                                                          ->
                                                                              _serde::export::Result<__S::Ok,
                                                                                                     __S::Error>
                                                                          where
                                                                          __S: _serde::Serializer {
                                                                             bytes::serialize(self.values.0,
                                                                                              __s)
                                                                         }
                                                                     }
                                                                     &__SerializeWith{values:
                                                                                          (&self.0,),
                                                                                      phantom:
                                                                                          _serde::export::PhantomData::<StorageKey>,}
                                                                 })
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_StorageKey: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <'de> _serde::Deserialize<'de> for StorageKey {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<StorageKey>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        StorageKey;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "tuple struct StorageKey")
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(self, __e: __E)
                         -> _serde::export::Result<Self::Value, __E::Error>
                         where __E: _serde::Deserializer<'de> {
                            let __field0: Vec<u8> =
                                match bytes::deserialize(__e) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            _serde::export::Ok(StorageKey(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match {
                                          struct __DeserializeWith<'de> {
                                              value: Vec<u8>,
                                              phantom: _serde::export::PhantomData<StorageKey>,
                                              lifetime: _serde::export::PhantomData<&'de ()>,
                                          }
                                          impl <'de> _serde::Deserialize<'de>
                                           for __DeserializeWith<'de> {
                                              fn deserialize<__D>(__deserializer:
                                                                      __D)
                                               ->
                                                   _serde::export::Result<Self,
                                                                          __D::Error>
                                               where
                                               __D: _serde::Deserializer<'de> {
                                                  _serde::export::Ok(__DeserializeWith{value:
                                                                                           match bytes::deserialize(__deserializer)
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
                                          _serde::export::Option::map(match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(&mut __seq)
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
                                        return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"tuple struct StorageKey with 1 element"));
                                    }
                                };
                            _serde::export::Ok(StorageKey(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                     "StorageKey",
                                                                     __Visitor{marker:
                                                                                   _serde::export::PhantomData::<StorageKey>,
                                                                               lifetime:
                                                                                   _serde::export::PhantomData,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for StorageKey {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                StorageKey(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("StorageKey");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::hash::Hash for StorageKey {
        fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                StorageKey(ref __self_0_0) => {
                    ::std::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialOrd for StorageKey {
        #[inline]
        fn partial_cmp(&self, other: &StorageKey)
         -> ::std::option::Option<::std::cmp::Ordering> {
            match *other {
                StorageKey(ref __self_1_0) =>
                match *self {
                    StorageKey(ref __self_0_0) =>
                    match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                              &(*__self_1_0))
                        {
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                        =>
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &StorageKey) -> bool {
            match *other {
                StorageKey(ref __self_1_0) =>
                match *self {
                    StorageKey(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        == ::std::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &StorageKey) -> bool {
            match *other {
                StorageKey(ref __self_1_0) =>
                match *self {
                    StorageKey(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        != ::std::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &StorageKey) -> bool {
            match *other {
                StorageKey(ref __self_1_0) =>
                match *self {
                    StorageKey(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        == ::std::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &StorageKey) -> bool {
            match *other {
                StorageKey(ref __self_1_0) =>
                match *self {
                    StorageKey(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        != ::std::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Ord for StorageKey {
        #[inline]
        fn cmp(&self, other: &StorageKey) -> ::std::cmp::Ordering {
            match *other {
                StorageKey(ref __self_1_0) =>
                match *self {
                    StorageKey(ref __self_0_0) =>
                    match ::std::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0))
                        {
                        ::std::cmp::Ordering::Equal =>
                        ::std::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for StorageKey {
        #[inline]
        fn clone(&self) -> StorageKey {
            match *self {
                StorageKey(ref __self_0_0) =>
                StorageKey(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    /// Contract storage entry data.
    #[structural_match]
    pub struct StorageData(
                           #[serde(with = "bytes")]
                           pub Vec<u8>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for StorageData {
        #[inline]
        fn eq(&self, other: &StorageData) -> bool {
            match *other {
                StorageData(ref __self_1_0) =>
                match *self {
                    StorageData(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &StorageData) -> bool {
            match *other {
                StorageData(ref __self_1_0) =>
                match *self {
                    StorageData(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for StorageData {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<Vec<u8>>; }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_StorageData: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl _serde::Serialize for StorageData {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    _serde::Serializer::serialize_newtype_struct(__serializer,
                                                                 "StorageData",
                                                                 {
                                                                     struct __SerializeWith<'__a> {
                                                                         values: (&'__a Vec<u8>,),
                                                                         phantom: _serde::export::PhantomData<StorageData>,
                                                                     }
                                                                     impl <'__a>
                                                                      _serde::Serialize
                                                                      for
                                                                      __SerializeWith<'__a>
                                                                      {
                                                                         fn serialize<__S>(&self,
                                                                                           __s:
                                                                                               __S)
                                                                          ->
                                                                              _serde::export::Result<__S::Ok,
                                                                                                     __S::Error>
                                                                          where
                                                                          __S: _serde::Serializer {
                                                                             bytes::serialize(self.values.0,
                                                                                              __s)
                                                                         }
                                                                     }
                                                                     &__SerializeWith{values:
                                                                                          (&self.0,),
                                                                                      phantom:
                                                                                          _serde::export::PhantomData::<StorageData>,}
                                                                 })
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_StorageData: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <'de> _serde::Deserialize<'de> for StorageData {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<StorageData>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        StorageData;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "tuple struct StorageData")
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(self, __e: __E)
                         -> _serde::export::Result<Self::Value, __E::Error>
                         where __E: _serde::Deserializer<'de> {
                            let __field0: Vec<u8> =
                                match bytes::deserialize(__e) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            _serde::export::Ok(StorageData(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match {
                                          struct __DeserializeWith<'de> {
                                              value: Vec<u8>,
                                              phantom: _serde::export::PhantomData<StorageData>,
                                              lifetime: _serde::export::PhantomData<&'de ()>,
                                          }
                                          impl <'de> _serde::Deserialize<'de>
                                           for __DeserializeWith<'de> {
                                              fn deserialize<__D>(__deserializer:
                                                                      __D)
                                               ->
                                                   _serde::export::Result<Self,
                                                                          __D::Error>
                                               where
                                               __D: _serde::Deserializer<'de> {
                                                  _serde::export::Ok(__DeserializeWith{value:
                                                                                           match bytes::deserialize(__deserializer)
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
                                          _serde::export::Option::map(match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(&mut __seq)
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
                                        return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"tuple struct StorageData with 1 element"));
                                    }
                                };
                            _serde::export::Ok(StorageData(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                     "StorageData",
                                                                     __Visitor{marker:
                                                                                   _serde::export::PhantomData::<StorageData>,
                                                                               lifetime:
                                                                                   _serde::export::PhantomData,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for StorageData {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                StorageData(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("StorageData");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::hash::Hash for StorageData {
        fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                StorageData(ref __self_0_0) => {
                    ::std::hash::Hash::hash(&(*__self_0_0), state)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialOrd for StorageData {
        #[inline]
        fn partial_cmp(&self, other: &StorageData)
         -> ::std::option::Option<::std::cmp::Ordering> {
            match *other {
                StorageData(ref __self_1_0) =>
                match *self {
                    StorageData(ref __self_0_0) =>
                    match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                              &(*__self_1_0))
                        {
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                        =>
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                },
            }
        }
        #[inline]
        fn lt(&self, other: &StorageData) -> bool {
            match *other {
                StorageData(ref __self_1_0) =>
                match *self {
                    StorageData(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        == ::std::cmp::Ordering::Less,
                },
            }
        }
        #[inline]
        fn le(&self, other: &StorageData) -> bool {
            match *other {
                StorageData(ref __self_1_0) =>
                match *self {
                    StorageData(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        != ::std::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn gt(&self, other: &StorageData) -> bool {
            match *other {
                StorageData(ref __self_1_0) =>
                match *self {
                    StorageData(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        == ::std::cmp::Ordering::Greater,
                },
            }
        }
        #[inline]
        fn ge(&self, other: &StorageData) -> bool {
            match *other {
                StorageData(ref __self_1_0) =>
                match *self {
                    StorageData(ref __self_0_0) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                         &(*__self_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        != ::std::cmp::Ordering::Less,
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Ord for StorageData {
        #[inline]
        fn cmp(&self, other: &StorageData) -> ::std::cmp::Ordering {
            match *other {
                StorageData(ref __self_1_0) =>
                match *self {
                    StorageData(ref __self_0_0) =>
                    match ::std::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0))
                        {
                        ::std::cmp::Ordering::Equal =>
                        ::std::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for StorageData {
        #[inline]
        fn clone(&self) -> StorageData {
            match *self {
                StorageData(ref __self_0_0) =>
                StorageData(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    /// Storage change set
    #[serde(rename_all = "camelCase")]
    #[structural_match]
    pub struct StorageChangeSet<Hash> {
        /// Block hash
        pub block: Hash,
        /// A list of changes
        pub changes: Vec<(StorageKey, Option<StorageData>)>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_StorageChangeSet: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <Hash> _serde::Serialize for StorageChangeSet<Hash> where
             Hash: _serde::Serialize {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                                                   "StorageChangeSet",
                                                                   false as
                                                                       usize +
                                                                       1 + 1)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "block",
                                                                        &self.block)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "changes",
                                                                        &self.changes)
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
    const _IMPL_DESERIALIZE_FOR_StorageChangeSet: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <'de, Hash> _serde::Deserialize<'de> for
             StorageChangeSet<Hash> where Hash: _serde::Deserialize<'de> {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type
                        Value
                        =
                        __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
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
                                "block" =>
                                _serde::export::Ok(__Field::__field0),
                                "changes" =>
                                _serde::export::Ok(__Field::__field1),
                                _ => { _serde::export::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"block" =>
                                _serde::export::Ok(__Field::__field0),
                                b"changes" =>
                                _serde::export::Ok(__Field::__field1),
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
                    struct __Visitor<'de, Hash> where
                           Hash: _serde::Deserialize<'de> {
                        marker: _serde::export::PhantomData<StorageChangeSet<Hash>>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de, Hash> _serde::de::Visitor<'de> for
                     __Visitor<'de, Hash> where Hash: _serde::Deserialize<'de>
                     {
                        type
                        Value
                        =
                        StorageChangeSet<Hash>;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "struct StorageChangeSet")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<Hash>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct StorageChangeSet with 2 elements"));
                                    }
                                };
                            let __field1 =
                                match match _serde::de::SeqAccess::next_element::<Vec<(StorageKey,
                                                                                       Option<StorageData>)>>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                     &"struct StorageChangeSet with 2 elements"));
                                    }
                                };
                            _serde::export::Ok(StorageChangeSet{block:
                                                                    __field0,
                                                                changes:
                                                                    __field1,})
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::MapAccess<'de> {
                            let mut __field0: _serde::export::Option<Hash> =
                                _serde::export::None;
                            let mut __field1:
                                    _serde::export::Option<Vec<(StorageKey,
                                                                Option<StorageData>)>> =
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
                                                                           _serde::de::Error>::duplicate_field("block"));
                                        }
                                        __field0 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Hash>(&mut __map)
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
                                                                           _serde::de::Error>::duplicate_field("changes"));
                                        }
                                        __field1 =
                                            _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<(StorageKey,
                                                                                                                Option<StorageData>)>>(&mut __map)
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
                                                _serde::export::Err(__err) =>
                                                {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::export::Some(__field0) =>
                                    __field0,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("block")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::export::Some(__field1) =>
                                    __field1,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("changes")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            _serde::export::Ok(StorageChangeSet{block:
                                                                    __field0,
                                                                changes:
                                                                    __field1,})
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["block", "changes"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                                                             "StorageChangeSet",
                                                             FIELDS,
                                                             __Visitor{marker:
                                                                           _serde::export::PhantomData::<StorageChangeSet<Hash>>,
                                                                       lifetime:
                                                                           _serde::export::PhantomData,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::fmt::Debug> ::std::fmt::Debug for
     StorageChangeSet<Hash> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                StorageChangeSet {
                block: ref __self_0_0, changes: ref __self_0_1 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("StorageChangeSet");
                    let _ =
                        debug_trait_builder.field("block", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("changes", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
     StorageChangeSet<Hash> {
        #[inline]
        fn eq(&self, other: &StorageChangeSet<Hash>) -> bool {
            match *other {
                StorageChangeSet {
                block: ref __self_1_0, changes: ref __self_1_1 } =>
                match *self {
                    StorageChangeSet {
                    block: ref __self_0_0, changes: ref __self_0_1 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &StorageChangeSet<Hash>) -> bool {
            match *other {
                StorageChangeSet {
                block: ref __self_1_0, changes: ref __self_1_1 } =>
                match *self {
                    StorageChangeSet {
                    block: ref __self_0_0, changes: ref __self_0_1 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Hash: ::std::cmp::Eq> ::std::cmp::Eq for StorageChangeSet<Hash> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<Hash>;
                let _:
                        ::std::cmp::AssertParamIsEq<Vec<(StorageKey,
                                                         Option<StorageData>)>>;
            }
        }
    }
    /// List of all well known keys and prefixes in storage.
    pub mod well_known_keys {
        /// Wasm code of the runtime.
        ///
        /// Stored as a raw byte vector. Required by substrate.
        pub const CODE: &'static [u8] = b":code";
        /// Number of wasm linear memory pages required for execution of the runtime.
        ///
        /// The type of this value is encoded `u64`.
        pub const HEAP_PAGES: &'static [u8] = b":heappages";
        /// Number of authorities.
        ///
        /// The type of this value is encoded `u32`. Required by substrate.
        pub const AUTHORITY_COUNT: &'static [u8] = b":auth:len";
        /// Prefix under which authorities are storied.
        ///
        /// The full key for N-th authority is generated as:
        ///
        /// `(n as u32).to_keyed_vec(AUTHORITY_PREFIX)`.
        pub const AUTHORITY_PREFIX: &'static [u8] = b":auth:";
        /// Current extrinsic index (u32) is stored under this key.
        pub const EXTRINSIC_INDEX: &'static [u8] = b":extrinsic_index";
        /// Sum of all lengths of executed extrinsics (u32).
        pub const ALL_EXTRINSICS_LEN: &'static [u8] = b":all_extrinsics_len";
        /// Changes trie configuration is stored under this key.
        pub const CHANGES_TRIE_CONFIG: &'static [u8] = b":changes_trie";
        /// Prefix of child storage keys.
        pub const CHILD_STORAGE_KEY_PREFIX: &'static [u8] =
            b":child_storage:";
        /// Whether a key is a child storage key.
        ///
        /// This is convenience function which basically checks if the given `key` starts
        /// with `CHILD_STORAGE_KEY_PREFIX` and doesn't do anything apart from that.
        pub fn is_child_storage_key(key: &[u8]) -> bool {
            key.starts_with(CHILD_STORAGE_KEY_PREFIX)
        }
    }
}
pub mod uint {
    //! An unsigned fixed-size integer.
    pub use primitive_types::U256;
}
mod changes_trie {
    //! Substrate changes trie configuration.
    #[cfg(any(feature = "std", test))]
    use serde::{Serialize, Deserialize};
    use parity_codec::{Encode, Decode};
    /// Substrate changes trie configuration.
    #[structural_match]
    pub struct ChangesTrieConfiguration {
        /// Interval (in blocks) at which level1-digests are created. Digests are not
        /// created when this is less or equal to 1.
        pub digest_interval: u64,
        /// Maximal number of digest levels in hierarchy. 0 means that digests are not
        /// created at all (even level1 digests). 1 means only level1-digests are created.
        /// 2 means that every digest_interval^2 there will be a level2-digest, and so on.
        pub digest_levels: u32,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ChangesTrieConfiguration: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl _serde::Serialize for ChangesTrieConfiguration {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                                                   "ChangesTrieConfiguration",
                                                                   false as
                                                                       usize +
                                                                       1 + 1)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "digest_interval",
                                                                        &self.digest_interval)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "digest_levels",
                                                                        &self.digest_levels)
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
    const _IMPL_DESERIALIZE_FOR_ChangesTrieConfiguration: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                             match $ __expr {
                             _serde :: export :: Ok ( __val ) => __val ,
                             _serde :: export :: Err ( __err ) => {
                             return _serde :: export :: Err ( __err ) ; } }
                             });
            #[automatically_derived]
            impl <'de> _serde::Deserialize<'de> for ChangesTrieConfiguration {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __ignore, }
                    struct __FieldVisitor;
                    impl <'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type
                        Value
                        =
                        __Field;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
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
                                "digest_interval" =>
                                _serde::export::Ok(__Field::__field0),
                                "digest_levels" =>
                                _serde::export::Ok(__Field::__field1),
                                _ => { _serde::export::Ok(__Field::__ignore) }
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8])
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            match __value {
                                b"digest_interval" =>
                                _serde::export::Ok(__Field::__field0),
                                b"digest_levels" =>
                                _serde::export::Ok(__Field::__field1),
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
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<ChangesTrieConfiguration>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        ChangesTrieConfiguration;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "struct ChangesTrieConfiguration")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::SeqAccess<'de> {
                            let __field0 =
                                match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                     &"struct ChangesTrieConfiguration with 2 elements"));
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
                                                                                                     &"struct ChangesTrieConfiguration with 2 elements"));
                                    }
                                };
                            _serde::export::Ok(ChangesTrieConfiguration{digest_interval:
                                                                            __field0,
                                                                        digest_levels:
                                                                            __field1,})
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A)
                         -> _serde::export::Result<Self::Value, __A::Error>
                         where __A: _serde::de::MapAccess<'de> {
                            let mut __field0: _serde::export::Option<u64> =
                                _serde::export::None;
                            let mut __field1: _serde::export::Option<u32> =
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
                                                                           _serde::de::Error>::duplicate_field("digest_interval"));
                                        }
                                        __field0 =
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
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1)
                                           {
                                            return _serde::export::Err(<__A::Error
                                                                           as
                                                                           _serde::de::Error>::duplicate_field("digest_levels"));
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
                                    _ => {
                                        let _ =
                                            match _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)
                                                {
                                                _serde::export::Ok(__val) =>
                                                __val,
                                                _serde::export::Err(__err) =>
                                                {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                    }
                                }
                            }
                            let __field0 =
                                match __field0 {
                                    _serde::export::Some(__field0) =>
                                    __field0,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("digest_interval")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            let __field1 =
                                match __field1 {
                                    _serde::export::Some(__field1) =>
                                    __field1,
                                    _serde::export::None =>
                                    match _serde::private::de::missing_field("digest_levels")
                                        {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                };
                            _serde::export::Ok(ChangesTrieConfiguration{digest_interval:
                                                                            __field0,
                                                                        digest_levels:
                                                                            __field1,})
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["digest_interval", "digest_levels"];
                    _serde::Deserializer::deserialize_struct(__deserializer,
                                                             "ChangesTrieConfiguration",
                                                             FIELDS,
                                                             __Visitor{marker:
                                                                           _serde::export::PhantomData::<ChangesTrieConfiguration>,
                                                                       lifetime:
                                                                           _serde::export::PhantomData,})
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ChangesTrieConfiguration {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ChangesTrieConfiguration {
                digest_interval: ref __self_0_0, digest_levels: ref __self_0_1
                } => {
                    let mut debug_trait_builder =
                        f.debug_struct("ChangesTrieConfiguration");
                    let _ =
                        debug_trait_builder.field("digest_interval",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("digest_levels",
                                                  &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ChangesTrieConfiguration {
        #[inline]
        fn clone(&self) -> ChangesTrieConfiguration {
            match *self {
                ChangesTrieConfiguration {
                digest_interval: ref __self_0_0, digest_levels: ref __self_0_1
                } =>
                ChangesTrieConfiguration{digest_interval:
                                             ::std::clone::Clone::clone(&(*__self_0_0)),
                                         digest_levels:
                                             ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for ChangesTrieConfiguration {
        #[inline]
        fn eq(&self, other: &ChangesTrieConfiguration) -> bool {
            match *other {
                ChangesTrieConfiguration {
                digest_interval: ref __self_1_0, digest_levels: ref __self_1_1
                } =>
                match *self {
                    ChangesTrieConfiguration {
                    digest_interval: ref __self_0_0,
                    digest_levels: ref __self_0_1 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ChangesTrieConfiguration) -> bool {
            match *other {
                ChangesTrieConfiguration {
                digest_interval: ref __self_1_0, digest_levels: ref __self_1_1
                } =>
                match *self {
                    ChangesTrieConfiguration {
                    digest_interval: ref __self_0_0,
                    digest_levels: ref __self_0_1 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for ChangesTrieConfiguration {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<u64>;
                let _: ::std::cmp::AssertParamIsEq<u32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::default::Default for ChangesTrieConfiguration {
        #[inline]
        fn default() -> ChangesTrieConfiguration {
            ChangesTrieConfiguration{digest_interval:
                                         ::std::default::Default::default(),
                                     digest_levels:
                                         ::std::default::Default::default(),}
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_ChangesTrieConfiguration: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for ChangesTrieConfiguration {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.digest_interval);
                    dest.push(&self.digest_levels);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_ChangesTrieConfiguration: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for ChangesTrieConfiguration {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(ChangesTrieConfiguration{digest_interval:
                                                      _parity_codec::Decode::decode(input)?,
                                                  digest_levels:
                                                      _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    impl ChangesTrieConfiguration {
        /// Is digest build enabled?
        pub fn is_digest_build_enabled(&self) -> bool {
            self.digest_interval > 1 && self.digest_levels > 0
        }
        /// Do we need to build digest at given block?
        pub fn is_digest_build_required_at_block(&self, block: u64) -> bool {
            block != 0 && self.is_digest_build_enabled() &&
                block % self.digest_interval == 0
        }
        /// Returns max digest interval. One if digests are not created at all.
        /// Returns ::std::u64::MAX instead of panic in the case of overflow.
        pub fn max_digest_interval(&self) -> u64 {
            if !self.is_digest_build_enabled() { return 1; }
            self.digest_interval.saturating_pow(self.digest_levels)
        }
        /// Returns Some if digest must be built at given block number.
        /// The tuple is:
        /// (
        ///  digest level
        ///  digest interval (in blocks)
        ///  step between blocks we're interested in when digest is built
        /// )
        pub fn digest_level_at_block(&self, block: u64)
         -> Option<(u32, u64, u64)> {
            if !self.is_digest_build_required_at_block(block) { return None; }
            let mut digest_interval = self.digest_interval;
            let mut current_level = 1u32;
            let mut digest_step = 1u64;
            while current_level < self.digest_levels {
                let new_digest_interval =
                    match digest_interval.checked_mul(self.digest_interval) {
                        Some(new_digest_interval) if
                        block % new_digest_interval == 0 =>
                        new_digest_interval,
                        _ => break ,
                    };
                digest_step = digest_interval;
                digest_interval = new_digest_interval;
                current_level = current_level + 1;
            }
            Some((current_level, digest_interval, digest_step))
        }
    }
}
pub use self::hash::{H160, H256, H512, convert_hash};
pub use self::uint::U256;
pub use changes_trie::ChangesTrieConfiguration;
#[cfg(feature = "std")]
pub use crypto::{DeriveJunction, Pair};
pub use hash_db::Hasher;
pub use self::hasher::blake2::Blake2Hasher;
/// Context for executing a call into the runtime.
#[repr(u8)]
pub enum ExecutionContext {

    /// Context for general importing (including own blocks).
    Importing,

    /// Context used when syncing the blockchain.
    Syncing,

    /// Context used for block construction.
    BlockConstruction,

    /// Offchain worker context.
    OffchainWorker(Box<OffchainExt>),

    /// Context used for other calls.
    Other,
}
/// An extended externalities for offchain workers.
pub trait OffchainExt {
    /// Submits an extrinsics.
    ///
    /// The extrinsic will either go to the pool (signed)
    /// or to the next produced block (inherent).
    fn submit_extrinsic(&mut self, extrinsic: Vec<u8>);
}
impl <T: OffchainExt + ?Sized> OffchainExt for Box<T> {
    fn submit_extrinsic(&mut self, ex: Vec<u8>) {
        (&mut **self).submit_extrinsic(ex)
    }
}
/// Hex-serialized shim for `Vec<u8>`.
#[structural_match]
pub struct Bytes(
                 #[serde(with = "bytes")]
                 pub Vec<u8>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Bytes {
    #[inline]
    fn eq(&self, other: &Bytes) -> bool {
        match *other {
            Bytes(ref __self_1_0) =>
            match *self {
                Bytes(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Bytes) -> bool {
        match *other {
            Bytes(ref __self_1_0) =>
            match *self {
                Bytes(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Bytes {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<Vec<u8>>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Bytes {
    #[inline]
    fn clone(&self) -> Bytes {
        match *self {
            Bytes(ref __self_0_0) =>
            Bytes(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Bytes: () =
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
        impl _serde::Serialize for Bytes {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "Bytes",
                                                             {
                                                                 struct __SerializeWith<'__a> {
                                                                     values: (&'__a Vec<u8>,),
                                                                     phantom: _serde::export::PhantomData<Bytes>,
                                                                 }
                                                                 impl <'__a>
                                                                  _serde::Serialize
                                                                  for
                                                                  __SerializeWith<'__a>
                                                                  {
                                                                     fn serialize<__S>(&self,
                                                                                       __s:
                                                                                           __S)
                                                                      ->
                                                                          _serde::export::Result<__S::Ok,
                                                                                                 __S::Error>
                                                                      where
                                                                      __S: _serde::Serializer {
                                                                         bytes::serialize(self.values.0,
                                                                                          __s)
                                                                     }
                                                                 }
                                                                 &__SerializeWith{values:
                                                                                      (&self.0,),
                                                                                  phantom:
                                                                                      _serde::export::PhantomData::<Bytes>,}
                                                             })
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Bytes: () =
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
        impl <'de> _serde::Deserialize<'de> for Bytes {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Bytes>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    Bytes;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct Bytes")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: Vec<u8> =
                            match bytes::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(Bytes(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match {
                                      struct __DeserializeWith<'de> {
                                          value: Vec<u8>,
                                          phantom: _serde::export::PhantomData<Bytes>,
                                          lifetime: _serde::export::PhantomData<&'de ()>,
                                      }
                                      impl <'de> _serde::Deserialize<'de> for
                                       __DeserializeWith<'de> {
                                          fn deserialize<__D>(__deserializer:
                                                                  __D)
                                           ->
                                               _serde::export::Result<Self,
                                                                      __D::Error>
                                           where
                                           __D: _serde::Deserializer<'de> {
                                              _serde::export::Ok(__DeserializeWith{value:
                                                                                       match bytes::deserialize(__deserializer)
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
                                      _serde::export::Option::map(match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(&mut __seq)
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
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"tuple struct Bytes with 1 element"));
                                }
                            };
                        _serde::export::Ok(Bytes(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "Bytes",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<Bytes>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Bytes(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Bytes");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::hash::Hash for Bytes {
    fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            Bytes(ref __self_0_0) => {
                ::std::hash::Hash::hash(&(*__self_0_0), state)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialOrd for Bytes {
    #[inline]
    fn partial_cmp(&self, other: &Bytes)
     -> ::std::option::Option<::std::cmp::Ordering> {
        match *other {
            Bytes(ref __self_1_0) =>
            match *self {
                Bytes(ref __self_0_0) =>
                match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                          &(*__self_1_0)) {
                    ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                    =>
                    ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &Bytes) -> bool {
        match *other {
            Bytes(ref __self_1_0) =>
            match *self {
                Bytes(ref __self_0_0) =>
                ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                     &(*__self_1_0)),
                                                 ::std::cmp::Ordering::Greater)
                    == ::std::cmp::Ordering::Less,
            },
        }
    }
    #[inline]
    fn le(&self, other: &Bytes) -> bool {
        match *other {
            Bytes(ref __self_1_0) =>
            match *self {
                Bytes(ref __self_0_0) =>
                ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                     &(*__self_1_0)),
                                                 ::std::cmp::Ordering::Greater)
                    != ::std::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn gt(&self, other: &Bytes) -> bool {
        match *other {
            Bytes(ref __self_1_0) =>
            match *self {
                Bytes(ref __self_0_0) =>
                ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                     &(*__self_1_0)),
                                                 ::std::cmp::Ordering::Less)
                    == ::std::cmp::Ordering::Greater,
            },
        }
    }
    #[inline]
    fn ge(&self, other: &Bytes) -> bool {
        match *other {
            Bytes(ref __self_1_0) =>
            match *self {
                Bytes(ref __self_0_0) =>
                ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                     &(*__self_1_0)),
                                                 ::std::cmp::Ordering::Less)
                    != ::std::cmp::Ordering::Less,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Ord for Bytes {
    #[inline]
    fn cmp(&self, other: &Bytes) -> ::std::cmp::Ordering {
        match *other {
            Bytes(ref __self_1_0) =>
            match *self {
                Bytes(ref __self_0_0) =>
                match ::std::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::std::cmp::Ordering::Equal =>
                    ::std::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
impl From<Vec<u8>> for Bytes {
    fn from(s: Vec<u8>) -> Self { Bytes(s) }
}
impl From<OpaqueMetadata> for Bytes {
    fn from(s: OpaqueMetadata) -> Self { Bytes(s.0) }
}
impl Deref for Bytes {
    type
    Target
    =
    [u8];
    fn deref(&self) -> &[u8] { &self.0[..] }
}
/// Stores the encoded `RuntimeMetadata` for the native side as opaque type.
pub struct OpaqueMetadata(Vec<u8>);
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_OpaqueMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for OpaqueMetadata {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_OpaqueMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for OpaqueMetadata {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(OpaqueMetadata(_parity_codec::Decode::decode(input)?))
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for OpaqueMetadata {
    #[inline]
    fn eq(&self, other: &OpaqueMetadata) -> bool {
        match *other {
            OpaqueMetadata(ref __self_1_0) =>
            match *self {
                OpaqueMetadata(ref __self_0_0) =>
                (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &OpaqueMetadata) -> bool {
        match *other {
            OpaqueMetadata(ref __self_1_0) =>
            match *self {
                OpaqueMetadata(ref __self_0_0) =>
                (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl OpaqueMetadata {
    /// Creates a new instance with the given metadata blob.
    pub fn new(metadata: Vec<u8>) -> Self { OpaqueMetadata(metadata) }
}
impl rstd::ops::Deref for OpaqueMetadata {
    type
    Target
    =
    Vec<u8>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
/// Something that is either a native or an encoded value.
#[cfg(feature = "std")]
pub enum NativeOrEncoded<R> {

    /// The native representation.
    Native(R),

    /// The encoded representation.
    Encoded(Vec<u8>),
}
#[cfg(feature = "std")]
impl <R: parity_codec::Encode> ::std::fmt::Debug for NativeOrEncoded<R> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.as_encoded().as_ref().fmt(f)
    }
}
#[cfg(feature = "std")]
impl <R: parity_codec::Encode> NativeOrEncoded<R> {
    /// Return the value as the encoded format.
    pub fn as_encoded<'a>(&'a self) -> Cow<'a, [u8]> {
        match self {
            NativeOrEncoded::Encoded(e) => Cow::Borrowed(e.as_slice()),
            NativeOrEncoded::Native(n) => Cow::Owned(n.encode()),
        }
    }
    /// Return the value as the encoded format.
    pub fn into_encoded(self) -> Vec<u8> {
        match self {
            NativeOrEncoded::Encoded(e) => e,
            NativeOrEncoded::Native(n) => n.encode(),
        }
    }
}
#[cfg(feature = "std")]
impl <R: PartialEq + parity_codec::Decode> PartialEq for NativeOrEncoded<R> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NativeOrEncoded::Native(l), NativeOrEncoded::Native(r)) =>
            l == r,
            (NativeOrEncoded::Native(n), NativeOrEncoded::Encoded(e)) |
            (NativeOrEncoded::Encoded(e), NativeOrEncoded::Native(n)) =>
            Some(n) == parity_codec::Decode::decode(&mut &e[..]).as_ref(),
            (NativeOrEncoded::Encoded(l), NativeOrEncoded::Encoded(r)) =>
            l == r,
        }
    }
}
/// A value that is never in a native representation.
/// This is type is useful in conjuction with `NativeOrEncoded`.
#[cfg(feature = "std")]
pub enum NeverNativeValue { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for NeverNativeValue {
    #[inline]
    fn eq(&self, other: &NeverNativeValue) -> bool {
        unsafe { ::std::intrinsics::unreachable() }
    }
}
#[cfg(feature = "std")]
impl parity_codec::Encode for NeverNativeValue {
    fn encode(&self) -> Vec<u8> {
        {
            {
                ::std::rt::begin_panic("internal error: entered unreachable code",
                                       &("core/primitives/src/lib.rs", 202u32,
                                         3u32))
            }
        }
    }
}
#[cfg(feature = "std")]
impl parity_codec::Decode for NeverNativeValue {
    fn decode<I: parity_codec::Input>(_: &mut I) -> Option<Self> { None }
}
