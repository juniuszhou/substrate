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

//! This crate provides means to instantiate and execute wasm modules.
//!
//! It works even when the user of this library executes from
//! inside the wasm VM. In this case the same VM is used for execution
//! of both the sandbox owner and the sandboxed module, without compromising security
//! and without the performance penalty of full wasm emulation inside wasm.
//!
//! This is achieved by using bindings to the wasm VM, which are published by the host API.
//! This API is thin and consists of only a handful functions. It contains functions for instantiating
//! modules and executing them, but doesn't contain functions for inspecting the module
//! structure. The user of this library is supposed to read the wasm module.
//!
//! When this crate is used in the `std` environment all these functions are implemented by directly
//! calling the wasm VM.
//!
//! Examples of possible use-cases for this library are not limited to the following:
//!
//! - implementing smart-contract runtimes that use wasm for contract code
//! - executing a wasm substrate runtime inside of a wasm parachain

#![warn(missing_docs)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

use rstd::prelude::*;

pub use primitives::sandbox::{TypedValue, ReturnValue, HostError};

mod imp {


















    use rstd::collections::btree_map::BTreeMap;
    use rstd::fmt;
    use wasmi::{Externals, FuncInstance, FuncRef, GlobalDescriptor, GlobalRef,
                ImportResolver, MemoryDescriptor, MemoryInstance, MemoryRef,
                Module, ModuleInstance, ModuleRef, RuntimeArgs, RuntimeValue,
                Signature, TableDescriptor, TableRef, Trap, TrapKind};
    use wasmi::memory_units::Pages;
    use super::{Error, TypedValue, ReturnValue, HostFuncType, HostError};
    pub struct Memory {
        memref: MemoryRef,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for Memory {
        #[inline]
        fn clone(&self) -> Memory {
            match *self {
                Memory { memref: ref __self_0_0 } =>
                Memory{memref: ::std::clone::Clone::clone(&(*__self_0_0)),},
            }
        }
    }
    impl Memory {
        pub fn new(initial: u32, maximum: Option<u32>)
         -> Result<Memory, Error> {
            Ok(Memory{memref:
                          MemoryInstance::alloc(Pages(initial as usize),
                                                maximum.map(|m|
                                                                Pages(m as
                                                                          usize))).map_err(|_|
                                                                                               Error::Module)?,})
        }
        pub fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<(), Error> {
            self.memref.get_into(ptr, buf).map_err(|_| Error::OutOfBounds)?;
            Ok(())
        }
        pub fn set(&self, ptr: u32, value: &[u8]) -> Result<(), Error> {
            self.memref.set(ptr, value).map_err(|_| Error::OutOfBounds)?;
            Ok(())
        }
    }
    struct HostFuncIndex(usize);
    struct DefinedHostFunctions<T> {
        funcs: Vec<HostFuncType<T>>,
    }
    impl <T> Clone for DefinedHostFunctions<T> {
        fn clone(&self) -> DefinedHostFunctions<T> {
            DefinedHostFunctions{funcs: self.funcs.clone(),}
        }
    }
    impl <T> DefinedHostFunctions<T> {
        fn new() -> DefinedHostFunctions<T> {
            DefinedHostFunctions{funcs: Vec::new(),}
        }
        fn define(&mut self, f: HostFuncType<T>) -> HostFuncIndex {
            let idx = self.funcs.len();
            self.funcs.push(f);
            HostFuncIndex(idx)
        }
    }
    struct DummyHostError;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for DummyHostError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                DummyHostError => {
                    let mut debug_trait_builder =
                        f.debug_tuple("DummyHostError");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl fmt::Display for DummyHostError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1(&["DummyHostError"],
                                                      &match () {
                                                           () => [],
                                                       }))
        }
    }
    impl wasmi::HostError for DummyHostError { }
    fn from_runtime_value(v: RuntimeValue) -> TypedValue {
        match v {
            RuntimeValue::I32(v) => TypedValue::I32(v),
            RuntimeValue::I64(v) => TypedValue::I64(v),
            RuntimeValue::F32(v) => TypedValue::F32(v.to_bits() as i32),
            RuntimeValue::F64(v) => TypedValue::F64(v.to_bits() as i64),
        }
    }
    fn to_runtime_value(v: TypedValue) -> RuntimeValue {
        use wasmi::nan_preserving_float::{F32, F64};
        match v {
            TypedValue::I32(v) => RuntimeValue::I32(v as i32),
            TypedValue::I64(v) => RuntimeValue::I64(v as i64),
            TypedValue::F32(v_bits) =>
            RuntimeValue::F32(F32::from_bits(v_bits as u32)),
            TypedValue::F64(v_bits) =>
            RuntimeValue::F64(F64::from_bits(v_bits as u64)),
        }
    }
    struct GuestExternals<'a, T: 'a> {
        state: &'a mut T,
        defined_host_functions: &'a DefinedHostFunctions<T>,
    }
    impl <'a, T> Externals for GuestExternals<'a, T> {
        fn invoke_index(&mut self, index: usize, args: RuntimeArgs)
         -> Result<Option<RuntimeValue>, Trap> {
            let args =
                args.as_ref().iter().cloned().map(from_runtime_value).collect::<Vec<_>>();
            let result =
                (self.defined_host_functions.funcs[index])(self.state, &args);
            match result {
                Ok(value) =>
                Ok(match value {
                       ReturnValue::Value(v) => Some(to_runtime_value(v)),
                       ReturnValue::Unit => None,
                   }),
                Err(HostError) =>
                Err(TrapKind::Host(Box::new(DummyHostError)).into()),
            }
        }
    }
    enum ExternVal { HostFunc(HostFuncIndex), Memory(Memory), }
    pub struct EnvironmentDefinitionBuilder<T> {
        map: BTreeMap<(Vec<u8>, Vec<u8>), ExternVal>,
        defined_host_functions: DefinedHostFunctions<T>,
    }
    impl <T> EnvironmentDefinitionBuilder<T> {
        pub fn new() -> EnvironmentDefinitionBuilder<T> {
            EnvironmentDefinitionBuilder{map: BTreeMap::new(),
                                         defined_host_functions:
                                             DefinedHostFunctions::new(),}
        }
        pub fn add_host_func<N1,
                             N2>(&mut self, module: N1, field: N2,
                                 f: HostFuncType<T>) where N1: Into<Vec<u8>>,
         N2: Into<Vec<u8>> {
            let idx = self.defined_host_functions.define(f);
            self.map.insert((module.into(), field.into()),
                            ExternVal::HostFunc(idx));
        }
        pub fn add_memory<N1,
                          N2>(&mut self, module: N1, field: N2, mem: Memory)
         where N1: Into<Vec<u8>>, N2: Into<Vec<u8>> {
            self.map.insert((module.into(), field.into()),
                            ExternVal::Memory(mem));
        }
    }
    impl <T> ImportResolver for EnvironmentDefinitionBuilder<T> {
        fn resolve_func(&self, module_name: &str, field_name: &str,
                        signature: &Signature)
         -> Result<FuncRef, wasmi::Error> {
            let key =
                (module_name.as_bytes().to_owned(),
                 field_name.as_bytes().to_owned());
            let externval =
                self.map.get(&key).ok_or_else(||
                                                  {
                                                      wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
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
            let host_func_idx =
                match *externval {
                    ExternVal::HostFunc(ref idx) => idx,
                    _ => {
                        return Err(wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                    ":",
                                                                                                                    " is not a host func"],
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
                };
            Ok(FuncInstance::alloc_host(signature.clone(), host_func_idx.0))
        }
        fn resolve_global(&self, _module_name: &str, _field_name: &str,
                          _global_type: &GlobalDescriptor)
         -> Result<GlobalRef, wasmi::Error> {
            Err(wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Importing globals is not supported yet"],
                                                                                               &match ()
                                                                                                    {
                                                                                                    ()
                                                                                                    =>
                                                                                                    [],
                                                                                                }))))
        }
        fn resolve_memory(&self, module_name: &str, field_name: &str,
                          _memory_type: &MemoryDescriptor)
         -> Result<MemoryRef, wasmi::Error> {
            let key =
                (module_name.as_bytes().to_owned(),
                 field_name.as_bytes().to_owned());
            let externval =
                self.map.get(&key).ok_or_else(||
                                                  {
                                                      wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
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
            let memory =
                match *externval {
                    ExternVal::Memory(ref m) => m,
                    _ => {
                        return Err(wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Export ",
                                                                                                                    ":",
                                                                                                                    " is not a memory"],
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
                };
            Ok(memory.memref.clone())
        }
        fn resolve_table(&self, _module_name: &str, _field_name: &str,
                         _table_type: &TableDescriptor)
         -> Result<TableRef, wasmi::Error> {
            Err(wasmi::Error::Instantiation(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Importing tables is not supported yet"],
                                                                                               &match ()
                                                                                                    {
                                                                                                    ()
                                                                                                    =>
                                                                                                    [],
                                                                                                }))))
        }
    }
    pub struct Instance<T> {
        instance: ModuleRef,
        defined_host_functions: DefinedHostFunctions<T>,
        _marker: ::std::marker::PhantomData<T>,
    }
    impl <T> Instance<T> {
        pub fn new(code: &[u8],
                   env_def_builder: &EnvironmentDefinitionBuilder<T>,
                   state: &mut T) -> Result<Instance<T>, Error> {
            let module =
                Module::from_buffer(code).map_err(|_| Error::Module)?;
            let not_started_instance =
                ModuleInstance::new(&module,
                                    env_def_builder).map_err(|_|
                                                                 Error::Module)?;
            let defined_host_functions =
                env_def_builder.defined_host_functions.clone();
            let instance =
                {
                    let mut externals =
                        GuestExternals{state,
                                       defined_host_functions:
                                           &defined_host_functions,};
                    let instance =
                        not_started_instance.run_start(&mut externals).map_err(|_|
                                                                                   Error::Execution)?;
                    instance
                };
            Ok(Instance{instance,
                        defined_host_functions,
                        _marker: ::std::marker::PhantomData::<T>,})
        }
        pub fn invoke(&mut self, name: &[u8], args: &[TypedValue],
                      state: &mut T) -> Result<ReturnValue, Error> {
            let args =
                args.iter().cloned().map(Into::into).collect::<Vec<_>>();
            let name =
                ::std::str::from_utf8(name).map_err(|_| Error::Execution)?;
            let mut externals =
                GuestExternals{state,
                               defined_host_functions:
                                   &self.defined_host_functions,};
            let result =
                self.instance.invoke_export(&name, &args, &mut externals);
            match result {
                Ok(None) => Ok(ReturnValue::Unit),
                Ok(Some(val)) => Ok(ReturnValue::Value(val.into())),
                Err(_err) => Err(Error::Execution),
            }
        }
    }
}
/// Error that can occur while using this crate.
pub enum Error {

    /// Module is not valid, couldn't be instantiated or it's `start` function trapped
    /// when executed.
    Module,

    /// Access to a memory or table was made with an address or an index which is out of bounds.
    ///
    /// Note that if wasm module makes an out-of-bounds access then trap will occur.
    OutOfBounds,

    /// Failed to invoke an exported function for some reason.
    Execution,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Error::Module,) => {
                let mut debug_trait_builder = f.debug_tuple("Module");
                debug_trait_builder.finish()
            }
            (&Error::OutOfBounds,) => {
                let mut debug_trait_builder = f.debug_tuple("OutOfBounds");
                debug_trait_builder.finish()
            }
            (&Error::Execution,) => {
                let mut debug_trait_builder = f.debug_tuple("Execution");
                debug_trait_builder.finish()
            }
        }
    }
}
impl From<Error> for HostError {
    fn from(_e: Error) -> HostError { HostError }
}
/// Function pointer for specifying functions by the
/// supervisor in [`EnvironmentDefinitionBuilder`].
///
/// [`EnvironmentDefinitionBuilder`]: struct.EnvironmentDefinitionBuilder.html
pub type HostFuncType<T>
    =
    fn(&mut T, &[TypedValue]) -> Result<ReturnValue, HostError>;
/// Reference to a sandboxed linear memory, that
/// will be used by the guest module.
///
/// The memory can't be directly accessed by supervisor, but only
/// through designated functions [`get`] and [`set`].
///
/// [`get`]: #method.get
/// [`set`]: #method.set
pub struct Memory {
    inner: imp::Memory,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Memory {
    #[inline]
    fn clone(&self) -> Memory {
        match *self {
            Memory { inner: ref __self_0_0 } =>
            Memory{inner: ::std::clone::Clone::clone(&(*__self_0_0)),},
        }
    }
}
impl Memory {
    /// Construct a new linear memory instance.
    ///
    /// The memory allocated with initial number of pages specified by `initial`.
    /// Minimal possible value for `initial` is 0 and maximum possible is `65536`.
    /// (Since maximum addressible memory is 2<sup>32</sup> = 4GiB = 65536 * 64KiB).
    ///
    /// It is possible to limit maximum number of pages this memory instance can have by specifying
    /// `maximum`. If not specified, this memory instance would be able to allocate up to 4GiB.
    ///
    /// Allocated memory is always zeroed.
    pub fn new(initial: u32, maximum: Option<u32>) -> Result<Memory, Error> {
        Ok(Memory{inner: imp::Memory::new(initial, maximum)?,})
    }
    /// Read a memory area at the address `ptr` with the size of the provided slice `buf`.
    ///
    /// Returns `Err` if the range is out-of-bounds.
    pub fn get(&self, ptr: u32, buf: &mut [u8]) -> Result<(), Error> {
        self.inner.get(ptr, buf)
    }
    /// Write a memory area at the address `ptr` with contents of the provided slice `buf`.
    ///
    /// Returns `Err` if the range is out-of-bounds.
    pub fn set(&self, ptr: u32, value: &[u8]) -> Result<(), Error> {
        self.inner.set(ptr, value)
    }
}
/// Struct that can be used for defining an environment for a sandboxed module.
///
/// The sandboxed module can access only the entities which were defined and passed
/// to the module at the instantiation time.
pub struct EnvironmentDefinitionBuilder<T> {
    inner: imp::EnvironmentDefinitionBuilder<T>,
}
impl <T> EnvironmentDefinitionBuilder<T> {
    /// Construct a new `EnvironmentDefinitionBuilder`.
    pub fn new() -> EnvironmentDefinitionBuilder<T> {
        EnvironmentDefinitionBuilder{inner:
                                         imp::EnvironmentDefinitionBuilder::new(),}
    }
    /// Register a host function in this environment definition.
    ///
    /// NOTE that there is no constraints on type of this function. An instance
    /// can import function passed here with any signature it wants. It can even import
    /// the same function (i.e. with same `module` and `field`) several times. It's up to
    /// the user code to check or constrain the types of signatures.
    pub fn add_host_func<N1,
                         N2>(&mut self, module: N1, field: N2,
                             f: HostFuncType<T>) where N1: Into<Vec<u8>>,
     N2: Into<Vec<u8>> {
        self.inner.add_host_func(module, field, f);
    }
    /// Register a memory in this environment definition.
    pub fn add_memory<N1, N2>(&mut self, module: N1, field: N2, mem: Memory)
     where N1: Into<Vec<u8>>, N2: Into<Vec<u8>> {
        self.inner.add_memory(module, field, mem.inner);
    }
}
/// Sandboxed instance of a wasm module.
///
/// This instance can be used for invoking exported functions.
pub struct Instance<T> {
    inner: imp::Instance<T>,
}
impl <T> Instance<T> {
    /// Instantiate a module with the given [`EnvironmentDefinitionBuilder`]. It will
    /// run the `start` function with the given `state`.
    ///
    /// Returns `Err(Error::Module)` if this module can't be instantiated with the given
    /// environment. If execution of `start` function generated a trap, then `Err(Error::Execution)` will
    /// be returned.
    ///
    /// [`EnvironmentDefinitionBuilder`]: struct.EnvironmentDefinitionBuilder.html
    pub fn new(code: &[u8], env_def_builder: &EnvironmentDefinitionBuilder<T>,
               state: &mut T) -> Result<Instance<T>, Error> {
        Ok(Instance{inner:
                        imp::Instance::new(code, &env_def_builder.inner,
                                           state)?,})
    }
    /// Invoke an exported function with the given name.
    ///
    /// # Errors
    ///
    /// Returns `Err(Error::Execution)` if:
    ///
    /// - An export function name isn't a proper utf8 byte sequence,
    /// - This module doesn't have an exported function with the given name,
    /// - If types of the arguments passed to the function doesn't match function signature
    ///   then trap occurs (as if the exported function was called via call_indirect),
    /// - Trap occured at the execution time.
    pub fn invoke(&mut self, name: &[u8], args: &[TypedValue], state: &mut T)
     -> Result<ReturnValue, Error> {
        self.inner.invoke(name, args, state)
    }
}
