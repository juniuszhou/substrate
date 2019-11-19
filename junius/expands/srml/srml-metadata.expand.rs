#![feature(prelude_import)]
#![no_std]
// Copyright 2018-2019 Parity Technologies (UK) Ltd.
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

//! Decodable variant of the RuntimeMetadata.
//!
//! This really doesn't belong here, but is necessary for the moment. In the future
//! it should be removed entirely to an external module for shimming on to the
//! codec-encoded metadata.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[cfg(feature = "std")]
use serde::Serialize;
#[cfg(feature = "std")]
use parity_codec::{Decode, Input};
use parity_codec::{Encode, Output};
use rstd::vec::Vec;

#[cfg(feature = "std")]
type StringBuf = String;

/// Curent prefix of metadata
pub const META_RESERVED: u32 = 0x6174656d;
// 'meta' warn endianness


/// A type that decodes to a different type than it encodes.
/// The user needs to make sure that both types use the same encoding.
///
/// For example a `&'static [ &'static str ]` can be decoded to a `Vec<String>`.
pub enum DecodeDifferent<B, O> where B: 'static, O: 'static {
    Encode(B),
    Decoded(O),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <B: ::std::clone::Clone, O: ::std::clone::Clone> ::std::clone::Clone for
 DecodeDifferent<B, O> where B: 'static, O: 'static {
    #[inline]
    fn clone(&self) -> DecodeDifferent<B, O> {
        match (&*self,) {
            (&DecodeDifferent::Encode(ref __self_0),) =>
            DecodeDifferent::Encode(::std::clone::Clone::clone(&(*__self_0))),
            (&DecodeDifferent::Decoded(ref __self_0),) =>
            DecodeDifferent::Decoded(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}

impl <B, O> Encode for DecodeDifferent<B, O> where B: Encode + 'static,
 O: Encode + 'static {
    fn encode_to<W: Output>(&self, dest: &mut W) {
        match self {
            DecodeDifferent::Encode(b) => b.encode_to(dest),
            DecodeDifferent::Decoded(o) => o.encode_to(dest),
        }
    }
}

#[cfg(feature = "std")]
impl <B, O> Decode for DecodeDifferent<B, O> where B: 'static, O: Decode +
 'static {
    fn decode<I: Input>(input: &mut I) -> Option<Self> {
        <O>::decode(input).and_then(|val|
                                        {
                                            Some(DecodeDifferent::Decoded(val))
                                        })
    }
}

impl <B, O> PartialEq for DecodeDifferent<B, O> where B: Encode + Eq +
 PartialEq + 'static, O: Encode + Eq + PartialEq + 'static {
    fn eq(&self, other: &Self) -> bool { self.encode() == other.encode() }
}

impl <B, O> Eq for DecodeDifferent<B, O> where B: Encode + Eq + PartialEq +
 'static, O: Encode + Eq + PartialEq + 'static {
}

#[cfg(feature = "std")]
impl <B, O> std::fmt::Debug for DecodeDifferent<B, O> where
 B: std::fmt::Debug + Eq + 'static, O: std::fmt::Debug + Eq + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DecodeDifferent::Encode(b) => b.fmt(f),
            DecodeDifferent::Decoded(o) => o.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl <B, O> serde::Serialize for DecodeDifferent<B, O> where
 B: serde::Serialize + 'static, O: serde::Serialize + 'static {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
     S: serde::Serializer {
        match self {
            DecodeDifferent::Encode(b) => b.serialize(serializer),
            DecodeDifferent::Decoded(o) => o.serialize(serializer),
        }
    }
}

pub type DecodeDifferentArray<B, O = B>
    =
    DecodeDifferent<&'static [B], Vec<O>>;

#[cfg(feature = "std")]
type DecodeDifferentStr = DecodeDifferent<&'static str, StringBuf>;

/// All the metadata about a function.
#[structural_match]
pub struct FunctionMetadata {
    pub name: DecodeDifferentStr,
    pub arguments: DecodeDifferentArray<FunctionArgumentMetadata>,
    pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for FunctionMetadata {
    #[inline]
    fn clone(&self) -> FunctionMetadata {
        match *self {
            FunctionMetadata {
            name: ref __self_0_0,
            arguments: ref __self_0_1,
            documentation: ref __self_0_2 } =>
            FunctionMetadata{name: ::std::clone::Clone::clone(&(*__self_0_0)),
                             arguments:
                                 ::std::clone::Clone::clone(&(*__self_0_1)),
                             documentation:
                                 ::std::clone::Clone::clone(&(*__self_0_2)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for FunctionMetadata {
    #[inline]
    fn eq(&self, other: &FunctionMetadata) -> bool {
        match *other {
            FunctionMetadata {
            name: ref __self_1_0,
            arguments: ref __self_1_1,
            documentation: ref __self_1_2 } =>
            match *self {
                FunctionMetadata {
                name: ref __self_0_0,
                arguments: ref __self_0_1,
                documentation: ref __self_0_2 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &FunctionMetadata) -> bool {
        match *other {
            FunctionMetadata {
            name: ref __self_1_0,
            arguments: ref __self_1_1,
            documentation: ref __self_1_2 } =>
            match *self {
                FunctionMetadata {
                name: ref __self_0_0,
                arguments: ref __self_0_1,
                documentation: ref __self_0_2 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for FunctionMetadata {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _:
                    ::std::cmp::AssertParamIsEq<DecodeDifferentArray<FunctionArgumentMetadata>>;
            let _:
                    ::std::cmp::AssertParamIsEq<DecodeDifferentArray<&'static str,
                                                                     StringBuf>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_FunctionMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for FunctionMetadata {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.name);
                dest.push(&self.arguments);
                dest.push(&self.documentation);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_FunctionMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for FunctionMetadata {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(FunctionMetadata{name:
                                          _parity_codec::Decode::decode(input)?,
                                      arguments:
                                          _parity_codec::Decode::decode(input)?,
                                      documentation:
                                          _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for FunctionMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FunctionMetadata {
            name: ref __self_0_0,
            arguments: ref __self_0_1,
            documentation: ref __self_0_2 } => {
                let mut debug_trait_builder =
                    f.debug_struct("FunctionMetadata");
                let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("arguments", &&(*__self_0_1));
                let _ =
                    debug_trait_builder.field("documentation",
                                              &&(*__self_0_2));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_FunctionMetadata: () =
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
        impl _serde::Serialize for FunctionMetadata {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "FunctionMetadata",
                                                               false as usize
                                                                   + 1 + 1 +
                                                                   1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "name",
                                                                    &self.name)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "arguments",
                                                                    &self.arguments)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "documentation",
                                                                    &self.documentation)
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

/// All the metadata about a function argument.
#[structural_match]
pub struct FunctionArgumentMetadata {
    pub name: DecodeDifferentStr,
    pub ty: DecodeDifferentStr,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for FunctionArgumentMetadata {
    #[inline]
    fn clone(&self) -> FunctionArgumentMetadata {
        match *self {
            FunctionArgumentMetadata {
            name: ref __self_0_0, ty: ref __self_0_1 } =>
            FunctionArgumentMetadata{name:
                                         ::std::clone::Clone::clone(&(*__self_0_0)),
                                     ty:
                                         ::std::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for FunctionArgumentMetadata {
    #[inline]
    fn eq(&self, other: &FunctionArgumentMetadata) -> bool {
        match *other {
            FunctionArgumentMetadata {
            name: ref __self_1_0, ty: ref __self_1_1 } =>
            match *self {
                FunctionArgumentMetadata {
                name: ref __self_0_0, ty: ref __self_0_1 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &FunctionArgumentMetadata) -> bool {
        match *other {
            FunctionArgumentMetadata {
            name: ref __self_1_0, ty: ref __self_1_1 } =>
            match *self {
                FunctionArgumentMetadata {
                name: ref __self_0_0, ty: ref __self_0_1 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for FunctionArgumentMetadata {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_FunctionArgumentMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for FunctionArgumentMetadata {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.name);
                dest.push(&self.ty);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_FunctionArgumentMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for FunctionArgumentMetadata {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(FunctionArgumentMetadata{name:
                                                  _parity_codec::Decode::decode(input)?,
                                              ty:
                                                  _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for FunctionArgumentMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            FunctionArgumentMetadata {
            name: ref __self_0_0, ty: ref __self_0_1 } => {
                let mut debug_trait_builder =
                    f.debug_struct("FunctionArgumentMetadata");
                let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                let _ = debug_trait_builder.field("ty", &&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_FunctionArgumentMetadata: () =
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
        impl _serde::Serialize for FunctionArgumentMetadata {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "FunctionArgumentMetadata",
                                                               false as usize
                                                                   + 1 + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "name",
                                                                    &self.name)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "ty",
                                                                    &self.ty)
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

/// Newtype wrapper for support encoding functions (actual the result of the function).
pub struct FnEncode<E>(pub fn() -> E) where E: Encode + 'static;
#[automatically_derived]
#[allow(unused_qualifications)]
impl <E: ::std::clone::Clone> ::std::clone::Clone for FnEncode<E> where
 E: Encode + 'static {
    #[inline]
    fn clone(&self) -> FnEncode<E> {
        match *self {
            FnEncode(ref __self_0_0) =>
            FnEncode(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <E: ::std::cmp::Eq> ::std::cmp::Eq for FnEncode<E> where E: Encode +
 'static {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<fn() -> E>; }
    }
}

impl <E: Encode> Encode for FnEncode<E> {
    fn encode_to<W: Output>(&self, dest: &mut W) {
        (self.0)().encode_to(dest);
    }
}

impl <E: Encode + PartialEq> PartialEq for FnEncode<E> {
    fn eq(&self, other: &Self) -> bool { (self.0)().eq(&(other.0)()) }
}

#[cfg(feature = "std")]
impl <E: Encode + ::std::fmt::Debug> std::fmt::Debug for FnEncode<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.0)().fmt(f)
    }
}

#[cfg(feature = "std")]
impl <E: Encode + serde::Serialize> serde::Serialize for FnEncode<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
     S: serde::Serializer {
        (self.0)().serialize(serializer)
    }
}

/// All the metadata about an outer event.
#[structural_match]
pub struct OuterEventMetadata {
    pub name: DecodeDifferentStr,
    pub events: DecodeDifferentArray<(&'static str,
                                      FnEncode<&'static [EventMetadata]>),
                                     (StringBuf, Vec<EventMetadata>)>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for OuterEventMetadata {
    #[inline]
    fn clone(&self) -> OuterEventMetadata {
        match *self {
            OuterEventMetadata { name: ref __self_0_0, events: ref __self_0_1
            } =>
            OuterEventMetadata{name:
                                   ::std::clone::Clone::clone(&(*__self_0_0)),
                               events:
                                   ::std::clone::Clone::clone(&(*__self_0_1)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for OuterEventMetadata {
    #[inline]
    fn eq(&self, other: &OuterEventMetadata) -> bool {
        match *other {
            OuterEventMetadata { name: ref __self_1_0, events: ref __self_1_1
            } =>
            match *self {
                OuterEventMetadata {
                name: ref __self_0_0, events: ref __self_0_1 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &OuterEventMetadata) -> bool {
        match *other {
            OuterEventMetadata { name: ref __self_1_0, events: ref __self_1_1
            } =>
            match *self {
                OuterEventMetadata {
                name: ref __self_0_0, events: ref __self_0_1 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for OuterEventMetadata {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _:
                    ::std::cmp::AssertParamIsEq<DecodeDifferentArray<(&'static str,
                                                                      FnEncode<&'static [EventMetadata]>),
                                                                     (StringBuf,
                                                                      Vec<EventMetadata>)>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_OuterEventMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for OuterEventMetadata {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.name);
                dest.push(&self.events);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_OuterEventMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for OuterEventMetadata {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(OuterEventMetadata{name:
                                            _parity_codec::Decode::decode(input)?,
                                        events:
                                            _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for OuterEventMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            OuterEventMetadata { name: ref __self_0_0, events: ref __self_0_1
            } => {
                let mut debug_trait_builder =
                    f.debug_struct("OuterEventMetadata");
                let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                let _ = debug_trait_builder.field("events", &&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_OuterEventMetadata: () =
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
        impl _serde::Serialize for OuterEventMetadata {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "OuterEventMetadata",
                                                               false as usize
                                                                   + 1 + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "name",
                                                                    &self.name)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "events",
                                                                    &self.events)
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

/// All the metadata about a event.
#[structural_match]
pub struct EventMetadata {
    pub name: DecodeDifferentStr,
    pub arguments: DecodeDifferentArray<&'static str, StringBuf>,
    pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for EventMetadata {
    #[inline]
    fn clone(&self) -> EventMetadata {
        match *self {
            EventMetadata {
            name: ref __self_0_0,
            arguments: ref __self_0_1,
            documentation: ref __self_0_2 } =>
            EventMetadata{name: ::std::clone::Clone::clone(&(*__self_0_0)),
                          arguments:
                              ::std::clone::Clone::clone(&(*__self_0_1)),
                          documentation:
                              ::std::clone::Clone::clone(&(*__self_0_2)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for EventMetadata {
    #[inline]
    fn eq(&self, other: &EventMetadata) -> bool {
        match *other {
            EventMetadata {
            name: ref __self_1_0,
            arguments: ref __self_1_1,
            documentation: ref __self_1_2 } =>
            match *self {
                EventMetadata {
                name: ref __self_0_0,
                arguments: ref __self_0_1,
                documentation: ref __self_0_2 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &EventMetadata) -> bool {
        match *other {
            EventMetadata {
            name: ref __self_1_0,
            arguments: ref __self_1_1,
            documentation: ref __self_1_2 } =>
            match *self {
                EventMetadata {
                name: ref __self_0_0,
                arguments: ref __self_0_1,
                documentation: ref __self_0_2 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for EventMetadata {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _:
                    ::std::cmp::AssertParamIsEq<DecodeDifferentArray<&'static str,
                                                                     StringBuf>>;
            let _:
                    ::std::cmp::AssertParamIsEq<DecodeDifferentArray<&'static str,
                                                                     StringBuf>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_EventMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for EventMetadata {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.name);
                dest.push(&self.arguments);
                dest.push(&self.documentation);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_EventMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for EventMetadata {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(EventMetadata{name:
                                       _parity_codec::Decode::decode(input)?,
                                   arguments:
                                       _parity_codec::Decode::decode(input)?,
                                   documentation:
                                       _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for EventMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EventMetadata {
            name: ref __self_0_0,
            arguments: ref __self_0_1,
            documentation: ref __self_0_2 } => {
                let mut debug_trait_builder = f.debug_struct("EventMetadata");
                let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("arguments", &&(*__self_0_1));
                let _ =
                    debug_trait_builder.field("documentation",
                                              &&(*__self_0_2));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_EventMetadata: () =
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
        impl _serde::Serialize for EventMetadata {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "EventMetadata",
                                                               false as usize
                                                                   + 1 + 1 +
                                                                   1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "name",
                                                                    &self.name)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "arguments",
                                                                    &self.arguments)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "documentation",
                                                                    &self.documentation)
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

/// All the metadata about a storage.
#[structural_match]
pub struct StorageMetadata {
    pub functions: DecodeDifferentArray<StorageFunctionMetadata>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for StorageMetadata {
    #[inline]
    fn clone(&self) -> StorageMetadata {
        match *self {
            StorageMetadata { functions: ref __self_0_0 } =>
            StorageMetadata{functions:
                                ::std::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for StorageMetadata {
    #[inline]
    fn eq(&self, other: &StorageMetadata) -> bool {
        match *other {
            StorageMetadata { functions: ref __self_1_0 } =>
            match *self {
                StorageMetadata { functions: ref __self_0_0 } =>
                (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &StorageMetadata) -> bool {
        match *other {
            StorageMetadata { functions: ref __self_1_0 } =>
            match *self {
                StorageMetadata { functions: ref __self_0_0 } =>
                (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for StorageMetadata {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _:
                    ::std::cmp::AssertParamIsEq<DecodeDifferentArray<StorageFunctionMetadata>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_StorageMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for StorageMetadata {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.functions);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_StorageMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for StorageMetadata {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(StorageMetadata{functions:
                                         _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for StorageMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            StorageMetadata { functions: ref __self_0_0 } => {
                let mut debug_trait_builder =
                    f.debug_struct("StorageMetadata");
                let _ =
                    debug_trait_builder.field("functions", &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_StorageMetadata: () =
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
        impl _serde::Serialize for StorageMetadata {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "StorageMetadata",
                                                               false as usize
                                                                   + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "functions",
                                                                    &self.functions)
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

/// All the metadata about a storage function.
#[structural_match]
pub struct StorageFunctionMetadata {
    pub name: DecodeDifferentStr,
    pub modifier: StorageFunctionModifier,
    pub ty: StorageFunctionType,
    pub default: ByteGetter,
    pub documentation: DecodeDifferentArray<&'static str, StringBuf>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for StorageFunctionMetadata {
    #[inline]
    fn clone(&self) -> StorageFunctionMetadata {
        match *self {
            StorageFunctionMetadata {
            name: ref __self_0_0,
            modifier: ref __self_0_1,
            ty: ref __self_0_2,
            default: ref __self_0_3,
            documentation: ref __self_0_4 } =>
            StorageFunctionMetadata{name:
                                        ::std::clone::Clone::clone(&(*__self_0_0)),
                                    modifier:
                                        ::std::clone::Clone::clone(&(*__self_0_1)),
                                    ty:
                                        ::std::clone::Clone::clone(&(*__self_0_2)),
                                    default:
                                        ::std::clone::Clone::clone(&(*__self_0_3)),
                                    documentation:
                                        ::std::clone::Clone::clone(&(*__self_0_4)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for StorageFunctionMetadata {
    #[inline]
    fn eq(&self, other: &StorageFunctionMetadata) -> bool {
        match *other {
            StorageFunctionMetadata {
            name: ref __self_1_0,
            modifier: ref __self_1_1,
            ty: ref __self_1_2,
            default: ref __self_1_3,
            documentation: ref __self_1_4 } =>
            match *self {
                StorageFunctionMetadata {
                name: ref __self_0_0,
                modifier: ref __self_0_1,
                ty: ref __self_0_2,
                default: ref __self_0_3,
                documentation: ref __self_0_4 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3) &&
                    (*__self_0_4) == (*__self_1_4),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &StorageFunctionMetadata) -> bool {
        match *other {
            StorageFunctionMetadata {
            name: ref __self_1_0,
            modifier: ref __self_1_1,
            ty: ref __self_1_2,
            default: ref __self_1_3,
            documentation: ref __self_1_4 } =>
            match *self {
                StorageFunctionMetadata {
                name: ref __self_0_0,
                modifier: ref __self_0_1,
                ty: ref __self_0_2,
                default: ref __self_0_3,
                documentation: ref __self_0_4 } =>
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
impl ::std::cmp::Eq for StorageFunctionMetadata {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _: ::std::cmp::AssertParamIsEq<StorageFunctionModifier>;
            let _: ::std::cmp::AssertParamIsEq<StorageFunctionType>;
            let _: ::std::cmp::AssertParamIsEq<ByteGetter>;
            let _:
                    ::std::cmp::AssertParamIsEq<DecodeDifferentArray<&'static str,
                                                                     StringBuf>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_StorageFunctionMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for StorageFunctionMetadata {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.name);
                dest.push(&self.modifier);
                dest.push(&self.ty);
                dest.push(&self.default);
                dest.push(&self.documentation);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_StorageFunctionMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for StorageFunctionMetadata {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(StorageFunctionMetadata{name:
                                                 _parity_codec::Decode::decode(input)?,
                                             modifier:
                                                 _parity_codec::Decode::decode(input)?,
                                             ty:
                                                 _parity_codec::Decode::decode(input)?,
                                             default:
                                                 _parity_codec::Decode::decode(input)?,
                                             documentation:
                                                 _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for StorageFunctionMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            StorageFunctionMetadata {
            name: ref __self_0_0,
            modifier: ref __self_0_1,
            ty: ref __self_0_2,
            default: ref __self_0_3,
            documentation: ref __self_0_4 } => {
                let mut debug_trait_builder =
                    f.debug_struct("StorageFunctionMetadata");
                let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("modifier", &&(*__self_0_1));
                let _ = debug_trait_builder.field("ty", &&(*__self_0_2));
                let _ = debug_trait_builder.field("default", &&(*__self_0_3));
                let _ =
                    debug_trait_builder.field("documentation",
                                              &&(*__self_0_4));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_StorageFunctionMetadata: () =
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
        impl _serde::Serialize for StorageFunctionMetadata {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "StorageFunctionMetadata",
                                                               false as usize
                                                                   + 1 + 1 + 1
                                                                   + 1 + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "name",
                                                                    &self.name)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "modifier",
                                                                    &self.modifier)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "ty",
                                                                    &self.ty)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "default",
                                                                    &self.default)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "documentation",
                                                                    &self.documentation)
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

/// A technical trait to store lazy initiated vec value as static dyn pointer.
pub trait DefaultByte {
    fn default_byte(&self)
    -> Vec<u8>;
}

/// Wrapper over dyn pointer for accessing a cached once byte value.
pub struct DefaultByteGetter(pub &'static dyn DefaultByte);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for DefaultByteGetter {
    #[inline]
    fn clone(&self) -> DefaultByteGetter {
        match *self {
            DefaultByteGetter(ref __self_0_0) =>
            DefaultByteGetter(::std::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}

/// Decode different for static lazy initiated byte value.
pub type ByteGetter = DecodeDifferent<DefaultByteGetter, Vec<u8>>;

impl Encode for DefaultByteGetter {
    fn encode_to<W: Output>(&self, dest: &mut W) {
        self.0.default_byte().encode_to(dest)
    }
}

impl PartialEq<DefaultByteGetter> for DefaultByteGetter {
    fn eq(&self, other: &DefaultByteGetter) -> bool {
        let left = self.0.default_byte();
        let right = other.0.default_byte();
        left.eq(&right)
    }
}

impl Eq for DefaultByteGetter { }

#[cfg(feature = "std")]
impl serde::Serialize for DefaultByteGetter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
     S: serde::Serializer {
        self.0.default_byte().serialize(serializer)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Debug for DefaultByteGetter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.default_byte().fmt(f)
    }
}

/// Hasher used by storage maps
#[structural_match]
pub enum StorageHasher {
    Blake2_128,
    Blake2_256,
    Twox128,
    Twox256,
    Twox64Concat,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for StorageHasher {
    #[inline]
    fn clone(&self) -> StorageHasher {
        match (&*self,) {
            (&StorageHasher::Blake2_128,) => StorageHasher::Blake2_128,
            (&StorageHasher::Blake2_256,) => StorageHasher::Blake2_256,
            (&StorageHasher::Twox128,) => StorageHasher::Twox128,
            (&StorageHasher::Twox256,) => StorageHasher::Twox256,
            (&StorageHasher::Twox64Concat,) => StorageHasher::Twox64Concat,
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for StorageHasher {
    #[inline]
    fn eq(&self, other: &StorageHasher) -> bool {
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
impl ::std::cmp::Eq for StorageHasher {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_StorageHasher: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for StorageHasher {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    StorageHasher::Blake2_128 => {
                        dest.push_byte(0usize as u8);
                    }
                    StorageHasher::Blake2_256 => {
                        dest.push_byte(1usize as u8);
                    }
                    StorageHasher::Twox128 => {
                        dest.push_byte(2usize as u8);
                    }
                    StorageHasher::Twox256 => {
                        dest.push_byte(3usize as u8);
                    }
                    StorageHasher::Twox64Concat => {
                        dest.push_byte(4usize as u8);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_StorageHasher: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for StorageHasher {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(StorageHasher::Blake2_128)
                    }
                    x if x == 1usize as u8 => {
                        Some(StorageHasher::Blake2_256)
                    }
                    x if x == 2usize as u8 => { Some(StorageHasher::Twox128) }
                    x if x == 3usize as u8 => { Some(StorageHasher::Twox256) }
                    x if x == 4usize as u8 => {
                        Some(StorageHasher::Twox64Concat)
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for StorageHasher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&StorageHasher::Blake2_128,) => {
                let mut debug_trait_builder = f.debug_tuple("Blake2_128");
                debug_trait_builder.finish()
            }
            (&StorageHasher::Blake2_256,) => {
                let mut debug_trait_builder = f.debug_tuple("Blake2_256");
                debug_trait_builder.finish()
            }
            (&StorageHasher::Twox128,) => {
                let mut debug_trait_builder = f.debug_tuple("Twox128");
                debug_trait_builder.finish()
            }
            (&StorageHasher::Twox256,) => {
                let mut debug_trait_builder = f.debug_tuple("Twox256");
                debug_trait_builder.finish()
            }
            (&StorageHasher::Twox64Concat,) => {
                let mut debug_trait_builder = f.debug_tuple("Twox64Concat");
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_StorageHasher: () =
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
        impl _serde::Serialize for StorageHasher {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    StorageHasher::Blake2_128 =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "StorageHasher",
                                                               0u32,
                                                               "Blake2_128"),
                    StorageHasher::Blake2_256 =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "StorageHasher",
                                                               1u32,
                                                               "Blake2_256"),
                    StorageHasher::Twox128 =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "StorageHasher",
                                                               2u32,
                                                               "Twox128"),
                    StorageHasher::Twox256 =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "StorageHasher",
                                                               3u32,
                                                               "Twox256"),
                    StorageHasher::Twox64Concat =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "StorageHasher",
                                                               4u32,
                                                               "Twox64Concat"),
                }
            }
        }
    };

/// A storage function type.
#[structural_match]
pub enum StorageFunctionType {
    Plain(DecodeDifferentStr),
    Map {
        hasher: StorageHasher,
        key: DecodeDifferentStr,
        value: DecodeDifferentStr,
        is_linked: bool,
    },
    DoubleMap {
        hasher: StorageHasher,
        key1: DecodeDifferentStr,
        key2: DecodeDifferentStr,
        value: DecodeDifferentStr,
        key2_hasher: DecodeDifferentStr,
    },
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for StorageFunctionType {
    #[inline]
    fn clone(&self) -> StorageFunctionType {
        match (&*self,) {
            (&StorageFunctionType::Plain(ref __self_0),) =>
            StorageFunctionType::Plain(::std::clone::Clone::clone(&(*__self_0))),
            (&StorageFunctionType::Map {
             hasher: ref __self_0,
             key: ref __self_1,
             value: ref __self_2,
             is_linked: ref __self_3 },) =>
            StorageFunctionType::Map{hasher:
                                         ::std::clone::Clone::clone(&(*__self_0)),
                                     key:
                                         ::std::clone::Clone::clone(&(*__self_1)),
                                     value:
                                         ::std::clone::Clone::clone(&(*__self_2)),
                                     is_linked:
                                         ::std::clone::Clone::clone(&(*__self_3)),},
            (&StorageFunctionType::DoubleMap {
             hasher: ref __self_0,
             key1: ref __self_1,
             key2: ref __self_2,
             value: ref __self_3,
             key2_hasher: ref __self_4 },) =>
            StorageFunctionType::DoubleMap{hasher:
                                               ::std::clone::Clone::clone(&(*__self_0)),
                                           key1:
                                               ::std::clone::Clone::clone(&(*__self_1)),
                                           key2:
                                               ::std::clone::Clone::clone(&(*__self_2)),
                                           value:
                                               ::std::clone::Clone::clone(&(*__self_3)),
                                           key2_hasher:
                                               ::std::clone::Clone::clone(&(*__self_4)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for StorageFunctionType {
    #[inline]
    fn eq(&self, other: &StorageFunctionType) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&StorageFunctionType::Plain(ref __self_0),
                     &StorageFunctionType::Plain(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&StorageFunctionType::Map {
                     hasher: ref __self_0,
                     key: ref __self_1,
                     value: ref __self_2,
                     is_linked: ref __self_3 }, &StorageFunctionType::Map {
                     hasher: ref __arg_1_0,
                     key: ref __arg_1_1,
                     value: ref __arg_1_2,
                     is_linked: ref __arg_1_3 }) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2) &&
                        (*__self_3) == (*__arg_1_3),
                    (&StorageFunctionType::DoubleMap {
                     hasher: ref __self_0,
                     key1: ref __self_1,
                     key2: ref __self_2,
                     value: ref __self_3,
                     key2_hasher: ref __self_4 },
                     &StorageFunctionType::DoubleMap {
                     hasher: ref __arg_1_0,
                     key1: ref __arg_1_1,
                     key2: ref __arg_1_2,
                     value: ref __arg_1_3,
                     key2_hasher: ref __arg_1_4 }) =>
                    (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1)
                        && (*__self_2) == (*__arg_1_2) &&
                        (*__self_3) == (*__arg_1_3) &&
                        (*__self_4) == (*__arg_1_4),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &StorageFunctionType) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&StorageFunctionType::Plain(ref __self_0),
                     &StorageFunctionType::Plain(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&StorageFunctionType::Map {
                     hasher: ref __self_0,
                     key: ref __self_1,
                     value: ref __self_2,
                     is_linked: ref __self_3 }, &StorageFunctionType::Map {
                     hasher: ref __arg_1_0,
                     key: ref __arg_1_1,
                     value: ref __arg_1_2,
                     is_linked: ref __arg_1_3 }) =>
                    (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1)
                        || (*__self_2) != (*__arg_1_2) ||
                        (*__self_3) != (*__arg_1_3),
                    (&StorageFunctionType::DoubleMap {
                     hasher: ref __self_0,
                     key1: ref __self_1,
                     key2: ref __self_2,
                     value: ref __self_3,
                     key2_hasher: ref __self_4 },
                     &StorageFunctionType::DoubleMap {
                     hasher: ref __arg_1_0,
                     key1: ref __arg_1_1,
                     key2: ref __arg_1_2,
                     value: ref __arg_1_3,
                     key2_hasher: ref __arg_1_4 }) =>
                    (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1)
                        || (*__self_2) != (*__arg_1_2) ||
                        (*__self_3) != (*__arg_1_3) ||
                        (*__self_4) != (*__arg_1_4),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for StorageFunctionType {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _: ::std::cmp::AssertParamIsEq<StorageHasher>;
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _: ::std::cmp::AssertParamIsEq<bool>;
            let _: ::std::cmp::AssertParamIsEq<StorageHasher>;
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_StorageFunctionType: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for StorageFunctionType {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    StorageFunctionType::Plain(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    StorageFunctionType::Map {
                    ref hasher, ref key, ref value, ref is_linked } => {
                        dest.push_byte(1usize as u8);
                        dest.push(hasher);
                        dest.push(key);
                        dest.push(value);
                        dest.push(is_linked);
                    }
                    StorageFunctionType::DoubleMap {
                    ref hasher, ref key1, ref key2, ref value, ref key2_hasher
                    } => {
                        dest.push_byte(2usize as u8);
                        dest.push(hasher);
                        dest.push(key1);
                        dest.push(key2);
                        dest.push(value);
                        dest.push(key2_hasher);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_StorageFunctionType: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for StorageFunctionType {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(StorageFunctionType::Plain(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(StorageFunctionType::Map{hasher:
                                                          _parity_codec::Decode::decode(input)?,
                                                      key:
                                                          _parity_codec::Decode::decode(input)?,
                                                      value:
                                                          _parity_codec::Decode::decode(input)?,
                                                      is_linked:
                                                          _parity_codec::Decode::decode(input)?,})
                    }
                    x if x == 2usize as u8 => {
                        Some(StorageFunctionType::DoubleMap{hasher:
                                                                _parity_codec::Decode::decode(input)?,
                                                            key1:
                                                                _parity_codec::Decode::decode(input)?,
                                                            key2:
                                                                _parity_codec::Decode::decode(input)?,
                                                            value:
                                                                _parity_codec::Decode::decode(input)?,
                                                            key2_hasher:
                                                                _parity_codec::Decode::decode(input)?,})
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for StorageFunctionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&StorageFunctionType::Plain(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Plain");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&StorageFunctionType::Map {
             hasher: ref __self_0,
             key: ref __self_1,
             value: ref __self_2,
             is_linked: ref __self_3 },) => {
                let mut debug_trait_builder = f.debug_struct("Map");
                let _ = debug_trait_builder.field("hasher", &&(*__self_0));
                let _ = debug_trait_builder.field("key", &&(*__self_1));
                let _ = debug_trait_builder.field("value", &&(*__self_2));
                let _ = debug_trait_builder.field("is_linked", &&(*__self_3));
                debug_trait_builder.finish()
            }
            (&StorageFunctionType::DoubleMap {
             hasher: ref __self_0,
             key1: ref __self_1,
             key2: ref __self_2,
             value: ref __self_3,
             key2_hasher: ref __self_4 },) => {
                let mut debug_trait_builder = f.debug_struct("DoubleMap");
                let _ = debug_trait_builder.field("hasher", &&(*__self_0));
                let _ = debug_trait_builder.field("key1", &&(*__self_1));
                let _ = debug_trait_builder.field("key2", &&(*__self_2));
                let _ = debug_trait_builder.field("value", &&(*__self_3));
                let _ =
                    debug_trait_builder.field("key2_hasher", &&(*__self_4));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_StorageFunctionType: () =
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
        impl _serde::Serialize for StorageFunctionType {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    StorageFunctionType::Plain(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "StorageFunctionType",
                                                                  0u32,
                                                                  "Plain",
                                                                  __field0),
                    StorageFunctionType::Map {
                    ref hasher, ref key, ref value, ref is_linked } => {
                        let mut __serde_state =
                            match _serde::Serializer::serialize_struct_variant(__serializer,
                                                                               "StorageFunctionType",
                                                                               1u32,
                                                                               "Map",
                                                                               0
                                                                                   +
                                                                                   1
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
                        match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                   "hasher",
                                                                                   hasher)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                   "key",
                                                                                   key)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                   "value",
                                                                                   value)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                   "is_linked",
                                                                                   is_linked)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                    StorageFunctionType::DoubleMap {
                    ref hasher, ref key1, ref key2, ref value, ref key2_hasher
                    } => {
                        let mut __serde_state =
                            match _serde::Serializer::serialize_struct_variant(__serializer,
                                                                               "StorageFunctionType",
                                                                               2u32,
                                                                               "DoubleMap",
                                                                               0
                                                                                   +
                                                                                   1
                                                                                   +
                                                                                   1
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
                        match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                   "hasher",
                                                                                   hasher)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                   "key1",
                                                                                   key1)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                   "key2",
                                                                                   key2)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                   "value",
                                                                                   value)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStructVariant::serialize_field(&mut __serde_state,
                                                                                   "key2_hasher",
                                                                                   key2_hasher)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };

/// A storage function modifier.
#[structural_match]
pub enum StorageFunctionModifier { Optional, Default, }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for StorageFunctionModifier {
    #[inline]
    fn clone(&self) -> StorageFunctionModifier {
        match (&*self,) {
            (&StorageFunctionModifier::Optional,) =>
            StorageFunctionModifier::Optional,
            (&StorageFunctionModifier::Default,) =>
            StorageFunctionModifier::Default,
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for StorageFunctionModifier {
    #[inline]
    fn eq(&self, other: &StorageFunctionModifier) -> bool {
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
impl ::std::cmp::Eq for StorageFunctionModifier {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_StorageFunctionModifier: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for StorageFunctionModifier {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    StorageFunctionModifier::Optional => {
                        dest.push_byte(0usize as u8);
                    }
                    StorageFunctionModifier::Default => {
                        dest.push_byte(1usize as u8);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_StorageFunctionModifier: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for StorageFunctionModifier {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(StorageFunctionModifier::Optional)
                    }
                    x if x == 1usize as u8 => {
                        Some(StorageFunctionModifier::Default)
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for StorageFunctionModifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&StorageFunctionModifier::Optional,) => {
                let mut debug_trait_builder = f.debug_tuple("Optional");
                debug_trait_builder.finish()
            }
            (&StorageFunctionModifier::Default,) => {
                let mut debug_trait_builder = f.debug_tuple("Default");
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_StorageFunctionModifier: () =
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
        impl _serde::Serialize for StorageFunctionModifier {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    StorageFunctionModifier::Optional =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "StorageFunctionModifier",
                                                               0u32,
                                                               "Optional"),
                    StorageFunctionModifier::Default =>
                    _serde::Serializer::serialize_unit_variant(__serializer,
                                                               "StorageFunctionModifier",
                                                               1u32,
                                                               "Default"),
                }
            }
        }
    };

/// Metadata prefixed by a u32 for reserved usage
#[structural_match]
pub struct RuntimeMetadataPrefixed(pub u32, pub RuntimeMetadata);
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for RuntimeMetadataPrefixed {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<RuntimeMetadata>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RuntimeMetadataPrefixed: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for RuntimeMetadataPrefixed {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.0);
                dest.push(&self.1);
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for RuntimeMetadataPrefixed {
    #[inline]
    fn eq(&self, other: &RuntimeMetadataPrefixed) -> bool {
        match *other {
            RuntimeMetadataPrefixed(ref __self_1_0, ref __self_1_1) =>
            match *self {
                RuntimeMetadataPrefixed(ref __self_0_0, ref __self_0_1) =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &RuntimeMetadataPrefixed) -> bool {
        match *other {
            RuntimeMetadataPrefixed(ref __self_1_0, ref __self_1_1) =>
            match *self {
                RuntimeMetadataPrefixed(ref __self_0_0, ref __self_0_1) =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_RuntimeMetadataPrefixed: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for RuntimeMetadataPrefixed {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(RuntimeMetadataPrefixed(_parity_codec::Decode::decode(input)?,
                                             _parity_codec::Decode::decode(input)?))
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for RuntimeMetadataPrefixed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RuntimeMetadataPrefixed(ref __self_0_0, ref __self_0_1) => {
                let mut debug_trait_builder =
                    f.debug_tuple("RuntimeMetadataPrefixed");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                let _ = debug_trait_builder.field(&&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_RuntimeMetadataPrefixed: () =
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
        impl _serde::Serialize for RuntimeMetadataPrefixed {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_tuple_struct(__serializer,
                                                                     "RuntimeMetadataPrefixed",
                                                                     0 + 1 +
                                                                         1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state,
                                                                         &self.0)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state,
                                                                         &self.1)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                _serde::ser::SerializeTupleStruct::end(__serde_state)
            }
        }
    };

/// The metadata of a runtime.
/// The version ID encoded/decoded through
/// the enum nature of `RuntimeMetadata`.
#[structural_match]
pub enum RuntimeMetadata {

    /// Unused; enum filler.
    V0(RuntimeMetadataDeprecated),

    /// Version 1 for runtime metadata. No longer used.
    V1(RuntimeMetadataDeprecated),

    /// Version 2 for runtime metadata. No longer used.
    V2(RuntimeMetadataDeprecated),

    /// Version 3 for runtime metadata. No longer used.
    V3(RuntimeMetadataDeprecated),

    /// Version 4 for runtime metadata.
    V4(RuntimeMetadataV4),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for RuntimeMetadata {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<RuntimeMetadataDeprecated>;
            let _: ::std::cmp::AssertParamIsEq<RuntimeMetadataDeprecated>;
            let _: ::std::cmp::AssertParamIsEq<RuntimeMetadataDeprecated>;
            let _: ::std::cmp::AssertParamIsEq<RuntimeMetadataDeprecated>;
            let _: ::std::cmp::AssertParamIsEq<RuntimeMetadataV4>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RuntimeMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for RuntimeMetadata {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RuntimeMetadata::V0(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    RuntimeMetadata::V1(ref aa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                    }
                    RuntimeMetadata::V2(ref aa) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                    }
                    RuntimeMetadata::V3(ref aa) => {
                        dest.push_byte(3usize as u8);
                        dest.push(aa);
                    }
                    RuntimeMetadata::V4(ref aa) => {
                        dest.push_byte(4usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for RuntimeMetadata {
    #[inline]
    fn eq(&self, other: &RuntimeMetadata) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RuntimeMetadata::V0(ref __self_0),
                     &RuntimeMetadata::V0(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RuntimeMetadata::V1(ref __self_0),
                     &RuntimeMetadata::V1(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RuntimeMetadata::V2(ref __self_0),
                     &RuntimeMetadata::V2(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RuntimeMetadata::V3(ref __self_0),
                     &RuntimeMetadata::V3(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    (&RuntimeMetadata::V4(ref __self_0),
                     &RuntimeMetadata::V4(ref __arg_1_0)) =>
                    (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { false }
        }
    }
    #[inline]
    fn ne(&self, other: &RuntimeMetadata) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*other) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RuntimeMetadata::V0(ref __self_0),
                     &RuntimeMetadata::V0(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RuntimeMetadata::V1(ref __self_0),
                     &RuntimeMetadata::V1(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RuntimeMetadata::V2(ref __self_0),
                     &RuntimeMetadata::V2(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RuntimeMetadata::V3(ref __self_0),
                     &RuntimeMetadata::V3(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    (&RuntimeMetadata::V4(ref __self_0),
                     &RuntimeMetadata::V4(ref __arg_1_0)) =>
                    (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::std::intrinsics::unreachable() }
                }
            } else { true }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_RuntimeMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for RuntimeMetadata {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RuntimeMetadata::V0(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(RuntimeMetadata::V1(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 2usize as u8 => {
                        Some(RuntimeMetadata::V2(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 3usize as u8 => {
                        Some(RuntimeMetadata::V3(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 4usize as u8 => {
                        Some(RuntimeMetadata::V4(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for RuntimeMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RuntimeMetadata::V0(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("V0");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RuntimeMetadata::V1(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("V1");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RuntimeMetadata::V2(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("V2");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RuntimeMetadata::V3(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("V3");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&RuntimeMetadata::V4(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("V4");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_RuntimeMetadata: () =
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
        impl _serde::Serialize for RuntimeMetadata {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self {
                    RuntimeMetadata::V0(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "RuntimeMetadata",
                                                                  0u32, "V0",
                                                                  __field0),
                    RuntimeMetadata::V1(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "RuntimeMetadata",
                                                                  1u32, "V1",
                                                                  __field0),
                    RuntimeMetadata::V2(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "RuntimeMetadata",
                                                                  2u32, "V2",
                                                                  __field0),
                    RuntimeMetadata::V3(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "RuntimeMetadata",
                                                                  3u32, "V3",
                                                                  __field0),
                    RuntimeMetadata::V4(ref __field0) =>
                    _serde::Serializer::serialize_newtype_variant(__serializer,
                                                                  "RuntimeMetadata",
                                                                  4u32, "V4",
                                                                  __field0),
                }
            }
        }
    };

/// Enum that should fail.
#[structural_match]
pub enum RuntimeMetadataDeprecated { }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for RuntimeMetadataDeprecated {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for RuntimeMetadataDeprecated {
    #[inline]
    fn eq(&self, other: &RuntimeMetadataDeprecated) -> bool {
        unsafe { ::std::intrinsics::unreachable() }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for RuntimeMetadataDeprecated {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        unsafe { ::std::intrinsics::unreachable() }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_RuntimeMetadataDeprecated: () =
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
        impl _serde::Serialize for RuntimeMetadataDeprecated {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                match *self { }
            }
        }
    };

impl Encode for RuntimeMetadataDeprecated {
    fn encode_to<W: Output>(&self, _dest: &mut W) { }
}

#[cfg(feature = "std")]
impl Decode for RuntimeMetadataDeprecated {
    fn decode<I: Input>(_input: &mut I) -> Option<Self> {





        {
            ::std::rt::begin_panic("not yet implemented",
                                   &("srml/metadata/src/lib.rs", 331u32,
                                     3u32))
        }
    }
}
/// The metadata of a runtime.
#[structural_match]
pub struct RuntimeMetadataV4 {
    pub modules: DecodeDifferentArray<ModuleMetadata>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for RuntimeMetadataV4 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _:
                    ::std::cmp::AssertParamIsEq<DecodeDifferentArray<ModuleMetadata>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RuntimeMetadataV4: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for RuntimeMetadataV4 {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.modules);
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for RuntimeMetadataV4 {
    #[inline]
    fn eq(&self, other: &RuntimeMetadataV4) -> bool {
        match *other {
            RuntimeMetadataV4 { modules: ref __self_1_0 } =>
            match *self {
                RuntimeMetadataV4 { modules: ref __self_0_0 } =>
                (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &RuntimeMetadataV4) -> bool {
        match *other {
            RuntimeMetadataV4 { modules: ref __self_1_0 } =>
            match *self {
                RuntimeMetadataV4 { modules: ref __self_0_0 } =>
                (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_RuntimeMetadataV4: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for RuntimeMetadataV4 {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(RuntimeMetadataV4{modules:
                                           _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for RuntimeMetadataV4 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RuntimeMetadataV4 { modules: ref __self_0_0 } => {
                let mut debug_trait_builder =
                    f.debug_struct("RuntimeMetadataV4");
                let _ = debug_trait_builder.field("modules", &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_RuntimeMetadataV4: () =
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
        impl _serde::Serialize for RuntimeMetadataV4 {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "RuntimeMetadataV4",
                                                               false as usize
                                                                   + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "modules",
                                                                    &self.modules)
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
/// All metadata about an runtime module.
#[structural_match]
pub struct ModuleMetadata {
    pub name: DecodeDifferentStr,
    pub prefix: DecodeDifferent<FnEncode<&'static str>, StringBuf>,
    pub storage: ODFnA<StorageFunctionMetadata>,
    pub calls: ODFnA<FunctionMetadata>,
    pub event: ODFnA<EventMetadata>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for ModuleMetadata {
    #[inline]
    fn clone(&self) -> ModuleMetadata {
        match *self {
            ModuleMetadata {
            name: ref __self_0_0,
            prefix: ref __self_0_1,
            storage: ref __self_0_2,
            calls: ref __self_0_3,
            event: ref __self_0_4 } =>
            ModuleMetadata{name: ::std::clone::Clone::clone(&(*__self_0_0)),
                           prefix: ::std::clone::Clone::clone(&(*__self_0_1)),
                           storage:
                               ::std::clone::Clone::clone(&(*__self_0_2)),
                           calls: ::std::clone::Clone::clone(&(*__self_0_3)),
                           event:
                               ::std::clone::Clone::clone(&(*__self_0_4)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for ModuleMetadata {
    #[inline]
    fn eq(&self, other: &ModuleMetadata) -> bool {
        match *other {
            ModuleMetadata {
            name: ref __self_1_0,
            prefix: ref __self_1_1,
            storage: ref __self_1_2,
            calls: ref __self_1_3,
            event: ref __self_1_4 } =>
            match *self {
                ModuleMetadata {
                name: ref __self_0_0,
                prefix: ref __self_0_1,
                storage: ref __self_0_2,
                calls: ref __self_0_3,
                event: ref __self_0_4 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3) &&
                    (*__self_0_4) == (*__self_1_4),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &ModuleMetadata) -> bool {
        match *other {
            ModuleMetadata {
            name: ref __self_1_0,
            prefix: ref __self_1_1,
            storage: ref __self_1_2,
            calls: ref __self_1_3,
            event: ref __self_1_4 } =>
            match *self {
                ModuleMetadata {
                name: ref __self_0_0,
                prefix: ref __self_0_1,
                storage: ref __self_0_2,
                calls: ref __self_0_3,
                event: ref __self_0_4 } =>
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
impl ::std::cmp::Eq for ModuleMetadata {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<DecodeDifferentStr>;
            let _:
                    ::std::cmp::AssertParamIsEq<DecodeDifferent<FnEncode<&'static str>,
                                                                StringBuf>>;
            let _:
                    ::std::cmp::AssertParamIsEq<ODFnA<StorageFunctionMetadata>>;
            let _: ::std::cmp::AssertParamIsEq<ODFnA<FunctionMetadata>>;
            let _: ::std::cmp::AssertParamIsEq<ODFnA<EventMetadata>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_ModuleMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for ModuleMetadata {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.name);
                dest.push(&self.prefix);
                dest.push(&self.storage);
                dest.push(&self.calls);
                dest.push(&self.event);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_ModuleMetadata: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for ModuleMetadata {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(ModuleMetadata{name:
                                        _parity_codec::Decode::decode(input)?,
                                    prefix:
                                        _parity_codec::Decode::decode(input)?,
                                    storage:
                                        _parity_codec::Decode::decode(input)?,
                                    calls:
                                        _parity_codec::Decode::decode(input)?,
                                    event:
                                        _parity_codec::Decode::decode(input)?,})
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for ModuleMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ModuleMetadata {
            name: ref __self_0_0,
            prefix: ref __self_0_1,
            storage: ref __self_0_2,
            calls: ref __self_0_3,
            event: ref __self_0_4 } => {
                let mut debug_trait_builder =
                    f.debug_struct("ModuleMetadata");
                let _ = debug_trait_builder.field("name", &&(*__self_0_0));
                let _ = debug_trait_builder.field("prefix", &&(*__self_0_1));
                let _ = debug_trait_builder.field("storage", &&(*__self_0_2));
                let _ = debug_trait_builder.field("calls", &&(*__self_0_3));
                let _ = debug_trait_builder.field("event", &&(*__self_0_4));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_ModuleMetadata: () =
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
        impl _serde::Serialize for ModuleMetadata {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "ModuleMetadata",
                                                               false as usize
                                                                   + 1 + 1 + 1
                                                                   + 1 + 1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "name",
                                                                    &self.name)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "prefix",
                                                                    &self.prefix)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "storage",
                                                                    &self.storage)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "calls",
                                                                    &self.calls)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "event",
                                                                    &self.event)
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
type ODFnA<T> = Option<DecodeDifferent<FnEncode<&'static [T]>, Vec<T>>>;
impl Into<primitives::OpaqueMetadata> for RuntimeMetadataPrefixed {
    fn into(self) -> primitives::OpaqueMetadata {
        primitives::OpaqueMetadata::new(self.encode())
    }
}
impl Into<RuntimeMetadataPrefixed> for RuntimeMetadata {
    fn into(self) -> RuntimeMetadataPrefixed {
        RuntimeMetadataPrefixed(META_RESERVED, self)
    }
}
