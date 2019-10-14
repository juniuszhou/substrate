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

//! Runtime Modules shared primitive types.

#![warn(missing_docs)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[doc(hidden)]
pub use parity_codec as codec;
#[cfg(feature = "std")]
#[doc(hidden)]
pub use serde;

#[cfg(feature = "std")]
pub use runtime_io::{StorageOverlay, ChildrenStorageOverlay};

use rstd::prelude::*;
use rstd::ops;
use substrate_primitives::{crypto, ed25519, sr25519, hash::{H256, H512}};
use codec::{Encode, Decode};

#[cfg(feature = "std")]
pub mod testing {























    // `rem` is inferior to one million, thus it fits into u64

    // `self` and `rem` are inferior to one million, thus the product fits into u64


    // `rem_multiplied_divided` is inferior to b, thus it can be converted back to N type
















    // `rem` is inferior to one billion, thus it fits into u64

    // `self` and `rem` are inferior to one billion, thus the product fits into u64


    // `rem_multiplied_divided` is inferior to b, thus it can be converted back to N type






































    // The `Lazy<T>` trait expresses something like `X: FnMut<Output = for<'a> &'a T>`.
    // unfortunately this is a lifetime relationship that can't
    // be expressed without generic associated types, better unification of HRTBs in type position,
    // and some kind of integration into the Fn* traits.





























    // all sort of from_percent

    // bounds

    // encode/decode regular item

    // encode/decode system item

    // interpret regular item using `generic::DigestItem`

    // interpret system item using `generic::DigestItem`

    // check that as-style methods are working with system items

    // check that as-style methods are not working with regular items











    // panics



    //! Testing utilities.
    use serde::{Serialize, Serializer, Deserialize, de::Error as DeError,
                Deserializer};
    use std::{fmt::Debug, ops::Deref, fmt};
    use crate::codec::{Codec, Encode, Decode};
    use crate::traits::{self, Checkable, Applyable, BlakeTwo256, Convert};
    use crate::generic::DigestItem as GenDigestItem;
    pub use substrate_primitives::H256;
    use substrate_primitives::U256;
    use substrate_primitives::sr25519::{Public as AuthorityId, Signature as
                                        AuthoritySignature};
    /// Authority Id
    #[structural_match]
    pub struct UintAuthorityId(pub u64);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::default::Default for UintAuthorityId {
        #[inline]
        fn default() -> UintAuthorityId {
            UintAuthorityId(::std::default::Default::default())
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for UintAuthorityId {
        #[inline]
        fn eq(&self, other: &UintAuthorityId) -> bool {
            match *other {
                UintAuthorityId(ref __self_1_0) =>
                match *self {
                    UintAuthorityId(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UintAuthorityId) -> bool {
            match *other {
                UintAuthorityId(ref __self_1_0) =>
                match *self {
                    UintAuthorityId(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for UintAuthorityId {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<u64>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for UintAuthorityId {
        #[inline]
        fn clone(&self) -> UintAuthorityId {
            match *self {
                UintAuthorityId(ref __self_0_0) =>
                UintAuthorityId(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_UintAuthorityId: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for UintAuthorityId {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.0);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_UintAuthorityId: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for UintAuthorityId {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(UintAuthorityId(_parity_codec::Decode::decode(input)?))
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UintAuthorityId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UintAuthorityId(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UintAuthorityId");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UintAuthorityId: () =
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
            impl _serde::Serialize for UintAuthorityId {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    _serde::Serializer::serialize_newtype_struct(__serializer,
                                                                 "UintAuthorityId",
                                                                 &self.0)
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UintAuthorityId: () =
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
            impl <'de> _serde::Deserialize<'de> for UintAuthorityId {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<UintAuthorityId>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type
                        Value
                        =
                        UintAuthorityId;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "tuple struct UintAuthorityId")
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(self, __e: __E)
                         -> _serde::export::Result<Self::Value, __E::Error>
                         where __E: _serde::Deserializer<'de> {
                            let __field0: u64 =
                                match <u64 as
                                          _serde::Deserialize>::deserialize(__e)
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            _serde::export::Ok(UintAuthorityId(__field0))
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
                                                                                                     &"tuple struct UintAuthorityId with 1 element"));
                                    }
                                };
                            _serde::export::Ok(UintAuthorityId(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                     "UintAuthorityId",
                                                                     __Visitor{marker:
                                                                                   _serde::export::PhantomData::<UintAuthorityId>,
                                                                               lifetime:
                                                                                   _serde::export::PhantomData,})
                }
            }
        };
    impl Into<AuthorityId> for UintAuthorityId {
        fn into(self) -> AuthorityId {
            let bytes: [u8; 32] = U256::from(self.0).into();
            AuthorityId(bytes)
        }
    }
    /// Converter between u64 and the AuthorityId wrapper type.
    pub struct ConvertUintAuthorityId;
    impl Convert<u64, Option<UintAuthorityId>> for ConvertUintAuthorityId {
        fn convert(a: u64) -> Option<UintAuthorityId> {
            Some(UintAuthorityId(a))
        }
    }
    /// Digest item
    pub type DigestItem
        =
        GenDigestItem<H256, AuthorityId, AuthoritySignature>;
    /// Header Digest
    #[structural_match]
    pub struct Digest {
        /// Generated logs
        pub logs: Vec<DigestItem>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::default::Default for Digest {
        #[inline]
        fn default() -> Digest {
            Digest{logs: ::std::default::Default::default(),}
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Digest {
        #[inline]
        fn eq(&self, other: &Digest) -> bool {
            match *other {
                Digest { logs: ref __self_1_0 } =>
                match *self {
                    Digest { logs: ref __self_0_0 } =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Digest) -> bool {
            match *other {
                Digest { logs: ref __self_1_0 } =>
                match *self {
                    Digest { logs: ref __self_0_0 } =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for Digest {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<Vec<DigestItem>>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Digest {
        #[inline]
        fn clone(&self) -> Digest {
            match *self {
                Digest { logs: ref __self_0_0 } =>
                Digest{logs: ::std::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Digest: () =
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
            impl _serde::Serialize for Digest {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                                                   "Digest",
                                                                   false as
                                                                       usize +
                                                                       1) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "logs",
                                                                        &self.logs)
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
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Digest {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Digest { logs: ref __self_0_0 } => {
                    let mut debug_trait_builder = f.debug_struct("Digest");
                    let _ =
                        debug_trait_builder.field("logs", &&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Digest: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for Digest {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.logs);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Digest: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for Digest {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Digest{logs: _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    impl traits::Digest for Digest {
        type
        Hash
        =
        H256;
        type
        Item
        =
        DigestItem;
        fn logs(&self) -> &[Self::Item] { &self.logs }
        fn push(&mut self, item: Self::Item) { self.logs.push(item); }
        fn pop(&mut self) -> Option<Self::Item> { self.logs.pop() }
    }
    /// Block Header
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[structural_match]
    pub struct Header {
        /// Parent hash
        pub parent_hash: H256,
        /// Block Number
        pub number: u64,
        /// Post-execution state trie root
        pub state_root: H256,
        /// Merkle root of block's extrinsics
        pub extrinsics_root: H256,
        /// Digest items
        pub digest: Digest,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for Header {
        #[inline]
        fn eq(&self, other: &Header) -> bool {
            match *other {
                Header {
                parent_hash: ref __self_1_0,
                number: ref __self_1_1,
                state_root: ref __self_1_2,
                extrinsics_root: ref __self_1_3,
                digest: ref __self_1_4 } =>
                match *self {
                    Header {
                    parent_hash: ref __self_0_0,
                    number: ref __self_0_1,
                    state_root: ref __self_0_2,
                    extrinsics_root: ref __self_0_3,
                    digest: ref __self_0_4 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2) &&
                        (*__self_0_3) == (*__self_1_3) &&
                        (*__self_0_4) == (*__self_1_4),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Header) -> bool {
            match *other {
                Header {
                parent_hash: ref __self_1_0,
                number: ref __self_1_1,
                state_root: ref __self_1_2,
                extrinsics_root: ref __self_1_3,
                digest: ref __self_1_4 } =>
                match *self {
                    Header {
                    parent_hash: ref __self_0_0,
                    number: ref __self_0_1,
                    state_root: ref __self_0_2,
                    extrinsics_root: ref __self_0_3,
                    digest: ref __self_0_4 } =>
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
    impl ::std::cmp::Eq for Header {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<H256>;
                let _: ::std::cmp::AssertParamIsEq<u64>;
                let _: ::std::cmp::AssertParamIsEq<H256>;
                let _: ::std::cmp::AssertParamIsEq<H256>;
                let _: ::std::cmp::AssertParamIsEq<Digest>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Header {
        #[inline]
        fn clone(&self) -> Header {
            match *self {
                Header {
                parent_hash: ref __self_0_0,
                number: ref __self_0_1,
                state_root: ref __self_0_2,
                extrinsics_root: ref __self_0_3,
                digest: ref __self_0_4 } =>
                Header{parent_hash:
                           ::std::clone::Clone::clone(&(*__self_0_0)),
                       number: ::std::clone::Clone::clone(&(*__self_0_1)),
                       state_root: ::std::clone::Clone::clone(&(*__self_0_2)),
                       extrinsics_root:
                           ::std::clone::Clone::clone(&(*__self_0_3)),
                       digest: ::std::clone::Clone::clone(&(*__self_0_4)),},
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Header: () =
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
            impl _serde::Serialize for Header {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                                                   "Header",
                                                                   false as
                                                                       usize +
                                                                       1 + 1 +
                                                                       1 + 1 +
                                                                       1) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "parentHash",
                                                                        &self.parent_hash)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "number",
                                                                        &self.number)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "stateRoot",
                                                                        &self.state_root)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "extrinsicsRoot",
                                                                        &self.extrinsics_root)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "digest",
                                                                        &self.digest)
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
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Header {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Header {
                parent_hash: ref __self_0_0,
                number: ref __self_0_1,
                state_root: ref __self_0_2,
                extrinsics_root: ref __self_0_3,
                digest: ref __self_0_4 } => {
                    let mut debug_trait_builder = f.debug_struct("Header");
                    let _ =
                        debug_trait_builder.field("parent_hash",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("number", &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("state_root",
                                                  &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("extrinsics_root",
                                                  &&(*__self_0_3));
                    let _ =
                        debug_trait_builder.field("digest", &&(*__self_0_4));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Header: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for Header {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.parent_hash);
                    dest.push(&self.number);
                    dest.push(&self.state_root);
                    dest.push(&self.extrinsics_root);
                    dest.push(&self.digest);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Header: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for Header {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Header{parent_hash:
                                    _parity_codec::Decode::decode(input)?,
                                number: _parity_codec::Decode::decode(input)?,
                                state_root:
                                    _parity_codec::Decode::decode(input)?,
                                extrinsics_root:
                                    _parity_codec::Decode::decode(input)?,
                                digest:
                                    _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    impl traits::Header for Header {
        type
        Number
        =
        u64;
        type
        Hashing
        =
        BlakeTwo256;
        type
        Hash
        =
        H256;
        type
        Digest
        =
        Digest;
        fn number(&self) -> &Self::Number { &self.number }
        fn set_number(&mut self, num: Self::Number) { self.number = num }
        fn extrinsics_root(&self) -> &Self::Hash { &self.extrinsics_root }
        fn set_extrinsics_root(&mut self, root: Self::Hash) {
            self.extrinsics_root = root
        }
        fn state_root(&self) -> &Self::Hash { &self.state_root }
        fn set_state_root(&mut self, root: Self::Hash) {
            self.state_root = root
        }
        fn parent_hash(&self) -> &Self::Hash { &self.parent_hash }
        fn set_parent_hash(&mut self, hash: Self::Hash) {
            self.parent_hash = hash
        }
        fn digest(&self) -> &Self::Digest { &self.digest }
        fn digest_mut(&mut self) -> &mut Self::Digest { &mut self.digest }
        fn set_digest(&mut self, digest: Self::Digest) {
            self.digest = digest
        }
        fn new(number: Self::Number, extrinsics_root: Self::Hash,
               state_root: Self::Hash, parent_hash: Self::Hash,
               digest: Self::Digest) -> Self {
            Header{number,
                   extrinsics_root: extrinsics_root,
                   state_root,
                   parent_hash,
                   digest,}
        }
    }
    impl <'a> Deserialize<'a> for Header {
        fn deserialize<D: Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
            let r = <Vec<u8>>::deserialize(de)?;
            Decode::decode(&mut &r[..]).ok_or(DeError::custom("Invalid value passed into decode"))
        }
    }
    /// An opaque extrinsic wrapper type.
    #[structural_match]
    pub struct ExtrinsicWrapper<Xt>(Xt);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Xt: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
     ExtrinsicWrapper<Xt> {
        #[inline]
        fn eq(&self, other: &ExtrinsicWrapper<Xt>) -> bool {
            match *other {
                ExtrinsicWrapper(ref __self_1_0) =>
                match *self {
                    ExtrinsicWrapper(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &ExtrinsicWrapper<Xt>) -> bool {
            match *other {
                ExtrinsicWrapper(ref __self_1_0) =>
                match *self {
                    ExtrinsicWrapper(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Xt: ::std::cmp::Eq> ::std::cmp::Eq for ExtrinsicWrapper<Xt> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            { let _: ::std::cmp::AssertParamIsEq<Xt>; }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Xt: ::std::clone::Clone> ::std::clone::Clone for
     ExtrinsicWrapper<Xt> {
        #[inline]
        fn clone(&self) -> ExtrinsicWrapper<Xt> {
            match *self {
                ExtrinsicWrapper(ref __self_0_0) =>
                ExtrinsicWrapper(::std::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Xt: ::std::fmt::Debug> ::std::fmt::Debug for ExtrinsicWrapper<Xt> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ExtrinsicWrapper(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ExtrinsicWrapper");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_ExtrinsicWrapper: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Xt> _parity_codec::Encode for ExtrinsicWrapper<Xt> where
             Xt: _parity_codec::Encode, Xt: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.0);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_ExtrinsicWrapper: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Xt> _parity_codec::Decode for ExtrinsicWrapper<Xt> where
             Xt: _parity_codec::Decode, Xt: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(ExtrinsicWrapper(_parity_codec::Decode::decode(input)?))
                }
            }
        };
    impl <Xt> traits::Extrinsic for ExtrinsicWrapper<Xt> {
        fn is_signed(&self) -> Option<bool> { None }
    }
    impl <Xt: Encode> serde::Serialize for ExtrinsicWrapper<Xt> {
        fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error> where
         S: ::serde::Serializer {
            self.using_encoded(|bytes| seq.serialize_bytes(bytes))
        }
    }
    impl <Xt> From<Xt> for ExtrinsicWrapper<Xt> {
        fn from(xt: Xt) -> Self { ExtrinsicWrapper(xt) }
    }
    impl <Xt> Deref for ExtrinsicWrapper<Xt> {
        type
        Target
        =
        Xt;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    /// Testing block
    #[structural_match]
    pub struct Block<Xt> {
        /// Block header
        pub header: Header,
        /// List of extrinsics
        pub extrinsics: Vec<Xt>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Xt: ::std::cmp::PartialEq> ::std::cmp::PartialEq for Block<Xt> {
        #[inline]
        fn eq(&self, other: &Block<Xt>) -> bool {
            match *other {
                Block { header: ref __self_1_0, extrinsics: ref __self_1_1 }
                =>
                match *self {
                    Block { header: ref __self_0_0, extrinsics: ref __self_0_1
                    } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Block<Xt>) -> bool {
            match *other {
                Block { header: ref __self_1_0, extrinsics: ref __self_1_1 }
                =>
                match *self {
                    Block { header: ref __self_0_0, extrinsics: ref __self_0_1
                    } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Xt: ::std::cmp::Eq> ::std::cmp::Eq for Block<Xt> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<Header>;
                let _: ::std::cmp::AssertParamIsEq<Vec<Xt>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Xt: ::std::clone::Clone> ::std::clone::Clone for Block<Xt> {
        #[inline]
        fn clone(&self) -> Block<Xt> {
            match *self {
                Block { header: ref __self_0_0, extrinsics: ref __self_0_1 }
                =>
                Block{header: ::std::clone::Clone::clone(&(*__self_0_0)),
                      extrinsics:
                          ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_Block: () =
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
            impl <Xt> _serde::Serialize for Block<Xt> where
             Xt: _serde::Serialize {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_struct(__serializer,
                                                                   "Block",
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
                                                                        "header",
                                                                        &self.header)
                        {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                        "extrinsics",
                                                                        &self.extrinsics)
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
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Xt: ::std::fmt::Debug> ::std::fmt::Debug for Block<Xt> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Block { header: ref __self_0_0, extrinsics: ref __self_0_1 }
                => {
                    let mut debug_trait_builder = f.debug_struct("Block");
                    let _ =
                        debug_trait_builder.field("header", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("extrinsics",
                                                  &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Block: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Xt> _parity_codec::Encode for Block<Xt> where
             Vec<Xt>: _parity_codec::Encode, Vec<Xt>: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.header);
                    dest.push(&self.extrinsics);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Block: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Xt> _parity_codec::Decode for Block<Xt> where
             Vec<Xt>: _parity_codec::Decode, Vec<Xt>: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(Block{header: _parity_codec::Decode::decode(input)?,
                               extrinsics:
                                   _parity_codec::Decode::decode(input)?,})
                }
            }
        };
    impl <Xt: 'static + Codec + Sized + Send + Sync + Serialize + Clone + Eq +
          Debug + traits::Extrinsic> traits::Block for Block<Xt> {
        type
        Extrinsic
        =
        Xt;
        type
        Header
        =
        Header;
        type
        Hash
        =
        <Header as traits::Header>::Hash;
        fn header(&self) -> &Self::Header { &self.header }
        fn extrinsics(&self) -> &[Self::Extrinsic] { &self.extrinsics[..] }
        fn deconstruct(self) -> (Self::Header, Vec<Self::Extrinsic>) {
            (self.header, self.extrinsics)
        }
        fn new(header: Self::Header, extrinsics: Vec<Self::Extrinsic>)
         -> Self {
            Block{header, extrinsics,}
        }
    }
    impl <'a, Xt> Deserialize<'a> for Block<Xt> where Block<Xt>: Decode {
        fn deserialize<D: Deserializer<'a>>(de: D) -> Result<Self, D::Error> {
            let r = <Vec<u8>>::deserialize(de)?;
            Decode::decode(&mut &r[..]).ok_or(DeError::custom("Invalid value passed into decode"))
        }
    }
    /// Test transaction, tuple of (sender, index, call)
    /// with index only used if sender is some.
    ///
    /// If sender is some then the transaction is signed otherwise it is unsigned.
    #[structural_match]
    pub struct TestXt<Call>(pub Option<u64>, pub u64, pub Call);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Call: ::std::cmp::PartialEq> ::std::cmp::PartialEq for TestXt<Call>
     {
        #[inline]
        fn eq(&self, other: &TestXt<Call>) -> bool {
            match *other {
                TestXt(ref __self_1_0, ref __self_1_1, ref __self_1_2) =>
                match *self {
                    TestXt(ref __self_0_0, ref __self_0_1, ref __self_0_2) =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &TestXt<Call>) -> bool {
            match *other {
                TestXt(ref __self_1_0, ref __self_1_1, ref __self_1_2) =>
                match *self {
                    TestXt(ref __self_0_0, ref __self_0_1, ref __self_0_2) =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Call: ::std::cmp::Eq> ::std::cmp::Eq for TestXt<Call> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<Option<u64>>;
                let _: ::std::cmp::AssertParamIsEq<u64>;
                let _: ::std::cmp::AssertParamIsEq<Call>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Call: ::std::clone::Clone> ::std::clone::Clone for TestXt<Call> {
        #[inline]
        fn clone(&self) -> TestXt<Call> {
            match *self {
                TestXt(ref __self_0_0, ref __self_0_1, ref __self_0_2) =>
                TestXt(::std::clone::Clone::clone(&(*__self_0_0)),
                       ::std::clone::Clone::clone(&(*__self_0_1)),
                       ::std::clone::Clone::clone(&(*__self_0_2))),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_TestXt: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Call> _parity_codec::Encode for TestXt<Call> where
             Call: _parity_codec::Encode, Call: _parity_codec::Encode {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    dest.push(&self.0);
                    dest.push(&self.1);
                    dest.push(&self.2);
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_TestXt: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl <Call> _parity_codec::Decode for TestXt<Call> where
             Call: _parity_codec::Decode, Call: _parity_codec::Decode {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    Some(TestXt(_parity_codec::Decode::decode(input)?,
                                _parity_codec::Decode::decode(input)?,
                                _parity_codec::Decode::decode(input)?))
                }
            }
        };
    impl <Call> Serialize for TestXt<Call> where TestXt<Call>: Encode {
        fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error> where
         S: Serializer {
            self.using_encoded(|bytes| seq.serialize_bytes(bytes))
        }
    }
    impl <Call> Debug for TestXt<Call> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(&["TestXt(", ", ", ")"],
                                                      &match (&self.0,
                                                              &self.1) {
                                                           (arg0, arg1) =>
                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                        ::std::fmt::Debug::fmt),
                                                            ::std::fmt::ArgumentV1::new(arg1,
                                                                                        ::std::fmt::Debug::fmt)],
                                                       }))
        }
    }
    impl <Call: Codec + Sync + Send, Context> Checkable<Context> for
     TestXt<Call> {
        type
        Checked
        =
        Self;
        fn check(self, _: &Context) -> Result<Self::Checked, &'static str> {
            Ok(self)
        }
    }
    impl <Call: Codec + Sync + Send> traits::Extrinsic for TestXt<Call> {
        fn is_signed(&self) -> Option<bool> { Some(self.0.is_some()) }
    }
    impl <Call> Applyable for TestXt<Call> where Call: 'static + Sized +
     Send + Sync + Clone + Eq + Codec + Debug {
        type
        AccountId
        =
        u64;
        type
        Index
        =
        u64;
        type
        Call
        =
        Call;
        fn sender(&self) -> Option<&u64> { self.0.as_ref() }
        fn index(&self) -> Option<&u64> { self.0.as_ref().map(|_| &self.1) }
        fn deconstruct(self) -> (Self::Call, Option<Self::AccountId>) {
            (self.2, self.0)
        }
    }
}
pub mod traits {
    //! Primitives for the runtime modules.
    use rstd::prelude::*;
    use rstd::{self, result, marker::PhantomData};
    use runtime_io;
    #[cfg(feature = "std")]
    use std::fmt::{Debug, Display};
    #[cfg(feature = "std")]
    use serde::{Serialize, Deserialize, de::DeserializeOwned};
    use substrate_primitives::{self, Hasher, Blake2Hasher};
    use crate::codec::{Codec, Encode, HasCompact};
    use crate::transaction_validity::TransactionValidity;
    pub use integer_sqrt::IntegerSquareRoot;
    pub use num_traits::{Zero, One, Bounded, CheckedAdd, CheckedSub,
                         CheckedMul, CheckedDiv, CheckedShl, CheckedShr,
                         Saturating};
    use rstd::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign,
                    DivAssign, RemAssign, Shl, Shr};
    /// A lazy value.
    pub trait Lazy<T: ?Sized> {
        /// Get a reference to the underlying value.
        ///
        /// This will compute the value if the function is invoked for the first time.
        fn get(&mut self)
        -> &T;
    }
    impl <'a> Lazy<[u8]> for &'a [u8] {
        fn get(&mut self) -> &[u8] { &**self }
    }
    /// Means of signature verification.
    pub trait Verify {
        /// Type of the signer.
        type
        Signer;
        /// Verify a signature. Return `true` if signature is valid for the value.
        fn verify<L: Lazy<[u8]>>(&self, msg: L, signer: &Self::Signer)
        -> bool;
    }
    impl Verify for substrate_primitives::ed25519::Signature {
        type
        Signer
        =
        substrate_primitives::ed25519::Public;
        fn verify<L: Lazy<[u8]>>(&self, mut msg: L, signer: &Self::Signer)
         -> bool {
            runtime_io::ed25519_verify(self.as_ref(), msg.get(), signer)
        }
    }
    impl Verify for substrate_primitives::sr25519::Signature {
        type
        Signer
        =
        substrate_primitives::sr25519::Public;
        fn verify<L: Lazy<[u8]>>(&self, mut msg: L, signer: &Self::Signer)
         -> bool {
            runtime_io::sr25519_verify(self.as_ref(), msg.get(), signer)
        }
    }
    /// Some sort of check on the origin is performed by this object.
    pub trait EnsureOrigin<OuterOrigin> {
        /// A return type.
        type
        Success;
        /// Perform the origin check.
        fn ensure_origin(o: OuterOrigin)
        -> result::Result<Self::Success, &'static str>;
    }
    /// Means of changing one type into another in a manner dependent on the source type.
    pub trait Lookup {
        /// Type to lookup from.
        type
        Source;
        /// Type to lookup into.
        type
        Target;
        /// Attempt a lookup.
        fn lookup(&self, s: Self::Source)
        -> result::Result<Self::Target, &'static str>;
    }
    /// Means of changing one type into another in a manner dependent on the source type.
    /// This variant is different to `Lookup` in that it doesn't (can cannot) require any
    /// context.
    pub trait StaticLookup {
        /// Type to lookup from.
        type
        Source: Codec +
        Clone +
        PartialEq +
        MaybeDebug;
        /// Type to lookup into.
        type
        Target;
        /// Attempt a lookup.
        fn lookup(s: Self::Source)
        -> result::Result<Self::Target, &'static str>;
        /// Convert from Target back to Source.
        fn unlookup(t: Self::Target)
        -> Self::Source;
    }
    /// A lookup implementation returning the input value.
    pub struct IdentityLookup<T>(PhantomData<T>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <T: ::std::default::Default> ::std::default::Default for
     IdentityLookup<T> {
        #[inline]
        fn default() -> IdentityLookup<T> {
            IdentityLookup(::std::default::Default::default())
        }
    }
    impl <T: Codec + Clone + PartialEq + MaybeDebug> StaticLookup for
     IdentityLookup<T> {
        type
        Source
        =
        T;
        type
        Target
        =
        T;
        fn lookup(x: T) -> result::Result<T, &'static str> { Ok(x) }
        fn unlookup(x: T) -> T { x }
    }
    impl <T> Lookup for IdentityLookup<T> {
        type
        Source
        =
        T;
        type
        Target
        =
        T;
        fn lookup(&self, x: T) -> result::Result<T, &'static str> { Ok(x) }
    }
    /// Get the "current" block number.
    pub trait CurrentHeight {
        /// The type of the block number.
        type
        BlockNumber;
        /// Return the current block number. Not allowed to fail.
        fn current_height(&self)
        -> Self::BlockNumber;
    }
    /// Translate a block number into a hash.
    pub trait BlockNumberToHash {
        /// The type of the block number.
        type
        BlockNumber: Zero;
        /// The type of the hash.
        type
        Hash: Encode;
        /// Get the hash for a given block number, or `None` if unknown.
        fn block_number_to_hash(&self, n: Self::BlockNumber)
        -> Option<Self::Hash>;
        /// Get the genesis block hash; this should always be known.
        fn genesis_hash(&self) -> Self::Hash {
            self.block_number_to_hash(Zero::zero()).expect("All blockchains must know their genesis block hash; qed")
        }
    }
    /// Extensible conversion trait. Generic over both source and destination types.
    pub trait Convert<A, B> {
        /// Make conversion.
        fn convert(a: A)
        -> B;
    }
    impl <A, B: Default> Convert<A, B> for () {
        fn convert(_: A) -> B { Default::default() }
    }
    /// A structure that performs identity conversion.
    pub struct Identity;
    impl <T> Convert<T, T> for Identity {
        fn convert(a: T) -> T { a }
    }
    /// Simple trait similar to `Into`, except that it can be used to convert numerics between each
    /// other.
    pub trait As<T> {
        /// Convert forward (ala `Into::into`).
        fn as_(self)
        -> T;
        /// Convert backward (ala `From::from`).
        fn sa(_: T)
        -> Self;
    }
    macro_rules! impl_numerics(( $ ( $ t : ty ) , * ) => {
                               $ (
                               impl_numerics ! (
                               $ t : u8 , u16 , u32 , u64 , u128 , usize , i8
                               , i16 , i32 , i64 , i128 , isize , ) ; ) * } ;
                               ( $ f : ty : $ t : ty , $ ( $ rest : ty , ) * )
                               => {
                               impl As < $ t > for $ f {
                               fn as_ ( self ) -> $ t { self as $ t } fn sa (
                               t : $ t ) -> Self { t as Self } } impl_numerics
                               ! ( $ f : $ ( $ rest , ) * ) ; } ; ( $ f : ty :
                               ) => {  });
    impl As<u8> for u8 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for u8 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for u8 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for u8 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for u8 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for u8 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for u8 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for u8 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for u8 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for u8 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for u8 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for u8 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for u16 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for u16 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for u16 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for u16 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for u16 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for u16 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for u16 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for u16 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for u16 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for u16 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for u16 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for u16 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for u32 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for u32 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for u32 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for u32 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for u32 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for u32 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for u32 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for u32 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for u32 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for u32 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for u32 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for u32 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for u64 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for u64 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for u64 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for u64 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for u64 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for u64 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for u64 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for u64 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for u64 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for u64 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for u64 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for u64 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for u128 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for u128 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for u128 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for u128 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for u128 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for u128 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for u128 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for u128 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for u128 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for u128 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for u128 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for u128 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for usize {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for usize {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for usize {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for usize {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for usize {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for usize {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for usize {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for usize {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for usize {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for usize {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for usize {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for usize {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for i8 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for i8 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for i8 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for i8 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for i8 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for i8 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for i8 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for i8 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for i8 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for i8 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for i8 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for i8 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for i16 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for i16 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for i16 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for i16 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for i16 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for i16 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for i16 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for i16 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for i16 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for i16 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for i16 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for i16 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for i32 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for i32 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for i32 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for i32 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for i32 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for i32 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for i32 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for i32 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for i32 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for i32 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for i32 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for i32 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for i64 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for i64 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for i64 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for i64 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for i64 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for i64 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for i64 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for i64 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for i64 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for i64 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for i64 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for i64 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for i128 {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for i128 {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for i128 {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for i128 {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for i128 {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for i128 {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for i128 {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for i128 {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for i128 {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for i128 {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for i128 {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for i128 {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    impl As<u8> for isize {
        fn as_(self) -> u8 { self as u8 }
        fn sa(t: u8) -> Self { t as Self }
    }
    impl As<u16> for isize {
        fn as_(self) -> u16 { self as u16 }
        fn sa(t: u16) -> Self { t as Self }
    }
    impl As<u32> for isize {
        fn as_(self) -> u32 { self as u32 }
        fn sa(t: u32) -> Self { t as Self }
    }
    impl As<u64> for isize {
        fn as_(self) -> u64 { self as u64 }
        fn sa(t: u64) -> Self { t as Self }
    }
    impl As<u128> for isize {
        fn as_(self) -> u128 { self as u128 }
        fn sa(t: u128) -> Self { t as Self }
    }
    impl As<usize> for isize {
        fn as_(self) -> usize { self as usize }
        fn sa(t: usize) -> Self { t as Self }
    }
    impl As<i8> for isize {
        fn as_(self) -> i8 { self as i8 }
        fn sa(t: i8) -> Self { t as Self }
    }
    impl As<i16> for isize {
        fn as_(self) -> i16 { self as i16 }
        fn sa(t: i16) -> Self { t as Self }
    }
    impl As<i32> for isize {
        fn as_(self) -> i32 { self as i32 }
        fn sa(t: i32) -> Self { t as Self }
    }
    impl As<i64> for isize {
        fn as_(self) -> i64 { self as i64 }
        fn sa(t: i64) -> Self { t as Self }
    }
    impl As<i128> for isize {
        fn as_(self) -> i128 { self as i128 }
        fn sa(t: i128) -> Self { t as Self }
    }
    impl As<isize> for isize {
        fn as_(self) -> isize { self as isize }
        fn sa(t: isize) -> Self { t as Self }
    }
    /// A meta trait for arithmetic.
    pub trait SimpleArithmetic: Zero + One + IntegerSquareRoot + As<u64> +
     Add<Self, Output = Self> + AddAssign<Self> + Sub<Self, Output = Self> +
     SubAssign<Self> + Mul<Self, Output = Self> + MulAssign<Self> + Div<Self,
     Output = Self> + DivAssign<Self> + Rem<Self, Output = Self> +
     RemAssign<Self> + Shl<u32, Output = Self> + Shr<u32, Output = Self> +
     CheckedShl + CheckedShr + CheckedAdd + CheckedSub + CheckedMul +
     CheckedDiv + Saturating + PartialOrd<Self> + Ord + Bounded + HasCompact {
    }
    impl <T: Zero + One + IntegerSquareRoot + As<u64> + Add<Self, Output =
          Self> + AddAssign<Self> + Sub<Self, Output = Self> +
          SubAssign<Self> + Mul<Self, Output = Self> + MulAssign<Self> +
          Div<Self, Output = Self> + DivAssign<Self> + Rem<Self, Output =
          Self> + RemAssign<Self> + Shl<u32, Output = Self> + Shr<u32, Output
          = Self> + CheckedShl + CheckedShr + CheckedAdd + CheckedSub +
          CheckedMul + CheckedDiv + Saturating + PartialOrd<Self> + Ord +
          Bounded + HasCompact> SimpleArithmetic for T {
    }
    /// Trait for things that can be clear (have no bits set). For numeric types, essentially the same
    /// as `Zero`.
    pub trait Clear {
        /// True iff no bits are set.
        fn is_clear(&self)
        -> bool;
        /// Return the value of Self that is clear.
        fn clear()
        -> Self;
    }
    impl <T: Default + Eq + PartialEq> Clear for T {
        fn is_clear(&self) -> bool { *self == Self::clear() }
        fn clear() -> Self { Default::default() }
    }
    /// A meta trait for all bit ops.
    pub trait SimpleBitOps: Sized + Clear + rstd::ops::BitOr<Self, Output =
     Self> + rstd::ops::BitXor<Self, Output = Self> + rstd::ops::BitAnd<Self,
     Output = Self> {
    }
    impl <T: Sized + Clear + rstd::ops::BitOr<Self, Output = Self> +
          rstd::ops::BitXor<Self, Output = Self> + rstd::ops::BitAnd<Self,
          Output = Self>> SimpleBitOps for T {
    }
    /// The block finalization trait. Implementing this lets you express what should happen
    /// for your module when the block is ending.
    pub trait OnFinalize<BlockNumber> {
        /// The block is being finalized. Implement to have something happen.
        fn on_finalize(_n: BlockNumber) { }
    }
    impl <N> OnFinalize<N> for () { }
    /// The block initialization trait. Implementing this lets you express what should happen
    /// for your module when the block is beginning (right before the first extrinsic is executed).
    pub trait OnInitialize<BlockNumber> {
        /// The block is being initialized. Implement to have something happen.
        fn on_initialize(_n: BlockNumber) { }
    }
    impl <N> OnInitialize<N> for () { }
    /// Off-chain computation trait.
    ///
    /// Implementing this trait on a module allows you to perform long-running tasks
    /// that make validators generate extrinsics (either transactions or inherents)
    /// with the results of those long-running computations.
    ///
    /// NOTE: This function runs off-chain, so it can access the block state,
    /// but cannot preform any alterations.
    pub trait OffchainWorker<BlockNumber> {
        /// This function is being called on every block.
        ///
        /// Implement this and use special `extern`s to generate transactions or inherents.
        /// Any state alterations are lost and are not persisted.
        fn generate_extrinsics(_n: BlockNumber) { }
    }
    impl <N> OffchainWorker<N> for () { }
    macro_rules! tuple_impl(( $ first : ident , $ ( $ rest : ident , ) + ) =>
                            {
                            tuple_impl ! (
                            [ $ first ] [ $ first ] [ $ ( $ rest ) + ] ) ; } ;
                            (
                            [ $ ( $ direct : ident ) + ] [
                            $ ( $ reverse : ident ) + ] [  ] ) => {
                            impl < Number : Copy , $ (
                            $ direct : OnFinalize < Number > ) , + >
                            OnFinalize < Number > for ( $ ( $ direct ) , + , )
                            {
                            fn on_finalize ( n : Number ) {
                            $ ( $ reverse :: on_finalize ( n ) ; ) + } } impl
                            < Number : Copy , $ (
                            $ direct : OnInitialize < Number > ) , + >
                            OnInitialize < Number > for ( $ ( $ direct ) , + ,
                            ) {
                            fn on_initialize ( n : Number ) {
                            $ ( $ direct :: on_initialize ( n ) ; ) + } } impl
                            < Number : Copy , $ (
                            $ direct : OffchainWorker < Number > ) , + >
                            OffchainWorker < Number > for (
                            $ ( $ direct ) , + , ) {
                            fn generate_extrinsics ( n : Number ) {
                            $ ( $ direct :: generate_extrinsics ( n ) ; ) + }
                            } } ; (
                            [ $ ( $ direct : ident ) + ] [
                            $ ( $ reverse : ident ) + ] [
                            $ first : ident $ ( $ rest : ident ) * ] ) => {
                            tuple_impl ! (
                            [ $ ( $ direct ) + ] [ $ ( $ reverse ) + ] [  ] )
                            ; tuple_impl ! (
                            [ $ ( $ direct ) + $ first ] [
                            $ first $ ( $ reverse ) + ] [ $ ( $ rest ) * ] ) ;
                            } ;);
    impl <Number: Copy, A: OnFinalize<Number>> OnFinalize<Number> for (A,) {
        fn on_finalize(n: Number) { A::on_finalize(n); }
    }
    impl <Number: Copy, A: OnInitialize<Number>> OnInitialize<Number> for (A,)
     {
        fn on_initialize(n: Number) { A::on_initialize(n); }
    }
    impl <Number: Copy, A: OffchainWorker<Number>> OffchainWorker<Number> for
     (A,) {
        fn generate_extrinsics(n: Number) { A::generate_extrinsics(n); }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>>
     OnFinalize<Number> for (A, B) {
        fn on_finalize(n: Number) { B::on_finalize(n); A::on_finalize(n); }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>>
     OnInitialize<Number> for (A, B) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>>
     OffchainWorker<Number> for (A, B) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>> OnFinalize<Number> for (A, B, C) {
        fn on_finalize(n: Number) {
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>> OnInitialize<Number> for (A, B, C) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>> OffchainWorker<Number> for (A, B, C) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D) {
        fn on_finalize(n: Number) {
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>>
     OnInitialize<Number> for (A, B, C, D) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>>
     OffchainWorker<Number> for (A, B, C, D) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>>
     OnFinalize<Number> for (A, B, C, D, E) {
        fn on_finalize(n: Number) {
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>> OnInitialize<Number> for (A, B, C, D, E) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>> OnFinalize<Number> for (A, B, C, D, E, F) {
        fn on_finalize(n: Number) {
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>>
     OnInitialize<Number> for (A, B, C, D, E, F) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>>
     OffchainWorker<Number> for (A, B, C, D, E, F) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G) {
        fn on_finalize(n: Number) {
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>>
     OnFinalize<Number> for (A, B, C, D, E, F, G, H) {
        fn on_finalize(n: Number) {
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>>
     OnInitialize<Number> for (A, B, C, D, E, F, G, H) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>>
     OffchainWorker<Number> for (A, B, C, D, E, F, G, H) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I) {
        fn on_finalize(n: Number) {
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J) {
        fn on_finalize(n: Number) {
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>>
     OnInitialize<Number> for (A, B, C, D, E, F, G, H, I, J) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>>
     OffchainWorker<Number> for (A, B, C, D, E, F, G, H, I, J) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>>
     OnFinalize<Number> for (A, B, C, D, E, F, G, H, I, J, K) {
        fn on_finalize(n: Number) {
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L) {
        fn on_finalize(n: Number) {
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>>
     OnInitialize<Number> for (A, B, C, D, E, F, G, H, I, J, K, L) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>>
     OffchainWorker<Number> for (A, B, C, D, E, F, G, H, I, J, K, L) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M) {
        fn on_finalize(n: Number) {
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>>
     OnFinalize<Number> for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
        fn on_finalize(n: Number) {
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>>
     OnInitialize<Number> for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>>
     OffchainWorker<Number> for (A, B, C, D, E, F, G, H, I, J, K, L, M, N) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
        fn on_finalize(n: Number) {
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
        fn on_finalize(n: Number) {
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>>
     OnInitialize<Number> for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
     {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>>
     OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>>
     OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) {
        fn on_finalize(n: Number) {
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>,
          R: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) {
        fn on_finalize(n: Number) {
            R::on_finalize(n);
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>, R: OnInitialize<Number>>
     OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
            R::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>, R: OffchainWorker<Number>>
     OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
            R::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>,
          R: OnFinalize<Number>, S: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
        fn on_finalize(n: Number) {
            S::on_finalize(n);
            R::on_finalize(n);
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>, R: OnInitialize<Number>,
          S: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
            R::on_initialize(n);
            S::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>, R: OffchainWorker<Number>,
          S: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
            R::generate_extrinsics(n);
            S::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>,
          R: OnFinalize<Number>, S: OnFinalize<Number>, T: OnFinalize<Number>>
     OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) {
        fn on_finalize(n: Number) {
            T::on_finalize(n);
            S::on_finalize(n);
            R::on_finalize(n);
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>, R: OnInitialize<Number>,
          S: OnInitialize<Number>, T: OnInitialize<Number>>
     OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
            R::on_initialize(n);
            S::on_initialize(n);
            T::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>, R: OffchainWorker<Number>,
          S: OffchainWorker<Number>, T: OffchainWorker<Number>>
     OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
            R::generate_extrinsics(n);
            S::generate_extrinsics(n);
            T::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>,
          R: OnFinalize<Number>, S: OnFinalize<Number>, T: OnFinalize<Number>,
          U: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) {
        fn on_finalize(n: Number) {
            U::on_finalize(n);
            T::on_finalize(n);
            S::on_finalize(n);
            R::on_finalize(n);
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>, R: OnInitialize<Number>,
          S: OnInitialize<Number>, T: OnInitialize<Number>,
          U: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
            R::on_initialize(n);
            S::on_initialize(n);
            T::on_initialize(n);
            U::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>, R: OffchainWorker<Number>,
          S: OffchainWorker<Number>, T: OffchainWorker<Number>,
          U: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
            R::generate_extrinsics(n);
            S::generate_extrinsics(n);
            T::generate_extrinsics(n);
            U::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>,
          R: OnFinalize<Number>, S: OnFinalize<Number>, T: OnFinalize<Number>,
          U: OnFinalize<Number>, V: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) {
        fn on_finalize(n: Number) {
            V::on_finalize(n);
            U::on_finalize(n);
            T::on_finalize(n);
            S::on_finalize(n);
            R::on_finalize(n);
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>, R: OnInitialize<Number>,
          S: OnInitialize<Number>, T: OnInitialize<Number>,
          U: OnInitialize<Number>, V: OnInitialize<Number>>
     OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
            R::on_initialize(n);
            S::on_initialize(n);
            T::on_initialize(n);
            U::on_initialize(n);
            V::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>, R: OffchainWorker<Number>,
          S: OffchainWorker<Number>, T: OffchainWorker<Number>,
          U: OffchainWorker<Number>, V: OffchainWorker<Number>>
     OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
            R::generate_extrinsics(n);
            S::generate_extrinsics(n);
            T::generate_extrinsics(n);
            U::generate_extrinsics(n);
            V::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>,
          R: OnFinalize<Number>, S: OnFinalize<Number>, T: OnFinalize<Number>,
          U: OnFinalize<Number>, V: OnFinalize<Number>, W: OnFinalize<Number>>
     OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) {
        fn on_finalize(n: Number) {
            W::on_finalize(n);
            V::on_finalize(n);
            U::on_finalize(n);
            T::on_finalize(n);
            S::on_finalize(n);
            R::on_finalize(n);
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>, R: OnInitialize<Number>,
          S: OnInitialize<Number>, T: OnInitialize<Number>,
          U: OnInitialize<Number>, V: OnInitialize<Number>,
          W: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
            R::on_initialize(n);
            S::on_initialize(n);
            T::on_initialize(n);
            U::on_initialize(n);
            V::on_initialize(n);
            W::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>, R: OffchainWorker<Number>,
          S: OffchainWorker<Number>, T: OffchainWorker<Number>,
          U: OffchainWorker<Number>, V: OffchainWorker<Number>,
          W: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
            R::generate_extrinsics(n);
            S::generate_extrinsics(n);
            T::generate_extrinsics(n);
            U::generate_extrinsics(n);
            V::generate_extrinsics(n);
            W::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>,
          R: OnFinalize<Number>, S: OnFinalize<Number>, T: OnFinalize<Number>,
          U: OnFinalize<Number>, V: OnFinalize<Number>, W: OnFinalize<Number>,
          X: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X)
     {
        fn on_finalize(n: Number) {
            X::on_finalize(n);
            W::on_finalize(n);
            V::on_finalize(n);
            U::on_finalize(n);
            T::on_finalize(n);
            S::on_finalize(n);
            R::on_finalize(n);
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>, R: OnInitialize<Number>,
          S: OnInitialize<Number>, T: OnInitialize<Number>,
          U: OnInitialize<Number>, V: OnInitialize<Number>,
          W: OnInitialize<Number>, X: OnInitialize<Number>>
     OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X)
     {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
            R::on_initialize(n);
            S::on_initialize(n);
            T::on_initialize(n);
            U::on_initialize(n);
            V::on_initialize(n);
            W::on_initialize(n);
            X::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>, R: OffchainWorker<Number>,
          S: OffchainWorker<Number>, T: OffchainWorker<Number>,
          U: OffchainWorker<Number>, V: OffchainWorker<Number>,
          W: OffchainWorker<Number>, X: OffchainWorker<Number>>
     OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X)
     {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
            R::generate_extrinsics(n);
            S::generate_extrinsics(n);
            T::generate_extrinsics(n);
            U::generate_extrinsics(n);
            V::generate_extrinsics(n);
            W::generate_extrinsics(n);
            X::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>,
          R: OnFinalize<Number>, S: OnFinalize<Number>, T: OnFinalize<Number>,
          U: OnFinalize<Number>, V: OnFinalize<Number>, W: OnFinalize<Number>,
          X: OnFinalize<Number>, Y: OnFinalize<Number>> OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,
      Y) {
        fn on_finalize(n: Number) {
            Y::on_finalize(n);
            X::on_finalize(n);
            W::on_finalize(n);
            V::on_finalize(n);
            U::on_finalize(n);
            T::on_finalize(n);
            S::on_finalize(n);
            R::on_finalize(n);
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>, R: OnInitialize<Number>,
          S: OnInitialize<Number>, T: OnInitialize<Number>,
          U: OnInitialize<Number>, V: OnInitialize<Number>,
          W: OnInitialize<Number>, X: OnInitialize<Number>,
          Y: OnInitialize<Number>> OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,
      Y) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
            R::on_initialize(n);
            S::on_initialize(n);
            T::on_initialize(n);
            U::on_initialize(n);
            V::on_initialize(n);
            W::on_initialize(n);
            X::on_initialize(n);
            Y::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>, R: OffchainWorker<Number>,
          S: OffchainWorker<Number>, T: OffchainWorker<Number>,
          U: OffchainWorker<Number>, V: OffchainWorker<Number>,
          W: OffchainWorker<Number>, X: OffchainWorker<Number>,
          Y: OffchainWorker<Number>> OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,
      Y) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
            R::generate_extrinsics(n);
            S::generate_extrinsics(n);
            T::generate_extrinsics(n);
            U::generate_extrinsics(n);
            V::generate_extrinsics(n);
            W::generate_extrinsics(n);
            X::generate_extrinsics(n);
            Y::generate_extrinsics(n);
        }
    }
    impl <Number: Copy, A: OnFinalize<Number>, B: OnFinalize<Number>,
          C: OnFinalize<Number>, D: OnFinalize<Number>, E: OnFinalize<Number>,
          F: OnFinalize<Number>, G: OnFinalize<Number>, H: OnFinalize<Number>,
          I: OnFinalize<Number>, J: OnFinalize<Number>, K: OnFinalize<Number>,
          L: OnFinalize<Number>, M: OnFinalize<Number>, N: OnFinalize<Number>,
          O: OnFinalize<Number>, P: OnFinalize<Number>, Q: OnFinalize<Number>,
          R: OnFinalize<Number>, S: OnFinalize<Number>, T: OnFinalize<Number>,
          U: OnFinalize<Number>, V: OnFinalize<Number>, W: OnFinalize<Number>,
          X: OnFinalize<Number>, Y: OnFinalize<Number>, Z: OnFinalize<Number>>
     OnFinalize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,
      Y, Z) {
        fn on_finalize(n: Number) {
            Z::on_finalize(n);
            Y::on_finalize(n);
            X::on_finalize(n);
            W::on_finalize(n);
            V::on_finalize(n);
            U::on_finalize(n);
            T::on_finalize(n);
            S::on_finalize(n);
            R::on_finalize(n);
            Q::on_finalize(n);
            P::on_finalize(n);
            O::on_finalize(n);
            N::on_finalize(n);
            M::on_finalize(n);
            L::on_finalize(n);
            K::on_finalize(n);
            J::on_finalize(n);
            I::on_finalize(n);
            H::on_finalize(n);
            G::on_finalize(n);
            F::on_finalize(n);
            E::on_finalize(n);
            D::on_finalize(n);
            C::on_finalize(n);
            B::on_finalize(n);
            A::on_finalize(n);
        }
    }
    impl <Number: Copy, A: OnInitialize<Number>, B: OnInitialize<Number>,
          C: OnInitialize<Number>, D: OnInitialize<Number>,
          E: OnInitialize<Number>, F: OnInitialize<Number>,
          G: OnInitialize<Number>, H: OnInitialize<Number>,
          I: OnInitialize<Number>, J: OnInitialize<Number>,
          K: OnInitialize<Number>, L: OnInitialize<Number>,
          M: OnInitialize<Number>, N: OnInitialize<Number>,
          O: OnInitialize<Number>, P: OnInitialize<Number>,
          Q: OnInitialize<Number>, R: OnInitialize<Number>,
          S: OnInitialize<Number>, T: OnInitialize<Number>,
          U: OnInitialize<Number>, V: OnInitialize<Number>,
          W: OnInitialize<Number>, X: OnInitialize<Number>,
          Y: OnInitialize<Number>, Z: OnInitialize<Number>>
     OnInitialize<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,
      Y, Z) {
        fn on_initialize(n: Number) {
            A::on_initialize(n);
            B::on_initialize(n);
            C::on_initialize(n);
            D::on_initialize(n);
            E::on_initialize(n);
            F::on_initialize(n);
            G::on_initialize(n);
            H::on_initialize(n);
            I::on_initialize(n);
            J::on_initialize(n);
            K::on_initialize(n);
            L::on_initialize(n);
            M::on_initialize(n);
            N::on_initialize(n);
            O::on_initialize(n);
            P::on_initialize(n);
            Q::on_initialize(n);
            R::on_initialize(n);
            S::on_initialize(n);
            T::on_initialize(n);
            U::on_initialize(n);
            V::on_initialize(n);
            W::on_initialize(n);
            X::on_initialize(n);
            Y::on_initialize(n);
            Z::on_initialize(n);
        }
    }
    impl <Number: Copy, A: OffchainWorker<Number>, B: OffchainWorker<Number>,
          C: OffchainWorker<Number>, D: OffchainWorker<Number>,
          E: OffchainWorker<Number>, F: OffchainWorker<Number>,
          G: OffchainWorker<Number>, H: OffchainWorker<Number>,
          I: OffchainWorker<Number>, J: OffchainWorker<Number>,
          K: OffchainWorker<Number>, L: OffchainWorker<Number>,
          M: OffchainWorker<Number>, N: OffchainWorker<Number>,
          O: OffchainWorker<Number>, P: OffchainWorker<Number>,
          Q: OffchainWorker<Number>, R: OffchainWorker<Number>,
          S: OffchainWorker<Number>, T: OffchainWorker<Number>,
          U: OffchainWorker<Number>, V: OffchainWorker<Number>,
          W: OffchainWorker<Number>, X: OffchainWorker<Number>,
          Y: OffchainWorker<Number>, Z: OffchainWorker<Number>>
     OffchainWorker<Number> for
     (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X,
      Y, Z) {
        fn generate_extrinsics(n: Number) {
            A::generate_extrinsics(n);
            B::generate_extrinsics(n);
            C::generate_extrinsics(n);
            D::generate_extrinsics(n);
            E::generate_extrinsics(n);
            F::generate_extrinsics(n);
            G::generate_extrinsics(n);
            H::generate_extrinsics(n);
            I::generate_extrinsics(n);
            J::generate_extrinsics(n);
            K::generate_extrinsics(n);
            L::generate_extrinsics(n);
            M::generate_extrinsics(n);
            N::generate_extrinsics(n);
            O::generate_extrinsics(n);
            P::generate_extrinsics(n);
            Q::generate_extrinsics(n);
            R::generate_extrinsics(n);
            S::generate_extrinsics(n);
            T::generate_extrinsics(n);
            U::generate_extrinsics(n);
            V::generate_extrinsics(n);
            W::generate_extrinsics(n);
            X::generate_extrinsics(n);
            Y::generate_extrinsics(n);
            Z::generate_extrinsics(n);
        }
    }
    /// Abstraction around hashing
    pub trait Hash: 'static + MaybeSerializeDebug + Clone + Eq + PartialEq {
        /// The hash type produced.
        type
        Output: Member +
        MaybeSerializeDebug +
        rstd::hash::Hash +
        AsRef<[u8]> +
        AsMut<[u8]> +
        Copy +
        Default;
        /// The associated hash_db Hasher type.
        type
        Hasher: Hasher<Out
        =
        Self::Output>;
        /// Produce the hash of some byte-slice.
        fn hash(s: &[u8])
        -> Self::Output;
        /// Produce the hash of some codec-encodable value.
        fn hash_of<S: Codec>(s: &S) -> Self::Output {
            Encode::using_encoded(s, Self::hash)
        }
        /// Produce the trie-db root of a mapping from indices to byte slices.
        fn enumerated_trie_root(items: &[&[u8]])
        -> Self::Output;
        /// Iterator-based version of `enumerated_trie_root`.
        fn ordered_trie_root<I: IntoIterator<Item = A> + Iterator<Item = A>,
                             A: AsRef<[u8]>>(input: I)
        -> Self::Output;
        /// The Patricia tree root of the given mapping as an iterator.
        fn trie_root<I: IntoIterator<Item = (A, B)>, A: AsRef<[u8]> + Ord,
                     B: AsRef<[u8]>>(input: I)
        -> Self::Output;
        /// Acquire the global storage root.
        fn storage_root()
        -> Self::Output;
        /// Acquire the global storage changes root.
        fn storage_changes_root(parent_hash: Self::Output, parent_number: u64)
        -> Option<Self::Output>;
    }
    /// Blake2-256 Hash implementation.
    #[structural_match]
    pub struct BlakeTwo256;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlakeTwo256 {
        #[inline]
        fn eq(&self, other: &BlakeTwo256) -> bool {
            match *other {
                BlakeTwo256 => match *self { BlakeTwo256 => true, },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for BlakeTwo256 {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for BlakeTwo256 {
        #[inline]
        fn clone(&self) -> BlakeTwo256 {
            match *self { BlakeTwo256 => BlakeTwo256, }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlakeTwo256 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                BlakeTwo256 => {
                    let mut debug_trait_builder =
                        f.debug_tuple("BlakeTwo256");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_BlakeTwo256: () =
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
            impl _serde::Serialize for BlakeTwo256 {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    _serde::Serializer::serialize_unit_struct(__serializer,
                                                              "BlakeTwo256")
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_BlakeTwo256: () =
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
            impl <'de> _serde::Deserialize<'de> for BlakeTwo256 {
                fn deserialize<__D>(__deserializer: __D)
                 -> _serde::export::Result<Self, __D::Error> where
                 __D: _serde::Deserializer<'de> {
                    struct __Visitor;
                    impl <'de> _serde::de::Visitor<'de> for __Visitor {
                        type
                        Value
                        =
                        BlakeTwo256;
                        fn expecting(&self,
                                     __formatter:
                                         &mut _serde::export::Formatter)
                         -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter,
                                                                 "unit struct BlakeTwo256")
                        }
                        #[inline]
                        fn visit_unit<__E>(self)
                         -> _serde::export::Result<Self::Value, __E> where
                         __E: _serde::de::Error {
                            _serde::export::Ok(BlakeTwo256)
                        }
                    }
                    _serde::Deserializer::deserialize_unit_struct(__deserializer,
                                                                  "BlakeTwo256",
                                                                  __Visitor)
                }
            }
        };
    impl Hash for BlakeTwo256 {
        type
        Output
        =
        substrate_primitives::H256;
        type
        Hasher
        =
        Blake2Hasher;
        fn hash(s: &[u8]) -> Self::Output { runtime_io::blake2_256(s).into() }
        fn enumerated_trie_root(items: &[&[u8]]) -> Self::Output {
            runtime_io::enumerated_trie_root::<Blake2Hasher>(items).into()
        }
        fn trie_root<I: IntoIterator<Item = (A, B)>, A: AsRef<[u8]> + Ord,
                     B: AsRef<[u8]>>(input: I) -> Self::Output {
            runtime_io::trie_root::<Blake2Hasher, _, _, _>(input).into()
        }
        fn ordered_trie_root<I: IntoIterator<Item = A> + Iterator<Item = A>,
                             A: AsRef<[u8]>>(input: I) -> Self::Output {
            runtime_io::ordered_trie_root::<Blake2Hasher, _, _>(input).into()
        }
        fn storage_root() -> Self::Output {
            runtime_io::storage_root().into()
        }
        fn storage_changes_root(parent_hash: Self::Output, parent_number: u64)
         -> Option<Self::Output> {
            runtime_io::storage_changes_root(parent_hash.into(),
                                             parent_number).map(Into::into)
        }
    }
    /// Something that can be checked for equality and printed out to a debug channel if bad.
    pub trait CheckEqual {
        /// Perform the equality check.
        fn check_equal(&self, other: &Self);
    }
    impl CheckEqual for substrate_primitives::H256 {
        #[cfg(feature = "std")]
        fn check_equal(&self, other: &Self) {
            use substrate_primitives::hexdisplay::HexDisplay;
            if self != other {
                {
                    ::std::io::_print(::std::fmt::Arguments::new_v1(&["Hash: given=",
                                                                      ", expected=",
                                                                      "\n"],
                                                                    &match (&HexDisplay::from(self.as_fixed_bytes()),
                                                                            &HexDisplay::from(other.as_fixed_bytes()))
                                                                         {
                                                                         (arg0,
                                                                          arg1)
                                                                         =>
                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                      ::std::fmt::Display::fmt),
                                                                          ::std::fmt::ArgumentV1::new(arg1,
                                                                                                      ::std::fmt::Display::fmt)],
                                                                     }));
                };
            }
        }
    }
    impl <I> CheckEqual for I where I: DigestItem {
        #[cfg(feature = "std")]
        fn check_equal(&self, other: &Self) {
            if self != other {
                {
                    ::std::io::_print(::std::fmt::Arguments::new_v1(&["DigestItem: given=",
                                                                      ", expected=",
                                                                      "\n"],
                                                                    &match (&self,
                                                                            &other)
                                                                         {
                                                                         (arg0,
                                                                          arg1)
                                                                         =>
                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                      ::std::fmt::Debug::fmt),
                                                                          ::std::fmt::ArgumentV1::new(arg1,
                                                                                                      ::std::fmt::Debug::fmt)],
                                                                     }));
                };
            }
        }
    }
    /// A type that implements Serialize and Debug when in std environment.
    #[cfg(feature = "std")]
    pub trait MaybeSerializeDebugButNotDeserialize: Serialize + Debug { }
    #[cfg(feature = "std")]
    impl <T: Serialize + Debug> MaybeSerializeDebugButNotDeserialize for T { }
    /// A type that implements Serialize when in std environment.
    #[cfg(feature = "std")]
    pub trait MaybeSerialize: Serialize { }
    #[cfg(feature = "std")]
    impl <T: Serialize> MaybeSerialize for T { }
    /// A type that implements Serialize, DeserializeOwned and Debug when in std environment.
    #[cfg(feature = "std")]
    pub trait MaybeSerializeDebug: Serialize + DeserializeOwned + Debug { }
    #[cfg(feature = "std")]
    impl <T: Serialize + DeserializeOwned + Debug> MaybeSerializeDebug for T {
    }
    /// A type that implements Debug when in std environment.
    #[cfg(feature = "std")]
    pub trait MaybeDebug: Debug { }
    #[cfg(feature = "std")]
    impl <T: Debug> MaybeDebug for T { }
    /// A type that implements Display when in std environment.
    #[cfg(feature = "std")]
    pub trait MaybeDisplay: Display { }
    #[cfg(feature = "std")]
    impl <T: Display> MaybeDisplay for T { }
    /// A type that implements Hash when in std environment.
    #[cfg(feature = "std")]
    pub trait MaybeHash: ::rstd::hash::Hash { }
    #[cfg(feature = "std")]
    impl <T: ::rstd::hash::Hash> MaybeHash for T { }
    /// A type that can be used in runtime structures.
    pub trait Member: Send + Sync + Sized + MaybeDebug + Eq + PartialEq +
     Clone + 'static {
    }
    impl <T: Send + Sync + Sized + MaybeDebug + Eq + PartialEq + Clone +
          'static> Member for T {
    }
    /// Something which fulfills the abstract idea of a Substrate header. It has types for a `Number`,
    /// a `Hash` and a `Digest`. It provides access to an `extrinsics_root`, `state_root` and
    /// `parent_hash`, as well as a `digest` and a block `number`.
    ///
    /// You can also create a `new` one from those fields.
    pub trait Header: Clone + Send + Sync + Codec + Eq +
     MaybeSerializeDebugButNotDeserialize + 'static {
        /// Header number.
        type
        Number: Member +
        MaybeSerializeDebug +
        ::rstd::hash::Hash +
        Copy +
        MaybeDisplay +
        SimpleArithmetic +
        Codec;
        /// Header hash type
        type
        Hash: Member +
        MaybeSerializeDebug +
        ::rstd::hash::Hash +
        Copy +
        MaybeDisplay +
        Default +
        SimpleBitOps +
        Codec +
        AsRef<[u8]> +
        AsMut<[u8]>;
        /// Hashing algorithm
        type
        Hashing: Hash<Output
        =
        Self::Hash>;
        /// Digest type
        type
        Digest: Digest<Hash
        =
        Self::Hash> +
        Codec;
        /// Creates new header.
        fn new(number: Self::Number, extrinsics_root: Self::Hash,
               state_root: Self::Hash, parent_hash: Self::Hash,
               digest: Self::Digest)
        -> Self;
        /// Returns a reference to the header number.
        fn number(&self)
        -> &Self::Number;
        /// Sets the header number.
        fn set_number(&mut self, number: Self::Number);
        /// Returns a reference to the extrinsics root.
        fn extrinsics_root(&self)
        -> &Self::Hash;
        /// Sets the extrinsic root.
        fn set_extrinsics_root(&mut self, root: Self::Hash);
        /// Returns a reference to the state root.
        fn state_root(&self)
        -> &Self::Hash;
        /// Sets the state root.
        fn set_state_root(&mut self, root: Self::Hash);
        /// Returns a reference to the parent hash.
        fn parent_hash(&self)
        -> &Self::Hash;
        /// Sets the parent hash.
        fn set_parent_hash(&mut self, hash: Self::Hash);
        /// Returns a reference to the digest.
        fn digest(&self)
        -> &Self::Digest;
        /// Get a mutable reference to the digest.
        fn digest_mut(&mut self)
        -> &mut Self::Digest;
        /// Sets the digest.
        fn set_digest(&mut self, digest: Self::Digest);
        /// Returns the hash of the header.
        fn hash(&self) -> Self::Hash {
            <Self::Hashing as Hash>::hash_of(self)
        }
    }
    /// Something which fulfills the abstract idea of a Substrate block. It has types for an
    /// `Extrinsic` piece of information as well as a `Header`.
    ///
    /// You can get an iterator over each of the `extrinsics` and retrieve the `header`.
    pub trait Block: Clone + Send + Sync + Codec + Eq +
     MaybeSerializeDebugButNotDeserialize + 'static {
        /// Type of extrinsics.
        type
        Extrinsic: Member +
        Codec +
        Extrinsic +
        MaybeSerialize;
        /// Header type.
        type
        Header: Header<Hash
        =
        Self::Hash>;
        /// Block hash type.
        type
        Hash: Member +
        MaybeSerializeDebug +
        ::rstd::hash::Hash +
        Copy +
        MaybeDisplay +
        Default +
        SimpleBitOps +
        Codec +
        AsRef<[u8]> +
        AsMut<[u8]>;
        /// Returns a reference to the header.
        fn header(&self)
        -> &Self::Header;
        /// Returns a reference to the list of extrinsics.
        fn extrinsics(&self)
        -> &[Self::Extrinsic];
        /// Split the block into header and list of extrinsics.
        fn deconstruct(self)
        -> (Self::Header, Vec<Self::Extrinsic>);
        /// Creates new block from header and extrinsics.
        fn new(header: Self::Header, extrinsics: Vec<Self::Extrinsic>)
        -> Self;
        /// Returns the hash of the block.
        fn hash(&self) -> Self::Hash {
            <<Self::Header as Header>::Hashing as
                Hash>::hash_of(self.header())
        }
    }
    /// Something that acts like an `Extrinsic`.
    pub trait Extrinsic {
        /// Is this `Extrinsic` signed?
        /// If no information are available about signed/unsigned, `None` should be returned.
        fn is_signed(&self) -> Option<bool> { None }
    }
    /// Extract the hashing type for a block.
    pub type HashFor<B> = <<B as Block>::Header as Header>::Hashing;
    /// Extract the number type for a block.
    pub type NumberFor<B> = <<B as Block>::Header as Header>::Number;
    /// Extract the digest type for a block.
    pub type DigestFor<B> = <<B as Block>::Header as Header>::Digest;
    /// Extract the digest item type for a block.
    pub type DigestItemFor<B> = <DigestFor<B> as Digest>::Item;
    /// Extract the authority ID type for a block.
    pub type AuthorityIdFor<B>
        =
        <DigestItemFor<B> as DigestItem>::AuthorityId;
    /// A "checkable" piece of information, used by the standard Substrate Executive in order to
    /// check the validity of a piece of extrinsic information, usually by verifying the signature.
    /// Implement for pieces of information that require some additional context `Context` in order to be
    /// checked.
    pub trait Checkable<Context>: Sized {
        /// Returned if `check` succeeds.
        type
        Checked;
        /// Check self, given an instance of Context.
        fn check(self, c: &Context)
        -> Result<Self::Checked, &'static str>;
    }
    /// A "checkable" piece of information, used by the standard Substrate Executive in order to
    /// check the validity of a piece of extrinsic information, usually by verifying the signature.
    /// Implement for pieces of information that don't require additional context in order to be
    /// checked.
    pub trait BlindCheckable: Sized {
        /// Returned if `check` succeeds.
        type
        Checked;
        /// Check self.
        fn check(self)
        -> Result<Self::Checked, &'static str>;
    }
    impl <T: BlindCheckable, Context> Checkable<Context> for T {
        type
        Checked
        =
        <Self as BlindCheckable>::Checked;
        fn check(self, _c: &Context) -> Result<Self::Checked, &'static str> {
            BlindCheckable::check(self)
        }
    }
    /// An "executable" piece of information, used by the standard Substrate Executive in order to
    /// enact a piece of extrinsic information by marshalling and dispatching to a named function
    /// call.
    ///
    /// Also provides information on to whom this information is attributable and an index that allows
    /// each piece of attributable information to be disambiguated.
    pub trait Applyable: Sized + Send + Sync {
        /// Id of the account that is responsible for this piece of information (sender).
        type
        AccountId: Member +
        MaybeDisplay;
        /// Index allowing to disambiguate other `Applyable`s from the same `AccountId`.
        type
        Index: Member +
        MaybeDisplay +
        SimpleArithmetic;
        /// Function call.
        type
        Call: Member;
        /// Returns a reference to the index if any.
        fn index(&self)
        -> Option<&Self::Index>;
        /// Returns a reference to the sender if any.
        fn sender(&self)
        -> Option<&Self::AccountId>;
        /// Deconstructs into function call and sender.
        fn deconstruct(self)
        -> (Self::Call, Option<Self::AccountId>);
    }
    /// Something that acts like a `Digest` - it can have `Log`s `push`ed onto it and these `Log`s are
    /// each `Codec`.
    pub trait Digest: Member + MaybeSerializeDebugButNotDeserialize +
     Default {
        /// Hash of the items.
        type
        Hash: Member;
        /// Digest item type.
        type
        Item: DigestItem<Hash
        =
        Self::Hash>;
        /// Get reference to all digest items.
        fn logs(&self)
        -> &[Self::Item];
        /// Push new digest item.
        fn push(&mut self, item: Self::Item);
        /// Pop a digest item.
        fn pop(&mut self)
        -> Option<Self::Item>;
        /// Get reference to the first digest item that matches the passed predicate.
        fn log<T: ?Sized, F: Fn(&Self::Item) ->
               Option<&T>>(&self, predicate: F) -> Option<&T> {
            self.logs().iter().filter_map(predicate).next()
        }
    }
    /// Single digest item. Could be any type that implements `Member` and provides methods
    /// for casting member to 'system' log items, known to substrate.
    ///
    /// If the runtime does not supports some 'system' items, use `()` as a stub.
    pub trait DigestItem: Codec + Member +
     MaybeSerializeDebugButNotDeserialize {
        /// `ChangesTrieRoot` payload.
        type
        Hash: Member;
        /// `AuthorityChange` payload.
        type
        AuthorityId: Member +
        MaybeHash +
        crate::codec::Encode +
        crate::codec::Decode;
        /// Returns Some if the entry is the `AuthoritiesChange` entry.
        fn as_authorities_change(&self)
        -> Option<&[Self::AuthorityId]>;
        /// Returns Some if the entry is the `ChangesTrieRoot` entry.
        fn as_changes_trie_root(&self)
        -> Option<&Self::Hash>;
    }
    /// Auxiliary wrapper that holds an api instance and binds it to the given lifetime.
    pub struct ApiRef<'a, T>(T, rstd::marker::PhantomData<&'a ()>);
    impl <'a, T> From<T> for ApiRef<'a, T> {
        fn from(api: T) -> Self { ApiRef(api, Default::default()) }
    }
    impl <'a, T> rstd::ops::Deref for ApiRef<'a, T> {
        type
        Target
        =
        T;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl <'a, T> rstd::ops::DerefMut for ApiRef<'a, T> {
        fn deref_mut(&mut self) -> &mut T { &mut self.0 }
    }
    /// Something that provides a runtime api.
    pub trait ProvideRuntimeApi {
        /// The concrete type that provides the api.
        type
        Api;
        /// Returns the runtime api.
        /// The returned instance will keep track of modifications to the storage. Any successful
        /// call to an api function, will `commit` its changes to an internal buffer. Otherwise,
        /// the modifications will be `discarded`. The modifications will not be applied to the
        /// storage, even on a `commit`.
        fn runtime_api<'a>(&'a self)
        -> ApiRef<'a, Self::Api>;
    }
    /// A marker trait for something that knows the type of the runtime block.
    pub trait GetRuntimeBlockType {
        /// The `RuntimeBlock` type.
        type
        RuntimeBlock: self::Block;
    }
    /// A marker trait for something that knows the type of the node block.
    pub trait GetNodeBlockType {
        /// The `NodeBlock` type.
        type
        NodeBlock: self::Block;
    }
    /// Something that provides information about a runtime api.
    pub trait RuntimeApiInfo {
        /// The identifier of the runtime api.
        const
        ID:
        [u8; 8];
        /// The version of the runtime api.
        const
        VERSION:
        u32;
    }
    /// Something that can validate unsigned extrinsics.
    pub trait ValidateUnsigned {
        /// The call to validate
        type
        Call;
        /// Return the validity of the call
        ///
        /// This doesn't execute any side-effects; it merely checks
        /// whether the transaction would panic if it were included or not.
        ///
        /// Changes made to storage should be discarded by caller.
        fn validate_unsigned(call: &Self::Call)
        -> TransactionValidity;
    }
}
pub mod generic {
    //! Generic implementations of Extrinsic/Header/Block.
    mod unchecked_extrinsic {
        //! Generic implementation of an unchecked (pre-verification) extrinsic.
        #[cfg(feature = "std")]
        use std::fmt;
        use rstd::prelude::*;
        use crate::codec::{Decode, Encode, Codec, Input, HasCompact};
        use crate::traits::{self, Member, SimpleArithmetic, MaybeDisplay,
                            Lookup, Extrinsic};
        use super::CheckedExtrinsic;
        #[structural_match]
        pub struct SignatureContent<Address, Index, Signature> where
                   Address: Codec, Index: HasCompact + Codec,
                   Signature: Codec {
            signed: Address,
            signature: Signature,
            index: Index,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::cmp::PartialEq, Index: ::std::cmp::PartialEq,
              Signature: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
         SignatureContent<Address, Index, Signature> where Address: Codec,
         Index: HasCompact + Codec, Signature: Codec {
            #[inline]
            fn eq(&self, other: &SignatureContent<Address, Index, Signature>)
             -> bool {
                match *other {
                    SignatureContent {
                    signed: ref __self_1_0,
                    signature: ref __self_1_1,
                    index: ref __self_1_2 } =>
                    match *self {
                        SignatureContent {
                        signed: ref __self_0_0,
                        signature: ref __self_0_1,
                        index: ref __self_0_2 } =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &SignatureContent<Address, Index, Signature>)
             -> bool {
                match *other {
                    SignatureContent {
                    signed: ref __self_1_0,
                    signature: ref __self_1_1,
                    index: ref __self_1_2 } =>
                    match *self {
                        SignatureContent {
                        signed: ref __self_0_0,
                        signature: ref __self_0_1,
                        index: ref __self_0_2 } =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::cmp::Eq, Index: ::std::cmp::Eq,
              Signature: ::std::cmp::Eq> ::std::cmp::Eq for
         SignatureContent<Address, Index, Signature> where Address: Codec,
         Index: HasCompact + Codec, Signature: Codec {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Address>;
                    let _: ::std::cmp::AssertParamIsEq<Signature>;
                    let _: ::std::cmp::AssertParamIsEq<Index>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::clone::Clone, Index: ::std::clone::Clone,
              Signature: ::std::clone::Clone> ::std::clone::Clone for
         SignatureContent<Address, Index, Signature> where Address: Codec,
         Index: HasCompact + Codec, Signature: Codec {
            #[inline]
            fn clone(&self) -> SignatureContent<Address, Index, Signature> {
                match *self {
                    SignatureContent {
                    signed: ref __self_0_0,
                    signature: ref __self_0_1,
                    index: ref __self_0_2 } =>
                    SignatureContent{signed:
                                         ::std::clone::Clone::clone(&(*__self_0_0)),
                                     signature:
                                         ::std::clone::Clone::clone(&(*__self_0_1)),
                                     index:
                                         ::std::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_ENCODE_FOR_SignatureContent: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl <Address, Index, Signature> _parity_codec::Encode for
                 SignatureContent<Address, Index, Signature> where
                 Address: Codec, Index: HasCompact + Codec, Signature: Codec,
                 Address: _parity_codec::Encode,
                 Address: _parity_codec::Encode,
                 Signature: _parity_codec::Encode,
                 Signature: _parity_codec::Encode,
                 Index: _parity_codec::Encode, Index: _parity_codec::Encode {
                    fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                                dest:
                                                                    &mut EncOut) {
                        dest.push(&self.signed);
                        dest.push(&self.signature);
                        dest.push(&self.index);
                    }
                }
            };
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_DECODE_FOR_SignatureContent: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl <Address, Index, Signature> _parity_codec::Decode for
                 SignatureContent<Address, Index, Signature> where
                 Address: Codec, Index: HasCompact + Codec, Signature: Codec,
                 Address: _parity_codec::Decode,
                 Address: _parity_codec::Decode,
                 Signature: _parity_codec::Decode,
                 Signature: _parity_codec::Decode,
                 Index: _parity_codec::Decode, Index: _parity_codec::Decode {
                    fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                     -> Option<Self> {
                        Some(SignatureContent{signed:
                                                  _parity_codec::Decode::decode(input)?,
                                              signature:
                                                  _parity_codec::Decode::decode(input)?,
                                              index:
                                                  _parity_codec::Decode::decode(input)?,})
                    }
                }
            };
        /// A extrinsic right from the external world. This is unchecked and so
        /// can contain a signature.
        #[structural_match]
        pub struct UncheckedExtrinsic<Address, Index, Call, Signature> where
                   Address: Codec, Index: HasCompact + Codec,
                   Signature: Codec {
            /// The signature, address and number of extrinsics have come before from
            /// the same signer, if this is a signed extrinsic.
            pub signature: Option<SignatureContent<Address, Index,
                                                   Signature>>,
            /// The function that should be called.
            pub function: Call,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::cmp::PartialEq, Index: ::std::cmp::PartialEq,
              Call: ::std::cmp::PartialEq, Signature: ::std::cmp::PartialEq>
         ::std::cmp::PartialEq for
         UncheckedExtrinsic<Address, Index, Call, Signature> where
         Address: Codec, Index: HasCompact + Codec, Signature: Codec {
            #[inline]
            fn eq(&self,
                  other: &UncheckedExtrinsic<Address, Index, Call, Signature>)
             -> bool {
                match *other {
                    UncheckedExtrinsic {
                    signature: ref __self_1_0, function: ref __self_1_1 } =>
                    match *self {
                        UncheckedExtrinsic {
                        signature: ref __self_0_0, function: ref __self_0_1 }
                        =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self,
                  other: &UncheckedExtrinsic<Address, Index, Call, Signature>)
             -> bool {
                match *other {
                    UncheckedExtrinsic {
                    signature: ref __self_1_0, function: ref __self_1_1 } =>
                    match *self {
                        UncheckedExtrinsic {
                        signature: ref __self_0_0, function: ref __self_0_1 }
                        =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::cmp::Eq, Index: ::std::cmp::Eq,
              Call: ::std::cmp::Eq, Signature: ::std::cmp::Eq> ::std::cmp::Eq
         for UncheckedExtrinsic<Address, Index, Call, Signature> where
         Address: Codec, Index: HasCompact + Codec, Signature: Codec {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _:
                            ::std::cmp::AssertParamIsEq<Option<SignatureContent<Address,
                                                                                Index,
                                                                                Signature>>>;
                    let _: ::std::cmp::AssertParamIsEq<Call>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::clone::Clone, Index: ::std::clone::Clone,
              Call: ::std::clone::Clone, Signature: ::std::clone::Clone>
         ::std::clone::Clone for
         UncheckedExtrinsic<Address, Index, Call, Signature> where
         Address: Codec, Index: HasCompact + Codec, Signature: Codec {
            #[inline]
            fn clone(&self)
             -> UncheckedExtrinsic<Address, Index, Call, Signature> {
                match *self {
                    UncheckedExtrinsic {
                    signature: ref __self_0_0, function: ref __self_0_1 } =>
                    UncheckedExtrinsic{signature:
                                           ::std::clone::Clone::clone(&(*__self_0_0)),
                                       function:
                                           ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        impl <Address, Index, Signature, Call>
         UncheckedExtrinsic<Address, Index, Call, Signature> where
         Address: Codec, Index: HasCompact + Codec, Signature: Codec {
            /// New instance of a signed extrinsic aka "transaction".
            pub fn new_signed(index: Index, function: Call, signed: Address,
                              signature: Signature) -> Self {
                UncheckedExtrinsic{signature:
                                       Some(SignatureContent{signed,
                                                             signature,
                                                             index,}),
                                   function,}
            }
            /// New instance of an unsigned extrinsic aka "inherent".
            pub fn new_unsigned(function: Call) -> Self {
                UncheckedExtrinsic{signature: None, function,}
            }
        }
        impl <Address, Index, Signature, Call, AccountId, Context>
         traits::Checkable<Context> for
         UncheckedExtrinsic<Address, Index, Call, Signature> where
         Address: Member + MaybeDisplay + Codec, Index: Member +
         MaybeDisplay + SimpleArithmetic + Codec, Call: Encode + Member,
         Signature: Member + traits::Verify<Signer = AccountId> + Codec,
         AccountId: Member + MaybeDisplay, Context: Lookup<Source = Address,
         Target = AccountId> {
            type
            Checked
            =
            CheckedExtrinsic<AccountId, Index, Call>;
            fn check(self, context: &Context)
             -> Result<Self::Checked, &'static str> {
                Ok(match self.signature {
                       Some(SignatureContent { signed, signature, index }) =>
                       {
                           let payload = (index, self.function);
                           let signed = context.lookup(signed)?;
                           if !crate::verify_encoded_lazy(&signature,
                                                          &payload, &signed) {
                               return Err(crate::BAD_SIGNATURE)
                           }
                           CheckedExtrinsic{signed: Some((signed, payload.0)),
                                            function: payload.1,}
                       }
                       None =>
                       CheckedExtrinsic{signed: None,
                                        function: self.function,},
                   })
            }
        }
        impl <Address: Codec, Index: HasCompact + Codec, Signature: Codec,
              Call> Extrinsic for
         UncheckedExtrinsic<Address, Index, Call, Signature> {
            fn is_signed(&self) -> Option<bool> {
                Some(self.signature.is_some())
            }
        }
        impl <Address: Codec, Index: HasCompact + Codec, Signature: Codec,
              Call: Decode> Decode for
         UncheckedExtrinsic<Address, Index, Call, Signature> {
            fn decode<I: Input>(input: &mut I) -> Option<Self> {
                let _length_do_not_remove_me_see_above: Vec<()> =
                    Decode::decode(input)?;
                Some(UncheckedExtrinsic{signature: Decode::decode(input)?,
                                        function: Decode::decode(input)?,})
            }
        }
        impl <Address: Codec, Index: HasCompact + Codec, Signature: Codec,
              Call: Encode> Encode for
         UncheckedExtrinsic<Address, Index, Call, Signature> {
            fn encode(&self) -> Vec<u8> {
                super::encode_with_vec_prefix::<Self,
                                                _>(|v|
                                                       {
                                                           self.signature.encode_to(v);
                                                           self.function.encode_to(v);
                                                       })
            }
        }
        #[cfg(feature = "std")]
        impl <Address: Codec, Index: HasCompact + Codec, Signature: Codec,
              Call: Encode> serde::Serialize for
         UncheckedExtrinsic<Address, Index, Call, Signature> {
            fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error> where
             S: ::serde::Serializer {
                self.using_encoded(|bytes|
                                       ::substrate_primitives::bytes::serialize(bytes,
                                                                                seq))
            }
        }
        #[cfg(feature = "std")]
        impl <Address, Index, Signature, Call> fmt::Debug for
         UncheckedExtrinsic<Address, Index, Call, Signature> where
         Address: fmt::Debug + Codec, Index: fmt::Debug + HasCompact + Codec,
         Signature: Codec, Call: fmt::Debug {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1(&["UncheckedExtrinsic(",
                                                            ", ", ")"],
                                                          &match (&self.signature.as_ref().map(|x|
                                                                                                   (&x.signed,
                                                                                                    &x.index)),
                                                                  &self.function)
                                                               {
                                                               (arg0, arg1) =>
                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                            ::std::fmt::Debug::fmt),
                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                            ::std::fmt::Debug::fmt)],
                                                           }))
            }
        }
    }
    mod unchecked_mortal_extrinsic {
        //! Generic implementation of an unchecked (pre-verification) extrinsic.
        #[cfg(feature = "std")]
        use std::fmt;
        use rstd::prelude::*;
        use runtime_io::blake2_256;
        use crate::codec::{Decode, Encode, Input};
        use crate::traits::{self, Member, SimpleArithmetic, MaybeDisplay,
                            CurrentHeight, BlockNumberToHash, Lookup,
                            Checkable, Extrinsic};
        use super::{CheckedExtrinsic, Era};
        const TRANSACTION_VERSION: u8 = 1;
        /// A extrinsic right from the external world. This is unchecked and so
        /// can contain a signature.
        #[structural_match]
        pub struct UncheckedMortalExtrinsic<Address, Index, Call, Signature> {
            /// The signature, address, number of extrinsics have come before from
            /// the same signer and an era describing the longevity of this transaction,
            /// if this is a signed extrinsic.
            pub signature: Option<(Address, Signature, Index, Era)>,
            /// The function that should be called.
            pub function: Call,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::cmp::PartialEq, Index: ::std::cmp::PartialEq,
              Call: ::std::cmp::PartialEq, Signature: ::std::cmp::PartialEq>
         ::std::cmp::PartialEq for
         UncheckedMortalExtrinsic<Address, Index, Call, Signature> {
            #[inline]
            fn eq(&self,
                  other:
                      &UncheckedMortalExtrinsic<Address, Index, Call,
                                                Signature>) -> bool {
                match *other {
                    UncheckedMortalExtrinsic {
                    signature: ref __self_1_0, function: ref __self_1_1 } =>
                    match *self {
                        UncheckedMortalExtrinsic {
                        signature: ref __self_0_0, function: ref __self_0_1 }
                        =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self,
                  other:
                      &UncheckedMortalExtrinsic<Address, Index, Call,
                                                Signature>) -> bool {
                match *other {
                    UncheckedMortalExtrinsic {
                    signature: ref __self_1_0, function: ref __self_1_1 } =>
                    match *self {
                        UncheckedMortalExtrinsic {
                        signature: ref __self_0_0, function: ref __self_0_1 }
                        =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::cmp::Eq, Index: ::std::cmp::Eq,
              Call: ::std::cmp::Eq, Signature: ::std::cmp::Eq> ::std::cmp::Eq
         for UncheckedMortalExtrinsic<Address, Index, Call, Signature> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _:
                            ::std::cmp::AssertParamIsEq<Option<(Address,
                                                                Signature,
                                                                Index, Era)>>;
                    let _: ::std::cmp::AssertParamIsEq<Call>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::clone::Clone, Index: ::std::clone::Clone,
              Call: ::std::clone::Clone, Signature: ::std::clone::Clone>
         ::std::clone::Clone for
         UncheckedMortalExtrinsic<Address, Index, Call, Signature> {
            #[inline]
            fn clone(&self)
             -> UncheckedMortalExtrinsic<Address, Index, Call, Signature> {
                match *self {
                    UncheckedMortalExtrinsic {
                    signature: ref __self_0_0, function: ref __self_0_1 } =>
                    UncheckedMortalExtrinsic{signature:
                                                 ::std::clone::Clone::clone(&(*__self_0_0)),
                                             function:
                                                 ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        impl <Address, Index, Call, Signature>
         UncheckedMortalExtrinsic<Address, Index, Call, Signature> {
            /// New instance of a signed extrinsic aka "transaction".
            pub fn new_signed(index: Index, function: Call, signed: Address,
                              signature: Signature, era: Era) -> Self {
                UncheckedMortalExtrinsic{signature:
                                             Some((signed, signature, index,
                                                   era)),
                                         function,}
            }
            /// New instance of an unsigned extrinsic aka "inherent".
            pub fn new_unsigned(function: Call) -> Self {
                UncheckedMortalExtrinsic{signature: None, function,}
            }
        }
        impl <Address: Encode, Index: Encode, Call: Encode, Signature: Encode>
         Extrinsic for
         UncheckedMortalExtrinsic<Address, Index, Call, Signature> {
            fn is_signed(&self) -> Option<bool> {
                Some(self.signature.is_some())
            }
        }
        impl <Address, AccountId, Index, Call, Signature, Context, Hash,
              BlockNumber> Checkable<Context> for
         UncheckedMortalExtrinsic<Address, Index, Call, Signature> where
         Address: Member + MaybeDisplay, Index: Encode + Member +
         MaybeDisplay + SimpleArithmetic, Call: Encode + Member,
         Signature: Member + traits::Verify<Signer = AccountId>,
         AccountId: Member + MaybeDisplay, BlockNumber: SimpleArithmetic,
         Hash: Encode, Context: Lookup<Source = Address, Target = AccountId> +
         CurrentHeight<BlockNumber = BlockNumber> +
         BlockNumberToHash<BlockNumber = BlockNumber, Hash = Hash> {
            type
            Checked
            =
            CheckedExtrinsic<AccountId, Index, Call>;
            fn check(self, context: &Context)
             -> Result<Self::Checked, &'static str> {
                Ok(match self.signature {
                       Some((signed, signature, index, era)) => {
                           let h =
                               context.block_number_to_hash(BlockNumber::sa(era.birth(context.current_height().as_()))).ok_or("transaction birth block ancient")?;
                           let signed = context.lookup(signed)?;
                           let raw_payload = (index, self.function, era, h);
                           if !raw_payload.using_encoded(|payload|
                                                             {
                                                                 if payload.len()
                                                                        > 256
                                                                    {
                                                                     signature.verify(&blake2_256(payload)[..],
                                                                                      &signed)
                                                                 } else {
                                                                     signature.verify(payload,
                                                                                      &signed)
                                                                 }
                                                             }) {
                               return Err(crate::BAD_SIGNATURE)
                           }
                           CheckedExtrinsic{signed:
                                                Some((signed, raw_payload.0)),
                                            function: raw_payload.1,}
                       }
                       None =>
                       CheckedExtrinsic{signed: None,
                                        function: self.function,},
                   })
            }
        }
        impl <Address, Index, Call, Signature> Decode for
         UncheckedMortalExtrinsic<Address, Index, Call, Signature> where
         Address: Decode, Signature: Decode, Index: Decode, Call: Decode {
            fn decode<I: Input>(input: &mut I) -> Option<Self> {
                let _length_do_not_remove_me_see_above: Vec<()> =
                    Decode::decode(input)?;
                let version = input.read_byte()?;
                let is_signed = version & 0b1000_0000 != 0;
                let version = version & 0b0111_1111;
                if version != TRANSACTION_VERSION { return None }
                Some(UncheckedMortalExtrinsic{signature:
                                                  if is_signed {
                                                      Some(Decode::decode(input)?)
                                                  } else { None },
                                              function:
                                                  Decode::decode(input)?,})
            }
        }
        impl <Address, Index, Call, Signature> Encode for
         UncheckedMortalExtrinsic<Address, Index, Call, Signature> where
         Address: Encode, Signature: Encode, Index: Encode, Call: Encode {
            fn encode(&self) -> Vec<u8> {
                super::encode_with_vec_prefix::<Self,
                                                _>(|v|
                                                       {
                                                           match self.signature.as_ref()
                                                               {
                                                               Some(s) => {
                                                                   v.push(TRANSACTION_VERSION
                                                                              |
                                                                              0b1000_0000);
                                                                   s.encode_to(v);
                                                               }
                                                               None => {
                                                                   v.push(TRANSACTION_VERSION
                                                                              &
                                                                              0b0111_1111);
                                                               }
                                                           }
                                                           self.function.encode_to(v);
                                                       })
            }
        }
        #[cfg(feature = "std")]
        impl <Address: Encode, Index: Encode, Signature: Encode, Call: Encode>
         serde::Serialize for
         UncheckedMortalExtrinsic<Address, Index, Call, Signature> {
            fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error> where
             S: ::serde::Serializer {
                self.using_encoded(|bytes| seq.serialize_bytes(bytes))
            }
        }
        #[cfg(feature = "std")]
        impl <Address, Index, Call, Signature> fmt::Debug for
         UncheckedMortalExtrinsic<Address, Index, Call, Signature> where
         Address: fmt::Debug, Index: fmt::Debug, Call: fmt::Debug {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1(&["UncheckedMortalExtrinsic(",
                                                            ", ", ")"],
                                                          &match (&self.signature.as_ref().map(|x|
                                                                                                   (&x.0,
                                                                                                    &x.2)),
                                                                  &self.function)
                                                               {
                                                               (arg0, arg1) =>
                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                            ::std::fmt::Debug::fmt),
                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                            ::std::fmt::Debug::fmt)],
                                                           }))
            }
        }
    }
    mod unchecked_mortal_compact_extrinsic {
        //! Generic implementation of an unchecked (pre-verification) extrinsic.
        #[cfg(feature = "std")]
        use std::fmt;
        use rstd::prelude::*;
        use runtime_io::blake2_256;
        use crate::codec::{Decode, Encode, Input, Compact};
        use crate::traits::{self, Member, SimpleArithmetic, MaybeDisplay,
                            CurrentHeight, BlockNumberToHash, Lookup,
                            Checkable, Extrinsic};
        use super::{CheckedExtrinsic, Era};
        const TRANSACTION_VERSION: u8 = 1;
        /// A extrinsic right from the external world. This is unchecked and so
        /// can contain a signature.
        #[structural_match]
        pub struct UncheckedMortalCompactExtrinsic<Address, Index, Call,
                                                   Signature> {
            /// The signature, address, number of extrinsics have come before from
            /// the same signer and an era describing the longevity of this transaction,
            /// if this is a signed extrinsic.
            pub signature: Option<(Address, Signature, Compact<Index>, Era)>,
            /// The function that should be called.
            pub function: Call,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::cmp::PartialEq, Index: ::std::cmp::PartialEq,
              Call: ::std::cmp::PartialEq, Signature: ::std::cmp::PartialEq>
         ::std::cmp::PartialEq for
         UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature> {
            #[inline]
            fn eq(&self,
                  other:
                      &UncheckedMortalCompactExtrinsic<Address, Index, Call,
                                                       Signature>) -> bool {
                match *other {
                    UncheckedMortalCompactExtrinsic {
                    signature: ref __self_1_0, function: ref __self_1_1 } =>
                    match *self {
                        UncheckedMortalCompactExtrinsic {
                        signature: ref __self_0_0, function: ref __self_0_1 }
                        =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self,
                  other:
                      &UncheckedMortalCompactExtrinsic<Address, Index, Call,
                                                       Signature>) -> bool {
                match *other {
                    UncheckedMortalCompactExtrinsic {
                    signature: ref __self_1_0, function: ref __self_1_1 } =>
                    match *self {
                        UncheckedMortalCompactExtrinsic {
                        signature: ref __self_0_0, function: ref __self_0_1 }
                        =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::cmp::Eq, Index: ::std::cmp::Eq,
              Call: ::std::cmp::Eq, Signature: ::std::cmp::Eq> ::std::cmp::Eq
         for UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature>
         {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _:
                            ::std::cmp::AssertParamIsEq<Option<(Address,
                                                                Signature,
                                                                Compact<Index>,
                                                                Era)>>;
                    let _: ::std::cmp::AssertParamIsEq<Call>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Address: ::std::clone::Clone, Index: ::std::clone::Clone,
              Call: ::std::clone::Clone, Signature: ::std::clone::Clone>
         ::std::clone::Clone for
         UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature> {
            #[inline]
            fn clone(&self)
             ->
                 UncheckedMortalCompactExtrinsic<Address, Index, Call,
                                                 Signature> {
                match *self {
                    UncheckedMortalCompactExtrinsic {
                    signature: ref __self_0_0, function: ref __self_0_1 } =>
                    UncheckedMortalCompactExtrinsic{signature:
                                                        ::std::clone::Clone::clone(&(*__self_0_0)),
                                                    function:
                                                        ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        impl <Address, Index, Call, Signature>
         UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature> {
            /// New instance of a signed extrinsic aka "transaction".
            pub fn new_signed(index: Index, function: Call, signed: Address,
                              signature: Signature, era: Era) -> Self {
                UncheckedMortalCompactExtrinsic{signature:
                                                    Some((signed, signature,
                                                          index.into(), era)),
                                                function,}
            }
            /// New instance of an unsigned extrinsic aka "inherent".
            pub fn new_unsigned(function: Call) -> Self {
                UncheckedMortalCompactExtrinsic{signature: None, function,}
            }
        }
        impl <Address: Encode, Index: Encode, Call: Encode, Signature: Encode>
         Extrinsic for
         UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature> {
            fn is_signed(&self) -> Option<bool> {
                Some(self.signature.is_some())
            }
        }
        impl <Address, AccountId, Index, Call, Signature, Context, Hash,
              BlockNumber> Checkable<Context> for
         UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature>
         where Address: Member + MaybeDisplay, Index: Member + MaybeDisplay +
         SimpleArithmetic, Compact<Index>: Encode, Call: Encode + Member,
         Signature: Member + traits::Verify<Signer = AccountId>,
         AccountId: Member + MaybeDisplay, BlockNumber: SimpleArithmetic,
         Hash: Encode, Context: Lookup<Source = Address, Target = AccountId> +
         CurrentHeight<BlockNumber = BlockNumber> +
         BlockNumberToHash<BlockNumber = BlockNumber, Hash = Hash> {
            type
            Checked
            =
            CheckedExtrinsic<AccountId, Index, Call>;
            fn check(self, context: &Context)
             -> Result<Self::Checked, &'static str> {
                Ok(match self.signature {
                       Some((signed, signature, index, era)) => {
                           let h =
                               context.block_number_to_hash(BlockNumber::sa(era.birth(context.current_height().as_()))).ok_or("transaction birth block ancient")?;
                           let signed = context.lookup(signed)?;
                           let raw_payload = (index, self.function, era, h);
                           if !raw_payload.using_encoded(|payload|
                                                             {
                                                                 if payload.len()
                                                                        > 256
                                                                    {
                                                                     signature.verify(&blake2_256(payload)[..],
                                                                                      &signed)
                                                                 } else {
                                                                     signature.verify(payload,
                                                                                      &signed)
                                                                 }
                                                             }) {
                               return Err(crate::BAD_SIGNATURE)
                           }
                           CheckedExtrinsic{signed:
                                                Some((signed,
                                                      (raw_payload.0).0)),
                                            function: raw_payload.1,}
                       }
                       None =>
                       CheckedExtrinsic{signed: None,
                                        function: self.function,},
                   })
            }
        }
        impl <Address, Index, Call, Signature> Decode for
         UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature>
         where Address: Decode, Signature: Decode, Compact<Index>: Decode,
         Call: Decode {
            fn decode<I: Input>(input: &mut I) -> Option<Self> {
                let _length_do_not_remove_me_see_above: Vec<()> =
                    Decode::decode(input)?;
                let version = input.read_byte()?;
                let is_signed = version & 0b1000_0000 != 0;
                let version = version & 0b0111_1111;
                if version != TRANSACTION_VERSION { return None }
                Some(UncheckedMortalCompactExtrinsic{signature:
                                                         if is_signed {
                                                             Some(Decode::decode(input)?)
                                                         } else { None },
                                                     function:
                                                         Decode::decode(input)?,})
            }
        }
        impl <Address, Index, Call, Signature> Encode for
         UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature>
         where Address: Encode, Signature: Encode, Compact<Index>: Encode,
         Call: Encode {
            fn encode(&self) -> Vec<u8> {
                super::encode_with_vec_prefix::<Self,
                                                _>(|v|
                                                       {
                                                           match self.signature.as_ref()
                                                               {
                                                               Some(s) => {
                                                                   v.push(TRANSACTION_VERSION
                                                                              |
                                                                              0b1000_0000);
                                                                   s.encode_to(v);
                                                               }
                                                               None => {
                                                                   v.push(TRANSACTION_VERSION
                                                                              &
                                                                              0b0111_1111);
                                                               }
                                                           }
                                                           self.function.encode_to(v);
                                                       })
            }
        }
        #[cfg(feature = "std")]
        impl <Address: Encode, Index, Signature: Encode, Call: Encode>
         serde::Serialize for
         UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature>
         where Compact<Index>: Encode {
            fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error> where
             S: ::serde::Serializer {
                self.using_encoded(|bytes| seq.serialize_bytes(bytes))
            }
        }
        #[cfg(feature = "std")]
        impl <Address, Index, Call, Signature> fmt::Debug for
         UncheckedMortalCompactExtrinsic<Address, Index, Call, Signature>
         where Address: fmt::Debug, Index: fmt::Debug, Call: fmt::Debug {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1(&["UncheckedMortalCompactExtrinsic(",
                                                            ", ", ")"],
                                                          &match (&self.signature.as_ref().map(|x|
                                                                                                   (&x.0,
                                                                                                    &x.2)),
                                                                  &self.function)
                                                               {
                                                               (arg0, arg1) =>
                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                            ::std::fmt::Debug::fmt),
                                                                ::std::fmt::ArgumentV1::new(arg1,
                                                                                            ::std::fmt::Debug::fmt)],
                                                           }))
            }
        }
    }
    mod era {
        //! Generic implementation of an unchecked (pre-verification) extrinsic.
        #[cfg(feature = "std")]
        use serde::{Serialize, Deserialize};
        use crate::codec::{Decode, Encode, Input, Output};
        pub type Period = u64;
        pub type Phase = u64;
        /// An era to describe the longevity of a transaction.
        #[structural_match]
        #[rustc_copy_clone_marker]
        pub enum Era {

            /// The transaction is valid forever. The genesis hash must be present in the signed content.
            Immortal,

            /// Period and phase are encoded:
            /// - The period of validity from the block hash found in the signing material.
            /// - The phase in the period that this transaction's lifetime begins (and, importantly,
            /// implies which block hash is included in the signature material). If the `period` is
            /// greater than 1 << 12, then it will be a factor of the times greater than 1<<12 that
            /// `period` is.
            Mortal(Period, Phase),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for Era {
            #[inline]
            fn eq(&self, other: &Era) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*other)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Era::Mortal(ref __self_0, ref __self_1),
                             &Era::Mortal(ref __arg_1_0, ref __arg_1_1)) =>
                            (*__self_0) == (*__arg_1_0) &&
                                (*__self_1) == (*__arg_1_1),
                            _ => true,
                        }
                    } else { false }
                }
            }
            #[inline]
            fn ne(&self, other: &Era) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*other)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Era::Mortal(ref __self_0, ref __self_1),
                             &Era::Mortal(ref __arg_1_0, ref __arg_1_1)) =>
                            (*__self_0) != (*__arg_1_0) ||
                                (*__self_1) != (*__arg_1_1),
                            _ => false,
                        }
                    } else { true }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::Eq for Era {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Period>;
                    let _: ::std::cmp::AssertParamIsEq<Phase>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Era {
            #[inline]
            fn clone(&self) -> Era {
                {
                    let _: ::std::clone::AssertParamIsClone<Period>;
                    let _: ::std::clone::AssertParamIsClone<Phase>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::marker::Copy for Era { }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Era: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
                #[automatically_derived]
                impl _serde::Serialize for Era {
                    fn serialize<__S>(&self, __serializer: __S)
                     -> _serde::export::Result<__S::Ok, __S::Error> where
                     __S: _serde::Serializer {
                        match *self {
                            Era::Immortal =>
                            _serde::Serializer::serialize_unit_variant(__serializer,
                                                                       "Era",
                                                                       0u32,
                                                                       "Immortal"),
                            Era::Mortal(ref __field0, ref __field1) => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_tuple_variant(__serializer,
                                                                                      "Era",
                                                                                      1u32,
                                                                                      "Mortal",
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
                        }
                    }
                }
            };
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_Era: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
                #[automatically_derived]
                impl <'de> _serde::Deserialize<'de> for Era {
                    fn deserialize<__D>(__deserializer: __D)
                     -> _serde::export::Result<Self, __D::Error> where
                     __D: _serde::Deserializer<'de> {
                        #[allow(non_camel_case_types)]
                        enum __Field { __field0, __field1, }
                        struct __FieldVisitor;
                        impl <'de> _serde::de::Visitor<'de> for __FieldVisitor
                         {
                            type
                            Value
                            =
                            __Field;
                            fn expecting(&self,
                                         __formatter:
                                             &mut _serde::export::Formatter)
                             -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(__formatter,
                                                                     "variant identifier")
                            }
                            fn visit_u64<__E>(self, __value: u64)
                             -> _serde::export::Result<Self::Value, __E> where
                             __E: _serde::de::Error {
                                match __value {
                                    0u64 =>
                                    _serde::export::Ok(__Field::__field0),
                                    1u64 =>
                                    _serde::export::Ok(__Field::__field1),
                                    _ =>
                                    _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                         &"variant index 0 <= i < 2")),
                                }
                            }
                            fn visit_str<__E>(self, __value: &str)
                             -> _serde::export::Result<Self::Value, __E> where
                             __E: _serde::de::Error {
                                match __value {
                                    "Immortal" =>
                                    _serde::export::Ok(__Field::__field0),
                                    "Mortal" =>
                                    _serde::export::Ok(__Field::__field1),
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
                                    b"Immortal" =>
                                    _serde::export::Ok(__Field::__field0),
                                    b"Mortal" =>
                                    _serde::export::Ok(__Field::__field1),
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
                        struct __Visitor<'de> {
                            marker: _serde::export::PhantomData<Era>,
                            lifetime: _serde::export::PhantomData<&'de ()>,
                        }
                        impl <'de> _serde::de::Visitor<'de> for __Visitor<'de>
                         {
                            type
                            Value
                            =
                            Era;
                            fn expecting(&self,
                                         __formatter:
                                             &mut _serde::export::Formatter)
                             -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(__formatter,
                                                                     "enum Era")
                            }
                            fn visit_enum<__A>(self, __data: __A)
                             ->
                                 _serde::export::Result<Self::Value,
                                                        __A::Error> where
                             __A: _serde::de::EnumAccess<'de> {
                                match match _serde::de::EnumAccess::variant(__data)
                                          {
                                          _serde::export::Ok(__val) => __val,
                                          _serde::export::Err(__err) => {
                                              return _serde::export::Err(__err);
                                          }
                                      } {
                                    (__Field::__field0, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant)
                                            {
                                            _serde::export::Ok(__val) =>
                                            __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        _serde::export::Ok(Era::Immortal)
                                    }
                                    (__Field::__field1, __variant) => {
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<Era>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl <'de> _serde::de::Visitor<'de>
                                         for __Visitor<'de> {
                                            type
                                            Value
                                            =
                                            Era;
                                            fn expecting(&self,
                                                         __formatter:
                                                             &mut _serde::export::Formatter)
                                             -> _serde::export::fmt::Result {
                                                _serde::export::Formatter::write_str(__formatter,
                                                                                     "tuple variant Era::Mortal")
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(self,
                                                              mut __seq: __A)
                                             ->
                                                 _serde::export::Result<Self::Value,
                                                                        __A::Error>
                                             where
                                             __A: _serde::de::SeqAccess<'de> {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<Period>(&mut __seq)
                                                              {
                                                              _serde::export::Ok(__val)
                                                              => __val,
                                                              _serde::export::Err(__err)
                                                              => {
                                                                  return _serde::export::Err(__err);
                                                              }
                                                          } {
                                                        _serde::export::Some(__value)
                                                        => __value,
                                                        _serde::export::None
                                                        => {
                                                            return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                                         &"tuple variant Era::Mortal with 2 elements"));
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<Phase>(&mut __seq)
                                                              {
                                                              _serde::export::Ok(__val)
                                                              => __val,
                                                              _serde::export::Err(__err)
                                                              => {
                                                                  return _serde::export::Err(__err);
                                                              }
                                                          } {
                                                        _serde::export::Some(__value)
                                                        => __value,
                                                        _serde::export::None
                                                        => {
                                                            return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                                         &"tuple variant Era::Mortal with 2 elements"));
                                                        }
                                                    };
                                                _serde::export::Ok(Era::Mortal(__field0,
                                                                               __field1))
                                            }
                                        }
                                        _serde::de::VariantAccess::tuple_variant(__variant,
                                                                                 2usize,
                                                                                 __Visitor{marker:
                                                                                               _serde::export::PhantomData::<Era>,
                                                                                           lifetime:
                                                                                               _serde::export::PhantomData,})
                                    }
                                }
                            }
                        }
                        const VARIANTS: &'static [&'static str] =
                            &["Immortal", "Mortal"];
                        _serde::Deserializer::deserialize_enum(__deserializer,
                                                               "Era",
                                                               VARIANTS,
                                                               __Visitor{marker:
                                                                             _serde::export::PhantomData::<Era>,
                                                                         lifetime:
                                                                             _serde::export::PhantomData,})
                    }
                }
            };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Era {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&Era::Immortal,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Immortal");
                        debug_trait_builder.finish()
                    }
                    (&Era::Mortal(ref __self_0, ref __self_1),) => {
                        let mut debug_trait_builder = f.debug_tuple("Mortal");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl Era {
            /// Create a new era based on a period (which should be a power of two between 4 and 65536 inclusive)
            /// and a block number on which it should start (or, for long periods, be shortly after the start).
            pub fn mortal(period: u64, current: u64) -> Self {
                let period =
                    period.checked_next_power_of_two().unwrap_or(1 <<
                                                                     16).max(4).min(1
                                                                                        <<
                                                                                        16);
                let phase = current % period;
                let quantize_factor = (period >> 12).max(1);
                let quantized_phase =
                    phase / quantize_factor * quantize_factor;
                Era::Mortal(period, quantized_phase)
            }
            /// Create an "immortal" transaction.
            pub fn immortal() -> Self { Era::Immortal }
            /// `true` if this is an immortal transaction.
            pub fn is_immortal(&self) -> bool {
                match self { Era::Immortal => true, _ => false, }
            }
            /// Get the block number of the start of the era whose properties this object
            /// describes that `current` belongs to.
            pub fn birth(self, current: u64) -> u64 {
                match self {
                    Era::Immortal => 0,
                    Era::Mortal(period, phase) =>
                    (current.max(phase) - phase) / period * period + phase,
                }
            }
            /// Get the block number of the first block at which the era has ended.
            pub fn death(self, current: u64) -> u64 {
                match self {
                    Era::Immortal => u64::max_value(),
                    Era::Mortal(period, _) => self.birth(current) + period,
                }
            }
        }
        impl Encode for Era {
            fn encode_to<T: Output>(&self, output: &mut T) {
                match self {
                    Era::Immortal => output.push_byte(0),
                    Era::Mortal(period, phase) => {
                        let quantize_factor = (*period as u64 >> 12).max(1);
                        let encoded =
                            (period.trailing_zeros() - 1).max(1).min(15) as
                                u16 | ((phase / quantize_factor) << 4) as u16;
                        output.push(&encoded);
                    }
                }
            }
        }
        impl Decode for Era {
            fn decode<I: Input>(input: &mut I) -> Option<Self> {
                let first = input.read_byte()?;
                if first == 0 {
                    Some(Era::Immortal)
                } else {
                    let encoded =
                        first as u64 + ((input.read_byte()? as u64) << 8);
                    let period = 2 << (encoded % (1 << 4));
                    let quantize_factor = (period >> 12).max(1);
                    let phase = (encoded >> 4) * quantize_factor;
                    if period >= 4 && phase < period {
                        Some(Era::Mortal(period, phase))
                    } else { None }
                }
            }
        }
    }
    mod checked_extrinsic {
        //! Generic implementation of an extrinsic that has passed the verification
        //! stage.
        use crate::traits::{self, Member, SimpleArithmetic, MaybeDisplay};
        /// Definition of something that the external world might want to say; its
        /// existence implies that it has been checked and is good, particularly with
        /// regards to the signature.
        #[structural_match]
        pub struct CheckedExtrinsic<AccountId, Index, Call> {
            /// Who this purports to be from and the number of extrinsics have come before
            /// from the same signer, if anyone (note this is not a signature).
            pub signed: Option<(AccountId, Index)>,
            /// The function that should be called.
            pub function: Call,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <AccountId: ::std::cmp::PartialEq, Index: ::std::cmp::PartialEq,
              Call: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
         CheckedExtrinsic<AccountId, Index, Call> {
            #[inline]
            fn eq(&self, other: &CheckedExtrinsic<AccountId, Index, Call>)
             -> bool {
                match *other {
                    CheckedExtrinsic {
                    signed: ref __self_1_0, function: ref __self_1_1 } =>
                    match *self {
                        CheckedExtrinsic {
                        signed: ref __self_0_0, function: ref __self_0_1 } =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &CheckedExtrinsic<AccountId, Index, Call>)
             -> bool {
                match *other {
                    CheckedExtrinsic {
                    signed: ref __self_1_0, function: ref __self_1_1 } =>
                    match *self {
                        CheckedExtrinsic {
                        signed: ref __self_0_0, function: ref __self_0_1 } =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <AccountId: ::std::cmp::Eq, Index: ::std::cmp::Eq,
              Call: ::std::cmp::Eq> ::std::cmp::Eq for
         CheckedExtrinsic<AccountId, Index, Call> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _:
                            ::std::cmp::AssertParamIsEq<Option<(AccountId,
                                                                Index)>>;
                    let _: ::std::cmp::AssertParamIsEq<Call>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <AccountId: ::std::clone::Clone, Index: ::std::clone::Clone,
              Call: ::std::clone::Clone> ::std::clone::Clone for
         CheckedExtrinsic<AccountId, Index, Call> {
            #[inline]
            fn clone(&self) -> CheckedExtrinsic<AccountId, Index, Call> {
                match *self {
                    CheckedExtrinsic {
                    signed: ref __self_0_0, function: ref __self_0_1 } =>
                    CheckedExtrinsic{signed:
                                         ::std::clone::Clone::clone(&(*__self_0_0)),
                                     function:
                                         ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <AccountId: ::std::fmt::Debug, Index: ::std::fmt::Debug,
              Call: ::std::fmt::Debug> ::std::fmt::Debug for
         CheckedExtrinsic<AccountId, Index, Call> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    CheckedExtrinsic {
                    signed: ref __self_0_0, function: ref __self_0_1 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("CheckedExtrinsic");
                        let _ =
                            debug_trait_builder.field("signed",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("function",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl <AccountId, Index, Call> traits::Applyable for
         CheckedExtrinsic<AccountId, Index, Call> where AccountId: Member +
         MaybeDisplay, Index: Member + MaybeDisplay + SimpleArithmetic,
         Call: Member {
            type
            Index
            =
            Index;
            type
            AccountId
            =
            AccountId;
            type
            Call
            =
            Call;
            fn index(&self) -> Option<&Self::Index> {
                self.signed.as_ref().map(|x| &x.1)
            }
            fn sender(&self) -> Option<&Self::AccountId> {
                self.signed.as_ref().map(|x| &x.0)
            }
            fn deconstruct(self) -> (Self::Call, Option<Self::AccountId>) {
                (self.function, self.signed.map(|x| x.0))
            }
        }
    }
    mod header {
        //! Generic implementation of a block header.
        #[cfg(feature = "std")]
        use serde::Serialize;
        use crate::codec::{Decode, Encode, Codec, Input, Output, HasCompact,
                           EncodeAsRef};
        use crate::traits::{self, Member, SimpleArithmetic, SimpleBitOps,
                            MaybeDisplay, Hash as HashT, DigestItem as
                            DigestItemT, MaybeSerializeDebug,
                            MaybeSerializeDebugButNotDeserialize};
        use crate::generic::Digest;
        /// Abstraction over a block header for a substrate chain.
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        #[structural_match]
        pub struct Header<Number: Copy + Into<u128>, Hash: HashT,
                          DigestItem> {
            /// The parent hash.
            pub parent_hash: <Hash as HashT>::Output,
            /// The block number.
            #[serde(serialize_with = "serialize_number")]
            pub number: Number,
            /// The state trie merkle root
            pub state_root: <Hash as HashT>::Output,
            /// The merkle root of the extrinsics.
            pub extrinsics_root: <Hash as HashT>::Output,
            /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
            pub digest: Digest<DigestItem>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Number: ::std::cmp::PartialEq + Copy + Into<u128>,
              Hash: ::std::cmp::PartialEq + HashT,
              DigestItem: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
         Header<Number, Hash, DigestItem> {
            #[inline]
            fn eq(&self, other: &Header<Number, Hash, DigestItem>) -> bool {
                match *other {
                    Header {
                    parent_hash: ref __self_1_0,
                    number: ref __self_1_1,
                    state_root: ref __self_1_2,
                    extrinsics_root: ref __self_1_3,
                    digest: ref __self_1_4 } =>
                    match *self {
                        Header {
                        parent_hash: ref __self_0_0,
                        number: ref __self_0_1,
                        state_root: ref __self_0_2,
                        extrinsics_root: ref __self_0_3,
                        digest: ref __self_0_4 } =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &Header<Number, Hash, DigestItem>) -> bool {
                match *other {
                    Header {
                    parent_hash: ref __self_1_0,
                    number: ref __self_1_1,
                    state_root: ref __self_1_2,
                    extrinsics_root: ref __self_1_3,
                    digest: ref __self_1_4 } =>
                    match *self {
                        Header {
                        parent_hash: ref __self_0_0,
                        number: ref __self_0_1,
                        state_root: ref __self_0_2,
                        extrinsics_root: ref __self_0_3,
                        digest: ref __self_0_4 } =>
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
        impl <Number: ::std::cmp::Eq + Copy + Into<u128>,
              Hash: ::std::cmp::Eq + HashT, DigestItem: ::std::cmp::Eq>
         ::std::cmp::Eq for Header<Number, Hash, DigestItem> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _:
                            ::std::cmp::AssertParamIsEq<<Hash as
                                                        HashT>::Output>;
                    let _: ::std::cmp::AssertParamIsEq<Number>;
                    let _:
                            ::std::cmp::AssertParamIsEq<<Hash as
                                                        HashT>::Output>;
                    let _:
                            ::std::cmp::AssertParamIsEq<<Hash as
                                                        HashT>::Output>;
                    let _: ::std::cmp::AssertParamIsEq<Digest<DigestItem>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Number: ::std::clone::Clone + Copy + Into<u128>,
              Hash: ::std::clone::Clone + HashT,
              DigestItem: ::std::clone::Clone> ::std::clone::Clone for
         Header<Number, Hash, DigestItem> {
            #[inline]
            fn clone(&self) -> Header<Number, Hash, DigestItem> {
                match *self {
                    Header {
                    parent_hash: ref __self_0_0,
                    number: ref __self_0_1,
                    state_root: ref __self_0_2,
                    extrinsics_root: ref __self_0_3,
                    digest: ref __self_0_4 } =>
                    Header{parent_hash:
                               ::std::clone::Clone::clone(&(*__self_0_0)),
                           number: ::std::clone::Clone::clone(&(*__self_0_1)),
                           state_root:
                               ::std::clone::Clone::clone(&(*__self_0_2)),
                           extrinsics_root:
                               ::std::clone::Clone::clone(&(*__self_0_3)),
                           digest:
                               ::std::clone::Clone::clone(&(*__self_0_4)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Number: ::std::fmt::Debug + Copy + Into<u128>,
              Hash: ::std::fmt::Debug + HashT, DigestItem: ::std::fmt::Debug>
         ::std::fmt::Debug for Header<Number, Hash, DigestItem> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Header {
                    parent_hash: ref __self_0_0,
                    number: ref __self_0_1,
                    state_root: ref __self_0_2,
                    extrinsics_root: ref __self_0_3,
                    digest: ref __self_0_4 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("Header");
                        let _ =
                            debug_trait_builder.field("parent_hash",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("number",
                                                      &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("state_root",
                                                      &&(*__self_0_2));
                        let _ =
                            debug_trait_builder.field("extrinsics_root",
                                                      &&(*__self_0_3));
                        let _ =
                            debug_trait_builder.field("digest",
                                                      &&(*__self_0_4));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Header: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
                #[automatically_derived]
                impl <Number: Copy + Into<u128>, Hash: HashT, DigestItem>
                 _serde::Serialize for Header<Number, Hash, DigestItem> where
                 Hash: _serde::Serialize, DigestItem: _serde::Serialize {
                    fn serialize<__S>(&self, __serializer: __S)
                     -> _serde::export::Result<__S::Ok, __S::Error> where
                     __S: _serde::Serializer {
                        let mut __serde_state =
                            match _serde::Serializer::serialize_struct(__serializer,
                                                                       "Header",
                                                                       false
                                                                           as
                                                                           usize
                                                                           + 1
                                                                           + 1
                                                                           + 1
                                                                           + 1
                                                                           +
                                                                           1)
                                {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                            "parentHash",
                                                                            &self.parent_hash)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                            "number",
                                                                            {
                                                                                struct __SerializeWith<'__a,
                                                                                                       Number: Copy +
                                                                                                       Into<u128> +
                                                                                                       '__a,
                                                                                                       Hash: HashT +
                                                                                                       '__a,
                                                                                                       DigestItem: '__a>
                                                                                       where
                                                                                       Hash: _serde::Serialize,
                                                                                       DigestItem: _serde::Serialize {
                                                                                    values: (&'__a Number,),
                                                                                    phantom: _serde::export::PhantomData<Header<Number,
                                                                                                                                Hash,
                                                                                                                                DigestItem>>,
                                                                                }
                                                                                impl <'__a,
                                                                                      Number: Copy +
                                                                                      Into<u128> +
                                                                                      '__a,
                                                                                      Hash: HashT +
                                                                                      '__a,
                                                                                      DigestItem: '__a>
                                                                                 _serde::Serialize
                                                                                 for
                                                                                 __SerializeWith<'__a,
                                                                                                 Number,
                                                                                                 Hash,
                                                                                                 DigestItem>
                                                                                 where
                                                                                 Hash: _serde::Serialize,
                                                                                 DigestItem: _serde::Serialize
                                                                                 {
                                                                                    fn serialize<__S>(&self,
                                                                                                      __s:
                                                                                                          __S)
                                                                                     ->
                                                                                         _serde::export::Result<__S::Ok,
                                                                                                                __S::Error>
                                                                                     where
                                                                                     __S: _serde::Serializer {
                                                                                        serialize_number(self.values.0,
                                                                                                         __s)
                                                                                    }
                                                                                }
                                                                                &__SerializeWith{values:
                                                                                                     (&self.number,),
                                                                                                 phantom:
                                                                                                     _serde::export::PhantomData::<Header<Number,
                                                                                                                                          Hash,
                                                                                                                                          DigestItem>>,}
                                                                            })
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                            "stateRoot",
                                                                            &self.state_root)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                            "extrinsicsRoot",
                                                                            &self.extrinsics_root)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                            "digest",
                                                                            &self.digest)
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
        #[cfg(feature = "std")]
        pub fn serialize_number<S, T: Copy + Into<u128>>(val: &T, s: S)
         -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
            use substrate_primitives::uint::U256;
            let v: u128 = (*val).into();
            let lower = U256::from(v as u64);
            let upper = U256::from(v.rotate_left(64) as u64) << 64;
            ::serde::Serialize::serialize(&(upper + lower), s)
        }
        impl <Number, Hash, DigestItem> Decode for
         Header<Number, Hash, DigestItem> where Number: HasCompact + Copy +
         Into<u128>, Hash: HashT, Hash::Output: Decode,
         DigestItem: DigestItemT + Decode {
            fn decode<I: Input>(input: &mut I) -> Option<Self> {
                Some(Header{parent_hash: Decode::decode(input)?,
                            number:
                                <<Number as
                                 HasCompact>::Type>::decode(input)?.into(),
                            state_root: Decode::decode(input)?,
                            extrinsics_root: Decode::decode(input)?,
                            digest: Decode::decode(input)?,})
            }
        }
        impl <Number, Hash, DigestItem> Encode for
         Header<Number, Hash, DigestItem> where Number: HasCompact + Copy +
         Into<u128>, Hash: HashT, Hash::Output: Encode,
         DigestItem: DigestItemT + Encode {
            fn encode_to<T: Output>(&self, dest: &mut T) {
                dest.push(&self.parent_hash);
                dest.push(&<<<Number as HasCompact>::Type as
                            EncodeAsRef<_>>::RefType>::from(&self.number));
                dest.push(&self.state_root);
                dest.push(&self.extrinsics_root);
                dest.push(&self.digest);
            }
        }
        impl <Number, Hash, DigestItem> traits::Header for
         Header<Number, Hash, DigestItem> where Number: Member +
         MaybeSerializeDebug + ::rstd::hash::Hash + MaybeDisplay +
         SimpleArithmetic + Codec + Copy + Into<u128>, Hash: HashT,
         DigestItem: DigestItemT<Hash = Hash::Output> + Codec,
         Hash::Output: Default + ::rstd::hash::Hash + Copy + Member +
         MaybeSerializeDebugButNotDeserialize + MaybeDisplay + SimpleBitOps +
         Codec {
            type
            Number
            =
            Number;
            type
            Hash
            =
            <Hash as HashT>::Output;
            type
            Hashing
            =
            Hash;
            type
            Digest
            =
            Digest<DigestItem>;
            fn number(&self) -> &Self::Number { &self.number }
            fn set_number(&mut self, num: Self::Number) { self.number = num }
            fn extrinsics_root(&self) -> &Self::Hash { &self.extrinsics_root }
            fn set_extrinsics_root(&mut self, root: Self::Hash) {
                self.extrinsics_root = root
            }
            fn state_root(&self) -> &Self::Hash { &self.state_root }
            fn set_state_root(&mut self, root: Self::Hash) {
                self.state_root = root
            }
            fn parent_hash(&self) -> &Self::Hash { &self.parent_hash }
            fn set_parent_hash(&mut self, hash: Self::Hash) {
                self.parent_hash = hash
            }
            fn digest(&self) -> &Self::Digest { &self.digest }
            fn digest_mut(&mut self) -> &mut Self::Digest { &mut self.digest }
            fn set_digest(&mut self, digest: Self::Digest) {
                self.digest = digest
            }
            fn new(number: Self::Number, extrinsics_root: Self::Hash,
                   state_root: Self::Hash, parent_hash: Self::Hash,
                   digest: Self::Digest) -> Self {
                Header{number,
                       extrinsics_root,
                       state_root,
                       parent_hash,
                       digest,}
            }
        }
        impl <Number, Hash, DigestItem> Header<Number, Hash, DigestItem> where
         Number: Member + ::rstd::hash::Hash + Copy + MaybeDisplay +
         SimpleArithmetic + Codec + Into<u128>, Hash: HashT,
         DigestItem: DigestItemT + Codec, Hash::Output: Default +
         ::rstd::hash::Hash + Copy + Member + MaybeDisplay + SimpleBitOps +
         Codec {
            /// Convenience helper for computing the hash of the header without having
            /// to import the trait.
            pub fn hash(&self) -> Hash::Output { Hash::hash_of(self) }
        }
    }
    mod block {
        //! Generic implementation of a block and associated items.
        #[cfg(feature = "std")]
        use std::fmt;
        #[cfg(feature = "std")]
        use serde::Serialize;
        use rstd::prelude::*;
        use crate::codec::{Codec, Encode, Decode};
        use crate::traits::{self, Member, Block as BlockT, Header as HeaderT,
                            MaybeSerialize};
        use crate::Justification;
        /// Something to identify a block.
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        #[structural_match]
        pub enum BlockId<Block: BlockT> {

            /// Identify by block header hash.
            Hash(<<Block as BlockT>::Header as HeaderT>::Hash),

            /// Identify by block number.
            Number(<<Block as BlockT>::Header as HeaderT>::Number),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Block: ::std::cmp::PartialEq + BlockT> ::std::cmp::PartialEq for
         BlockId<Block> {
            #[inline]
            fn eq(&self, other: &BlockId<Block>) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*other)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&BlockId::Hash(ref __self_0),
                             &BlockId::Hash(ref __arg_1_0)) =>
                            (*__self_0) == (*__arg_1_0),
                            (&BlockId::Number(ref __self_0),
                             &BlockId::Number(ref __arg_1_0)) =>
                            (*__self_0) == (*__arg_1_0),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { false }
                }
            }
            #[inline]
            fn ne(&self, other: &BlockId<Block>) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*other)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&BlockId::Hash(ref __self_0),
                             &BlockId::Hash(ref __arg_1_0)) =>
                            (*__self_0) != (*__arg_1_0),
                            (&BlockId::Number(ref __self_0),
                             &BlockId::Number(ref __arg_1_0)) =>
                            (*__self_0) != (*__arg_1_0),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { true }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Block: ::std::cmp::Eq + BlockT> ::std::cmp::Eq for
         BlockId<Block> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _:
                            ::std::cmp::AssertParamIsEq<<<Block as
                                                         BlockT>::Header as
                                                        HeaderT>::Hash>;
                    let _:
                            ::std::cmp::AssertParamIsEq<<<Block as
                                                         BlockT>::Header as
                                                        HeaderT>::Number>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Block: ::std::clone::Clone + BlockT> ::std::clone::Clone for
         BlockId<Block> {
            #[inline]
            fn clone(&self) -> BlockId<Block> {
                match (&*self,) {
                    (&BlockId::Hash(ref __self_0),) =>
                    BlockId::Hash(::std::clone::Clone::clone(&(*__self_0))),
                    (&BlockId::Number(ref __self_0),) =>
                    BlockId::Number(::std::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Block: ::std::fmt::Debug + BlockT> ::std::fmt::Debug for
         BlockId<Block> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&BlockId::Hash(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Hash");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&BlockId::Number(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Number");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_BlockId: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
                #[automatically_derived]
                impl <Block: BlockT> _serde::Serialize for BlockId<Block>
                 where Block: _serde::Serialize {
                    fn serialize<__S>(&self, __serializer: __S)
                     -> _serde::export::Result<__S::Ok, __S::Error> where
                     __S: _serde::Serializer {
                        match *self {
                            BlockId::Hash(ref __field0) =>
                            _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                          "BlockId",
                                                                          0u32,
                                                                          "hash",
                                                                          __field0),
                            BlockId::Number(ref __field0) =>
                            _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                          "BlockId",
                                                                          1u32,
                                                                          "number",
                                                                          __field0),
                        }
                    }
                }
            };
        impl <Block: BlockT> BlockId<Block> {
            /// Create a block ID from a hash.
            pub fn hash(hash: Block::Hash) -> Self { BlockId::Hash(hash) }
            /// Create a block ID from a number.
            pub fn number(number: <Block::Header as HeaderT>::Number)
             -> Self {
                BlockId::Number(number)
            }
        }
        impl <Block: BlockT> Copy for BlockId<Block> { }
        #[cfg(feature = "std")]
        impl <Block: BlockT> fmt::Display for BlockId<Block> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                          &match (&self,) {
                                                               (arg0,) =>
                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                            ::std::fmt::Debug::fmt)],
                                                           }))
            }
        }
        /// Abstraction over a substrate block.
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        #[structural_match]
        pub struct Block<Header, Extrinsic: MaybeSerialize> {
            /// The block header.
            pub header: Header,
            /// The accompanying extrinsics.
            pub extrinsics: Vec<Extrinsic>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::PartialEq,
              Extrinsic: ::std::cmp::PartialEq + MaybeSerialize>
         ::std::cmp::PartialEq for Block<Header, Extrinsic> {
            #[inline]
            fn eq(&self, other: &Block<Header, Extrinsic>) -> bool {
                match *other {
                    Block { header: ref __self_1_0, extrinsics: ref __self_1_1
                    } =>
                    match *self {
                        Block {
                        header: ref __self_0_0, extrinsics: ref __self_0_1 }
                        =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &Block<Header, Extrinsic>) -> bool {
                match *other {
                    Block { header: ref __self_1_0, extrinsics: ref __self_1_1
                    } =>
                    match *self {
                        Block {
                        header: ref __self_0_0, extrinsics: ref __self_0_1 }
                        =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::Eq, Extrinsic: ::std::cmp::Eq +
              MaybeSerialize> ::std::cmp::Eq for Block<Header, Extrinsic> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Header>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<Extrinsic>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::clone::Clone, Extrinsic: ::std::clone::Clone +
              MaybeSerialize> ::std::clone::Clone for Block<Header, Extrinsic>
         {
            #[inline]
            fn clone(&self) -> Block<Header, Extrinsic> {
                match *self {
                    Block { header: ref __self_0_0, extrinsics: ref __self_0_1
                    } =>
                    Block{header: ::std::clone::Clone::clone(&(*__self_0_0)),
                          extrinsics:
                              ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_ENCODE_FOR_Block: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl <Header, Extrinsic: MaybeSerialize> _parity_codec::Encode
                 for Block<Header, Extrinsic> where
                 Header: _parity_codec::Encode, Header: _parity_codec::Encode,
                 Vec<Extrinsic>: _parity_codec::Encode,
                 Vec<Extrinsic>: _parity_codec::Encode {
                    fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                                dest:
                                                                    &mut EncOut) {
                        dest.push(&self.header);
                        dest.push(&self.extrinsics);
                    }
                }
            };
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_DECODE_FOR_Block: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl <Header, Extrinsic: MaybeSerialize> _parity_codec::Decode
                 for Block<Header, Extrinsic> where
                 Header: _parity_codec::Decode, Header: _parity_codec::Decode,
                 Vec<Extrinsic>: _parity_codec::Decode,
                 Vec<Extrinsic>: _parity_codec::Decode {
                    fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                     -> Option<Self> {
                        Some(Block{header:
                                       _parity_codec::Decode::decode(input)?,
                                   extrinsics:
                                       _parity_codec::Decode::decode(input)?,})
                    }
                }
            };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::fmt::Debug, Extrinsic: ::std::fmt::Debug +
              MaybeSerialize> ::std::fmt::Debug for Block<Header, Extrinsic> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Block { header: ref __self_0_0, extrinsics: ref __self_0_1
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Block");
                        let _ =
                            debug_trait_builder.field("header",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("extrinsics",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Block: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
                #[automatically_derived]
                impl <Header, Extrinsic: MaybeSerialize> _serde::Serialize for
                 Block<Header, Extrinsic> where Header: _serde::Serialize,
                 Extrinsic: _serde::Serialize {
                    fn serialize<__S>(&self, __serializer: __S)
                     -> _serde::export::Result<__S::Ok, __S::Error> where
                     __S: _serde::Serializer {
                        let mut __serde_state =
                            match _serde::Serializer::serialize_struct(__serializer,
                                                                       "Block",
                                                                       false
                                                                           as
                                                                           usize
                                                                           + 1
                                                                           +
                                                                           1)
                                {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                            "header",
                                                                            &self.header)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                            "extrinsics",
                                                                            &self.extrinsics)
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
        impl <Header, Extrinsic: MaybeSerialize> traits::Block for
         Block<Header, Extrinsic> where Header: HeaderT, Extrinsic: Member +
         Codec + traits::Extrinsic {
            type
            Extrinsic
            =
            Extrinsic;
            type
            Header
            =
            Header;
            type
            Hash
            =
            <Self::Header as traits::Header>::Hash;
            fn header(&self) -> &Self::Header { &self.header }
            fn extrinsics(&self) -> &[Self::Extrinsic] {
                &self.extrinsics[..]
            }
            fn deconstruct(self) -> (Self::Header, Vec<Self::Extrinsic>) {
                (self.header, self.extrinsics)
            }
            fn new(header: Self::Header, extrinsics: Vec<Self::Extrinsic>)
             -> Self {
                Block{header, extrinsics,}
            }
        }
        /// Abstraction over a substrate block and justification.
        #[serde(rename_all = "camelCase")]
        #[serde(deny_unknown_fields)]
        #[structural_match]
        pub struct SignedBlock<Block> {
            /// Full block.
            pub block: Block,
            /// Block justification.
            pub justification: Option<Justification>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Block: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
         SignedBlock<Block> {
            #[inline]
            fn eq(&self, other: &SignedBlock<Block>) -> bool {
                match *other {
                    SignedBlock {
                    block: ref __self_1_0, justification: ref __self_1_1 } =>
                    match *self {
                        SignedBlock {
                        block: ref __self_0_0, justification: ref __self_0_1 }
                        =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &SignedBlock<Block>) -> bool {
                match *other {
                    SignedBlock {
                    block: ref __self_1_0, justification: ref __self_1_1 } =>
                    match *self {
                        SignedBlock {
                        block: ref __self_0_0, justification: ref __self_0_1 }
                        =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Block: ::std::cmp::Eq> ::std::cmp::Eq for SignedBlock<Block> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Block>;
                    let _: ::std::cmp::AssertParamIsEq<Option<Justification>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Block: ::std::clone::Clone> ::std::clone::Clone for
         SignedBlock<Block> {
            #[inline]
            fn clone(&self) -> SignedBlock<Block> {
                match *self {
                    SignedBlock {
                    block: ref __self_0_0, justification: ref __self_0_1 } =>
                    SignedBlock{block:
                                    ::std::clone::Clone::clone(&(*__self_0_0)),
                                justification:
                                    ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_ENCODE_FOR_SignedBlock: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl <Block> _parity_codec::Encode for SignedBlock<Block>
                 where Block: _parity_codec::Encode,
                 Block: _parity_codec::Encode {
                    fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                                dest:
                                                                    &mut EncOut) {
                        dest.push(&self.block);
                        dest.push(&self.justification);
                    }
                }
            };
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_DECODE_FOR_SignedBlock: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl <Block> _parity_codec::Decode for SignedBlock<Block>
                 where Block: _parity_codec::Decode,
                 Block: _parity_codec::Decode {
                    fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                     -> Option<Self> {
                        Some(SignedBlock{block:
                                             _parity_codec::Decode::decode(input)?,
                                         justification:
                                             _parity_codec::Decode::decode(input)?,})
                    }
                }
            };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Block: ::std::fmt::Debug> ::std::fmt::Debug for
         SignedBlock<Block> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    SignedBlock {
                    block: ref __self_0_0, justification: ref __self_0_1 } =>
                    {
                        let mut debug_trait_builder =
                            f.debug_struct("SignedBlock");
                        let _ =
                            debug_trait_builder.field("block",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("justification",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_SignedBlock: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
                #[automatically_derived]
                impl <Block> _serde::Serialize for SignedBlock<Block> where
                 Block: _serde::Serialize {
                    fn serialize<__S>(&self, __serializer: __S)
                     -> _serde::export::Result<__S::Ok, __S::Error> where
                     __S: _serde::Serializer {
                        let mut __serde_state =
                            match _serde::Serializer::serialize_struct(__serializer,
                                                                       "SignedBlock",
                                                                       false
                                                                           as
                                                                           usize
                                                                           + 1
                                                                           +
                                                                           1)
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
                                                                            "justification",
                                                                            &self.justification)
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
    }
    mod digest {
        //! Generic implementation of a digest.
        #[cfg(feature = "std")]
        use serde::Serialize;
        use rstd::prelude::*;
        use crate::ConsensusEngineId;
        use crate::codec::{Decode, Encode, Codec, Input};
        use crate::traits::{self, Member, DigestItem as DigestItemT,
                            MaybeHash};
        /// Generic header digest.
        #[structural_match]
        pub struct Digest<Item> {
            /// A list of logs in the digest.
            pub logs: Vec<Item>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Item: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
         Digest<Item> {
            #[inline]
            fn eq(&self, other: &Digest<Item>) -> bool {
                match *other {
                    Digest { logs: ref __self_1_0 } =>
                    match *self {
                        Digest { logs: ref __self_0_0 } =>
                        (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &Digest<Item>) -> bool {
                match *other {
                    Digest { logs: ref __self_1_0 } =>
                    match *self {
                        Digest { logs: ref __self_0_0 } =>
                        (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Item: ::std::cmp::Eq> ::std::cmp::Eq for Digest<Item> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                { let _: ::std::cmp::AssertParamIsEq<Vec<Item>>; }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Item: ::std::clone::Clone> ::std::clone::Clone for Digest<Item>
         {
            #[inline]
            fn clone(&self) -> Digest<Item> {
                match *self {
                    Digest { logs: ref __self_0_0 } =>
                    Digest{logs: ::std::clone::Clone::clone(&(*__self_0_0)),},
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_ENCODE_FOR_Digest: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl <Item> _parity_codec::Encode for Digest<Item> where
                 Vec<Item>: _parity_codec::Encode,
                 Vec<Item>: _parity_codec::Encode {
                    fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                                dest:
                                                                    &mut EncOut) {
                        dest.push(&self.logs);
                    }
                }
            };
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_DECODE_FOR_Digest: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl <Item> _parity_codec::Decode for Digest<Item> where
                 Vec<Item>: _parity_codec::Decode,
                 Vec<Item>: _parity_codec::Decode {
                    fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                     -> Option<Self> {
                        Some(Digest{logs:
                                        _parity_codec::Decode::decode(input)?,})
                    }
                }
            };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Item: ::std::fmt::Debug> ::std::fmt::Debug for Digest<Item> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Digest { logs: ref __self_0_0 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("Digest");
                        let _ =
                            debug_trait_builder.field("logs",
                                                      &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Digest: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
                #[automatically_derived]
                impl <Item> _serde::Serialize for Digest<Item> where
                 Item: _serde::Serialize {
                    fn serialize<__S>(&self, __serializer: __S)
                     -> _serde::export::Result<__S::Ok, __S::Error> where
                     __S: _serde::Serializer {
                        let mut __serde_state =
                            match _serde::Serializer::serialize_struct(__serializer,
                                                                       "Digest",
                                                                       false
                                                                           as
                                                                           usize
                                                                           +
                                                                           1)
                                {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                            "logs",
                                                                            &self.logs)
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
        impl <Item> Default for Digest<Item> {
            fn default() -> Self { Digest{logs: Vec::new(),} }
        }
        impl <Item> traits::Digest for Digest<Item> where Item: DigestItemT +
         Codec {
            type
            Hash
            =
            Item::Hash;
            type
            Item
            =
            Item;
            fn logs(&self) -> &[Self::Item] { &self.logs }
            fn push(&mut self, item: Self::Item) { self.logs.push(item); }
            fn pop(&mut self) -> Option<Self::Item> { self.logs.pop() }
        }
        /// Digest item that is able to encode/decode 'system' digest items and
        /// provide opaque access to other items.
        #[allow(deprecated)]
        #[structural_match]
        pub enum DigestItem<Hash, AuthorityId, SealSignature> {

            /// System digest item announcing that authorities set has been changed
            /// in the block. Contains the new set of authorities.
            AuthoritiesChange(Vec<AuthorityId>),

            /// System digest item that contains the root of changes trie at given
            /// block. It is created for every block iff runtime supports changes
            /// trie creation.
            ChangesTrieRoot(Hash),

            /// The old way to put a Seal on it.  Deprecated.
            #[deprecated(since = "1.0",
                         note =
                             "New versions of Substrate will never generate this, and it will be rejected on new blockchains.")]
            Seal(u64, SealSignature),

            /// Put a Seal on it
            Consensus(ConsensusEngineId, Vec<u8>),

            /// Any 'non-system' digest item, opaque to the native code.
            Other(Vec<u8>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(deprecated)]
        impl <Hash: ::std::cmp::PartialEq, AuthorityId: ::std::cmp::PartialEq,
              SealSignature: ::std::cmp::PartialEq> ::std::cmp::PartialEq for
         DigestItem<Hash, AuthorityId, SealSignature> {
            #[inline]
            fn eq(&self, other: &DigestItem<Hash, AuthorityId, SealSignature>)
             -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*other)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&DigestItem::AuthoritiesChange(ref __self_0),
                             &DigestItem::AuthoritiesChange(ref __arg_1_0)) =>
                            (*__self_0) == (*__arg_1_0),
                            (&DigestItem::ChangesTrieRoot(ref __self_0),
                             &DigestItem::ChangesTrieRoot(ref __arg_1_0)) =>
                            (*__self_0) == (*__arg_1_0),
                            (&DigestItem::Seal(ref __self_0, ref __self_1),
                             &DigestItem::Seal(ref __arg_1_0, ref __arg_1_1))
                            =>
                            (*__self_0) == (*__arg_1_0) &&
                                (*__self_1) == (*__arg_1_1),
                            (&DigestItem::Consensus(ref __self_0,
                                                    ref __self_1),
                             &DigestItem::Consensus(ref __arg_1_0,
                                                    ref __arg_1_1)) =>
                            (*__self_0) == (*__arg_1_0) &&
                                (*__self_1) == (*__arg_1_1),
                            (&DigestItem::Other(ref __self_0),
                             &DigestItem::Other(ref __arg_1_0)) =>
                            (*__self_0) == (*__arg_1_0),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { false }
                }
            }
            #[inline]
            fn ne(&self, other: &DigestItem<Hash, AuthorityId, SealSignature>)
             -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*other)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&DigestItem::AuthoritiesChange(ref __self_0),
                             &DigestItem::AuthoritiesChange(ref __arg_1_0)) =>
                            (*__self_0) != (*__arg_1_0),
                            (&DigestItem::ChangesTrieRoot(ref __self_0),
                             &DigestItem::ChangesTrieRoot(ref __arg_1_0)) =>
                            (*__self_0) != (*__arg_1_0),
                            (&DigestItem::Seal(ref __self_0, ref __self_1),
                             &DigestItem::Seal(ref __arg_1_0, ref __arg_1_1))
                            =>
                            (*__self_0) != (*__arg_1_0) ||
                                (*__self_1) != (*__arg_1_1),
                            (&DigestItem::Consensus(ref __self_0,
                                                    ref __self_1),
                             &DigestItem::Consensus(ref __arg_1_0,
                                                    ref __arg_1_1)) =>
                            (*__self_0) != (*__arg_1_0) ||
                                (*__self_1) != (*__arg_1_1),
                            (&DigestItem::Other(ref __self_0),
                             &DigestItem::Other(ref __arg_1_0)) =>
                            (*__self_0) != (*__arg_1_0),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { true }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(deprecated)]
        impl <Hash: ::std::cmp::Eq, AuthorityId: ::std::cmp::Eq,
              SealSignature: ::std::cmp::Eq> ::std::cmp::Eq for
         DigestItem<Hash, AuthorityId, SealSignature> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Vec<AuthorityId>>;
                    let _: ::std::cmp::AssertParamIsEq<Hash>;
                    let _: ::std::cmp::AssertParamIsEq<u64>;
                    let _: ::std::cmp::AssertParamIsEq<SealSignature>;
                    let _: ::std::cmp::AssertParamIsEq<ConsensusEngineId>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(deprecated)]
        impl <Hash: ::std::clone::Clone, AuthorityId: ::std::clone::Clone,
              SealSignature: ::std::clone::Clone> ::std::clone::Clone for
         DigestItem<Hash, AuthorityId, SealSignature> {
            #[inline]
            fn clone(&self) -> DigestItem<Hash, AuthorityId, SealSignature> {
                match (&*self,) {
                    (&DigestItem::AuthoritiesChange(ref __self_0),) =>
                    DigestItem::AuthoritiesChange(::std::clone::Clone::clone(&(*__self_0))),
                    (&DigestItem::ChangesTrieRoot(ref __self_0),) =>
                    DigestItem::ChangesTrieRoot(::std::clone::Clone::clone(&(*__self_0))),
                    (&DigestItem::Seal(ref __self_0, ref __self_1),) =>
                    DigestItem::Seal(::std::clone::Clone::clone(&(*__self_0)),
                                     ::std::clone::Clone::clone(&(*__self_1))),
                    (&DigestItem::Consensus(ref __self_0, ref __self_1),) =>
                    DigestItem::Consensus(::std::clone::Clone::clone(&(*__self_0)),
                                          ::std::clone::Clone::clone(&(*__self_1))),
                    (&DigestItem::Other(ref __self_0),) =>
                    DigestItem::Other(::std::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(deprecated)]
        impl <Hash: ::std::fmt::Debug, AuthorityId: ::std::fmt::Debug,
              SealSignature: ::std::fmt::Debug> ::std::fmt::Debug for
         DigestItem<Hash, AuthorityId, SealSignature> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&DigestItem::AuthoritiesChange(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("AuthoritiesChange");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&DigestItem::ChangesTrieRoot(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("ChangesTrieRoot");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&DigestItem::Seal(ref __self_0, ref __self_1),) => {
                        let mut debug_trait_builder = f.debug_tuple("Seal");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&DigestItem::Consensus(ref __self_0, ref __self_1),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Consensus");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&DigestItem::Other(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Other");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[cfg(feature = "std")]
        impl <Hash: Encode, AuthorityId: Encode, SealSignature: Encode>
         ::serde::Serialize for DigestItem<Hash, AuthorityId, SealSignature> {
            fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error> where
             S: ::serde::Serializer {
                self.using_encoded(|bytes|
                                       {
                                           ::substrate_primitives::bytes::serialize(bytes,
                                                                                    seq)
                                       })
            }
        }
        /// A 'referencing view' for digest item. Does not own its contents. Used by
        /// final runtime implementations for encoding/decoding its log items.
        #[allow(deprecated)]
        #[structural_match]
        pub enum DigestItemRef<'a, Hash: 'a, AuthorityId: 'a,
                               SealSignature: 'a> {

            /// Reference to `DigestItem::AuthoritiesChange`.
            AuthoritiesChange(&'a [AuthorityId]),

            /// Reference to `DigestItem::ChangesTrieRoot`.
            ChangesTrieRoot(&'a Hash),

            /// A deprecated sealed signature for testing
            #[deprecated]
            Seal(&'a u64, &'a SealSignature),

            /// A sealed signature for testing
            Consensus(&'a ConsensusEngineId, &'a [u8]),

            /// Any 'non-system' digest item, opaque to the native code.
            /// Reference to `DigestItem::Other`.
            Other(&'a [u8]),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(deprecated)]
        impl <'a, Hash: ::std::cmp::PartialEq + 'a,
              AuthorityId: ::std::cmp::PartialEq + 'a,
              SealSignature: ::std::cmp::PartialEq + 'a> ::std::cmp::PartialEq
         for DigestItemRef<'a, Hash, AuthorityId, SealSignature> {
            #[inline]
            fn eq(&self,
                  other: &DigestItemRef<'a, Hash, AuthorityId, SealSignature>)
             -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*other)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&DigestItemRef::AuthoritiesChange(ref __self_0),
                             &DigestItemRef::AuthoritiesChange(ref __arg_1_0))
                            => (*__self_0) == (*__arg_1_0),
                            (&DigestItemRef::ChangesTrieRoot(ref __self_0),
                             &DigestItemRef::ChangesTrieRoot(ref __arg_1_0))
                            => (*__self_0) == (*__arg_1_0),
                            (&DigestItemRef::Seal(ref __self_0, ref __self_1),
                             &DigestItemRef::Seal(ref __arg_1_0,
                                                  ref __arg_1_1)) =>
                            (*__self_0) == (*__arg_1_0) &&
                                (*__self_1) == (*__arg_1_1),
                            (&DigestItemRef::Consensus(ref __self_0,
                                                       ref __self_1),
                             &DigestItemRef::Consensus(ref __arg_1_0,
                                                       ref __arg_1_1)) =>
                            (*__self_0) == (*__arg_1_0) &&
                                (*__self_1) == (*__arg_1_1),
                            (&DigestItemRef::Other(ref __self_0),
                             &DigestItemRef::Other(ref __arg_1_0)) =>
                            (*__self_0) == (*__arg_1_0),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { false }
                }
            }
            #[inline]
            fn ne(&self,
                  other: &DigestItemRef<'a, Hash, AuthorityId, SealSignature>)
             -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as isize;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*other)
                        } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&DigestItemRef::AuthoritiesChange(ref __self_0),
                             &DigestItemRef::AuthoritiesChange(ref __arg_1_0))
                            => (*__self_0) != (*__arg_1_0),
                            (&DigestItemRef::ChangesTrieRoot(ref __self_0),
                             &DigestItemRef::ChangesTrieRoot(ref __arg_1_0))
                            => (*__self_0) != (*__arg_1_0),
                            (&DigestItemRef::Seal(ref __self_0, ref __self_1),
                             &DigestItemRef::Seal(ref __arg_1_0,
                                                  ref __arg_1_1)) =>
                            (*__self_0) != (*__arg_1_0) ||
                                (*__self_1) != (*__arg_1_1),
                            (&DigestItemRef::Consensus(ref __self_0,
                                                       ref __self_1),
                             &DigestItemRef::Consensus(ref __arg_1_0,
                                                       ref __arg_1_1)) =>
                            (*__self_0) != (*__arg_1_0) ||
                                (*__self_1) != (*__arg_1_1),
                            (&DigestItemRef::Other(ref __self_0),
                             &DigestItemRef::Other(ref __arg_1_0)) =>
                            (*__self_0) != (*__arg_1_0),
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { true }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(deprecated)]
        impl <'a, Hash: ::std::cmp::Eq + 'a, AuthorityId: ::std::cmp::Eq + 'a,
              SealSignature: ::std::cmp::Eq + 'a> ::std::cmp::Eq for
         DigestItemRef<'a, Hash, AuthorityId, SealSignature> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<&'a [AuthorityId]>;
                    let _: ::std::cmp::AssertParamIsEq<&'a Hash>;
                    let _: ::std::cmp::AssertParamIsEq<&'a u64>;
                    let _: ::std::cmp::AssertParamIsEq<&'a SealSignature>;
                    let _: ::std::cmp::AssertParamIsEq<&'a ConsensusEngineId>;
                    let _: ::std::cmp::AssertParamIsEq<&'a [u8]>;
                    let _: ::std::cmp::AssertParamIsEq<&'a [u8]>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(deprecated)]
        impl <'a, Hash: ::std::clone::Clone + 'a,
              AuthorityId: ::std::clone::Clone + 'a,
              SealSignature: ::std::clone::Clone + 'a> ::std::clone::Clone for
         DigestItemRef<'a, Hash, AuthorityId, SealSignature> {
            #[inline]
            fn clone(&self)
             -> DigestItemRef<'a, Hash, AuthorityId, SealSignature> {
                match (&*self,) {
                    (&DigestItemRef::AuthoritiesChange(ref __self_0),) =>
                    DigestItemRef::AuthoritiesChange(::std::clone::Clone::clone(&(*__self_0))),
                    (&DigestItemRef::ChangesTrieRoot(ref __self_0),) =>
                    DigestItemRef::ChangesTrieRoot(::std::clone::Clone::clone(&(*__self_0))),
                    (&DigestItemRef::Seal(ref __self_0, ref __self_1),) =>
                    DigestItemRef::Seal(::std::clone::Clone::clone(&(*__self_0)),
                                        ::std::clone::Clone::clone(&(*__self_1))),
                    (&DigestItemRef::Consensus(ref __self_0, ref __self_1),)
                    =>
                    DigestItemRef::Consensus(::std::clone::Clone::clone(&(*__self_0)),
                                             ::std::clone::Clone::clone(&(*__self_1))),
                    (&DigestItemRef::Other(ref __self_0),) =>
                    DigestItemRef::Other(::std::clone::Clone::clone(&(*__self_0))),
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(deprecated)]
        impl <'a, Hash: ::std::fmt::Debug + 'a,
              AuthorityId: ::std::fmt::Debug + 'a,
              SealSignature: ::std::fmt::Debug + 'a> ::std::fmt::Debug for
         DigestItemRef<'a, Hash, AuthorityId, SealSignature> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&DigestItemRef::AuthoritiesChange(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("AuthoritiesChange");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&DigestItemRef::ChangesTrieRoot(ref __self_0),) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("ChangesTrieRoot");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&DigestItemRef::Seal(ref __self_0, ref __self_1),) => {
                        let mut debug_trait_builder = f.debug_tuple("Seal");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&DigestItemRef::Consensus(ref __self_0, ref __self_1),)
                    => {
                        let mut debug_trait_builder =
                            f.debug_tuple("Consensus");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                    (&DigestItemRef::Other(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Other");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        /// Type of the digest item. Used to gain explicit control over `DigestItem` encoding
        /// process. We need an explicit control, because final runtimes are encoding their own
        /// digest items using `DigestItemRef` type and we can't auto-derive `Decode`
        /// trait for `DigestItemRef`.
        #[repr(u32)]
        enum DigestItemType {
            Other = 0,
            AuthoritiesChange = 1,
            ChangesTrieRoot = 2,
            Seal = 3,
            Consensus = 4,
        }
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_ENCODE_FOR_DigestItemType: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl _parity_codec::Encode for DigestItemType {
                    fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                                dest:
                                                                    &mut EncOut) {
                        match *self {
                            DigestItemType::Other => {
                                dest.push_byte(0 as u8);
                            }
                            DigestItemType::AuthoritiesChange => {
                                dest.push_byte(1 as u8);
                            }
                            DigestItemType::ChangesTrieRoot => {
                                dest.push_byte(2 as u8);
                            }
                            DigestItemType::Seal => {
                                dest.push_byte(3 as u8);
                            }
                            DigestItemType::Consensus => {
                                dest.push_byte(4 as u8);
                            }
                            _ => (),
                        }
                    }
                }
            };
        #[allow(non_upper_case_globals,
                unused_attributes,
                unused_qualifications)]
        const _IMPL_DECODE_FOR_DigestItemType: () =
            {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate parity_codec as _parity_codec;
                impl _parity_codec::Decode for DigestItemType {
                    fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                     -> Option<Self> {
                        match input.read_byte()? {
                            x if x == 0 as u8 => {
                                Some(DigestItemType::Other)
                            }
                            x if x == 1 as u8 => {
                                Some(DigestItemType::AuthoritiesChange)
                            }
                            x if x == 2 as u8 => {
                                Some(DigestItemType::ChangesTrieRoot)
                            }
                            x if x == 3 as u8 => {
                                Some(DigestItemType::Seal)
                            }
                            x if x == 4 as u8 => {
                                Some(DigestItemType::Consensus)
                            }
                            _ => None,
                        }
                    }
                }
            };
        impl <Hash, AuthorityId, SealSignature>
         DigestItem<Hash, AuthorityId, SealSignature> {
            /// Returns Some if `self` is a `DigestItem::Other`.
            pub fn as_other(&self) -> Option<&Vec<u8>> {
                match *self {
                    DigestItem::Other(ref v) => Some(v),
                    _ => None,
                }
            }
            /// Returns a 'referencing view' for this digest item.
            #[allow(deprecated)]
            fn dref<'a>(&'a self)
             -> DigestItemRef<'a, Hash, AuthorityId, SealSignature> {
                match *self {
                    DigestItem::AuthoritiesChange(ref v) =>
                    DigestItemRef::AuthoritiesChange(v),
                    DigestItem::ChangesTrieRoot(ref v) =>
                    DigestItemRef::ChangesTrieRoot(v),
                    DigestItem::Seal(ref v, ref s) =>
                    DigestItemRef::Seal(v, s),
                    DigestItem::Consensus(ref v, ref s) =>
                    DigestItemRef::Consensus(v, s),
                    DigestItem::Other(ref v) => DigestItemRef::Other(v),
                }
            }
        }
        impl <Hash: Codec + Member, AuthorityId: Codec + Member + MaybeHash,
              SealSignature: Codec + Member> traits::DigestItem for
         DigestItem<Hash, AuthorityId, SealSignature> {
            type
            Hash
            =
            Hash;
            type
            AuthorityId
            =
            AuthorityId;
            fn as_authorities_change(&self) -> Option<&[Self::AuthorityId]> {
                self.dref().as_authorities_change()
            }
            fn as_changes_trie_root(&self) -> Option<&Self::Hash> {
                self.dref().as_changes_trie_root()
            }
        }
        impl <Hash: Encode, AuthorityId: Encode, SealSignature: Encode> Encode
         for DigestItem<Hash, AuthorityId, SealSignature> {
            fn encode(&self) -> Vec<u8> { self.dref().encode() }
        }
        impl <Hash: Decode, AuthorityId: Decode, SealSignature: Decode> Decode
         for DigestItem<Hash, AuthorityId, SealSignature> {
            #[allow(deprecated)]
            fn decode<I: Input>(input: &mut I) -> Option<Self> {
                let item_type: DigestItemType = Decode::decode(input)?;
                match item_type {
                    DigestItemType::AuthoritiesChange =>
                    Some(DigestItem::AuthoritiesChange(Decode::decode(input)?)),
                    DigestItemType::ChangesTrieRoot =>
                    Some(DigestItem::ChangesTrieRoot(Decode::decode(input)?)),
                    DigestItemType::Seal => {
                        let vals: (u64, SealSignature) =
                            Decode::decode(input)?;
                        Some(DigestItem::Seal(vals.0, vals.1))
                    }
                    DigestItemType::Consensus => {
                        let vals: (ConsensusEngineId, Vec<u8>) =
                            Decode::decode(input)?;
                        Some(DigestItem::Consensus(vals.0, vals.1))
                    }
                    DigestItemType::Other =>
                    Some(DigestItem::Other(Decode::decode(input)?)),
                }
            }
        }
        impl <'a, Hash: Codec + Member, AuthorityId: Codec + Member,
              SealSignature: Codec + Member>
         DigestItemRef<'a, Hash, AuthorityId, SealSignature> {
            /// Cast this digest item into `AuthoritiesChange`.
            pub fn as_authorities_change(&self) -> Option<&'a [AuthorityId]> {
                match *self {
                    DigestItemRef::AuthoritiesChange(ref authorities) =>
                    Some(authorities),
                    _ => None,
                }
            }
            /// Cast this digest item into `ChangesTrieRoot`.
            pub fn as_changes_trie_root(&self) -> Option<&'a Hash> {
                match *self {
                    DigestItemRef::ChangesTrieRoot(ref changes_trie_root) =>
                    Some(changes_trie_root),
                    _ => None,
                }
            }
        }
        #[allow(deprecated)]
        impl <'a, Hash: Encode, AuthorityId: Encode, SealSignature: Encode>
         Encode for DigestItemRef<'a, Hash, AuthorityId, SealSignature> {
            fn encode(&self) -> Vec<u8> {
                let mut v = Vec::new();
                match *self {
                    DigestItemRef::AuthoritiesChange(authorities) => {
                        DigestItemType::AuthoritiesChange.encode_to(&mut v);
                        authorities.encode_to(&mut v);
                    }
                    DigestItemRef::ChangesTrieRoot(changes_trie_root) => {
                        DigestItemType::ChangesTrieRoot.encode_to(&mut v);
                        changes_trie_root.encode_to(&mut v);
                    }
                    DigestItemRef::Seal(val, sig) => {
                        DigestItemType::Seal.encode_to(&mut v);
                        (val, sig).encode_to(&mut v);
                    }
                    DigestItemRef::Consensus(val, sig) => {
                        DigestItemType::Consensus.encode_to(&mut v);
                        (val, sig).encode_to(&mut v);
                    }
                    DigestItemRef::Other(val) => {
                        DigestItemType::Other.encode_to(&mut v);
                        val.encode_to(&mut v);
                    }
                }
                v
            }
        }
    }
    pub use self::unchecked_extrinsic::UncheckedExtrinsic;
    pub use self::unchecked_mortal_extrinsic::UncheckedMortalExtrinsic;
    pub use self::unchecked_mortal_compact_extrinsic::UncheckedMortalCompactExtrinsic;
    pub use self::era::Era;
    pub use self::checked_extrinsic::CheckedExtrinsic;
    pub use self::header::Header;
    pub use self::block::{Block, SignedBlock, BlockId};
    pub use self::digest::{Digest, DigestItem, DigestItemRef};
    use crate::codec::Encode;
    use rstd::prelude::*;
    fn encode_with_vec_prefix<T: Encode, F: Fn(&mut Vec<u8>)>(encoder: F)
     -> Vec<u8> {
        let size = ::rstd::mem::size_of::<T>();
        let reserve =
            match size {
                0 ...0b00111111 => 1,
                0 ...0b00111111_11111111 => 2,
                _ => 4,
            };
        let mut v = Vec::with_capacity(reserve + size);
        v.resize(reserve, 0);
        encoder(&mut v);
        let mut length: Vec<()> = Vec::new();
        length.resize(v.len() - reserve, ());
        length.using_encoded(|s|
                                 {
                                     v.splice(0..reserve, s.iter().cloned());
                                 });
        v
    }
}
pub mod transaction_validity {
    //! Transaction validity interface.
    use rstd::prelude::*;
    use crate::codec::{Encode, Decode};
    /// Priority for a transaction. Additive. Higher is better.
    pub type TransactionPriority = u64;
    /// Minimum number of blocks a transaction will remain valid for.
    /// `TransactionLongevity::max_value()` means "forever".
    pub type TransactionLongevity = u64;
    /// Tag for a transaction. No two transactions with the same tag should be placed on-chain.
    pub type TransactionTag = Vec<u8>;
    /// Information on a transaction's validity and, if valid, on how it relates to other transactions.
    #[structural_match]
    pub enum TransactionValidity {

        /// Transaction is invalid. Details are described by the error code.
        Invalid(i8),

        /// Transaction is valid.
        Valid {
            /// Priority of the transaction.
            ///
            /// Priority determines the ordering of two transactions that have all
            /// their dependencies (required tags) satisfied.
            priority: TransactionPriority,
            /// Transaction dependencies
            ///
            /// A non-empty list signifies that some other transactions which provide
            /// given tags are required to be included before that one.
            requires: Vec<TransactionTag>,
            /// Provided tags
            ///
            /// A list of tags this transaction provides. Successfully importing the transaction
            /// will enable other transactions that depend on (require) those tags to be included as well.
            /// Provided and requried tags allow Substrate to build a dependency graph of transactions
            /// and import them in the right (linear) order.
            provides: Vec<TransactionTag>,
            /// Transaction longevity
            ///
            /// Longevity describes minimum number of blocks the validity is correct.
            /// After this period transaction should be removed from the pool or revalidated.
            longevity: TransactionLongevity,
        },

        /// Transaction validity can't be determined.
        Unknown(i8),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for TransactionValidity {
        #[inline]
        fn clone(&self) -> TransactionValidity {
            match (&*self,) {
                (&TransactionValidity::Invalid(ref __self_0),) =>
                TransactionValidity::Invalid(::std::clone::Clone::clone(&(*__self_0))),
                (&TransactionValidity::Valid {
                 priority: ref __self_0,
                 requires: ref __self_1,
                 provides: ref __self_2,
                 longevity: ref __self_3 },) =>
                TransactionValidity::Valid{priority:
                                               ::std::clone::Clone::clone(&(*__self_0)),
                                           requires:
                                               ::std::clone::Clone::clone(&(*__self_1)),
                                           provides:
                                               ::std::clone::Clone::clone(&(*__self_2)),
                                           longevity:
                                               ::std::clone::Clone::clone(&(*__self_3)),},
                (&TransactionValidity::Unknown(ref __self_0),) =>
                TransactionValidity::Unknown(::std::clone::Clone::clone(&(*__self_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for TransactionValidity {
        #[inline]
        fn eq(&self, other: &TransactionValidity) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TransactionValidity::Invalid(ref __self_0),
                         &TransactionValidity::Invalid(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        (&TransactionValidity::Valid {
                         priority: ref __self_0,
                         requires: ref __self_1,
                         provides: ref __self_2,
                         longevity: ref __self_3 },
                         &TransactionValidity::Valid {
                         priority: ref __arg_1_0,
                         requires: ref __arg_1_1,
                         provides: ref __arg_1_2,
                         longevity: ref __arg_1_3 }) =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1) &&
                            (*__self_2) == (*__arg_1_2) &&
                            (*__self_3) == (*__arg_1_3),
                        (&TransactionValidity::Unknown(ref __self_0),
                         &TransactionValidity::Unknown(ref __arg_1_0)) =>
                        (*__self_0) == (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &TransactionValidity) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&TransactionValidity::Invalid(ref __self_0),
                         &TransactionValidity::Invalid(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        (&TransactionValidity::Valid {
                         priority: ref __self_0,
                         requires: ref __self_1,
                         provides: ref __self_2,
                         longevity: ref __self_3 },
                         &TransactionValidity::Valid {
                         priority: ref __arg_1_0,
                         requires: ref __arg_1_1,
                         provides: ref __arg_1_2,
                         longevity: ref __arg_1_3 }) =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1) ||
                            (*__self_2) != (*__arg_1_2) ||
                            (*__self_3) != (*__arg_1_3),
                        (&TransactionValidity::Unknown(ref __self_0),
                         &TransactionValidity::Unknown(ref __arg_1_0)) =>
                        (*__self_0) != (*__arg_1_0),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::Eq for TransactionValidity {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<i8>;
                let _: ::std::cmp::AssertParamIsEq<TransactionPriority>;
                let _: ::std::cmp::AssertParamIsEq<Vec<TransactionTag>>;
                let _: ::std::cmp::AssertParamIsEq<Vec<TransactionTag>>;
                let _: ::std::cmp::AssertParamIsEq<TransactionLongevity>;
                let _: ::std::cmp::AssertParamIsEq<i8>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_TransactionValidity: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Encode for TransactionValidity {
                fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                            dest:
                                                                &mut EncOut) {
                    match *self {
                        TransactionValidity::Invalid(ref aa) => {
                            dest.push_byte(0usize as u8);
                            dest.push(aa);
                        }
                        TransactionValidity::Valid {
                        ref priority,
                        ref requires,
                        ref provides,
                        ref longevity } => {
                            dest.push_byte(1usize as u8);
                            dest.push(priority);
                            dest.push(requires);
                            dest.push(provides);
                            dest.push(longevity);
                        }
                        TransactionValidity::Unknown(ref aa) => {
                            dest.push_byte(2usize as u8);
                            dest.push(aa);
                        }
                        _ => (),
                    }
                }
            }
        };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_TransactionValidity: () =
        {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate parity_codec as _parity_codec;
            impl _parity_codec::Decode for TransactionValidity {
                fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
                 -> Option<Self> {
                    match input.read_byte()? {
                        x if x == 0usize as u8 => {
                            Some(TransactionValidity::Invalid(_parity_codec::Decode::decode(input)?))
                        }
                        x if x == 1usize as u8 => {
                            Some(TransactionValidity::Valid{priority:
                                                                _parity_codec::Decode::decode(input)?,
                                                            requires:
                                                                _parity_codec::Decode::decode(input)?,
                                                            provides:
                                                                _parity_codec::Decode::decode(input)?,
                                                            longevity:
                                                                _parity_codec::Decode::decode(input)?,})
                        }
                        x if x == 2usize as u8 => {
                            Some(TransactionValidity::Unknown(_parity_codec::Decode::decode(input)?))
                        }
                        _ => None,
                    }
                }
            }
        };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for TransactionValidity {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&TransactionValidity::Invalid(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Invalid");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&TransactionValidity::Valid {
                 priority: ref __self_0,
                 requires: ref __self_1,
                 provides: ref __self_2,
                 longevity: ref __self_3 },) => {
                    let mut debug_trait_builder = f.debug_struct("Valid");
                    let _ =
                        debug_trait_builder.field("priority", &&(*__self_0));
                    let _ =
                        debug_trait_builder.field("requires", &&(*__self_1));
                    let _ =
                        debug_trait_builder.field("provides", &&(*__self_2));
                    let _ =
                        debug_trait_builder.field("longevity", &&(*__self_3));
                    debug_trait_builder.finish()
                }
                (&TransactionValidity::Unknown(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Unknown");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
}
/// A message indicating an invalid signature in extrinsic.
pub const BAD_SIGNATURE: &str = "bad signature in extrinsic";
/// Full block error message.
///
/// This allows modules to indicate that given transaction is potentially valid
/// in the future, but can't be executed in the current state.
/// Note this error should be returned early in the execution to prevent DoS,
/// cause the fees are not being paid if this error is returned.
///
/// Example: block gas limit is reached (the transaction can be retried in the next block though).
pub const BLOCK_FULL: &str = "block size limit is reached";
/// Justification type.
pub type Justification = Vec<u8>;
use traits::{Verify, Lazy};
/// A String that is a `&'static str` on `no_std` and a `Cow<'static, str>` on `std`.
#[cfg(feature = "std")]
pub type RuntimeString = ::std::borrow::Cow<'static, str>;
/// Create a const [RuntimeString].
#[cfg(feature = "std")]
#[macro_export]
macro_rules! create_runtime_str(( $ y : expr ) => {
                                { :: std :: borrow :: Cow :: Borrowed ( $ y )
                                } });
#[cfg(feature = "std")]
pub use serde::{Serialize, Deserialize, de::DeserializeOwned};
/// Complex storage builder stuff.
#[cfg(feature = "std")]
pub trait BuildStorage: Sized {
    /// Build the storage out of this builder.
    fn build_storage(self)
     -> Result<(StorageOverlay, ChildrenStorageOverlay), String> {
        let mut storage = Default::default();
        let mut child_storage = Default::default();
        self.assimilate_storage(&mut storage, &mut child_storage)?;
        Ok((storage, child_storage))
    }
    /// Assimilate the storage for this module into pre-existing overlays.
    fn assimilate_storage(self, storage: &mut StorageOverlay,
                          child_storage: &mut ChildrenStorageOverlay)
    -> Result<(), String>;
}
#[cfg(feature = "std")]
impl BuildStorage for StorageOverlay {
    fn build_storage(self)
     -> Result<(StorageOverlay, ChildrenStorageOverlay), String> {
        Ok((self, Default::default()))
    }
    fn assimilate_storage(self, storage: &mut StorageOverlay,
                          _child_storage: &mut ChildrenStorageOverlay)
     -> Result<(), String> {
        storage.extend(self);
        Ok(())
    }
}
#[cfg(feature = "std")]
impl BuildStorage for (StorageOverlay, ChildrenStorageOverlay) {
    fn build_storage(self)
     -> Result<(StorageOverlay, ChildrenStorageOverlay), String> {
        Ok(self)
    }
    fn assimilate_storage(self, storage: &mut StorageOverlay,
                          child_storage: &mut ChildrenStorageOverlay)
     -> Result<(), String> {
        storage.extend(self.0);
        child_storage.extend(self.1);
        Ok(())
    }
}
/// Consensus engine unique ID.
pub type ConsensusEngineId = [u8; 4];
/// Permill is parts-per-million (i.e. after multiplying by this, divide by 1000000).
#[structural_match]
#[rustc_copy_clone_marker]
pub struct Permill(u32);
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Permill: () =
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
        impl _serde::Serialize for Permill {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "Permill",
                                                             &self.0)
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Permill: () =
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
        impl <'de> _serde::Deserialize<'de> for Permill {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Permill>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    Permill;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct Permill")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: u32 =
                            match <u32 as
                                      _serde::Deserialize>::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(Permill(__field0))
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
                                                                                                 &"tuple struct Permill with 1 element"));
                                }
                            };
                        _serde::export::Ok(Permill(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "Permill",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<Permill>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Permill {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Permill(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Permill");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Permill: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Permill {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Permill: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Permill {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(Permill(_parity_codec::Decode::decode(input)?))
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::default::Default for Permill {
    #[inline]
    fn default() -> Permill { Permill(::std::default::Default::default()) }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for Permill { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Permill {
    #[inline]
    fn clone(&self) -> Permill {
        { let _: ::std::clone::AssertParamIsClone<u32>; *self }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Permill {
    #[inline]
    fn eq(&self, other: &Permill) -> bool {
        match *other {
            Permill(ref __self_1_0) =>
            match *self {
                Permill(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Permill) -> bool {
        match *other {
            Permill(ref __self_1_0) =>
            match *self {
                Permill(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Permill {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<u32>; }
    }
}
impl Permill {
    /// Nothing.
    pub fn zero() -> Self { Self(0) }
    /// `true` if this is nothing.
    pub fn is_zero(&self) -> bool { self.0 == 0 }
    /// Everything.
    pub fn one() -> Self { Self(1_000_000) }
    /// From an explicitly defined number of parts per maximum of the type.
    pub fn from_parts(x: u32) -> Self { Self(x.min(1_000_000)) }
    /// Converts from a percent. Equal to `x / 100`.
    pub fn from_percent(x: u32) -> Self { Self(x.min(100) * 10_000) }
    /// Converts a fraction into `Permill`.
    #[cfg(feature = "std")]
    pub fn from_fraction(x: f64) -> Self { Self((x * 1_000_000.0) as u32) }
}
impl <N> ops::Mul<N> for Permill where N: Clone + traits::As<u64> +
 ops::Rem<N, Output = N> + ops::Div<N, Output = N> + ops::Mul<N, Output = N> +
 ops::Add<N, Output = N> {
    type
    Output
    =
    N;
    fn mul(self, b: N) -> Self::Output {
        let million = <N as traits::As<u64>>::sa(1_000_000);
        let part = <N as traits::As<u64>>::sa(self.0 as u64);
        let rem_multiplied_divided =
            {
                let rem = b.clone().rem(million.clone());
                let rem_u64: u64 = rem.as_();
                let rem_multiplied_u64 = rem_u64 * self.0 as u64;
                let rem_multiplied_divided_u64 =
                    rem_multiplied_u64 / 1_000_000;
                traits::As::sa(rem_multiplied_divided_u64)
            };
        (b / million) * part + rem_multiplied_divided
    }
}
#[cfg(feature = "std")]
impl From<f64> for Permill {
    fn from(x: f64) -> Permill { Permill::from_fraction(x) }
}
#[cfg(feature = "std")]
impl From<f32> for Permill {
    fn from(x: f32) -> Permill { Permill::from_fraction(x as f64) }
}
impl codec::CompactAs for Permill {
    type
    As
    =
    u32;
    fn encode_as(&self) -> &u32 { &self.0 }
    fn decode_from(x: u32) -> Permill { Permill(x) }
}
impl From<codec::Compact<Permill>> for Permill {
    fn from(x: codec::Compact<Permill>) -> Permill { x.0 }
}
/// Perbill is parts-per-billion. It stores a value between 0 and 1 in fixed point and
/// provides a means to multiply some other value by that.
#[structural_match]
#[rustc_copy_clone_marker]
pub struct Perbill(u32);
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Perbill: () =
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
        impl _serde::Serialize for Perbill {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "Perbill",
                                                             &self.0)
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Perbill: () =
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
        impl <'de> _serde::Deserialize<'de> for Perbill {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Perbill>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    Perbill;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct Perbill")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: u32 =
                            match <u32 as
                                      _serde::Deserialize>::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(Perbill(__field0))
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
                                                                                                 &"tuple struct Perbill with 1 element"));
                                }
                            };
                        _serde::export::Ok(Perbill(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "Perbill",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<Perbill>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Perbill {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Perbill(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Perbill");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Perbill: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for Perbill {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Perbill: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for Perbill {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(Perbill(_parity_codec::Decode::decode(input)?))
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::default::Default for Perbill {
    #[inline]
    fn default() -> Perbill { Perbill(::std::default::Default::default()) }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for Perbill { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Perbill {
    #[inline]
    fn clone(&self) -> Perbill {
        { let _: ::std::clone::AssertParamIsClone<u32>; *self }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for Perbill {
    #[inline]
    fn eq(&self, other: &Perbill) -> bool {
        match *other {
            Perbill(ref __self_1_0) =>
            match *self {
                Perbill(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Perbill) -> bool {
        match *other {
            Perbill(ref __self_1_0) =>
            match *self {
                Perbill(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for Perbill {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<u32>; }
    }
}
impl Perbill {
    /// Nothing.
    pub fn zero() -> Self { Self(0) }
    /// `true` if this is nothing.
    pub fn is_zero(&self) -> bool { self.0 == 0 }
    /// Everything.
    pub fn one() -> Self { Self(1_000_000_000) }
    /// From an explicitly defined number of parts per maximum of the type.
    pub fn from_parts(x: u32) -> Self { Self(x.min(1_000_000_000)) }
    /// Converts from a percent. Equal to `x / 100`.
    pub fn from_percent(x: u32) -> Self { Self(x.min(100) * 10_000_000) }
    /// Construct new instance where `x` is in millionths. Value equivalent to `x / 1,000,000`.
    pub fn from_millionths(x: u32) -> Self { Self(x.min(1_000_000) * 1000) }
    #[cfg(feature = "std")]
    /// Construct new instance whose value is equal to `x` (between 0 and 1).
    pub fn from_fraction(x: f64) -> Self {
        Self((x.max(0.0).min(1.0) * 1_000_000_000.0) as u32)
    }
}
impl <N> ops::Mul<N> for Perbill where N: Clone + traits::As<u64> +
 ops::Rem<N, Output = N> + ops::Div<N, Output = N> + ops::Mul<N, Output = N> +
 ops::Add<N, Output = N> {
    type
    Output
    =
    N;
    fn mul(self, b: N) -> Self::Output {
        let billion = <N as traits::As<u64>>::sa(1_000_000_000);
        let part = <N as traits::As<u64>>::sa(self.0 as u64);
        let rem_multiplied_divided =
            {
                let rem = b.clone().rem(billion.clone());
                let rem_u64: u64 = rem.as_();
                let rem_multiplied_u64 = rem_u64 * self.0 as u64;
                let rem_multiplied_divided_u64 =
                    rem_multiplied_u64 / 1_000_000_000;
                traits::As::sa(rem_multiplied_divided_u64)
            };
        (b / billion) * part + rem_multiplied_divided
    }
}
#[cfg(feature = "std")]
impl From<f64> for Perbill {
    fn from(x: f64) -> Perbill { Perbill::from_fraction(x) }
}
#[cfg(feature = "std")]
impl From<f32> for Perbill {
    fn from(x: f32) -> Perbill { Perbill::from_fraction(x as f64) }
}
impl codec::CompactAs for Perbill {
    type
    As
    =
    u32;
    fn encode_as(&self) -> &u32 { &self.0 }
    fn decode_from(x: u32) -> Perbill { Perbill(x) }
}
impl From<codec::Compact<Perbill>> for Perbill {
    fn from(x: codec::Compact<Perbill>) -> Perbill { x.0 }
}
/// PerU128 is parts-per-u128-max-value. It stores a value between 0 and 1 in fixed point and
/// provides a means to multiply some other value by that.
#[structural_match]
#[rustc_copy_clone_marker]
pub struct PerU128(u128);
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_PerU128: () =
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
        impl _serde::Serialize for PerU128 {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "PerU128",
                                                             &self.0)
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_PerU128: () =
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
        impl <'de> _serde::Deserialize<'de> for PerU128 {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<PerU128>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    PerU128;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct PerU128")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: u128 =
                            match <u128 as
                                      _serde::Deserialize>::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(PerU128(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<u128>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"tuple struct PerU128 with 1 element"));
                                }
                            };
                        _serde::export::Ok(PerU128(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "PerU128",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<PerU128>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for PerU128 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PerU128(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("PerU128");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_PerU128: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for PerU128 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_PerU128: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for PerU128 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(PerU128(_parity_codec::Decode::decode(input)?))
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::default::Default for PerU128 {
    #[inline]
    fn default() -> PerU128 { PerU128(::std::default::Default::default()) }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for PerU128 { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for PerU128 {
    #[inline]
    fn clone(&self) -> PerU128 {
        { let _: ::std::clone::AssertParamIsClone<u128>; *self }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for PerU128 {
    #[inline]
    fn eq(&self, other: &PerU128) -> bool {
        match *other {
            PerU128(ref __self_1_0) =>
            match *self {
                PerU128(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &PerU128) -> bool {
        match *other {
            PerU128(ref __self_1_0) =>
            match *self {
                PerU128(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for PerU128 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<u128>; }
    }
}
const U128: u128 = u128::max_value();
impl PerU128 {
    /// Nothing.
    pub fn zero() -> Self { Self(0) }
    /// `true` if this is nothing.
    pub fn is_zero(&self) -> bool { self.0 == 0 }
    /// Everything.
    pub fn one() -> Self { Self(U128) }
    /// From an explicitly defined number of parts per maximum of the type.
    pub fn from_parts(x: u128) -> Self { Self(x) }
    /// Construct new instance where `x` is denominator and the nominator is 1.
    pub fn from_xth(x: u128) -> Self { Self(U128 / x.max(1)) }
}
impl ::rstd::ops::Deref for PerU128 {
    type
    Target
    =
    u128;
    fn deref(&self) -> &u128 { &self.0 }
}
impl codec::CompactAs for PerU128 {
    type
    As
    =
    u128;
    fn encode_as(&self) -> &u128 { &self.0 }
    fn decode_from(x: u128) -> PerU128 { Self(x) }
}
impl From<codec::Compact<PerU128>> for PerU128 {
    fn from(x: codec::Compact<PerU128>) -> PerU128 { x.0 }
}
/// Signature verify that can work with any known signature types..
#[structural_match]
pub enum MultiSignature {

    /// An Ed25519 signature.
    Ed25519(ed25519::Signature),

    /// An Sr25519 signature.
    Sr25519(sr25519::Signature),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for MultiSignature {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<ed25519::Signature>;
            let _: ::std::cmp::AssertParamIsEq<sr25519::Signature>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for MultiSignature {
    #[inline]
    fn eq(&self, other: &MultiSignature) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSignature::Ed25519(ref __self_0),
                     &MultiSignature::Ed25519(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&MultiSignature::Sr25519(ref __self_0),
                     &MultiSignature::Sr25519(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &MultiSignature) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSignature::Ed25519(ref __self_0),
                     &MultiSignature::Ed25519(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&MultiSignature::Sr25519(ref __self_0),
                     &MultiSignature::Sr25519(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for MultiSignature {
    #[inline]
    fn clone(&self) -> MultiSignature {
        match (&*self,) {
            (&MultiSignature::Ed25519(ref __self_0),) =>
            MultiSignature::Ed25519(::std::clone::Clone::clone(&(*__self_0))),
            (&MultiSignature::Sr25519(ref __self_0),) =>
            MultiSignature::Sr25519(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_MultiSignature: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for MultiSignature {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    MultiSignature::Ed25519(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    MultiSignature::Sr25519(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_MultiSignature: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for MultiSignature {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(MultiSignature::Ed25519(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(MultiSignature::Sr25519(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for MultiSignature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&MultiSignature::Ed25519(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Ed25519");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&MultiSignature::Sr25519(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Sr25519");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl From<ed25519::Signature> for MultiSignature {
    fn from(x: ed25519::Signature) -> Self { MultiSignature::Ed25519(x) }
}
impl From<sr25519::Signature> for MultiSignature {
    fn from(x: sr25519::Signature) -> Self { MultiSignature::Sr25519(x) }
}
impl Default for MultiSignature {
    fn default() -> Self { MultiSignature::Ed25519(Default::default()) }
}
/// Public key for any known crypto algorithm.
#[structural_match]
pub enum MultiSigner {

    /// An Ed25519 identity.
    Ed25519(ed25519::Public),

    /// An Sr25519 identity.
    Sr25519(sr25519::Public),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for MultiSigner {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<ed25519::Public>;
            let _: ::std::cmp::AssertParamIsEq<sr25519::Public>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for MultiSigner {
    #[inline]
    fn eq(&self, other: &MultiSigner) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSigner::Ed25519(ref __self_0),
                     &MultiSigner::Ed25519(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&MultiSigner::Sr25519(ref __self_0),
                     &MultiSigner::Sr25519(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &MultiSigner) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSigner::Ed25519(ref __self_0),
                     &MultiSigner::Ed25519(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&MultiSigner::Sr25519(ref __self_0),
                     &MultiSigner::Sr25519(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Ord for MultiSigner {
    #[inline]
    fn cmp(&self, other: &MultiSigner) -> ::std::cmp::Ordering {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSigner::Ed25519(ref __self_0),
                     &MultiSigner::Ed25519(ref __arg_1_0)) =>
                    match ::std::cmp::Ord::cmp(&(*__self_0), &(*__arg_1_0)) {
                        ::std::cmp::Ordering::Equal =>
                        ::std::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                    (&MultiSigner::Sr25519(ref __self_0),
                     &MultiSigner::Sr25519(ref __arg_1_0)) =>
                    match ::std::cmp::Ord::cmp(&(*__self_0), &(*__arg_1_0)) {
                        ::std::cmp::Ordering::Equal =>
                        ::std::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { __self_vi.cmp(&__arg_1_vi) }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialOrd for MultiSigner {
    #[inline]
    fn partial_cmp(&self, other: &MultiSigner)
     -> ::std::option::Option<::std::cmp::Ordering> {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSigner::Ed25519(ref __self_0),
                     &MultiSigner::Ed25519(ref __arg_1_0)) =>
                    match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                              &(*__arg_1_0)) {
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                        =>
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                    (&MultiSigner::Sr25519(ref __self_0),
                     &MultiSigner::Sr25519(ref __arg_1_0)) =>
                    match ::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                              &(*__arg_1_0)) {
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal)
                        =>
                        ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                        cmp => cmp,
                    },
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { __self_vi.partial_cmp(&__arg_1_vi) }
        }
    }
    #[inline]
    fn lt(&self, other: &MultiSigner) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSigner::Ed25519(ref __self_0),
                     &MultiSigner::Ed25519(ref __arg_1_0)) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                         &(*__arg_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        == ::std::cmp::Ordering::Less,
                    (&MultiSigner::Sr25519(ref __self_0),
                     &MultiSigner::Sr25519(ref __arg_1_0)) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                         &(*__arg_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        == ::std::cmp::Ordering::Less,
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { __self_vi.lt(&__arg_1_vi) }
        }
    }
    #[inline]
    fn le(&self, other: &MultiSigner) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSigner::Ed25519(ref __self_0),
                     &MultiSigner::Ed25519(ref __arg_1_0)) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                         &(*__arg_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        != ::std::cmp::Ordering::Greater,
                    (&MultiSigner::Sr25519(ref __self_0),
                     &MultiSigner::Sr25519(ref __arg_1_0)) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                         &(*__arg_1_0)),
                                                     ::std::cmp::Ordering::Greater)
                        != ::std::cmp::Ordering::Greater,
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { __self_vi.le(&__arg_1_vi) }
        }
    }
    #[inline]
    fn gt(&self, other: &MultiSigner) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSigner::Ed25519(ref __self_0),
                     &MultiSigner::Ed25519(ref __arg_1_0)) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                         &(*__arg_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        == ::std::cmp::Ordering::Greater,
                    (&MultiSigner::Sr25519(ref __self_0),
                     &MultiSigner::Sr25519(ref __arg_1_0)) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                         &(*__arg_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        == ::std::cmp::Ordering::Greater,
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { __self_vi.gt(&__arg_1_vi) }
        }
    }
    #[inline]
    fn ge(&self, other: &MultiSigner) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&MultiSigner::Ed25519(ref __self_0),
                     &MultiSigner::Ed25519(ref __arg_1_0)) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                         &(*__arg_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        != ::std::cmp::Ordering::Less,
                    (&MultiSigner::Sr25519(ref __self_0),
                     &MultiSigner::Sr25519(ref __arg_1_0)) =>
                    ::std::option::Option::unwrap_or(::std::cmp::PartialOrd::partial_cmp(&(*__self_0),
                                                                                         &(*__arg_1_0)),
                                                     ::std::cmp::Ordering::Less)
                        != ::std::cmp::Ordering::Less,
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { __self_vi.ge(&__arg_1_vi) }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for MultiSigner {
    #[inline]
    fn clone(&self) -> MultiSigner {
        match (&*self,) {
            (&MultiSigner::Ed25519(ref __self_0),) =>
            MultiSigner::Ed25519(::std::clone::Clone::clone(&(*__self_0))),
            (&MultiSigner::Sr25519(ref __self_0),) =>
            MultiSigner::Sr25519(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_MultiSigner: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for MultiSigner {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    MultiSigner::Ed25519(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    MultiSigner::Sr25519(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_MultiSigner: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for MultiSigner {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(MultiSigner::Ed25519(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(MultiSigner::Sr25519(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for MultiSigner {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&MultiSigner::Ed25519(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Ed25519");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&MultiSigner::Sr25519(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Sr25519");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_MultiSigner: () =
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
        impl _serde::Serialize for MultiSigner {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    MultiSigner::Ed25519(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "MultiSigner",
                                                                  0u32,
                                                                  "Ed25519",
                                                                  __field0),
                    MultiSigner::Sr25519(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "MultiSigner",
                                                                  1u32,
                                                                  "Sr25519",
                                                                  __field0),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_MultiSigner: () =
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
        impl <'de> _serde::Deserialize<'de> for MultiSigner {
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
                                                             "variant identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"variant index 0 <= i < 2")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "Ed25519" =>
                            _serde::export::Ok(__Field::__field0),
                            "Sr25519" =>
                            _serde::export::Ok(__Field::__field1),
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
                            b"Ed25519" =>
                            _serde::export::Ok(__Field::__field0),
                            b"Sr25519" =>
                            _serde::export::Ok(__Field::__field1),
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
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<MultiSigner>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    MultiSigner;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "enum MultiSigner")
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
                            (__Field::__field0, __variant) =>
                            _serde::export::Result::map(_serde::de::VariantAccess::newtype_variant::<ed25519::Public>(__variant),
                                                        MultiSigner::Ed25519),
                            (__Field::__field1, __variant) =>
                            _serde::export::Result::map(_serde::de::VariantAccess::newtype_variant::<sr25519::Public>(__variant),
                                                        MultiSigner::Sr25519),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] =
                    &["Ed25519", "Sr25519"];
                _serde::Deserializer::deserialize_enum(__deserializer,
                                                       "MultiSigner",
                                                       VARIANTS,
                                                       __Visitor{marker:
                                                                     _serde::export::PhantomData::<MultiSigner>,
                                                                 lifetime:
                                                                     _serde::export::PhantomData,})
            }
        }
    };
impl Default for MultiSigner {
    fn default() -> Self { MultiSigner::Ed25519(Default::default()) }
}
/// NOTE: This implementations is required by `SimpleAddressDeterminator`,
/// we convert the hash into some AccountId, it's fine to use any scheme.
impl <T: Into<H256>> crypto::UncheckedFrom<T> for MultiSigner {
    fn unchecked_from(x: T) -> Self {
        ed25519::Public::unchecked_from(x.into()).into()
    }
}
impl AsRef<[u8]> for MultiSigner {
    fn as_ref(&self) -> &[u8] {
        match *self {
            MultiSigner::Ed25519(ref who) => who.as_ref(),
            MultiSigner::Sr25519(ref who) => who.as_ref(),
        }
    }
}
impl From<ed25519::Public> for MultiSigner {
    fn from(x: ed25519::Public) -> Self { MultiSigner::Ed25519(x) }
}
impl From<sr25519::Public> for MultiSigner {
    fn from(x: sr25519::Public) -> Self { MultiSigner::Sr25519(x) }
}
#[cfg(feature = "std")]
impl std::fmt::Display for MultiSigner {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MultiSigner::Ed25519(ref who) =>
            fmt.write_fmt(::std::fmt::Arguments::new_v1(&["ed25519: "],
                                                        &match (&who,) {
                                                             (arg0,) =>
                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                          ::std::fmt::Display::fmt)],
                                                         })),
            MultiSigner::Sr25519(ref who) =>
            fmt.write_fmt(::std::fmt::Arguments::new_v1(&["sr25519: "],
                                                        &match (&who,) {
                                                             (arg0,) =>
                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                          ::std::fmt::Display::fmt)],
                                                         })),
        }
    }
}
impl Verify for MultiSignature {
    type
    Signer
    =
    MultiSigner;
    fn verify<L: Lazy<[u8]>>(&self, msg: L, signer: &Self::Signer) -> bool {
        match (self, signer) {
            (MultiSignature::Ed25519(ref sig), &MultiSigner::Ed25519(ref who))
            => sig.verify(msg, who),
            (MultiSignature::Sr25519(ref sig), &MultiSigner::Sr25519(ref who))
            => sig.verify(msg, who),
            _ => false,
        }
    }
}
/// Signature verify that can work with any known signature types..
#[structural_match]
pub struct AnySignature(H512);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for AnySignature {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<H512>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for AnySignature {
    #[inline]
    fn eq(&self, other: &AnySignature) -> bool {
        match *other {
            AnySignature(ref __self_1_0) =>
            match *self {
                AnySignature(ref __self_0_0) =>
                (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &AnySignature) -> bool {
        match *other {
            AnySignature(ref __self_1_0) =>
            match *self {
                AnySignature(ref __self_0_0) =>
                (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for AnySignature {
    #[inline]
    fn clone(&self) -> AnySignature {
        match *self {
            AnySignature(ref __self_0_0) =>
            AnySignature(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::default::Default for AnySignature {
    #[inline]
    fn default() -> AnySignature {
        AnySignature(::std::default::Default::default())
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_AnySignature: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for AnySignature {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_AnySignature: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for AnySignature {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(AnySignature(_parity_codec::Decode::decode(input)?))
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for AnySignature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AnySignature(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("AnySignature");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_AnySignature: () =
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
        impl _serde::Serialize for AnySignature {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                _serde::Serializer::serialize_newtype_struct(__serializer,
                                                             "AnySignature",
                                                             &self.0)
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_AnySignature: () =
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
        impl <'de> _serde::Deserialize<'de> for AnySignature {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<AnySignature>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type
                    Value
                    =
                    AnySignature;
                    fn expecting(&self,
                                 __formatter: &mut _serde::export::Formatter)
                     -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter,
                                                             "tuple struct AnySignature")
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(self, __e: __E)
                     -> _serde::export::Result<Self::Value, __E::Error> where
                     __E: _serde::Deserializer<'de> {
                        let __field0: H512 =
                            match <H512 as
                                      _serde::Deserialize>::deserialize(__e) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        _serde::export::Ok(AnySignature(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::SeqAccess<'de> {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<H512>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"tuple struct AnySignature with 1 element"));
                                }
                            };
                        _serde::export::Ok(AnySignature(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(__deserializer,
                                                                 "AnySignature",
                                                                 __Visitor{marker:
                                                                               _serde::export::PhantomData::<AnySignature>,
                                                                           lifetime:
                                                                               _serde::export::PhantomData,})
            }
        }
    };
impl Verify for AnySignature {
    type
    Signer
    =
    sr25519::Public;
    fn verify<L: Lazy<[u8]>>(&self, mut msg: L, signer: &sr25519::Public)
     -> bool {
        runtime_io::sr25519_verify(self.0.as_fixed_bytes(), msg.get(),
                                   &signer.0) ||
            runtime_io::ed25519_verify(self.0.as_fixed_bytes(), msg.get(),
                                       &signer.0)
    }
}
impl From<sr25519::Signature> for AnySignature {
    fn from(s: sr25519::Signature) -> Self { AnySignature(s.into()) }
}
impl From<ed25519::Signature> for AnySignature {
    fn from(s: ed25519::Signature) -> Self { AnySignature(s.into()) }
}
#[repr(u8)]
/// Outcome of a valid extrinsic application. Capable of being sliced.
#[structural_match]
#[rustc_copy_clone_marker]
pub enum ApplyOutcome {

    /// Successful application (extrinsic reported no issue).
    Success = 0,

    /// Failed application (extrinsic was probably a no-op other than fees).
    Fail = 1,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for ApplyOutcome {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for ApplyOutcome {
    #[inline]
    fn eq(&self, other: &ApplyOutcome) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    u8;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    u8;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) { _ => true, }
            } else { false }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for ApplyOutcome {
    #[inline]
    fn clone(&self) -> ApplyOutcome { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for ApplyOutcome { }
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_ApplyOutcome: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for ApplyOutcome {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0 as u8 => { Some(ApplyOutcome::Success) }
                    x if x == 1 as u8 => { Some(ApplyOutcome::Fail) }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for ApplyOutcome {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&ApplyOutcome::Success,) => {
                let mut debug_trait_builder = f.debug_tuple("Success");
                debug_trait_builder.finish()
            }
            (&ApplyOutcome::Fail,) => {
                let mut debug_trait_builder = f.debug_tuple("Fail");
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ApplyOutcome: () =
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
        impl _serde::Serialize for ApplyOutcome {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    ApplyOutcome::Success =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "ApplyOutcome",
                                                               0u32,
                                                               "Success"),
                    ApplyOutcome::Fail =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "ApplyOutcome",
                                                               1u32, "Fail"),
                }
            }
        }
    };
impl codec::Encode for ApplyOutcome {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        f(&[*self as u8])
    }
}
#[repr(u8)]
/// Reason why an extrinsic couldn't be applied (i.e. invalid extrinsic).
#[structural_match]
#[rustc_copy_clone_marker]
pub enum ApplyError {

    /// Bad signature.
    BadSignature = 0,

    /// Nonce too low.
    Stale = 1,

    /// Nonce too high.
    Future = 2,

    /// Sending account had too low a balance.
    CantPay = 3,

    /// Block is full, no more extrinsics can be applied.
    FullBlock = 255,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for ApplyError {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for ApplyError {
    #[inline]
    fn eq(&self, other: &ApplyError) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    u8;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    u8;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) { _ => true, }
            } else { false }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for ApplyError {
    #[inline]
    fn clone(&self) -> ApplyError { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::marker::Copy for ApplyError { }
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_ApplyError: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for ApplyError {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0 as u8 => { Some(ApplyError::BadSignature) }
                    x if x == 1 as u8 => { Some(ApplyError::Stale) }
                    x if x == 2 as u8 => { Some(ApplyError::Future) }
                    x if x == 3 as u8 => { Some(ApplyError::CantPay) }
                    x if x == 255 as u8 => { Some(ApplyError::FullBlock) }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for ApplyError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&ApplyError::BadSignature,) => {
                let mut debug_trait_builder = f.debug_tuple("BadSignature");
                debug_trait_builder.finish()
            }
            (&ApplyError::Stale,) => {
                let mut debug_trait_builder = f.debug_tuple("Stale");
                debug_trait_builder.finish()
            }
            (&ApplyError::Future,) => {
                let mut debug_trait_builder = f.debug_tuple("Future");
                debug_trait_builder.finish()
            }
            (&ApplyError::CantPay,) => {
                let mut debug_trait_builder = f.debug_tuple("CantPay");
                debug_trait_builder.finish()
            }
            (&ApplyError::FullBlock,) => {
                let mut debug_trait_builder = f.debug_tuple("FullBlock");
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ApplyError: () =
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
        impl _serde::Serialize for ApplyError {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    ApplyError::BadSignature =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "ApplyError",
                                                               0u32,
                                                               "BadSignature"),
                    ApplyError::Stale =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "ApplyError",
                                                               1u32, "Stale"),
                    ApplyError::Future =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "ApplyError",
                                                               2u32,
                                                               "Future"),
                    ApplyError::CantPay =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "ApplyError",
                                                               3u32,
                                                               "CantPay"),
                    ApplyError::FullBlock =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "ApplyError",
                                                               4u32,
                                                               "FullBlock"),
                }
            }
        }
    };
impl codec::Encode for ApplyError {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        f(&[*self as u8])
    }
}
/// Result from attempt to apply an extrinsic.
pub type ApplyResult = Result<ApplyOutcome, ApplyError>;
/// Verify a signature on an encoded value in a lazy manner. This can be
/// an optimization if the signature scheme has an "unsigned" escape hash.
pub fn verify_encoded_lazy<V: Verify,
                           T: codec::Encode>(sig: &V, item: &T,
                                             signer: &V::Signer) -> bool {
    struct LazyEncode<F> {
        inner: F,
        encoded: Option<Vec<u8>>,
    }
    impl <F: Fn() -> Vec<u8>> traits::Lazy<[u8]> for LazyEncode<F> {
        fn get(&mut self) -> &[u8] {
            self.encoded.get_or_insert_with(&self.inner).as_slice()
        }
    }
    sig.verify(LazyEncode{inner: || item.encode(), encoded: None,}, signer)
}
/// Helper macro for `impl_outer_config`
#[macro_export]
macro_rules! __impl_outer_config_types((
                                       $ concrete : ident $ config : ident $
                                       snake : ident < $ ignore : ident , $
                                       instance : path > $ ( $ rest : tt ) * )
                                       => {
                                       # [
                                       cfg ( any ( feature = "std" , test ) )
                                       ] pub type $ config = $ snake ::
                                       GenesisConfig < $ concrete , $ instance
                                       > ; $ crate ::
                                       __impl_outer_config_types ! {
                                       $ concrete $ ( $ rest ) * } } ; (
                                       $ concrete : ident $ config : ident $
                                       snake : ident < $ ignore : ident > $ (
                                       $ rest : tt ) * ) => {
                                       # [
                                       cfg ( any ( feature = "std" , test ) )
                                       ] pub type $ config = $ snake ::
                                       GenesisConfig < $ concrete > ; $ crate
                                       :: __impl_outer_config_types ! {
                                       $ concrete $ ( $ rest ) * } } ; (
                                       $ concrete : ident $ config : ident $
                                       snake : ident $ ( $ rest : tt ) * ) =>
                                       {
                                       # [
                                       cfg ( any ( feature = "std" , test ) )
                                       ] pub type $ config = $ snake ::
                                       GenesisConfig ;
                                       __impl_outer_config_types ! {
                                       $ concrete $ ( $ rest ) * } } ; (
                                       $ concrete : ident ) => (  ));
/// Implement the output "meta" module configuration struct,
/// which is basically:
/// pub struct GenesisConfig {
/// 	rust_module_one: Option<ModuleOneConfig>,
/// 	...
/// }
#[macro_export]
macro_rules! impl_outer_config((
                               pub struct $ main : ident for $ concrete :
                               ident {
                               $ (
                               $ config : ident => $ snake : ident $ (
                               < $ generic : ident $ ( , $ instance : path ) ?
                               > ) * , ) * } ) => {
                               $ crate :: __impl_outer_config_types ! {
                               $ concrete $ (
                               $ config $ snake $ (
                               < $ generic $ ( , $ instance ) ? > ) * ) * } #
                               [ cfg ( any ( feature = "std" , test ) ) ] # [
                               derive (
                               $ crate :: serde :: Serialize , $ crate ::
                               serde :: Deserialize ) ] # [
                               serde ( rename_all = "camelCase" ) ] # [
                               serde ( deny_unknown_fields ) ] pub struct $
                               main {
                               $ ( pub $ snake : Option < $ config > , ) * } #
                               [ cfg ( any ( feature = "std" , test ) ) ] impl
                               $ crate :: BuildStorage for $ main {
                               fn assimilate_storage (
                               self , top : & mut $ crate :: StorageOverlay ,
                               children : & mut $ crate ::
                               ChildrenStorageOverlay ) -> :: std :: result ::
                               Result < (  ) , String > {
                               $ (
                               if let Some ( extra ) = self . $ snake {
                               extra . assimilate_storage ( top , children ) ?
                               ; } ) * Ok ( (  ) ) } } });
/// Generates enum that contains all possible log entries for the runtime.
/// Every individual module of the runtime that is mentioned, must
/// expose a `Log` and `RawLog` enums.
///
/// Generated enum is binary-compatible with and could be interpreted
/// as `generic::DigestItem`.
///
/// Runtime requirements:
/// 1) binary representation of all supported 'system' log items should stay
///    the same. Otherwise, the native code will be unable to read log items
///    generated by previous runtime versions
/// 2) the support of 'system' log items should never be dropped by runtime.
///    Otherwise, native code will lost its ability to read items of this type
///    even if they were generated by the versions which have supported these
///    items.
#[macro_export]
macro_rules! impl_outer_log((
                            $ ( # [ $ attr : meta ] ) * pub enum $ name :
                            ident (
                            $ internal : ident : DigestItem < $ (
                            $ genarg : ty ) , * > ) for $ trait : ident {
                            $ (
                            $ module : ident $ ( < $ instance : path > ) ? (
                            $ ( $ sitem : ident ) , * ) ) , * } ) => {
                            /// Wrapper for all possible log entries for the `$trait` runtime. Provides binary-compatible

                            /// `Encode`/`Decode` implementations with the corresponding `generic::DigestItem`.
                             # [ derive ( Clone , PartialEq , Eq ) ] # [
                            cfg_attr (
                            feature = "std" , derive (
                            Debug , $ crate :: serde :: Serialize ) ) ] $ (
                            # [ $ attr ] ) * # [
                            allow ( non_camel_case_types ) ] pub struct $ name
                            ( $ internal ) ;
                            /// All possible log entries for the `$trait` runtime. `Encode`/`Decode` implementations

                            /// are auto-generated => it is not binary-compatible with `generic::DigestItem`.
                             # [
                            derive (
                            Clone , PartialEq , Eq , $ crate :: codec ::
                            Encode , $ crate :: codec :: Decode ) ] # [
                            cfg_attr (
                            feature = "std" , derive (
                            Debug , $ crate :: serde :: Serialize ) ) ] $ (
                            # [ $ attr ] ) * # [
                            allow ( non_camel_case_types ) ] pub enum
                            InternalLog {
                            $ (
                            $ module (
                            $ module :: Log < $ trait $ ( , $ instance ) ? > )
                            , ) * } impl $ name {
                            /// Try to convert `$name` into `generic::DigestItemRef`. Returns Some when

                            /// `self` is a 'system' log && it has been marked as 'system' in macro call.
                             /// Otherwise, None is returned.
                             # [ allow ( unreachable_patterns ) ] fn dref < 'a
                            > ( & 'a self ) -> Option < $ crate :: generic ::
                            DigestItemRef < 'a , $ ( $ genarg ) , * >> {
                            match self . 0 {
                            $ (
                            $ (
                            $ internal :: $ module (
                            $ module :: RawLog :: $ sitem ( ref v ) ) => Some
                            (
                            $ crate :: generic :: DigestItemRef :: $ sitem ( v
                            ) ) , ) * ) * _ => None , } } } impl $ crate ::
                            traits :: DigestItem for $ name {
                            type Hash = < $ crate :: generic :: DigestItem < $
                            ( $ genarg ) , * > as $ crate :: traits ::
                            DigestItem > :: Hash ; type AuthorityId = < $
                            crate :: generic :: DigestItem < $ ( $ genarg ) ,
                            * > as $ crate :: traits :: DigestItem > ::
                            AuthorityId ; fn as_authorities_change ( & self )
                            -> Option < & [ Self :: AuthorityId ] > {
                            self . dref (  ) . and_then (
                            | dref | dref . as_authorities_change (  ) ) } fn
                            as_changes_trie_root ( & self ) -> Option < & Self
                            :: Hash > {
                            self . dref (  ) . and_then (
                            | dref | dref . as_changes_trie_root (  ) ) } }
                            impl From < $ crate :: generic :: DigestItem < $ (
                            $ genarg ) , * >> for $ name {
                            /// Converts `generic::DigestItem` into `$name`. If `generic::DigestItem` represents

                            /// a system item which is supported by the runtime, it is returned.

                            /// Otherwise we expect a `Other` log item. Trying to convert from anything other

                            /// will lead to panic in runtime, since the runtime does not supports this 'system'
                             /// log item.
                             # [ allow ( unreachable_patterns ) ] fn from (
                            gen : $ crate :: generic :: DigestItem < $ (
                            $ genarg ) , * > ) -> Self {
                            match gen {
                            $ (
                            $ (
                            $ crate :: generic :: DigestItem :: $ sitem (
                            value ) => $ name (
                            $ internal :: $ module (
                            $ module :: RawLog :: $ sitem ( value ) ) ) , ) *
                            ) * _ => gen . as_other (  ) . and_then (
                            | value | $ crate :: codec :: Decode :: decode (
                            & mut & value [ .. ] ) ) . map ( $ name ) . expect
                            ( "not allowed to fail in runtime" ) , } } } impl
                            $ crate :: codec :: Decode for $ name {
                            /// `generic::DigestItem` binary compatible decode.
                             fn decode < I : $ crate :: codec :: Input > (
                            input : & mut I ) -> Option < Self > {
                            let gen : $ crate :: generic :: DigestItem < $ (
                            $ genarg ) , * > = $ crate :: codec :: Decode ::
                            decode ( input ) ? ; Some ( $ name :: from ( gen )
                            ) } } impl $ crate :: codec :: Encode for $ name {
                            /// `generic::DigestItem` binary compatible encode.
                             fn encode ( & self ) -> Vec < u8 > {
                            match self . dref (  ) {
                            Some ( dref ) => dref . encode (  ) , None => {
                            let gen : $ crate :: generic :: DigestItem < $ (
                            $ genarg ) , * > = $ crate :: generic ::
                            DigestItem :: Other ( self . 0 . encode (  ) ) ;
                            gen . encode (  ) } , } } } $ (
                            impl From < $ module :: Log < $ trait $ (
                            , $ instance ) ? >> for $ name {
                            /// Converts single module log item into `$name`.
                             fn from (
                            x : $ module :: Log < $ trait $ ( , $ instance ) ?
                            > ) -> Self { $ name ( x . into (  ) ) } } impl
                            From < $ module :: Log < $ trait $ ( , $ instance
                            ) ? >> for InternalLog {
                            /// Converts single module log item into `$internal`.
                             fn from (
                            x : $ module :: Log < $ trait $ ( , $ instance ) ?
                            > ) -> Self { InternalLog :: $ module ( x ) } } )
                            * } ;);
/// Simple blob to hold an extrinsic without committing to its format and ensure it is serialized
/// correctly.
#[structural_match]
pub struct OpaqueExtrinsic(pub Vec<u8>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for OpaqueExtrinsic {
    #[inline]
    fn eq(&self, other: &OpaqueExtrinsic) -> bool {
        match *other {
            OpaqueExtrinsic(ref __self_1_0) =>
            match *self {
                OpaqueExtrinsic(ref __self_0_0) =>
                (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &OpaqueExtrinsic) -> bool {
        match *other {
            OpaqueExtrinsic(ref __self_1_0) =>
            match *self {
                OpaqueExtrinsic(ref __self_0_0) =>
                (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for OpaqueExtrinsic {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<Vec<u8>>; }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for OpaqueExtrinsic {
    #[inline]
    fn clone(&self) -> OpaqueExtrinsic {
        match *self {
            OpaqueExtrinsic(ref __self_0_0) =>
            OpaqueExtrinsic(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::default::Default for OpaqueExtrinsic {
    #[inline]
    fn default() -> OpaqueExtrinsic {
        OpaqueExtrinsic(::std::default::Default::default())
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_OpaqueExtrinsic: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for OpaqueExtrinsic {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_OpaqueExtrinsic: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for OpaqueExtrinsic {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(OpaqueExtrinsic(_parity_codec::Decode::decode(input)?))
            }
        }
    };
#[cfg(feature = "std")]
impl std::fmt::Debug for OpaqueExtrinsic {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                    &match (&substrate_primitives::hexdisplay::HexDisplay::from(&self.0),)
                                                         {
                                                         (arg0,) =>
                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                      ::std::fmt::Display::fmt)],
                                                     }))
    }
}
#[cfg(feature = "std")]
impl ::serde::Serialize for OpaqueExtrinsic {
    fn serialize<S>(&self, seq: S) -> Result<S::Ok, S::Error> where
     S: ::serde::Serializer {
        codec::Encode::using_encoded(&self.0,
                                     |bytes|
                                         ::substrate_primitives::bytes::serialize(bytes,
                                                                                  seq))
    }
}
impl traits::Extrinsic for OpaqueExtrinsic {
    fn is_signed(&self) -> Option<bool> { None }
}
