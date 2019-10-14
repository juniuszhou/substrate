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

//! Temporary crate for contracts implementations.
//!
//! This will be replaced with WASM contracts stored on-chain.
//! ** NOTE ***
//! This is entirely deprecated with the idea of a single-module Wasm module for state transition.
//! The dispatch table should be replaced with the specific functions needed:
//! - execute_block(bytes)
//! - init_block(PrevBlock?) -> InProgressBlock
//! - add_transaction(InProgressBlock) -> InProgressBlock
//! It is left as is for now as it might be removed before this is ever done.

#![warn(missing_docs)]
#![recursion_limit = "128"]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

#[macro_use]
mod wasm_utils {



    //! Rust implementation of Substrate contracts.
    use wasmi::{ValueType, RuntimeValue, HostError};
    use wasmi::nan_preserving_float::{F32, F64};
    use std::fmt;
    pub struct UserError(pub &'static str);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for UserError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                UserError(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("UserError");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for UserError {
        #[inline]
        fn eq(&self, other: &UserError) -> bool {
            match *other {
                UserError(ref __self_1_0) =>
                match *self {
                    UserError(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UserError) -> bool {
            match *other {
                UserError(ref __self_1_0) =>
                match *self {
                    UserError(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl fmt::Display for UserError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(&["UserError: "],
                                                      &match (&self.0,) {
                                                           (arg0,) =>
                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                        ::std::fmt::Display::fmt)],
                                                       }))
        }
    }
    impl HostError for UserError { }
    pub trait ConvertibleToWasm {
        const
        VALUE_TYPE:
        ValueType;
        type
        NativeType;
        fn to_runtime_value(self)
        -> RuntimeValue;
    }
    impl ConvertibleToWasm for i32 {
        type
        NativeType
        =
        i32;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::I32;
        fn to_runtime_value(self) -> RuntimeValue { RuntimeValue::I32(self) }
    }
    impl ConvertibleToWasm for u32 {
        type
        NativeType
        =
        u32;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::I32;
        fn to_runtime_value(self) -> RuntimeValue {
            RuntimeValue::I32(self as i32)
        }
    }
    impl ConvertibleToWasm for i64 {
        type
        NativeType
        =
        i64;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::I64;
        fn to_runtime_value(self) -> RuntimeValue { RuntimeValue::I64(self) }
    }
    impl ConvertibleToWasm for u64 {
        type
        NativeType
        =
        u64;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::I64;
        fn to_runtime_value(self) -> RuntimeValue {
            RuntimeValue::I64(self as i64)
        }
    }
    impl ConvertibleToWasm for F32 {
        type
        NativeType
        =
        F32;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::F32;
        fn to_runtime_value(self) -> RuntimeValue { RuntimeValue::F32(self) }
    }
    impl ConvertibleToWasm for F64 {
        type
        NativeType
        =
        F64;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::F64;
        fn to_runtime_value(self) -> RuntimeValue { RuntimeValue::F64(self) }
    }
    impl ConvertibleToWasm for isize {
        type
        NativeType
        =
        i32;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::I32;
        fn to_runtime_value(self) -> RuntimeValue {
            RuntimeValue::I32(self as i32)
        }
    }
    impl ConvertibleToWasm for usize {
        type
        NativeType
        =
        u32;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::I32;
        fn to_runtime_value(self) -> RuntimeValue {
            RuntimeValue::I32(self as u32 as i32)
        }
    }
    impl <T> ConvertibleToWasm for *const T {
        type
        NativeType
        =
        u32;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::I32;
        fn to_runtime_value(self) -> RuntimeValue {
            RuntimeValue::I32(self as isize as i32)
        }
    }
    impl <T> ConvertibleToWasm for *mut T {
        type
        NativeType
        =
        u32;
        const
        VALUE_TYPE:
        ValueType
        =
        ValueType::I32;
        fn to_runtime_value(self) -> RuntimeValue {
            RuntimeValue::I32(self as isize as i32)
        }
    }
    /// Converts arguments into respective WASM types.
    #[macro_export]
    macro_rules! convert_args((  ) => ( [  ] ) ; ( $ ( $ t : ty ) , * ) => (
                              [
                              $ (
                              {
                              use $ crate :: wasm_utils :: ConvertibleToWasm ;
                              < $ t > :: VALUE_TYPE } , ) * ] ) ;);
    /// Generates a WASM signature for given list of parameters.
    #[macro_export]
    macro_rules! gen_signature(( ( $ ( $ params : ty ) , * ) ) => (
                               {
                               $ crate :: wasmi :: Signature :: new (
                               & convert_args ! ( $ ( $ params ) , * ) [ .. ]
                               , None ) } ) ; (
                               ( $ ( $ params : ty ) , * ) -> $ returns : ty )
                               => (
                               {
                               $ crate :: wasmi :: Signature :: new (
                               & convert_args ! ( $ ( $ params ) , * ) [ .. ]
                               , Some (
                               {
                               use $ crate :: wasm_utils :: ConvertibleToWasm
                               ; < $ returns > :: VALUE_TYPE } ) ) } ) ;);
    macro_rules! resolve_fn((
                            @ iter $ index : expr , $ sig_var : ident , $
                            name_var : ident ) => (  ) ; (
                            @ iter $ index : expr , $ sig_var : ident , $
                            name_var : ident $ name : ident (
                            $ ( $ params : ty ) , * ) $ ( -> $ returns : ty )
                            * => $ ( $ tail : tt ) * ) => (
                            if $ name_var == stringify ! ( $ name ) {
                            let signature = gen_signature ! (
                            ( $ ( $ params ) , * ) $ ( -> $ returns ) * ) ; if
                            $ sig_var != & signature {
                            return Err (
                            $ crate :: wasmi :: Error :: Instantiation (
                            format ! (
                            "Export {} has different signature {:?}" , $
                            name_var , $ sig_var ) , ) ) ; } return Ok (
                            $ crate :: wasmi :: FuncInstance :: alloc_host (
                            signature , $ index ) ) ; } resolve_fn ! (
                            @ iter $ index + 1 , $ sig_var , $ name_var $ (
                            $ tail ) * ) ) ; (
                            $ sig_var : ident , $ name_var : ident , $ (
                            $ tail : tt ) * ) => (
                            resolve_fn ! (
                            @ iter 0 , $ sig_var , $ name_var $ ( $ tail ) * )
                            ; ) ;);
    /// Converts the list of arguments coming from WASM into their native types.
    #[macro_export]
    macro_rules! unmarshall_args((
                                 $ body : tt , $ objectname : ident , $
                                 args_iter : ident , $ (
                                 $ names : ident : $ params : ty ) , * ) => (
                                 {
                                 $ (
                                 let $ names : < $ params as $ crate ::
                                 wasm_utils :: ConvertibleToWasm > ::
                                 NativeType = $ args_iter . next (  ) .
                                 and_then ( | rt_val | rt_val . try_into (  )
                                 ) . expect (
                                 "`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						"
                                 ) ; ) * $ body } ));
    /// Since we can't specify the type of closure directly at binding site:
    ///
    /// ```nocompile
    /// let f: FnOnce() -> Result<<u32 as ConvertibleToWasm>::NativeType, _> = || { /* ... */ };
    /// ```
    ///
    /// we use this function to constrain the type of the closure.
    #[inline(always)]
    pub fn constrain_closure<R, F>(f: F) -> F where F: FnOnce() ->
     Result<R, ::wasmi::Trap> {
        f
    }
    /// Pass the list of parameters by converting them to respective WASM types.
    #[macro_export]
    macro_rules! marshall((
                          $ args_iter : ident , $ objectname : ident , (
                          $ ( $ names : ident : $ params : ty ) , * ) -> $
                          returns : ty => $ body : tt ) => (
                          {
                          let body = $ crate :: wasm_utils ::
                          constrain_closure :: < < $ returns as $ crate ::
                          wasm_utils :: ConvertibleToWasm > :: NativeType , _
                          > (
                          || {
                          unmarshall_args ! (
                          $ body , $ objectname , $ args_iter , $ (
                          $ names : $ params ) , * ) } ) ; let r = body (  ) ?
                          ; return Ok (
                          Some (
                          {
                          use $ crate :: wasm_utils :: ConvertibleToWasm ; r .
                          to_runtime_value (  ) } ) ) } ) ; (
                          $ args_iter : ident , $ objectname : ident , (
                          $ ( $ names : ident : $ params : ty ) , * ) => $
                          body : tt ) => (
                          {
                          let body = $ crate :: wasm_utils ::
                          constrain_closure :: < (  ) , _ > (
                          || {
                          unmarshall_args ! (
                          $ body , $ objectname , $ args_iter , $ (
                          $ names : $ params ) , * ) } ) ; body (  ) ? ;
                          return Ok ( None ) } ));
    macro_rules! dispatch_fn((
                             @ iter $ index : expr , $ index_ident : ident , $
                             objectname : ident , $ args_iter : ident ) => {
                             panic ! (
                             "fn with index {} is undefined" , $ index ) ; } ;
                             (
                             @ iter $ index : expr , $ index_ident : ident , $
                             objectname : ident , $ args_iter : ident , $ name
                             : ident (
                             $ ( $ names : ident : $ params : ty ) , * ) $ (
                             -> $ returns : ty ) * => $ body : tt $ (
                             $ tail : tt ) * ) => (
                             if $ index_ident == $ index {
                             {
                             marshall ! (
                             $ args_iter , $ objectname , (
                             $ ( $ names : $ params ) , * ) $ ( -> $ returns )
                             * => $ body ) } } dispatch_fn ! (
                             @ iter $ index + 1 , $ index_ident , $ objectname
                             , $ args_iter $ ( $ tail ) * ) ) ; (
                             $ index_ident : ident , $ objectname : ident , $
                             args_iter : ident , $ ( $ tail : tt ) * ) => (
                             dispatch_fn ! (
                             @ iter 0 , $ index_ident , $ objectname , $
                             args_iter , $ ( $ tail ) * ) ; ) ;);
    /// Implements `wasmi::Externals` trait and `Resolver` for given struct.
    #[macro_export]
    macro_rules! impl_function_executor((
                                        $ objectname : ident : $ structname :
                                        ty , $ (
                                        $ name : ident (
                                        $ ( $ names : ident : $ params : ty )
                                        , * ) $ ( -> $ returns : ty ) * => $
                                        body : tt , ) * => $ ( $ pre : tt ) +
                                        ) => (
                                        impl $ ( $ pre ) + $ structname {
                                        # [ allow ( unused ) ] fn resolver (
                                        ) -> & 'static $ crate :: wasmi ::
                                        ModuleImportResolver {
                                        struct Resolver ; impl $ crate ::
                                        wasmi :: ModuleImportResolver for
                                        Resolver {
                                        fn resolve_func (
                                        & self , name : & str , signature : &
                                        $ crate :: wasmi :: Signature ) -> ::
                                        std :: result :: Result < $ crate ::
                                        wasmi :: FuncRef , $ crate :: wasmi ::
                                        Error > {
                                        resolve_fn ! (
                                        signature , name , $ (
                                        $ name ( $ ( $ params ) , * ) $ (
                                        -> $ returns ) * => ) * ) ; Err (
                                        $ crate :: wasmi :: Error ::
                                        Instantiation (
                                        format ! (
                                        "Export {} not found" , name ) , ) ) }
                                        } & Resolver } } impl $ ( $ pre ) + $
                                        crate :: wasmi :: Externals for $
                                        structname {
                                        fn invoke_index (
                                        & mut self , index : usize , args : $
                                        crate :: wasmi :: RuntimeArgs , ) ->
                                        :: std :: result :: Result < Option <
                                        $ crate :: wasmi :: RuntimeValue > , $
                                        crate :: wasmi :: Trap > {
                                        let $ objectname = self ; let mut args
                                        = args . as_ref (  ) . iter (  ) ;
                                        dispatch_fn ! (
                                        index , $ objectname , args , $ (
                                        $ name ( $ ( $ names : $ params ) , *
                                        ) $ ( -> $ returns ) * => $ body ) , *
                                        ) ; } } ) ;);
}
mod wasm_executor {
    //! Rust implementation of Substrate contracts.
    use std::collections::HashMap;
    use tiny_keccak;
    use secp256k1;
    use wasmi::{Module, ModuleInstance, MemoryInstance, MemoryRef, TableRef,
                ImportsBuilder, ModuleRef};
    use wasmi::RuntimeValue::{I32, I64, self};
    use wasmi::memory_units::{Pages};
    use state_machine::{Externalities, ChildStorageKey};
    use crate::error::{Error, ErrorKind, Result};
    use crate::wasm_utils::UserError;
    use primitives::{blake2_128, blake2_256, twox_64, twox_128, twox_256,
                     ed25519, sr25519, Pair};
    use primitives::hexdisplay::HexDisplay;
    use primitives::sandbox as sandbox_primitives;
    use primitives::{H256, Blake2Hasher};
    use trie::ordered_trie_root;
    use crate::sandbox;
    use crate::allocator;
    use log::trace;
    #[cfg(not(feature = "wasm-extern-trace"))]
    macro_rules! debug_trace(( $ ( $ x : tt ) * ) => (  ));
    struct FunctionExecutor<'e, E: Externalities<Blake2Hasher> + 'e> {
        sandbox_store: sandbox::Store,
        heap: allocator::FreeingBumpHeapAllocator,
        memory: MemoryRef,
        table: Option<TableRef>,
        ext: &'e mut E,
        hash_lookup: HashMap<Vec<u8>, Vec<u8>>,
    }
    impl <'e, E: Externalities<Blake2Hasher>> FunctionExecutor<'e, E> {
        fn new(m: MemoryRef, t: Option<TableRef>, e: &'e mut E)
         -> Result<Self> {
            Ok(FunctionExecutor{sandbox_store: sandbox::Store::new(),
                                heap:
                                    allocator::FreeingBumpHeapAllocator::new(m.clone()),
                                memory: m,
                                table: t,
                                ext: e,
                                hash_lookup: HashMap::new(),})
        }
    }
    impl <'e, E: Externalities<Blake2Hasher>> sandbox::SandboxCapabilities for
     FunctionExecutor<'e, E> {
        fn store(&self) -> &sandbox::Store { &self.sandbox_store }
        fn store_mut(&mut self) -> &mut sandbox::Store {
            &mut self.sandbox_store
        }
        fn allocate(&mut self, len: u32)
         -> ::std::result::Result<u32, UserError> {
            self.heap.allocate(len)
        }
        fn deallocate(&mut self, ptr: u32)
         -> ::std::result::Result<(), UserError> {
            self.heap.deallocate(ptr)
        }
        fn write_memory(&mut self, ptr: u32, data: &[u8])
         -> ::std::result::Result<(), UserError> {
            self.memory.set(ptr,
                            data).map_err(|_|
                                              UserError("Invalid attempt to write_memory"))
        }
        fn read_memory(&self, ptr: u32, len: u32)
         -> ::std::result::Result<Vec<u8>, UserError> {
            self.memory.get(ptr,
                            len as
                                usize).map_err(|_|
                                                   UserError("Invalid attempt to write_memory"))
        }
    }
    trait WritePrimitive<T: Sized> {
        fn write_primitive(&self, offset: u32, t: T)
        -> ::std::result::Result<(), UserError>;
    }
    impl WritePrimitive<u32> for MemoryInstance {
        fn write_primitive(&self, offset: u32, t: u32)
         -> ::std::result::Result<(), UserError> {
            use byteorder::{LittleEndian, ByteOrder};
            let mut r = [0u8; 4];
            LittleEndian::write_u32(&mut r, t);
            self.set(offset,
                     &r).map_err(|_|
                                     UserError("Invalid attempt to write_primitive"))
        }
    }
    trait ReadPrimitive<T: Sized> {
        fn read_primitive(&self, offset: u32)
        -> ::std::result::Result<T, UserError>;
    }
    impl ReadPrimitive<u32> for MemoryInstance {
        fn read_primitive(&self, offset: u32)
         -> ::std::result::Result<u32, UserError> {
            use byteorder::{LittleEndian, ByteOrder};
            Ok(LittleEndian::read_u32(&self.get(offset,
                                                4).map_err(|_|
                                                               UserError("Invalid attempt to read_primitive"))?))
        }
    }
    impl <'e, E: Externalities<Blake2Hasher> + 'e> FunctionExecutor<'e, E> {
        #[allow(unused)]
        fn resolver() -> &'static crate::wasmi::ModuleImportResolver {
            struct Resolver;
            impl crate::wasmi::ModuleImportResolver for Resolver {
                fn resolve_func(&self, name: &str,
                                signature: &crate::wasmi::Signature)
                 ->
                     ::std::result::Result<crate::wasmi::FuncRef,
                                           crate::wasmi::Error> {
                    if name == "ext_print_utf8" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0));
                    }
                    if name == "ext_print_hex" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 +
                                                                             1));
                    }
                    if name == "ext_print_num" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u64>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_malloc" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <usize>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <*mut u8>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_free" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_set_storage" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_set_child_storage" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_clear_child_storage" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_clear_storage" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_exists_storage" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_exists_child_storage" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_clear_prefix" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_kill_child_storage" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_get_allocated_storage" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <*mut u8>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_get_allocated_child_storage" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <*mut u8>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_get_storage_into" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_get_child_storage_into" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_storage_root" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_child_storage_root" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <*mut u8>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_storage_changes_root" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u64>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_blake2_256_enumerated_trie_root" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_chain_id" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u64>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_twox_64" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_twox_128" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_twox_256" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_blake2_128" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_blake2_256" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_keccak_256" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_ed25519_verify" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_sr25519_verify" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_secp256k1_ecdsa_recover" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_submit_extrinsic" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_sandbox_instantiate" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <usize>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <usize>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <usize>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <usize>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_sandbox_instance_teardown" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_sandbox_invoke" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <usize>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <usize>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <usize>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <usize>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_sandbox_memory_new" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_sandbox_memory_get" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*mut u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_sandbox_memory_set" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <*const u8>::VALUE_TYPE
                                                               },
                                                               {
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..],
                                                             Some({
                                                                      use crate::wasm_utils::ConvertibleToWasm;
                                                                      <u32>::VALUE_TYPE
                                                                  }))
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    }
                    if name == "ext_sandbox_memory_teardown" {
                        let signature =
                            {
                                crate::wasmi::Signature::new(&[{
                                                                   use crate::wasm_utils::ConvertibleToWasm;
                                                                   <u32>::VALUE_TYPE
                                                               }][..], None)
                            };
                        if signature != &signature {
                            return Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                               " has different signature "],
                                                                                                                             &match (&name,
                                                                                                                                     &signature)
                                                                                                                                  {
                                                                                                                                  (arg0,
                                                                                                                                   arg1)
                                                                                                                                  =>
                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                              }))));
                        }
                        return Ok(crate::wasmi::FuncInstance::alloc_host(signature,
                                                                         0 + 1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1
                                                                             +
                                                                             1));
                    };
                    Err(crate::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                " not found"],
                                                                                                              &match (&name,)
                                                                                                                   {
                                                                                                                   (arg0,)
                                                                                                                   =>
                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                ::std::fmt::Display::fmt)],
                                                                                                               }))))
                }
            }
            &Resolver
        }
    }
    impl <'e, E: Externalities<Blake2Hasher> + 'e> crate::wasmi::Externals for
     FunctionExecutor<'e, E> {
        fn invoke_index(&mut self, index: usize,
                        args: crate::wasmi::RuntimeArgs)
         ->
             ::std::result::Result<Option<crate::wasmi::RuntimeValue>,
                                   crate::wasmi::Trap> {
            let this = self;
            let mut args = args.as_ref().iter();
            if index == 0 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let utf8_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let utf8_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      if let Ok(utf8)
                                                                                             =
                                                                                             this.memory.get(utf8_data,
                                                                                                             utf8_len
                                                                                                                 as
                                                                                                                 usize)
                                                                                             {
                                                                                          if let Ok(message)
                                                                                                 =
                                                                                                 String::from_utf8(utf8)
                                                                                                 {
                                                                                              {
                                                                                                  ::std::io::_print(::std::fmt::Arguments::new_v1(&["",
                                                                                                                                                    "\n"],
                                                                                                                                                  &match (&message,)
                                                                                                                                                       {
                                                                                                                                                       (arg0,)
                                                                                                                                                       =>
                                                                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                    ::std::fmt::Display::fmt)],
                                                                                                                                                   }));
                                                                                              };
                                                                                          }
                                                                                      }
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      if let Ok(hex)
                                                                                             =
                                                                                             this.memory.get(data,
                                                                                                             len
                                                                                                                 as
                                                                                                                 usize)
                                                                                             {
                                                                                          {
                                                                                              ::std::io::_print(::std::fmt::Arguments::new_v1(&["",
                                                                                                                                                "\n"],
                                                                                                                                              &match (&HexDisplay::from(&hex),)
                                                                                                                                                   {
                                                                                                                                                   (arg0,)
                                                                                                                                                   =>
                                                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                ::std::fmt::Display::fmt)],
                                                                                                                                               }));
                                                                                          };
                                                                                      }
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let number:
                                                                                          <u64
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      {
                                                                                          ::std::io::_print(::std::fmt::Arguments::new_v1(&["",
                                                                                                                                            "\n"],
                                                                                                                                          &match (&number,)
                                                                                                                                               {
                                                                                                                                               (arg0,)
                                                                                                                                               =>
                                                                                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                            ::std::fmt::Display::fmt)],
                                                                                                                                           }));
                                                                                      };
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<*mut u8 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let size:
                                                                                          <usize
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let r =
                                                                                          this.heap.allocate(size)?;
                                                                                      Ok(r)
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let addr:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      this.heap.deallocate(addr)?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine key in ext_set_storage"))?;
                                                                                      let value =
                                                                                          this.memory.get(value_data,
                                                                                                          value_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine value in ext_set_storage"))?;
                                                                                      if let Some(_preimage)
                                                                                             =
                                                                                             this.hash_lookup.get(&key)
                                                                                             {
                                                                                      } else {
                                                                                      }
                                                                                      this.ext.set_storage(key,
                                                                                                           value);
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let storage_key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let storage_key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let storage_key =
                                                                                          this.memory.get(storage_key_data,
                                                                                                          storage_key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine storage_key in ext_set_child_storage"))?;
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine key in ext_set_child_storage"))?;
                                                                                      let value =
                                                                                          this.memory.get(value_data,
                                                                                                          value_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine value in ext_set_child_storage"))?;
                                                                                      if let Some(_preimage)
                                                                                             =
                                                                                             this.hash_lookup.get(&key)
                                                                                             {
                                                                                      } else {
                                                                                      }
                                                                                      let storage_key =
                                                                                          ChildStorageKey::from_vec(storage_key).ok_or_else(||
                                                                                                                                                UserError("ext_set_child_storage: child storage key is invalid"))?;
                                                                                      this.ext.set_child_storage(storage_key,
                                                                                                                 key,
                                                                                                                 value);
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let storage_key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let storage_key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let storage_key =
                                                                                          this.memory.get(storage_key_data,
                                                                                                          storage_key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine storage_key in ext_clear_child_storage"))?;
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine key in ext_clear_child_storage"))?;
                                                                                      let storage_key =
                                                                                          ChildStorageKey::from_vec(storage_key).ok_or_else(||
                                                                                                                                                UserError("ext_clear_child_storage: child storage key is not valid"))?;
                                                                                      this.ext.clear_child_storage(storage_key,
                                                                                                                   &key);
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine key in ext_clear_storage"))?;
                                                                                      this.ext.clear_storage(&key);
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine key in ext_exists_storage"))?;
                                                                                      Ok(if this.ext.exists_storage(&key)
                                                                                            {
                                                                                             1
                                                                                         } else {
                                                                                             0
                                                                                         })
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let storage_key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let storage_key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let storage_key =
                                                                                          this.memory.get(storage_key_data,
                                                                                                          storage_key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine storage_key in ext_exists_child_storage"))?;
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine key in ext_exists_child_storage"))?;
                                                                                      let storage_key =
                                                                                          ChildStorageKey::from_vec(storage_key).ok_or_else(||
                                                                                                                                                UserError("ext_exists_child_storage: child storage key is not valid"))?;
                                                                                      Ok(if this.ext.exists_child_storage(storage_key,
                                                                                                                          &key)
                                                                                            {
                                                                                             1
                                                                                         } else {
                                                                                             0
                                                                                         })
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let prefix_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let prefix_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let prefix =
                                                                                          this.memory.get(prefix_data,
                                                                                                          prefix_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine prefix in ext_clear_prefix"))?;
                                                                                      this.ext.clear_prefix(&prefix);
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let storage_key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let storage_key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let storage_key =
                                                                                          this.memory.get(storage_key_data,
                                                                                                          storage_key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine storage_key in ext_kill_child_storage"))?;
                                                                                      let storage_key =
                                                                                          ChildStorageKey::from_vec(storage_key).ok_or_else(||
                                                                                                                                                UserError("ext_exists_child_storage: child storage key is not valid"))?;
                                                                                      this.ext.kill_child_storage(storage_key);
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index == 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
               {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<*mut u8 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let written_out:
                                                                                          <*mut u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine key in ext_get_allocated_storage"))?;
                                                                                      let maybe_value =
                                                                                          this.ext.storage(&key);
                                                                                      if let Some(value)
                                                                                             =
                                                                                             maybe_value
                                                                                             {
                                                                                          let offset =
                                                                                              this.heap.allocate(value.len()
                                                                                                                     as
                                                                                                                     u32)?
                                                                                                  as
                                                                                                  u32;
                                                                                          this.memory.set(offset,
                                                                                                          &value).map_err(|_|
                                                                                                                              UserError("Invalid attempt to set memory in ext_get_allocated_storage"))?;
                                                                                          this.memory.write_primitive(written_out,
                                                                                                                      value.len()
                                                                                                                          as
                                                                                                                          u32).map_err(|_|
                                                                                                                                           UserError("Invalid attempt to write written_out in ext_get_allocated_storage"))?;
                                                                                          Ok(offset)
                                                                                      } else {
                                                                                          this.memory.write_primitive(written_out,
                                                                                                                      u32::max_value()).map_err(|_|
                                                                                                                                                    UserError("Invalid attempt to write failed written_out in ext_get_allocated_storage"))?;
                                                                                          Ok(0)
                                                                                      }
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<*mut u8 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let storage_key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let storage_key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let written_out:
                                                                                          <*mut u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let storage_key =
                                                                                          this.memory.get(storage_key_data,
                                                                                                          storage_key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine storage_key in ext_get_allocated_child_storage"))?;
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine key in ext_get_allocated_child_storage"))?;
                                                                                      let maybe_value =
                                                                                          {
                                                                                              let storage_key =
                                                                                                  ChildStorageKey::from_slice(&storage_key).ok_or_else(||
                                                                                                                                                           UserError("ext_get_allocated_child_storage: child storage key is not valid"))?;
                                                                                              this.ext.child_storage(storage_key,
                                                                                                                     &key)
                                                                                          };
                                                                                      if let Some(value)
                                                                                             =
                                                                                             maybe_value
                                                                                             {
                                                                                          let offset =
                                                                                              this.heap.allocate(value.len()
                                                                                                                     as
                                                                                                                     u32)?
                                                                                                  as
                                                                                                  u32;
                                                                                          this.memory.set(offset,
                                                                                                          &value).map_err(|_|
                                                                                                                              UserError("Invalid attempt to set memory in ext_get_allocated_child_storage"))?;
                                                                                          this.memory.write_primitive(written_out,
                                                                                                                      value.len()
                                                                                                                          as
                                                                                                                          u32).map_err(|_|
                                                                                                                                           UserError("Invalid attempt to write written_out in ext_get_allocated_child_storage"))?;
                                                                                          Ok(offset)
                                                                                      } else {
                                                                                          this.memory.write_primitive(written_out,
                                                                                                                      u32::max_value()).map_err(|_|
                                                                                                                                                    UserError("Invalid attempt to write failed written_out in ext_get_allocated_child_storage"))?;
                                                                                          Ok(0)
                                                                                      }
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_data:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_offset:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to get key in ext_get_storage_into"))?;
                                                                                      let maybe_value =
                                                                                          this.ext.storage(&key);
                                                                                      if let Some(value)
                                                                                             =
                                                                                             maybe_value
                                                                                             {
                                                                                          let value =
                                                                                              &value[value_offset
                                                                                                         as
                                                                                                         usize..];
                                                                                          let written =
                                                                                              ::std::cmp::min(value_len
                                                                                                                  as
                                                                                                                  usize,
                                                                                                              value.len());
                                                                                          this.memory.set(value_data,
                                                                                                          &value[..written]).map_err(|_|
                                                                                                                                         UserError("Invalid attempt to set value in ext_get_storage_into"))?;
                                                                                          Ok(written
                                                                                                 as
                                                                                                 u32)
                                                                                      } else {
                                                                                          Ok(u32::max_value())
                                                                                      }
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let storage_key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let storage_key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_data:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let value_offset:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let storage_key =
                                                                                          this.memory.get(storage_key_data,
                                                                                                          storage_key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine storage_key in ext_get_child_storage_into"))?;
                                                                                      let key =
                                                                                          this.memory.get(key_data,
                                                                                                          key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to get key in ext_get_child_storage_into"))?;
                                                                                      let maybe_value =
                                                                                          {
                                                                                              let storage_key =
                                                                                                  ChildStorageKey::from_slice(&*storage_key).ok_or_else(||
                                                                                                                                                            UserError("ext_get_child_storage_into: child storage key is not valid"))?;
                                                                                              this.ext.child_storage(storage_key,
                                                                                                                     &key)
                                                                                          };
                                                                                      if let Some(value)
                                                                                             =
                                                                                             maybe_value
                                                                                             {
                                                                                          let value =
                                                                                              &value[value_offset
                                                                                                         as
                                                                                                         usize..];
                                                                                          let written =
                                                                                              ::std::cmp::min(value_len
                                                                                                                  as
                                                                                                                  usize,
                                                                                                              value.len());
                                                                                          this.memory.set(value_data,
                                                                                                          &value[..written]).map_err(|_|
                                                                                                                                         UserError("Invalid attempt to set value in ext_get_child_storage_into"))?;
                                                                                          Ok(written
                                                                                                 as
                                                                                                 u32)
                                                                                      } else {
                                                                                          Ok(u32::max_value())
                                                                                      }
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let result:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let r =
                                                                                          this.ext.storage_root();
                                                                                      this.memory.set(result,
                                                                                                      r.as_ref()).map_err(|_|
                                                                                                                              UserError("Invalid attempt to set memory in ext_storage_root"))?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<*mut u8 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let storage_key_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let storage_key_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let written_out:
                                                                                          <*mut u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let storage_key =
                                                                                          this.memory.get(storage_key_data,
                                                                                                          storage_key_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to determine storage_key in ext_child_storage_root"))?;
                                                                                      let storage_key =
                                                                                          ChildStorageKey::from_slice(&*storage_key).ok_or_else(||
                                                                                                                                                    UserError("ext_child_storage_root: child storage key is not valid"))?;
                                                                                      let value =
                                                                                          this.ext.child_storage_root(storage_key);
                                                                                      let offset =
                                                                                          this.heap.allocate(value.len()
                                                                                                                 as
                                                                                                                 u32)?
                                                                                              as
                                                                                              u32;
                                                                                      this.memory.set(offset,
                                                                                                      &value).map_err(|_|
                                                                                                                          UserError("Invalid attempt to set memory in ext_child_storage_root"))?;
                                                                                      this.memory.write_primitive(written_out,
                                                                                                                  value.len()
                                                                                                                      as
                                                                                                                      u32).map_err(|_|
                                                                                                                                       UserError("Invalid attempt to write written_out in ext_child_storage_root"))?;
                                                                                      Ok(offset)
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let parent_hash_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let parent_hash_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let parent_number:
                                                                                          <u64
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let result:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let mut parent_hash =
                                                                                          H256::default();
                                                                                      if parent_hash_len
                                                                                             !=
                                                                                             parent_hash.as_ref().len()
                                                                                                 as
                                                                                                 u32
                                                                                         {
                                                                                          return Err(UserError("Invalid parent_hash_len in ext_storage_changes_root").into());
                                                                                      }
                                                                                      let raw_parent_hash =
                                                                                          this.memory.get(parent_hash_data,
                                                                                                          parent_hash_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to get parent_hash in ext_storage_changes_root"))?;
                                                                                      parent_hash.as_mut().copy_from_slice(&raw_parent_hash[..]);
                                                                                      let r =
                                                                                          this.ext.storage_changes_root(parent_hash,
                                                                                                                        parent_number);
                                                                                      if let Some(r)
                                                                                             =
                                                                                             r
                                                                                             {
                                                                                          this.memory.set(result,
                                                                                                          &r[..]).map_err(|_|
                                                                                                                              UserError("Invalid attempt to set memory in ext_storage_changes_root"))?;
                                                                                          Ok(1)
                                                                                      } else {
                                                                                          Ok(0)
                                                                                      }
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let values_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let lens_data:
                                                                                          <*const u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let lens_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let result:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let values =
                                                                                          (0..lens_len).map(|i|
                                                                                                                this.memory.read_primitive(lens_data
                                                                                                                                               +
                                                                                                                                               i
                                                                                                                                                   *
                                                                                                                                                   4)).collect::<::std::result::Result<Vec<u32>,
                                                                                                                                                                                       UserError>>()?.into_iter().scan(0u32,
                                                                                                                                                                                                                       |acc,
                                                                                                                                                                                                                        v|
                                                                                                                                                                                                                           {
                                                                                                                                                                                                                               let o =
                                                                                                                                                                                                                                   *acc;
                                                                                                                                                                                                                               *acc
                                                                                                                                                                                                                                   +=
                                                                                                                                                                                                                                   v;
                                                                                                                                                                                                                               Some((o,
                                                                                                                                                                                                                                     v))
                                                                                                                                                                                                                           }).map(|(offset,
                                                                                                                                                                                                                                    len)|
                                                                                                                                                                                                                                      this.memory.get(values_data
                                                                                                                                                                                                                                                          +
                                                                                                                                                                                                                                                          offset,
                                                                                                                                                                                                                                                      len
                                                                                                                                                                                                                                                          as
                                                                                                                                                                                                                                                          usize).map_err(|_|
                                                                                                                                                                                                                                                                             UserError("Invalid attempt to get memory in ext_blake2_256_enumerated_trie_root"))).collect::<::std::result::Result<Vec<_>,
                                                                                                                                                                                                                                                                                                                                                                                                 UserError>>()?;
                                                                                      let r =
                                                                                          ordered_trie_root::<Blake2Hasher,
                                                                                                              _,
                                                                                                              _>(values.into_iter());
                                                                                      this.memory.set(result,
                                                                                                      &r[..]).map_err(|_|
                                                                                                                          UserError("Invalid attempt to set memory in ext_blake2_256_enumerated_trie_root"))?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u64 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  {
                                                                                      Ok(this.ext.chain_id())
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let out:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let result:
                                                                                              [u8; 8] =
                                                                                          if len
                                                                                                 ==
                                                                                                 0
                                                                                             {
                                                                                              let hashed =
                                                                                                  twox_64(&[0u8;
                                                                                                               0]);
                                                                                              this.hash_lookup.insert(hashed.to_vec(),
                                                                                                                      <[_]>::into_vec(box
                                                                                                                                          []));
                                                                                              hashed
                                                                                          } else {
                                                                                              let key =
                                                                                                  this.memory.get(data,
                                                                                                                  len
                                                                                                                      as
                                                                                                                      usize).map_err(|_|
                                                                                                                                         UserError("Invalid attempt to get key in ext_twox_64"))?;
                                                                                              let hashed_key =
                                                                                                  twox_64(&key);
                                                                                              this.hash_lookup.insert(hashed_key.to_vec(),
                                                                                                                      key);
                                                                                              hashed_key
                                                                                          };
                                                                                      this.memory.set(out,
                                                                                                      &result).map_err(|_|
                                                                                                                           UserError("Invalid attempt to set result in ext_twox_64"))?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let out:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let result:
                                                                                              [u8; 16] =
                                                                                          if len
                                                                                                 ==
                                                                                                 0
                                                                                             {
                                                                                              let hashed =
                                                                                                  twox_128(&[0u8;
                                                                                                                0]);
                                                                                              this.hash_lookup.insert(hashed.to_vec(),
                                                                                                                      <[_]>::into_vec(box
                                                                                                                                          []));
                                                                                              hashed
                                                                                          } else {
                                                                                              let key =
                                                                                                  this.memory.get(data,
                                                                                                                  len
                                                                                                                      as
                                                                                                                      usize).map_err(|_|
                                                                                                                                         UserError("Invalid attempt to get key in ext_twox_128"))?;
                                                                                              let hashed_key =
                                                                                                  twox_128(&key);
                                                                                              this.hash_lookup.insert(hashed_key.to_vec(),
                                                                                                                      key);
                                                                                              hashed_key
                                                                                          };
                                                                                      this.memory.set(out,
                                                                                                      &result).map_err(|_|
                                                                                                                           UserError("Invalid attempt to set result in ext_twox_128"))?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let out:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let result:
                                                                                              [u8; 32] =
                                                                                          if len
                                                                                                 ==
                                                                                                 0
                                                                                             {
                                                                                              twox_256(&[0u8;
                                                                                                            0])
                                                                                          } else {
                                                                                              twox_256(&this.memory.get(data,
                                                                                                                        len
                                                                                                                            as
                                                                                                                            usize).map_err(|_|
                                                                                                                                               UserError("Invalid attempt to get data in ext_twox_256"))?)
                                                                                          };
                                                                                      this.memory.set(out,
                                                                                                      &result).map_err(|_|
                                                                                                                           UserError("Invalid attempt to set result in ext_twox_256"))?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let out:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let result:
                                                                                              [u8; 16] =
                                                                                          if len
                                                                                                 ==
                                                                                                 0
                                                                                             {
                                                                                              let hashed =
                                                                                                  blake2_128(&[0u8;
                                                                                                                  0]);
                                                                                              this.hash_lookup.insert(hashed.to_vec(),
                                                                                                                      <[_]>::into_vec(box
                                                                                                                                          []));
                                                                                              hashed
                                                                                          } else {
                                                                                              let key =
                                                                                                  this.memory.get(data,
                                                                                                                  len
                                                                                                                      as
                                                                                                                      usize).map_err(|_|
                                                                                                                                         UserError("Invalid attempt to get key in ext_blake2_128"))?;
                                                                                              let hashed_key =
                                                                                                  blake2_128(&key);
                                                                                              this.hash_lookup.insert(hashed_key.to_vec(),
                                                                                                                      key);
                                                                                              hashed_key
                                                                                          };
                                                                                      this.memory.set(out,
                                                                                                      &result).map_err(|_|
                                                                                                                           UserError("Invalid attempt to set result in ext_blake2_128"))?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let out:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let result:
                                                                                              [u8; 32] =
                                                                                          if len
                                                                                                 ==
                                                                                                 0
                                                                                             {
                                                                                              blake2_256(&[0u8;
                                                                                                              0])
                                                                                          } else {
                                                                                              blake2_256(&this.memory.get(data,
                                                                                                                          len
                                                                                                                              as
                                                                                                                              usize).map_err(|_|
                                                                                                                                                 UserError("Invalid attempt to get data in ext_blake2_256"))?)
                                                                                          };
                                                                                      this.memory.set(out,
                                                                                                      &result).map_err(|_|
                                                                                                                           UserError("Invalid attempt to set result in ext_blake2_256"))?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let out:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let result:
                                                                                              [u8; 32] =
                                                                                          if len
                                                                                                 ==
                                                                                                 0
                                                                                             {
                                                                                              tiny_keccak::keccak256(&[0u8;
                                                                                                                          0])
                                                                                          } else {
                                                                                              tiny_keccak::keccak256(&this.memory.get(data,
                                                                                                                                      len
                                                                                                                                          as
                                                                                                                                          usize).map_err(|_|
                                                                                                                                                             UserError("Invalid attempt to get data in ext_keccak_256"))?)
                                                                                          };
                                                                                      this.memory.set(out,
                                                                                                      &result).map_err(|_|
                                                                                                                           UserError("Invalid attempt to set result in ext_keccak_256"))?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let msg_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let msg_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let sig_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let pubkey_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let mut sig =
                                                                                          [0u8;
                                                                                              64];
                                                                                      this.memory.get_into(sig_data,
                                                                                                           &mut sig[..]).map_err(|_|
                                                                                                                                     UserError("Invalid attempt to get signature in ext_ed25519_verify"))?;
                                                                                      let mut pubkey =
                                                                                          [0u8;
                                                                                              32];
                                                                                      this.memory.get_into(pubkey_data,
                                                                                                           &mut pubkey[..]).map_err(|_|
                                                                                                                                        UserError("Invalid attempt to get pubkey in ext_ed25519_verify"))?;
                                                                                      let msg =
                                                                                          this.memory.get(msg_data,
                                                                                                          msg_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to get message in ext_ed25519_verify"))?;
                                                                                      Ok(if ed25519::Pair::verify_weak(&sig,
                                                                                                                       &msg,
                                                                                                                       &pubkey)
                                                                                            {
                                                                                             0
                                                                                         } else {
                                                                                             5
                                                                                         })
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let msg_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let msg_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let sig_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let pubkey_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let mut sig =
                                                                                          [0u8;
                                                                                              64];
                                                                                      this.memory.get_into(sig_data,
                                                                                                           &mut sig[..]).map_err(|_|
                                                                                                                                     UserError("Invalid attempt to get signature in ext_sr25519_verify"))?;
                                                                                      let mut pubkey =
                                                                                          [0u8;
                                                                                              32];
                                                                                      this.memory.get_into(pubkey_data,
                                                                                                           &mut pubkey[..]).map_err(|_|
                                                                                                                                        UserError("Invalid attempt to get pubkey in ext_sr25519_verify"))?;
                                                                                      let msg =
                                                                                          this.memory.get(msg_data,
                                                                                                          msg_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("Invalid attempt to get message in ext_sr25519_verify"))?;
                                                                                      Ok(if sr25519::Pair::verify_weak(&sig,
                                                                                                                       &msg,
                                                                                                                       &pubkey)
                                                                                            {
                                                                                             0
                                                                                         } else {
                                                                                             5
                                                                                         })
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let msg_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let sig_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let pubkey_data:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let mut sig =
                                                                                          [0u8;
                                                                                              65];
                                                                                      this.memory.get_into(sig_data,
                                                                                                           &mut sig[..]).map_err(|_|
                                                                                                                                     UserError("Invalid attempt to get signature in ext_secp256k1_ecdsa_recover"))?;
                                                                                      let rs =
                                                                                          match secp256k1::Signature::parse_slice(&sig[0..64])
                                                                                              {
                                                                                              Ok(rs)
                                                                                              =>
                                                                                              rs,
                                                                                              _
                                                                                              =>
                                                                                              return Ok(1),
                                                                                          };
                                                                                      let v =
                                                                                          match secp256k1::RecoveryId::parse(if sig[64]
                                                                                                                                    >
                                                                                                                                    26
                                                                                                                                {
                                                                                                                                 sig[64]
                                                                                                                                     -
                                                                                                                                     27
                                                                                                                             } else {
                                                                                                                                 sig[64]
                                                                                                                             }
                                                                                                                                 as
                                                                                                                                 u8)
                                                                                              {
                                                                                              Ok(v)
                                                                                              =>
                                                                                              v,
                                                                                              _
                                                                                              =>
                                                                                              return Ok(2),
                                                                                          };
                                                                                      let mut msg =
                                                                                          [0u8;
                                                                                              32];
                                                                                      this.memory.get_into(msg_data,
                                                                                                           &mut msg[..]).map_err(|_|
                                                                                                                                     UserError("Invalid attempt to get message in ext_secp256k1_ecdsa_recover"))?;
                                                                                      let pubkey =
                                                                                          match secp256k1::recover(&secp256k1::Message::parse(&msg),
                                                                                                                   &rs,
                                                                                                                   &v)
                                                                                              {
                                                                                              Ok(pk)
                                                                                              =>
                                                                                              pk,
                                                                                              _
                                                                                              =>
                                                                                              return Ok(3),
                                                                                          };
                                                                                      this.memory.set(pubkey_data,
                                                                                                      &pubkey.serialize()[1..65]).map_err(|_|
                                                                                                                                              UserError("Invalid attempt to set pubkey in ext_secp256k1_ecdsa_recover"))?;
                                                                                      Ok(0)
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let msg_data:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let extrinsic =
                                                                                          this.memory.get(msg_data,
                                                                                                          len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("OOB while ext_submit_extrinsic: wasm"))?;
                                                                                      this.ext.submit_extrinsic(extrinsic).map_err(|_|
                                                                                                                                       UserError("Calling unavailable API ext_submit_extrinsic: wasm"))?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let dispatch_thunk_idx:
                                                                                          <usize
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let wasm_ptr:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let wasm_len:
                                                                                          <usize
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let imports_ptr:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let imports_len:
                                                                                          <usize
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let state:
                                                                                          <usize
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let wasm =
                                                                                          this.memory.get(wasm_ptr,
                                                                                                          wasm_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("OOB while ext_sandbox_instantiate: wasm"))?;
                                                                                      let raw_env_def =
                                                                                          this.memory.get(imports_ptr,
                                                                                                          imports_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("OOB while ext_sandbox_instantiate: imports"))?;
                                                                                      let dispatch_thunk =
                                                                                          {
                                                                                              let table =
                                                                                                  this.table.as_ref().ok_or_else(||
                                                                                                                                     UserError("Runtime doesn't have a table; sandbox is unavailable"))?;
                                                                                              table.get(dispatch_thunk_idx).map_err(|_|
                                                                                                                                        UserError("dispatch_thunk_idx is out of the table bounds"))?.ok_or_else(||
                                                                                                                                                                                                                    UserError("dispatch_thunk_idx points on an empty table entry"))?.clone()
                                                                                          };
                                                                                      let instance_idx_or_err_code =
                                                                                          match sandbox::instantiate(this,
                                                                                                                     dispatch_thunk,
                                                                                                                     &wasm,
                                                                                                                     &raw_env_def,
                                                                                                                     state)
                                                                                              {
                                                                                              Ok(instance_idx)
                                                                                              =>
                                                                                              instance_idx,
                                                                                              Err(sandbox::InstantiationError::StartTrapped)
                                                                                              =>
                                                                                              sandbox_primitives::ERR_EXECUTION,
                                                                                              Err(_)
                                                                                              =>
                                                                                              sandbox_primitives::ERR_MODULE,
                                                                                          };
                                                                                      Ok(instance_idx_or_err_code
                                                                                             as
                                                                                             u32)
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let instance_idx:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      this.sandbox_store.instance_teardown(instance_idx)?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let instance_idx:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let export_ptr:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let export_len:
                                                                                          <usize
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let args_ptr:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let args_len:
                                                                                          <usize
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let return_val_ptr:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let return_val_len:
                                                                                          <usize
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let state:
                                                                                          <usize
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      use parity_codec::{Decode,
                                                                                                         Encode};
                                                                                      {
                                                                                          let lvl =
                                                                                              ::log::Level::Trace;
                                                                                          if lvl
                                                                                                 <=
                                                                                                 ::log::STATIC_MAX_LEVEL
                                                                                                 &&
                                                                                                 lvl
                                                                                                     <=
                                                                                                     ::log::max_level()
                                                                                             {
                                                                                              ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["invoke, instance_idx="],
                                                                                                                                                     &match (&instance_idx,)
                                                                                                                                                          {
                                                                                                                                                          (arg0,)
                                                                                                                                                          =>
                                                                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       ::std::fmt::Display::fmt)],
                                                                                                                                                      }),
                                                                                                                       lvl,
                                                                                                                       &("sr-sandbox",
                                                                                                                         "substrate_executor::wasm_executor",
                                                                                                                         "core/executor/src/wasm_executor.rs",
                                                                                                                         640u32));
                                                                                          }
                                                                                      };
                                                                                      let export =
                                                                                          this.memory.get(export_ptr,
                                                                                                          export_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("OOB while ext_sandbox_invoke: export")).and_then(|b|
                                                                                                                                                                                                 String::from_utf8(b).map_err(|_|
                                                                                                                                                                                                                                  UserError("export name should be a valid utf-8 sequence")))?;
                                                                                      let serialized_args =
                                                                                          this.memory.get(args_ptr,
                                                                                                          args_len
                                                                                                              as
                                                                                                              usize).map_err(|_|
                                                                                                                                 UserError("OOB while ext_sandbox_invoke: args"))?;
                                                                                      let args =
                                                                                          Vec::<sandbox_primitives::TypedValue>::decode(&mut &serialized_args[..]).ok_or_else(||
                                                                                                                                                                                  UserError("Can't decode serialized arguments for the invocation"))?.into_iter().map(Into::into).collect::<Vec<_>>();
                                                                                      let instance =
                                                                                          this.sandbox_store.instance(instance_idx)?;
                                                                                      let result =
                                                                                          instance.invoke(&export,
                                                                                                          &args,
                                                                                                          this,
                                                                                                          state);
                                                                                      match result
                                                                                          {
                                                                                          Ok(None)
                                                                                          =>
                                                                                          Ok(sandbox_primitives::ERR_OK),
                                                                                          Ok(Some(val))
                                                                                          =>
                                                                                          {
                                                                                              sandbox_primitives::ReturnValue::Value(val.into()).using_encoded(|val|
                                                                                                                                                                   {
                                                                                                                                                                       if val.len()
                                                                                                                                                                              >
                                                                                                                                                                              return_val_len
                                                                                                                                                                                  as
                                                                                                                                                                                  usize
                                                                                                                                                                          {
                                                                                                                                                                           Err(UserError("Return value buffer is too small"))?;
                                                                                                                                                                       }
                                                                                                                                                                       this.memory.set(return_val_ptr,
                                                                                                                                                                                       val).map_err(|_|
                                                                                                                                                                                                        UserError("Return value buffer is OOB"))?;
                                                                                                                                                                       Ok(sandbox_primitives::ERR_OK)
                                                                                                                                                                   })
                                                                                          }
                                                                                          Err(_)
                                                                                          =>
                                                                                          Ok(sandbox_primitives::ERR_EXECUTION),
                                                                                      }
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let initial:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let maximum:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let mem_idx =
                                                                                          this.sandbox_store.new_memory(initial,
                                                                                                                        maximum)?;
                                                                                      Ok(mem_idx)
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let memory_idx:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let offset:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let buf_ptr:
                                                                                          <*mut u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let buf_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let sandboxed_memory =
                                                                                          this.sandbox_store.memory(memory_idx)?;
                                                                                      match MemoryInstance::transfer(&sandboxed_memory,
                                                                                                                     offset
                                                                                                                         as
                                                                                                                         usize,
                                                                                                                     &this.memory,
                                                                                                                     buf_ptr
                                                                                                                         as
                                                                                                                         usize,
                                                                                                                     buf_len
                                                                                                                         as
                                                                                                                         usize)
                                                                                          {
                                                                                          Ok(())
                                                                                          =>
                                                                                          Ok(sandbox_primitives::ERR_OK),
                                                                                          Err(_)
                                                                                          =>
                                                                                          Ok(sandbox_primitives::ERR_OUT_OF_BOUNDS),
                                                                                      }
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<<u32 as
                                                                   crate::wasm_utils::ConvertibleToWasm>::NativeType,
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let memory_idx:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let offset:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let val_ptr:
                                                                                          <*const u8
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  let val_len:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      let sandboxed_memory =
                                                                                          this.sandbox_store.memory(memory_idx)?;
                                                                                      match MemoryInstance::transfer(&this.memory,
                                                                                                                     val_ptr
                                                                                                                         as
                                                                                                                         usize,
                                                                                                                     &sandboxed_memory,
                                                                                                                     offset
                                                                                                                         as
                                                                                                                         usize,
                                                                                                                     val_len
                                                                                                                         as
                                                                                                                         usize)
                                                                                          {
                                                                                          Ok(())
                                                                                          =>
                                                                                          Ok(sandbox_primitives::ERR_OK),
                                                                                          Err(_)
                                                                                          =>
                                                                                          Ok(sandbox_primitives::ERR_OUT_OF_BOUNDS),
                                                                                      }
                                                                                  }
                                                                              }
                                                                          });
                        let r = body()?;
                        return Ok(Some({
                                           use crate::wasm_utils::ConvertibleToWasm;
                                           r.to_runtime_value()
                                       }))
                    }
                }
            }
            if index ==
                   0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 +
                       1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 {
                {
                    {
                        let body =
                            crate::wasm_utils::constrain_closure::<(),
                                                                   _>(||
                                                                          {
                                                                              {
                                                                                  let memory_idx:
                                                                                          <u32
                                                                                          as
                                                                                          crate::wasm_utils::ConvertibleToWasm>::NativeType =
                                                                                      args.next().and_then(|rt_val|
                                                                                                               rt_val.try_into()).expect("`$args_iter` comes from an argument of Externals::invoke_index;
						args to an external call always matches the signature of the external;
						external signatures are built with count and types and in order defined by `$params`;
						here, we iterating on `$params`;
						qed;
						");
                                                                                  {
                                                                                      this.sandbox_store.memory_teardown(memory_idx)?;
                                                                                      Ok(())
                                                                                  }
                                                                              }
                                                                          });
                        body()?;
                        return Ok(None)
                    }
                }
            }
            {
                ::std::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1(&["fn with index ",
                                                                            " is undefined"],
                                                                          &match (&(0
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1
                                                                                        +
                                                                                        1),)
                                                                               {
                                                                               (arg0,)
                                                                               =>
                                                                               [::std::fmt::ArgumentV1::new(arg0,
                                                                                                            ::std::fmt::Display::fmt)],
                                                                           }),
                                           &("core/executor/src/wasm_executor.rs",
                                             116u32, 1u32))
            };
        }
    }
    /// Wasm rust executor for contracts.
    ///
    /// Executes the provided code in a sandboxed wasm runtime.
    pub struct WasmExecutor;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for WasmExecutor {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                WasmExecutor => {
                    let mut debug_trait_builder =
                        f.debug_tuple("WasmExecutor");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for WasmExecutor {
        #[inline]
        fn clone(&self) -> WasmExecutor {
            match *self { WasmExecutor => WasmExecutor, }
        }
    }
    impl WasmExecutor {
        /// Create a new instance.
        pub fn new() -> Self { WasmExecutor }
        /// Call a given method in the given code.
        ///
        /// Signature of this method needs to be `(I32, I32) -> I64`.
        ///
        /// This should be used for tests only.
        pub fn call<E: Externalities<Blake2Hasher>>(&self, ext: &mut E,
                                                    heap_pages: usize,
                                                    code: &[u8], method: &str,
                                                    data: &[u8])
         -> Result<Vec<u8>> {
            let module = ::wasmi::Module::from_buffer(code)?;
            let module = self.prepare_module(ext, heap_pages, &module)?;
            self.call_in_wasm_module(ext, &module, method, data)
        }
        /// Call a given method with a custom signature in the given code.
        ///
        /// This should be used for tests only.
        pub fn call_with_custom_signature<E: Externalities<Blake2Hasher>,
                                          F: FnOnce(&mut FnMut(&[u8]) ->
                                                         Result<u32>) ->
                                          Result<Vec<RuntimeValue>>,
                                          FR: FnOnce(Option<RuntimeValue>,
                                                     &MemoryRef) ->
                                          Result<Option<R>>,
                                          R>(&self, ext: &mut E,
                                             heap_pages: usize, code: &[u8],
                                             method: &str,
                                             create_parameters: F,
                                             filter_result: FR) -> Result<R> {
            let module = wasmi::Module::from_buffer(code)?;
            let module = self.prepare_module(ext, heap_pages, &module)?;
            self.call_in_wasm_module_with_custom_signature(ext, &module,
                                                           method,
                                                           create_parameters,
                                                           filter_result)
        }
        fn get_mem_instance(module: &ModuleRef) -> Result<MemoryRef> {
            Ok(module.export_by_name("memory").ok_or_else(||
                                                              Error::from(ErrorKind::InvalidMemoryReference))?.as_memory().ok_or_else(||
                                                                                                                                          Error::from(ErrorKind::InvalidMemoryReference))?.clone())
        }
        /// Call a given method in the given wasm-module runtime.
        pub fn call_in_wasm_module<E: Externalities<Blake2Hasher>>(&self,
                                                                   ext:
                                                                       &mut E,
                                                                   module_instance:
                                                                       &ModuleRef,
                                                                   method:
                                                                       &str,
                                                                   data:
                                                                       &[u8])
         -> Result<Vec<u8>> {
            self.call_in_wasm_module_with_custom_signature(ext,
                                                           module_instance,
                                                           method,
                                                           |alloc|
                                                               {
                                                                   let offset =
                                                                       alloc(data)?;
                                                                   Ok(<[_]>::into_vec(box
                                                                                          [I32(offset
                                                                                                   as
                                                                                                   i32),
                                                                                           I32(data.len()
                                                                                                   as
                                                                                                   i32)]))
                                                               },
                                                           |res, memory|
                                                               {
                                                                   if let Some(I64(r))
                                                                          =
                                                                          res
                                                                          {
                                                                       let offset =
                                                                           r
                                                                               as
                                                                               u32;
                                                                       let length =
                                                                           (r
                                                                                as
                                                                                u64
                                                                                >>
                                                                                32)
                                                                               as
                                                                               usize;
                                                                       memory.get(offset,
                                                                                  length).map_err(|_|
                                                                                                      ErrorKind::Runtime.into()).map(Some)
                                                                   } else {
                                                                       Ok(None)
                                                                   }
                                                               })
        }
        /// Call a given method in the given wasm-module runtime.
        fn call_in_wasm_module_with_custom_signature<E: Externalities<Blake2Hasher>,
                                                     F: FnOnce(&mut FnMut(&[u8])
                                                                    ->
                                                                    Result<u32>)
                                                     ->
                                                     Result<Vec<RuntimeValue>>,
                                                     FR: FnOnce(Option<RuntimeValue>,
                                                                &MemoryRef) ->
                                                     Result<Option<R>>,
                                                     R>(&self, ext: &mut E,
                                                        module_instance:
                                                            &ModuleRef,
                                                        method: &str,
                                                        create_parameters: F,
                                                        filter_result: FR)
         -> Result<R> {
            let memory = Self::get_mem_instance(module_instance)?;
            let table: Option<TableRef> =
                module_instance.export_by_name("__indirect_function_table").and_then(|e|
                                                                                         e.as_table().cloned());
            let low = memory.lowest_used();
            let used_mem = memory.used_size();
            let mut fec = FunctionExecutor::new(memory.clone(), table, ext)?;
            let parameters =
                create_parameters(&mut (|data: &[u8]|
                                            {
                                                let offset =
                                                    fec.heap.allocate(data.len()
                                                                          as
                                                                          u32).map_err(|_|
                                                                                           ErrorKind::Runtime)?;
                                                memory.set(offset, &data)?;
                                                Ok(offset)
                                            }))?;
            let result =
                module_instance.invoke_export(method, &parameters, &mut fec);
            let result =
                match result {
                    Ok(val) =>
                    match filter_result(val, &memory)? {
                        Some(val) => Ok(val),
                        None => Err(ErrorKind::InvalidReturn.into()),
                    },
                    Err(e) => {
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL &&
                                   lvl <= ::log::max_level() {
                                ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Failed to execute code with ",
                                                                                         " pages"],
                                                                                       &match (&memory.current_size().0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        }),
                                                         lvl,
                                                         &("wasm-executor",
                                                           "substrate_executor::wasm_executor",
                                                           "core/executor/src/wasm_executor.rs",
                                                           847u32));
                            }
                        };
                        Err(e.into())
                    }
                };
            let new_low = memory.lowest_used();
            if new_low < low {
                memory.zero(new_low as usize, (low - new_low) as usize)?;
                memory.reset_lowest_used(low);
            }
            memory.with_direct_access_mut(|buf| buf.resize(used_mem.0, 0));
            result
        }
        /// Prepare module instance
        pub fn prepare_module<E: Externalities<Blake2Hasher>>(&self,
                                                              ext: &mut E,
                                                              heap_pages:
                                                                  usize,
                                                              module: &Module)
         -> Result<ModuleRef> {
            let intermediate_instance =
                ModuleInstance::new(module,
                                    &ImportsBuilder::new().with_resolver("env",
                                                                         FunctionExecutor::<E>::resolver()))?;
            let memory =
                Self::get_mem_instance(intermediate_instance.not_started_instance())?;
            memory.grow(Pages(heap_pages)).map_err(|_|
                                                       Error::from(ErrorKind::Runtime))?;
            let table: Option<TableRef> =
                intermediate_instance.not_started_instance().export_by_name("__indirect_function_table").and_then(|e|
                                                                                                                      e.as_table().cloned());
            let mut fec = FunctionExecutor::new(memory.clone(), table, ext)?;
            Ok(intermediate_instance.run_start(&mut fec)?)
        }
    }
}
#[macro_use]
mod native_executor {
    use std::{borrow::BorrowMut, result, cell::{RefMut, RefCell}};
    use crate::error::{Error, ErrorKind, Result};
    use state_machine::{CodeExecutor, Externalities};
    use crate::wasm_executor::WasmExecutor;
    use wasmi::{Module as WasmModule, ModuleRef as WasmModuleInstanceRef};
    use runtime_version::{NativeVersion, RuntimeVersion};
    use std::{collections::HashMap, panic::UnwindSafe};
    use parity_codec::{Decode, Encode};
    use crate::RuntimeInfo;
    use primitives::{Blake2Hasher, NativeOrEncoded};
    use primitives::storage::well_known_keys;
    use log::trace;
    /// Default num of pages for the heap
    const DEFAULT_HEAP_PAGES: u64 = 1024;
    enum RuntimePreproc {
        InvalidCode,
        ValidCode(WasmModuleInstanceRef, Option<RuntimeVersion>),
    }
    type CacheType = HashMap<[u8; 32], RuntimePreproc>;
    const RUNTIMES_CACHE: ::std::thread::LocalKey<RefCell<CacheType>> =
        {
            #[inline]
            fn __init() -> RefCell<CacheType> { RefCell::new(HashMap::new()) }
            unsafe fn __getit()
             ->
                 ::std::option::Option<&'static ::std::cell::UnsafeCell<::std::option::Option<RefCell<CacheType>>>> {
                #[thread_local]
                #[cfg(all(target_thread_local,
                          not(all(target_arch = "wasm32",
                                  not(target_feature = "atomics")))))]
                static __KEY:
                       ::std::thread::__FastLocalKeyInner<RefCell<CacheType>>
                       =
                    ::std::thread::__FastLocalKeyInner::new();
                __KEY.get()
            }
            unsafe { ::std::thread::LocalKey::new(__getit, __init) }
        };
    /// fetch a runtime version from the cache or if there is no cached version yet, create
    /// the runtime version entry for `code`, determines whether `Compatibility::IsCompatible`
    /// can be used by comparing returned RuntimeVersion to `ref_version`
    fn fetch_cached_runtime_version<'a,
                                    E: Externalities<Blake2Hasher>>(wasm_executor:
                                                                        &WasmExecutor,
                                                                    cache:
                                                                        &'a mut RefMut<CacheType>,
                                                                    ext:
                                                                        &mut E,
                                                                    default_heap_pages:
                                                                        Option<u64>)
     -> Result<(&'a WasmModuleInstanceRef, &'a Option<RuntimeVersion>)> {
        let code_hash =
            match ext.original_storage_hash(well_known_keys::CODE) {
                Some(code_hash) => code_hash,
                None =>
                return Err(ErrorKind::InvalidCode(<[_]>::into_vec(box
                                                                      [])).into()),
            };
        let maybe_runtime_preproc =
            cache.borrow_mut().entry(code_hash.into()).or_insert_with(||
                                                                          {
                                                                              let code =
                                                                                  match ext.original_storage(well_known_keys::CODE)
                                                                                      {
                                                                                      Some(code)
                                                                                      =>
                                                                                      code,
                                                                                      None
                                                                                      =>
                                                                                      return RuntimePreproc::InvalidCode,
                                                                                  };
                                                                              let heap_pages =
                                                                                  ext.storage(well_known_keys::HEAP_PAGES).and_then(|pages|
                                                                                                                                        u64::decode(&mut &pages[..])).or(default_heap_pages).unwrap_or(DEFAULT_HEAP_PAGES);
                                                                              match WasmModule::from_buffer(code).map_err(|_|
                                                                                                                              ErrorKind::InvalidCode(<[_]>::into_vec(box
                                                                                                                                                                         [])).into()).and_then(|module|
                                                                                                                                                                                                   wasm_executor.prepare_module(ext,
                                                                                                                                                                                                                                heap_pages
                                                                                                                                                                                                                                    as
                                                                                                                                                                                                                                    usize,
                                                                                                                                                                                                                                &module))
                                                                                  {
                                                                                  Ok(module)
                                                                                  =>
                                                                                  {
                                                                                      let version =
                                                                                          wasm_executor.call_in_wasm_module(ext,
                                                                                                                            &module,
                                                                                                                            "Core_version",
                                                                                                                            &[]).ok().and_then(|v|
                                                                                                                                                   RuntimeVersion::decode(&mut v.as_slice()));
                                                                                      RuntimePreproc::ValidCode(module,
                                                                                                                version)
                                                                                  }
                                                                                  Err(e)
                                                                                  =>
                                                                                  {
                                                                                      {
                                                                                          let lvl =
                                                                                              ::log::Level::Trace;
                                                                                          if lvl
                                                                                                 <=
                                                                                                 ::log::STATIC_MAX_LEVEL
                                                                                                 &&
                                                                                                 lvl
                                                                                                     <=
                                                                                                     ::log::max_level()
                                                                                             {
                                                                                              ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Invalid code presented to executor (",
                                                                                                                                                       ")"],
                                                                                                                                                     &match (&e,)
                                                                                                                                                          {
                                                                                                                                                          (arg0,)
                                                                                                                                                          =>
                                                                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       ::std::fmt::Debug::fmt)],
                                                                                                                                                      }),
                                                                                                                       lvl,
                                                                                                                       &("executor",
                                                                                                                         "substrate_executor::native_executor",
                                                                                                                         "core/executor/src/native_executor.rs",
                                                                                                                         82u32));
                                                                                          }
                                                                                      };
                                                                                      RuntimePreproc::InvalidCode
                                                                                  }
                                                                              }
                                                                          });
        match maybe_runtime_preproc {
            RuntimePreproc::InvalidCode => {
                let code =
                    ext.original_storage(well_known_keys::CODE).unwrap_or(<[_]>::into_vec(box
                                                                                              []));
                Err(ErrorKind::InvalidCode(code).into())
            }
            RuntimePreproc::ValidCode(m, v) => { Ok((m, v)) }
        }
    }
    fn safe_call<F, U>(f: F) -> Result<U> where F: UnwindSafe + FnOnce() ->
     U {
        let _guard = panic_handler::AbortGuard::new(false);
        ::std::panic::catch_unwind(f).map_err(|_| ErrorKind::Runtime.into())
    }
    /// Set up the externalities and safe calling environment to execute calls to a native runtime.
    ///
    /// If the inner closure panics, it will be caught and return an error.
    pub fn with_native_environment<F,
                                   U>(ext: &mut Externalities<Blake2Hasher>,
                                      f: F) -> Result<U> where F: UnwindSafe +
     FnOnce() -> U {
        ::runtime_io::with_externalities(ext, move || safe_call(f))
    }
    /// Delegate for dispatching a CodeExecutor call to native code.
    pub trait NativeExecutionDispatch: Send + Sync {
        /// Get the wasm code that the native dispatch will be equivalent to.
        fn native_equivalent()
        -> &'static [u8];
        /// Dispatch a method and input data to be executed natively. Returns `Some` result or `None`
        /// if the `method` is unknown. Panics if there's an unrecoverable error.
        fn dispatch(ext: &mut Externalities<Blake2Hasher>, method: &str,
                    data: &[u8])
        -> Result<Vec<u8>>;
        /// Provide native runtime version.
        fn native_version()
        -> NativeVersion;
        /// Construct corresponding `NativeExecutor`
        fn new(default_heap_pages: Option<u64>)
        -> NativeExecutor<Self>
        where
        Self: Sized;
    }
    /// A generic `CodeExecutor` implementation that uses a delegate to determine wasm code equivalence
    /// and dispatch to native code when possible, falling back on `WasmExecutor` when not.
    pub struct NativeExecutor<D: NativeExecutionDispatch> {
        /// Dummy field to avoid the compiler complaining about us not using `D`.
        _dummy: ::std::marker::PhantomData<D>,
        /// The fallback executor in case native isn't available.
        fallback: WasmExecutor,
        /// Native runtime version info.
        native_version: NativeVersion,
        /// The default number of 64KB pages to allocate for Wasm execution.
        default_heap_pages: Option<u64>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <D: ::std::fmt::Debug + NativeExecutionDispatch> ::std::fmt::Debug
     for NativeExecutor<D> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                NativeExecutor {
                _dummy: ref __self_0_0,
                fallback: ref __self_0_1,
                native_version: ref __self_0_2,
                default_heap_pages: ref __self_0_3 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("NativeExecutor");
                    let _ =
                        debug_trait_builder.field("_dummy", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("fallback",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("native_version",
                                                  &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("default_heap_pages",
                                                  &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <D: NativeExecutionDispatch> NativeExecutor<D> {
        /// Create new instance.
        pub fn new(default_heap_pages: Option<u64>) -> Self {
            NativeExecutor{_dummy: Default::default(),
                           fallback: WasmExecutor::new(),
                           native_version: D::native_version(),
                           default_heap_pages,}
        }
    }
    impl <D: NativeExecutionDispatch> Clone for NativeExecutor<D> {
        fn clone(&self) -> Self {
            NativeExecutor{_dummy: Default::default(),
                           fallback: self.fallback.clone(),
                           native_version: D::native_version(),
                           default_heap_pages: self.default_heap_pages,}
        }
    }
    impl <D: NativeExecutionDispatch> RuntimeInfo for NativeExecutor<D> {
        fn native_version(&self) -> &NativeVersion { &self.native_version }
        fn runtime_version<E: Externalities<Blake2Hasher>>(&self, ext: &mut E)
         -> Option<RuntimeVersion> {
            RUNTIMES_CACHE.with(|c|
                                    fetch_cached_runtime_version(&self.fallback,
                                                                 &mut c.borrow_mut(),
                                                                 ext,
                                                                 self.default_heap_pages).ok()?.1.clone())
        }
    }
    impl <D: NativeExecutionDispatch> CodeExecutor<Blake2Hasher> for
     NativeExecutor<D> {
        type
        Error
        =
        Error;
        fn call<E: Externalities<Blake2Hasher>, R: Decode + Encode +
                PartialEq, NC: FnOnce() -> result::Result<R, &'static str> +
                UnwindSafe>(&self, ext: &mut E, method: &str, data: &[u8],
                            use_native: bool, native_call: Option<NC>)
         -> (Result<NativeOrEncoded<R>>, bool) {
            RUNTIMES_CACHE.with(|c|
                                    {
                                        let mut c = c.borrow_mut();
                                        let (module, onchain_version) =
                                            match fetch_cached_runtime_version(&self.fallback,
                                                                               &mut c,
                                                                               ext,
                                                                               self.default_heap_pages)
                                                {
                                                Ok((module, onchain_version))
                                                => (module, onchain_version),
                                                Err(e) =>
                                                return (Err(e), false),
                                            };
                                        match (use_native,
                                               onchain_version.as_ref().map_or(false,
                                                                               |v|
                                                                                   v.can_call_with(&self.native_version.runtime_version)),
                                               native_call) {
                                            (_, false, _) => {
                                                {
                                                    let lvl =
                                                        ::log::Level::Trace;
                                                    if lvl <=
                                                           ::log::STATIC_MAX_LEVEL
                                                           &&
                                                           lvl <=
                                                               ::log::max_level()
                                                       {
                                                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Request for native execution failed (native: ",
                                                                                                                 ", chain: ",
                                                                                                                 ")"],
                                                                                                               &match (&self.native_version.runtime_version,
                                                                                                                       &onchain_version.as_ref().map_or_else(||
                                                                                                                                                                 "<None>".into(),
                                                                                                                                                             |v|
                                                                                                                                                                 ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                                    &match (&v,)
                                                                                                                                                                                                                         {
                                                                                                                                                                                                                         (arg0,)
                                                                                                                                                                                                                         =>
                                                                                                                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                                      ::std::fmt::Display::fmt)],
                                                                                                                                                                                                                     }))))
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
                                                                                 &("executor",
                                                                                   "substrate_executor::native_executor",
                                                                                   "core/executor/src/native_executor.rs",
                                                                                   217u32));
                                                    }
                                                };
                                                (self.fallback.call_in_wasm_module(ext,
                                                                                   module,
                                                                                   method,
                                                                                   data).map(NativeOrEncoded::Encoded),
                                                 false)
                                            }
                                            (false, _, _) => {
                                                (self.fallback.call_in_wasm_module(ext,
                                                                                   module,
                                                                                   method,
                                                                                   data).map(NativeOrEncoded::Encoded),
                                                 false)
                                            }
                                            (true, true, Some(call)) => {
                                                {
                                                    let lvl =
                                                        ::log::Level::Trace;
                                                    if lvl <=
                                                           ::log::STATIC_MAX_LEVEL
                                                           &&
                                                           lvl <=
                                                               ::log::max_level()
                                                       {
                                                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Request for native execution with native call succeeded (native: ",
                                                                                                                 ", chain: ",
                                                                                                                 ")."],
                                                                                                               &match (&self.native_version.runtime_version,
                                                                                                                       &onchain_version.as_ref().map_or_else(||
                                                                                                                                                                 "<None>".into(),
                                                                                                                                                             |v|
                                                                                                                                                                 ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                                    &match (&v,)
                                                                                                                                                                                                                         {
                                                                                                                                                                                                                         (arg0,)
                                                                                                                                                                                                                         =>
                                                                                                                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                                      ::std::fmt::Display::fmt)],
                                                                                                                                                                                                                     }))))
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
                                                                                 &("executor",
                                                                                   "substrate_executor::native_executor",
                                                                                   "core/executor/src/native_executor.rs",
                                                                                   241u32));
                                                    }
                                                };
                                                (with_native_environment(ext,
                                                                         move
                                                                             ||
                                                                             (call)()).and_then(|r|
                                                                                                    r.map(NativeOrEncoded::Native).map_err(Into::into)),
                                                 true)
                                            }
                                            _ => {
                                                {
                                                    let lvl =
                                                        ::log::Level::Trace;
                                                    if lvl <=
                                                           ::log::STATIC_MAX_LEVEL
                                                           &&
                                                           lvl <=
                                                               ::log::max_level()
                                                       {
                                                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Request for native execution succeeded (native: ",
                                                                                                                 ", chain: ",
                                                                                                                 ")"],
                                                                                                               &match (&self.native_version.runtime_version,
                                                                                                                       &onchain_version.as_ref().map_or_else(||
                                                                                                                                                                 "<None>".into(),
                                                                                                                                                             |v|
                                                                                                                                                                 ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                                    &match (&v,)
                                                                                                                                                                                                                         {
                                                                                                                                                                                                                         (arg0,)
                                                                                                                                                                                                                         =>
                                                                                                                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                                      ::std::fmt::Display::fmt)],
                                                                                                                                                                                                                     }))))
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
                                                                                 &("executor",
                                                                                   "substrate_executor::native_executor",
                                                                                   "core/executor/src/native_executor.rs",
                                                                                   256u32));
                                                    }
                                                };
                                                (D::dispatch(ext, method,
                                                             data).map(NativeOrEncoded::Encoded),
                                                 true)
                                            }
                                        }
                                    })
        }
    }
    /// Implements a `NativeExecutionDispatch` for provided parameters.
    #[macro_export]
    macro_rules! native_executor_instance((
                                          $ pub : vis $ name : ident , $
                                          dispatcher : path , $ version : path
                                          , $ code : expr ) => {
                                          /// A unit struct which implements `NativeExecutionDispatch` feeding in the hard-coded runtime.
                                           $ pub struct $ name ;
                                          native_executor_instance ! (
                                          IMPL $ name , $ dispatcher , $
                                          version , $ code ) ; } ; (
                                          IMPL $ name : ident , $ dispatcher :
                                          path , $ version : path , $ code :
                                          expr ) => {
                                          impl $ crate ::
                                          NativeExecutionDispatch for $ name {
                                          fn native_equivalent (  ) -> &
                                          'static [ u8 ] { $ code } fn
                                          dispatch (
                                          ext : & mut $ crate :: Externalities
                                          < $ crate :: Blake2Hasher > , method
                                          : & str , data : & [ u8 ] ) -> $
                                          crate :: error :: Result < Vec < u8
                                          >> {
                                          $ crate :: with_native_environment (
                                          ext , move || $ dispatcher (
                                          method , data ) ) ? . ok_or_else (
                                          || $ crate :: error :: ErrorKind ::
                                          MethodNotFound (
                                          method . to_owned (  ) ) . into (  )
                                          ) } fn native_version (  ) -> $
                                          crate :: NativeVersion {
                                          $ version (  ) } fn new (
                                          default_heap_pages : Option < u64 >
                                          ) -> $ crate :: NativeExecutor < $
                                          name > {
                                          $ crate :: NativeExecutor :: new (
                                          default_heap_pages ) } } });
}
mod sandbox {
    #![warn(missing_docs)]
    //! This module implements sandboxing support in the runtime.
    use std::collections::HashMap;
    use std::rc::Rc;
    use parity_codec::{Decode, Encode};
    use primitives::sandbox as sandbox_primitives;
    use crate::wasm_utils::UserError;
    use wasmi;
    use wasmi::memory_units::Pages;
    use wasmi::{Externals, FuncRef, ImportResolver, MemoryInstance, MemoryRef,
                Module, ModuleInstance, ModuleRef, RuntimeArgs, RuntimeValue,
                Trap, TrapKind};
    /// Index of a function inside the supervisor.
    ///
    /// This is a typically an index in the default table of the supervisor, however
    /// the exact meaning of this index is depends on the implementation of dispatch function.
    #[rustc_copy_clone_marker]
    struct SupervisorFuncIndex(usize);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for SupervisorFuncIndex { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for SupervisorFuncIndex {
        #[inline]
        fn clone(&self) -> SupervisorFuncIndex {
            { let _: ::std::clone::AssertParamIsClone<usize>; *self }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for SupervisorFuncIndex {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                SupervisorFuncIndex(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("SupervisorFuncIndex");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for SupervisorFuncIndex {
        #[inline]
        fn eq(&self, other: &SupervisorFuncIndex) -> bool {
            match *other {
                SupervisorFuncIndex(ref __self_1_0) =>
                match *self {
                    SupervisorFuncIndex(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SupervisorFuncIndex) -> bool {
            match *other {
                SupervisorFuncIndex(ref __self_1_0) =>
                match *self {
                    SupervisorFuncIndex(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    /// Index of a function within guest index space.
    ///
    /// This index is supposed to be used with as index for `Externals`.
    #[rustc_copy_clone_marker]
    struct GuestFuncIndex(usize);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for GuestFuncIndex { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for GuestFuncIndex {
        #[inline]
        fn clone(&self) -> GuestFuncIndex {
            { let _: ::std::clone::AssertParamIsClone<usize>; *self }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for GuestFuncIndex {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                GuestFuncIndex(ref __self_0_0) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("GuestFuncIndex");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for GuestFuncIndex {
        #[inline]
        fn eq(&self, other: &GuestFuncIndex) -> bool {
            match *other {
                GuestFuncIndex(ref __self_1_0) =>
                match *self {
                    GuestFuncIndex(ref __self_0_0) =>
                    (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &GuestFuncIndex) -> bool {
            match *other {
                GuestFuncIndex(ref __self_1_0) =>
                match *self {
                    GuestFuncIndex(ref __self_0_0) =>
                    (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    /// This struct holds a mapping from guest index space to supervisor.
    struct GuestToSupervisorFunctionMapping {
        funcs: Vec<SupervisorFuncIndex>,
    }
    impl GuestToSupervisorFunctionMapping {
        fn new() -> GuestToSupervisorFunctionMapping {
            GuestToSupervisorFunctionMapping{funcs: Vec::new(),}
        }
        fn define(&mut self, supervisor_func: SupervisorFuncIndex)
         -> GuestFuncIndex {
            let idx = self.funcs.len();
            self.funcs.push(supervisor_func);
            GuestFuncIndex(idx)
        }
        fn func_by_guest_index(&self, guest_func_idx: GuestFuncIndex)
         -> Option<SupervisorFuncIndex> {
            self.funcs.get(guest_func_idx.0).cloned()
        }
    }
    struct Imports {
        func_map: HashMap<(Vec<u8>, Vec<u8>), GuestFuncIndex>,
        memories_map: HashMap<(Vec<u8>, Vec<u8>), MemoryRef>,
    }
    impl ImportResolver for Imports {
        fn resolve_func(&self, module_name: &str, field_name: &str,
                        signature: &::wasmi::Signature)
         -> Result<FuncRef, ::wasmi::Error> {
            let key =
                (module_name.as_bytes().to_owned(),
                 field_name.as_bytes().to_owned());
            let idx =
                *self.func_map.get(&key).ok_or_else(||
                                                        {
                                                            ::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                                               ":",
                                                                                                                                               " not found"],
                                                                                                                                             &match (&module_name,
                                                                                                                                                     &field_name)
                                                                                                                                                  {
                                                                                                                                                  (arg0,
                                                                                                                                                   arg1)
                                                                                                                                                  =>
                                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                               ::std::fmt::Display::fmt),
                                                                                                                                                   ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                               ::std::fmt::Display::fmt)],
                                                                                                                                              })))
                                                        })?;
            Ok(::wasmi::FuncInstance::alloc_host(signature.clone(), idx.0))
        }
        fn resolve_memory(&self, module_name: &str, field_name: &str,
                          _memory_type: &::wasmi::MemoryDescriptor)
         -> Result<MemoryRef, ::wasmi::Error> {
            let key =
                (module_name.as_bytes().to_vec(),
                 field_name.as_bytes().to_vec());
            let mem =
                self.memories_map.get(&key).ok_or_else(||
                                                           {
                                                               ::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                                                  ":",
                                                                                                                                                  " not found"],
                                                                                                                                                &match (&module_name,
                                                                                                                                                        &field_name)
                                                                                                                                                     {
                                                                                                                                                     (arg0,
                                                                                                                                                      arg1)
                                                                                                                                                     =>
                                                                                                                                                     [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                  ::std::fmt::Display::fmt),
                                                                                                                                                      ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                                                  ::std::fmt::Display::fmt)],
                                                                                                                                                 })))
                                                           })?.clone();
            Ok(mem)
        }
        fn resolve_global(&self, module_name: &str, field_name: &str,
                          _global_type: &::wasmi::GlobalDescriptor)
         -> Result<::wasmi::GlobalRef, ::wasmi::Error> {
            Err(::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                   ":",
                                                                                                   " not found"],
                                                                                                 &match (&module_name,
                                                                                                         &field_name)
                                                                                                      {
                                                                                                      (arg0,
                                                                                                       arg1)
                                                                                                      =>
                                                                                                      [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                   ::std::fmt::Display::fmt),
                                                                                                       ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                   ::std::fmt::Display::fmt)],
                                                                                                  }))))
        }
        fn resolve_table(&self, module_name: &str, field_name: &str,
                         _table_type: &::wasmi::TableDescriptor)
         -> Result<::wasmi::TableRef, ::wasmi::Error> {
            Err(::wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                   ":",
                                                                                                   " not found"],
                                                                                                 &match (&module_name,
                                                                                                         &field_name)
                                                                                                      {
                                                                                                      (arg0,
                                                                                                       arg1)
                                                                                                      =>
                                                                                                      [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                   ::std::fmt::Display::fmt),
                                                                                                       ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                   ::std::fmt::Display::fmt)],
                                                                                                  }))))
        }
    }
    /// This trait encapsulates sandboxing capabilities.
    ///
    /// Note that this functions are only called in the `supervisor` context.
    pub trait SandboxCapabilities {
        /// Returns a reference to an associated sandbox `Store`.
        fn store(&self)
        -> &Store;
        /// Returns a mutable reference to an associated sandbox `Store`.
        fn store_mut(&mut self)
        -> &mut Store;
        /// Allocate space of the specified length in the supervisor memory.
        ///
        /// # Errors
        ///
        /// Returns `Err` if allocation not possible or errors during heap management.
        ///
        /// Returns pointer to the allocated block.
        fn allocate(&mut self, len: u32)
        -> Result<u32, UserError>;
        /// Deallocate space specified by the pointer that was previously returned by [`allocate`].
        ///
        /// # Errors
        ///
        /// Returns `Err` if deallocation not possible or because of errors in heap management.
        ///
        /// [`allocate`]: #tymethod.allocate
        fn deallocate(&mut self, ptr: u32)
        -> Result<(), UserError>;
        /// Write `data` into the supervisor memory at offset specified by `ptr`.
        ///
        /// # Errors
        ///
        /// Returns `Err` if `ptr + data.len()` is out of bounds.
        fn write_memory(&mut self, ptr: u32, data: &[u8])
        -> Result<(), UserError>;
        /// Read `len` bytes from the supervisor memory.
        ///
        /// # Errors
        ///
        /// Returns `Err` if `ptr + len` is out of bounds.
        fn read_memory(&self, ptr: u32, len: u32)
        -> Result<Vec<u8>, UserError>;
    }
    /// Implementation of [`Externals`] that allows execution of guest module with
    /// [externals][`Externals`] that might refer functions defined by supervisor.
    ///
    /// [`Externals`]: ../../wasmi/trait.Externals.html
    pub struct GuestExternals<'a, FE: SandboxCapabilities + Externals + 'a> {
        supervisor_externals: &'a mut FE,
        sandbox_instance: &'a SandboxInstance,
        state: u32,
    }
    fn trap(msg: &'static str) -> Trap {
        TrapKind::Host(Box::new(UserError(msg))).into()
    }
    fn deserialize_result(serialized_result: &[u8])
     -> Result<Option<RuntimeValue>, Trap> {
        use self::sandbox_primitives::{HostError, ReturnValue};
        let result_val =
            Result::<ReturnValue,
                     HostError>::decode(&mut &serialized_result[..]).ok_or_else(||
                                                                                    trap("Decoding Result<ReturnValue, HostError> failed!"))?;
        match result_val {
            Ok(return_value) =>
            Ok(match return_value {
                   ReturnValue::Unit => None,
                   ReturnValue::Value(typed_value) =>
                   Some(RuntimeValue::from(typed_value)),
               }),
            Err(HostError) =>
            Err(trap("Supervisor function returned sandbox::HostError")),
        }
    }
    impl <'a, FE: SandboxCapabilities + Externals + 'a> Externals for
     GuestExternals<'a, FE> {
        fn invoke_index(&mut self, index: usize, args: RuntimeArgs)
         -> Result<Option<RuntimeValue>, Trap> {
            let index = GuestFuncIndex(index);
            let dispatch_thunk = self.sandbox_instance.dispatch_thunk.clone();
            let func_idx =
                self.sandbox_instance.guest_to_supervisor_mapping.func_by_guest_index(index).expect("`invoke_index` is called with indexes registered via `FuncInstance::alloc_host`;
					`FuncInstance::alloc_host` is called with indexes that was obtained from `guest_to_supervisor_mapping`;
					`func_by_guest_index` called with `index` can't return `None`;
					qed");
            let invoke_args_data: Vec<u8> =
                args.as_ref().iter().cloned().map(sandbox_primitives::TypedValue::from).collect::<Vec<_>>().encode();
            let state = self.state;
            let invoke_args_ptr =
                self.supervisor_externals.allocate(invoke_args_data.len() as
                                                       u32)?;
            self.supervisor_externals.write_memory(invoke_args_ptr,
                                                   &invoke_args_data)?;
            let result =
                ::wasmi::FuncInstance::invoke(&dispatch_thunk,
                                              &[RuntimeValue::I32(invoke_args_ptr
                                                                      as i32),
                                                RuntimeValue::I32(invoke_args_data.len()
                                                                      as i32),
                                                RuntimeValue::I32(state as
                                                                      i32),
                                                RuntimeValue::I32(func_idx.0
                                                                      as
                                                                      i32)],
                                              self.supervisor_externals);
            self.supervisor_externals.deallocate(invoke_args_ptr)?;
            let (serialized_result_val_ptr, serialized_result_val_len) =
                match result {
                    Ok(Some(RuntimeValue::I64(v))) => {
                        let v = v as u64;
                        let ptr = (v as u64 >> 32) as u32;
                        let len = (v & 0xFFFFFFFF) as u32;
                        (ptr, len)
                    }
                    Ok(_) =>
                    return Err(trap("Supervisor function returned unexpected result!")),
                    Err(_) =>
                    return Err(trap("Supervisor function trapped!")),
                };
            let serialized_result_val =
                self.supervisor_externals.read_memory(serialized_result_val_ptr,
                                                      serialized_result_val_len)?;
            self.supervisor_externals.deallocate(serialized_result_val_ptr)?;
            deserialize_result(&serialized_result_val)
        }
    }
    fn with_guest_externals<FE, R,
                            F>(supervisor_externals: &mut FE,
                               sandbox_instance: &SandboxInstance, state: u32,
                               f: F) -> R where FE: SandboxCapabilities +
     Externals, F: FnOnce(&mut GuestExternals<FE>) -> R {
        let mut guest_externals =
            GuestExternals{supervisor_externals, sandbox_instance, state,};
        f(&mut guest_externals)
    }
    /// Sandboxed instance of a wasm module.
    ///
    /// It's primary purpose is to [`invoke`] exported functions on it.
    ///
    /// All imports of this instance are specified at the creation time and
    /// imports are implemented by the supervisor.
    ///
    /// Hence, in order to invoke an exported function on a sandboxed module instance,
    /// it's required to provide supervisor externals: it will be used to execute
    /// code in the supervisor context.
    ///
    /// [`invoke`]: #method.invoke
    pub struct SandboxInstance {
        instance: ModuleRef,
        dispatch_thunk: FuncRef,
        guest_to_supervisor_mapping: GuestToSupervisorFunctionMapping,
    }
    impl SandboxInstance {
        /// Invoke an exported function by a name.
        ///
        /// `supervisor_externals` is required to execute the implementations
        /// of the syscalls that published to a sandboxed module instance.
        ///
        /// The `state` parameter can be used to provide custom data for
        /// these syscall implementations.
        pub fn invoke<FE: SandboxCapabilities +
                      Externals>(&self, export_name: &str,
                                 args: &[RuntimeValue],
                                 supervisor_externals: &mut FE, state: u32)
         -> Result<Option<wasmi::RuntimeValue>, wasmi::Error> {
            with_guest_externals(supervisor_externals, self, state,
                                 |guest_externals|
                                     {
                                         self.instance.invoke_export(export_name,
                                                                     args,
                                                                     guest_externals)
                                     })
        }
    }
    /// Error occured during instantiation of a sandboxed module.
    pub enum InstantiationError {

        /// Something wrong with the environment definition. It either can't
        /// be decoded, have a reference to a non-existent or torn down memory instance.
        EnvironmentDefintionCorrupted,

        /// Provided module isn't recognized as a valid webassembly binary.
        ModuleDecoding,

        /// Module is a well-formed webassembly binary but could not be instantiated. This could
        /// happen because, e.g. the module imports entries not provided by the environment.
        Instantiation,

        /// Module is well-formed, instantiated and linked, but while executing the start function
        /// a trap was generated.
        StartTrapped,
    }
    fn decode_environment_definition(raw_env_def: &[u8],
                                     memories: &[Option<MemoryRef>])
     ->
         Result<(Imports, GuestToSupervisorFunctionMapping),
                InstantiationError> {
        let env_def =
            sandbox_primitives::EnvironmentDefinition::decode(&mut &raw_env_def[..]).ok_or_else(||
                                                                                                    InstantiationError::EnvironmentDefintionCorrupted)?;
        let mut func_map = HashMap::new();
        let mut memories_map = HashMap::new();
        let mut guest_to_supervisor_mapping =
            GuestToSupervisorFunctionMapping::new();
        for entry in &env_def.entries {
            let module = entry.module_name.clone();
            let field = entry.field_name.clone();
            match entry.entity {
                sandbox_primitives::ExternEntity::Function(func_idx) => {
                    let externals_idx =
                        guest_to_supervisor_mapping.define(SupervisorFuncIndex(func_idx
                                                                                   as
                                                                                   usize));
                    func_map.insert((module, field), externals_idx);
                }
                sandbox_primitives::ExternEntity::Memory(memory_idx) => {
                    let memory_ref =
                        memories.get(memory_idx as
                                         usize).cloned().ok_or_else(||
                                                                        InstantiationError::EnvironmentDefintionCorrupted)?.ok_or_else(||
                                                                                                                                           InstantiationError::EnvironmentDefintionCorrupted)?;
                    memories_map.insert((module, field), memory_ref);
                }
            }
        }
        Ok((Imports{func_map, memories_map,}, guest_to_supervisor_mapping))
    }
    /// Instantiate a guest module and return it's index in the store.
    ///
    /// The guest module's code is specified in `wasm`. Environment that will be available to
    /// guest module is specified in `raw_env_def` (serialized version of [`EnvironmentDefinition`]).
    /// `dispatch_thunk` is used as function that handle calls from guests.
    ///
    /// # Errors
    ///
    /// Returns `Err` if any of the following conditions happens:
    ///
    /// - `raw_env_def` can't be deserialized as a [`EnvironmentDefinition`].
    /// - Module in `wasm` is invalid or couldn't be instantiated.
    ///
    /// [`EnvironmentDefinition`]: ../../sandbox/struct.EnvironmentDefinition.html
    pub fn instantiate<FE: SandboxCapabilities +
                       Externals>(supervisor_externals: &mut FE,
                                  dispatch_thunk: FuncRef, wasm: &[u8],
                                  raw_env_def: &[u8], state: u32)
     -> Result<u32, InstantiationError> {
        let (imports, guest_to_supervisor_mapping) =
            decode_environment_definition(raw_env_def,
                                          &supervisor_externals.store().memories)?;
        let module =
            Module::from_buffer(wasm).map_err(|_|
                                                  InstantiationError::ModuleDecoding)?;
        let instance =
            ModuleInstance::new(&module,
                                &imports).map_err(|_|
                                                      InstantiationError::Instantiation)?;
        let sandbox_instance =
            Rc::new(SandboxInstance{instance:
                                        instance.not_started_instance().clone(),
                                    dispatch_thunk,
                                    guest_to_supervisor_mapping,});
        with_guest_externals(supervisor_externals, &sandbox_instance, state,
                             |guest_externals|
                                 {
                                     instance.run_start(guest_externals).map_err(|_|
                                                                                     InstantiationError::StartTrapped)
                                 })?;
        let instance_idx =
            supervisor_externals.store_mut().register_sandbox_instance(sandbox_instance);
        Ok(instance_idx)
    }
    /// This struct keeps track of all sandboxed components.
    pub struct Store {
        instances: Vec<Option<Rc<SandboxInstance>>>,
        memories: Vec<Option<MemoryRef>>,
    }
    impl Store {
        /// Create a new empty sandbox store.
        pub fn new() -> Store {
            Store{instances: Vec::new(), memories: Vec::new(),}
        }
        /// Create a new memory instance and return it's index.
        ///
        /// # Errors
        ///
        /// Returns `Err` if the memory couldn't be created.
        /// Typically happens if `initial` is more than `maximum`.
        pub fn new_memory(&mut self, initial: u32, maximum: u32)
         -> Result<u32, UserError> {
            let maximum =
                match maximum {
                    sandbox_primitives::MEM_UNLIMITED => None,
                    specified_limit => Some(Pages(specified_limit as usize)),
                };
            let mem =
                MemoryInstance::alloc(Pages(initial as usize),
                                      maximum).map_err(|_|
                                                           UserError("Sandboxed memory allocation error"))?;
            let mem_idx = self.memories.len();
            self.memories.push(Some(mem));
            Ok(mem_idx as u32)
        }
        /// Returns `SandboxInstance` by `instance_idx`.
        ///
        /// # Errors
        ///
        /// Returns `Err` If `instance_idx` isn't a valid index of an instance or
        /// instance is already torndown.
        pub fn instance(&self, instance_idx: u32)
         -> Result<Rc<SandboxInstance>, UserError> {
            self.instances.get(instance_idx as
                                   usize).cloned().ok_or_else(||
                                                                  UserError("Trying to access a non-existent instance"))?.ok_or_else(||
                                                                                                                                         UserError("Trying to access a torndown instance"))
        }
        /// Returns reference to a memory instance by `memory_idx`.
        ///
        /// # Errors
        ///
        /// Returns `Err` If `memory_idx` isn't a valid index of an memory or
        /// if memory has been torn down.
        pub fn memory(&self, memory_idx: u32)
         -> Result<MemoryRef, UserError> {
            self.memories.get(memory_idx as
                                  usize).cloned().ok_or_else(||
                                                                 UserError("Trying to access a non-existent sandboxed memory"))?.ok_or_else(||
                                                                                                                                                UserError("Trying to access a torndown sandboxed memory"))
        }
        /// Tear down the memory at the specified index.
        ///
        /// # Errors
        ///
        /// Returns `Err` if `memory_idx` isn't a valid index of an memory or
        /// if it has been torn down.
        pub fn memory_teardown(&mut self, memory_idx: u32)
         -> Result<(), UserError> {
            match self.memories.get_mut(memory_idx as usize) {
                None =>
                Err(UserError("Trying to teardown a non-existent sandboxed memory")),
                Some(None) =>
                Err(UserError("Double teardown of a sandboxed memory")),
                Some(memory) => { *memory = None; Ok(()) }
            }
        }
        /// Tear down the instance at the specified index.
        ///
        /// # Errors
        ///
        /// Returns `Err` if `instance_idx` isn't a valid index of an instance or
        /// if it has been torn down.
        pub fn instance_teardown(&mut self, instance_idx: u32)
         -> Result<(), UserError> {
            match self.instances.get_mut(instance_idx as usize) {
                None =>
                Err(UserError("Trying to teardown a non-existent instance")),
                Some(None) =>
                Err(UserError("Double teardown of an instance")),
                Some(instance) => { *instance = None; Ok(()) }
            }
        }
        fn register_sandbox_instance(&mut self,
                                     sandbox_instance: Rc<SandboxInstance>)
         -> u32 {
            let instance_idx = self.instances.len();
            self.instances.push(Some(sandbox_instance));
            instance_idx as u32
        }
    }
}
mod allocator {
    //! This module implements a freeing-bump allocator.
    //! See more details at https://github.com/paritytech/substrate/issues/1615.
    use crate::wasm_utils::UserError;
    use log::trace;
    use wasmi::Error;
    use wasmi::MemoryRef;
    use wasmi::memory_units::Bytes;
    const ALIGNMENT: u32 = 8;
    const N: usize = 22;
    const MAX_POSSIBLE_ALLOCATION: u32 = 16777216;
    pub const OUT_OF_SPACE: &str =
        "Requested allocation size does not fit into remaining heap space";
    pub const REQUESTED_SIZE_TOO_LARGE: &str =
        "Requested size to allocate is too large";
    pub struct FreeingBumpHeapAllocator {
        bumper: u32,
        heads: [u32; N],
        heap: MemoryRef,
        max_heap_size: u32,
        ptr_offset: u32,
        total_size: u32,
    }
    impl FreeingBumpHeapAllocator {
        /// Creates a new allocation heap which follows a freeing-bump strategy.
        /// The maximum size which can be allocated at once is 16 MiB.
        ///
        /// # Arguments
        ///
        /// * `ptr_offset` - The pointers returned by `allocate()` start from this
        ///   offset on. The pointer offset needs to be aligned to a multiple of 8,
        ///   hence a padding might be added to align `ptr_offset` properly.
        ///
        /// * `heap_size` - The size available to this heap instance (in bytes) for
        ///   allocating memory.
        ///
        /// * `heap` - A `MemoryRef` to the available `MemoryInstance` which is
        ///   used as the heap.
        ///
        pub fn new(mem: MemoryRef) -> Self {
            let current_size: Bytes = mem.current_size().into();
            let current_size = current_size.0 as u32;
            let used_size = mem.used_size().0 as u32;
            let heap_size = current_size - used_size;
            let mut ptr_offset = used_size;
            let padding = ptr_offset % ALIGNMENT;
            if padding != 0 { ptr_offset += ALIGNMENT - padding; }
            FreeingBumpHeapAllocator{bumper: 0,
                                     heads: [0; N],
                                     heap: mem,
                                     max_heap_size: heap_size,
                                     ptr_offset: ptr_offset,
                                     total_size: 0,}
        }
        /// Gets requested number of bytes to allocate and returns a pointer.
        /// The maximum size which can be allocated at once is 16 MiB.
        pub fn allocate(&mut self, size: u32) -> Result<u32, UserError> {
            if size > MAX_POSSIBLE_ALLOCATION {
                return Err(UserError(REQUESTED_SIZE_TOO_LARGE));
            }
            let size = size.max(8);
            let item_size = size.next_power_of_two();
            if item_size + 8 + self.total_size > self.max_heap_size {
                return Err(UserError(OUT_OF_SPACE));
            }
            let list_index = (item_size.trailing_zeros() - 3) as usize;
            let ptr: u32 =
                if self.heads[list_index] != 0 {
                    let item = self.heads[list_index];
                    let four_bytes =
                        self.get_heap_4bytes(item).map_err(|_|
                                                               UserError("Unable to get bytes at pointer taken from list of free items"))?;
                    self.heads[list_index] =
                        FreeingBumpHeapAllocator::le_bytes_to_u32(four_bytes);
                    item + 8
                } else { self.bump(item_size + 8) + 8 };
            for i in 1..8 {
                self.set_heap(ptr - i,
                              255).map_err(|_|
                                               UserError("Unable to successively write bytes into heap at pointer prefix"))?;
            }
            self.set_heap(ptr - 8,
                          list_index as
                              u8).map_err(|_|
                                              UserError("Unable to write byte into heap at pointer prefix"))?;
            self.total_size = self.total_size + item_size + 8;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Heap size is ",
                                                                             " bytes after allocation"],
                                                                           &match (&self.total_size,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }),
                                             lvl,
                                             &("wasm-heap",
                                               "substrate_executor::allocator",
                                               "core/executor/src/allocator.rs",
                                               123u32));
                }
            };
            Ok(self.ptr_offset + ptr)
        }
        /// Deallocates the space which was allocated for a pointer.
        pub fn deallocate(&mut self, ptr: u32) -> Result<(), UserError> {
            let ptr = ptr - self.ptr_offset;
            if ptr < 8 {
                return Err(UserError("Invalid pointer for deallocation"));
            }
            let list_index =
                self.get_heap_byte(ptr -
                                       8).map_err(|_|
                                                      UserError("Unable to access pointer prefix"))?
                    as usize;
            for i in 1..8 {
                let heap_byte =
                    self.get_heap_byte(ptr -
                                           i).map_err(|_|
                                                          UserError("Unable to write single bytes into heap at pointer"))?;
                if true {
                    if !(heap_byte == 255) {
                        {
                            ::std::rt::begin_panic("assertion failed: heap_byte == 255",
                                                   &("core/executor/src/allocator.rs",
                                                     140u32, 4u32))
                        }
                    };
                }
            }
            let tail = self.heads[list_index];
            self.heads[list_index] = ptr - 8;
            let mut slice =
                self.get_heap_4bytes(ptr -
                                         8).map_err(|_|
                                                        UserError("Unable to get 4 bytes from heap at pointer prefix"))?;
            FreeingBumpHeapAllocator::write_u32_into_le_bytes(tail,
                                                              &mut slice);
            self.set_heap_4bytes(ptr - 8,
                                 slice).map_err(|_|
                                                    UserError("Unable to write 4 bytes into heap at pointer prefix"))?;
            let item_size =
                FreeingBumpHeapAllocator::get_item_size_from_index(list_index);
            self.total_size =
                self.total_size.checked_sub(item_size as u32 +
                                                8).ok_or_else(||
                                                                  UserError("Unable to subtract from total heap size without overflow"))?;
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Heap size is ",
                                                                             " bytes after deallocation"],
                                                                           &match (&self.total_size,)
                                                                                {
                                                                                (arg0,)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            }),
                                             lvl,
                                             &("wasm-heap",
                                               "substrate_executor::allocator",
                                               "core/executor/src/allocator.rs",
                                               154u32));
                }
            };
            Ok(())
        }
        fn bump(&mut self, n: u32) -> u32 {
            let res = self.bumper;
            self.bumper += n;
            res
        }
        fn le_bytes_to_u32(arr: [u8; 4]) -> u32 {
            let bytes = [arr[0], arr[1], arr[2], arr[3]];
            unsafe { std::mem::transmute::<[u8; 4], u32>(bytes) }.to_le()
        }
        fn write_u32_into_le_bytes(bytes: u32, slice: &mut [u8]) {
            let bytes: [u8; 4] =
                unsafe { std::mem::transmute::<u32, [u8; 4]>(bytes.to_le()) };
            for i in 0..4 { slice[i] = bytes[i]; }
        }
        fn get_item_size_from_index(index: usize) -> usize { 1 << 3 << index }
        fn get_heap_4bytes(&mut self, ptr: u32) -> Result<[u8; 4], Error> {
            let mut arr = [0u8; 4];
            self.heap.get_into(self.ptr_offset + ptr, &mut arr)?;
            Ok(arr)
        }
        fn get_heap_byte(&mut self, ptr: u32) -> Result<u8, Error> {
            let mut arr = [0u8; 1];
            self.heap.get_into(self.ptr_offset + ptr, &mut arr)?;
            Ok(arr[0])
        }
        fn set_heap(&mut self, ptr: u32, value: u8) -> Result<(), Error> {
            self.heap.set(self.ptr_offset + ptr, &[value])
        }
        fn set_heap_4bytes(&mut self, ptr: u32, value: [u8; 4])
         -> Result<(), Error> {
            self.heap.set(self.ptr_offset + ptr, &value)
        }
    }
}
pub mod error {
    //! Rust executor possible errors.
    #![allow(deprecated)]
    use state_machine;
    use serializer;
    use wasmi;
    use error_chain::{error_chain, error_chain_processing,
                      impl_error_chain_processed, impl_extract_backtrace,
                      impl_error_chain_kind};
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
                None => {
                    match self.0
                        {
                         #[doc = "Unserializable Data"]
                         ErrorKind::InvalidData(ref foreign_err) => {
                             foreign_err.cause()
                         }
                          #[doc = "Trap occured during execution"]
                          ErrorKind::Trap(ref foreign_err) => {
                              foreign_err.cause()
                          }
                           #[doc = "Wasmi loading/instantiating error"]
                           ErrorKind::Wasmi(ref foreign_err) => {
                               foreign_err.cause()
                           }
                        _ => None,
                    }
                }
            }
        }
    }
    impl ::std::fmt::Display for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    #[doc = "Unserializable Data"]
    impl From<serializer::Error> for Error {
        fn from(e: serializer::Error) -> Self {
            Error::from_kind(ErrorKind::InvalidData(e))
        }
    }
    #[doc = "Trap occured during execution"]
    impl From<wasmi::Trap> for Error {
        fn from(e: wasmi::Trap) -> Self {
            Error::from_kind(ErrorKind::Trap(e))
        }
    }
    #[doc = "Wasmi loading/instantiating error"]
    impl From<wasmi::Error> for Error {
        fn from(e: wasmi::Error) -> Self {
            Error::from_kind(ErrorKind::Wasmi(e))
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

        #[doc = "Unserializable Data"]
        InvalidData(serializer::Error),

        #[doc = "Trap occured during execution"]
        Trap(wasmi::Trap),

        #[doc = "Wasmi loading/instantiating error"]
        Wasmi(wasmi::Error),

        #[doc = r" Method is not found"]
        MethodNotFound(String),

        #[doc = r" Code is invalid (expected single byte)"]
        InvalidCode(Vec<u8>),

        #[doc = r" Could not get runtime version."]
        VersionInvalid,

        #[doc = r" Externalities have failed."]
        Externalities,

        #[doc = r" Invalid index."]
        InvalidIndex,

        #[doc = r" Invalid return type."]
        InvalidReturn,

        #[doc = r" Runtime failed."]
        Runtime,

        #[doc = r" Runtime failed."]
        InvalidMemoryReference,

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
                (&ErrorKind::InvalidData(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidData");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Trap(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Trap");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Wasmi(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Wasmi");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::MethodNotFound(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("MethodNotFound");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::InvalidCode(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidCode");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&ErrorKind::VersionInvalid,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("VersionInvalid");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Externalities,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("Externalities");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::InvalidIndex,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidIndex");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::InvalidReturn,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidReturn");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::Runtime,) => {
                    let mut debug_trait_builder = f.debug_tuple("Runtime");
                    debug_trait_builder.finish()
                }
                (&ErrorKind::InvalidMemoryReference,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidMemoryReference");
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
                  #[doc = "Unserializable Data"]
                  ErrorKind::InvalidData(ref err) => {
                      let display_fn =
                          |_, f: &mut ::std::fmt::Formatter|
                              {
                                  f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                            &match (&err,)
                                                                                 {
                                                                                 (arg0,)
                                                                                 =>
                                                                                 [::std::fmt::ArgumentV1::new(arg0,
                                                                                                              ::std::fmt::Display::fmt)],
                                                                             }))
                              };
                      display_fn(self, fmt)
                  }
                   #[doc = "Trap occured during execution"]
                   ErrorKind::Trap(ref err) => {
                       let display_fn =
                           |_, f: &mut ::std::fmt::Formatter|
                               {
                                   f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                             &match (&err,)
                                                                                  {
                                                                                  (arg0,)
                                                                                  =>
                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                               ::std::fmt::Display::fmt)],
                                                                              }))
                               };
                       display_fn(self, fmt)
                   }
                    #[doc = "Wasmi loading/instantiating error"]
                    ErrorKind::Wasmi(ref err) => {
                        let display_fn =
                            |_, f: &mut ::std::fmt::Formatter|
                                {
                                    f.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                              &match (&err,)
                                                                                   {
                                                                                   (arg0,)
                                                                                   =>
                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                ::std::fmt::Display::fmt)],
                                                                               }))
                                };
                        display_fn(self, fmt)
                    }
                     #[doc = r" Method is not found"]
                     ErrorKind::MethodNotFound(ref t) => {
                         let display_fn =
                             |_, f: &mut ::std::fmt::Formatter|
                                 {
                                     f.write_fmt(::std::fmt::Arguments::new_v1(&["Method not found: \'",
                                                                                 "\'"],
                                                                               &match (&t,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                }))
                                 };
                         display_fn(self, fmt)
                     }
                      #[doc = r" Code is invalid (expected single byte)"]
                      ErrorKind::InvalidCode(ref c) => {
                          let display_fn =
                              |_, f: &mut ::std::fmt::Formatter|
                                  {
                                      f.write_fmt(::std::fmt::Arguments::new_v1(&["Invalid Code: "],
                                                                                &match (&c,)
                                                                                     {
                                                                                     (arg0,)
                                                                                     =>
                                                                                     [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                  ::std::fmt::Debug::fmt)],
                                                                                 }))
                                  };
                          display_fn(self, fmt)
                      }
                       #[doc = r" Could not get runtime version."]
                       ErrorKind::VersionInvalid => {
                           let display_fn =
                               |_, f: &mut ::std::fmt::Formatter|
                                   {
                                       f.write_fmt(::std::fmt::Arguments::new_v1(&["On-chain runtime does not specify version"],
                                                                                 &match ()
                                                                                      {
                                                                                      ()
                                                                                      =>
                                                                                      [],
                                                                                  }))
                                   };
                           display_fn(self, fmt)
                       }
                        #[doc = r" Externalities have failed."]
                        ErrorKind::Externalities => {
                            let display_fn =
                                |_, f: &mut ::std::fmt::Formatter|
                                    {
                                        f.write_fmt(::std::fmt::Arguments::new_v1(&["Externalities error"],
                                                                                  &match ()
                                                                                       {
                                                                                       ()
                                                                                       =>
                                                                                       [],
                                                                                   }))
                                    };
                            display_fn(self, fmt)
                        }
                         #[doc = r" Invalid index."]
                         ErrorKind::InvalidIndex => {
                             let display_fn =
                                 |_, f: &mut ::std::fmt::Formatter|
                                     {
                                         f.write_fmt(::std::fmt::Arguments::new_v1(&["Invalid index provided"],
                                                                                   &match ()
                                                                                        {
                                                                                        ()
                                                                                        =>
                                                                                        [],
                                                                                    }))
                                     };
                             display_fn(self, fmt)
                         }
                          #[doc = r" Invalid return type."]
                          ErrorKind::InvalidReturn => {
                              let display_fn =
                                  |_, f: &mut ::std::fmt::Formatter|
                                      {
                                          f.write_fmt(::std::fmt::Arguments::new_v1(&["Invalid type returned (should be u64)"],
                                                                                    &match ()
                                                                                         {
                                                                                         ()
                                                                                         =>
                                                                                         [],
                                                                                     }))
                                      };
                              display_fn(self, fmt)
                          }
                           #[doc = r" Runtime failed."]
                           ErrorKind::Runtime => {
                               let display_fn =
                                   |_, f: &mut ::std::fmt::Formatter|
                                       {
                                           f.write_fmt(::std::fmt::Arguments::new_v1(&["Runtime error"],
                                                                                     &match ()
                                                                                          {
                                                                                          ()
                                                                                          =>
                                                                                          [],
                                                                                      }))
                                       };
                               display_fn(self, fmt)
                           }
                            #[doc = r" Runtime failed."]
                            ErrorKind::InvalidMemoryReference => {
                                let display_fn =
                                    |_, f: &mut ::std::fmt::Formatter|
                                        {
                                            f.write_fmt(::std::fmt::Arguments::new_v1(&["Invalid memory reference"],
                                                                                      &match ()
                                                                                           {
                                                                                           ()
                                                                                           =>
                                                                                           [],
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
                  #[doc = "Unserializable Data"]
                  ErrorKind::InvalidData(ref err) => {
                      ::std::error::Error::description(err)
                  }
                   #[doc = "Trap occured during execution"]
                   ErrorKind::Trap(ref err) => {
                       ::std::error::Error::description(err)
                   }
                    #[doc = "Wasmi loading/instantiating error"]
                    ErrorKind::Wasmi(ref err) => {
                        ::std::error::Error::description(err)
                    }
                     #[doc = r" Method is not found"]
                     ErrorKind::MethodNotFound(ref t) => {
                         "method not found"
                     }
                      #[doc = r" Code is invalid (expected single byte)"]
                      ErrorKind::InvalidCode(ref c) => {
                          "invalid code"
                      }
                       #[doc = r" Could not get runtime version."]
                       ErrorKind::VersionInvalid => {
                           "Runtime version error"
                       }
                        #[doc = r" Externalities have failed."]
                        ErrorKind::Externalities => {
                            "externalities failure"
                        }
                         #[doc = r" Invalid index."]
                         ErrorKind::InvalidIndex => {
                             "index given was not in range"
                         }
                          #[doc = r" Invalid return type."]
                          ErrorKind::InvalidReturn => {
                              "u64 was not returned"
                          }
                           #[doc = r" Runtime failed."]
                           ErrorKind::Runtime => {
                               "runtime failure"
                           }
                            #[doc = r" Runtime failed."]
                            ErrorKind::InvalidMemoryReference => {
                                "invalid memory reference"
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
    impl state_machine::Error for Error { }
}
pub use wasmi;
pub use wasm_executor::WasmExecutor;
pub use native_executor::{with_native_environment, NativeExecutor,
                          NativeExecutionDispatch};
pub use state_machine::Externalities;
pub use runtime_version::{RuntimeVersion, NativeVersion};
pub use parity_codec::Codec;
#[doc(hidden)]
pub use primitives::Blake2Hasher;
/// Provides runtime information.
pub trait RuntimeInfo {
    /// Native runtime information.
    fn native_version(&self)
    -> &NativeVersion;
    /// Extract RuntimeVersion of given :code block
    fn runtime_version<E: Externalities<Blake2Hasher>>(&self, ext: &mut E)
    -> Option<RuntimeVersion>;
}
