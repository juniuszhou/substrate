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

//! Version module for the Substrate runtime; Provides a function that returns the runtime version.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


#[cfg(feature = "std")]
use serde::Serialize;
#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
use std::collections::HashSet;
#[cfg(feature = "std")]
use runtime_primitives::traits::RuntimeApiInfo;

use parity_codec::Encode;
#[cfg(feature = "std")]
use parity_codec::Decode;
use runtime_primitives::RuntimeString;
pub use runtime_primitives::create_runtime_str;

/// The identity of a particular API interface that the runtime might provide.
pub type ApiId = [u8; 8];

/// A vector of pairs of `ApiId` and a `u32` for version. For `"std"` builds, this
/// is a `Cow`.
#[cfg(feature = "std")]
pub type ApisVec = ::std::borrow::Cow<'static, [(ApiId, u32)]>;

/// Create a vector of Api declarations.
#[macro_export]
#[cfg(feature = "std")]
macro_rules! create_apis_vec(( $ y : expr ) => {
                             :: std :: borrow :: Cow :: Borrowed ( & $ y ) });

/// Runtime version.
/// This should not be thought of as classic Semver (major/minor/tiny).
/// This triplet have different semantics and mis-interpretation could cause problems.
/// In particular: bug fixes should result in an increment of `spec_version` and possibly `authoring_version`,
/// absolutely not `impl_version` since they change the semantics of the runtime.
#[serde(rename_all = "camelCase")]
#[structural_match]
pub struct RuntimeVersion {
    /// Identifies the different Substrate runtimes. There'll be at least polkadot and node.
    /// A different on-chain spec_name to that of the native runtime would normally result
    /// in node not attempting to sync or author blocks.
    pub spec_name: RuntimeString,

    /// Name of the implementation of the spec. This is of little consequence for the node
    /// and serves only to differentiate code of different implementation teams. For this
    /// codebase, it will be parity-polkadot. If there were a non-Rust implementation of the
    /// Polkadot runtime (e.g. C++), then it would identify itself with an accordingly different
    /// `impl_name`.
    pub impl_name: RuntimeString,

    /// `authoring_version` is the version of the authorship interface. An authoring node
    /// will not attempt to author blocks unless this is equal to its native runtime.
    pub authoring_version: u32,

    /// Version of the runtime specification. A full-node will not attempt to use its native
    /// runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    /// `spec_version` and `authoring_version` are the same between Wasm and native.
    pub spec_version: u32,

    /// Version of the implementation of the specification. Nodes are free to ignore this; it
    /// serves only as an indication that the code is different; as long as the other two versions
    /// are the same then while the actual code may be different, it is nonetheless required to
    /// do the same thing.
    /// Non-consensus-breaking optimizations are about the only changes that could be made which
    /// would result in only the `impl_version` changing.
    pub impl_version: u32,

    /// List of supported API "features" along with their versions.
    #[serde(serialize_with = "apis_serialize::serialize")]
    pub apis: ApisVec,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for RuntimeVersion {
    #[inline]
    fn clone(&self) -> RuntimeVersion {
        match *self {
            RuntimeVersion {
            spec_name: ref __self_0_0,
            impl_name: ref __self_0_1,
            authoring_version: ref __self_0_2,
            spec_version: ref __self_0_3,
            impl_version: ref __self_0_4,
            apis: ref __self_0_5 } =>
            RuntimeVersion{spec_name:
                               ::std::clone::Clone::clone(&(*__self_0_0)),
                           impl_name:
                               ::std::clone::Clone::clone(&(*__self_0_1)),
                           authoring_version:
                               ::std::clone::Clone::clone(&(*__self_0_2)),
                           spec_version:
                               ::std::clone::Clone::clone(&(*__self_0_3)),
                           impl_version:
                               ::std::clone::Clone::clone(&(*__self_0_4)),
                           apis: ::std::clone::Clone::clone(&(*__self_0_5)),},
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::PartialEq for RuntimeVersion {
    #[inline]
    fn eq(&self, other: &RuntimeVersion) -> bool {
        match *other {
            RuntimeVersion {
            spec_name: ref __self_1_0,
            impl_name: ref __self_1_1,
            authoring_version: ref __self_1_2,
            spec_version: ref __self_1_3,
            impl_version: ref __self_1_4,
            apis: ref __self_1_5 } =>
            match *self {
                RuntimeVersion {
                spec_name: ref __self_0_0,
                impl_name: ref __self_0_1,
                authoring_version: ref __self_0_2,
                spec_version: ref __self_0_3,
                impl_version: ref __self_0_4,
                apis: ref __self_0_5 } =>
                (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1) &&
                    (*__self_0_2) == (*__self_1_2) &&
                    (*__self_0_3) == (*__self_1_3) &&
                    (*__self_0_4) == (*__self_1_4) &&
                    (*__self_0_5) == (*__self_1_5),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &RuntimeVersion) -> bool {
        match *other {
            RuntimeVersion {
            spec_name: ref __self_1_0,
            impl_name: ref __self_1_1,
            authoring_version: ref __self_1_2,
            spec_version: ref __self_1_3,
            impl_version: ref __self_1_4,
            apis: ref __self_1_5 } =>
            match *self {
                RuntimeVersion {
                spec_name: ref __self_0_0,
                impl_name: ref __self_0_1,
                authoring_version: ref __self_0_2,
                spec_version: ref __self_0_3,
                impl_version: ref __self_0_4,
                apis: ref __self_0_5 } =>
                (*__self_0_0) != (*__self_1_0) ||
                    (*__self_0_1) != (*__self_1_1) ||
                    (*__self_0_2) != (*__self_1_2) ||
                    (*__self_0_3) != (*__self_1_3) ||
                    (*__self_0_4) != (*__self_1_4) ||
                    (*__self_0_5) != (*__self_1_5),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::cmp::Eq for RuntimeVersion {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::std::cmp::AssertParamIsEq<RuntimeString>;
            let _: ::std::cmp::AssertParamIsEq<RuntimeString>;
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<u32>;
            let _: ::std::cmp::AssertParamIsEq<ApisVec>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RuntimeVersion: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for RuntimeVersion {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.spec_name);
                dest.push(&self.impl_name);
                dest.push(&self.authoring_version);
                dest.push(&self.spec_version);
                dest.push(&self.impl_version);
                dest.push(&self.apis);
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for RuntimeVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RuntimeVersion {
            spec_name: ref __self_0_0,
            impl_name: ref __self_0_1,
            authoring_version: ref __self_0_2,
            spec_version: ref __self_0_3,
            impl_version: ref __self_0_4,
            apis: ref __self_0_5 } => {
                let mut debug_trait_builder =
                    f.debug_struct("RuntimeVersion");
                let _ =
                    debug_trait_builder.field("spec_name", &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("impl_name", &&(*__self_0_1));
                let _ =
                    debug_trait_builder.field("authoring_version",
                                              &&(*__self_0_2));
                let _ =
                    debug_trait_builder.field("spec_version",
                                              &&(*__self_0_3));
                let _ =
                    debug_trait_builder.field("impl_version",
                                              &&(*__self_0_4));
                let _ = debug_trait_builder.field("apis", &&(*__self_0_5));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_RuntimeVersion: () =
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
        impl _serde::Serialize for RuntimeVersion {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "RuntimeVersion",
                                                               false as usize
                                                                   + 1 + 1 + 1
                                                                   + 1 + 1 +
                                                                   1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "specName",
                                                                    &self.spec_name)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "implName",
                                                                    &self.impl_name)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "authoringVersion",
                                                                    &self.authoring_version)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "specVersion",
                                                                    &self.spec_version)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "implVersion",
                                                                    &self.impl_version)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "apis",
                                                                    {
                                                                        struct __SerializeWith<'__a> {
                                                                            values: (&'__a ApisVec,),
                                                                            phantom: _serde::export::PhantomData<RuntimeVersion>,
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
                                                                                apis_serialize::serialize(self.values.0,
                                                                                                          __s)
                                                                            }
                                                                        }
                                                                        &__SerializeWith{values:
                                                                                             (&self.apis,),
                                                                                         phantom:
                                                                                             _serde::export::PhantomData::<RuntimeVersion>,}
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
const _IMPL_DECODE_FOR_RuntimeVersion: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for RuntimeVersion {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(RuntimeVersion{spec_name:
                                        _parity_codec::Decode::decode(input)?,
                                    impl_name:
                                        _parity_codec::Decode::decode(input)?,
                                    authoring_version:
                                        _parity_codec::Decode::decode(input)?,
                                    spec_version:
                                        _parity_codec::Decode::decode(input)?,
                                    impl_version:
                                        _parity_codec::Decode::decode(input)?,
                                    apis:
                                        _parity_codec::Decode::decode(input)?,})
            }
        }
    };

#[cfg(feature = "std")]
impl fmt::Display for RuntimeVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {









        f.write_fmt(::std::fmt::Arguments::new_v1(&["", "-", ":", "(", "-",
                                                    ")"],
                                                  &match (&self.spec_name,
                                                          &self.spec_version,
                                                          &self.authoring_version,
                                                          &self.impl_name,
                                                          &self.impl_version)
                                                       {
                                                       (arg0, arg1, arg2,
                                                        arg3, arg4) =>
                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                    ::std::fmt::Display::fmt),
                                                        ::std::fmt::ArgumentV1::new(arg1,
                                                                                    ::std::fmt::Display::fmt),
                                                        ::std::fmt::ArgumentV1::new(arg2,
                                                                                    ::std::fmt::Display::fmt),
                                                        ::std::fmt::ArgumentV1::new(arg3,
                                                                                    ::std::fmt::Display::fmt),
                                                        ::std::fmt::ArgumentV1::new(arg4,
                                                                                    ::std::fmt::Display::fmt)],
                                                   }))
    }
}
#[cfg(feature = "std")]
impl RuntimeVersion {
    /// Check if this version matches other version for calling into runtime.
    pub fn can_call_with(&self, other: &RuntimeVersion) -> bool {
        self.spec_version == other.spec_version &&
            self.spec_name == other.spec_name &&
            self.authoring_version == other.authoring_version
    }
    /// Check if this version supports a particular API.
    pub fn has_api<A: RuntimeApiInfo + ?Sized>(&self) -> bool {
        self.apis.iter().any(|(s, v)| { s == &A::ID && *v == A::VERSION })
    }
    /// Check if the given api is implemented and the version passes a predicate.
    pub fn has_api_with<A: RuntimeApiInfo + ?Sized, P: Fn(u32) ->
                        bool>(&self, pred: P) -> bool {
        self.apis.iter().any(|(s, v)| { s == &A::ID && pred(*v) })
    }
}
#[cfg(feature = "std")]
pub struct NativeVersion {
    /// Basic runtime version info.
    pub runtime_version: RuntimeVersion,
    /// Authoring runtimes that this native runtime supports.
    pub can_author_with: HashSet<u32>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for NativeVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NativeVersion {
            runtime_version: ref __self_0_0, can_author_with: ref __self_0_1 }
            => {
                let mut debug_trait_builder = f.debug_struct("NativeVersion");
                let _ =
                    debug_trait_builder.field("runtime_version",
                                              &&(*__self_0_0));
                let _ =
                    debug_trait_builder.field("can_author_with",
                                              &&(*__self_0_1));
                debug_trait_builder.finish()
            }
        }
    }
}
#[cfg(feature = "std")]
impl NativeVersion {
    /// Check if this version matches other version for authoring blocks.
    pub fn can_author_with(&self, other: &RuntimeVersion) -> bool {
        self.runtime_version.spec_name == other.spec_name &&
            (self.runtime_version.authoring_version == other.authoring_version
                 || self.can_author_with.contains(&other.authoring_version))
    }
}
#[cfg(feature = "std")]
mod apis_serialize {
    use super::*;
    use impl_serde::serialize as bytes;
    use serde::{Serializer, ser::SerializeTuple};
    struct ApiId<'a>(
                     #[serde(serialize_with = "serialize_bytesref")]
                     &'a super::ApiId, &'a u32);
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_ApiId: () =
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
            impl <'a> _serde::Serialize for ApiId<'a> {
                fn serialize<__S>(&self, __serializer: __S)
                 -> _serde::export::Result<__S::Ok, __S::Error> where
                 __S: _serde::Serializer {
                    let mut __serde_state =
                        match _serde::Serializer::serialize_tuple_struct(__serializer,
                                                                         "ApiId",
                                                                         0 + 1
                                                                             +
                                                                             1)
                            {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                    match _serde::ser::SerializeTupleStruct::serialize_field(&mut __serde_state,
                                                                             {
                                                                                 struct __SerializeWith<'__a,
                                                                                                        'a: '__a> {
                                                                                     values: (&'__a &'a super::ApiId,),
                                                                                     phantom: _serde::export::PhantomData<ApiId<'a>>,
                                                                                 }
                                                                                 impl <'__a,
                                                                                       'a: '__a>
                                                                                  _serde::Serialize
                                                                                  for
                                                                                  __SerializeWith<'__a,
                                                                                                  'a>
                                                                                  {
                                                                                     fn serialize<__S>(&self,
                                                                                                       __s:
                                                                                                           __S)
                                                                                      ->
                                                                                          _serde::export::Result<__S::Ok,
                                                                                                                 __S::Error>
                                                                                      where
                                                                                      __S: _serde::Serializer {
                                                                                         serialize_bytesref(self.values.0,
                                                                                                            __s)
                                                                                     }
                                                                                 }
                                                                                 &__SerializeWith{values:
                                                                                                      (&self.0,),
                                                                                                  phantom:
                                                                                                      _serde::export::PhantomData::<ApiId<'a>>,}
                                                                             })
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
    pub fn serialize<S>(apis: &ApisVec, ser: S) -> Result<S::Ok, S::Error>
     where S: Serializer {
        let len = apis.len();
        let mut seq = ser.serialize_tuple(len)?;
        for (api, ver) in &**apis {
            seq.serialize_element(&ApiId(api, ver))?;
        }
        seq.end()
    }
    pub fn serialize_bytesref<S>(apis: &&super::ApiId, ser: S)
     -> Result<S::Ok, S::Error> where S: Serializer {
        bytes::serialize(*apis, ser)
    }
}
