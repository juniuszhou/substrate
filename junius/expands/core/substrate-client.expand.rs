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

//! Substrate Client and associated logic.

#![warn(missing_docs)]
#![recursion_limit = "128"]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

#[macro_use]
pub mod runtime_api {



    //! All the functionality required for declaring and implementing runtime apis.
    #[doc(hidden)]
    #[cfg(feature = "std")]
    pub use state_machine::OverlayedChanges;
    #[doc(hidden)]
    #[cfg(feature = "std")]
    pub use primitives::NativeOrEncoded;
    #[doc(hidden)]
    pub use runtime_primitives::{traits::{AuthorityIdFor, Block as BlockT,
                                          GetNodeBlockType,
                                          GetRuntimeBlockType, Header as
                                          HeaderT, ApiRef, RuntimeApiInfo,
                                          Hash as HashT}, generic::BlockId,
                                 transaction_validity::TransactionValidity};
    #[doc(hidden)]
    pub use primitives::{ExecutionContext, OffchainExt};
    #[doc(hidden)]
    pub use runtime_version::{ApiId, RuntimeVersion, ApisVec,
                              create_apis_vec};
    #[doc(hidden)]
    pub use rstd::{slice, mem};
    #[cfg(feature = "std")]
    use rstd::result;
    #[doc(hidden)]
    pub use parity_codec::{Encode, Decode};
    #[cfg(feature = "std")]
    use crate::error;
    use sr_api_macros::decl_runtime_apis;
    use primitives::OpaqueMetadata;
    #[cfg(feature = "std")]
    use std::{panic::UnwindSafe, cell::RefCell, rc::Rc};
    use rstd::vec::Vec;
    #[cfg(feature = "std")]
    use primitives::Hasher as HasherT;
    #[cfg(feature = "std")]
    /// A type that records all accessed trie nodes and generates a proof out of it.
    pub type ProofRecorder<B>
        =
        state_machine::ProofRecorder<<<<<B as BlockT>::Header as
                                       HeaderT>::Hashing as HashT>::Hasher as
                                     HasherT>::Out>;
    /// Something that can be constructed to a runtime api.
    #[cfg(feature = "std")]
    pub trait ConstructRuntimeApi<Block: BlockT, C: CallRuntimeAt<Block>> {
        /// The actual runtime api that will be constructed.
        type
        RuntimeApi;
        /// Construct an instance of the runtime api.
        fn construct_runtime_api<'a>(call: &'a C)
        -> ApiRef<'a, Self::RuntimeApi>;
    }
    /// An extension for the `RuntimeApi`.
    #[cfg(feature = "std")]
    pub trait ApiExt<Block: BlockT> {
        /// The given closure will be called with api instance. Inside the closure any api call is
        /// allowed. After doing the api call, the closure is allowed to map the `Result` to a
        /// different `Result` type. This can be important, as the internal data structure that keeps
        /// track of modifications to the storage, discards changes when the `Result` is an `Err`.
        /// On `Ok`, the structure commits the changes to an internal buffer.
        fn map_api_result<F: FnOnce(&Self) -> result::Result<R, E>, R,
                          E>(&self, map_call: F)
        -> result::Result<R, E>
        where
        Self: Sized;
        /// Checks if the given api is implemented and versions match.
        fn has_api<A: RuntimeApiInfo + ?Sized>(&self, at: &BlockId<Block>)
         -> error::Result<bool> where Self: Sized {
            self.runtime_version_at(at).map(|v| v.has_api::<A>())
        }
        /// Check if the given api is implemented and the version passes a predicate.
        fn has_api_with<A: RuntimeApiInfo + ?Sized, P: Fn(u32) ->
                        bool>(&self, at: &BlockId<Block>, pred: P)
         -> error::Result<bool> where Self: Sized {
            self.runtime_version_at(at).map(|v| v.has_api_with::<A, _>(pred))
        }
        /// Returns the runtime version at the given block id.
        fn runtime_version_at(&self, at: &BlockId<Block>)
        -> error::Result<RuntimeVersion>;
        /// Start recording all accessed trie nodes for generating proofs.
        fn record_proof(&mut self);
        /// Extract the recorded proof.
        /// This stops the proof recording.
        fn extract_proof(&mut self)
        -> Option<Vec<Vec<u8>>>;
    }
    /// Before calling any runtime api function, the runtime need to be initialized
    /// at the requested block. However, some functions like `execute_block` or
    /// `initialize_block` itself don't require to have the runtime initialized
    /// at the requested block.
    ///
    /// `call_api_at` is instructed by this enum to do the initialization or to skip
    /// it.
    #[cfg(feature = "std")]
    #[rustc_copy_clone_marker]
    pub enum InitializeBlock<'a, Block: BlockT> {

        /// Skip initializing the runtime for a given block.
        ///
        /// This is used by functions who do the initialization by themself or don't
        /// require it.
        Skip,

        /// Initialize the runtime for a given block.
        ///
        /// If the stored `BlockId` is `Some(_)`, the runtime is currently initialized
        /// at this block.
        Do(&'a RefCell<Option<BlockId<Block>>>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a, Block: ::std::clone::Clone + BlockT> ::std::clone::Clone for
     InitializeBlock<'a, Block> {
        #[inline]
        fn clone(&self) -> InitializeBlock<'a, Block> {
            match (&*self,) {
                (&InitializeBlock::Skip,) => InitializeBlock::Skip,
                (&InitializeBlock::Do(ref __self_0),) =>
                InitializeBlock::Do(::std::clone::Clone::clone(&(*__self_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a, Block: ::std::marker::Copy + BlockT> ::std::marker::Copy for
     InitializeBlock<'a, Block> {
    }
    /// Something that can call into the runtime at a given block.
    #[cfg(feature = "std")]
    pub trait CallRuntimeAt<Block: BlockT> {
        /// Calls the given api function with the given encoded arguments at the given block
        /// and returns the encoded result.
        fn call_api_at<'a, R: Encode + Decode + PartialEq, NC: FnOnce() ->
                       result::Result<R, &'static str> + UnwindSafe,
                       C: Core<Block>>(&self, core_api: &C,
                                       at: &BlockId<Block>,
                                       function: &'static str, args: Vec<u8>,
                                       changes: &RefCell<OverlayedChanges>,
                                       initialize_block:
                                           InitializeBlock<'a, Block>,
                                       native_call: Option<NC>,
                                       context: ExecutionContext,
                                       recorder:
                                           &Option<Rc<RefCell<ProofRecorder<Block>>>>)
        -> error::Result<NativeOrEncoded<R>>;
        /// Returns the runtime version at the given block.
        fn runtime_version_at(&self, at: &BlockId<Block>)
        -> error::Result<RuntimeVersion>;
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[allow(deprecated)]
    pub mod runtime_decl_for_Core {
        use super::*;
        #[doc = " The `Core` api trait that is mandatory for each runtime."]
        pub trait Core<Block: crate::runtime_api::BlockT> {
            #[doc = " Returns the version of the runtime."]
            fn version()
            -> RuntimeVersion;
            #[doc = " Execute the given block."]
            fn execute_block(block: Block);
            #[doc = " Initialize a block with the given header."]
            fn initialize_block(header: &<Block as BlockT>::Header);
            #[doc = " Returns the authorities."]
            #[deprecated(since = "1.0",
                         note = "Please switch to `AuthoritiesApi`.")]
            fn authorities()
            -> Vec<AuthorityIdFor<Block>>;
        }
        pub const VERSION: u32 = 2u32;
        pub const ID: [u8; 8] =
            [223u8, 106u8, 203u8, 104u8, 153u8, 7u8, 96u8, 155u8];
        #[cfg(any(feature = "std", test))]
        fn convert_between_block_types<I: crate::runtime_api::Encode,
                                       R: crate::runtime_api::Decode>(input:
                                                                          &I,
                                                                      error_desc:
                                                                          &'static str)
         -> ::std::result::Result<R, &'static str> {
            <R as
                crate::runtime_api::Decode>::decode(&mut &crate::runtime_api::Encode::encode(input)[..]).ok_or_else(||
                                                                                                                        error_desc)
        }
        #[cfg(any(feature = "std", test))]
        pub fn version_native_call_generator<'a, ApiImpl: Core<Block>,
                                             NodeBlock: crate::runtime_api::BlockT,
                                             Block: crate::runtime_api::BlockT +
                                             'a>()
         ->
             impl FnOnce() ->
             ::std::result::Result<RuntimeVersion, &'static str> + 'a {
            move || { let res = ApiImpl::version(); Ok(res) }
        }
        #[cfg(any(feature = "std", test))]
        pub fn execute_block_native_call_generator<'a, ApiImpl: Core<Block>,
                                                   NodeBlock: crate::runtime_api::BlockT,
                                                   Block: crate::runtime_api::BlockT +
                                                   'a>(block: NodeBlock)
         -> impl FnOnce() -> ::std::result::Result<(), &'static str> + 'a {
            move ||
                {
                    let block: Block =
                        convert_between_block_types(&block,
                                                    "Could not convert parameter `block` between node and runtime!")?;
                    let res = ApiImpl::execute_block(block);
                    Ok(res)
                }
        }
        #[cfg(any(feature = "std", test))]
        pub fn initialize_block_native_call_generator<'a,
                                                      ApiImpl: Core<Block>,
                                                      NodeBlock: crate::runtime_api::BlockT,
                                                      Block: crate::runtime_api::BlockT +
                                                      'a>(header:
                                                              &'a <NodeBlock
                                                                  as
                                                                  BlockT>::Header)
         -> impl FnOnce() -> ::std::result::Result<(), &'static str> + 'a {
            move ||
                {
                    let header: <Block as BlockT>::Header =
                        convert_between_block_types(&header,
                                                    "Could not convert parameter `header` between node and runtime!")?;
                    let res = ApiImpl::initialize_block(&header);
                    Ok(res)
                }
        }
        #[cfg(any(feature = "std", test))]
        pub fn authorities_native_call_generator<'a, ApiImpl: Core<Block>,
                                                 NodeBlock: crate::runtime_api::BlockT,
                                                 Block: crate::runtime_api::BlockT +
                                                 'a>()
         ->
             impl FnOnce() ->
             ::std::result::Result<Vec<AuthorityIdFor<NodeBlock>>,
                                   &'static str> + 'a {
            move ||
                {
                    let res = ApiImpl::authorities();
                    convert_between_block_types(&res,
                                                "Could not convert return value from runtime to node!")
                }
        }
        #[cfg(any(feature = "std", test))]
        pub fn version_call_api_at<R: crate::runtime_api::Encode +
                                   crate::runtime_api::Decode + PartialEq,
                                   NC: FnOnce() ->
                                   ::std::result::Result<R, &'static str> +
                                   ::std::panic::UnwindSafe,
                                   Block: crate::runtime_api::BlockT,
                                   T: crate::runtime_api::CallRuntimeAt<Block>,
                                   C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                           &T,
                                                                       core_api:
                                                                           &C,
                                                                       at:
                                                                           &crate::runtime_api::BlockId<Block>,
                                                                       args:
                                                                           Vec<u8>,
                                                                       changes:
                                                                           &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                       initialized_block:
                                                                           &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                       native_call:
                                                                           Option<NC>,
                                                                       context:
                                                                           crate::runtime_api::ExecutionContext,
                                                                       recorder:
                                                                           &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
         -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
            let version = call_runtime_at.runtime_version_at(at)?;
            use crate::runtime_api::InitializeBlock;
            let initialize_block =
                if false {
                    InitializeBlock::Skip
                } else { InitializeBlock::Do(&initialized_block) };
            let update_initialized_block = || ();
            let ret =
                call_runtime_at.call_api_at(core_api, at, "Core_version",
                                            args, changes, initialize_block,
                                            native_call, context, recorder)?;
            update_initialized_block();
            Ok(ret)
        }
        #[cfg(any(feature = "std", test))]
        pub fn execute_block_call_api_at<R: crate::runtime_api::Encode +
                                         crate::runtime_api::Decode +
                                         PartialEq, NC: FnOnce() ->
                                         ::std::result::Result<R,
                                                               &'static str> +
                                         ::std::panic::UnwindSafe,
                                         Block: crate::runtime_api::BlockT,
                                         T: crate::runtime_api::CallRuntimeAt<Block>,
                                         C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                                 &T,
                                                                             core_api:
                                                                                 &C,
                                                                             at:
                                                                                 &crate::runtime_api::BlockId<Block>,
                                                                             args:
                                                                                 Vec<u8>,
                                                                             changes:
                                                                                 &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                             initialized_block:
                                                                                 &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                             native_call:
                                                                                 Option<NC>,
                                                                             context:
                                                                                 crate::runtime_api::ExecutionContext,
                                                                             recorder:
                                                                                 &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
         -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
            let version = call_runtime_at.runtime_version_at(at)?;
            use crate::runtime_api::InitializeBlock;
            let initialize_block =
                if true {
                    InitializeBlock::Skip
                } else { InitializeBlock::Do(&initialized_block) };
            let update_initialized_block = || ();
            let ret =
                call_runtime_at.call_api_at(core_api, at,
                                            "Core_execute_block", args,
                                            changes, initialize_block,
                                            native_call, context, recorder)?;
            update_initialized_block();
            Ok(ret)
        }
        #[cfg(any(feature = "std", test))]
        pub fn initialize_block_call_api_at<R: crate::runtime_api::Encode +
                                            crate::runtime_api::Decode +
                                            PartialEq, NC: FnOnce() ->
                                            ::std::result::Result<R,
                                                                  &'static str> +
                                            ::std::panic::UnwindSafe,
                                            Block: crate::runtime_api::BlockT,
                                            T: crate::runtime_api::CallRuntimeAt<Block>,
                                            C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                                    &T,
                                                                                core_api:
                                                                                    &C,
                                                                                at:
                                                                                    &crate::runtime_api::BlockId<Block>,
                                                                                args:
                                                                                    Vec<u8>,
                                                                                changes:
                                                                                    &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                                initialized_block:
                                                                                    &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                                native_call:
                                                                                    Option<NC>,
                                                                                context:
                                                                                    crate::runtime_api::ExecutionContext,
                                                                                recorder:
                                                                                    &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
         -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
            let version = call_runtime_at.runtime_version_at(at)?;
            use crate::runtime_api::InitializeBlock;
            let initialize_block =
                if true {
                    InitializeBlock::Skip
                } else { InitializeBlock::Do(&initialized_block) };
            let update_initialized_block =
                || *initialized_block.borrow_mut() = Some(*at);
            if version.apis.iter().any(|(s, v)| { s == &ID && *v < 2u32 }) {
                let ret =
                    call_runtime_at.call_api_at::<R, fn() -> _,
                                                  _>(core_api, at,
                                                     "Core_initialise_block",
                                                     args, changes,
                                                     initialize_block, None,
                                                     context, recorder)?;
                update_initialized_block();
                return Ok(ret);
            }
            let ret =
                call_runtime_at.call_api_at(core_api, at,
                                            "Core_initialize_block", args,
                                            changes, initialize_block,
                                            native_call, context, recorder)?;
            update_initialized_block();
            Ok(ret)
        }
        #[cfg(any(feature = "std", test))]
        pub fn authorities_call_api_at<R: crate::runtime_api::Encode +
                                       crate::runtime_api::Decode + PartialEq,
                                       NC: FnOnce() ->
                                       ::std::result::Result<R,
                                                             &'static str> +
                                       ::std::panic::UnwindSafe,
                                       Block: crate::runtime_api::BlockT,
                                       T: crate::runtime_api::CallRuntimeAt<Block>,
                                       C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                               &T,
                                                                           core_api:
                                                                               &C,
                                                                           at:
                                                                               &crate::runtime_api::BlockId<Block>,
                                                                           args:
                                                                               Vec<u8>,
                                                                           changes:
                                                                               &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                           initialized_block:
                                                                               &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                           native_call:
                                                                               Option<NC>,
                                                                           context:
                                                                               crate::runtime_api::ExecutionContext,
                                                                           recorder:
                                                                               &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
         -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
            let version = call_runtime_at.runtime_version_at(at)?;
            use crate::runtime_api::InitializeBlock;
            let initialize_block =
                if false {
                    InitializeBlock::Skip
                } else { InitializeBlock::Do(&initialized_block) };
            let update_initialized_block = || ();
            let ret =
                call_runtime_at.call_api_at(core_api, at, "Core_authorities",
                                            args, changes, initialize_block,
                                            native_call, context, recorder)?;
            update_initialized_block();
            Ok(ret)
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[allow(deprecated)]
    pub mod runtime_decl_for_Metadata {
        use super::*;
        #[doc =
              " The `Metadata` api trait that returns metadata for the runtime."]
        pub trait Metadata<Block: crate::runtime_api::BlockT> {
            #[doc = " Returns the metadata of a runtime."]
            fn metadata()
            -> OpaqueMetadata;
        }
        pub const VERSION: u32 = 1u32;
        pub const ID: [u8; 8] =
            [55u8, 227u8, 151u8, 252u8, 124u8, 145u8, 245u8, 228u8];
        #[cfg(any(feature = "std", test))]
        fn convert_between_block_types<I: crate::runtime_api::Encode,
                                       R: crate::runtime_api::Decode>(input:
                                                                          &I,
                                                                      error_desc:
                                                                          &'static str)
         -> ::std::result::Result<R, &'static str> {
            <R as
                crate::runtime_api::Decode>::decode(&mut &crate::runtime_api::Encode::encode(input)[..]).ok_or_else(||
                                                                                                                        error_desc)
        }
        #[cfg(any(feature = "std", test))]
        pub fn metadata_native_call_generator<'a, ApiImpl: Metadata<Block>,
                                              NodeBlock: crate::runtime_api::BlockT,
                                              Block: crate::runtime_api::BlockT +
                                              'a>()
         ->
             impl FnOnce() ->
             ::std::result::Result<OpaqueMetadata, &'static str> + 'a {
            move || { let res = ApiImpl::metadata(); Ok(res) }
        }
        #[cfg(any(feature = "std", test))]
        pub fn metadata_call_api_at<R: crate::runtime_api::Encode +
                                    crate::runtime_api::Decode + PartialEq,
                                    NC: FnOnce() ->
                                    ::std::result::Result<R, &'static str> +
                                    ::std::panic::UnwindSafe,
                                    Block: crate::runtime_api::BlockT,
                                    T: crate::runtime_api::CallRuntimeAt<Block>,
                                    C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                            &T,
                                                                        core_api:
                                                                            &C,
                                                                        at:
                                                                            &crate::runtime_api::BlockId<Block>,
                                                                        args:
                                                                            Vec<u8>,
                                                                        changes:
                                                                            &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                        initialized_block:
                                                                            &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                        native_call:
                                                                            Option<NC>,
                                                                        context:
                                                                            crate::runtime_api::ExecutionContext,
                                                                        recorder:
                                                                            &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
         -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
            let version = call_runtime_at.runtime_version_at(at)?;
            use crate::runtime_api::InitializeBlock;
            let initialize_block =
                if false {
                    InitializeBlock::Skip
                } else { InitializeBlock::Do(&initialized_block) };
            let update_initialized_block = || ();
            let ret =
                call_runtime_at.call_api_at(core_api, at, "Metadata_metadata",
                                            args, changes, initialize_block,
                                            native_call, context, recorder)?;
            update_initialized_block();
            Ok(ret)
        }
    }
    #[doc(hidden)]
    #[allow(dead_code)]
    #[allow(deprecated)]
    pub mod runtime_decl_for_TaggedTransactionQueue {
        use super::*;
        #[doc =
              " The `TaggedTransactionQueue` api trait for interfering with the new transaction queue."]
        pub trait TaggedTransactionQueue<Block: crate::runtime_api::BlockT> {
            #[doc = " Validate the given transaction."]
            fn validate_transaction(tx: <Block as BlockT>::Extrinsic)
            -> TransactionValidity;
        }
        pub const VERSION: u32 = 1u32;
        pub const ID: [u8; 8] =
            [210u8, 188u8, 152u8, 151u8, 238u8, 208u8, 143u8, 21u8];
        #[cfg(any(feature = "std", test))]
        fn convert_between_block_types<I: crate::runtime_api::Encode,
                                       R: crate::runtime_api::Decode>(input:
                                                                          &I,
                                                                      error_desc:
                                                                          &'static str)
         -> ::std::result::Result<R, &'static str> {
            <R as
                crate::runtime_api::Decode>::decode(&mut &crate::runtime_api::Encode::encode(input)[..]).ok_or_else(||
                                                                                                                        error_desc)
        }
        #[cfg(any(feature = "std", test))]
        pub fn validate_transaction_native_call_generator<'a,
                                                          ApiImpl: TaggedTransactionQueue<Block>,
                                                          NodeBlock: crate::runtime_api::BlockT,
                                                          Block: crate::runtime_api::BlockT +
                                                          'a>(tx:
                                                                  <NodeBlock
                                                                  as
                                                                  BlockT>::Extrinsic)
         ->
             impl FnOnce() ->
             ::std::result::Result<TransactionValidity, &'static str> + 'a {
            move ||
                {
                    let tx: <Block as BlockT>::Extrinsic =
                        convert_between_block_types(&tx,
                                                    "Could not convert parameter `tx` between node and runtime!")?;
                    let res = ApiImpl::validate_transaction(tx);
                    Ok(res)
                }
        }
        #[cfg(any(feature = "std", test))]
        pub fn validate_transaction_call_api_at<R: crate::runtime_api::Encode +
                                                crate::runtime_api::Decode +
                                                PartialEq, NC: FnOnce() ->
                                                ::std::result::Result<R,
                                                                      &'static str> +
                                                ::std::panic::UnwindSafe,
                                                Block: crate::runtime_api::BlockT,
                                                T: crate::runtime_api::CallRuntimeAt<Block>,
                                                C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                                        &T,
                                                                                    core_api:
                                                                                        &C,
                                                                                    at:
                                                                                        &crate::runtime_api::BlockId<Block>,
                                                                                    args:
                                                                                        Vec<u8>,
                                                                                    changes:
                                                                                        &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                                    initialized_block:
                                                                                        &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                                    native_call:
                                                                                        Option<NC>,
                                                                                    context:
                                                                                        crate::runtime_api::ExecutionContext,
                                                                                    recorder:
                                                                                        &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
         -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
            let version = call_runtime_at.runtime_version_at(at)?;
            use crate::runtime_api::InitializeBlock;
            let initialize_block =
                if false {
                    InitializeBlock::Skip
                } else { InitializeBlock::Do(&initialized_block) };
            let update_initialized_block = || ();
            let ret =
                call_runtime_at.call_api_at(core_api, at,
                                            "TaggedTransactionQueue_validate_transaction",
                                            args, changes, initialize_block,
                                            native_call, context, recorder)?;
            update_initialized_block();
            Ok(ret)
        }
    }
    #[doc = " The `Core` api trait that is mandatory for each runtime."]
    #[cfg(any(feature = "std", test))]
    pub trait Core<Block: crate::runtime_api::BlockT>: 'static + Send + Sync +
     crate::runtime_api::ApiExt<Block> {
        #[doc = " Returns the version of the runtime."]
        fn version(&self, at: &crate::runtime_api::BlockId<Block>)
         -> ::std::result::Result<RuntimeVersion, crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&());
            self.Core_version_runtime_api_impl(at,
                                               crate::runtime_api::ExecutionContext::Other,
                                               Some(()),
                                               runtime_api_impl_params_encoded).and_then(|r|
                                                                                             match r
                                                                                                 {
                                                                                                 crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                 =>
                                                                                                 {
                                                                                                     Ok(n)
                                                                                                 }
                                                                                                 crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                 =>
                                                                                                 {
                                                                                                     <RuntimeVersion
                                                                                                         as
                                                                                                         crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                         crate::error::Error::CallResultDecode("version").into())
                                                                                                 }
                                                                                             })
        }
        #[doc = " Returns the version of the runtime."]
        fn version_with_context(&self,
                                at: &crate::runtime_api::BlockId<Block>,
                                context: crate::runtime_api::ExecutionContext)
         -> ::std::result::Result<RuntimeVersion, crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&());
            self.Core_version_runtime_api_impl(at, context, Some(()),
                                               runtime_api_impl_params_encoded).and_then(|r|
                                                                                             match r
                                                                                                 {
                                                                                                 crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                 =>
                                                                                                 {
                                                                                                     Ok(n)
                                                                                                 }
                                                                                                 crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                 =>
                                                                                                 {
                                                                                                     <RuntimeVersion
                                                                                                         as
                                                                                                         crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                         crate::error::Error::CallResultDecode("version").into())
                                                                                                 }
                                                                                             })
        }
        #[doc(hidden)]
        fn Core_version_runtime_api_impl(&self,
                                         at:
                                             &crate::runtime_api::BlockId<Block>,
                                         context:
                                             crate::runtime_api::ExecutionContext,
                                         params: Option<()>,
                                         params_encoded: Vec<u8>)
        ->
            crate::error::Result<crate::runtime_api::NativeOrEncoded<RuntimeVersion>>;
        #[doc = " Execute the given block."]
        fn execute_block(&self, at: &crate::runtime_api::BlockId<Block>,
                         block: Block)
         -> ::std::result::Result<(), crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&(&block));
            self.Core_execute_block_runtime_api_impl(at,
                                                     crate::runtime_api::ExecutionContext::Other,
                                                     Some((block)),
                                                     runtime_api_impl_params_encoded).and_then(|r|
                                                                                                   match r
                                                                                                       {
                                                                                                       crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                       =>
                                                                                                       {
                                                                                                           Ok(n)
                                                                                                       }
                                                                                                       crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                       =>
                                                                                                       {
                                                                                                           <()
                                                                                                               as
                                                                                                               crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                               crate::error::Error::CallResultDecode("execute_block").into())
                                                                                                       }
                                                                                                   })
        }
        #[doc = " Execute the given block."]
        fn execute_block_with_context(&self,
                                      at: &crate::runtime_api::BlockId<Block>,
                                      context:
                                          crate::runtime_api::ExecutionContext,
                                      block: Block)
         -> ::std::result::Result<(), crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&(&block));
            self.Core_execute_block_runtime_api_impl(at, context,
                                                     Some((block)),
                                                     runtime_api_impl_params_encoded).and_then(|r|
                                                                                                   match r
                                                                                                       {
                                                                                                       crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                       =>
                                                                                                       {
                                                                                                           Ok(n)
                                                                                                       }
                                                                                                       crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                       =>
                                                                                                       {
                                                                                                           <()
                                                                                                               as
                                                                                                               crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                               crate::error::Error::CallResultDecode("execute_block").into())
                                                                                                       }
                                                                                                   })
        }
        #[doc(hidden)]
        fn Core_execute_block_runtime_api_impl(&self,
                                               at:
                                                   &crate::runtime_api::BlockId<Block>,
                                               context:
                                                   crate::runtime_api::ExecutionContext,
                                               params: Option<(Block)>,
                                               params_encoded: Vec<u8>)
        -> crate::error::Result<crate::runtime_api::NativeOrEncoded<()>>;
        #[doc = " Initialize a block with the given header."]
        fn initialize_block(&self, at: &crate::runtime_api::BlockId<Block>,
                            header: &<Block as BlockT>::Header)
         -> ::std::result::Result<(), crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&(&header));
            self.Core_initialize_block_runtime_api_impl(at,
                                                        crate::runtime_api::ExecutionContext::Other,
                                                        Some((header)),
                                                        runtime_api_impl_params_encoded).and_then(|r|
                                                                                                      match r
                                                                                                          {
                                                                                                          crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                          =>
                                                                                                          {
                                                                                                              Ok(n)
                                                                                                          }
                                                                                                          crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                          =>
                                                                                                          {
                                                                                                              <()
                                                                                                                  as
                                                                                                                  crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                  crate::error::Error::CallResultDecode("initialize_block").into())
                                                                                                          }
                                                                                                      })
        }
        #[doc = " Initialize a block with the given header."]
        fn initialize_block_with_context(&self,
                                         at:
                                             &crate::runtime_api::BlockId<Block>,
                                         context:
                                             crate::runtime_api::ExecutionContext,
                                         header: &<Block as BlockT>::Header)
         -> ::std::result::Result<(), crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&(&header));
            self.Core_initialize_block_runtime_api_impl(at, context,
                                                        Some((header)),
                                                        runtime_api_impl_params_encoded).and_then(|r|
                                                                                                      match r
                                                                                                          {
                                                                                                          crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                          =>
                                                                                                          {
                                                                                                              Ok(n)
                                                                                                          }
                                                                                                          crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                          =>
                                                                                                          {
                                                                                                              <()
                                                                                                                  as
                                                                                                                  crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                  crate::error::Error::CallResultDecode("initialize_block").into())
                                                                                                          }
                                                                                                      })
        }
        #[doc(hidden)]
        fn Core_initialize_block_runtime_api_impl(&self,
                                                  at:
                                                      &crate::runtime_api::BlockId<Block>,
                                                  context:
                                                      crate::runtime_api::ExecutionContext,
                                                  params:
                                                      Option<(&<Block as
                                                               BlockT>::Header)>,
                                                  params_encoded: Vec<u8>)
        -> crate::error::Result<crate::runtime_api::NativeOrEncoded<()>>;
        #[doc = " Returns the authorities."]
        #[deprecated(since = "1.0",
                     note = "Please switch to `AuthoritiesApi`.")]
        fn authorities(&self, at: &crate::runtime_api::BlockId<Block>)
         ->
             ::std::result::Result<Vec<AuthorityIdFor<Block>>,
                                   crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&());
            self.Core_authorities_runtime_api_impl(at,
                                                   crate::runtime_api::ExecutionContext::Other,
                                                   Some(()),
                                                   runtime_api_impl_params_encoded).and_then(|r|
                                                                                                 match r
                                                                                                     {
                                                                                                     crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                     =>
                                                                                                     {
                                                                                                         Ok(n)
                                                                                                     }
                                                                                                     crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                     =>
                                                                                                     {
                                                                                                         <Vec<AuthorityIdFor<Block>>
                                                                                                             as
                                                                                                             crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                             crate::error::Error::CallResultDecode("authorities").into())
                                                                                                     }
                                                                                                 })
        }
        #[doc = " Returns the authorities."]
        #[deprecated(since = "1.0",
                     note = "Please switch to `AuthoritiesApi`.")]
        fn authorities_with_context(&self,
                                    at: &crate::runtime_api::BlockId<Block>,
                                    context:
                                        crate::runtime_api::ExecutionContext)
         ->
             ::std::result::Result<Vec<AuthorityIdFor<Block>>,
                                   crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&());
            self.Core_authorities_runtime_api_impl(at, context, Some(()),
                                                   runtime_api_impl_params_encoded).and_then(|r|
                                                                                                 match r
                                                                                                     {
                                                                                                     crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                     =>
                                                                                                     {
                                                                                                         Ok(n)
                                                                                                     }
                                                                                                     crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                     =>
                                                                                                     {
                                                                                                         <Vec<AuthorityIdFor<Block>>
                                                                                                             as
                                                                                                             crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                             crate::error::Error::CallResultDecode("authorities").into())
                                                                                                     }
                                                                                                 })
        }
        #[doc(hidden)]
        fn Core_authorities_runtime_api_impl(&self,
                                             at:
                                                 &crate::runtime_api::BlockId<Block>,
                                             context:
                                                 crate::runtime_api::ExecutionContext,
                                             params: Option<()>,
                                             params_encoded: Vec<u8>)
        ->
            crate::error::Result<crate::runtime_api::NativeOrEncoded<Vec<AuthorityIdFor<Block>>>>;
    }
    #[cfg(any(feature = "std", test))]
    impl <Block: crate::runtime_api::BlockT>
     crate::runtime_api::RuntimeApiInfo for Core<Block> {
        const
        ID:
        [u8; 8]
        =
        [223u8, 106u8, 203u8, 104u8, 153u8, 7u8, 96u8, 155u8];
        const
        VERSION:
        u32
        =
        2u32;
    }
    #[doc =
          " The `Metadata` api trait that returns metadata for the runtime."]
    #[cfg(any(feature = "std", test))]
    pub trait Metadata<Block: crate::runtime_api::BlockT>: crate::runtime_api::Core<Block> {
        #[doc = " Returns the metadata of a runtime."]
        fn metadata(&self, at: &crate::runtime_api::BlockId<Block>)
         -> ::std::result::Result<OpaqueMetadata, crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&());
            self.Metadata_metadata_runtime_api_impl(at,
                                                    crate::runtime_api::ExecutionContext::Other,
                                                    Some(()),
                                                    runtime_api_impl_params_encoded).and_then(|r|
                                                                                                  match r
                                                                                                      {
                                                                                                      crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                      =>
                                                                                                      {
                                                                                                          Ok(n)
                                                                                                      }
                                                                                                      crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                      =>
                                                                                                      {
                                                                                                          <OpaqueMetadata
                                                                                                              as
                                                                                                              crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                              crate::error::Error::CallResultDecode("metadata").into())
                                                                                                      }
                                                                                                  })
        }
        #[doc = " Returns the metadata of a runtime."]
        fn metadata_with_context(&self,
                                 at: &crate::runtime_api::BlockId<Block>,
                                 context:
                                     crate::runtime_api::ExecutionContext)
         -> ::std::result::Result<OpaqueMetadata, crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&());
            self.Metadata_metadata_runtime_api_impl(at, context, Some(()),
                                                    runtime_api_impl_params_encoded).and_then(|r|
                                                                                                  match r
                                                                                                      {
                                                                                                      crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                      =>
                                                                                                      {
                                                                                                          Ok(n)
                                                                                                      }
                                                                                                      crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                      =>
                                                                                                      {
                                                                                                          <OpaqueMetadata
                                                                                                              as
                                                                                                              crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                              crate::error::Error::CallResultDecode("metadata").into())
                                                                                                      }
                                                                                                  })
        }
        #[doc(hidden)]
        fn Metadata_metadata_runtime_api_impl(&self,
                                              at:
                                                  &crate::runtime_api::BlockId<Block>,
                                              context:
                                                  crate::runtime_api::ExecutionContext,
                                              params: Option<()>,
                                              params_encoded: Vec<u8>)
        ->
            crate::error::Result<crate::runtime_api::NativeOrEncoded<OpaqueMetadata>>;
    }
    #[cfg(any(feature = "std", test))]
    impl <Block: crate::runtime_api::BlockT>
     crate::runtime_api::RuntimeApiInfo for Metadata<Block> {
        const
        ID:
        [u8; 8]
        =
        [55u8, 227u8, 151u8, 252u8, 124u8, 145u8, 245u8, 228u8];
        const
        VERSION:
        u32
        =
        1u32;
    }
    #[doc =
          " The `TaggedTransactionQueue` api trait for interfering with the new transaction queue."]
    #[cfg(any(feature = "std", test))]
    pub trait TaggedTransactionQueue<Block: crate::runtime_api::BlockT>: crate::runtime_api::Core<Block> {
        #[doc = " Validate the given transaction."]
        fn validate_transaction(&self,
                                at: &crate::runtime_api::BlockId<Block>,
                                tx: <Block as BlockT>::Extrinsic)
         -> ::std::result::Result<TransactionValidity, crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&(&tx));
            self.TaggedTransactionQueue_validate_transaction_runtime_api_impl(at,
                                                                              crate::runtime_api::ExecutionContext::Other,
                                                                              Some((tx)),
                                                                              runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                            match r
                                                                                                                                {
                                                                                                                                crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                                =>
                                                                                                                                {
                                                                                                                                    Ok(n)
                                                                                                                                }
                                                                                                                                crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                                =>
                                                                                                                                {
                                                                                                                                    <TransactionValidity
                                                                                                                                        as
                                                                                                                                        crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                                        crate::error::Error::CallResultDecode("validate_transaction").into())
                                                                                                                                }
                                                                                                                            })
        }
        #[doc = " Validate the given transaction."]
        fn validate_transaction_with_context(&self,
                                             at:
                                                 &crate::runtime_api::BlockId<Block>,
                                             context:
                                                 crate::runtime_api::ExecutionContext,
                                             tx: <Block as BlockT>::Extrinsic)
         -> ::std::result::Result<TransactionValidity, crate::error::Error> {
            let runtime_api_impl_params_encoded =
                crate::runtime_api::Encode::encode(&(&tx));
            self.TaggedTransactionQueue_validate_transaction_runtime_api_impl(at,
                                                                              context,
                                                                              Some((tx)),
                                                                              runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                            match r
                                                                                                                                {
                                                                                                                                crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                                =>
                                                                                                                                {
                                                                                                                                    Ok(n)
                                                                                                                                }
                                                                                                                                crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                                =>
                                                                                                                                {
                                                                                                                                    <TransactionValidity
                                                                                                                                        as
                                                                                                                                        crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                                        crate::error::Error::CallResultDecode("validate_transaction").into())
                                                                                                                                }
                                                                                                                            })
        }
        #[doc(hidden)]
        fn TaggedTransactionQueue_validate_transaction_runtime_api_impl(&self,
                                                                        at:
                                                                            &crate::runtime_api::BlockId<Block>,
                                                                        context:
                                                                            crate::runtime_api::ExecutionContext,
                                                                        params:
                                                                            Option<(<Block
                                                                                    as
                                                                                    BlockT>::Extrinsic)>,
                                                                        params_encoded:
                                                                            Vec<u8>)
        ->
            crate::error::Result<crate::runtime_api::NativeOrEncoded<TransactionValidity>>;
    }
    #[cfg(any(feature = "std", test))]
    impl <Block: crate::runtime_api::BlockT>
     crate::runtime_api::RuntimeApiInfo for TaggedTransactionQueue<Block> {
        const
        ID:
        [u8; 8]
        =
        [210u8, 188u8, 152u8, 151u8, 238u8, 208u8, 143u8, 21u8];
        const
        VERSION:
        u32
        =
        1u32;
    }
}
#[cfg(feature = "std")]
pub mod error {
    //! Substrate client possible errors.
    use std::{self, error, result};
    use state_machine;
    use runtime_primitives::ApplyError;
    use consensus;
    use derive_more::{Display, From};
    /// Client Result type alias
    pub type Result<T> = result::Result<T, Error>;
    /// Substrate Client error
    pub enum Error {

        /// Consensus Error
        #[display(fmt = "Consensus: {}", _0)]
        Consensus(consensus::Error),

        /// Backend error.
        #[display(fmt = "Backend error: {}", _0)]
        Backend(String),

        /// Unknown block.
        #[display(fmt = "UnknownBlock: {}", _0)]
        UnknownBlock(String),

        /// Applying extrinsic error.
        #[display(fmt = "Extrinsic error: {:?}", _0)]
        ApplyExtrinsicFailed(ApplyError),

        /// Execution error.
        #[display(fmt = "Execution: {}", _0)]
        Execution(Box<state_machine::Error>),

        /// Blockchain error.
        #[display(fmt = "Blockchain: {}", _0)]
        Blockchain(Box<Error>),

        /// Invalid authorities set received from the runtime.
        #[display(fmt =
                      "Current state of blockchain has invalid authorities set")]
        InvalidAuthoritiesSet,

        /// Could not get runtime version.
        #[display(fmt = "On-chain runtime does not specify version")]
        VersionInvalid,

        /// Genesis config is invalid.
        #[display(fmt = "Genesis config provided is invalid")]
        GenesisInvalid,

        /// Error decoding header justification.
        #[display(fmt = "error decoding justification for header")]
        JustificationDecode,

        /// Justification for header is correctly encoded, but invalid.
        #[display(fmt = "bad justification for header: {}", _0)]
        BadJustification(String),

        /// Not available on light client.
        #[display(fmt =
                      "This method is not currently available when running in light client mode")]
        NotAvailableOnLightClient,

        /// Invalid remote CHT-based proof.
        #[display(fmt =
                      "Remote node has responded with invalid header proof")]
        InvalidCHTProof,

        /// Remote fetch has been cancelled.
        #[display(fmt = "Remote data fetch has been cancelled")]
        RemoteFetchCancelled,

        /// Remote fetch has been failed.
        #[display(fmt = "Remote data fetch has been failed")]
        RemoteFetchFailed,

        /// Error decoding call result.
        #[display(fmt = "Error decoding call result of {}", _0)]
        CallResultDecode(&'static str),

        /// Error converting a parameter between runtime and node.
        #[display(fmt = "Error converting `{}` between runtime and node", _0)]
        RuntimeParamConversion(&'static str),

        /// Changes tries are not supported.
        #[display(fmt = "Changes tries are not supported by the runtime")]
        ChangesTriesNotSupported,

        /// Key changes query has failed.
        #[display(fmt = "Failed to check changes proof: {}", _0)]
        ChangesTrieAccessFailed(String),

        /// Last finalized block not parent of current.
        #[display(fmt = "Did not finalize blocks in sequential order.")]
        NonSequentialFinalization(String),

        /// Safety violation: new best block not descendent of last finalized.
        #[display(fmt =
                      "Potential long-range attack: block not in finalized chain.")]
        NotInFinalizedChain,

        /// Hash that is required for building CHT is missing.
        #[display(fmt = "Failed to get hash of block#{} for building CHT#{}",
                  _0,
                  _1)]
        MissingHashRequiredForCHT(u64, u64),

        /// A convenience variant for String
        #[display(fmt = "{}", _0)]
        Msg(String),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&Error::Consensus(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Consensus");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::Backend(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Backend");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::UnknownBlock(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("UnknownBlock");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::ApplyExtrinsicFailed(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ApplyExtrinsicFailed");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::Execution(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Execution");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::Blockchain(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Blockchain");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::InvalidAuthoritiesSet,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidAuthoritiesSet");
                    debug_trait_builder.finish()
                }
                (&Error::VersionInvalid,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("VersionInvalid");
                    debug_trait_builder.finish()
                }
                (&Error::GenesisInvalid,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("GenesisInvalid");
                    debug_trait_builder.finish()
                }
                (&Error::JustificationDecode,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("JustificationDecode");
                    debug_trait_builder.finish()
                }
                (&Error::BadJustification(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("BadJustification");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::NotAvailableOnLightClient,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NotAvailableOnLightClient");
                    debug_trait_builder.finish()
                }
                (&Error::InvalidCHTProof,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InvalidCHTProof");
                    debug_trait_builder.finish()
                }
                (&Error::RemoteFetchCancelled,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("RemoteFetchCancelled");
                    debug_trait_builder.finish()
                }
                (&Error::RemoteFetchFailed,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("RemoteFetchFailed");
                    debug_trait_builder.finish()
                }
                (&Error::CallResultDecode(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("CallResultDecode");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::RuntimeParamConversion(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("RuntimeParamConversion");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::ChangesTriesNotSupported,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ChangesTriesNotSupported");
                    debug_trait_builder.finish()
                }
                (&Error::ChangesTrieAccessFailed(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("ChangesTrieAccessFailed");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::NonSequentialFinalization(ref __self_0),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NonSequentialFinalization");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
                (&Error::NotInFinalizedChain,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("NotInFinalizedChain");
                    debug_trait_builder.finish()
                }
                (&Error::MissingHashRequiredForCHT(ref __self_0,
                                                   ref __self_1),) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("MissingHashRequiredForCHT");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
                (&Error::Msg(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("Msg");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl ::std::fmt::Display for Error {
        #[allow(unused_variables)]
        #[inline]
        fn fmt(&self,
               _derive_more_Display_formatter: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match self {
                Error::Consensus(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Consensus: "],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::Backend(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Backend error: "],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::UnknownBlock(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["UnknownBlock: "],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::ApplyExtrinsicFailed(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Extrinsic error: "],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Debug::fmt)],
                                                                                        })),
                Error::Execution(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Execution: "],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::Blockchain(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Blockchain: "],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::InvalidAuthoritiesSet =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Current state of blockchain has invalid authorities set"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::VersionInvalid =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["On-chain runtime does not specify version"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::GenesisInvalid =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Genesis config provided is invalid"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::JustificationDecode =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["error decoding justification for header"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::BadJustification(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["bad justification for header: "],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::NotAvailableOnLightClient =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["This method is not currently available when running in light client mode"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::InvalidCHTProof =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Remote node has responded with invalid header proof"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::RemoteFetchCancelled =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Remote data fetch has been cancelled"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::RemoteFetchFailed =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Remote data fetch has been failed"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::CallResultDecode(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Error decoding call result of "],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::RuntimeParamConversion(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Error converting `",
                                                                                         "` between runtime and node"],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::ChangesTriesNotSupported =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Changes tries are not supported by the runtime"],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::ChangesTrieAccessFailed(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Failed to check changes proof: "],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::NonSequentialFinalization(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Did not finalize blocks in sequential order."],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::NotInFinalizedChain =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Potential long-range attack: block not in finalized chain."],
                                                                                       &match ()
                                                                                            {
                                                                                            ()
                                                                                            =>
                                                                                            [],
                                                                                        })),
                Error::MissingHashRequiredForCHT(_0, _1) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&["Failed to get hash of block#",
                                                                                         " for building CHT#"],
                                                                                       &match (&_0,
                                                                                               &_1)
                                                                                            {
                                                                                            (arg0,
                                                                                             arg1)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt),
                                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                Error::Msg(_0) =>
                _derive_more_Display_formatter.write_fmt(::std::fmt::Arguments::new_v1(&[""],
                                                                                       &match (&_0,)
                                                                                            {
                                                                                            (arg0,)
                                                                                            =>
                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                        })),
                _ => Ok(()),
            }
        }
    }
    impl ::std::convert::From<(consensus::Error)> for Error {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (consensus::Error)) -> Error {
            Error::Consensus(original)
        }
    }
    impl ::std::convert::From<(ApplyError)> for Error {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (ApplyError)) -> Error {
            Error::ApplyExtrinsicFailed(original)
        }
    }
    impl ::std::convert::From<(Box<state_machine::Error>)> for Error {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (Box<state_machine::Error>)) -> Error {
            Error::Execution(original)
        }
    }
    impl ::std::convert::From<(Box<Error>)> for Error {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (Box<Error>)) -> Error {
            Error::Blockchain(original)
        }
    }
    impl ::std::convert::From<(u64, u64)> for Error {
        #[allow(unused_variables)]
        #[inline]
        fn from(original: (u64, u64)) -> Error {
            Error::MissingHashRequiredForCHT(original.0, original.1)
        }
    }
    impl error::Error for Error {
        fn source(&self) -> Option<&(error::Error + 'static)> {
            match self {
                Error::Consensus(e) => Some(e),
                Error::Blockchain(e) => Some(e),
                _ => None,
            }
        }
    }
    impl From<String> for Error {
        fn from(s: String) -> Self { Error::Msg(s) }
    }
    impl <'a> From<&'a str> for Error {
        fn from(s: &'a str) -> Self { Error::Msg(s.into()) }
    }
    impl Error {
        /// Chain a blockchain error.
        pub fn from_blockchain(e: Box<Error>) -> Self { Error::Blockchain(e) }
        /// Chain a state error.
        pub fn from_state(e: Box<state_machine::Error + Send>) -> Self {
            Error::Execution(e)
        }
    }
    impl state_machine::Error for Error { }
}
#[cfg(feature = "std")]
pub mod blockchain {
    //! Substrate blockchain trait
    use std::sync::Arc;
    use runtime_primitives::traits::{Block as BlockT, Header as HeaderT,
                                     NumberFor};
    use runtime_primitives::generic::BlockId;
    use runtime_primitives::Justification;
    use consensus::well_known_cache_keys;
    use crate::error::{Error, Result};
    /// Blockchain database header backend. Does not perform any validation.
    pub trait HeaderBackend<Block: BlockT>: Send + Sync {
        /// Get block header. Returns `None` if block is not found.
        fn header(&self, id: BlockId<Block>)
        -> Result<Option<Block::Header>>;
        /// Get blockchain info.
        fn info(&self)
        -> Result<Info<Block>>;
        /// Get block status.
        fn status(&self, id: BlockId<Block>)
        -> Result<BlockStatus>;
        /// Get block number by hash. Returns `None` if the header is not in the chain.
        fn number(&self, hash: Block::Hash)
        -> Result<Option<<<Block as BlockT>::Header as HeaderT>::Number>>;
        /// Get block hash by number. Returns `None` if the header is not in the chain.
        fn hash(&self, number: NumberFor<Block>)
        -> Result<Option<Block::Hash>>;
        /// Convert an arbitrary block ID into a block hash.
        fn block_hash_from_id(&self, id: &BlockId<Block>)
         -> Result<Option<Block::Hash>> {
            match *id {
                BlockId::Hash(h) => Ok(Some(h)),
                BlockId::Number(n) => self.hash(n),
            }
        }
        /// Convert an arbitrary block ID into a block hash.
        fn block_number_from_id(&self, id: &BlockId<Block>)
         -> Result<Option<NumberFor<Block>>> {
            match *id {
                BlockId::Hash(_) =>
                Ok(self.header(*id)?.map(|h| h.number().clone())),
                BlockId::Number(n) => Ok(Some(n)),
            }
        }
        /// Get block header. Returns `UnknownBlock` error if block is not found.
        fn expect_header(&self, id: BlockId<Block>) -> Result<Block::Header> {
            self.header(id)?.ok_or_else(||
                                            Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                   &match (&id,)
                                                                                                                        {
                                                                                                                        (arg0,)
                                                                                                                        =>
                                                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                     ::std::fmt::Display::fmt)],
                                                                                                                    }))))
        }
        /// Convert an arbitrary block ID into a block number. Returns `UnknownBlock` error if block is not found.
        fn expect_block_number_from_id(&self, id: &BlockId<Block>)
         -> Result<NumberFor<Block>> {
            self.block_number_from_id(id).and_then(|n|
                                                       n.ok_or_else(||
                                                                        Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                               &match (&id,)
                                                                                                                                                    {
                                                                                                                                                    (arg0,)
                                                                                                                                                    =>
                                                                                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                                                                                })))))
        }
        /// Convert an arbitrary block ID into a block hash. Returns `UnknownBlock` error if block is not found.
        fn expect_block_hash_from_id(&self, id: &BlockId<Block>)
         -> Result<Block::Hash> {
            self.block_hash_from_id(id).and_then(|n|
                                                     n.ok_or_else(||
                                                                      Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                             &match (&id,)
                                                                                                                                                  {
                                                                                                                                                  (arg0,)
                                                                                                                                                  =>
                                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                               ::std::fmt::Display::fmt)],
                                                                                                                                              })))))
        }
    }
    /// Blockchain database backend. Does not perform any validation.
    pub trait Backend<Block: BlockT>: HeaderBackend<Block> {
        /// Get block body. Returns `None` if block is not found.
        fn body(&self, id: BlockId<Block>)
        -> Result<Option<Vec<<Block as BlockT>::Extrinsic>>>;
        /// Get block justification. Returns `None` if justification does not exist.
        fn justification(&self, id: BlockId<Block>)
        -> Result<Option<Justification>>;
        /// Get last finalized block hash.
        fn last_finalized(&self)
        -> Result<Block::Hash>;
        /// Returns data cache reference, if it is enabled on this backend.
        fn cache(&self)
        -> Option<Arc<Cache<Block>>>;
        /// Returns hashes of all blocks that are leaves of the block tree.
        /// in other words, that have no children, are chain heads.
        /// Results must be ordered best (longest, heighest) chain first.
        fn leaves(&self)
        -> Result<Vec<Block::Hash>>;
        /// Return hashes of all blocks that are children of the block with `parent_hash`.
        fn children(&self, parent_hash: Block::Hash)
        -> Result<Vec<Block::Hash>>;
    }
    /// Provides access to the optional cache.
    pub trait ProvideCache<Block: BlockT> {
        /// Returns data cache reference, if it is enabled on this backend.
        fn cache(&self)
        -> Option<Arc<Cache<Block>>>;
    }
    /// Blockchain optional data cache.
    pub trait Cache<Block: BlockT>: Send + Sync {
        /// Initialize genesis value for the given cache.
        ///
        /// The operation should be performed once before anything else is inserted in the cache.
        /// Otherwise cache may end up in inconsistent state.
        fn initialize(&self, key: &well_known_cache_keys::Id,
                      value_at_genesis: Vec<u8>)
        -> Result<()>;
        /// Returns cached value by the given key.
        fn get_at(&self, key: &well_known_cache_keys::Id,
                  block: &BlockId<Block>)
        -> Option<Vec<u8>>;
    }
    /// Blockchain info
    pub struct Info<Block: BlockT> {
        /// Best block hash.
        pub best_hash: Block::Hash,
        /// Best block number.
        pub best_number: <<Block as BlockT>::Header as HeaderT>::Number,
        /// Genesis block hash.
        pub genesis_hash: Block::Hash,
        /// The head of the finalized chain.
        pub finalized_hash: Block::Hash,
        /// Last finalized block number.
        pub finalized_number: <<Block as BlockT>::Header as HeaderT>::Number,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::fmt::Debug + BlockT> ::std::fmt::Debug for Info<Block>
     where Block::Hash: ::std::fmt::Debug, Block::Hash: ::std::fmt::Debug,
     Block::Hash: ::std::fmt::Debug {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Info {
                best_hash: ref __self_0_0,
                best_number: ref __self_0_1,
                genesis_hash: ref __self_0_2,
                finalized_hash: ref __self_0_3,
                finalized_number: ref __self_0_4 } => {
                    let mut debug_trait_builder = f.debug_struct("Info");
                    let _ =
                        debug_trait_builder.field("best_hash",
                                                  &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("best_number",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("genesis_hash",
                                                  &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("finalized_hash",
                                                  &&(*__self_0_3));
                    let _ =
                        debug_trait_builder.field("finalized_number",
                                                  &&(*__self_0_4));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Block status.
    #[structural_match]
    pub enum BlockStatus {

        /// Already in the blockchain.
        InChain,

        /// Not in the queue or the blockchain.
        Unknown,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlockStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&BlockStatus::InChain,) => {
                    let mut debug_trait_builder = f.debug_tuple("InChain");
                    debug_trait_builder.finish()
                }
                (&BlockStatus::Unknown,) => {
                    let mut debug_trait_builder = f.debug_tuple("Unknown");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlockStatus {
        #[inline]
        fn eq(&self, other: &BlockStatus) -> bool {
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
    impl ::std::cmp::Eq for BlockStatus {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    /// An entry in a tree route.
    pub struct RouteEntry<Block: BlockT> {
        /// The number of the block.
        pub number: <Block::Header as HeaderT>::Number,
        /// The hash of the block.
        pub hash: Block::Hash,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::fmt::Debug + BlockT> ::std::fmt::Debug for
     RouteEntry<Block> where Block::Header: ::std::fmt::Debug,
     Block::Hash: ::std::fmt::Debug {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                RouteEntry { number: ref __self_0_0, hash: ref __self_0_1 } =>
                {
                    let mut debug_trait_builder =
                        f.debug_struct("RouteEntry");
                    let _ =
                        debug_trait_builder.field("number", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("hash", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// A tree-route from one block to another in the chain.
    ///
    /// All blocks prior to the pivot in the deque is the reverse-order unique ancestry
    /// of the first block, the block at the pivot index is the common ancestor,
    /// and all blocks after the pivot is the ancestry of the second block, in
    /// order.
    ///
    /// The ancestry sets will include the given blocks, and thus the tree-route is
    /// never empty.
    ///
    /// ```text
    /// Tree route from R1 to E2. Retracted is [R1, R2, R3], Common is C, enacted [E1, E2]
    ///   <- R3 <- R2 <- R1
    ///  /
    /// C
    ///  \-> E1 -> E2
    /// ```
    ///
    /// ```text
    /// Tree route from C to E2. Retracted empty. Common is C, enacted [E1, E2]
    /// C -> E1 -> E2
    /// ```
    pub struct TreeRoute<Block: BlockT> {
        route: Vec<RouteEntry<Block>>,
        pivot: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::fmt::Debug + BlockT> ::std::fmt::Debug for
     TreeRoute<Block> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                TreeRoute { route: ref __self_0_0, pivot: ref __self_0_1 } =>
                {
                    let mut debug_trait_builder = f.debug_struct("TreeRoute");
                    let _ =
                        debug_trait_builder.field("route", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("pivot", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <Block: BlockT> TreeRoute<Block> {
        /// Get a slice of all retracted blocks in reverse order (towards common ancestor)
        pub fn retracted(&self) -> &[RouteEntry<Block>] {
            &self.route[..self.pivot]
        }
        /// Get the common ancestor block. This might be one of the two blocks of the
        /// route.
        pub fn common_block(&self) -> &RouteEntry<Block> {
            self.route.get(self.pivot).expect("tree-routes are computed between blocks; \
			which are included in the route; \
			thus it is never empty; qed")
        }
        /// Get a slice of enacted blocks (descendents of the common ancestor)
        pub fn enacted(&self) -> &[RouteEntry<Block>] {
            &self.route[self.pivot + 1..]
        }
    }
    /// Compute a tree-route between two blocks. See tree-route docs for more details.
    pub fn tree_route<Block: BlockT,
                      Backend: HeaderBackend<Block>>(backend: &Backend,
                                                     from: BlockId<Block>,
                                                     to: BlockId<Block>)
     -> Result<TreeRoute<Block>> {
        use runtime_primitives::traits::Header;
        let load_header =
            |id: BlockId<Block>|
                {
                    match backend.header(id) {
                        Ok(Some(hdr)) => Ok(hdr),
                        Ok(None) =>
                        Err(Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Unknown block "],
                                                                                                   &match (&id,)
                                                                                                        {
                                                                                                        (arg0,)
                                                                                                        =>
                                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                     ::std::fmt::Debug::fmt)],
                                                                                                    })))),
                        Err(e) => Err(e),
                    }
                };
        let mut from = load_header(from)?;
        let mut to = load_header(to)?;
        let mut from_branch = Vec::new();
        let mut to_branch = Vec::new();
        while to.number() > from.number() {
            to_branch.push(RouteEntry{number: to.number().clone(),
                                      hash: to.hash(),});
            to = load_header(BlockId::Hash(*to.parent_hash()))?;
        }
        while from.number() > to.number() {
            from_branch.push(RouteEntry{number: from.number().clone(),
                                        hash: from.hash(),});
            from = load_header(BlockId::Hash(*from.parent_hash()))?;
        }
        while to != from {
            to_branch.push(RouteEntry{number: to.number().clone(),
                                      hash: to.hash(),});
            to = load_header(BlockId::Hash(*to.parent_hash()))?;
            from_branch.push(RouteEntry{number: from.number().clone(),
                                        hash: from.hash(),});
            from = load_header(BlockId::Hash(*from.parent_hash()))?;
        }
        let pivot = from_branch.len();
        from_branch.push(RouteEntry{number: to.number().clone(),
                                    hash: to.hash(),});
        from_branch.extend(to_branch.into_iter().rev());
        Ok(TreeRoute{route: from_branch, pivot,})
    }
}
#[cfg(feature = "std")]
pub mod backend {
    //! Substrate Client data backend
    use std::collections::HashMap;
    use crate::error;
    use primitives::ChangesTrieConfiguration;
    use runtime_primitives::{generic::BlockId, Justification, StorageOverlay,
                             ChildrenStorageOverlay};
    use runtime_primitives::traits::{Block as BlockT, NumberFor};
    use state_machine::backend::Backend as StateBackend;
    use state_machine::ChangesTrieStorage as StateChangesTrieStorage;
    use consensus::well_known_cache_keys;
    use hash_db::Hasher;
    use trie::MemoryDB;
    /// State of a new block.
    #[structural_match]
    #[rustc_copy_clone_marker]
    pub enum NewBlockState {

        /// Normal block.
        Normal,

        /// New best block.
        Best,

        /// Newly finalized block (implicitly best).
        Final,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for NewBlockState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&NewBlockState::Normal,) => {
                    let mut debug_trait_builder = f.debug_tuple("Normal");
                    debug_trait_builder.finish()
                }
                (&NewBlockState::Best,) => {
                    let mut debug_trait_builder = f.debug_tuple("Best");
                    debug_trait_builder.finish()
                }
                (&NewBlockState::Final,) => {
                    let mut debug_trait_builder = f.debug_tuple("Final");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for NewBlockState {
        #[inline]
        fn clone(&self) -> NewBlockState { { *self } }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::marker::Copy for NewBlockState { }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for NewBlockState {
        #[inline]
        fn eq(&self, other: &NewBlockState) -> bool {
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
    impl ::std::cmp::Eq for NewBlockState {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    impl NewBlockState {
        /// Whether this block is the new best block.
        pub fn is_best(self) -> bool {
            match self {
                NewBlockState::Best | NewBlockState::Final => true,
                NewBlockState::Normal => false,
            }
        }
        /// Whether this block is considered final.
        pub fn is_final(self) -> bool {
            match self {
                NewBlockState::Final => true,
                NewBlockState::Best | NewBlockState::Normal => false,
            }
        }
    }
    /// Block insertion operation. Keeps hold if the inserted block state and data.
    pub trait BlockImportOperation<Block, H> where Block: BlockT,
     H: Hasher<Out = Block::Hash> {
        /// Associated state backend type.
        type
        State: StateBackend<H>;
        /// Returns pending state. Returns None for backends with locally-unavailable state data.
        fn state(&self)
        -> error::Result<Option<&Self::State>>;
        /// Append block data to the transaction.
        fn set_block_data(&mut self, header: Block::Header,
                          body: Option<Vec<Block::Extrinsic>>,
                          justification: Option<Justification>,
                          state: NewBlockState)
        -> error::Result<()>;
        /// Update cached data.
        fn update_cache(&mut self,
                        cache: HashMap<well_known_cache_keys::Id, Vec<u8>>);
        /// Inject storage data into the database.
        fn update_db_storage(&mut self,
                             update:
                                 <Self::State as
                                 StateBackend<H>>::Transaction)
        -> error::Result<()>;
        /// Inject storage data into the database replacing any existing data.
        fn reset_storage(&mut self, top: StorageOverlay,
                         children: ChildrenStorageOverlay)
        -> error::Result<H::Out>;
        /// Set top level storage changes.
        fn update_storage(&mut self, update: Vec<(Vec<u8>, Option<Vec<u8>>)>)
        -> error::Result<()>;
        /// Inject changes trie data into the database.
        fn update_changes_trie(&mut self, update: MemoryDB<H>)
        -> error::Result<()>;
        /// Insert auxiliary keys. Values are `None` if should be deleted.
        fn insert_aux<I>(&mut self, ops: I)
        -> error::Result<()>
        where
        I: IntoIterator<Item
        =
        (Vec<u8>, Option<Vec<u8>>)>;
        /// Mark a block as finalized.
        fn mark_finalized(&mut self, id: BlockId<Block>,
                          justification: Option<Justification>)
        -> error::Result<()>;
        /// Mark a block as new head. If both block import and set head are specified, set head overrides block import's best block rule.
        fn mark_head(&mut self, id: BlockId<Block>)
        -> error::Result<()>;
    }
    /// Provides access to an auxiliary database.
    pub trait AuxStore {
        /// Insert auxiliary data into key-value store. Deletions occur after insertions.
        fn insert_aux<'a, 'b: 'a, 'c: 'a, I: IntoIterator<Item =
                      &'a (&'c [u8], &'c [u8])>, D: IntoIterator<Item =
                      &'a &'b [u8]>>(&self, insert: I, delete: D)
        -> error::Result<()>;
        /// Query auxiliary data from key-value store.
        fn get_aux(&self, key: &[u8])
        -> error::Result<Option<Vec<u8>>>;
    }
    /// Client backend. Manages the data layer.
    ///
    /// Note on state pruning: while an object from `state_at` is alive, the state
    /// should not be pruned. The backend should internally reference-count
    /// its state objects.
    ///
    /// The same applies for live `BlockImportOperation`s: while an import operation building on a parent `P`
    /// is alive, the state for `P` should not be pruned.
    pub trait Backend<Block, H>: AuxStore + Send + Sync where Block: BlockT,
     H: Hasher<Out = Block::Hash> {
        /// Associated block insertion operation type.
        type
        BlockImportOperation: BlockImportOperation<Block, H,
        State
        =
        Self::State>;
        /// Associated blockchain backend type.
        type
        Blockchain: crate::blockchain::Backend<Block>;
        /// Associated state backend type.
        type
        State: StateBackend<H>;
        /// Changes trie storage.
        type
        ChangesTrieStorage: PrunableStateChangesTrieStorage<H>;
        /// Begin a new block insertion transaction with given parent block id.
        /// When constructing the genesis, this is called with all-zero hash.
        fn begin_operation(&self)
        -> error::Result<Self::BlockImportOperation>;
        /// Note an operation to contain state transition.
        fn begin_state_operation(&self,
                                 operation: &mut Self::BlockImportOperation,
                                 block: BlockId<Block>)
        -> error::Result<()>;
        /// Commit block insertion.
        fn commit_operation(&self, transaction: Self::BlockImportOperation)
        -> error::Result<()>;
        /// Finalize block with given Id. This should only be called if the parent of the given
        /// block has been finalized.
        fn finalize_block(&self, block: BlockId<Block>,
                          justification: Option<Justification>)
        -> error::Result<()>;
        /// Returns reference to blockchain backend.
        fn blockchain(&self)
        -> &Self::Blockchain;
        /// Returns the used state cache, if existent.
        fn used_state_cache_size(&self)
        -> Option<usize>;
        /// Returns reference to changes trie storage.
        fn changes_trie_storage(&self)
        -> Option<&Self::ChangesTrieStorage>;
        /// Returns true if state for given block is available.
        fn have_state_at(&self, hash: &Block::Hash, _number: NumberFor<Block>)
         -> bool {
            self.state_at(BlockId::Hash(hash.clone())).is_ok()
        }
        /// Returns state backend with post-state of given block.
        fn state_at(&self, block: BlockId<Block>)
        -> error::Result<Self::State>;
        /// Destroy state and save any useful data, such as cache.
        fn destroy_state(&self, _state: Self::State) -> error::Result<()> {
            Ok(())
        }
        /// Attempts to revert the chain by `n` blocks. Returns the number of blocks that were
        /// successfully reverted.
        fn revert(&self, n: NumberFor<Block>)
        -> error::Result<NumberFor<Block>>;
        /// Insert auxiliary data into key-value store.
        fn insert_aux<'a, 'b: 'a, 'c: 'a, I: IntoIterator<Item =
                      &'a (&'c [u8], &'c [u8])>, D: IntoIterator<Item =
                      &'a &'b [u8]>>(&self, insert: I, delete: D)
         -> error::Result<()> {
            AuxStore::insert_aux(self, insert, delete)
        }
        /// Query auxiliary data from key-value store.
        fn get_aux(&self, key: &[u8]) -> error::Result<Option<Vec<u8>>> {
            AuxStore::get_aux(self, key)
        }
    }
    /// Changes trie storage that supports pruning.
    pub trait PrunableStateChangesTrieStorage<H: Hasher>: StateChangesTrieStorage<H> {
        /// Get number block of oldest, non-pruned changes trie.
        fn oldest_changes_trie_block(&self, config: &ChangesTrieConfiguration,
                                     best_finalized: u64)
        -> u64;
    }
    /// Mark for all Backend implementations, that are making use of state data, stored locally.
    pub trait LocalBackend<Block, H>: Backend<Block, H> where Block: BlockT,
     H: Hasher<Out = Block::Hash> {
    }
    /// Mark for all Backend implementations, that are fetching required state data from remote nodes.
    pub trait RemoteBackend<Block, H>: Backend<Block, H> where Block: BlockT,
     H: Hasher<Out = Block::Hash> {
        /// Returns true if the state for given block is available locally.
        fn is_local_state_available(&self, block: &BlockId<Block>)
        -> bool;
    }
}
#[cfg(feature = "std")]
pub mod cht {
    //! Canonical hash trie definitions and helper functions.
    //!
    //! Each CHT is a trie mapping block numbers to canonical hash.
    //! One is generated for every `SIZE` blocks, allowing us to discard those blocks in
    //! favor of the trie root. When the "ancient" blocks need to be accessed, we simply
    //! request an inclusion proof of a specific block number against the trie with the
    //! root has. A correct proof implies that the claimed block is identical to the one
    //! we discarded.
    use std::collections::HashSet;
    use hash_db;
    use trie;
    use primitives::{H256, convert_hash};
    use runtime_primitives::traits::{As, Header as HeaderT, SimpleArithmetic,
                                     One};
    use state_machine::backend::InMemory as InMemoryState;
    use state_machine::{MemoryDB, TrieBackend, Backend as StateBackend,
                        prove_read_on_trie_backend, read_proof_check,
                        read_proof_check_on_proving_backend};
    use crate::error::{Error as ClientError, Result as ClientResult};
    /// The size of each CHT. This value is passed to every CHT-related function from
    /// production code. Other values are passed from tests.
    pub const SIZE: u64 = 2048;
    /// Returns Some(cht_number) if CHT is need to be built when the block with given number is canonized.
    pub fn is_build_required<N>(cht_size: u64, block_num: N) -> Option<N>
     where N: Clone + SimpleArithmetic {
        let block_cht_num = block_to_cht_number(cht_size, block_num.clone())?;
        let two = N::one() + N::one();
        if block_cht_num < two { return None; }
        let cht_start = start_number(cht_size, block_cht_num.clone());
        if cht_start != block_num { return None; }
        Some(block_cht_num - two)
    }
    /// Compute a CHT root from an iterator of block hashes. Fails if shorter than
    /// SIZE items. The items are assumed to proceed sequentially from `start_number(cht_num)`.
    /// Discards the trie's nodes.
    pub fn compute_root<Header, Hasher,
                        I>(cht_size: u64, cht_num: Header::Number, hashes: I)
     -> ClientResult<Hasher::Out> where Header: HeaderT,
     Hasher: hash_db::Hasher, Hasher::Out: Ord, I: IntoIterator<Item =
     ClientResult<Option<Header::Hash>>> {
        Ok(trie::trie_root::<Hasher, _, _,
                             _>(build_pairs::<Header,
                                              I>(cht_size, cht_num, hashes)?))
    }
    /// Build CHT-based header proof.
    pub fn build_proof<Header, Hasher, BlocksI,
                       HashesI>(cht_size: u64, cht_num: Header::Number,
                                blocks: BlocksI, hashes: HashesI)
     -> ClientResult<Vec<Vec<u8>>> where Header: HeaderT,
     Hasher: hash_db::Hasher, Hasher::Out: Ord, BlocksI: IntoIterator<Item =
     Header::Number>, HashesI: IntoIterator<Item =
     ClientResult<Option<Header::Hash>>> {
        let transaction =
            build_pairs::<Header,
                          _>(cht_size, cht_num,
                             hashes)?.into_iter().map(|(k, v)|
                                                          (None, k,
                                                           Some(v))).collect::<Vec<_>>();
        let storage = InMemoryState::<Hasher>::default().update(transaction);
        let trie_storage =
            storage.try_into_trie_backend().expect("InMemoryState::try_into_trie_backend always returns Some; qed");
        let mut total_proof = HashSet::new();
        for block in blocks.into_iter() {
            if true {
                {
                    match (&block_to_cht_number(cht_size, block),
                           &Some(cht_num)) {
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
                                                               &("core/client/src/cht.rs",
                                                                 103u32,
                                                                 3u32))
                                }
                            }
                        }
                    }
                };
            };
            let (value, proof) =
                prove_read_on_trie_backend(&trie_storage,
                                           &encode_cht_key(block))?;
            if !value.is_some() {
                {
                    ::std::rt::begin_panic("we have just built trie that includes the value for block",
                                           &("core/client/src/cht.rs", 106u32,
                                             3u32))
                }
            };
            total_proof.extend(proof);
        }
        Ok(total_proof.into_iter().collect())
    }
    /// Check CHT-based header proof.
    pub fn check_proof<Header,
                       Hasher>(local_root: Header::Hash,
                               local_number: Header::Number,
                               remote_hash: Header::Hash,
                               remote_proof: Vec<Vec<u8>>) -> ClientResult<()>
     where Header: HeaderT, Hasher: hash_db::Hasher, Hasher::Out: Ord {
        do_check_proof::<Header, Hasher,
                         _>(local_root, local_number, remote_hash,
                            move |local_root, local_cht_key|
                                read_proof_check::<Hasher>(local_root,
                                                           remote_proof,
                                                           local_cht_key).map_err(|e|
                                                                                      ClientError::from(e)))
    }
    /// Check CHT-based header proof on pre-created proving backend.
    pub fn check_proof_on_proving_backend<Header,
                                          Hasher>(local_root: Header::Hash,
                                                  local_number:
                                                      Header::Number,
                                                  remote_hash: Header::Hash,
                                                  proving_backend:
                                                      &TrieBackend<MemoryDB<Hasher>,
                                                                   Hasher>)
     -> ClientResult<()> where Header: HeaderT, Hasher: hash_db::Hasher,
     Hasher::Out: Ord {
        do_check_proof::<Header, Hasher,
                         _>(local_root, local_number, remote_hash,
                            |_, local_cht_key|
                                read_proof_check_on_proving_backend::<Hasher>(proving_backend,
                                                                              local_cht_key).map_err(|e|
                                                                                                         ClientError::from(e)))
    }
    /// Check CHT-based header proof using passed checker function.
    fn do_check_proof<Header, Hasher,
                      F>(local_root: Header::Hash,
                         local_number: Header::Number,
                         remote_hash: Header::Hash, checker: F)
     -> ClientResult<()> where Header: HeaderT, Hasher: hash_db::Hasher,
     Hasher::Out: Ord, F: FnOnce(Hasher::Out, &[u8]) ->
     ClientResult<Option<Vec<u8>>> {
        let root: Hasher::Out = convert_hash(&local_root);
        let local_cht_key = encode_cht_key(local_number);
        let local_cht_value = checker(root, &local_cht_key)?;
        let local_cht_value =
            local_cht_value.ok_or_else(|| ClientError::InvalidCHTProof)?;
        let local_hash =
            decode_cht_value(&local_cht_value).ok_or_else(||
                                                              ClientError::InvalidCHTProof)?;
        match &local_hash[..] == remote_hash.as_ref() {
            true => Ok(()),
            false => Err(ClientError::InvalidCHTProof.into()),
        }
    }
    /// Group ordered blocks by CHT number and call functor with blocks of each group.
    pub fn for_each_cht_group<Header, I, F,
                              P>(cht_size: u64, blocks: I, mut functor: F,
                                 mut functor_param: P) -> ClientResult<()>
     where Header: HeaderT, I: IntoIterator<Item = Header::Number>,
     F: FnMut(P, Header::Number, Vec<Header::Number>) -> ClientResult<P> {
        let mut current_cht_num = None;
        let mut current_cht_blocks = Vec::new();
        for block in blocks {
            let new_cht_num =
                match block_to_cht_number(cht_size, block.as_()) {
                    Some(new_cht_num) => new_cht_num,
                    None =>
                    return Err(ClientError::Backend(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Cannot compute CHT root for the block #"],
                                                                                                       &match (&block,)
                                                                                                            {
                                                                                                            (arg0,)
                                                                                                            =>
                                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                                        }))).into()),
                };
            let advance_to_next_cht =
                current_cht_num.is_some() &&
                    current_cht_num != Some(new_cht_num);
            if advance_to_next_cht {
                let current_cht_num =
                    current_cht_num.expect("advance_to_next_cht is true;
				it is true only when current_cht_num is Some; qed");
                if !(new_cht_num > current_cht_num) {
                    {
                        ::std::rt::begin_panic("for_each_cht_group only supports ordered iterators",
                                               &("core/client/src/cht.rs",
                                                 197u32, 4u32))
                    }
                };
                functor_param =
                    functor(functor_param, As::sa(current_cht_num),
                            ::std::mem::replace(&mut current_cht_blocks,
                                                Vec::new()))?;
            }
            current_cht_blocks.push(block);
            current_cht_num = Some(new_cht_num);
        }
        if let Some(current_cht_num) = current_cht_num {
            functor(functor_param, As::sa(current_cht_num),
                    ::std::mem::replace(&mut current_cht_blocks,
                                        Vec::new()))?;
        }
        Ok(())
    }
    /// Build pairs for computing CHT.
    fn build_pairs<Header,
                   I>(cht_size: u64, cht_num: Header::Number, hashes: I)
     -> ClientResult<Vec<(Vec<u8>, Vec<u8>)>> where Header: HeaderT,
     I: IntoIterator<Item = ClientResult<Option<Header::Hash>>> {
        let start_num = start_number(cht_size, cht_num);
        let mut pairs = Vec::new();
        let mut hash_number = start_num;
        for hash in hashes.into_iter().take(cht_size as usize) {
            let hash =
                hash?.ok_or_else(||
                                     ClientError::from(ClientError::MissingHashRequiredForCHT(cht_num.as_(),
                                                                                              hash_number.as_())))?;
            pairs.push((encode_cht_key(hash_number).to_vec(),
                        encode_cht_value(hash)));
            hash_number += Header::Number::one();
        }
        if pairs.len() as u64 == cht_size {
            Ok(pairs)
        } else {
            Err(ClientError::MissingHashRequiredForCHT(cht_num.as_(),
                                                       hash_number.as_()))
        }
    }
    /// Get the starting block of a given CHT.
    /// CHT 0 includes block 1...SIZE,
    /// CHT 1 includes block SIZE + 1 ... 2*SIZE
    /// More generally: CHT N includes block (1 + N*SIZE)...((N+1)*SIZE).
    /// This is because the genesis hash is assumed to be known
    /// and including it would be redundant.
    pub fn start_number<N: SimpleArithmetic>(cht_size: u64, cht_num: N) -> N {
        (cht_num * As::sa(cht_size)) + N::one()
    }
    /// Get the ending block of a given CHT.
    pub fn end_number<N: SimpleArithmetic>(cht_size: u64, cht_num: N) -> N {
        (cht_num + N::one()) * As::sa(cht_size)
    }
    /// Convert a block number to a CHT number.
    /// Returns `None` for `block_num` == 0, `Some` otherwise.
    pub fn block_to_cht_number<N: SimpleArithmetic>(cht_size: u64,
                                                    block_num: N)
     -> Option<N> {
        if block_num == N::zero() {
            None
        } else { Some((block_num - N::one()) / As::sa(cht_size)) }
    }
    /// Convert header number into CHT key.
    pub fn encode_cht_key<N: As<u64>>(number: N) -> Vec<u8> {
        let number: u64 = number.as_();
        <[_]>::into_vec(box
                            [(number >> 56) as u8,
                             ((number >> 48) & 0xff) as u8,
                             ((number >> 40) & 0xff) as u8,
                             ((number >> 32) & 0xff) as u8,
                             ((number >> 24) & 0xff) as u8,
                             ((number >> 16) & 0xff) as u8,
                             ((number >> 8) & 0xff) as u8,
                             (number & 0xff) as u8])
    }
    /// Convert header hash into CHT value.
    fn encode_cht_value<Hash: AsRef<[u8]>>(hash: Hash) -> Vec<u8> {
        hash.as_ref().to_vec()
    }
    /// Convert CHT value into block header hash.
    pub fn decode_cht_value(value: &[u8]) -> Option<H256> {
        match value.len() {
            32 => Some(H256::from_slice(&value[0..32])),
            _ => None,
        }
    }
}
#[cfg(feature = "std")]
pub mod in_mem {
    //! In memory client backend
    use std::collections::HashMap;
    use std::sync::Arc;
    use parking_lot::RwLock;
    use primitives::{ChangesTrieConfiguration, storage::well_known_keys};
    use runtime_primitives::generic::BlockId;
    use runtime_primitives::traits::{Block as BlockT, Header as HeaderT, Zero,
                                     NumberFor, As, Digest, DigestItem};
    use runtime_primitives::{Justification, StorageOverlay,
                             ChildrenStorageOverlay};
    use state_machine::backend::{Backend as StateBackend, InMemory};
    use state_machine::{self, InMemoryChangesTrieStorage,
                        ChangesTrieAnchorBlockId};
    use hash_db::Hasher;
    use trie::MemoryDB;
    use consensus::well_known_cache_keys::Id as CacheKeyId;
    use crate::error;
    use crate::backend::{self, NewBlockState};
    use crate::light;
    use crate::leaves::LeafSet;
    use crate::blockchain::{self, BlockStatus, HeaderBackend};
    struct PendingBlock<B: BlockT> {
        block: StoredBlock<B>,
        state: NewBlockState,
    }
    #[structural_match]
    enum StoredBlock<B: BlockT> {
        Header(B::Header, Option<Justification>),
        Full(B, Option<Justification>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <B: ::std::cmp::PartialEq + BlockT> ::std::cmp::PartialEq for
     StoredBlock<B> where B::Header: ::std::cmp::PartialEq {
        #[inline]
        fn eq(&self, other: &StoredBlock<B>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&StoredBlock::Header(ref __self_0, ref __self_1),
                         &StoredBlock::Header(ref __arg_1_0, ref __arg_1_1))
                        =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1),
                        (&StoredBlock::Full(ref __self_0, ref __self_1),
                         &StoredBlock::Full(ref __arg_1_0, ref __arg_1_1)) =>
                        (*__self_0) == (*__arg_1_0) &&
                            (*__self_1) == (*__arg_1_1),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { false }
            }
        }
        #[inline]
        fn ne(&self, other: &StoredBlock<B>) -> bool {
            {
                let __self_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*self) }
                        as isize;
                let __arg_1_vi =
                    unsafe { ::std::intrinsics::discriminant_value(&*other) }
                        as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&StoredBlock::Header(ref __self_0, ref __self_1),
                         &StoredBlock::Header(ref __arg_1_0, ref __arg_1_1))
                        =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1),
                        (&StoredBlock::Full(ref __self_0, ref __self_1),
                         &StoredBlock::Full(ref __arg_1_0, ref __arg_1_1)) =>
                        (*__self_0) != (*__arg_1_0) ||
                            (*__self_1) != (*__arg_1_1),
                        _ => unsafe { ::std::intrinsics::unreachable() }
                    }
                } else { true }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <B: ::std::cmp::Eq + BlockT> ::std::cmp::Eq for StoredBlock<B> where
     B::Header: ::std::cmp::Eq {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<B::Header>;
                let _: ::std::cmp::AssertParamIsEq<Option<Justification>>;
                let _: ::std::cmp::AssertParamIsEq<B>;
                let _: ::std::cmp::AssertParamIsEq<Option<Justification>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <B: ::std::clone::Clone + BlockT> ::std::clone::Clone for
     StoredBlock<B> where B::Header: ::std::clone::Clone {
        #[inline]
        fn clone(&self) -> StoredBlock<B> {
            match (&*self,) {
                (&StoredBlock::Header(ref __self_0, ref __self_1),) =>
                StoredBlock::Header(::std::clone::Clone::clone(&(*__self_0)),
                                    ::std::clone::Clone::clone(&(*__self_1))),
                (&StoredBlock::Full(ref __self_0, ref __self_1),) =>
                StoredBlock::Full(::std::clone::Clone::clone(&(*__self_0)),
                                  ::std::clone::Clone::clone(&(*__self_1))),
            }
        }
    }
    impl <B: BlockT> StoredBlock<B> {
        fn new(header: B::Header, body: Option<Vec<B::Extrinsic>>,
               just: Option<Justification>) -> Self {
            match body {
                Some(body) => StoredBlock::Full(B::new(header, body), just),
                None => StoredBlock::Header(header, just),
            }
        }
        fn header(&self) -> &B::Header {
            match *self {
                StoredBlock::Header(ref h, _) => h,
                StoredBlock::Full(ref b, _) => b.header(),
            }
        }
        fn justification(&self) -> Option<&Justification> {
            match *self {
                StoredBlock::Header(_, ref j) | StoredBlock::Full(_, ref j) =>
                j.as_ref(),
            }
        }
        fn extrinsics(&self) -> Option<&[B::Extrinsic]> {
            match *self {
                StoredBlock::Header(_, _) => None,
                StoredBlock::Full(ref b, _) => Some(b.extrinsics()),
            }
        }
        fn into_inner(self)
         -> (B::Header, Option<Vec<B::Extrinsic>>, Option<Justification>) {
            match self {
                StoredBlock::Header(header, just) => (header, None, just),
                StoredBlock::Full(block, just) => {
                    let (header, body) = block.deconstruct();
                    (header, Some(body), just)
                }
            }
        }
    }
    struct BlockchainStorage<Block: BlockT> {
        blocks: HashMap<Block::Hash, StoredBlock<Block>>,
        hashes: HashMap<NumberFor<Block>, Block::Hash>,
        best_hash: Block::Hash,
        best_number: NumberFor<Block>,
        finalized_hash: Block::Hash,
        finalized_number: NumberFor<Block>,
        genesis_hash: Block::Hash,
        header_cht_roots: HashMap<NumberFor<Block>, Block::Hash>,
        changes_trie_cht_roots: HashMap<NumberFor<Block>, Block::Hash>,
        leaves: LeafSet<Block::Hash, NumberFor<Block>>,
        aux: HashMap<Vec<u8>, Vec<u8>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::clone::Clone + BlockT> ::std::clone::Clone for
     BlockchainStorage<Block> where Block::Hash: ::std::clone::Clone,
     Block::Hash: ::std::clone::Clone, Block::Hash: ::std::clone::Clone,
     Block::Hash: ::std::clone::Clone, Block::Hash: ::std::clone::Clone,
     Block::Hash: ::std::clone::Clone, Block::Hash: ::std::clone::Clone,
     Block::Hash: ::std::clone::Clone {
        #[inline]
        fn clone(&self) -> BlockchainStorage<Block> {
            match *self {
                BlockchainStorage {
                blocks: ref __self_0_0,
                hashes: ref __self_0_1,
                best_hash: ref __self_0_2,
                best_number: ref __self_0_3,
                finalized_hash: ref __self_0_4,
                finalized_number: ref __self_0_5,
                genesis_hash: ref __self_0_6,
                header_cht_roots: ref __self_0_7,
                changes_trie_cht_roots: ref __self_0_8,
                leaves: ref __self_0_9,
                aux: ref __self_0_10 } =>
                BlockchainStorage{blocks:
                                      ::std::clone::Clone::clone(&(*__self_0_0)),
                                  hashes:
                                      ::std::clone::Clone::clone(&(*__self_0_1)),
                                  best_hash:
                                      ::std::clone::Clone::clone(&(*__self_0_2)),
                                  best_number:
                                      ::std::clone::Clone::clone(&(*__self_0_3)),
                                  finalized_hash:
                                      ::std::clone::Clone::clone(&(*__self_0_4)),
                                  finalized_number:
                                      ::std::clone::Clone::clone(&(*__self_0_5)),
                                  genesis_hash:
                                      ::std::clone::Clone::clone(&(*__self_0_6)),
                                  header_cht_roots:
                                      ::std::clone::Clone::clone(&(*__self_0_7)),
                                  changes_trie_cht_roots:
                                      ::std::clone::Clone::clone(&(*__self_0_8)),
                                  leaves:
                                      ::std::clone::Clone::clone(&(*__self_0_9)),
                                  aux:
                                      ::std::clone::Clone::clone(&(*__self_0_10)),},
            }
        }
    }
    /// In-memory blockchain. Supports concurrent reads.
    pub struct Blockchain<Block: BlockT> {
        storage: Arc<RwLock<BlockchainStorage<Block>>>,
    }
    impl <Block: BlockT + Clone> Clone for Blockchain<Block> {
        fn clone(&self) -> Self {
            let storage = Arc::new(RwLock::new(self.storage.read().clone()));
            Blockchain{storage: storage.clone(),}
        }
    }
    impl <Block: BlockT> Blockchain<Block> {
        /// Get header hash of given block.
        pub fn id(&self, id: BlockId<Block>) -> Option<Block::Hash> {
            match id {
                BlockId::Hash(h) => Some(h),
                BlockId::Number(n) =>
                self.storage.read().hashes.get(&n).cloned(),
            }
        }
        /// Create new in-memory blockchain storage.
        pub fn new() -> Blockchain<Block> {
            let storage =
                Arc::new(RwLock::new(BlockchainStorage{blocks: HashMap::new(),
                                                       hashes: HashMap::new(),
                                                       best_hash:
                                                           Default::default(),
                                                       best_number:
                                                           Zero::zero(),
                                                       finalized_hash:
                                                           Default::default(),
                                                       finalized_number:
                                                           Zero::zero(),
                                                       genesis_hash:
                                                           Default::default(),
                                                       header_cht_roots:
                                                           HashMap::new(),
                                                       changes_trie_cht_roots:
                                                           HashMap::new(),
                                                       leaves: LeafSet::new(),
                                                       aux:
                                                           HashMap::new(),}));
            Blockchain{storage: storage.clone(),}
        }
        /// Insert a block header and associated data.
        pub fn insert(&self, hash: Block::Hash,
                      header: <Block as BlockT>::Header,
                      justification: Option<Justification>,
                      body: Option<Vec<<Block as BlockT>::Extrinsic>>,
                      new_state: NewBlockState) -> crate::error::Result<()> {
            let number = header.number().clone();
            if new_state.is_best() { self.apply_head(&header)?; }
            {
                let mut storage = self.storage.write();
                storage.leaves.import(hash.clone(), number.clone(),
                                      header.parent_hash().clone());
                storage.blocks.insert(hash.clone(),
                                      StoredBlock::new(header, body,
                                                       justification));
                if let NewBlockState::Final = new_state {
                    storage.finalized_hash = hash;
                    storage.finalized_number = number.clone();
                }
                if number == Zero::zero() { storage.genesis_hash = hash; }
            }
            Ok(())
        }
        /// Get total number of blocks.
        pub fn blocks_count(&self) -> usize {
            self.storage.read().blocks.len()
        }
        /// Compare this blockchain with another in-mem blockchain
        pub fn equals_to(&self, other: &Self) -> bool {
            self.canon_equals_to(other) &&
                self.storage.read().blocks == other.storage.read().blocks
        }
        /// Compare canonical chain to other canonical chain.
        pub fn canon_equals_to(&self, other: &Self) -> bool {
            let this = self.storage.read();
            let other = other.storage.read();
            this.hashes == other.hashes && this.best_hash == other.best_hash
                && this.best_number == other.best_number &&
                this.genesis_hash == other.genesis_hash
        }
        /// Insert header CHT root.
        pub fn insert_cht_root(&self, block: NumberFor<Block>,
                               cht_root: Block::Hash) {
            self.storage.write().header_cht_roots.insert(block, cht_root);
        }
        /// Set an existing block as head.
        pub fn set_head(&self, id: BlockId<Block>) -> error::Result<()> {
            let header =
                match self.header(id)? {
                    Some(h) => h,
                    None =>
                    return Err(error::Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                             &match (&id,)
                                                                                                                  {
                                                                                                                  (arg0,)
                                                                                                                  =>
                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                               ::std::fmt::Display::fmt)],
                                                                                                              })))),
                };
            self.apply_head(&header)
        }
        fn apply_head(&self, header: &<Block as BlockT>::Header)
         -> error::Result<()> {
            let hash = header.hash();
            let number = header.number();
            let best_tree_route =
                {
                    let best_hash = self.storage.read().best_hash;
                    if &best_hash == header.parent_hash() {
                        None
                    } else {
                        let route =
                            crate::blockchain::tree_route(self,
                                                          BlockId::Hash(best_hash),
                                                          BlockId::Hash(*header.parent_hash()))?;
                        Some(route)
                    }
                };
            let mut storage = self.storage.write();
            if let Some(tree_route) = best_tree_route {
                let enacted = tree_route.enacted();
                for entry in enacted {
                    storage.hashes.insert(entry.number, entry.hash);
                }
                for entry in tree_route.retracted().iter().skip(enacted.len())
                    {
                    storage.hashes.remove(&entry.number);
                }
            }
            storage.best_hash = hash.clone();
            storage.best_number = number.clone();
            storage.hashes.insert(number.clone(), hash.clone());
            Ok(())
        }
        fn finalize_header(&self, id: BlockId<Block>,
                           justification: Option<Justification>)
         -> error::Result<()> {
            let hash =
                match self.header(id)? {
                    Some(h) => h.hash(),
                    None =>
                    return Err(error::Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                             &match (&id,)
                                                                                                                  {
                                                                                                                  (arg0,)
                                                                                                                  =>
                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                               ::std::fmt::Display::fmt)],
                                                                                                              })))),
                };
            let mut storage = self.storage.write();
            storage.finalized_hash = hash;
            if justification.is_some() {
                let block =
                    storage.blocks.get_mut(&hash).expect("hash was fetched from a block in the db; qed");
                let block_justification =
                    match block {
                        StoredBlock::Header(_, ref mut j) |
                        StoredBlock::Full(_, ref mut j) => j,
                    };
                *block_justification = justification;
            }
            Ok(())
        }
        fn write_aux(&self, ops: Vec<(Vec<u8>, Option<Vec<u8>>)>) {
            let mut storage = self.storage.write();
            for (k, v) in ops {
                match v {
                    Some(v) => storage.aux.insert(k, v),
                    None => storage.aux.remove(&k),
                };
            }
        }
    }
    impl <Block: BlockT> HeaderBackend<Block> for Blockchain<Block> {
        fn header(&self, id: BlockId<Block>)
         -> error::Result<Option<<Block as BlockT>::Header>> {
            Ok(self.id(id).and_then(|hash|
                                        {
                                            self.storage.read().blocks.get(&hash).map(|b|
                                                                                          b.header().clone())
                                        }))
        }
        fn info(&self) -> error::Result<blockchain::Info<Block>> {
            let storage = self.storage.read();
            Ok(blockchain::Info{best_hash: storage.best_hash,
                                best_number: storage.best_number,
                                genesis_hash: storage.genesis_hash,
                                finalized_hash: storage.finalized_hash,
                                finalized_number: storage.finalized_number,})
        }
        fn status(&self, id: BlockId<Block>) -> error::Result<BlockStatus> {
            match self.id(id).map_or(false,
                                     |hash|
                                         self.storage.read().blocks.contains_key(&hash))
                {
                true => Ok(BlockStatus::InChain),
                false => Ok(BlockStatus::Unknown),
            }
        }
        fn number(&self, hash: Block::Hash)
         -> error::Result<Option<NumberFor<Block>>> {
            Ok(self.storage.read().blocks.get(&hash).map(|b|
                                                             *b.header().number()))
        }
        fn hash(&self, number: <<Block as BlockT>::Header as HeaderT>::Number)
         -> error::Result<Option<Block::Hash>> {
            Ok(self.id(BlockId::Number(number)))
        }
    }
    impl <Block: BlockT> blockchain::Backend<Block> for Blockchain<Block> {
        fn body(&self, id: BlockId<Block>)
         -> error::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {
            Ok(self.id(id).and_then(|hash|
                                        {
                                            self.storage.read().blocks.get(&hash).and_then(|b|
                                                                                               b.extrinsics().map(|x|
                                                                                                                      x.to_vec()))
                                        }))
        }
        fn justification(&self, id: BlockId<Block>)
         -> error::Result<Option<Justification>> {
            Ok(self.id(id).and_then(|hash|
                                        self.storage.read().blocks.get(&hash).and_then(|b|
                                                                                           b.justification().map(|x|
                                                                                                                     x.clone()))))
        }
        fn last_finalized(&self) -> error::Result<Block::Hash> {
            Ok(self.storage.read().finalized_hash.clone())
        }
        fn cache(&self) -> Option<Arc<blockchain::Cache<Block>>> { None }
        fn leaves(&self) -> error::Result<Vec<Block::Hash>> {
            Ok(self.storage.read().leaves.hashes())
        }
        fn children(&self, _parent_hash: Block::Hash)
         -> error::Result<Vec<Block::Hash>> {
            {
                ::std::rt::begin_panic("not yet implemented",
                                       &("core/client/src/in_mem.rs", 353u32,
                                         3u32))
            }
        }
    }
    impl <Block: BlockT> blockchain::ProvideCache<Block> for Blockchain<Block>
     {
        fn cache(&self) -> Option<Arc<blockchain::Cache<Block>>> { None }
    }
    impl <Block: BlockT> backend::AuxStore for Blockchain<Block> {
        fn insert_aux<'a, 'b: 'a, 'c: 'a, I: IntoIterator<Item =
                      &'a (&'c [u8], &'c [u8])>, D: IntoIterator<Item =
                      &'a &'b [u8]>>(&self, insert: I, delete: D)
         -> error::Result<()> {
            let mut storage = self.storage.write();
            for (k, v) in insert {
                storage.aux.insert(k.to_vec(), v.to_vec());
            }
            for k in delete { storage.aux.remove(*k); }
            Ok(())
        }
        fn get_aux(&self, key: &[u8]) -> error::Result<Option<Vec<u8>>> {
            Ok(self.storage.read().aux.get(key).cloned())
        }
    }
    impl <Block: BlockT> light::blockchain::Storage<Block> for
     Blockchain<Block> where Block::Hash: From<[u8; 32]> {
        fn import_header(&self, header: Block::Header,
                         _cache: HashMap<CacheKeyId, Vec<u8>>,
                         state: NewBlockState,
                         aux_ops: Vec<(Vec<u8>, Option<Vec<u8>>)>)
         -> error::Result<()> {
            let hash = header.hash();
            self.insert(hash, header, None, None, state)?;
            self.write_aux(aux_ops);
            Ok(())
        }
        fn set_head(&self, id: BlockId<Block>) -> error::Result<()> {
            Blockchain::set_head(self, id)
        }
        fn last_finalized(&self) -> error::Result<Block::Hash> {
            Ok(self.storage.read().finalized_hash.clone())
        }
        fn finalize_header(&self, id: BlockId<Block>) -> error::Result<()> {
            Blockchain::finalize_header(self, id, None)
        }
        fn header_cht_root(&self, _cht_size: u64, block: NumberFor<Block>)
         -> error::Result<Block::Hash> {
            self.storage.read().header_cht_roots.get(&block).cloned().ok_or_else(||
                                                                                     error::Error::Backend(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Header CHT for block ",
                                                                                                                                                                " not exists"],
                                                                                                                                                              &match (&block,)
                                                                                                                                                                   {
                                                                                                                                                                   (arg0,)
                                                                                                                                                                   =>
                                                                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                ::std::fmt::Display::fmt)],
                                                                                                                                                               }))))
        }
        fn changes_trie_cht_root(&self, _cht_size: u64,
                                 block: NumberFor<Block>)
         -> error::Result<Block::Hash> {
            self.storage.read().changes_trie_cht_roots.get(&block).cloned().ok_or_else(||
                                                                                           error::Error::Backend(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Changes trie CHT for block ",
                                                                                                                                                                      " not exists"],
                                                                                                                                                                    &match (&block,)
                                                                                                                                                                         {
                                                                                                                                                                         (arg0,)
                                                                                                                                                                         =>
                                                                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                      ::std::fmt::Display::fmt)],
                                                                                                                                                                     }))))
        }
        fn cache(&self) -> Option<Arc<blockchain::Cache<Block>>> { None }
    }
    /// In-memory operation.
    pub struct BlockImportOperation<Block: BlockT, H: Hasher> {
        pending_block: Option<PendingBlock<Block>>,
        pending_cache: HashMap<CacheKeyId, Vec<u8>>,
        old_state: InMemory<H>,
        new_state: Option<InMemory<H>>,
        changes_trie_update: Option<MemoryDB<H>>,
        aux: Vec<(Vec<u8>, Option<Vec<u8>>)>,
        finalized_blocks: Vec<(BlockId<Block>, Option<Justification>)>,
        set_head: Option<BlockId<Block>>,
    }
    impl <Block, H> backend::BlockImportOperation<Block, H> for
     BlockImportOperation<Block, H> where Block: BlockT, H: Hasher<Out =
     Block::Hash>, H::Out: Ord {
        type
        State
        =
        InMemory<H>;
        fn state(&self) -> error::Result<Option<&Self::State>> {
            Ok(Some(&self.old_state))
        }
        fn set_block_data(&mut self, header: <Block as BlockT>::Header,
                          body: Option<Vec<<Block as BlockT>::Extrinsic>>,
                          justification: Option<Justification>,
                          state: NewBlockState) -> error::Result<()> {
            if !self.pending_block.is_none() {
                {
                    ::std::rt::begin_panic("Only one block per operation is allowed",
                                           &("core/client/src/in_mem.rs",
                                             463u32, 3u32))
                }
            };
            self.pending_block =
                Some(PendingBlock{block:
                                      StoredBlock::new(header, body,
                                                       justification),
                                  state,});
            Ok(())
        }
        fn update_cache(&mut self, cache: HashMap<CacheKeyId, Vec<u8>>) {
            self.pending_cache = cache;
        }
        fn update_db_storage(&mut self,
                             update:
                                 <InMemory<H> as
                                 StateBackend<H>>::Transaction)
         -> error::Result<()> {
            self.new_state = Some(self.old_state.update(update));
            Ok(())
        }
        fn update_changes_trie(&mut self, update: MemoryDB<H>)
         -> error::Result<()> {
            self.changes_trie_update = Some(update);
            Ok(())
        }
        fn reset_storage(&mut self, top: StorageOverlay,
                         children: ChildrenStorageOverlay)
         -> error::Result<H::Out> {
            check_genesis_storage(&top, &children)?;
            let child_delta =
                children.into_iter().map(|(storage_key, child_overlay)|
                                             (storage_key,
                                              child_overlay.into_iter().map(|(k,
                                                                              v)|
                                                                                (k,
                                                                                 Some(v)))));
            let (root, transaction) =
                self.old_state.full_storage_root(top.into_iter().map(|(k, v)|
                                                                         (k,
                                                                          Some(v))),
                                                 child_delta);
            self.new_state = Some(InMemory::from(transaction));
            Ok(root)
        }
        fn insert_aux<I>(&mut self, ops: I) -> error::Result<()> where
         I: IntoIterator<Item = (Vec<u8>, Option<Vec<u8>>)> {
            self.aux.append(&mut ops.into_iter().collect());
            Ok(())
        }
        fn update_storage(&mut self, _update: Vec<(Vec<u8>, Option<Vec<u8>>)>)
         -> error::Result<()> {
            Ok(())
        }
        fn mark_finalized(&mut self, block: BlockId<Block>,
                          justification: Option<Justification>)
         -> error::Result<()> {
            self.finalized_blocks.push((block, justification));
            Ok(())
        }
        fn mark_head(&mut self, block: BlockId<Block>) -> error::Result<()> {
            if !self.pending_block.is_none() {
                {
                    ::std::rt::begin_panic("Only one set block per operation is allowed",
                                           &("core/client/src/in_mem.rs",
                                             518u32, 3u32))
                }
            };
            self.set_head = Some(block);
            Ok(())
        }
    }
    /// In-memory backend. Keeps all states and blocks in memory. Useful for testing.
    pub struct Backend<Block, H> where Block: BlockT, H: Hasher<Out =
               Block::Hash>, H::Out: Ord {
        states: RwLock<HashMap<Block::Hash, InMemory<H>>>,
        changes_trie_storage: ChangesTrieStorage<H>,
        blockchain: Blockchain<Block>,
    }
    impl <Block, H> Backend<Block, H> where Block: BlockT, H: Hasher<Out =
     Block::Hash>, H::Out: Ord {
        /// Create a new instance of in-mem backend.
        pub fn new() -> Backend<Block, H> {
            Backend{states: RwLock::new(HashMap::new()),
                    changes_trie_storage:
                        ChangesTrieStorage(InMemoryChangesTrieStorage::new()),
                    blockchain: Blockchain::new(),}
        }
    }
    impl <Block, H> backend::AuxStore for Backend<Block, H> where
     Block: BlockT, H: Hasher<Out = Block::Hash>, H::Out: Ord {
        fn insert_aux<'a, 'b: 'a, 'c: 'a, I: IntoIterator<Item =
                      &'a (&'c [u8], &'c [u8])>, D: IntoIterator<Item =
                      &'a &'b [u8]>>(&self, insert: I, delete: D)
         -> error::Result<()> {
            self.blockchain.insert_aux(insert, delete)
        }
        fn get_aux(&self, key: &[u8]) -> error::Result<Option<Vec<u8>>> {
            self.blockchain.get_aux(key)
        }
    }
    impl <Block, H> backend::Backend<Block, H> for Backend<Block, H> where
     Block: BlockT, H: Hasher<Out = Block::Hash>, H::Out: Ord {
        type
        BlockImportOperation
        =
        BlockImportOperation<Block, H>;
        type
        Blockchain
        =
        Blockchain<Block>;
        type
        State
        =
        InMemory<H>;
        type
        ChangesTrieStorage
        =
        ChangesTrieStorage<H>;
        fn begin_operation(&self)
         -> error::Result<Self::BlockImportOperation> {
            let old_state = self.state_at(BlockId::Hash(Default::default()))?;
            Ok(BlockImportOperation{pending_block: None,
                                    pending_cache: Default::default(),
                                    old_state,
                                    new_state: None,
                                    changes_trie_update: None,
                                    aux: Default::default(),
                                    finalized_blocks: Default::default(),
                                    set_head: None,})
        }
        fn begin_state_operation(&self,
                                 operation: &mut Self::BlockImportOperation,
                                 block: BlockId<Block>) -> error::Result<()> {
            operation.old_state = self.state_at(block)?;
            Ok(())
        }
        fn commit_operation(&self, operation: Self::BlockImportOperation)
         -> error::Result<()> {
            if !operation.finalized_blocks.is_empty() {
                for (block, justification) in operation.finalized_blocks {
                    self.blockchain.finalize_header(block, justification)?;
                }
            }
            if let Some(pending_block) = operation.pending_block {
                let old_state = &operation.old_state;
                let (header, body, justification) =
                    pending_block.block.into_inner();
                let hash = header.hash();
                self.states.write().insert(hash,
                                           operation.new_state.unwrap_or_else(||
                                                                                  old_state.clone()));
                let changes_trie_root =
                    header.digest().log(DigestItem::as_changes_trie_root).cloned();
                if let Some(changes_trie_root) = changes_trie_root {
                    if let Some(changes_trie_update) =
                           operation.changes_trie_update {
                        let changes_trie_root: H::Out =
                            changes_trie_root.into();
                        self.changes_trie_storage.0.insert(header.number().as_(),
                                                           changes_trie_root,
                                                           changes_trie_update);
                    }
                }
                self.blockchain.insert(hash, header, justification, body,
                                       pending_block.state)?;
            }
            if !operation.aux.is_empty() {
                self.blockchain.write_aux(operation.aux);
            }
            if let Some(set_head) = operation.set_head {
                self.blockchain.set_head(set_head)?;
            }
            Ok(())
        }
        fn finalize_block(&self, block: BlockId<Block>,
                          justification: Option<Justification>)
         -> error::Result<()> {
            self.blockchain.finalize_header(block, justification)
        }
        fn blockchain(&self) -> &Self::Blockchain { &self.blockchain }
        fn used_state_cache_size(&self) -> Option<usize> { None }
        fn changes_trie_storage(&self) -> Option<&Self::ChangesTrieStorage> {
            Some(&self.changes_trie_storage)
        }
        fn state_at(&self, block: BlockId<Block>)
         -> error::Result<Self::State> {
            match block {
                BlockId::Hash(h) if h == Default::default() => {
                    return Ok(Self::State::default());
                }
                _ => { }
            }
            match self.blockchain.id(block).and_then(|id|
                                                         self.states.read().get(&id).cloned())
                {
                Some(state) => Ok(state),
                None =>
                Err(error::Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                  &match (&block,)
                                                                                                       {
                                                                                                       (arg0,)
                                                                                                       =>
                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                    ::std::fmt::Display::fmt)],
                                                                                                   })))),
            }
        }
        fn revert(&self, _n: NumberFor<Block>)
         -> error::Result<NumberFor<Block>> {
            Ok(As::sa(0))
        }
    }
    impl <Block, H> backend::LocalBackend<Block, H> for Backend<Block, H>
     where Block: BlockT, H: Hasher<Out = Block::Hash>, H::Out: Ord {
    }
    impl <Block, H> backend::RemoteBackend<Block, H> for Backend<Block, H>
     where Block: BlockT, H: Hasher<Out = Block::Hash>, H::Out: Ord {
        fn is_local_state_available(&self, block: &BlockId<Block>) -> bool {
            self.blockchain.expect_block_number_from_id(block).map(|num|
                                                                       num.is_zero()).unwrap_or(false)
        }
    }
    /// Prunable in-memory changes trie storage.
    pub struct ChangesTrieStorage<H: Hasher>(InMemoryChangesTrieStorage<H>);
    impl <H: Hasher> backend::PrunableStateChangesTrieStorage<H> for
     ChangesTrieStorage<H> {
        fn oldest_changes_trie_block(&self,
                                     _config: &ChangesTrieConfiguration,
                                     _best_finalized: u64) -> u64 {
            0
        }
    }
    impl <H: Hasher> state_machine::ChangesTrieRootsStorage<H> for
     ChangesTrieStorage<H> {
        fn root(&self, anchor: &ChangesTrieAnchorBlockId<H::Out>, block: u64)
         -> Result<Option<H::Out>, String> {
            self.0.root(anchor, block)
        }
    }
    impl <H: Hasher> state_machine::ChangesTrieStorage<H> for
     ChangesTrieStorage<H> {
        fn get(&self, key: &H::Out, prefix: &[u8])
         -> Result<Option<state_machine::DBValue>, String> {
            self.0.get(key, prefix)
        }
    }
    /// Check that genesis storage is valid.
    pub fn check_genesis_storage(top: &StorageOverlay,
                                 children: &ChildrenStorageOverlay)
     -> error::Result<()> {
        if top.iter().any(|(k, _)| well_known_keys::is_child_storage_key(k)) {
            return Err(error::Error::GenesisInvalid.into());
        }
        if children.keys().any(|child_key|
                                   !well_known_keys::is_child_storage_key(&child_key))
           {
            return Err(error::Error::GenesisInvalid.into());
        }
        Ok(())
    }
}
#[cfg(feature = "std")]
pub mod genesis {
    //! Tool for creating the genesis block.
    use runtime_primitives::traits::{Block as BlockT, Header as HeaderT, Hash
                                     as HashT, Zero};
    /// Create a genesis block, given the initial storage.
    pub fn construct_genesis_block<Block: BlockT>(state_root: Block::Hash)
     -> Block {
        let extrinsics_root =
            <<<Block as BlockT>::Header as HeaderT>::Hashing as
                HashT>::trie_root(::std::iter::empty::<(&[u8], &[u8])>());
        Block::new(<<Block as BlockT>::Header as
                       HeaderT>::new(Zero::zero(), extrinsics_root,
                                     state_root, Default::default(),
                                     Default::default()), Default::default())
    }
}
pub mod block_builder {
    //! Utility struct to build a block.
    #[cfg(feature = "std")]
    mod block_builder {
        use super::api::BlockBuilder as BlockBuilderApi;
        use std::vec::Vec;
        use parity_codec::Encode;
        use runtime_primitives::ApplyOutcome;
        use runtime_primitives::generic::BlockId;
        use runtime_primitives::traits::{Header as HeaderT, Hash, Block as
                                         BlockT, One, HashFor,
                                         ProvideRuntimeApi, ApiRef};
        use primitives::{H256, ExecutionContext};
        use crate::blockchain::HeaderBackend;
        use crate::runtime_api::{Core, ApiExt};
        use crate::error;
        /// Utility for building new (valid) blocks from a stream of extrinsics.
        pub struct BlockBuilder<'a, Block, A: ProvideRuntimeApi> where
                   Block: BlockT {
            header: <Block as BlockT>::Header,
            extrinsics: Vec<<Block as BlockT>::Extrinsic>,
            api: ApiRef<'a, A::Api>,
            block_id: BlockId<Block>,
        }
        impl <'a, Block, A> BlockBuilder<'a, Block, A> where
         Block: BlockT<Hash = H256>, A: ProvideRuntimeApi +
         HeaderBackend<Block> + 'a, A::Api: BlockBuilderApi<Block> {
            /// Create a new instance of builder from the given client, building on the latest block.
            pub fn new(api: &'a A) -> error::Result<Self> {
                api.info().and_then(|i|
                                        Self::at_block(&BlockId::Hash(i.best_hash),
                                                       api, false))
            }
            /// Create a new instance of builder from the given client using a
            /// particular block's ID to build upon with optional proof recording enabled.
            ///
            /// While proof recording is enabled, all accessed trie nodes are saved.
            /// These recorded trie nodes can be used by a third party to proof the
            /// output of this block builder without having access to the full storage.
            pub fn at_block(block_id: &BlockId<Block>, api: &'a A,
                            proof_recording: bool) -> error::Result<Self> {
                let number =
                    api.block_number_from_id(block_id)?.ok_or_else(||
                                                                       error::Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                     &match (&block_id,)
                                                                                                                                                          {
                                                                                                                                                          (arg0,)
                                                                                                                                                          =>
                                                                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                       ::std::fmt::Display::fmt)],
                                                                                                                                                      }))))?
                        + One::one();
                let parent_hash =
                    api.block_hash_from_id(block_id)?.ok_or_else(||
                                                                     error::Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                   &match (&block_id,)
                                                                                                                                                        {
                                                                                                                                                        (arg0,)
                                                                                                                                                        =>
                                                                                                                                                        [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                     ::std::fmt::Display::fmt)],
                                                                                                                                                    }))))?;
                let header =
                    <<Block as BlockT>::Header as
                        HeaderT>::new(number, Default::default(),
                                      Default::default(), parent_hash,
                                      Default::default());
                let mut api = api.runtime_api();
                if proof_recording { api.record_proof(); }
                api.initialize_block_with_context(block_id,
                                                  ExecutionContext::BlockConstruction,
                                                  &header)?;
                Ok(BlockBuilder{header,
                                extrinsics: Vec::new(),
                                api,
                                block_id: *block_id,})
            }
            /// Push onto the block's list of extrinsics.
            ///
            /// This will ensure the extrinsic can be validly executed (by executing it);
            pub fn push(&mut self, xt: <Block as BlockT>::Extrinsic)
             -> error::Result<()> {
                let block_id = &self.block_id;
                let extrinsics = &mut self.extrinsics;
                self.api.map_api_result(|api|
                                            {
                                                match api.apply_extrinsic_with_context(block_id,
                                                                                       ExecutionContext::BlockConstruction,
                                                                                       xt.clone())?
                                                    {
                                                    Ok(ApplyOutcome::Success)
                                                    | Ok(ApplyOutcome::Fail)
                                                    => {
                                                        extrinsics.push(xt);
                                                        Ok(())
                                                    }
                                                    Err(e) => {
                                                        Err(error::Error::ApplyExtrinsicFailed(e))
                                                    }
                                                }
                                            })
            }
            /// Consume the builder to return a valid `Block` containing all pushed extrinsics.
            pub fn bake(mut self) -> error::Result<Block> {
                self.bake_impl()?;
                Ok(<Block as BlockT>::new(self.header, self.extrinsics))
            }
            fn bake_impl(&mut self) -> error::Result<()> {
                self.header =
                    self.api.finalize_block_with_context(&self.block_id,
                                                         ExecutionContext::BlockConstruction)?;
                if true {
                    {
                        {
                            match (&self.header.extrinsics_root().clone(),
                                   &HashFor::<Block>::ordered_trie_root(self.extrinsics.iter().map(Encode::encode)))
                                {
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
                                                                       &("core/client/src/block_builder/block_builder.rs",
                                                                         129u32,
                                                                         3u32))
                                        }
                                    }
                                }
                            }
                        }
                    };
                };
                Ok(())
            }
            /// Consume the builder to return a valid `Block` containing all pushed extrinsics
            /// and the generated proof.
            ///
            /// The proof will be `Some(_)`, if proof recording was enabled while creating
            /// the block builder.
            pub fn bake_and_extract_proof(mut self)
             -> error::Result<(Block, Option<Vec<Vec<u8>>>)> {
                self.bake_impl()?;
                let proof = self.api.extract_proof();
                Ok((<Block as BlockT>::new(self.header, self.extrinsics),
                    proof))
            }
        }
    }
    #[cfg(feature = "std")]
    pub use self::block_builder::*;
    pub mod api {
        //! The runtime api for building blocks.
        use runtime_primitives::{traits::Block as BlockT, ApplyResult};
        use rstd::vec::Vec;
        use sr_api_macros::decl_runtime_apis;
        pub use inherents::{InherentData, CheckInherentsResult};
        #[doc(hidden)]
        #[allow(dead_code)]
        #[allow(deprecated)]
        pub mod runtime_decl_for_BlockBuilder {
            use super::*;
            #[doc =
                  " The `BlockBuilder` api trait that provides required functions for building a block for a runtime."]
            pub trait BlockBuilder<Block: crate::runtime_api::BlockT> {
                #[doc = " Apply the given extrinsics."]
                fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic)
                -> ApplyResult;
                #[doc = " Finish the current block."]
                fn finalize_block()
                -> <Block as BlockT>::Header;
                #[doc =
                      " Generate inherent extrinsics. The inherent data will vary from chain to chain."]
                fn inherent_extrinsics(inherent: InherentData)
                -> Vec<<Block as BlockT>::Extrinsic>;
                #[doc =
                      " Check that the inherents are valid. The inherent data will vary from chain to chain."]
                fn check_inherents(block: Block, data: InherentData)
                -> CheckInherentsResult;
                #[doc = " Generate a random seed."]
                fn random_seed()
                -> <Block as BlockT>::Hash;
            }
            pub const VERSION: u32 = 3u32;
            pub const ID: [u8; 8] =
                [64u8, 254u8, 58u8, 212u8, 1u8, 248u8, 149u8, 154u8];
            #[cfg(any(feature = "std", test))]
            fn convert_between_block_types<I: crate::runtime_api::Encode,
                                           R: crate::runtime_api::Decode>(input:
                                                                              &I,
                                                                          error_desc:
                                                                              &'static str)
             -> ::std::result::Result<R, &'static str> {
                <R as
                    crate::runtime_api::Decode>::decode(&mut &crate::runtime_api::Encode::encode(input)[..]).ok_or_else(||
                                                                                                                            error_desc)
            }
            #[cfg(any(feature = "std", test))]
            pub fn apply_extrinsic_native_call_generator<'a,
                                                         ApiImpl: BlockBuilder<Block>,
                                                         NodeBlock: crate::runtime_api::BlockT,
                                                         Block: crate::runtime_api::BlockT +
                                                         'a>(extrinsic:
                                                                 <NodeBlock as
                                                                 BlockT>::Extrinsic)
             ->
                 impl FnOnce() ->
                 ::std::result::Result<ApplyResult, &'static str> + 'a {
                move ||
                    {
                        let extrinsic: <Block as BlockT>::Extrinsic =
                            convert_between_block_types(&extrinsic,
                                                        "Could not convert parameter `extrinsic` between node and runtime!")?;
                        let res = ApiImpl::apply_extrinsic(extrinsic);
                        Ok(res)
                    }
            }
            #[cfg(any(feature = "std", test))]
            pub fn finalize_block_native_call_generator<'a,
                                                        ApiImpl: BlockBuilder<Block>,
                                                        NodeBlock: crate::runtime_api::BlockT,
                                                        Block: crate::runtime_api::BlockT +
                                                        'a>()
             ->
                 impl FnOnce() ->
                 ::std::result::Result<<NodeBlock as BlockT>::Header,
                                       &'static str> + 'a {
                move ||
                    {
                        let res = ApiImpl::finalize_block();
                        convert_between_block_types(&res,
                                                    "Could not convert return value from runtime to node!")
                    }
            }
            #[cfg(any(feature = "std", test))]
            pub fn inherent_extrinsics_native_call_generator<'a,
                                                             ApiImpl: BlockBuilder<Block>,
                                                             NodeBlock: crate::runtime_api::BlockT,
                                                             Block: crate::runtime_api::BlockT +
                                                             'a>(inherent:
                                                                     InherentData)
             ->
                 impl FnOnce() ->
                 ::std::result::Result<Vec<<NodeBlock as BlockT>::Extrinsic>,
                                       &'static str> + 'a {
                move ||
                    {
                        let res = ApiImpl::inherent_extrinsics(inherent);
                        convert_between_block_types(&res,
                                                    "Could not convert return value from runtime to node!")
                    }
            }
            #[cfg(any(feature = "std", test))]
            pub fn check_inherents_native_call_generator<'a,
                                                         ApiImpl: BlockBuilder<Block>,
                                                         NodeBlock: crate::runtime_api::BlockT,
                                                         Block: crate::runtime_api::BlockT +
                                                         'a>(block: NodeBlock,
                                                             data:
                                                                 InherentData)
             ->
                 impl FnOnce() ->
                 ::std::result::Result<CheckInherentsResult, &'static str> +
                 'a {
                move ||
                    {
                        let block: Block =
                            convert_between_block_types(&block,
                                                        "Could not convert parameter `block` between node and runtime!")?;
                        let res = ApiImpl::check_inherents(block, data);
                        Ok(res)
                    }
            }
            #[cfg(any(feature = "std", test))]
            pub fn random_seed_native_call_generator<'a,
                                                     ApiImpl: BlockBuilder<Block>,
                                                     NodeBlock: crate::runtime_api::BlockT,
                                                     Block: crate::runtime_api::BlockT +
                                                     'a>()
             ->
                 impl FnOnce() ->
                 ::std::result::Result<<NodeBlock as BlockT>::Hash,
                                       &'static str> + 'a {
                move ||
                    {
                        let res = ApiImpl::random_seed();
                        convert_between_block_types(&res,
                                                    "Could not convert return value from runtime to node!")
                    }
            }
            #[cfg(any(feature = "std", test))]
            pub fn apply_extrinsic_call_api_at<R: crate::runtime_api::Encode +
                                               crate::runtime_api::Decode +
                                               PartialEq, NC: FnOnce() ->
                                               ::std::result::Result<R,
                                                                     &'static str> +
                                               ::std::panic::UnwindSafe,
                                               Block: crate::runtime_api::BlockT,
                                               T: crate::runtime_api::CallRuntimeAt<Block>,
                                               C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                                       &T,
                                                                                   core_api:
                                                                                       &C,
                                                                                   at:
                                                                                       &crate::runtime_api::BlockId<Block>,
                                                                                   args:
                                                                                       Vec<u8>,
                                                                                   changes:
                                                                                       &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                                   initialized_block:
                                                                                       &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                                   native_call:
                                                                                       Option<NC>,
                                                                                   context:
                                                                                       crate::runtime_api::ExecutionContext,
                                                                                   recorder:
                                                                                       &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
             -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
                let version = call_runtime_at.runtime_version_at(at)?;
                use crate::runtime_api::InitializeBlock;
                let initialize_block =
                    if false {
                        InitializeBlock::Skip
                    } else { InitializeBlock::Do(&initialized_block) };
                let update_initialized_block = || ();
                let ret =
                    call_runtime_at.call_api_at(core_api, at,
                                                "BlockBuilder_apply_extrinsic",
                                                args, changes,
                                                initialize_block, native_call,
                                                context, recorder)?;
                update_initialized_block();
                Ok(ret)
            }
            #[cfg(any(feature = "std", test))]
            pub fn finalize_block_call_api_at<R: crate::runtime_api::Encode +
                                              crate::runtime_api::Decode +
                                              PartialEq, NC: FnOnce() ->
                                              ::std::result::Result<R,
                                                                    &'static str> +
                                              ::std::panic::UnwindSafe,
                                              Block: crate::runtime_api::BlockT,
                                              T: crate::runtime_api::CallRuntimeAt<Block>,
                                              C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                                      &T,
                                                                                  core_api:
                                                                                      &C,
                                                                                  at:
                                                                                      &crate::runtime_api::BlockId<Block>,
                                                                                  args:
                                                                                      Vec<u8>,
                                                                                  changes:
                                                                                      &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                                  initialized_block:
                                                                                      &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                                  native_call:
                                                                                      Option<NC>,
                                                                                  context:
                                                                                      crate::runtime_api::ExecutionContext,
                                                                                  recorder:
                                                                                      &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
             -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
                let version = call_runtime_at.runtime_version_at(at)?;
                use crate::runtime_api::InitializeBlock;
                let initialize_block =
                    if false {
                        InitializeBlock::Skip
                    } else { InitializeBlock::Do(&initialized_block) };
                let update_initialized_block = || ();
                if version.apis.iter().any(|(s, v)| { s == &ID && *v < 3u32 })
                   {
                    let ret =
                        call_runtime_at.call_api_at::<R, fn() -> _,
                                                      _>(core_api, at,
                                                         "BlockBuilder_finalise_block",
                                                         args, changes,
                                                         initialize_block,
                                                         None, context,
                                                         recorder)?;
                    update_initialized_block();
                    return Ok(ret);
                }
                let ret =
                    call_runtime_at.call_api_at(core_api, at,
                                                "BlockBuilder_finalize_block",
                                                args, changes,
                                                initialize_block, native_call,
                                                context, recorder)?;
                update_initialized_block();
                Ok(ret)
            }
            #[cfg(any(feature = "std", test))]
            pub fn inherent_extrinsics_call_api_at<R: crate::runtime_api::Encode +
                                                   crate::runtime_api::Decode +
                                                   PartialEq, NC: FnOnce() ->
                                                   ::std::result::Result<R,
                                                                         &'static str> +
                                                   ::std::panic::UnwindSafe,
                                                   Block: crate::runtime_api::BlockT,
                                                   T: crate::runtime_api::CallRuntimeAt<Block>,
                                                   C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                                           &T,
                                                                                       core_api:
                                                                                           &C,
                                                                                       at:
                                                                                           &crate::runtime_api::BlockId<Block>,
                                                                                       args:
                                                                                           Vec<u8>,
                                                                                       changes:
                                                                                           &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                                       initialized_block:
                                                                                           &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                                       native_call:
                                                                                           Option<NC>,
                                                                                       context:
                                                                                           crate::runtime_api::ExecutionContext,
                                                                                       recorder:
                                                                                           &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
             -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
                let version = call_runtime_at.runtime_version_at(at)?;
                use crate::runtime_api::InitializeBlock;
                let initialize_block =
                    if false {
                        InitializeBlock::Skip
                    } else { InitializeBlock::Do(&initialized_block) };
                let update_initialized_block = || ();
                let ret =
                    call_runtime_at.call_api_at(core_api, at,
                                                "BlockBuilder_inherent_extrinsics",
                                                args, changes,
                                                initialize_block, native_call,
                                                context, recorder)?;
                update_initialized_block();
                Ok(ret)
            }
            #[cfg(any(feature = "std", test))]
            pub fn check_inherents_call_api_at<R: crate::runtime_api::Encode +
                                               crate::runtime_api::Decode +
                                               PartialEq, NC: FnOnce() ->
                                               ::std::result::Result<R,
                                                                     &'static str> +
                                               ::std::panic::UnwindSafe,
                                               Block: crate::runtime_api::BlockT,
                                               T: crate::runtime_api::CallRuntimeAt<Block>,
                                               C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                                       &T,
                                                                                   core_api:
                                                                                       &C,
                                                                                   at:
                                                                                       &crate::runtime_api::BlockId<Block>,
                                                                                   args:
                                                                                       Vec<u8>,
                                                                                   changes:
                                                                                       &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                                   initialized_block:
                                                                                       &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                                   native_call:
                                                                                       Option<NC>,
                                                                                   context:
                                                                                       crate::runtime_api::ExecutionContext,
                                                                                   recorder:
                                                                                       &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
             -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
                let version = call_runtime_at.runtime_version_at(at)?;
                use crate::runtime_api::InitializeBlock;
                let initialize_block =
                    if false {
                        InitializeBlock::Skip
                    } else { InitializeBlock::Do(&initialized_block) };
                let update_initialized_block = || ();
                let ret =
                    call_runtime_at.call_api_at(core_api, at,
                                                "BlockBuilder_check_inherents",
                                                args, changes,
                                                initialize_block, native_call,
                                                context, recorder)?;
                update_initialized_block();
                Ok(ret)
            }
            #[cfg(any(feature = "std", test))]
            pub fn random_seed_call_api_at<R: crate::runtime_api::Encode +
                                           crate::runtime_api::Decode +
                                           PartialEq, NC: FnOnce() ->
                                           ::std::result::Result<R,
                                                                 &'static str> +
                                           ::std::panic::UnwindSafe,
                                           Block: crate::runtime_api::BlockT,
                                           T: crate::runtime_api::CallRuntimeAt<Block>,
                                           C: crate::runtime_api::Core<Block>>(call_runtime_at:
                                                                                   &T,
                                                                               core_api:
                                                                                   &C,
                                                                               at:
                                                                                   &crate::runtime_api::BlockId<Block>,
                                                                               args:
                                                                                   Vec<u8>,
                                                                               changes:
                                                                                   &std::cell::RefCell<crate::runtime_api::OverlayedChanges>,
                                                                               initialized_block:
                                                                                   &std::cell::RefCell<Option<crate::runtime_api::BlockId<Block>>>,
                                                                               native_call:
                                                                                   Option<NC>,
                                                                               context:
                                                                                   crate::runtime_api::ExecutionContext,
                                                                               recorder:
                                                                                   &Option<std::rc::Rc<std::cell::RefCell<crate::runtime_api::ProofRecorder<Block>>>>)
             -> crate::error::Result<crate::runtime_api::NativeOrEncoded<R>> {
                let version = call_runtime_at.runtime_version_at(at)?;
                use crate::runtime_api::InitializeBlock;
                let initialize_block =
                    if false {
                        InitializeBlock::Skip
                    } else { InitializeBlock::Do(&initialized_block) };
                let update_initialized_block = || ();
                let ret =
                    call_runtime_at.call_api_at(core_api, at,
                                                "BlockBuilder_random_seed",
                                                args, changes,
                                                initialize_block, native_call,
                                                context, recorder)?;
                update_initialized_block();
                Ok(ret)
            }
        }
        #[doc =
              " The `BlockBuilder` api trait that provides required functions for building a block for a runtime."]
        #[cfg(any(feature = "std", test))]
        pub trait BlockBuilder<Block: crate::runtime_api::BlockT>: crate::runtime_api::Core<Block> {
            #[doc = " Apply the given extrinsics."]
            fn apply_extrinsic(&self, at: &crate::runtime_api::BlockId<Block>,
                               extrinsic: <Block as BlockT>::Extrinsic)
             -> ::std::result::Result<ApplyResult, crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&(&extrinsic));
                self.BlockBuilder_apply_extrinsic_runtime_api_impl(at,
                                                                   crate::runtime_api::ExecutionContext::Other,
                                                                   Some((extrinsic)),
                                                                   runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                 match r
                                                                                                                     {
                                                                                                                     crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                     =>
                                                                                                                     {
                                                                                                                         Ok(n)
                                                                                                                     }
                                                                                                                     crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                     =>
                                                                                                                     {
                                                                                                                         <ApplyResult
                                                                                                                             as
                                                                                                                             crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                             crate::error::Error::CallResultDecode("apply_extrinsic").into())
                                                                                                                     }
                                                                                                                 })
            }
            #[doc = " Apply the given extrinsics."]
            fn apply_extrinsic_with_context(&self,
                                            at:
                                                &crate::runtime_api::BlockId<Block>,
                                            context:
                                                crate::runtime_api::ExecutionContext,
                                            extrinsic:
                                                <Block as BlockT>::Extrinsic)
             -> ::std::result::Result<ApplyResult, crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&(&extrinsic));
                self.BlockBuilder_apply_extrinsic_runtime_api_impl(at,
                                                                   context,
                                                                   Some((extrinsic)),
                                                                   runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                 match r
                                                                                                                     {
                                                                                                                     crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                     =>
                                                                                                                     {
                                                                                                                         Ok(n)
                                                                                                                     }
                                                                                                                     crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                     =>
                                                                                                                     {
                                                                                                                         <ApplyResult
                                                                                                                             as
                                                                                                                             crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                             crate::error::Error::CallResultDecode("apply_extrinsic").into())
                                                                                                                     }
                                                                                                                 })
            }
            #[doc(hidden)]
            fn BlockBuilder_apply_extrinsic_runtime_api_impl(&self,
                                                             at:
                                                                 &crate::runtime_api::BlockId<Block>,
                                                             context:
                                                                 crate::runtime_api::ExecutionContext,
                                                             params:
                                                                 Option<(<Block
                                                                         as
                                                                         BlockT>::Extrinsic)>,
                                                             params_encoded:
                                                                 Vec<u8>)
            ->
                crate::error::Result<crate::runtime_api::NativeOrEncoded<ApplyResult>>;
            #[doc = " Finish the current block."]
            fn finalize_block(&self, at: &crate::runtime_api::BlockId<Block>)
             ->
                 ::std::result::Result<<Block as BlockT>::Header,
                                       crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&());
                self.BlockBuilder_finalize_block_runtime_api_impl(at,
                                                                  crate::runtime_api::ExecutionContext::Other,
                                                                  Some(()),
                                                                  runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                match r
                                                                                                                    {
                                                                                                                    crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                    =>
                                                                                                                    {
                                                                                                                        Ok(n)
                                                                                                                    }
                                                                                                                    crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                    =>
                                                                                                                    {
                                                                                                                        <<Block
                                                                                                                         as
                                                                                                                         BlockT>::Header
                                                                                                                            as
                                                                                                                            crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                            crate::error::Error::CallResultDecode("finalize_block").into())
                                                                                                                    }
                                                                                                                })
            }
            #[doc = " Finish the current block."]
            fn finalize_block_with_context(&self,
                                           at:
                                               &crate::runtime_api::BlockId<Block>,
                                           context:
                                               crate::runtime_api::ExecutionContext)
             ->
                 ::std::result::Result<<Block as BlockT>::Header,
                                       crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&());
                self.BlockBuilder_finalize_block_runtime_api_impl(at, context,
                                                                  Some(()),
                                                                  runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                match r
                                                                                                                    {
                                                                                                                    crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                    =>
                                                                                                                    {
                                                                                                                        Ok(n)
                                                                                                                    }
                                                                                                                    crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                    =>
                                                                                                                    {
                                                                                                                        <<Block
                                                                                                                         as
                                                                                                                         BlockT>::Header
                                                                                                                            as
                                                                                                                            crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                            crate::error::Error::CallResultDecode("finalize_block").into())
                                                                                                                    }
                                                                                                                })
            }
            #[doc(hidden)]
            fn BlockBuilder_finalize_block_runtime_api_impl(&self,
                                                            at:
                                                                &crate::runtime_api::BlockId<Block>,
                                                            context:
                                                                crate::runtime_api::ExecutionContext,
                                                            params:
                                                                Option<()>,
                                                            params_encoded:
                                                                Vec<u8>)
            ->
                crate::error::Result<crate::runtime_api::NativeOrEncoded<<Block
                                                                         as
                                                                         BlockT>::Header>>;
            #[doc =
                  " Generate inherent extrinsics. The inherent data will vary from chain to chain."]
            fn inherent_extrinsics(&self,
                                   at: &crate::runtime_api::BlockId<Block>,
                                   inherent: InherentData)
             ->
                 ::std::result::Result<Vec<<Block as BlockT>::Extrinsic>,
                                       crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&(&inherent));
                self.BlockBuilder_inherent_extrinsics_runtime_api_impl(at,
                                                                       crate::runtime_api::ExecutionContext::Other,
                                                                       Some((inherent)),
                                                                       runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                     match r
                                                                                                                         {
                                                                                                                         crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                         =>
                                                                                                                         {
                                                                                                                             Ok(n)
                                                                                                                         }
                                                                                                                         crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                         =>
                                                                                                                         {
                                                                                                                             <Vec<<Block
                                                                                                                                  as
                                                                                                                                  BlockT>::Extrinsic>
                                                                                                                                 as
                                                                                                                                 crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                                 crate::error::Error::CallResultDecode("inherent_extrinsics").into())
                                                                                                                         }
                                                                                                                     })
            }
            #[doc =
                  " Generate inherent extrinsics. The inherent data will vary from chain to chain."]
            fn inherent_extrinsics_with_context(&self,
                                                at:
                                                    &crate::runtime_api::BlockId<Block>,
                                                context:
                                                    crate::runtime_api::ExecutionContext,
                                                inherent: InherentData)
             ->
                 ::std::result::Result<Vec<<Block as BlockT>::Extrinsic>,
                                       crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&(&inherent));
                self.BlockBuilder_inherent_extrinsics_runtime_api_impl(at,
                                                                       context,
                                                                       Some((inherent)),
                                                                       runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                     match r
                                                                                                                         {
                                                                                                                         crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                         =>
                                                                                                                         {
                                                                                                                             Ok(n)
                                                                                                                         }
                                                                                                                         crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                         =>
                                                                                                                         {
                                                                                                                             <Vec<<Block
                                                                                                                                  as
                                                                                                                                  BlockT>::Extrinsic>
                                                                                                                                 as
                                                                                                                                 crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                                 crate::error::Error::CallResultDecode("inherent_extrinsics").into())
                                                                                                                         }
                                                                                                                     })
            }
            #[doc(hidden)]
            fn BlockBuilder_inherent_extrinsics_runtime_api_impl(&self,
                                                                 at:
                                                                     &crate::runtime_api::BlockId<Block>,
                                                                 context:
                                                                     crate::runtime_api::ExecutionContext,
                                                                 params:
                                                                     Option<(InherentData)>,
                                                                 params_encoded:
                                                                     Vec<u8>)
            ->
                crate::error::Result<crate::runtime_api::NativeOrEncoded<Vec<<Block
                                                                             as
                                                                             BlockT>::Extrinsic>>>;
            #[doc =
                  " Check that the inherents are valid. The inherent data will vary from chain to chain."]
            fn check_inherents(&self, at: &crate::runtime_api::BlockId<Block>,
                               block: Block, data: InherentData)
             ->
                 ::std::result::Result<CheckInherentsResult,
                                       crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&(&block, &data));
                self.BlockBuilder_check_inherents_runtime_api_impl(at,
                                                                   crate::runtime_api::ExecutionContext::Other,
                                                                   Some((block,
                                                                         data)),
                                                                   runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                 match r
                                                                                                                     {
                                                                                                                     crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                     =>
                                                                                                                     {
                                                                                                                         Ok(n)
                                                                                                                     }
                                                                                                                     crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                     =>
                                                                                                                     {
                                                                                                                         <CheckInherentsResult
                                                                                                                             as
                                                                                                                             crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                             crate::error::Error::CallResultDecode("check_inherents").into())
                                                                                                                     }
                                                                                                                 })
            }
            #[doc =
                  " Check that the inherents are valid. The inherent data will vary from chain to chain."]
            fn check_inherents_with_context(&self,
                                            at:
                                                &crate::runtime_api::BlockId<Block>,
                                            context:
                                                crate::runtime_api::ExecutionContext,
                                            block: Block, data: InherentData)
             ->
                 ::std::result::Result<CheckInherentsResult,
                                       crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&(&block, &data));
                self.BlockBuilder_check_inherents_runtime_api_impl(at,
                                                                   context,
                                                                   Some((block,
                                                                         data)),
                                                                   runtime_api_impl_params_encoded).and_then(|r|
                                                                                                                 match r
                                                                                                                     {
                                                                                                                     crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                     =>
                                                                                                                     {
                                                                                                                         Ok(n)
                                                                                                                     }
                                                                                                                     crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                     =>
                                                                                                                     {
                                                                                                                         <CheckInherentsResult
                                                                                                                             as
                                                                                                                             crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                             crate::error::Error::CallResultDecode("check_inherents").into())
                                                                                                                     }
                                                                                                                 })
            }
            #[doc(hidden)]
            fn BlockBuilder_check_inherents_runtime_api_impl(&self,
                                                             at:
                                                                 &crate::runtime_api::BlockId<Block>,
                                                             context:
                                                                 crate::runtime_api::ExecutionContext,
                                                             params:
                                                                 Option<(Block,
                                                                         InherentData)>,
                                                             params_encoded:
                                                                 Vec<u8>)
            ->
                crate::error::Result<crate::runtime_api::NativeOrEncoded<CheckInherentsResult>>;
            #[doc = " Generate a random seed."]
            fn random_seed(&self, at: &crate::runtime_api::BlockId<Block>)
             ->
                 ::std::result::Result<<Block as BlockT>::Hash,
                                       crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&());
                self.BlockBuilder_random_seed_runtime_api_impl(at,
                                                               crate::runtime_api::ExecutionContext::Other,
                                                               Some(()),
                                                               runtime_api_impl_params_encoded).and_then(|r|
                                                                                                             match r
                                                                                                                 {
                                                                                                                 crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                 =>
                                                                                                                 {
                                                                                                                     Ok(n)
                                                                                                                 }
                                                                                                                 crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                 =>
                                                                                                                 {
                                                                                                                     <<Block
                                                                                                                      as
                                                                                                                      BlockT>::Hash
                                                                                                                         as
                                                                                                                         crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                         crate::error::Error::CallResultDecode("random_seed").into())
                                                                                                                 }
                                                                                                             })
            }
            #[doc = " Generate a random seed."]
            fn random_seed_with_context(&self,
                                        at:
                                            &crate::runtime_api::BlockId<Block>,
                                        context:
                                            crate::runtime_api::ExecutionContext)
             ->
                 ::std::result::Result<<Block as BlockT>::Hash,
                                       crate::error::Error> {
                let runtime_api_impl_params_encoded =
                    crate::runtime_api::Encode::encode(&());
                self.BlockBuilder_random_seed_runtime_api_impl(at, context,
                                                               Some(()),
                                                               runtime_api_impl_params_encoded).and_then(|r|
                                                                                                             match r
                                                                                                                 {
                                                                                                                 crate::runtime_api::NativeOrEncoded::Native(n)
                                                                                                                 =>
                                                                                                                 {
                                                                                                                     Ok(n)
                                                                                                                 }
                                                                                                                 crate::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                                 =>
                                                                                                                 {
                                                                                                                     <<Block
                                                                                                                      as
                                                                                                                      BlockT>::Hash
                                                                                                                         as
                                                                                                                         crate::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                         crate::error::Error::CallResultDecode("random_seed").into())
                                                                                                                 }
                                                                                                             })
            }
            #[doc(hidden)]
            fn BlockBuilder_random_seed_runtime_api_impl(&self,
                                                         at:
                                                             &crate::runtime_api::BlockId<Block>,
                                                         context:
                                                             crate::runtime_api::ExecutionContext,
                                                         params: Option<()>,
                                                         params_encoded:
                                                             Vec<u8>)
            ->
                crate::error::Result<crate::runtime_api::NativeOrEncoded<<Block
                                                                         as
                                                                         BlockT>::Hash>>;
        }
        #[cfg(any(feature = "std", test))]
        impl <Block: crate::runtime_api::BlockT>
         crate::runtime_api::RuntimeApiInfo for BlockBuilder<Block> {
            const
            ID:
            [u8; 8]
            =
            [64u8, 254u8, 58u8, 212u8, 1u8, 248u8, 149u8, 154u8];
            const
            VERSION:
            u32
            =
            3u32;
        }
    }
}
#[cfg(feature = "std")]
pub mod light {
    //! Light client components.
    pub mod backend {
        //! Light client backend. Only stores headers and justifications of blocks.
        //! Everything else is requested from full nodes on demand.
        use std::collections::HashMap;
        use std::sync::{Arc, Weak};
        use futures::{Future, IntoFuture};
        use parking_lot::RwLock;
        use runtime_primitives::{generic::BlockId, Justification,
                                 StorageOverlay, ChildrenStorageOverlay};
        use state_machine::{Backend as StateBackend, TrieBackend,
                            backend::InMemory as InMemoryState};
        use runtime_primitives::traits::{Block as BlockT, NumberFor, Zero,
                                         Header};
        use crate::in_mem::{self, check_genesis_storage};
        use crate::backend::{AuxStore, Backend as ClientBackend,
                             BlockImportOperation, RemoteBackend,
                             NewBlockState};
        use crate::blockchain::HeaderBackend as BlockchainHeaderBackend;
        use crate::error::{Error as ClientError, Result as ClientResult};
        use crate::light::blockchain::{Blockchain, Storage as
                                       BlockchainStorage};
        use crate::light::fetcher::{Fetcher, RemoteReadRequest};
        use hash_db::Hasher;
        use trie::MemoryDB;
        use consensus::well_known_cache_keys;
        const IN_MEMORY_EXPECT_PROOF: &str =
            "InMemory state backend has Void error type and always suceeds; qed";
        /// Light client backend.
        pub struct Backend<S, F, H> {
            blockchain: Arc<Blockchain<S, F>>,
            genesis_state: RwLock<Option<InMemoryState<H>>>,
        }
        /// Light block (header and justification) import operation.
        pub struct ImportOperation<Block: BlockT, S, F, H> {
            header: Option<Block::Header>,
            cache: HashMap<well_known_cache_keys::Id, Vec<u8>>,
            leaf_state: NewBlockState,
            aux_ops: Vec<(Vec<u8>, Option<Vec<u8>>)>,
            finalized_blocks: Vec<BlockId<Block>>,
            set_head: Option<BlockId<Block>>,
            storage_update: Option<InMemoryState<H>>,
            _phantom: ::std::marker::PhantomData<(S, F)>,
        }
        /// On-demand state.
        pub struct OnDemandState<Block: BlockT, S, F> {
            fetcher: Weak<F>,
            blockchain: Weak<Blockchain<S, F>>,
            block: Block::Hash,
            cached_header: RwLock<Option<Block::Header>>,
        }
        /// On-demand or in-memory genesis state.
        pub enum OnDemandOrGenesisState<Block: BlockT, S, F, H> {

            /// On-demand state - storage values are fetched from remote nodes.
            OnDemand(OnDemandState<Block, S, F>),

            /// Genesis state - storage values are stored in-memory.
            Genesis(InMemoryState<H>),
        }
        impl <S, F, H> Backend<S, F, H> {
            /// Create new light backend.
            pub fn new(blockchain: Arc<Blockchain<S, F>>) -> Self {
                Self{blockchain, genesis_state: RwLock::new(None),}
            }
            /// Get shared blockchain reference.
            pub fn blockchain(&self) -> &Arc<Blockchain<S, F>> {
                &self.blockchain
            }
        }
        impl <S: AuxStore, F, H> AuxStore for Backend<S, F, H> {
            fn insert_aux<'a, 'b: 'a, 'c: 'a, I: IntoIterator<Item =
                          &'a (&'c [u8], &'c [u8])>, D: IntoIterator<Item =
                          &'a &'b [u8]>>(&self, insert: I, delete: D)
             -> ClientResult<()> {
                self.blockchain.storage().insert_aux(insert, delete)
            }
            fn get_aux(&self, key: &[u8]) -> ClientResult<Option<Vec<u8>>> {
                self.blockchain.storage().get_aux(key)
            }
        }
        impl <S, F, Block, H> ClientBackend<Block, H> for Backend<S, F, H>
         where Block: BlockT, S: BlockchainStorage<Block>, F: Fetcher<Block>,
         H: Hasher<Out = Block::Hash>, H::Out: Ord {
            type
            BlockImportOperation
            =
            ImportOperation<Block, S, F, H>;
            type
            Blockchain
            =
            Blockchain<S, F>;
            type
            State
            =
            OnDemandOrGenesisState<Block, S, F, H>;
            type
            ChangesTrieStorage
            =
            in_mem::ChangesTrieStorage<H>;
            fn begin_operation(&self)
             -> ClientResult<Self::BlockImportOperation> {
                Ok(ImportOperation{header: None,
                                   cache: Default::default(),
                                   leaf_state: NewBlockState::Normal,
                                   aux_ops: Vec::new(),
                                   finalized_blocks: Vec::new(),
                                   set_head: None,
                                   storage_update: None,
                                   _phantom: Default::default(),})
            }
            fn begin_state_operation(&self,
                                     _operation:
                                         &mut Self::BlockImportOperation,
                                     _block: BlockId<Block>)
             -> ClientResult<()> {
                Ok(())
            }
            fn commit_operation(&self,
                                mut operation: Self::BlockImportOperation)
             -> ClientResult<()> {
                if !operation.finalized_blocks.is_empty() {
                    for block in operation.finalized_blocks {
                        self.blockchain.storage().finalize_header(block)?;
                    }
                }
                if let Some(header) = operation.header {
                    let is_genesis_import = header.number().is_zero();
                    self.blockchain.storage().import_header(header,
                                                            operation.cache,
                                                            operation.leaf_state,
                                                            operation.aux_ops)?;
                    if is_genesis_import {
                        *self.genesis_state.write() =
                            operation.storage_update.take();
                    }
                } else {
                    for (key, maybe_val) in operation.aux_ops {
                        match maybe_val {
                            Some(val) =>
                            self.blockchain.storage().insert_aux(&[(&key[..],
                                                                    &val[..])],
                                                                 ::std::iter::empty())?,
                            None =>
                            self.blockchain.storage().insert_aux(::std::iter::empty(),
                                                                 &[&key[..]])?,
                        }
                    }
                }
                if let Some(set_head) = operation.set_head {
                    self.blockchain.storage().set_head(set_head)?;
                }
                Ok(())
            }
            fn finalize_block(&self, block: BlockId<Block>,
                              _justification: Option<Justification>)
             -> ClientResult<()> {
                self.blockchain.storage().finalize_header(block)
            }
            fn blockchain(&self) -> &Blockchain<S, F> { &self.blockchain }
            fn used_state_cache_size(&self) -> Option<usize> { None }
            fn changes_trie_storage(&self)
             -> Option<&Self::ChangesTrieStorage> {
                None
            }
            fn state_at(&self, block: BlockId<Block>)
             -> ClientResult<Self::State> {
                let block_number =
                    self.blockchain.expect_block_number_from_id(&block)?;
                if block_number.is_zero() {
                    if let Some(genesis_state) =
                           self.genesis_state.read().clone() {
                        return Ok(OnDemandOrGenesisState::Genesis(genesis_state));
                    }
                }
                let block_hash =
                    self.blockchain.expect_block_hash_from_id(&block)?;
                Ok(OnDemandOrGenesisState::OnDemand(OnDemandState{fetcher:
                                                                      self.blockchain.fetcher(),
                                                                  blockchain:
                                                                      Arc::downgrade(&self.blockchain),
                                                                  block:
                                                                      block_hash,
                                                                  cached_header:
                                                                      RwLock::new(None),}))
            }
            fn revert(&self, _n: NumberFor<Block>)
             -> ClientResult<NumberFor<Block>> {
                Err(ClientError::NotAvailableOnLightClient.into())
            }
        }
        impl <S, F, Block, H> RemoteBackend<Block, H> for Backend<S, F, H>
         where Block: BlockT, S: BlockchainStorage<Block>, F: Fetcher<Block>,
         H: Hasher<Out = Block::Hash>, H::Out: Ord {
            fn is_local_state_available(&self, block: &BlockId<Block>)
             -> bool {
                self.genesis_state.read().is_some() &&
                    self.blockchain.expect_block_number_from_id(block).map(|num|
                                                                               num.is_zero()).unwrap_or(false)
            }
        }
        impl <S, F, Block, H> BlockImportOperation<Block, H> for
         ImportOperation<Block, S, F, H> where Block: BlockT,
         F: Fetcher<Block>, S: BlockchainStorage<Block>, H: Hasher<Out =
         Block::Hash>, H::Out: Ord {
            type
            State
            =
            OnDemandOrGenesisState<Block, S, F, H>;
            fn state(&self) -> ClientResult<Option<&Self::State>> { Ok(None) }
            fn set_block_data(&mut self, header: Block::Header,
                              _body: Option<Vec<Block::Extrinsic>>,
                              _justification: Option<Justification>,
                              state: NewBlockState) -> ClientResult<()> {
                self.leaf_state = state;
                self.header = Some(header);
                Ok(())
            }
            fn update_cache(&mut self,
                            cache:
                                HashMap<well_known_cache_keys::Id, Vec<u8>>) {
                self.cache = cache;
            }
            fn update_db_storage(&mut self,
                                 _update:
                                     <Self::State as
                                     StateBackend<H>>::Transaction)
             -> ClientResult<()> {
                Ok(())
            }
            fn update_changes_trie(&mut self, _update: MemoryDB<H>)
             -> ClientResult<()> {
                Ok(())
            }
            fn reset_storage(&mut self, top: StorageOverlay,
                             children: ChildrenStorageOverlay)
             -> ClientResult<H::Out> {
                check_genesis_storage(&top, &children)?;
                let mut storage: HashMap<Option<Vec<u8>>, StorageOverlay> =
                    HashMap::new();
                storage.insert(None, top);
                let child_delta =
                    children.keys().cloned().map(|storage_key|
                                                     (storage_key,
                                                      None)).collect::<Vec<_>>();
                for (child_key, child_storage) in children {
                    storage.insert(Some(child_key), child_storage);
                }
                let storage_update: InMemoryState<H> = storage.into();
                let (storage_root, _) =
                    storage_update.full_storage_root(::std::iter::empty(),
                                                     child_delta);
                self.storage_update = Some(storage_update);
                Ok(storage_root)
            }
            fn insert_aux<I>(&mut self, ops: I) -> ClientResult<()> where
             I: IntoIterator<Item = (Vec<u8>, Option<Vec<u8>>)> {
                self.aux_ops.append(&mut ops.into_iter().collect());
                Ok(())
            }
            fn update_storage(&mut self,
                              _update: Vec<(Vec<u8>, Option<Vec<u8>>)>)
             -> ClientResult<()> {
                Ok(())
            }
            fn mark_finalized(&mut self, block: BlockId<Block>,
                              _justification: Option<Justification>)
             -> ClientResult<()> {
                self.finalized_blocks.push(block);
                Ok(())
            }
            fn mark_head(&mut self, block: BlockId<Block>)
             -> ClientResult<()> {
                self.set_head = Some(block);
                Ok(())
            }
        }
        impl <Block, S, F, H> StateBackend<H> for OnDemandState<Block, S, F>
         where Block: BlockT, S: BlockchainStorage<Block>, F: Fetcher<Block>,
         H: Hasher<Out = Block::Hash> {
            type
            Error
            =
            ClientError;
            type
            Transaction
            =
            ();
            type
            TrieBackendStorage
            =
            MemoryDB<H>;
            fn storage(&self, key: &[u8]) -> ClientResult<Option<Vec<u8>>> {
                let mut header = self.cached_header.read().clone();
                if header.is_none() {
                    let cached_header =
                        self.blockchain.upgrade().ok_or_else(||
                                                                 ClientError::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                              &match (&self.block,)
                                                                                                                                                   {
                                                                                                                                                   (arg0,)
                                                                                                                                                   =>
                                                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                ::std::fmt::Display::fmt)],
                                                                                                                                               })))).and_then(|blockchain|
                                                                                                                                                                  blockchain.expect_header(BlockId::Hash(self.block)))?;
                    header = Some(cached_header.clone());
                    *self.cached_header.write() = Some(cached_header);
                }
                self.fetcher.upgrade().ok_or(ClientError::NotAvailableOnLightClient)?.remote_read(RemoteReadRequest{block:
                                                                                                                        self.block,
                                                                                                                    header:
                                                                                                                        header.expect("if block above guarantees that header is_some(); qed"),
                                                                                                                    key:
                                                                                                                        key.to_vec(),
                                                                                                                    retry_count:
                                                                                                                        None,}).into_future().wait()
            }
            fn child_storage(&self, _storage_key: &[u8], _key: &[u8])
             -> ClientResult<Option<Vec<u8>>> {
                Err(ClientError::NotAvailableOnLightClient.into())
            }
            fn for_keys_with_prefix<A: FnMut(&[u8])>(&self, _prefix: &[u8],
                                                     _action: A) {
            }
            fn for_keys_in_child_storage<A: FnMut(&[u8])>(&self,
                                                          _storage_key: &[u8],
                                                          _action: A) {
            }
            fn storage_root<I>(&self, _delta: I)
             -> (H::Out, Self::Transaction) where I: IntoIterator<Item =
             (Vec<u8>, Option<Vec<u8>>)> {
                (H::Out::default(), ())
            }
            fn child_storage_root<I>(&self, _key: &[u8], _delta: I)
             -> (Vec<u8>, bool, Self::Transaction) where I: IntoIterator<Item
             = (Vec<u8>, Option<Vec<u8>>)> {
                (H::Out::default().as_ref().to_vec(), true, ())
            }
            fn pairs(&self) -> Vec<(Vec<u8>, Vec<u8>)> { Vec::new() }
            fn keys(&self, _prefix: &[u8]) -> Vec<Vec<u8>> { Vec::new() }
            fn try_into_trie_backend(self)
             -> Option<TrieBackend<Self::TrieBackendStorage, H>> {
                None
            }
        }
        impl <Block, S, F, H> StateBackend<H> for
         OnDemandOrGenesisState<Block, S, F, H> where Block: BlockT,
         F: Fetcher<Block>, S: BlockchainStorage<Block>, H: Hasher<Out =
         Block::Hash>, H::Out: Ord {
            type
            Error
            =
            ClientError;
            type
            Transaction
            =
            ();
            type
            TrieBackendStorage
            =
            MemoryDB<H>;
            fn storage(&self, key: &[u8]) -> ClientResult<Option<Vec<u8>>> {
                match *self {
                    OnDemandOrGenesisState::OnDemand(ref state) =>
                    StateBackend::<H>::storage(state, key),
                    OnDemandOrGenesisState::Genesis(ref state) =>
                    Ok(state.storage(key).expect(IN_MEMORY_EXPECT_PROOF)),
                }
            }
            fn child_storage(&self, storage_key: &[u8], key: &[u8])
             -> ClientResult<Option<Vec<u8>>> {
                match *self {
                    OnDemandOrGenesisState::OnDemand(ref state) =>
                    StateBackend::<H>::child_storage(state, storage_key, key),
                    OnDemandOrGenesisState::Genesis(ref state) =>
                    Ok(state.child_storage(storage_key,
                                           key).expect(IN_MEMORY_EXPECT_PROOF)),
                }
            }
            fn for_keys_with_prefix<A: FnMut(&[u8])>(&self, prefix: &[u8],
                                                     action: A) {
                match *self {
                    OnDemandOrGenesisState::OnDemand(ref state) =>
                    StateBackend::<H>::for_keys_with_prefix(state, prefix,
                                                            action),
                    OnDemandOrGenesisState::Genesis(ref state) =>
                    state.for_keys_with_prefix(prefix, action),
                }
            }
            fn for_keys_in_child_storage<A: FnMut(&[u8])>(&self,
                                                          storage_key: &[u8],
                                                          action: A) {
                match *self {
                    OnDemandOrGenesisState::OnDemand(ref state) =>
                    StateBackend::<H>::for_keys_in_child_storage(state,
                                                                 storage_key,
                                                                 action),
                    OnDemandOrGenesisState::Genesis(ref state) =>
                    state.for_keys_in_child_storage(storage_key, action),
                }
            }
            fn storage_root<I>(&self, delta: I) -> (H::Out, Self::Transaction)
             where I: IntoIterator<Item = (Vec<u8>, Option<Vec<u8>>)> {
                match *self {
                    OnDemandOrGenesisState::OnDemand(ref state) =>
                    StateBackend::<H>::storage_root(state, delta),
                    OnDemandOrGenesisState::Genesis(ref state) => {
                        let (root, _) = state.storage_root(delta);
                        (root, ())
                    }
                }
            }
            fn child_storage_root<I>(&self, key: &[u8], delta: I)
             -> (Vec<u8>, bool, Self::Transaction) where I: IntoIterator<Item
             = (Vec<u8>, Option<Vec<u8>>)> {
                match *self {
                    OnDemandOrGenesisState::OnDemand(ref state) =>
                    StateBackend::<H>::child_storage_root(state, key, delta),
                    OnDemandOrGenesisState::Genesis(ref state) => {
                        let (root, is_equal, _) =
                            state.child_storage_root(key, delta);
                        (root, is_equal, ())
                    }
                }
            }
            fn pairs(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
                match *self {
                    OnDemandOrGenesisState::OnDemand(ref state) =>
                    StateBackend::<H>::pairs(state),
                    OnDemandOrGenesisState::Genesis(ref state) =>
                    state.pairs(),
                }
            }
            fn keys(&self, prefix: &[u8]) -> Vec<Vec<u8>> {
                match *self {
                    OnDemandOrGenesisState::OnDemand(ref state) =>
                    StateBackend::<H>::keys(state, prefix),
                    OnDemandOrGenesisState::Genesis(ref state) =>
                    state.keys(prefix),
                }
            }
            fn try_into_trie_backend(self)
             -> Option<TrieBackend<Self::TrieBackendStorage, H>> {
                match self {
                    OnDemandOrGenesisState::OnDemand(state) =>
                    state.try_into_trie_backend(),
                    OnDemandOrGenesisState::Genesis(state) =>
                    state.try_into_trie_backend(),
                }
            }
        }
    }
    pub mod blockchain {
        //! Light client blockchain backend. Only stores headers and justifications of recent
        //! blocks. CHT roots are stored for headers of ancient blocks.
        use std::{sync::{Weak, Arc}, collections::HashMap};
        use futures::{Future, IntoFuture};
        use parking_lot::Mutex;
        use runtime_primitives::{Justification, generic::BlockId};
        use runtime_primitives::traits::{Block as BlockT, Header as HeaderT,
                                         NumberFor, Zero};
        use consensus::well_known_cache_keys;
        use crate::backend::{AuxStore, NewBlockState};
        use crate::blockchain::{Backend as BlockchainBackend, BlockStatus,
                                Cache as BlockchainCache, HeaderBackend as
                                BlockchainHeaderBackend, Info as
                                BlockchainInfo, ProvideCache};
        use crate::cht;
        use crate::error::{Error as ClientError, Result as ClientResult};
        use crate::light::fetcher::{Fetcher, RemoteBodyRequest,
                                    RemoteHeaderRequest};
        /// Light client blockchain storage.
        pub trait Storage<Block: BlockT>: AuxStore +
         BlockchainHeaderBackend<Block> {
            /// Store new header. Should refuse to revert any finalized blocks.
            ///
            /// Takes new authorities, the leaf state of the new block, and
            /// any auxiliary storage updates to place in the same operation.
            fn import_header(&self, header: Block::Header,
                             cache:
                                 HashMap<well_known_cache_keys::Id, Vec<u8>>,
                             state: NewBlockState,
                             aux_ops: Vec<(Vec<u8>, Option<Vec<u8>>)>)
            -> ClientResult<()>;
            /// Set an existing block as new best block.
            fn set_head(&self, block: BlockId<Block>)
            -> ClientResult<()>;
            /// Mark historic header as finalized.
            fn finalize_header(&self, block: BlockId<Block>)
            -> ClientResult<()>;
            /// Get last finalized header.
            fn last_finalized(&self)
            -> ClientResult<Block::Hash>;
            /// Get headers CHT root for given block. Fails if the block is not pruned (not a part of any CHT).
            fn header_cht_root(&self, cht_size: u64, block: NumberFor<Block>)
            -> ClientResult<Block::Hash>;
            /// Get changes trie CHT root for given block. Fails if the block is not pruned (not a part of any CHT).
            fn changes_trie_cht_root(&self, cht_size: u64,
                                     block: NumberFor<Block>)
            -> ClientResult<Block::Hash>;
            /// Get storage cache.
            fn cache(&self)
            -> Option<Arc<BlockchainCache<Block>>>;
        }
        /// Light client blockchain.
        pub struct Blockchain<S, F> {
            fetcher: Mutex<Weak<F>>,
            storage: S,
        }
        impl <S, F> Blockchain<S, F> {
            /// Create new light blockchain backed with given storage.
            pub fn new(storage: S) -> Self {
                Self{fetcher: Mutex::new(Default::default()), storage,}
            }
            /// Sets fetcher reference.
            pub fn set_fetcher(&self, fetcher: Weak<F>) {
                *self.fetcher.lock() = fetcher;
            }
            /// Get fetcher weak reference.
            pub fn fetcher(&self) -> Weak<F> { self.fetcher.lock().clone() }
            /// Get storage reference.
            pub fn storage(&self) -> &S { &self.storage }
        }
        impl <S, F, Block> BlockchainHeaderBackend<Block> for Blockchain<S, F>
         where Block: BlockT, S: Storage<Block>, F: Fetcher<Block> {
            fn header(&self, id: BlockId<Block>)
             -> ClientResult<Option<Block::Header>> {
                match self.storage.header(id)? {
                    Some(header) => Ok(Some(header)),
                    None => {
                        let number =
                            match id {
                                BlockId::Hash(hash) =>
                                match self.storage.number(hash)? {
                                    Some(number) => number,
                                    None => return Ok(None),
                                },
                                BlockId::Number(number) => number,
                            };
                        if number.is_zero() ||
                               self.storage.status(BlockId::Number(number))?
                                   == BlockStatus::Unknown {
                            return Ok(None);
                        }
                        self.fetcher().upgrade().ok_or(ClientError::NotAvailableOnLightClient)?.remote_header(RemoteHeaderRequest{cht_root:
                                                                                                                                      self.storage.header_cht_root(cht::SIZE,
                                                                                                                                                                   number)?,
                                                                                                                                  block:
                                                                                                                                      number,
                                                                                                                                  retry_count:
                                                                                                                                      None,}).into_future().wait().map(Some)
                    }
                }
            }
            fn info(&self) -> ClientResult<BlockchainInfo<Block>> {
                self.storage.info()
            }
            fn status(&self, id: BlockId<Block>)
             -> ClientResult<BlockStatus> {
                self.storage.status(id)
            }
            fn number(&self, hash: Block::Hash)
             -> ClientResult<Option<NumberFor<Block>>> {
                self.storage.number(hash)
            }
            fn hash(&self,
                    number: <<Block as BlockT>::Header as HeaderT>::Number)
             -> ClientResult<Option<Block::Hash>> {
                self.storage.hash(number)
            }
        }
        impl <S, F, Block> BlockchainBackend<Block> for Blockchain<S, F> where
         Block: BlockT, S: Storage<Block>, F: Fetcher<Block> {
            fn body(&self, id: BlockId<Block>)
             -> ClientResult<Option<Vec<Block::Extrinsic>>> {
                let header =
                    match self.header(id)? {
                        Some(header) => header,
                        None => return Ok(None),
                    };
                self.fetcher().upgrade().ok_or(ClientError::NotAvailableOnLightClient)?.remote_body(RemoteBodyRequest{header,
                                                                                                                      retry_count:
                                                                                                                          None,}).into_future().wait().map(Some)
            }
            fn justification(&self, _id: BlockId<Block>)
             -> ClientResult<Option<Justification>> {
                Ok(None)
            }
            fn last_finalized(&self) -> ClientResult<Block::Hash> {
                self.storage.last_finalized()
            }
            fn cache(&self) -> Option<Arc<BlockchainCache<Block>>> {
                self.storage.cache()
            }
            fn leaves(&self) -> ClientResult<Vec<Block::Hash>> {
                {
                    ::std::rt::begin_panic("not yet implemented",
                                           &("core/client/src/light/blockchain.rs",
                                             175u32, 3u32))
                }
            }
            fn children(&self, _parent_hash: Block::Hash)
             -> ClientResult<Vec<Block::Hash>> {
                {
                    ::std::rt::begin_panic("not yet implemented",
                                           &("core/client/src/light/blockchain.rs",
                                             179u32, 3u32))
                }
            }
        }
        impl <S: Storage<Block>, F, Block: BlockT> ProvideCache<Block> for
         Blockchain<S, F> {
            fn cache(&self) -> Option<Arc<BlockchainCache<Block>>> {
                self.storage.cache()
            }
        }
    }
    pub mod call_executor {
        //! Light client call executor. Executes methods on remote full nodes, fetching
        //! execution proof and checking it locally.
        use std::{collections::HashSet, sync::Arc, panic::UnwindSafe, result,
                  marker::PhantomData, cell::RefCell, rc::Rc};
        use futures::{IntoFuture, Future};
        use parity_codec::{Encode, Decode};
        use primitives::{H256, Blake2Hasher, convert_hash, NativeOrEncoded,
                         OffchainExt};
        use runtime_primitives::generic::BlockId;
        use runtime_primitives::traits::{As, Block as BlockT, Header as
                                         HeaderT};
        use state_machine::{self, Backend as StateBackend, CodeExecutor,
                            OverlayedChanges, ExecutionStrategy,
                            create_proof_check_backend,
                            execution_proof_check_on_trie_backend,
                            ExecutionManager, NeverOffchainExt};
        use hash_db::Hasher;
        use crate::runtime_api::{ProofRecorder, InitializeBlock};
        use crate::backend::RemoteBackend;
        use crate::blockchain::Backend as ChainBackend;
        use crate::call_executor::CallExecutor;
        use crate::error::{Error as ClientError, Result as ClientResult};
        use crate::light::fetcher::{Fetcher, RemoteCallRequest};
        use executor::{RuntimeVersion, NativeVersion};
        use trie::MemoryDB;
        /// Call executor that executes methods on remote node, querying execution proof
        /// and checking proof by re-executing locally.
        pub struct RemoteCallExecutor<B, F> {
            blockchain: Arc<B>,
            fetcher: Arc<F>,
        }
        /// Remote or local call executor.
        ///
        /// Calls are executed locally if state is available locally. Otherwise, calls
        /// are redirected to remote call executor.
        pub struct RemoteOrLocalCallExecutor<Block: BlockT<Hash = H256>, B, R,
                                             L> {
            backend: Arc<B>,
            remote: R,
            local: L,
            _block: PhantomData<Block>,
        }
        impl <B, F> Clone for RemoteCallExecutor<B, F> {
            fn clone(&self) -> Self {
                RemoteCallExecutor{blockchain: self.blockchain.clone(),
                                   fetcher: self.fetcher.clone(),}
            }
        }
        impl <B, F> RemoteCallExecutor<B, F> {
            /// Creates new instance of remote call executor.
            pub fn new(blockchain: Arc<B>, fetcher: Arc<F>) -> Self {
                RemoteCallExecutor{blockchain, fetcher,}
            }
        }
        impl <B, F, Block> CallExecutor<Block, Blake2Hasher> for
         RemoteCallExecutor<B, F> where Block: BlockT<Hash = H256>,
         B: ChainBackend<Block>, F: Fetcher<Block>, Block::Hash: Ord {
            type
            Error
            =
            ClientError;
            fn call<O: OffchainExt>(&self, id: &BlockId<Block>, method: &str,
                                    call_data: &[u8],
                                    _strategy: ExecutionStrategy,
                                    _side_effects_handler: Option<&mut O>)
             -> ClientResult<Vec<u8>> {
                let block_hash =
                    self.blockchain.expect_block_hash_from_id(id)?;
                let block_header = self.blockchain.expect_header(id.clone())?;
                self.fetcher.remote_call(RemoteCallRequest{block: block_hash,
                                                           header:
                                                               block_header,
                                                           method:
                                                               method.into(),
                                                           call_data:
                                                               call_data.to_vec(),
                                                           retry_count:
                                                               None,}).into_future().wait()
            }
            fn contextual_call<'a, O: OffchainExt, IB: Fn() ->
                               ClientResult<()>,
                               EM: Fn(Result<NativeOrEncoded<R>, Self::Error>,
                                      Result<NativeOrEncoded<R>, Self::Error>)
                               -> Result<NativeOrEncoded<R>, Self::Error>,
                               R: Encode + Decode + PartialEq,
                               NC>(&self, _initialize_block_fn: IB,
                                   at: &BlockId<Block>, method: &str,
                                   call_data: &[u8],
                                   changes: &RefCell<OverlayedChanges>,
                                   initialize_block:
                                       InitializeBlock<'a, Block>,
                                   execution_manager: ExecutionManager<EM>,
                                   _native_call: Option<NC>,
                                   side_effects_handler: Option<&mut O>,
                                   _recorder:
                                       &Option<Rc<RefCell<ProofRecorder<Block>>>>)
             -> ClientResult<NativeOrEncoded<R>> where
             ExecutionManager<EM>: Clone {
                let block_initialized =
                    match initialize_block {
                        InitializeBlock::Do(ref init_block) => {
                            init_block.borrow().is_some()
                        }
                        InitializeBlock::Skip => false,
                    };
                if !changes.borrow().is_empty() || block_initialized {
                    return Err(ClientError::NotAvailableOnLightClient.into());
                }
                self.call(at, method, call_data, (&execution_manager).into(),
                          side_effects_handler).map(NativeOrEncoded::Encoded)
            }
            fn runtime_version(&self, id: &BlockId<Block>)
             -> ClientResult<RuntimeVersion> {
                let call_result =
                    self.call(id, "Core_version", &[],
                              ExecutionStrategy::NativeElseWasm,
                              NeverOffchainExt::new())?;
                RuntimeVersion::decode(&mut call_result.as_slice()).ok_or_else(||
                                                                                   ClientError::VersionInvalid.into())
            }
            fn call_at_state<O: OffchainExt, S: StateBackend<Blake2Hasher>,
                             FF: FnOnce(Result<NativeOrEncoded<R>,
                                               Self::Error>,
                                        Result<NativeOrEncoded<R>,
                                               Self::Error>) ->
                             Result<NativeOrEncoded<R>, Self::Error>,
                             R: Encode + Decode + PartialEq, NC: FnOnce() ->
                             result::Result<R,
                                            &'static str>>(&self, _state: &S,
                                                           _changes:
                                                               &mut OverlayedChanges,
                                                           _method: &str,
                                                           _call_data: &[u8],
                                                           _m:
                                                               ExecutionManager<FF>,
                                                           _native_call:
                                                               Option<NC>,
                                                           _side_effects_handler:
                                                               Option<&mut O>)
             ->
                 ClientResult<(NativeOrEncoded<R>, S::Transaction,
                               Option<MemoryDB<Blake2Hasher>>)> {
                Err(ClientError::NotAvailableOnLightClient.into())
            }
            fn prove_at_trie_state<S: state_machine::TrieBackendStorage<Blake2Hasher>>(&self,
                                                                                       _state:
                                                                                           &state_machine::TrieBackend<S,
                                                                                                                       Blake2Hasher>,
                                                                                       _changes:
                                                                                           &mut OverlayedChanges,
                                                                                       _method:
                                                                                           &str,
                                                                                       _call_data:
                                                                                           &[u8])
             -> ClientResult<(Vec<u8>, Vec<Vec<u8>>)> {
                Err(ClientError::NotAvailableOnLightClient.into())
            }
            fn native_runtime_version(&self) -> Option<&NativeVersion> {
                None
            }
        }
        impl <Block, B, R, L> Clone for
         RemoteOrLocalCallExecutor<Block, B, R, L> where Block: BlockT<Hash =
         H256>, B: RemoteBackend<Block, Blake2Hasher>,
         R: CallExecutor<Block, Blake2Hasher> + Clone,
         L: CallExecutor<Block, Blake2Hasher> + Clone {
            fn clone(&self) -> Self {
                RemoteOrLocalCallExecutor{backend: self.backend.clone(),
                                          remote: self.remote.clone(),
                                          local: self.local.clone(),
                                          _block: Default::default(),}
            }
        }
        impl <Block, B, Remote, Local>
         RemoteOrLocalCallExecutor<Block, B, Remote, Local> where
         Block: BlockT<Hash = H256>, B: RemoteBackend<Block, Blake2Hasher>,
         Remote: CallExecutor<Block, Blake2Hasher>,
         Local: CallExecutor<Block, Blake2Hasher> {
            /// Creates new instance of remote/local call executor.
            pub fn new(backend: Arc<B>, remote: Remote, local: Local)
             -> Self {
                RemoteOrLocalCallExecutor{backend,
                                          remote,
                                          local,
                                          _block: Default::default(),}
            }
        }
        impl <Block, B, Remote, Local> CallExecutor<Block, Blake2Hasher> for
         RemoteOrLocalCallExecutor<Block, B, Remote, Local> where
         Block: BlockT<Hash = H256>, B: RemoteBackend<Block, Blake2Hasher>,
         Remote: CallExecutor<Block, Blake2Hasher>,
         Local: CallExecutor<Block, Blake2Hasher> {
            type
            Error
            =
            ClientError;
            fn call<O: OffchainExt>(&self, id: &BlockId<Block>, method: &str,
                                    call_data: &[u8],
                                    strategy: ExecutionStrategy,
                                    side_effects_handler: Option<&mut O>)
             -> ClientResult<Vec<u8>> {
                match self.backend.is_local_state_available(id) {
                    true =>
                    self.local.call(id, method, call_data, strategy,
                                    side_effects_handler),
                    false =>
                    self.remote.call(id, method, call_data, strategy,
                                     side_effects_handler),
                }
            }
            fn contextual_call<'a, O: OffchainExt, IB: Fn() ->
                               ClientResult<()>,
                               EM: Fn(Result<NativeOrEncoded<R>, Self::Error>,
                                      Result<NativeOrEncoded<R>, Self::Error>)
                               -> Result<NativeOrEncoded<R>, Self::Error>,
                               R: Encode + Decode + PartialEq, NC: FnOnce() ->
                               result::Result<R, &'static str> +
                               UnwindSafe>(&self, initialize_block_fn: IB,
                                           at: &BlockId<Block>, method: &str,
                                           call_data: &[u8],
                                           changes:
                                               &RefCell<OverlayedChanges>,
                                           initialize_block:
                                               InitializeBlock<'a, Block>,
                                           _manager: ExecutionManager<EM>,
                                           native_call: Option<NC>,
                                           side_effects_handler:
                                               Option<&mut O>,
                                           recorder:
                                               &Option<Rc<RefCell<ProofRecorder<Block>>>>)
             -> ClientResult<NativeOrEncoded<R>> where
             ExecutionManager<EM>: Clone {
                match self.backend.is_local_state_available(at) {
                    true =>
                    CallExecutor::contextual_call::<_, _,
                                                    fn(Result<NativeOrEncoded<R>,
                                                              Local::Error>,
                                                       Result<NativeOrEncoded<R>,
                                                              Local::Error>)
                                                        ->
                                                            Result<NativeOrEncoded<R>,
                                                                   Local::Error>,
                                                    _,
                                                    NC>(&self.local,
                                                        initialize_block_fn,
                                                        at, method, call_data,
                                                        changes,
                                                        initialize_block,
                                                        ExecutionManager::NativeWhenPossible,
                                                        native_call,
                                                        side_effects_handler,
                                                        recorder).map_err(|e|
                                                                              ClientError::Execution(Box::new(e.to_string()))),
                    false =>
                    CallExecutor::contextual_call::<_, _,
                                                    fn(Result<NativeOrEncoded<R>,
                                                              Remote::Error>,
                                                       Result<NativeOrEncoded<R>,
                                                              Remote::Error>)
                                                        ->
                                                            Result<NativeOrEncoded<R>,
                                                                   Remote::Error>,
                                                    _,
                                                    NC>(&self.remote,
                                                        initialize_block_fn,
                                                        at, method, call_data,
                                                        changes,
                                                        initialize_block,
                                                        ExecutionManager::NativeWhenPossible,
                                                        native_call,
                                                        side_effects_handler,
                                                        recorder).map_err(|e|
                                                                              ClientError::Execution(Box::new(e.to_string()))),
                }
            }
            fn runtime_version(&self, id: &BlockId<Block>)
             -> ClientResult<RuntimeVersion> {
                match self.backend.is_local_state_available(id) {
                    true => self.local.runtime_version(id),
                    false => self.remote.runtime_version(id),
                }
            }
            fn call_at_state<O: OffchainExt, S: StateBackend<Blake2Hasher>,
                             FF: FnOnce(Result<NativeOrEncoded<R>,
                                               Self::Error>,
                                        Result<NativeOrEncoded<R>,
                                               Self::Error>) ->
                             Result<NativeOrEncoded<R>, Self::Error>,
                             R: Encode + Decode + PartialEq, NC: FnOnce() ->
                             result::Result<R, &'static str> +
                             UnwindSafe>(&self, state: &S,
                                         changes: &mut OverlayedChanges,
                                         method: &str, call_data: &[u8],
                                         _manager: ExecutionManager<FF>,
                                         native_call: Option<NC>,
                                         side_effects_handler: Option<&mut O>)
             ->
                 ClientResult<(NativeOrEncoded<R>, S::Transaction,
                               Option<MemoryDB<Blake2Hasher>>)> {
                CallExecutor::call_at_state::<_, _,
                                              fn(Result<NativeOrEncoded<R>,
                                                        Remote::Error>,
                                                 Result<NativeOrEncoded<R>,
                                                        Remote::Error>)
                                                  ->
                                                      Result<NativeOrEncoded<R>,
                                                             Remote::Error>,
                                              _,
                                              NC>(&self.remote, state,
                                                  changes, method, call_data,
                                                  ExecutionManager::NativeWhenPossible,
                                                  native_call,
                                                  side_effects_handler).map_err(|e|
                                                                                    ClientError::Execution(Box::new(e.to_string())))
            }
            fn prove_at_trie_state<S: state_machine::TrieBackendStorage<Blake2Hasher>>(&self,
                                                                                       state:
                                                                                           &state_machine::TrieBackend<S,
                                                                                                                       Blake2Hasher>,
                                                                                       changes:
                                                                                           &mut OverlayedChanges,
                                                                                       method:
                                                                                           &str,
                                                                                       call_data:
                                                                                           &[u8])
             -> ClientResult<(Vec<u8>, Vec<Vec<u8>>)> {
                self.remote.prove_at_trie_state(state, changes, method,
                                                call_data)
            }
            fn native_runtime_version(&self) -> Option<&NativeVersion> {
                None
            }
        }
        /// Prove contextual execution using given block header in environment.
        ///
        /// Method is executed using passed header as environment' current block.
        /// Proof includes both environment preparation proof and method execution proof.
        pub fn prove_execution<Block, S,
                               E>(state: S, header: Block::Header,
                                  executor: &E, method: &str,
                                  call_data: &[u8])
         -> ClientResult<(Vec<u8>, Vec<Vec<u8>>)> where Block: BlockT<Hash =
         H256>, S: StateBackend<Blake2Hasher>,
         E: CallExecutor<Block, Blake2Hasher> {
            let trie_state =
                state.try_into_trie_backend().ok_or_else(||
                                                             Box::new(state_machine::ExecutionError::UnableToGenerateProof)
                                                                 as
                                                                 Box<state_machine::Error>)?;
            let mut changes = Default::default();
            let (_, init_proof) =
                executor.prove_at_trie_state(&trie_state, &mut changes,
                                             "Core_initialize_block",
                                             &header.encode())?;
            let (result, exec_proof) =
                executor.prove_at_trie_state(&trie_state, &mut changes,
                                             method, call_data)?;
            let total_proof =
                init_proof.into_iter().chain(exec_proof.into_iter()).collect::<HashSet<_>>().into_iter().collect();
            Ok((result, total_proof))
        }
        /// Check remote contextual execution proof using given backend.
        ///
        /// Method is executed using passed header as environment' current block.
        /// Proof should include both environment preparation proof and method execution proof.
        pub fn check_execution_proof<Header, E,
                                     H>(executor: &E,
                                        request: &RemoteCallRequest<Header>,
                                        remote_proof: Vec<Vec<u8>>)
         -> ClientResult<Vec<u8>> where Header: HeaderT, E: CodeExecutor<H>,
         H: Hasher, H::Out: Ord {
            let local_state_root = request.header.state_root();
            let root: H::Out = convert_hash(&local_state_root);
            let mut changes = OverlayedChanges::default();
            let trie_backend =
                create_proof_check_backend(root, remote_proof)?;
            let next_block =
                <Header as
                    HeaderT>::new(*request.header.number() + As::sa(1),
                                  Default::default(), Default::default(),
                                  request.header.hash(), Default::default());
            execution_proof_check_on_trie_backend::<H,
                                                    _>(&trie_backend,
                                                       &mut changes, executor,
                                                       "Core_initialize_block",
                                                       &next_block.encode())?;
            let local_result =
                execution_proof_check_on_trie_backend::<H,
                                                        _>(&trie_backend,
                                                           &mut changes,
                                                           executor,
                                                           &request.method,
                                                           &request.call_data)?;
            Ok(local_result)
        }
    }
    pub mod fetcher {
        //! Light client data fetcher. Fetches requested data from remote full nodes.
        use std::sync::Arc;
        use std::collections::BTreeMap;
        use std::marker::PhantomData;
        use futures::IntoFuture;
        use hash_db::{HashDB, Hasher};
        use parity_codec::Encode;
        use primitives::{ChangesTrieConfiguration, convert_hash};
        use runtime_primitives::traits::{As, Block as BlockT, Header as
                                         HeaderT, Hash, HashFor, NumberFor};
        use state_machine::{CodeExecutor, ChangesTrieRootsStorage,
                            ChangesTrieAnchorBlockId, TrieBackend,
                            read_proof_check, key_changes_proof_check,
                            create_proof_check_backend_storage,
                            read_child_proof_check};
        use crate::cht;
        use crate::error::{Error as ClientError, Result as ClientResult};
        use crate::light::blockchain::{Blockchain, Storage as
                                       BlockchainStorage};
        use crate::light::call_executor::check_execution_proof;
        /// Remote call request.
        #[structural_match]
        pub struct RemoteCallRequest<Header: HeaderT> {
            /// Call at state of given block.
            pub block: Header::Hash,
            /// Header of block at which call is performed.
            pub header: Header,
            /// Method to call.
            pub method: String,
            /// Call data.
            pub call_data: Vec<u8>,
            /// Number of times to retry request. None means that default RETRY_COUNT is used.
            pub retry_count: Option<usize>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::clone::Clone + HeaderT> ::std::clone::Clone for
         RemoteCallRequest<Header> where Header::Hash: ::std::clone::Clone {
            #[inline]
            fn clone(&self) -> RemoteCallRequest<Header> {
                match *self {
                    RemoteCallRequest {
                    block: ref __self_0_0,
                    header: ref __self_0_1,
                    method: ref __self_0_2,
                    call_data: ref __self_0_3,
                    retry_count: ref __self_0_4 } =>
                    RemoteCallRequest{block:
                                          ::std::clone::Clone::clone(&(*__self_0_0)),
                                      header:
                                          ::std::clone::Clone::clone(&(*__self_0_1)),
                                      method:
                                          ::std::clone::Clone::clone(&(*__self_0_2)),
                                      call_data:
                                          ::std::clone::Clone::clone(&(*__self_0_3)),
                                      retry_count:
                                          ::std::clone::Clone::clone(&(*__self_0_4)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::fmt::Debug + HeaderT> ::std::fmt::Debug for
         RemoteCallRequest<Header> where Header::Hash: ::std::fmt::Debug {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    RemoteCallRequest {
                    block: ref __self_0_0,
                    header: ref __self_0_1,
                    method: ref __self_0_2,
                    call_data: ref __self_0_3,
                    retry_count: ref __self_0_4 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("RemoteCallRequest");
                        let _ =
                            debug_trait_builder.field("block",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("header",
                                                      &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("method",
                                                      &&(*__self_0_2));
                        let _ =
                            debug_trait_builder.field("call_data",
                                                      &&(*__self_0_3));
                        let _ =
                            debug_trait_builder.field("retry_count",
                                                      &&(*__self_0_4));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::PartialEq + HeaderT> ::std::cmp::PartialEq
         for RemoteCallRequest<Header> where
         Header::Hash: ::std::cmp::PartialEq {
            #[inline]
            fn eq(&self, other: &RemoteCallRequest<Header>) -> bool {
                match *other {
                    RemoteCallRequest {
                    block: ref __self_1_0,
                    header: ref __self_1_1,
                    method: ref __self_1_2,
                    call_data: ref __self_1_3,
                    retry_count: ref __self_1_4 } =>
                    match *self {
                        RemoteCallRequest {
                        block: ref __self_0_0,
                        header: ref __self_0_1,
                        method: ref __self_0_2,
                        call_data: ref __self_0_3,
                        retry_count: ref __self_0_4 } =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RemoteCallRequest<Header>) -> bool {
                match *other {
                    RemoteCallRequest {
                    block: ref __self_1_0,
                    header: ref __self_1_1,
                    method: ref __self_1_2,
                    call_data: ref __self_1_3,
                    retry_count: ref __self_1_4 } =>
                    match *self {
                        RemoteCallRequest {
                        block: ref __self_0_0,
                        header: ref __self_0_1,
                        method: ref __self_0_2,
                        call_data: ref __self_0_3,
                        retry_count: ref __self_0_4 } =>
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
        impl <Header: ::std::cmp::Eq + HeaderT> ::std::cmp::Eq for
         RemoteCallRequest<Header> where Header::Hash: ::std::cmp::Eq {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Header::Hash>;
                    let _: ::std::cmp::AssertParamIsEq<Header>;
                    let _: ::std::cmp::AssertParamIsEq<String>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
                    let _: ::std::cmp::AssertParamIsEq<Option<usize>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::hash::Hash + HeaderT> ::std::hash::Hash for
         RemoteCallRequest<Header> where Header::Hash: ::std::hash::Hash {
            fn hash<__HHeader: ::std::hash::Hasher>(&self,
                                                    state: &mut __HHeader)
             -> () {
                match *self {
                    RemoteCallRequest {
                    block: ref __self_0_0,
                    header: ref __self_0_1,
                    method: ref __self_0_2,
                    call_data: ref __self_0_3,
                    retry_count: ref __self_0_4 } => {
                        ::std::hash::Hash::hash(&(*__self_0_0), state);
                        ::std::hash::Hash::hash(&(*__self_0_1), state);
                        ::std::hash::Hash::hash(&(*__self_0_2), state);
                        ::std::hash::Hash::hash(&(*__self_0_3), state);
                        ::std::hash::Hash::hash(&(*__self_0_4), state)
                    }
                }
            }
        }
        /// Remote canonical header request.
        #[structural_match]
        pub struct RemoteHeaderRequest<Header: HeaderT> {
            /// The root of CHT this block is included in.
            pub cht_root: Header::Hash,
            /// Number of the header to query.
            pub block: Header::Number,
            /// Number of times to retry request. None means that default RETRY_COUNT is used.
            pub retry_count: Option<usize>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::clone::Clone + HeaderT> ::std::clone::Clone for
         RemoteHeaderRequest<Header> where Header::Hash: ::std::clone::Clone,
         Header::Number: ::std::clone::Clone {
            #[inline]
            fn clone(&self) -> RemoteHeaderRequest<Header> {
                match *self {
                    RemoteHeaderRequest {
                    cht_root: ref __self_0_0,
                    block: ref __self_0_1,
                    retry_count: ref __self_0_2 } =>
                    RemoteHeaderRequest{cht_root:
                                            ::std::clone::Clone::clone(&(*__self_0_0)),
                                        block:
                                            ::std::clone::Clone::clone(&(*__self_0_1)),
                                        retry_count:
                                            ::std::clone::Clone::clone(&(*__self_0_2)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::fmt::Debug + HeaderT> ::std::fmt::Debug for
         RemoteHeaderRequest<Header> where Header::Hash: ::std::fmt::Debug,
         Header::Number: ::std::fmt::Debug {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    RemoteHeaderRequest {
                    cht_root: ref __self_0_0,
                    block: ref __self_0_1,
                    retry_count: ref __self_0_2 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("RemoteHeaderRequest");
                        let _ =
                            debug_trait_builder.field("cht_root",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("block",
                                                      &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("retry_count",
                                                      &&(*__self_0_2));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::default::Default + HeaderT>
         ::std::default::Default for RemoteHeaderRequest<Header> where
         Header::Hash: ::std::default::Default,
         Header::Number: ::std::default::Default {
            #[inline]
            fn default() -> RemoteHeaderRequest<Header> {
                RemoteHeaderRequest{cht_root:
                                        ::std::default::Default::default(),
                                    block: ::std::default::Default::default(),
                                    retry_count:
                                        ::std::default::Default::default(),}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::PartialEq + HeaderT> ::std::cmp::PartialEq
         for RemoteHeaderRequest<Header> where
         Header::Hash: ::std::cmp::PartialEq,
         Header::Number: ::std::cmp::PartialEq {
            #[inline]
            fn eq(&self, other: &RemoteHeaderRequest<Header>) -> bool {
                match *other {
                    RemoteHeaderRequest {
                    cht_root: ref __self_1_0,
                    block: ref __self_1_1,
                    retry_count: ref __self_1_2 } =>
                    match *self {
                        RemoteHeaderRequest {
                        cht_root: ref __self_0_0,
                        block: ref __self_0_1,
                        retry_count: ref __self_0_2 } =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RemoteHeaderRequest<Header>) -> bool {
                match *other {
                    RemoteHeaderRequest {
                    cht_root: ref __self_1_0,
                    block: ref __self_1_1,
                    retry_count: ref __self_1_2 } =>
                    match *self {
                        RemoteHeaderRequest {
                        cht_root: ref __self_0_0,
                        block: ref __self_0_1,
                        retry_count: ref __self_0_2 } =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::Eq + HeaderT> ::std::cmp::Eq for
         RemoteHeaderRequest<Header> where Header::Hash: ::std::cmp::Eq,
         Header::Number: ::std::cmp::Eq {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Header::Hash>;
                    let _: ::std::cmp::AssertParamIsEq<Header::Number>;
                    let _: ::std::cmp::AssertParamIsEq<Option<usize>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::hash::Hash + HeaderT> ::std::hash::Hash for
         RemoteHeaderRequest<Header> where Header::Hash: ::std::hash::Hash,
         Header::Number: ::std::hash::Hash {
            fn hash<__HHeader: ::std::hash::Hasher>(&self,
                                                    state: &mut __HHeader)
             -> () {
                match *self {
                    RemoteHeaderRequest {
                    cht_root: ref __self_0_0,
                    block: ref __self_0_1,
                    retry_count: ref __self_0_2 } => {
                        ::std::hash::Hash::hash(&(*__self_0_0), state);
                        ::std::hash::Hash::hash(&(*__self_0_1), state);
                        ::std::hash::Hash::hash(&(*__self_0_2), state)
                    }
                }
            }
        }
        /// Remote storage read request.
        #[structural_match]
        pub struct RemoteReadRequest<Header: HeaderT> {
            /// Read at state of given block.
            pub block: Header::Hash,
            /// Header of block at which read is performed.
            pub header: Header,
            /// Storage key to read.
            pub key: Vec<u8>,
            /// Number of times to retry request. None means that default RETRY_COUNT is used.
            pub retry_count: Option<usize>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::clone::Clone + HeaderT> ::std::clone::Clone for
         RemoteReadRequest<Header> where Header::Hash: ::std::clone::Clone {
            #[inline]
            fn clone(&self) -> RemoteReadRequest<Header> {
                match *self {
                    RemoteReadRequest {
                    block: ref __self_0_0,
                    header: ref __self_0_1,
                    key: ref __self_0_2,
                    retry_count: ref __self_0_3 } =>
                    RemoteReadRequest{block:
                                          ::std::clone::Clone::clone(&(*__self_0_0)),
                                      header:
                                          ::std::clone::Clone::clone(&(*__self_0_1)),
                                      key:
                                          ::std::clone::Clone::clone(&(*__self_0_2)),
                                      retry_count:
                                          ::std::clone::Clone::clone(&(*__self_0_3)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::fmt::Debug + HeaderT> ::std::fmt::Debug for
         RemoteReadRequest<Header> where Header::Hash: ::std::fmt::Debug {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    RemoteReadRequest {
                    block: ref __self_0_0,
                    header: ref __self_0_1,
                    key: ref __self_0_2,
                    retry_count: ref __self_0_3 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("RemoteReadRequest");
                        let _ =
                            debug_trait_builder.field("block",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("header",
                                                      &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("key", &&(*__self_0_2));
                        let _ =
                            debug_trait_builder.field("retry_count",
                                                      &&(*__self_0_3));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::PartialEq + HeaderT> ::std::cmp::PartialEq
         for RemoteReadRequest<Header> where
         Header::Hash: ::std::cmp::PartialEq {
            #[inline]
            fn eq(&self, other: &RemoteReadRequest<Header>) -> bool {
                match *other {
                    RemoteReadRequest {
                    block: ref __self_1_0,
                    header: ref __self_1_1,
                    key: ref __self_1_2,
                    retry_count: ref __self_1_3 } =>
                    match *self {
                        RemoteReadRequest {
                        block: ref __self_0_0,
                        header: ref __self_0_1,
                        key: ref __self_0_2,
                        retry_count: ref __self_0_3 } =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RemoteReadRequest<Header>) -> bool {
                match *other {
                    RemoteReadRequest {
                    block: ref __self_1_0,
                    header: ref __self_1_1,
                    key: ref __self_1_2,
                    retry_count: ref __self_1_3 } =>
                    match *self {
                        RemoteReadRequest {
                        block: ref __self_0_0,
                        header: ref __self_0_1,
                        key: ref __self_0_2,
                        retry_count: ref __self_0_3 } =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::Eq + HeaderT> ::std::cmp::Eq for
         RemoteReadRequest<Header> where Header::Hash: ::std::cmp::Eq {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Header::Hash>;
                    let _: ::std::cmp::AssertParamIsEq<Header>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
                    let _: ::std::cmp::AssertParamIsEq<Option<usize>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::hash::Hash + HeaderT> ::std::hash::Hash for
         RemoteReadRequest<Header> where Header::Hash: ::std::hash::Hash {
            fn hash<__HHeader: ::std::hash::Hasher>(&self,
                                                    state: &mut __HHeader)
             -> () {
                match *self {
                    RemoteReadRequest {
                    block: ref __self_0_0,
                    header: ref __self_0_1,
                    key: ref __self_0_2,
                    retry_count: ref __self_0_3 } => {
                        ::std::hash::Hash::hash(&(*__self_0_0), state);
                        ::std::hash::Hash::hash(&(*__self_0_1), state);
                        ::std::hash::Hash::hash(&(*__self_0_2), state);
                        ::std::hash::Hash::hash(&(*__self_0_3), state)
                    }
                }
            }
        }
        /// Remote storage read child request.
        #[structural_match]
        pub struct RemoteReadChildRequest<Header: HeaderT> {
            /// Read at state of given block.
            pub block: Header::Hash,
            /// Header of block at which read is performed.
            pub header: Header,
            /// Storage key for child.
            pub storage_key: Vec<u8>,
            /// Child storage key to read.
            pub key: Vec<u8>,
            /// Number of times to retry request. None means that default RETRY_COUNT is used.
            pub retry_count: Option<usize>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::clone::Clone + HeaderT> ::std::clone::Clone for
         RemoteReadChildRequest<Header> where
         Header::Hash: ::std::clone::Clone {
            #[inline]
            fn clone(&self) -> RemoteReadChildRequest<Header> {
                match *self {
                    RemoteReadChildRequest {
                    block: ref __self_0_0,
                    header: ref __self_0_1,
                    storage_key: ref __self_0_2,
                    key: ref __self_0_3,
                    retry_count: ref __self_0_4 } =>
                    RemoteReadChildRequest{block:
                                               ::std::clone::Clone::clone(&(*__self_0_0)),
                                           header:
                                               ::std::clone::Clone::clone(&(*__self_0_1)),
                                           storage_key:
                                               ::std::clone::Clone::clone(&(*__self_0_2)),
                                           key:
                                               ::std::clone::Clone::clone(&(*__self_0_3)),
                                           retry_count:
                                               ::std::clone::Clone::clone(&(*__self_0_4)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::fmt::Debug + HeaderT> ::std::fmt::Debug for
         RemoteReadChildRequest<Header> where Header::Hash: ::std::fmt::Debug
         {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    RemoteReadChildRequest {
                    block: ref __self_0_0,
                    header: ref __self_0_1,
                    storage_key: ref __self_0_2,
                    key: ref __self_0_3,
                    retry_count: ref __self_0_4 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("RemoteReadChildRequest");
                        let _ =
                            debug_trait_builder.field("block",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("header",
                                                      &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("storage_key",
                                                      &&(*__self_0_2));
                        let _ =
                            debug_trait_builder.field("key", &&(*__self_0_3));
                        let _ =
                            debug_trait_builder.field("retry_count",
                                                      &&(*__self_0_4));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::PartialEq + HeaderT> ::std::cmp::PartialEq
         for RemoteReadChildRequest<Header> where
         Header::Hash: ::std::cmp::PartialEq {
            #[inline]
            fn eq(&self, other: &RemoteReadChildRequest<Header>) -> bool {
                match *other {
                    RemoteReadChildRequest {
                    block: ref __self_1_0,
                    header: ref __self_1_1,
                    storage_key: ref __self_1_2,
                    key: ref __self_1_3,
                    retry_count: ref __self_1_4 } =>
                    match *self {
                        RemoteReadChildRequest {
                        block: ref __self_0_0,
                        header: ref __self_0_1,
                        storage_key: ref __self_0_2,
                        key: ref __self_0_3,
                        retry_count: ref __self_0_4 } =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RemoteReadChildRequest<Header>) -> bool {
                match *other {
                    RemoteReadChildRequest {
                    block: ref __self_1_0,
                    header: ref __self_1_1,
                    storage_key: ref __self_1_2,
                    key: ref __self_1_3,
                    retry_count: ref __self_1_4 } =>
                    match *self {
                        RemoteReadChildRequest {
                        block: ref __self_0_0,
                        header: ref __self_0_1,
                        storage_key: ref __self_0_2,
                        key: ref __self_0_3,
                        retry_count: ref __self_0_4 } =>
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
        impl <Header: ::std::cmp::Eq + HeaderT> ::std::cmp::Eq for
         RemoteReadChildRequest<Header> where Header::Hash: ::std::cmp::Eq {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Header::Hash>;
                    let _: ::std::cmp::AssertParamIsEq<Header>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
                    let _: ::std::cmp::AssertParamIsEq<Option<usize>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::hash::Hash + HeaderT> ::std::hash::Hash for
         RemoteReadChildRequest<Header> where Header::Hash: ::std::hash::Hash
         {
            fn hash<__HHeader: ::std::hash::Hasher>(&self,
                                                    state: &mut __HHeader)
             -> () {
                match *self {
                    RemoteReadChildRequest {
                    block: ref __self_0_0,
                    header: ref __self_0_1,
                    storage_key: ref __self_0_2,
                    key: ref __self_0_3,
                    retry_count: ref __self_0_4 } => {
                        ::std::hash::Hash::hash(&(*__self_0_0), state);
                        ::std::hash::Hash::hash(&(*__self_0_1), state);
                        ::std::hash::Hash::hash(&(*__self_0_2), state);
                        ::std::hash::Hash::hash(&(*__self_0_3), state);
                        ::std::hash::Hash::hash(&(*__self_0_4), state)
                    }
                }
            }
        }
        /// Remote key changes read request.
        #[structural_match]
        pub struct RemoteChangesRequest<Header: HeaderT> {
            /// Changes trie configuration.
            pub changes_trie_config: ChangesTrieConfiguration,
            /// Query changes from range of blocks, starting (and including) with this hash...
            pub first_block: (Header::Number, Header::Hash),
            /// ...ending (and including) with this hash. Should come after first_block and
            /// be the part of the same fork.
            pub last_block: (Header::Number, Header::Hash),
            /// Only use digests from blocks up to this hash. Should be last_block OR come
            /// after this block and be the part of the same fork.
            pub max_block: (Header::Number, Header::Hash),
            /// Known changes trie roots for the range of blocks [tries_roots.0..max_block].
            /// Proofs for roots of ascendants of tries_roots.0 are provided by the remote node.
            pub tries_roots: (Header::Number, Header::Hash,
                              Vec<Header::Hash>),
            /// Storage key to read.
            pub key: Vec<u8>,
            /// Number of times to retry request. None means that default RETRY_COUNT is used.
            pub retry_count: Option<usize>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::clone::Clone + HeaderT> ::std::clone::Clone for
         RemoteChangesRequest<Header> where
         Header::Number: ::std::clone::Clone,
         Header::Hash: ::std::clone::Clone,
         Header::Number: ::std::clone::Clone,
         Header::Hash: ::std::clone::Clone,
         Header::Number: ::std::clone::Clone,
         Header::Hash: ::std::clone::Clone,
         Header::Number: ::std::clone::Clone,
         Header::Hash: ::std::clone::Clone, Header::Hash: ::std::clone::Clone
         {
            #[inline]
            fn clone(&self) -> RemoteChangesRequest<Header> {
                match *self {
                    RemoteChangesRequest {
                    changes_trie_config: ref __self_0_0,
                    first_block: ref __self_0_1,
                    last_block: ref __self_0_2,
                    max_block: ref __self_0_3,
                    tries_roots: ref __self_0_4,
                    key: ref __self_0_5,
                    retry_count: ref __self_0_6 } =>
                    RemoteChangesRequest{changes_trie_config:
                                             ::std::clone::Clone::clone(&(*__self_0_0)),
                                         first_block:
                                             ::std::clone::Clone::clone(&(*__self_0_1)),
                                         last_block:
                                             ::std::clone::Clone::clone(&(*__self_0_2)),
                                         max_block:
                                             ::std::clone::Clone::clone(&(*__self_0_3)),
                                         tries_roots:
                                             ::std::clone::Clone::clone(&(*__self_0_4)),
                                         key:
                                             ::std::clone::Clone::clone(&(*__self_0_5)),
                                         retry_count:
                                             ::std::clone::Clone::clone(&(*__self_0_6)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::fmt::Debug + HeaderT> ::std::fmt::Debug for
         RemoteChangesRequest<Header> where Header::Number: ::std::fmt::Debug,
         Header::Hash: ::std::fmt::Debug, Header::Number: ::std::fmt::Debug,
         Header::Hash: ::std::fmt::Debug, Header::Number: ::std::fmt::Debug,
         Header::Hash: ::std::fmt::Debug, Header::Number: ::std::fmt::Debug,
         Header::Hash: ::std::fmt::Debug, Header::Hash: ::std::fmt::Debug {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    RemoteChangesRequest {
                    changes_trie_config: ref __self_0_0,
                    first_block: ref __self_0_1,
                    last_block: ref __self_0_2,
                    max_block: ref __self_0_3,
                    tries_roots: ref __self_0_4,
                    key: ref __self_0_5,
                    retry_count: ref __self_0_6 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("RemoteChangesRequest");
                        let _ =
                            debug_trait_builder.field("changes_trie_config",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("first_block",
                                                      &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("last_block",
                                                      &&(*__self_0_2));
                        let _ =
                            debug_trait_builder.field("max_block",
                                                      &&(*__self_0_3));
                        let _ =
                            debug_trait_builder.field("tries_roots",
                                                      &&(*__self_0_4));
                        let _ =
                            debug_trait_builder.field("key", &&(*__self_0_5));
                        let _ =
                            debug_trait_builder.field("retry_count",
                                                      &&(*__self_0_6));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::PartialEq + HeaderT> ::std::cmp::PartialEq
         for RemoteChangesRequest<Header> where
         Header::Number: ::std::cmp::PartialEq,
         Header::Hash: ::std::cmp::PartialEq,
         Header::Number: ::std::cmp::PartialEq,
         Header::Hash: ::std::cmp::PartialEq,
         Header::Number: ::std::cmp::PartialEq,
         Header::Hash: ::std::cmp::PartialEq,
         Header::Number: ::std::cmp::PartialEq,
         Header::Hash: ::std::cmp::PartialEq,
         Header::Hash: ::std::cmp::PartialEq {
            #[inline]
            fn eq(&self, other: &RemoteChangesRequest<Header>) -> bool {
                match *other {
                    RemoteChangesRequest {
                    changes_trie_config: ref __self_1_0,
                    first_block: ref __self_1_1,
                    last_block: ref __self_1_2,
                    max_block: ref __self_1_3,
                    tries_roots: ref __self_1_4,
                    key: ref __self_1_5,
                    retry_count: ref __self_1_6 } =>
                    match *self {
                        RemoteChangesRequest {
                        changes_trie_config: ref __self_0_0,
                        first_block: ref __self_0_1,
                        last_block: ref __self_0_2,
                        max_block: ref __self_0_3,
                        tries_roots: ref __self_0_4,
                        key: ref __self_0_5,
                        retry_count: ref __self_0_6 } =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3) &&
                            (*__self_0_4) == (*__self_1_4) &&
                            (*__self_0_5) == (*__self_1_5) &&
                            (*__self_0_6) == (*__self_1_6),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RemoteChangesRequest<Header>) -> bool {
                match *other {
                    RemoteChangesRequest {
                    changes_trie_config: ref __self_1_0,
                    first_block: ref __self_1_1,
                    last_block: ref __self_1_2,
                    max_block: ref __self_1_3,
                    tries_roots: ref __self_1_4,
                    key: ref __self_1_5,
                    retry_count: ref __self_1_6 } =>
                    match *self {
                        RemoteChangesRequest {
                        changes_trie_config: ref __self_0_0,
                        first_block: ref __self_0_1,
                        last_block: ref __self_0_2,
                        max_block: ref __self_0_3,
                        tries_roots: ref __self_0_4,
                        key: ref __self_0_5,
                        retry_count: ref __self_0_6 } =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3) ||
                            (*__self_0_4) != (*__self_1_4) ||
                            (*__self_0_5) != (*__self_1_5) ||
                            (*__self_0_6) != (*__self_1_6),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::Eq + HeaderT> ::std::cmp::Eq for
         RemoteChangesRequest<Header> where Header::Number: ::std::cmp::Eq,
         Header::Hash: ::std::cmp::Eq, Header::Number: ::std::cmp::Eq,
         Header::Hash: ::std::cmp::Eq, Header::Number: ::std::cmp::Eq,
         Header::Hash: ::std::cmp::Eq, Header::Number: ::std::cmp::Eq,
         Header::Hash: ::std::cmp::Eq, Header::Hash: ::std::cmp::Eq {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _:
                            ::std::cmp::AssertParamIsEq<ChangesTrieConfiguration>;
                    let _:
                            ::std::cmp::AssertParamIsEq<(Header::Number,
                                                         Header::Hash)>;
                    let _:
                            ::std::cmp::AssertParamIsEq<(Header::Number,
                                                         Header::Hash)>;
                    let _:
                            ::std::cmp::AssertParamIsEq<(Header::Number,
                                                         Header::Hash)>;
                    let _:
                            ::std::cmp::AssertParamIsEq<(Header::Number,
                                                         Header::Hash,
                                                         Vec<Header::Hash>)>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<u8>>;
                    let _: ::std::cmp::AssertParamIsEq<Option<usize>>;
                }
            }
        }
        /// Key changes read proof.
        #[structural_match]
        pub struct ChangesProof<Header: HeaderT> {
            /// Max block that has been used in changes query.
            pub max_block: Header::Number,
            /// All touched nodes of all changes tries.
            pub proof: Vec<Vec<u8>>,
            /// All changes tries roots that have been touched AND are missing from
            /// the requester' node. It is a map of block number => changes trie root.
            pub roots: BTreeMap<Header::Number, Header::Hash>,
            /// The proofs for all changes tries roots that have been touched AND are
            /// missing from the requester' node. It is a map of CHT number => proof.
            pub roots_proof: Vec<Vec<u8>>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::fmt::Debug + HeaderT> ::std::fmt::Debug for
         ChangesProof<Header> where Header::Number: ::std::fmt::Debug,
         Header::Number: ::std::fmt::Debug, Header::Hash: ::std::fmt::Debug {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    ChangesProof {
                    max_block: ref __self_0_0,
                    proof: ref __self_0_1,
                    roots: ref __self_0_2,
                    roots_proof: ref __self_0_3 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("ChangesProof");
                        let _ =
                            debug_trait_builder.field("max_block",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("proof",
                                                      &&(*__self_0_1));
                        let _ =
                            debug_trait_builder.field("roots",
                                                      &&(*__self_0_2));
                        let _ =
                            debug_trait_builder.field("roots_proof",
                                                      &&(*__self_0_3));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::PartialEq + HeaderT> ::std::cmp::PartialEq
         for ChangesProof<Header> where Header::Number: ::std::cmp::PartialEq,
         Header::Number: ::std::cmp::PartialEq,
         Header::Hash: ::std::cmp::PartialEq {
            #[inline]
            fn eq(&self, other: &ChangesProof<Header>) -> bool {
                match *other {
                    ChangesProof {
                    max_block: ref __self_1_0,
                    proof: ref __self_1_1,
                    roots: ref __self_1_2,
                    roots_proof: ref __self_1_3 } =>
                    match *self {
                        ChangesProof {
                        max_block: ref __self_0_0,
                        proof: ref __self_0_1,
                        roots: ref __self_0_2,
                        roots_proof: ref __self_0_3 } =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1) &&
                            (*__self_0_2) == (*__self_1_2) &&
                            (*__self_0_3) == (*__self_1_3),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &ChangesProof<Header>) -> bool {
                match *other {
                    ChangesProof {
                    max_block: ref __self_1_0,
                    proof: ref __self_1_1,
                    roots: ref __self_1_2,
                    roots_proof: ref __self_1_3 } =>
                    match *self {
                        ChangesProof {
                        max_block: ref __self_0_0,
                        proof: ref __self_0_1,
                        roots: ref __self_0_2,
                        roots_proof: ref __self_0_3 } =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1) ||
                            (*__self_0_2) != (*__self_1_2) ||
                            (*__self_0_3) != (*__self_1_3),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::Eq + HeaderT> ::std::cmp::Eq for
         ChangesProof<Header> where Header::Number: ::std::cmp::Eq,
         Header::Number: ::std::cmp::Eq, Header::Hash: ::std::cmp::Eq {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Header::Number>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<Vec<u8>>>;
                    let _:
                            ::std::cmp::AssertParamIsEq<BTreeMap<Header::Number,
                                                                 Header::Hash>>;
                    let _: ::std::cmp::AssertParamIsEq<Vec<Vec<u8>>>;
                }
            }
        }
        /// Remote block body request
        #[structural_match]
        pub struct RemoteBodyRequest<Header: HeaderT> {
            /// Header of the requested block body
            pub header: Header,
            /// Number of times to retry request. None means that default RETRY_COUNT is used.
            pub retry_count: Option<usize>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::clone::Clone + HeaderT> ::std::clone::Clone for
         RemoteBodyRequest<Header> {
            #[inline]
            fn clone(&self) -> RemoteBodyRequest<Header> {
                match *self {
                    RemoteBodyRequest {
                    header: ref __self_0_0, retry_count: ref __self_0_1 } =>
                    RemoteBodyRequest{header:
                                          ::std::clone::Clone::clone(&(*__self_0_0)),
                                      retry_count:
                                          ::std::clone::Clone::clone(&(*__self_0_1)),},
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::default::Default + HeaderT>
         ::std::default::Default for RemoteBodyRequest<Header> {
            #[inline]
            fn default() -> RemoteBodyRequest<Header> {
                RemoteBodyRequest{header: ::std::default::Default::default(),
                                  retry_count:
                                      ::std::default::Default::default(),}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::fmt::Debug + HeaderT> ::std::fmt::Debug for
         RemoteBodyRequest<Header> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    RemoteBodyRequest {
                    header: ref __self_0_0, retry_count: ref __self_0_1 } => {
                        let mut debug_trait_builder =
                            f.debug_struct("RemoteBodyRequest");
                        let _ =
                            debug_trait_builder.field("header",
                                                      &&(*__self_0_0));
                        let _ =
                            debug_trait_builder.field("retry_count",
                                                      &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::PartialEq + HeaderT> ::std::cmp::PartialEq
         for RemoteBodyRequest<Header> {
            #[inline]
            fn eq(&self, other: &RemoteBodyRequest<Header>) -> bool {
                match *other {
                    RemoteBodyRequest {
                    header: ref __self_1_0, retry_count: ref __self_1_1 } =>
                    match *self {
                        RemoteBodyRequest {
                        header: ref __self_0_0, retry_count: ref __self_0_1 }
                        =>
                        (*__self_0_0) == (*__self_1_0) &&
                            (*__self_0_1) == (*__self_1_1),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &RemoteBodyRequest<Header>) -> bool {
                match *other {
                    RemoteBodyRequest {
                    header: ref __self_1_0, retry_count: ref __self_1_1 } =>
                    match *self {
                        RemoteBodyRequest {
                        header: ref __self_0_0, retry_count: ref __self_0_1 }
                        =>
                        (*__self_0_0) != (*__self_1_0) ||
                            (*__self_0_1) != (*__self_1_1),
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::cmp::Eq + HeaderT> ::std::cmp::Eq for
         RemoteBodyRequest<Header> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::std::cmp::AssertParamIsEq<Header>;
                    let _: ::std::cmp::AssertParamIsEq<Option<usize>>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <Header: ::std::hash::Hash + HeaderT> ::std::hash::Hash for
         RemoteBodyRequest<Header> {
            fn hash<__HHeader: ::std::hash::Hasher>(&self,
                                                    state: &mut __HHeader)
             -> () {
                match *self {
                    RemoteBodyRequest {
                    header: ref __self_0_0, retry_count: ref __self_0_1 } => {
                        ::std::hash::Hash::hash(&(*__self_0_0), state);
                        ::std::hash::Hash::hash(&(*__self_0_1), state)
                    }
                }
            }
        }
        /// Light client data fetcher. Implementations of this trait must check if remote data
        /// is correct (see FetchedDataChecker) and return already checked data.
        pub trait Fetcher<Block: BlockT>: Send + Sync {
            /// Remote header future.
            type
            RemoteHeaderResult: IntoFuture<Item
            =
            Block::Header,
            Error
            =
            ClientError>;
            /// Remote storage read future.
            type
            RemoteReadResult: IntoFuture<Item
            =
            Option<Vec<u8>>,
            Error
            =
            ClientError>;
            /// Remote call result future.
            type
            RemoteCallResult: IntoFuture<Item
            =
            Vec<u8>,
            Error
            =
            ClientError>;
            /// Remote changes result future.
            type
            RemoteChangesResult: IntoFuture<Item
            =
            Vec<(NumberFor<Block>, u32)>,
            Error
            =
            ClientError>;
            /// Remote block body result future.
            type
            RemoteBodyResult: IntoFuture<Item
            =
            Vec<Block::Extrinsic>,
            Error
            =
            ClientError>;
            /// Fetch remote header.
            fn remote_header(&self,
                             request: RemoteHeaderRequest<Block::Header>)
            -> Self::RemoteHeaderResult;
            /// Fetch remote storage value.
            fn remote_read(&self, request: RemoteReadRequest<Block::Header>)
            -> Self::RemoteReadResult;
            /// Fetch remote storage child value.
            fn remote_read_child(&self,
                                 request:
                                     RemoteReadChildRequest<Block::Header>)
            -> Self::RemoteReadResult;
            /// Fetch remote call result.
            fn remote_call(&self, request: RemoteCallRequest<Block::Header>)
            -> Self::RemoteCallResult;
            /// Fetch remote changes ((block number, extrinsic index)) where given key has been changed
            /// at a given blocks range.
            fn remote_changes(&self,
                              request: RemoteChangesRequest<Block::Header>)
            -> Self::RemoteChangesResult;
            /// Fetch remote block body
            fn remote_body(&self, request: RemoteBodyRequest<Block::Header>)
            -> Self::RemoteBodyResult;
        }
        /// Light client remote data checker.
        ///
        /// Implementations of this trait should not use any prunable blockchain data
        /// except that is passed to its methods.
        pub trait FetchChecker<Block: BlockT>: Send + Sync {
            /// Check remote header proof.
            fn check_header_proof(&self,
                                  request:
                                      &RemoteHeaderRequest<Block::Header>,
                                  header: Option<Block::Header>,
                                  remote_proof: Vec<Vec<u8>>)
            -> ClientResult<Block::Header>;
            /// Check remote storage read proof.
            fn check_read_proof(&self,
                                request: &RemoteReadRequest<Block::Header>,
                                remote_proof: Vec<Vec<u8>>)
            -> ClientResult<Option<Vec<u8>>>;
            /// Check remote storage read proof.
            fn check_read_child_proof(&self,
                                      request:
                                          &RemoteReadChildRequest<Block::Header>,
                                      remote_proof: Vec<Vec<u8>>)
            -> ClientResult<Option<Vec<u8>>>;
            /// Check remote method execution proof.
            fn check_execution_proof(&self,
                                     request:
                                         &RemoteCallRequest<Block::Header>,
                                     remote_proof: Vec<Vec<u8>>)
            -> ClientResult<Vec<u8>>;
            /// Check remote changes query proof.
            fn check_changes_proof(&self,
                                   request:
                                       &RemoteChangesRequest<Block::Header>,
                                   proof: ChangesProof<Block::Header>)
            -> ClientResult<Vec<(NumberFor<Block>, u32)>>;
            /// Check remote body proof.
            fn check_body_proof(&self,
                                request: &RemoteBodyRequest<Block::Header>,
                                body: Vec<Block::Extrinsic>)
            -> ClientResult<Vec<Block::Extrinsic>>;
        }
        /// Remote data checker.
        pub struct LightDataChecker<E, H, B: BlockT, S: BlockchainStorage<B>,
                                    F> {
            blockchain: Arc<Blockchain<S, F>>,
            executor: E,
            _hasher: PhantomData<(B, H)>,
        }
        impl <E, H, B: BlockT, S: BlockchainStorage<B>, F>
         LightDataChecker<E, H, B, S, F> {
            /// Create new light data checker.
            pub fn new(blockchain: Arc<Blockchain<S, F>>, executor: E)
             -> Self {
                Self{blockchain, executor, _hasher: PhantomData,}
            }
            /// Check remote changes query proof assuming that CHT-s are of given size.
            fn check_changes_proof_with_cht_size(&self,
                                                 request:
                                                     &RemoteChangesRequest<B::Header>,
                                                 remote_proof:
                                                     ChangesProof<B::Header>,
                                                 cht_size: u64)
             -> ClientResult<Vec<(NumberFor<B>, u32)>> where H: Hasher,
             H::Out: Ord {
                if remote_proof.max_block > request.max_block.0 ||
                       remote_proof.max_block < request.last_block.0 {
                    return Err(ClientError::ChangesTrieAccessFailed(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Invalid max_block used by the remote node: ",
                                                                                                                         ". Local: ",
                                                                                                                         "..",
                                                                                                                         ".."],
                                                                                                                       &match (&remote_proof.max_block,
                                                                                                                               &request.first_block.0,
                                                                                                                               &request.last_block.0,
                                                                                                                               &request.max_block.0)
                                                                                                                            {
                                                                                                                            (arg0,
                                                                                                                             arg1,
                                                                                                                             arg2,
                                                                                                                             arg3)
                                                                                                                            =>
                                                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                         ::std::fmt::Display::fmt),
                                                                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                         ::std::fmt::Display::fmt),
                                                                                                                             ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                                                         ::std::fmt::Display::fmt),
                                                                                                                             ::std::fmt::ArgumentV1::new(arg3,
                                                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                                                        }))).into());
                }
                let is_extra_first_root =
                    remote_proof.roots.keys().next().map(|first_root|
                                                             *first_root <
                                                                 request.first_block.0
                                                                 ||
                                                                 *first_root
                                                                     >=
                                                                     request.tries_roots.0).unwrap_or(false);
                let is_extra_last_root =
                    remote_proof.roots.keys().next_back().map(|last_root|
                                                                  *last_root
                                                                      >=
                                                                      request.tries_roots.0).unwrap_or(false);
                if is_extra_first_root || is_extra_last_root {
                    return Err(ClientError::ChangesTrieAccessFailed(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Extra changes tries roots proofs provided by the remote node: [",
                                                                                                                         "..",
                                                                                                                         "]. Expected in range: [",
                                                                                                                         "; ",
                                                                                                                         ")"],
                                                                                                                       &match (&remote_proof.roots.keys().next(),
                                                                                                                               &remote_proof.roots.keys().next_back(),
                                                                                                                               &request.first_block.0,
                                                                                                                               &request.tries_roots.0)
                                                                                                                            {
                                                                                                                            (arg0,
                                                                                                                             arg1,
                                                                                                                             arg2,
                                                                                                                             arg3)
                                                                                                                            =>
                                                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                         ::std::fmt::Debug::fmt),
                                                                                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                                                         ::std::fmt::Debug::fmt),
                                                                                                                             ::std::fmt::ArgumentV1::new(arg2,
                                                                                                                                                         ::std::fmt::Display::fmt),
                                                                                                                             ::std::fmt::ArgumentV1::new(arg3,
                                                                                                                                                         ::std::fmt::Display::fmt)],
                                                                                                                        }))).into());
                }
                let remote_max_block = remote_proof.max_block;
                let remote_roots = remote_proof.roots;
                let remote_roots_proof = remote_proof.roots_proof;
                let remote_proof = remote_proof.proof;
                if !remote_roots.is_empty() {
                    self.check_changes_tries_proof(cht_size, &remote_roots,
                                                   remote_roots_proof)?;
                }
                key_changes_proof_check::<_,
                                          H>(&request.changes_trie_config,
                                             &RootsStorage{roots:
                                                               (request.tries_roots.0,
                                                                &request.tries_roots.2),
                                                           prev_roots:
                                                               remote_roots,},
                                             remote_proof,
                                             request.first_block.0.as_(),
                                             &ChangesTrieAnchorBlockId{hash:
                                                                           convert_hash(&request.last_block.1),
                                                                       number:
                                                                           request.last_block.0.as_(),},
                                             remote_max_block.as_(),
                                             &request.key).map(|pairs|
                                                                   pairs.into_iter().map(|(b,
                                                                                           x)|
                                                                                             (As::sa(b),
                                                                                              x)).collect()).map_err(|err|
                                                                                                                         ClientError::ChangesTrieAccessFailed(err))
            }
            /// Check CHT-based proof for changes tries roots.
            fn check_changes_tries_proof(&self, cht_size: u64,
                                         remote_roots:
                                             &BTreeMap<NumberFor<B>, B::Hash>,
                                         remote_roots_proof: Vec<Vec<u8>>)
             -> ClientResult<()> where H: Hasher, H::Out: Ord {
                let storage =
                    create_proof_check_backend_storage(remote_roots_proof);
                let blocks = remote_roots.keys().cloned();
                cht::for_each_cht_group::<B::Header, _, _,
                                          _>(cht_size, blocks,
                                             |mut storage, _, cht_blocks|
                                                 {
                                                     let first_block =
                                                         cht_blocks.first().cloned().expect("for_each_cht_group never calls callback with empty groups");
                                                     let local_cht_root =
                                                         self.blockchain.storage().changes_trie_cht_root(cht_size,
                                                                                                         first_block)?;
                                                     for block in cht_blocks {
                                                         let mut cht_root =
                                                             H::Out::default();
                                                         cht_root.as_mut().copy_from_slice(local_cht_root.as_ref());
                                                         if !storage.contains(&cht_root,
                                                                              &[])
                                                            {
                                                             return Err(ClientError::InvalidCHTProof.into());
                                                         }
                                                         let proving_backend =
                                                             TrieBackend::new(storage,
                                                                              cht_root);
                                                         let remote_changes_trie_root =
                                                             remote_roots[&block];
                                                         cht::check_proof_on_proving_backend::<B::Header,
                                                                                               H>(local_cht_root,
                                                                                                  block,
                                                                                                  remote_changes_trie_root,
                                                                                                  &proving_backend)?;
                                                         storage =
                                                             proving_backend.into_storage();
                                                     }
                                                     Ok(storage)
                                                 }, storage)
            }
        }
        impl <E, Block, H, S, F> FetchChecker<Block> for
         LightDataChecker<E, H, Block, S, F> where Block: BlockT,
         E: CodeExecutor<H>, H: Hasher, H::Out: Ord,
         S: BlockchainStorage<Block>, F: Send + Sync {
            fn check_header_proof(&self,
                                  request:
                                      &RemoteHeaderRequest<Block::Header>,
                                  remote_header: Option<Block::Header>,
                                  remote_proof: Vec<Vec<u8>>)
             -> ClientResult<Block::Header> {
                let remote_header =
                    remote_header.ok_or_else(||
                                                 ClientError::from(ClientError::InvalidCHTProof))?;
                let remote_header_hash = remote_header.hash();
                cht::check_proof::<Block::Header,
                                   H>(request.cht_root, request.block,
                                      remote_header_hash,
                                      remote_proof).map(|_| remote_header)
            }
            fn check_read_proof(&self,
                                request: &RemoteReadRequest<Block::Header>,
                                remote_proof: Vec<Vec<u8>>)
             -> ClientResult<Option<Vec<u8>>> {
                read_proof_check::<H>(convert_hash(request.header.state_root()),
                                      remote_proof,
                                      &request.key).map_err(Into::into)
            }
            fn check_read_child_proof(&self,
                                      request:
                                          &RemoteReadChildRequest<Block::Header>,
                                      remote_proof: Vec<Vec<u8>>)
             -> ClientResult<Option<Vec<u8>>> {
                read_child_proof_check::<H>(convert_hash(request.header.state_root()),
                                            remote_proof,
                                            &request.storage_key,
                                            &request.key).map_err(Into::into)
            }
            fn check_execution_proof(&self,
                                     request:
                                         &RemoteCallRequest<Block::Header>,
                                     remote_proof: Vec<Vec<u8>>)
             -> ClientResult<Vec<u8>> {
                check_execution_proof::<_, _,
                                        H>(&self.executor, request,
                                           remote_proof)
            }
            fn check_changes_proof(&self,
                                   request:
                                       &RemoteChangesRequest<Block::Header>,
                                   remote_proof: ChangesProof<Block::Header>)
             -> ClientResult<Vec<(NumberFor<Block>, u32)>> {
                self.check_changes_proof_with_cht_size(request, remote_proof,
                                                       cht::SIZE)
            }
            fn check_body_proof(&self,
                                request: &RemoteBodyRequest<Block::Header>,
                                body: Vec<Block::Extrinsic>)
             -> ClientResult<Vec<Block::Extrinsic>> {
                let extrinsics_root =
                    HashFor::<Block>::ordered_trie_root(body.iter().map(Encode::encode));
                if *request.header.extrinsics_root() == extrinsics_root {
                    Ok(body)
                } else {
                    Err(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["RemoteBodyRequest: invalid extrinsics root expected: ",
                                                                             " but got "],
                                                                           &match (&*request.header.extrinsics_root(),
                                                                                   &extrinsics_root)
                                                                                {
                                                                                (arg0,
                                                                                 arg1)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                             ::std::fmt::Display::fmt)],
                                                                            })).into())
                }
            }
        }
        /// A view of BTreeMap<Number, Hash> as a changes trie roots storage.
        struct RootsStorage<'a, Number: As<u64>, Hash: 'a> {
            roots: (Number, &'a [Hash]),
            prev_roots: BTreeMap<Number, Hash>,
        }
        impl <'a, H, Number, Hash> ChangesTrieRootsStorage<H> for
         RootsStorage<'a, Number, Hash> where H: Hasher, Number: Send + Sync +
         Eq + ::std::cmp::Ord + Copy + As<u64>, Hash: 'a + Send + Sync +
         Clone + AsRef<[u8]> {
            fn root(&self, _anchor: &ChangesTrieAnchorBlockId<H::Out>,
                    block: u64) -> Result<Option<H::Out>, String> {
                let root =
                    if block < self.roots.0.as_() {
                        self.prev_roots.get(&As::sa(block)).cloned()
                    } else {
                        block.checked_sub(self.roots.0.as_()).and_then(|index|
                                                                           self.roots.1.get(index
                                                                                                as
                                                                                                usize)).cloned()
                    };
                Ok(root.map(|root|
                                {
                                    let mut hasher_root: H::Out =
                                        Default::default();
                                    hasher_root.as_mut().copy_from_slice(root.as_ref());
                                    hasher_root
                                }))
            }
        }
    }
    use std::sync::Arc;
    use executor::RuntimeInfo;
    use primitives::{H256, Blake2Hasher};
    use runtime_primitives::BuildStorage;
    use runtime_primitives::traits::Block as BlockT;
    use state_machine::CodeExecutor;
    use crate::call_executor::LocalCallExecutor;
    use crate::client::Client;
    use crate::error::Result as ClientResult;
    use crate::light::backend::Backend;
    use crate::light::blockchain::{Blockchain, Storage as BlockchainStorage};
    use crate::light::call_executor::{RemoteCallExecutor,
                                      RemoteOrLocalCallExecutor};
    use crate::light::fetcher::{Fetcher, LightDataChecker};
    /// Create an instance of light client blockchain backend.
    pub fn new_light_blockchain<B: BlockT, S: BlockchainStorage<B>,
                                F>(storage: S) -> Arc<Blockchain<S, F>> {
        Arc::new(Blockchain::new(storage))
    }
    /// Create an instance of light client backend.
    pub fn new_light_backend<B, S,
                             F>(blockchain: Arc<Blockchain<S, F>>,
                                fetcher: Arc<F>)
     -> Arc<Backend<S, F, Blake2Hasher>> where B: BlockT,
     S: BlockchainStorage<B>, F: Fetcher<B> {
        blockchain.set_fetcher(Arc::downgrade(&fetcher));
        Arc::new(Backend::new(blockchain))
    }
    /// Create an instance of light client.
    pub fn new_light<B, S, F, GS, RA,
                     E>(backend: Arc<Backend<S, F, Blake2Hasher>>,
                        fetcher: Arc<F>, genesis_storage: GS,
                        code_executor: E)
     ->
         ClientResult<Client<Backend<S, F, Blake2Hasher>,
                             RemoteOrLocalCallExecutor<B,
                                                       Backend<S, F,
                                                               Blake2Hasher>,
                                                       RemoteCallExecutor<Blockchain<S,
                                                                                     F>,
                                                                          F>,
                                                       LocalCallExecutor<Backend<S,
                                                                                 F,
                                                                                 Blake2Hasher>,
                                                                         E>>,
                             B, RA>> where B: BlockT<Hash = H256>,
     S: BlockchainStorage<B>, F: Fetcher<B>, GS: BuildStorage,
     E: CodeExecutor<Blake2Hasher> + RuntimeInfo {
        let remote_executor =
            RemoteCallExecutor::new(backend.blockchain().clone(), fetcher);
        let local_executor =
            LocalCallExecutor::new(backend.clone(), code_executor);
        let executor =
            RemoteOrLocalCallExecutor::new(backend.clone(), remote_executor,
                                           local_executor);
        Client::new(backend, executor, genesis_storage, Default::default())
    }
    /// Create an instance of fetch data checker.
    pub fn new_fetch_checker<E, B: BlockT, S: BlockchainStorage<B>,
                             F>(blockchain: Arc<Blockchain<S, F>>,
                                executor: E)
     -> LightDataChecker<E, Blake2Hasher, B, S, F> where
     E: CodeExecutor<Blake2Hasher> {
        LightDataChecker::new(blockchain, executor)
    }
}
#[cfg(feature = "std")]
pub mod leaves {
    //! Helper for managing the set of available leaves in the chain for DB implementations.
    use std::collections::BTreeMap;
    use std::cmp::Reverse;
    use kvdb::{KeyValueDB, DBTransaction};
    use runtime_primitives::traits::SimpleArithmetic;
    use parity_codec::{Encode, Decode};
    use crate::error;
    #[structural_match]
    struct LeafSetItem<H, N> {
        hash: H,
        number: Reverse<N>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <H: ::std::fmt::Debug, N: ::std::fmt::Debug> ::std::fmt::Debug for
     LeafSetItem<H, N> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LeafSetItem { hash: ref __self_0_0, number: ref __self_0_1 }
                => {
                    let mut debug_trait_builder =
                        f.debug_struct("LeafSetItem");
                    let _ =
                        debug_trait_builder.field("hash", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("number", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <H: ::std::clone::Clone, N: ::std::clone::Clone> ::std::clone::Clone
     for LeafSetItem<H, N> {
        #[inline]
        fn clone(&self) -> LeafSetItem<H, N> {
            match *self {
                LeafSetItem { hash: ref __self_0_0, number: ref __self_0_1 }
                =>
                LeafSetItem{hash: ::std::clone::Clone::clone(&(*__self_0_0)),
                            number:
                                ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <H: ::std::cmp::PartialEq, N: ::std::cmp::PartialEq>
     ::std::cmp::PartialEq for LeafSetItem<H, N> {
        #[inline]
        fn eq(&self, other: &LeafSetItem<H, N>) -> bool {
            match *other {
                LeafSetItem { hash: ref __self_1_0, number: ref __self_1_1 }
                =>
                match *self {
                    LeafSetItem { hash: ref __self_0_0, number: ref __self_0_1
                    } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LeafSetItem<H, N>) -> bool {
            match *other {
                LeafSetItem { hash: ref __self_1_0, number: ref __self_1_1 }
                =>
                match *self {
                    LeafSetItem { hash: ref __self_0_0, number: ref __self_0_1
                    } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <H: ::std::cmp::Eq, N: ::std::cmp::Eq> ::std::cmp::Eq for
     LeafSetItem<H, N> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::std::cmp::AssertParamIsEq<H>;
                let _: ::std::cmp::AssertParamIsEq<Reverse<N>>;
            }
        }
    }
    /// A displaced leaf after import.
    #[must_use = "Displaced items from the leaf set must be handled."]
    pub struct ImportDisplaced<H, N> {
        new_hash: H,
        displaced: LeafSetItem<H, N>,
    }
    /// Displaced leaves after finalization.
    #[must_use = "Displaced items from the leaf set must be handled."]
    pub struct FinalizationDisplaced<H, N> {
        leaves: BTreeMap<Reverse<N>, Vec<H>>,
    }
    impl <H, N: Ord> FinalizationDisplaced<H, N> {
        /// Merge with another. This should only be used for displaced items that
        /// are produced within one transaction of each other.
        pub fn merge(&mut self, mut other: Self) {
            self.leaves.append(&mut other.leaves);
        }
    }
    /// list of leaf hashes ordered by number (descending).
    /// stored in memory for fast access.
    /// this allows very fast checking and modification of active leaves.
    #[structural_match]
    pub struct LeafSet<H, N> {
        storage: BTreeMap<Reverse<N>, Vec<H>>,
        pending_added: Vec<LeafSetItem<H, N>>,
        pending_removed: Vec<H>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <H: ::std::fmt::Debug, N: ::std::fmt::Debug> ::std::fmt::Debug for
     LeafSet<H, N> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                LeafSet {
                storage: ref __self_0_0,
                pending_added: ref __self_0_1,
                pending_removed: ref __self_0_2 } => {
                    let mut debug_trait_builder = f.debug_struct("LeafSet");
                    let _ =
                        debug_trait_builder.field("storage", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("pending_added",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("pending_removed",
                                                  &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <H: ::std::clone::Clone, N: ::std::clone::Clone> ::std::clone::Clone
     for LeafSet<H, N> {
        #[inline]
        fn clone(&self) -> LeafSet<H, N> {
            match *self {
                LeafSet {
                storage: ref __self_0_0,
                pending_added: ref __self_0_1,
                pending_removed: ref __self_0_2 } =>
                LeafSet{storage: ::std::clone::Clone::clone(&(*__self_0_0)),
                        pending_added:
                            ::std::clone::Clone::clone(&(*__self_0_1)),
                        pending_removed:
                            ::std::clone::Clone::clone(&(*__self_0_2)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <H: ::std::cmp::PartialEq, N: ::std::cmp::PartialEq>
     ::std::cmp::PartialEq for LeafSet<H, N> {
        #[inline]
        fn eq(&self, other: &LeafSet<H, N>) -> bool {
            match *other {
                LeafSet {
                storage: ref __self_1_0,
                pending_added: ref __self_1_1,
                pending_removed: ref __self_1_2 } =>
                match *self {
                    LeafSet {
                    storage: ref __self_0_0,
                    pending_added: ref __self_0_1,
                    pending_removed: ref __self_0_2 } =>
                    (*__self_0_0) == (*__self_1_0) &&
                        (*__self_0_1) == (*__self_1_1) &&
                        (*__self_0_2) == (*__self_1_2),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LeafSet<H, N>) -> bool {
            match *other {
                LeafSet {
                storage: ref __self_1_0,
                pending_added: ref __self_1_1,
                pending_removed: ref __self_1_2 } =>
                match *self {
                    LeafSet {
                    storage: ref __self_0_0,
                    pending_added: ref __self_0_1,
                    pending_removed: ref __self_0_2 } =>
                    (*__self_0_0) != (*__self_1_0) ||
                        (*__self_0_1) != (*__self_1_1) ||
                        (*__self_0_2) != (*__self_1_2),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <H: ::std::cmp::Eq, N: ::std::cmp::Eq> ::std::cmp::Eq for
     LeafSet<H, N> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _:
                        ::std::cmp::AssertParamIsEq<BTreeMap<Reverse<N>,
                                                             Vec<H>>>;
                let _: ::std::cmp::AssertParamIsEq<Vec<LeafSetItem<H, N>>>;
                let _: ::std::cmp::AssertParamIsEq<Vec<H>>;
            }
        }
    }
    impl <H, N> LeafSet<H, N> where H: Clone + PartialEq + Decode + Encode,
     N: std::fmt::Debug + Clone + SimpleArithmetic + Decode + Encode {
        /// Construct a new, blank leaf set.
        pub fn new() -> Self {
            Self{storage: BTreeMap::new(),
                 pending_added: Vec::new(),
                 pending_removed: Vec::new(),}
        }
        /// Read the leaf list from the DB, using given prefix for keys.
        pub fn read_from_db(db: &KeyValueDB, column: Option<u32>,
                            prefix: &[u8]) -> error::Result<Self> {
            let mut storage = BTreeMap::new();
            for (key, value) in db.iter_from_prefix(column, prefix) {
                if !key.starts_with(prefix) { break  }
                let raw_hash = &mut &key[prefix.len()..];
                let hash =
                    match Decode::decode(raw_hash) {
                        Some(hash) => hash,
                        None =>
                        return Err(error::Error::Backend("Error decoding hash".into())),
                    };
                let number =
                    match Decode::decode(&mut &value[..]) {
                        Some(number) => number,
                        None =>
                        return Err(error::Error::Backend("Error decoding number".into())),
                    };
                storage.entry(Reverse(number)).or_insert_with(Vec::new).push(hash);
            }
            Ok(Self{storage,
                    pending_added: Vec::new(),
                    pending_removed: Vec::new(),})
        }
        /// update the leaf list on import. returns a displaced leaf if there was one.
        pub fn import(&mut self, hash: H, number: N, parent_hash: H)
         -> Option<ImportDisplaced<H, N>> {
            let displaced =
                if number != N::zero() {
                    let new_number = Reverse(number.clone() - N::one());
                    let was_displaced =
                        self.remove_leaf(&new_number, &parent_hash);
                    if was_displaced {
                        self.pending_removed.push(parent_hash.clone());
                        Some(ImportDisplaced{new_hash: hash.clone(),
                                             displaced:
                                                 LeafSetItem{hash:
                                                                 parent_hash,
                                                             number:
                                                                 new_number,},})
                    } else { None }
                } else { None };
            self.insert_leaf(Reverse(number.clone()), hash.clone());
            self.pending_added.push(LeafSetItem{hash,
                                                number: Reverse(number),});
            displaced
        }
        /// Note a block height finalized, displacing all leaves with number less than the finalized block's.
        ///
        /// Although it would be more technically correct to also prune out leaves at the
        /// same number as the finalized block, but with different hashes, the current behavior
        /// is simpler and our assumptions about how finalization works means that those leaves
        /// will be pruned soon afterwards anyway.
        pub fn finalize_height(&mut self, number: N)
         -> FinalizationDisplaced<H, N> {
            let boundary =
                if number == N::zero() {
                    return FinalizationDisplaced{leaves: BTreeMap::new(),};
                } else { number - N::one() };
            let below_boundary = self.storage.split_off(&Reverse(boundary));
            self.pending_removed.extend(below_boundary.values().flat_map(|h|
                                                                             h.iter()).cloned());
            FinalizationDisplaced{leaves: below_boundary,}
        }
        /// Undo all pending operations.
        ///
        /// This returns an `Undo` struct, where any
        /// `Displaced` objects that have returned by previous method calls
        /// should be passed to via the appropriate methods. Otherwise,
        /// the on-disk state may get out of sync with in-memory state.
        pub fn undo(&mut self) -> Undo<H, N> { Undo{inner: self,} }
        /// currently since revert only affects the canonical chain
        /// we assume that parent has no further children
        /// and we add it as leaf again
        pub fn revert(&mut self, hash: H, number: N, parent_hash: H) {
            self.insert_leaf(Reverse(number.clone() - N::one()), parent_hash);
            self.remove_leaf(&Reverse(number), &hash);
        }
        /// returns an iterator over all hashes in the leaf set
        /// ordered by their block number descending.
        pub fn hashes(&self) -> Vec<H> {
            self.storage.iter().flat_map(|(_, hashes)|
                                             hashes.iter()).cloned().collect()
        }
        /// Write the leaf list to the database transaction.
        pub fn prepare_transaction(&mut self, tx: &mut DBTransaction,
                                   column: Option<u32>, prefix: &[u8]) {
            let mut buf = prefix.to_vec();
            for LeafSetItem { hash, number } in self.pending_added.drain(..) {
                hash.using_encoded(|s| buf.extend(s));
                tx.put_vec(column, &buf[..], number.0.encode());
                buf.truncate(prefix.len());
            }
            for hash in self.pending_removed.drain(..) {
                hash.using_encoded(|s| buf.extend(s));
                tx.delete(column, &buf[..]);
                buf.truncate(prefix.len());
            }
        }
        fn insert_leaf(&mut self, number: Reverse<N>, hash: H) {
            self.storage.entry(number).or_insert_with(Vec::new).push(hash);
        }
        fn remove_leaf(&mut self, number: &Reverse<N>, hash: &H) -> bool {
            let mut empty = false;
            let removed =
                self.storage.get_mut(number).map_or(false,
                                                    |leaves|
                                                        {
                                                            let mut found =
                                                                false;
                                                            leaves.retain(|h|
                                                                              if h
                                                                                     ==
                                                                                     hash
                                                                                 {
                                                                                  found
                                                                                      =
                                                                                      true;
                                                                                  false
                                                                              } else {
                                                                                  true
                                                                              });
                                                            if leaves.is_empty()
                                                               {
                                                                empty = true
                                                            }
                                                            found
                                                        });
            if removed && empty { self.storage.remove(number); }
            removed
        }
    }
    /// Helper for undoing operations.
    pub struct Undo<'a, H: 'a, N: 'a> {
        inner: &'a mut LeafSet<H, N>,
    }
    impl <'a, H: 'a, N: 'a> Undo<'a, H, N> where H: Clone + PartialEq +
     Decode + Encode, N: std::fmt::Debug + Clone + SimpleArithmetic + Decode +
     Encode {
        /// Undo an imported block by providing the displaced leaf.
        pub fn undo_import(&mut self, displaced: ImportDisplaced<H, N>) {
            let new_number =
                Reverse(displaced.displaced.number.0.clone() + N::one());
            self.inner.remove_leaf(&new_number, &displaced.new_hash);
            self.inner.insert_leaf(new_number, displaced.displaced.hash);
        }
        /// Undo a finalization operation by providing the displaced leaves.
        pub fn undo_finalization(&mut self,
                                 mut displaced: FinalizationDisplaced<H, N>) {
            self.inner.storage.append(&mut displaced.leaves);
        }
    }
    impl <'a, H: 'a, N: 'a> Drop for Undo<'a, H, N> {
        fn drop(&mut self) {
            self.inner.pending_added.clear();
            self.inner.pending_removed.clear();
        }
    }
}
#[cfg(feature = "std")]
pub mod children {
    //! Functionality for reading and storing children hashes from db.
    use kvdb::{KeyValueDB, DBTransaction};
    use parity_codec::{Encode, Decode};
    use crate::error;
    use std::hash::Hash;
    /// Returns the hashes of the children blocks of the block with `parent_hash`.
    pub fn read_children<K: Eq + Hash + Clone + Encode + Decode, V: Eq +
                         Hash + Clone + Encode +
                         Decode>(db: &KeyValueDB, column: Option<u32>,
                                 prefix: &[u8], parent_hash: K)
     -> error::Result<Vec<V>> {
        let mut buf = prefix.to_vec();
        parent_hash.using_encoded(|s| buf.extend(s));
        let raw_val_opt =
            match db.get(column, &buf[..]) {
                Ok(raw_val_opt) => raw_val_opt,
                Err(_) =>
                return Err(error::Error::Backend("Error reading value from database".into())),
            };
        let raw_val =
            match raw_val_opt {
                Some(val) => val,
                None => return Ok(Vec::new()),
            };
        let children: Vec<V> =
            match Decode::decode(&mut &raw_val[..]) {
                Some(children) => children,
                None =>
                return Err(error::Error::Backend("Error decoding children".into())),
            };
        Ok(children)
    }
    /// Insert the key-value pair (`parent_hash`, `children_hashes`) in the transaction.
    /// Any existing value is overwritten upon write.
    pub fn write_children<K: Eq + Hash + Clone + Encode + Decode, V: Eq +
                          Hash + Clone + Encode +
                          Decode>(tx: &mut DBTransaction, column: Option<u32>,
                                  prefix: &[u8], parent_hash: K,
                                  children_hashes: V) {
        let mut key = prefix.to_vec();
        parent_hash.using_encoded(|s| key.extend(s));
        tx.put_vec(column, &key[..], children_hashes.encode());
    }
    /// Prepare transaction to remove the children of `parent_hash`.
    pub fn remove_children<K: Eq + Hash + Clone + Encode +
                           Decode>(tx: &mut DBTransaction,
                                   column: Option<u32>, prefix: &[u8],
                                   parent_hash: K) {
        let mut key = prefix.to_vec();
        parent_hash.using_encoded(|s| key.extend(s));
        tx.delete(column, &key[..]);
    }
}
#[cfg(feature = "std")]
mod call_executor {
    use std::{sync::Arc, cmp::Ord, panic::UnwindSafe, result, cell::RefCell,
              rc::Rc};
    use parity_codec::{Encode, Decode};
    use runtime_primitives::{generic::BlockId, traits::Block as BlockT};
    use state_machine::{self, OverlayedChanges, Ext, CodeExecutor,
                        ExecutionManager, ExecutionStrategy, NeverOffchainExt,
                        backend::Backend as _};
    use executor::{RuntimeVersion, RuntimeInfo, NativeVersion};
    use hash_db::Hasher;
    use trie::MemoryDB;
    use primitives::{H256, Blake2Hasher, NativeOrEncoded, NeverNativeValue,
                     OffchainExt};
    use crate::runtime_api::{ProofRecorder, InitializeBlock};
    use crate::backend;
    use crate::error;
    /// Method call executor.
    pub trait CallExecutor<B, H> where B: BlockT, H: Hasher<Out = B::Hash>,
     H::Out: Ord {
        /// Externalities error type.
        type
        Error: state_machine::Error;
        /// Execute a call to a contract on top of state in a block of given hash.
        ///
        /// No changes are made.
        fn call<O: OffchainExt>(&self, id: &BlockId<B>, method: &str,
                                call_data: &[u8], strategy: ExecutionStrategy,
                                side_effects_handler: Option<&mut O>)
        -> Result<Vec<u8>, error::Error>;
        /// Execute a contextual call on top of state in a block of a given hash.
        ///
        /// No changes are made.
        /// Before executing the method, passed header is installed as the current header
        /// of the execution context.
        fn contextual_call<'a, O: OffchainExt, IB: Fn() -> error::Result<()>,
                           EM: Fn(Result<NativeOrEncoded<R>, Self::Error>,
                                  Result<NativeOrEncoded<R>, Self::Error>) ->
                           Result<NativeOrEncoded<R>, Self::Error>,
                           R: Encode + Decode + PartialEq, NC: FnOnce() ->
                           result::Result<R, &'static str> +
                           UnwindSafe>(&self, initialize_block_fn: IB,
                                       at: &BlockId<B>, method: &str,
                                       call_data: &[u8],
                                       changes: &RefCell<OverlayedChanges>,
                                       initialize_block:
                                           InitializeBlock<'a, B>,
                                       execution_manager:
                                           ExecutionManager<EM>,
                                       native_call: Option<NC>,
                                       side_effects_handler: Option<&mut O>,
                                       proof_recorder:
                                           &Option<Rc<RefCell<ProofRecorder<B>>>>)
        -> error::Result<NativeOrEncoded<R>>
        where
        ExecutionManager<EM>: Clone;
        /// Extract RuntimeVersion of given block
        ///
        /// No changes are made.
        fn runtime_version(&self, id: &BlockId<B>)
        -> Result<RuntimeVersion, error::Error>;
        /// Execute a call to a contract on top of given state.
        ///
        /// No changes are made.
        fn call_at_state<O: OffchainExt, S: state_machine::Backend<H>,
                         F: FnOnce(Result<NativeOrEncoded<R>, Self::Error>,
                                   Result<NativeOrEncoded<R>, Self::Error>) ->
                         Result<NativeOrEncoded<R>, Self::Error>, R: Encode +
                         Decode + PartialEq, NC: FnOnce() ->
                         result::Result<R, &'static str> +
                         UnwindSafe>(&self, state: &S,
                                     overlay: &mut OverlayedChanges,
                                     method: &str, call_data: &[u8],
                                     manager: ExecutionManager<F>,
                                     native_call: Option<NC>,
                                     side_effects_handler: Option<&mut O>)
        ->
            Result<(NativeOrEncoded<R>, S::Transaction, Option<MemoryDB<H>>),
                   error::Error>;
        /// Execute a call to a contract on top of given state, gathering execution proof.
        ///
        /// No changes are made.
        fn prove_at_state<S: state_machine::Backend<H>>(&self, state: S,
                                                        overlay:
                                                            &mut OverlayedChanges,
                                                        method: &str,
                                                        call_data: &[u8])
         -> Result<(Vec<u8>, Vec<Vec<u8>>), error::Error> {
            let trie_state =
                state.try_into_trie_backend().ok_or_else(||
                                                             Box::new(state_machine::ExecutionError::UnableToGenerateProof)
                                                                 as
                                                                 Box<state_machine::Error>)?;
            self.prove_at_trie_state(&trie_state, overlay, method, call_data)
        }
        /// Execute a call to a contract on top of given trie state, gathering execution proof.
        ///
        /// No changes are made.
        fn prove_at_trie_state<S: state_machine::TrieBackendStorage<H>>(&self,
                                                                        trie_state:
                                                                            &state_machine::TrieBackend<S,
                                                                                                        H>,
                                                                        overlay:
                                                                            &mut OverlayedChanges,
                                                                        method:
                                                                            &str,
                                                                        call_data:
                                                                            &[u8])
        -> Result<(Vec<u8>, Vec<Vec<u8>>), error::Error>;
        /// Get runtime version if supported.
        fn native_runtime_version(&self)
        -> Option<&NativeVersion>;
    }
    /// Call executor that executes methods locally, querying all required
    /// data from local backend.
    pub struct LocalCallExecutor<B, E> {
        backend: Arc<B>,
        executor: E,
    }
    impl <B, E> LocalCallExecutor<B, E> {
        /// Creates new instance of local call executor.
        pub fn new(backend: Arc<B>, executor: E) -> Self {
            LocalCallExecutor{backend, executor,}
        }
    }
    impl <B, E> Clone for LocalCallExecutor<B, E> where E: Clone {
        fn clone(&self) -> Self {
            LocalCallExecutor{backend: self.backend.clone(),
                              executor: self.executor.clone(),}
        }
    }
    impl <B, E, Block> CallExecutor<Block, Blake2Hasher> for
     LocalCallExecutor<B, E> where B: backend::Backend<Block, Blake2Hasher>,
     E: CodeExecutor<Blake2Hasher> + RuntimeInfo, Block: BlockT<Hash = H256> {
        type
        Error
        =
        E::Error;
        fn call<O: OffchainExt>(&self, id: &BlockId<Block>, method: &str,
                                call_data: &[u8], strategy: ExecutionStrategy,
                                side_effects_handler: Option<&mut O>)
         -> error::Result<Vec<u8>> {
            let mut changes = OverlayedChanges::default();
            let state = self.backend.state_at(*id)?;
            let return_data =
                state_machine::new(&state,
                                   self.backend.changes_trie_storage(),
                                   side_effects_handler, &mut changes,
                                   &self.executor, method,
                                   call_data).execute_using_consensus_failure_handler::<_,
                                                                                        NeverNativeValue,
                                                                                        fn()
                                                                                            ->
                                                                                                _>(strategy.get_manager(),
                                                                                                   false,
                                                                                                   None).map(|(result,
                                                                                                               _,
                                                                                                               _)|
                                                                                                                 result)?;
            self.backend.destroy_state(state)?;
            Ok(return_data.into_encoded())
        }
        fn contextual_call<'a, O: OffchainExt, IB: Fn() -> error::Result<()>,
                           EM: Fn(Result<NativeOrEncoded<R>, Self::Error>,
                                  Result<NativeOrEncoded<R>, Self::Error>) ->
                           Result<NativeOrEncoded<R>, Self::Error>,
                           R: Encode + Decode + PartialEq, NC: FnOnce() ->
                           result::Result<R, &'static str> +
                           UnwindSafe>(&self, initialize_block_fn: IB,
                                       at: &BlockId<Block>, method: &str,
                                       call_data: &[u8],
                                       changes: &RefCell<OverlayedChanges>,
                                       initialize_block:
                                           InitializeBlock<'a, Block>,
                                       execution_manager:
                                           ExecutionManager<EM>,
                                       native_call: Option<NC>,
                                       side_effects_handler: Option<&mut O>,
                                       recorder:
                                           &Option<Rc<RefCell<ProofRecorder<Block>>>>)
         -> Result<NativeOrEncoded<R>, error::Error> where
         ExecutionManager<EM>: Clone {
            match initialize_block {
                InitializeBlock::Do(ref init_block) if
                init_block.borrow().as_ref().map(|id|
                                                     id != at).unwrap_or(true)
                => {
                    initialize_block_fn()?;
                }
                _ => { }
            }
            let state = self.backend.state_at(*at)?;
            match recorder {
                Some(recorder) => {
                    let trie_state =
                        state.try_into_trie_backend().ok_or_else(||
                                                                     Box::new(state_machine::ExecutionError::UnableToGenerateProof)
                                                                         as
                                                                         Box<state_machine::Error>)?;
                    let backend =
                        state_machine::ProvingBackend::new_with_recorder(&trie_state,
                                                                         recorder.clone());
                    state_machine::new(&backend,
                                       self.backend.changes_trie_storage(),
                                       side_effects_handler,
                                       &mut *changes.borrow_mut(),
                                       &self.executor, method,
                                       call_data).execute_using_consensus_failure_handler(execution_manager,
                                                                                          false,
                                                                                          native_call).map(|(result,
                                                                                                             _,
                                                                                                             _)|
                                                                                                               result).map_err(Into::into)
                }
                None =>
                state_machine::new(&state,
                                   self.backend.changes_trie_storage(),
                                   side_effects_handler,
                                   &mut *changes.borrow_mut(), &self.executor,
                                   method,
                                   call_data).execute_using_consensus_failure_handler(execution_manager,
                                                                                      false,
                                                                                      native_call).map(|(result,
                                                                                                         _,
                                                                                                         _)|
                                                                                                           result).map_err(Into::into),
            }
        }
        fn runtime_version(&self, id: &BlockId<Block>)
         -> error::Result<RuntimeVersion> {
            let mut overlay = OverlayedChanges::default();
            let state = self.backend.state_at(*id)?;
            let mut ext =
                Ext::new(&mut overlay, &state,
                         self.backend.changes_trie_storage(),
                         NeverOffchainExt::new());
            self.executor.runtime_version(&mut ext).ok_or(error::Error::VersionInvalid.into())
        }
        fn call_at_state<O: OffchainExt,
                         S: state_machine::Backend<Blake2Hasher>,
                         F: FnOnce(Result<NativeOrEncoded<R>, Self::Error>,
                                   Result<NativeOrEncoded<R>, Self::Error>) ->
                         Result<NativeOrEncoded<R>, Self::Error>, R: Encode +
                         Decode + PartialEq, NC: FnOnce() ->
                         result::Result<R, &'static str> +
                         UnwindSafe>(&self, state: &S,
                                     changes: &mut OverlayedChanges,
                                     method: &str, call_data: &[u8],
                                     manager: ExecutionManager<F>,
                                     native_call: Option<NC>,
                                     side_effects_handler: Option<&mut O>)
         ->
             error::Result<(NativeOrEncoded<R>, S::Transaction,
                            Option<MemoryDB<Blake2Hasher>>)> {
            state_machine::new(state, self.backend.changes_trie_storage(),
                               side_effects_handler, changes, &self.executor,
                               method,
                               call_data).execute_using_consensus_failure_handler(manager,
                                                                                  true,
                                                                                  native_call).map(|(result,
                                                                                                     storage_tx,
                                                                                                     changes_tx)|
                                                                                                       (result,
                                                                                                        storage_tx.expect("storage_tx is always computed when compute_tx is true; qed"),
                                                                                                        changes_tx)).map_err(Into::into)
        }
        fn prove_at_trie_state<S: state_machine::TrieBackendStorage<Blake2Hasher>>(&self,
                                                                                   trie_state:
                                                                                       &state_machine::TrieBackend<S,
                                                                                                                   Blake2Hasher>,
                                                                                   overlay:
                                                                                       &mut OverlayedChanges,
                                                                                   method:
                                                                                       &str,
                                                                                   call_data:
                                                                                       &[u8])
         -> Result<(Vec<u8>, Vec<Vec<u8>>), error::Error> {
            state_machine::prove_execution_on_trie_backend(trie_state,
                                                           overlay,
                                                           &self.executor,
                                                           method,
                                                           call_data).map_err(Into::into)
        }
        fn native_runtime_version(&self) -> Option<&NativeVersion> {
            Some(self.executor.native_version())
        }
    }
}
#[cfg(feature = "std")]
mod client {
    //! Substrate Client
    use std::{marker::PhantomData, collections::{HashSet, BTreeMap, HashMap},
              sync::Arc, panic::UnwindSafe, result, cell::RefCell, rc::Rc};
    use crate::error::Error;
    use futures::sync::mpsc;
    use parking_lot::{Mutex, RwLock};
    use primitives::NativeOrEncoded;
    use runtime_primitives::{Justification, generic::{BlockId, SignedBlock}};
    use consensus::{Error as ConsensusError, ErrorKind as ConsensusErrorKind,
                    ImportBlock, ImportResult, BlockOrigin,
                    ForkChoiceStrategy, well_known_cache_keys::Id as
                    CacheKeyId, SelectChain, self};
    use runtime_primitives::traits::{Block as BlockT, Header as HeaderT, Zero,
                                     As, NumberFor, CurrentHeight,
                                     BlockNumberToHash, ApiRef,
                                     ProvideRuntimeApi, Digest, DigestItem};
    use runtime_primitives::BuildStorage;
    use crate::runtime_api::{CallRuntimeAt, ConstructRuntimeApi, Core as
                             CoreApi, ProofRecorder, InitializeBlock};
    use primitives::{Blake2Hasher, H256, ChangesTrieConfiguration,
                     convert_hash, NeverNativeValue, ExecutionContext};
    use primitives::storage::{StorageKey, StorageData};
    use primitives::storage::well_known_keys;
    use parity_codec::{Encode, Decode};
    use state_machine::{DBValue, Backend as StateBackend, CodeExecutor,
                        ChangesTrieAnchorBlockId, ExecutionStrategy,
                        ExecutionManager, prove_read, prove_child_read,
                        ChangesTrieRootsStorage, ChangesTrieStorage,
                        key_changes, key_changes_proof, OverlayedChanges,
                        NeverOffchainExt};
    use hash_db::Hasher;
    use crate::backend::{self, BlockImportOperation,
                         PrunableStateChangesTrieStorage};
    use crate::blockchain::{self, Info as ChainInfo, Backend as ChainBackend,
                            HeaderBackend as ChainHeaderBackend, ProvideCache,
                            Cache};
    use crate::call_executor::{CallExecutor, LocalCallExecutor};
    use executor::{RuntimeVersion, RuntimeInfo};
    use crate::notifications::{StorageNotifications, StorageEventStream};
    use crate::light::{call_executor::prove_execution, fetcher::ChangesProof};
    use crate::cht;
    use crate::error;
    use crate::in_mem;
    use crate::block_builder::{self, api::BlockBuilder as BlockBuilderAPI};
    use crate::genesis;
    use substrate_telemetry::{telemetry, SUBSTRATE_INFO};
    use log::{info, trace, warn};
    /// Type that implements `futures::Stream` of block import events.
    pub type ImportNotifications<Block>
        =
        mpsc::UnboundedReceiver<BlockImportNotification<Block>>;
    /// A stream of block finality notifications.
    pub type FinalityNotifications<Block>
        =
        mpsc::UnboundedReceiver<FinalityNotification<Block>>;
    type StorageUpdate<B, Block>
        =
        <<<B as backend::Backend<Block, Blake2Hasher>>::BlockImportOperation
         as BlockImportOperation<Block, Blake2Hasher>>::State as
        state_machine::Backend<Blake2Hasher>>::Transaction;
    type ChangesUpdate = trie::MemoryDB<Blake2Hasher>;
    /// Execution strategies settings.
    pub struct ExecutionStrategies {
        /// Execution strategy used when syncing.
        pub syncing: ExecutionStrategy,
        /// Execution strategy used when importing blocks.
        pub importing: ExecutionStrategy,
        /// Execution strategy used when constructing blocks.
        pub block_construction: ExecutionStrategy,
        /// Execution strategy used for offchain workers.
        pub offchain_worker: ExecutionStrategy,
        /// Execution strategy used in other cases.
        pub other: ExecutionStrategy,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for ExecutionStrategies {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ExecutionStrategies {
                syncing: ref __self_0_0,
                importing: ref __self_0_1,
                block_construction: ref __self_0_2,
                offchain_worker: ref __self_0_3,
                other: ref __self_0_4 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("ExecutionStrategies");
                    let _ =
                        debug_trait_builder.field("syncing", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("importing",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("block_construction",
                                                  &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("offchain_worker",
                                                  &&(*__self_0_3));
                    let _ =
                        debug_trait_builder.field("other", &&(*__self_0_4));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for ExecutionStrategies {
        #[inline]
        fn clone(&self) -> ExecutionStrategies {
            match *self {
                ExecutionStrategies {
                syncing: ref __self_0_0,
                importing: ref __self_0_1,
                block_construction: ref __self_0_2,
                offchain_worker: ref __self_0_3,
                other: ref __self_0_4 } =>
                ExecutionStrategies{syncing:
                                        ::std::clone::Clone::clone(&(*__self_0_0)),
                                    importing:
                                        ::std::clone::Clone::clone(&(*__self_0_1)),
                                    block_construction:
                                        ::std::clone::Clone::clone(&(*__self_0_2)),
                                    offchain_worker:
                                        ::std::clone::Clone::clone(&(*__self_0_3)),
                                    other:
                                        ::std::clone::Clone::clone(&(*__self_0_4)),},
            }
        }
    }
    impl Default for ExecutionStrategies {
        fn default() -> ExecutionStrategies {
            ExecutionStrategies{syncing: ExecutionStrategy::NativeElseWasm,
                                importing: ExecutionStrategy::NativeElseWasm,
                                block_construction:
                                    ExecutionStrategy::AlwaysWasm,
                                offchain_worker:
                                    ExecutionStrategy::NativeWhenPossible,
                                other: ExecutionStrategy::NativeElseWasm,}
        }
    }
    /// Substrate Client
    pub struct Client<B, E, Block, RA> where Block: BlockT {
        backend: Arc<B>,
        executor: E,
        storage_notifications: Mutex<StorageNotifications<Block>>,
        import_notification_sinks: Mutex<Vec<mpsc::UnboundedSender<BlockImportNotification<Block>>>>,
        finality_notification_sinks: Mutex<Vec<mpsc::UnboundedSender<FinalityNotification<Block>>>>,
        import_lock: Arc<Mutex<()>>,
        importing_block: RwLock<Option<Block::Hash>>,
        execution_strategies: ExecutionStrategies,
        _phantom: PhantomData<RA>,
    }
    /// Client import operation, a wrapper for the backend.
    pub struct ClientImportOperation<Block: BlockT, H: Hasher<Out =
                                     Block::Hash>,
                                     B: backend::Backend<Block, H>> {
        op: B::BlockImportOperation,
        notify_imported: Option<(Block::Hash, BlockOrigin, Block::Header,
                                 bool,
                                 Option<Vec<(Vec<u8>, Option<Vec<u8>>)>>)>,
        notify_finalized: Vec<Block::Hash>,
    }
    /// A source of blockchain events.
    pub trait BlockchainEvents<Block: BlockT> {
        /// Get block import event stream. Not guaranteed to be fired for every
        /// imported block.
        fn import_notification_stream(&self)
        -> ImportNotifications<Block>;
        /// Get a stream of finality notifications. Not guaranteed to be fired for every
        /// finalized block.
        fn finality_notification_stream(&self)
        -> FinalityNotifications<Block>;
        /// Get storage changes event stream.
        ///
        /// Passing `None` as `filter_keys` subscribes to all storage changes.
        fn storage_changes_notification_stream(&self,
                                               filter_keys:
                                                   Option<&[StorageKey]>)
        -> error::Result<StorageEventStream<Block::Hash>>;
    }
    /// Fetch block body by ID.
    pub trait BlockBody<Block: BlockT> {
        /// Get block body by ID. Returns `None` if the body is not stored.
        fn block_body(&self, id: &BlockId<Block>)
        -> error::Result<Option<Vec<<Block as BlockT>::Extrinsic>>>;
    }
    /// Client info
    pub struct ClientInfo<Block: BlockT> {
        /// Best block hash.
        pub chain: ChainInfo<Block>,
        /// Best block number in the queue.
        pub best_queued_number: Option<<<Block as BlockT>::Header as
                                       HeaderT>::Number>,
        /// Best queued block hash.
        pub best_queued_hash: Option<Block::Hash>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::fmt::Debug + BlockT> ::std::fmt::Debug for
     ClientInfo<Block> where Block::Hash: ::std::fmt::Debug {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                ClientInfo {
                chain: ref __self_0_0,
                best_queued_number: ref __self_0_1,
                best_queued_hash: ref __self_0_2 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("ClientInfo");
                    let _ =
                        debug_trait_builder.field("chain", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("best_queued_number",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("best_queued_hash",
                                                  &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Block status.
    #[structural_match]
    pub enum BlockStatus {

        /// Added to the import queue.
        Queued,

        /// Already in the blockchain and the state is available.
        InChainWithState,

        /// In the blockchain, but the state is not available.
        InChainPruned,

        /// Block or parent is known to be bad.
        KnownBad,

        /// Not in the queue or the blockchain.
        Unknown,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for BlockStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match (&*self,) {
                (&BlockStatus::Queued,) => {
                    let mut debug_trait_builder = f.debug_tuple("Queued");
                    debug_trait_builder.finish()
                }
                (&BlockStatus::InChainWithState,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InChainWithState");
                    debug_trait_builder.finish()
                }
                (&BlockStatus::InChainPruned,) => {
                    let mut debug_trait_builder =
                        f.debug_tuple("InChainPruned");
                    debug_trait_builder.finish()
                }
                (&BlockStatus::KnownBad,) => {
                    let mut debug_trait_builder = f.debug_tuple("KnownBad");
                    debug_trait_builder.finish()
                }
                (&BlockStatus::Unknown,) => {
                    let mut debug_trait_builder = f.debug_tuple("Unknown");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::cmp::PartialEq for BlockStatus {
        #[inline]
        fn eq(&self, other: &BlockStatus) -> bool {
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
    impl ::std::cmp::Eq for BlockStatus {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () { { } }
    }
    /// Summary of an imported block
    pub struct BlockImportNotification<Block: BlockT> {
        /// Imported block header hash.
        pub hash: Block::Hash,
        /// Imported block origin.
        pub origin: BlockOrigin,
        /// Imported block header.
        pub header: Block::Header,
        /// Is this the new best block.
        pub is_new_best: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::clone::Clone + BlockT> ::std::clone::Clone for
     BlockImportNotification<Block> where Block::Hash: ::std::clone::Clone,
     Block::Header: ::std::clone::Clone {
        #[inline]
        fn clone(&self) -> BlockImportNotification<Block> {
            match *self {
                BlockImportNotification {
                hash: ref __self_0_0,
                origin: ref __self_0_1,
                header: ref __self_0_2,
                is_new_best: ref __self_0_3 } =>
                BlockImportNotification{hash:
                                            ::std::clone::Clone::clone(&(*__self_0_0)),
                                        origin:
                                            ::std::clone::Clone::clone(&(*__self_0_1)),
                                        header:
                                            ::std::clone::Clone::clone(&(*__self_0_2)),
                                        is_new_best:
                                            ::std::clone::Clone::clone(&(*__self_0_3)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::fmt::Debug + BlockT> ::std::fmt::Debug for
     BlockImportNotification<Block> where Block::Hash: ::std::fmt::Debug,
     Block::Header: ::std::fmt::Debug {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                BlockImportNotification {
                hash: ref __self_0_0,
                origin: ref __self_0_1,
                header: ref __self_0_2,
                is_new_best: ref __self_0_3 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("BlockImportNotification");
                    let _ =
                        debug_trait_builder.field("hash", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("origin", &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("header", &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("is_new_best",
                                                  &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    /// Summary of a finalized block.
    pub struct FinalityNotification<Block: BlockT> {
        /// Imported block header hash.
        pub hash: Block::Hash,
        /// Imported block header.
        pub header: Block::Header,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::clone::Clone + BlockT> ::std::clone::Clone for
     FinalityNotification<Block> where Block::Hash: ::std::clone::Clone,
     Block::Header: ::std::clone::Clone {
        #[inline]
        fn clone(&self) -> FinalityNotification<Block> {
            match *self {
                FinalityNotification {
                hash: ref __self_0_0, header: ref __self_0_1 } =>
                FinalityNotification{hash:
                                         ::std::clone::Clone::clone(&(*__self_0_0)),
                                     header:
                                         ::std::clone::Clone::clone(&(*__self_0_1)),},
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::fmt::Debug + BlockT> ::std::fmt::Debug for
     FinalityNotification<Block> where Block::Hash: ::std::fmt::Debug,
     Block::Header: ::std::fmt::Debug {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                FinalityNotification {
                hash: ref __self_0_0, header: ref __self_0_1 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("FinalityNotification");
                    let _ =
                        debug_trait_builder.field("hash", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("header", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    enum PrePostHeader<H> { Same(H), Different(H, H), }
    impl <H> PrePostHeader<H> {
        fn pre(&self) -> &H {
            match *self {
                PrePostHeader::Same(ref h) => h,
                PrePostHeader::Different(ref h, _) => h,
            }
        }
        fn post(&self) -> &H {
            match *self {
                PrePostHeader::Same(ref h) => h,
                PrePostHeader::Different(_, ref h) => h,
            }
        }
        fn into_post(self) -> H {
            match self {
                PrePostHeader::Same(h) => h,
                PrePostHeader::Different(_, h) => h,
            }
        }
    }
    /// Create an instance of in-memory client.
    pub fn new_in_mem<E, Block, S, RA>(executor: E, genesis_storage: S)
     ->
         error::Result<Client<in_mem::Backend<Block, Blake2Hasher>,
                              LocalCallExecutor<in_mem::Backend<Block,
                                                                Blake2Hasher>,
                                                E>, Block, RA>> where
     E: CodeExecutor<Blake2Hasher> + RuntimeInfo, S: BuildStorage,
     Block: BlockT<Hash = H256> {
        new_with_backend(Arc::new(in_mem::Backend::new()), executor,
                         genesis_storage)
    }
    /// Create a client with the explicitly provided backend.
    /// This is useful for testing backend implementations.
    pub fn new_with_backend<B, E, Block, S,
                            RA>(backend: Arc<B>, executor: E,
                                build_genesis_storage: S)
     -> error::Result<Client<B, LocalCallExecutor<B, E>, Block, RA>> where
     E: CodeExecutor<Blake2Hasher> + RuntimeInfo, S: BuildStorage,
     Block: BlockT<Hash = H256>,
     B: backend::LocalBackend<Block, Blake2Hasher> {
        let call_executor = LocalCallExecutor::new(backend.clone(), executor);
        Client::new(backend, call_executor, build_genesis_storage,
                    Default::default())
    }
    impl <B, E, Block, RA> Client<B, E, Block, RA> where
     B: backend::Backend<Block, Blake2Hasher>,
     E: CallExecutor<Block, Blake2Hasher>, Block: BlockT<Hash = H256> {
        /// Creates new Substrate Client with given blockchain and code executor.
        pub fn new<S: BuildStorage>(backend: Arc<B>, executor: E,
                                    build_genesis_storage: S,
                                    execution_strategies: ExecutionStrategies)
         -> error::Result<Self> {
            if backend.blockchain().header(BlockId::Number(Zero::zero()))?.is_none()
               {
                let (genesis_storage, children_genesis_storage) =
                    build_genesis_storage.build_storage()?;
                let mut op = backend.begin_operation()?;
                backend.begin_state_operation(&mut op,
                                              BlockId::Hash(Default::default()))?;
                let state_root =
                    op.reset_storage(genesis_storage,
                                     children_genesis_storage)?;
                let genesis_block =
                    genesis::construct_genesis_block::<Block>(state_root.into());
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Initializing Genesis block/state (state: ",
                                                                                 ", header-hash: ",
                                                                                 ")"],
                                                                               &match (&genesis_block.header().state_root(),
                                                                                       &genesis_block.header().hash())
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
                                                 &("substrate_client::client",
                                                   "substrate_client::client",
                                                   "core/client/src/client.rs",
                                                   289u32));
                    }
                };
                op.set_block_data(genesis_block.deconstruct().0,
                                  Some(<[_]>::into_vec(box [])), None,
                                  crate::backend::NewBlockState::Final)?;
                backend.commit_operation(op)?;
            }
            Ok(Client{backend,
                      executor,
                      storage_notifications: Default::default(),
                      import_notification_sinks: Default::default(),
                      finality_notification_sinks: Default::default(),
                      import_lock: Default::default(),
                      importing_block: Default::default(),
                      execution_strategies,
                      _phantom: Default::default(),})
        }
        /// Get a reference to the execution strategies.
        pub fn execution_strategies(&self) -> &ExecutionStrategies {
            &self.execution_strategies
        }
        /// Get a reference to the state at a given block.
        pub fn state_at(&self, block: &BlockId<Block>)
         -> error::Result<B::State> {
            self.backend.state_at(*block)
        }
        /// Expose backend reference. To be used in tests only
        #[doc(hidden)]
        #[deprecated(note =
                         "Rather than relying on `client` to provide this, access \
	to the backend should be handled at setup only - see #1134. This function \
	will be removed once that is in place.")]
        pub fn backend(&self) -> &Arc<B> { &self.backend }
        /// Expose reference to import lock
        #[doc(hidden)]
        #[deprecated(note =
                         "Rather than relying on `client` to provide this, access \
	to the backend should be handled at setup only - see #1134. This function \
	will be removed once that is in place.")]
        pub fn import_lock(&self) -> Arc<Mutex<()>> {
            self.import_lock.clone()
        }
        /// Given a `BlockId` and a key prefix, return the matching child storage keys in that block.
        pub fn storage_keys(&self, id: &BlockId<Block>,
                            key_prefix: &StorageKey)
         -> error::Result<Vec<StorageKey>> {
            let keys =
                self.state_at(id)?.keys(&key_prefix.0).into_iter().map(StorageKey).collect();
            Ok(keys)
        }
        /// Given a `BlockId` and a key, return the value under the key in that block.
        pub fn storage(&self, id: &BlockId<Block>, key: &StorageKey)
         -> error::Result<Option<StorageData>> {
            Ok(self.state_at(id)?.storage(&key.0).map_err(|e|
                                                              error::Error::from_state(Box::new(e)))?.map(StorageData))
        }
        /// Given a `BlockId`, a key prefix, and a child storage key, return the matching child storage keys.
        pub fn child_storage_keys(&self, id: &BlockId<Block>,
                                  child_storage_key: &StorageKey,
                                  key_prefix: &StorageKey)
         -> error::Result<Vec<StorageKey>> {
            let keys =
                self.state_at(id)?.child_keys(&child_storage_key.0,
                                              &key_prefix.0).into_iter().map(StorageKey).collect();
            Ok(keys)
        }
        /// Given a `BlockId`, a key and a child storage key, return the value under the key in that block.
        pub fn child_storage(&self, id: &BlockId<Block>,
                             child_storage_key: &StorageKey, key: &StorageKey)
         -> error::Result<Option<StorageData>> {
            Ok(self.state_at(id)?.child_storage(&child_storage_key.0,
                                                &key.0).map_err(|e|
                                                                    error::Error::from_state(Box::new(e)))?.map(StorageData))
        }
        /// Get the code at a given block.
        pub fn code_at(&self, id: &BlockId<Block>) -> error::Result<Vec<u8>> {
            Ok(self.storage(id,
                            &StorageKey(well_known_keys::CODE.to_vec()))?.expect("None is returned if there's no value stored for the given key; ':code' key is always defined; qed").0)
        }
        /// Get the RuntimeVersion at a given block.
        pub fn runtime_version_at(&self, id: &BlockId<Block>)
         -> error::Result<RuntimeVersion> {
            self.executor.runtime_version(id)
        }
        /// Get call executor reference.
        pub fn executor(&self) -> &E { &self.executor }
        /// Reads storage value at a given block + key, returning read proof.
        pub fn read_proof(&self, id: &BlockId<Block>, key: &[u8])
         -> error::Result<Vec<Vec<u8>>> {
            self.state_at(id).and_then(|state|
                                           prove_read(state,
                                                      key).map(|(_, proof)|
                                                                   proof).map_err(Into::into))
        }
        /// Reads child storage value at a given block + storage_key + key, returning
        /// read proof.
        pub fn read_child_proof(&self, id: &BlockId<Block>,
                                storage_key: &[u8], key: &[u8])
         -> error::Result<Vec<Vec<u8>>> {
            self.state_at(id).and_then(|state|
                                           prove_child_read(state,
                                                            storage_key,
                                                            key).map(|(_,
                                                                       proof)|
                                                                         proof).map_err(Into::into))
        }
        /// Execute a call to a contract on top of state in a block of given hash
        /// AND returning execution proof.
        ///
        /// No changes are made.
        pub fn execution_proof(&self, id: &BlockId<Block>, method: &str,
                               call_data: &[u8])
         -> error::Result<(Vec<u8>, Vec<Vec<u8>>)> {
            let state = self.state_at(id)?;
            let header = self.prepare_environment_block(id)?;
            prove_execution(state, header, &self.executor, method, call_data)
        }
        /// Reads given header and generates CHT-based header proof.
        pub fn header_proof(&self, id: &BlockId<Block>)
         -> error::Result<(Block::Header, Vec<Vec<u8>>)> {
            self.header_proof_with_cht_size(id, cht::SIZE)
        }
        /// Get block hash by number.
        pub fn block_hash(&self,
                          block_number:
                              <<Block as BlockT>::Header as HeaderT>::Number)
         -> error::Result<Option<Block::Hash>> {
            self.backend.blockchain().hash(block_number)
        }
        /// Reads given header and generates CHT-based header proof for CHT of given size.
        pub fn header_proof_with_cht_size(&self, id: &BlockId<Block>,
                                          cht_size: u64)
         -> error::Result<(Block::Header, Vec<Vec<u8>>)> {
            let proof_error =
                ||
                    error::Error::Backend(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Failed to generate header proof for "],
                                                                                             &match (&id,)
                                                                                                  {
                                                                                                  (arg0,)
                                                                                                  =>
                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                              })));
            let header = self.backend.blockchain().expect_header(*id)?;
            let block_num = *header.number();
            let cht_num =
                cht::block_to_cht_number(cht_size,
                                         block_num).ok_or_else(proof_error)?;
            let cht_start = cht::start_number(cht_size, cht_num);
            let headers =
                (cht_start.as_()..).map(|num| self.block_hash(As::sa(num)));
            let proof =
                cht::build_proof::<Block::Header, Blake2Hasher, _,
                                   _>(cht_size, cht_num,
                                      ::std::iter::once(block_num), headers)?;
            Ok((header, proof))
        }
        /// Get longest range within [first; last] that is possible to use in `key_changes`
        /// and `key_changes_proof` calls.
        /// Range could be shortened from the beginning if some changes tries have been pruned.
        /// Returns Ok(None) if changes trues are not supported.
        pub fn max_key_changes_range(&self, first: NumberFor<Block>,
                                     last: BlockId<Block>)
         -> error::Result<Option<(NumberFor<Block>, BlockId<Block>)>> {
            let (config, storage) =
                match self.require_changes_trie().ok() {
                    Some((config, storage)) => (config, storage),
                    None => return Ok(None),
                };
            let first = first.as_();
            let last_num =
                self.backend.blockchain().expect_block_number_from_id(&last)?.as_();
            if first > last_num {
                return Err(error::Error::ChangesTrieAccessFailed("Invalid changes trie range".into()));
            }
            let finalized_number =
                self.backend.blockchain().info()?.finalized_number;
            let oldest =
                storage.oldest_changes_trie_block(&config,
                                                  finalized_number.as_());
            let first = As::sa(::std::cmp::max(first, oldest));
            Ok(Some((first, last)))
        }
        /// Get pairs of (block, extrinsic) where key has been changed at given blocks range.
        /// Works only for runtimes that are supporting changes tries.
        pub fn key_changes(&self, first: NumberFor<Block>,
                           last: BlockId<Block>, key: &StorageKey)
         -> error::Result<Vec<(NumberFor<Block>, u32)>> {
            let (config, storage) = self.require_changes_trie()?;
            let last_number =
                self.backend.blockchain().expect_block_number_from_id(&last)?.as_();
            let last_hash =
                self.backend.blockchain().expect_block_hash_from_id(&last)?;
            key_changes::<_,
                          Blake2Hasher>(&config, &*storage, first.as_(),
                                        &ChangesTrieAnchorBlockId{hash:
                                                                      convert_hash(&last_hash),
                                                                  number:
                                                                      last_number,},
                                        self.backend.blockchain().info()?.best_number.as_(),
                                        &key.0).and_then(|r|
                                                             r.map(|r|
                                                                       r.map(|(block,
                                                                               tx)|
                                                                                 (As::sa(block),
                                                                                  tx))).collect::<Result<_,
                                                                                                         _>>()).map_err(|err|
                                                                                                                            error::Error::ChangesTrieAccessFailed(err))
        }
        /// Get proof for computation of (block, extrinsic) pairs where key has been changed at given blocks range.
        /// `min` is the hash of the first block, which changes trie root is known to the requester - when we're using
        /// changes tries from ascendants of this block, we should provide proofs for changes tries roots
        /// `max` is the hash of the last block known to the requester - we can't use changes tries from descendants
        /// of this block.
        /// Works only for runtimes that are supporting changes tries.
        pub fn key_changes_proof(&self, first: Block::Hash, last: Block::Hash,
                                 min: Block::Hash, max: Block::Hash,
                                 key: &StorageKey)
         -> error::Result<ChangesProof<Block::Header>> {
            self.key_changes_proof_with_cht_size(first, last, min, max, key,
                                                 cht::SIZE)
        }
        /// Does the same work as `key_changes_proof`, but assumes that CHTs are of passed size.
        pub fn key_changes_proof_with_cht_size(&self, first: Block::Hash,
                                               last: Block::Hash,
                                               min: Block::Hash,
                                               max: Block::Hash,
                                               key: &StorageKey,
                                               cht_size: u64)
         -> error::Result<ChangesProof<Block::Header>> {
            struct AccessedRootsRecorder<'a, Block: BlockT> {
                storage: &'a ChangesTrieStorage<Blake2Hasher>,
                min: u64,
                required_roots_proofs: Mutex<BTreeMap<NumberFor<Block>,
                                                      H256>>,
            }
            impl <'a, Block: BlockT> ChangesTrieRootsStorage<Blake2Hasher> for
             AccessedRootsRecorder<'a, Block> {
                fn root(&self, anchor: &ChangesTrieAnchorBlockId<H256>,
                        block: u64) -> Result<Option<H256>, String> {
                    let root = self.storage.root(anchor, block)?;
                    if block < self.min {
                        if let Some(ref root) = root {
                            self.required_roots_proofs.lock().insert(As::sa(block),
                                                                     root.clone());
                        }
                    }
                    Ok(root)
                }
            }
            impl <'a, Block: BlockT> ChangesTrieStorage<Blake2Hasher> for
             AccessedRootsRecorder<'a, Block> {
                fn get(&self, key: &H256, prefix: &[u8])
                 -> Result<Option<DBValue>, String> {
                    self.storage.get(key, prefix)
                }
            }
            let (config, storage) = self.require_changes_trie()?;
            let min_number =
                self.backend.blockchain().expect_block_number_from_id(&BlockId::Hash(min))?;
            let recording_storage =
                AccessedRootsRecorder::<Block>{storage,
                                               min: min_number.as_(),
                                               required_roots_proofs:
                                                   Mutex::new(BTreeMap::new()),};
            let max_number =
                ::std::cmp::min(self.backend.blockchain().info()?.best_number,
                                self.backend.blockchain().expect_block_number_from_id(&BlockId::Hash(max))?);
            let first_number =
                self.backend.blockchain().expect_block_number_from_id(&BlockId::Hash(first))?.as_();
            let last_number =
                self.backend.blockchain().expect_block_number_from_id(&BlockId::Hash(last))?.as_();
            let key_changes_proof =
                key_changes_proof::<_,
                                    Blake2Hasher>(&config, &recording_storage,
                                                  first_number,
                                                  &ChangesTrieAnchorBlockId{hash:
                                                                                convert_hash(&last),
                                                                            number:
                                                                                last_number,},
                                                  max_number.as_(),
                                                  &key.0).map_err(|err|
                                                                      error::Error::from(error::Error::ChangesTrieAccessFailed(err)))?;
            let roots = recording_storage.required_roots_proofs.into_inner();
            let roots_proof =
                self.changes_trie_roots_proof(cht_size,
                                              roots.keys().cloned())?;
            Ok(ChangesProof{max_block: max_number,
                            proof: key_changes_proof,
                            roots:
                                roots.into_iter().map(|(n, h)|
                                                          (n,
                                                           convert_hash(&h))).collect(),
                            roots_proof,})
        }
        /// Generate CHT-based proof for roots of changes tries at given blocks.
        fn changes_trie_roots_proof<I: IntoIterator<Item =
                                    NumberFor<Block>>>(&self, cht_size: u64,
                                                       blocks: I)
         -> error::Result<Vec<Vec<u8>>> {
            let mut proof = HashSet::new();
            cht::for_each_cht_group::<Block::Header, _, _,
                                      _>(cht_size, blocks,
                                         |_, cht_num, cht_blocks|
                                             {
                                                 let cht_proof =
                                                     self.changes_trie_roots_proof_at_cht(cht_size,
                                                                                          cht_num,
                                                                                          cht_blocks)?;
                                                 proof.extend(cht_proof);
                                                 Ok(())
                                             }, ())?;
            Ok(proof.into_iter().collect())
        }
        /// Generates CHT-based proof for roots of changes tries at given blocks (that are part of single CHT).
        fn changes_trie_roots_proof_at_cht(&self, cht_size: u64,
                                           cht_num: NumberFor<Block>,
                                           blocks: Vec<NumberFor<Block>>)
         -> error::Result<Vec<Vec<u8>>> {
            let cht_start = cht::start_number(cht_size, cht_num);
            let roots =
                (cht_start.as_()..).map(|num|
                                            self.header(&BlockId::Number(As::sa(num))).map(|block|
                                                                                               block.and_then(|block|
                                                                                                                  block.digest().log(DigestItem::as_changes_trie_root).cloned())));
            let proof =
                cht::build_proof::<Block::Header, Blake2Hasher, _,
                                   _>(cht_size, cht_num, blocks, roots)?;
            Ok(proof)
        }
        /// Returns changes trie configuration and storage or an error if it is not supported.
        fn require_changes_trie(&self)
         ->
             error::Result<(ChangesTrieConfiguration,
                            &B::ChangesTrieStorage)> {
            let config = self.changes_trie_config()?;
            let storage = self.backend.changes_trie_storage();
            match (config, storage) {
                (Some(config), Some(storage)) => Ok((config, storage)),
                _ => Err(error::Error::ChangesTriesNotSupported.into()),
            }
        }
        /// Create a new block, built on the head of the chain.
        pub fn new_block(&self)
         -> error::Result<block_builder::BlockBuilder<Block, Self>> where
         E: Clone + Send + Sync, RA: Send + Sync, Self: ProvideRuntimeApi,
         <Self as ProvideRuntimeApi>::Api: BlockBuilderAPI<Block> {
            block_builder::BlockBuilder::new(self)
        }
        /// Create a new block, built on top of `parent`.
        pub fn new_block_at(&self, parent: &BlockId<Block>)
         -> error::Result<block_builder::BlockBuilder<Block, Self>> where
         E: Clone + Send + Sync, RA: Send + Sync, Self: ProvideRuntimeApi,
         <Self as ProvideRuntimeApi>::Api: BlockBuilderAPI<Block> {
            block_builder::BlockBuilder::at_block(parent, &self, false)
        }
        /// Create a new block, built on top of `parent` with proof recording enabled.
        ///
        /// While proof recording is enabled, all accessed trie nodes are saved.
        /// These recorded trie nodes can be used by a third party to proof the
        /// output of this block builder without having access to the full storage.
        pub fn new_block_at_with_proof_recording(&self,
                                                 parent: &BlockId<Block>)
         -> error::Result<block_builder::BlockBuilder<Block, Self>> where
         E: Clone + Send + Sync, RA: Send + Sync, Self: ProvideRuntimeApi,
         <Self as ProvideRuntimeApi>::Api: BlockBuilderAPI<Block> {
            block_builder::BlockBuilder::at_block(parent, &self, true)
        }
        /// Lock the import lock, and run operations inside.
        pub fn lock_import_and_run<R, Err, F>(&self, f: F) -> Result<R, Err>
         where F: FnOnce(&mut ClientImportOperation<Block, Blake2Hasher, B>)
         -> Result<R, Err>, Err: From<error::Error> {
            let inner =
                ||
                    {
                        let _import_lock = self.import_lock.lock();
                        let mut op =
                            ClientImportOperation{op:
                                                      self.backend.begin_operation()?,
                                                  notify_imported: None,
                                                  notify_finalized:
                                                      Vec::new(),};
                        let r = f(&mut op)?;
                        let ClientImportOperation {
                                op, notify_imported, notify_finalized } = op;
                        self.backend.commit_operation(op)?;
                        self.notify_finalized(notify_finalized)?;
                        if let Some(notify_imported) = notify_imported {
                            self.notify_imported(notify_imported)?;
                        }
                        Ok(r)
                    };
            let result = inner();
            *self.importing_block.write() = None;
            result
        }
        /// Set a block as best block.
        pub fn set_head(&self, id: BlockId<Block>) -> error::Result<()> {
            self.lock_import_and_run(|operation|
                                         { self.apply_head(operation, id) })
        }
        /// Set a block as best block, and apply it to an operation.
        pub fn apply_head(&self,
                          operation:
                              &mut ClientImportOperation<Block, Blake2Hasher,
                                                         B>,
                          id: BlockId<Block>) -> error::Result<()> {
            operation.op.mark_head(id)
        }
        /// Apply a checked and validated block to an operation. If a justification is provided
        /// then `finalized` *must* be true.
        pub fn apply_block(&self,
                           operation:
                               &mut ClientImportOperation<Block, Blake2Hasher,
                                                          B>,
                           import_block: ImportBlock<Block>,
                           new_cache: HashMap<CacheKeyId, Vec<u8>>)
         -> error::Result<ImportResult> where
         E: CallExecutor<Block, Blake2Hasher> + Send + Sync + Clone {
            let ImportBlock {
                    origin,
                    header,
                    justification,
                    post_digests,
                    body,
                    finalized,
                    auxiliary,
                    fork_choice } = import_block;
            if !(justification.is_some() && finalized ||
                     justification.is_none()) {
                {
                    ::std::rt::begin_panic("assertion failed: justification.is_some() && finalized || justification.is_none()",
                                           &("core/client/src/client.rs",
                                             760u32, 3u32))
                }
            };
            let parent_hash = header.parent_hash().clone();
            match self.backend.blockchain().status(BlockId::Hash(parent_hash))?
                {
                blockchain::BlockStatus::InChain => { }
                blockchain::BlockStatus::Unknown =>
                return Ok(ImportResult::UnknownParent),
            }
            let import_headers =
                if post_digests.is_empty() {
                    PrePostHeader::Same(header)
                } else {
                    let mut post_header = header.clone();
                    for item in post_digests {
                        post_header.digest_mut().push(item);
                    }
                    PrePostHeader::Different(header, post_header)
                };
            let hash = import_headers.post().hash();
            let height: u64 = import_headers.post().number().as_();
            *self.importing_block.write() = Some(hash);
            let result =
                self.execute_and_import_block(operation, origin, hash,
                                              import_headers, justification,
                                              body, new_cache, finalized,
                                              auxiliary, fork_choice);
            ::substrate_telemetry::with_logger(|l|
                                                   {
                                                       if ::slog::Level::Info.as_usize()
                                                              <=
                                                              ::slog::__slog_static_max_level().as_usize()
                                                          {
                                                           l.log(&{
                                                                      static RS:
                                                                             ::slog::RecordStatic<'static>
                                                                             =
                                                                          {
                                                                              static LOC:
                                                                                     ::slog::RecordLocation
                                                                                     =
                                                                                  ::slog::RecordLocation{file:
                                                                                                             "core/client/src/client.rs",
                                                                                                         line:
                                                                                                             797u32,
                                                                                                         column:
                                                                                                             3u32,
                                                                                                         function:
                                                                                                             "",
                                                                                                         module:
                                                                                                             "substrate_client::client",};
                                                                              ::slog::RecordStatic{location:
                                                                                                       &LOC,
                                                                                                   level:
                                                                                                       ::slog::Level::Info,
                                                                                                   tag:
                                                                                                       SUBSTRATE_INFO,}
                                                                          };
                                                                      ::slog::Record::new(&RS,
                                                                                          &::std::fmt::Arguments::new_v1(&["block.import"],
                                                                                                                         &match ()
                                                                                                                              {
                                                                                                                              ()
                                                                                                                              =>
                                                                                                                              [],
                                                                                                                          }),
                                                                                          ::slog::BorrowedKV(&(::slog::SingleKV::from(("origin",
                                                                                                                                       ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                     &match (&origin,)
                                                                                                                                                                          {
                                                                                                                                                                          (arg0,)
                                                                                                                                                                          =>
                                                                                                                                                                          [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                       ::std::fmt::Debug::fmt)],
                                                                                                                                                                      }))),
                                                                                                               (::slog::SingleKV::from(("best",
                                                                                                                                        ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                      &match (&hash,)
                                                                                                                                                                           {
                                                                                                                                                                           (arg0,)
                                                                                                                                                                           =>
                                                                                                                                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                        ::std::fmt::Debug::fmt)],
                                                                                                                                                                       }))),
                                                                                                                (::slog::SingleKV::from(("height",
                                                                                                                                         height)),
                                                                                                                 ())))))
                                                                  })
                                                       }
                                                   });
            result
        }
        fn execute_and_import_block(&self,
                                    operation:
                                        &mut ClientImportOperation<Block,
                                                                   Blake2Hasher,
                                                                   B>,
                                    origin: BlockOrigin, hash: Block::Hash,
                                    import_headers:
                                        PrePostHeader<Block::Header>,
                                    justification: Option<Justification>,
                                    body: Option<Vec<Block::Extrinsic>>,
                                    new_cache: HashMap<CacheKeyId, Vec<u8>>,
                                    finalized: bool,
                                    aux: Vec<(Vec<u8>, Option<Vec<u8>>)>,
                                    fork_choice: ForkChoiceStrategy)
         -> error::Result<ImportResult> where
         E: CallExecutor<Block, Blake2Hasher> + Send + Sync + Clone {
            let parent_hash = import_headers.post().parent_hash().clone();
            match self.backend.blockchain().status(BlockId::Hash(hash))? {
                blockchain::BlockStatus::InChain =>
                return Ok(ImportResult::AlreadyInChain),
                blockchain::BlockStatus::Unknown => { }
            }
            let (last_best, last_best_number) =
                {
                    let info = self.backend.blockchain().info()?;
                    (info.best_hash, info.best_number)
                };
            let make_notifications =
                match origin {
                    BlockOrigin::NetworkBroadcast | BlockOrigin::Own |
                    BlockOrigin::ConsensusBroadcast => true,
                    BlockOrigin::Genesis | BlockOrigin::NetworkInitialSync |
                    BlockOrigin::File => false,
                };
            self.backend.begin_state_operation(&mut operation.op,
                                               BlockId::Hash(parent_hash))?;
            if finalized {
                self.apply_finality_with_block_hash(operation, parent_hash,
                                                    None, last_best,
                                                    make_notifications)?;
            }
            let (storage_update, changes_update, storage_changes) =
                self.block_execution(&operation.op, &import_headers, origin,
                                     hash, body.clone())?;
            let is_new_best =
                finalized ||
                    match fork_choice {
                        ForkChoiceStrategy::LongestChain =>
                        import_headers.post().number() > &last_best_number,
                        ForkChoiceStrategy::Custom(v) => v,
                    };
            let leaf_state =
                if finalized {
                    crate::backend::NewBlockState::Final
                } else if is_new_best {
                    crate::backend::NewBlockState::Best
                } else { crate::backend::NewBlockState::Normal };
            {
                let lvl = ::log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Imported ",
                                                                             ", (#",
                                                                             "), best=",
                                                                             ", origin="],
                                                                           &match (&hash,
                                                                                   &import_headers.post().number(),
                                                                                   &is_new_best,
                                                                                   &origin)
                                                                                {
                                                                                (arg0,
                                                                                 arg1,
                                                                                 arg2,
                                                                                 arg3)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg2,
                                                                                                             ::std::fmt::Display::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg3,
                                                                                                             ::std::fmt::Debug::fmt)],
                                                                            }),
                                             lvl,
                                             &("substrate_client::client",
                                               "substrate_client::client",
                                               "core/client/src/client.rs",
                                               863u32));
                }
            };
            operation.op.set_block_data(import_headers.post().clone(), body,
                                        justification, leaf_state)?;
            operation.op.update_cache(new_cache);
            if let Some(storage_update) = storage_update {
                operation.op.update_db_storage(storage_update)?;
            }
            if let Some(storage_changes) = storage_changes.clone() {
                operation.op.update_storage(storage_changes)?;
            }
            if let Some(Some(changes_update)) = changes_update {
                operation.op.update_changes_trie(changes_update)?;
            }
            operation.op.insert_aux(aux)?;
            if make_notifications {
                if finalized { operation.notify_finalized.push(hash); }
                operation.notify_imported =
                    Some((hash, origin, import_headers.into_post(),
                          is_new_best, storage_changes));
            }
            Ok(ImportResult::imported())
        }
        fn block_execution(&self, transaction: &B::BlockImportOperation,
                           import_headers: &PrePostHeader<Block::Header>,
                           origin: BlockOrigin, hash: Block::Hash,
                           body: Option<Vec<Block::Extrinsic>>)
         ->
             error::Result<(Option<StorageUpdate<B, Block>>,
                            Option<Option<ChangesUpdate>>,
                            Option<Vec<(Vec<u8>, Option<Vec<u8>>)>>)> where
         E: CallExecutor<Block, Blake2Hasher> + Send + Sync + Clone {
            match transaction.state()? {
                Some(transaction_state) => {
                    let mut overlay = Default::default();
                    let get_execution_manager =
                        |execution_strategy: ExecutionStrategy|
                            {
                                match execution_strategy {
                                    ExecutionStrategy::NativeElseWasm =>
                                    ExecutionManager::NativeElseWasm,
                                    ExecutionStrategy::AlwaysWasm =>
                                    ExecutionManager::AlwaysWasm,
                                    ExecutionStrategy::NativeWhenPossible =>
                                    ExecutionManager::NativeWhenPossible,
                                    ExecutionStrategy::Both =>
                                    ExecutionManager::Both(|wasm_result,
                                                            native_result|
                                                               {
                                                                   let header =
                                                                       import_headers.post();
                                                                   {
                                                                       let lvl =
                                                                           ::log::Level::Warn;
                                                                       if lvl
                                                                              <=
                                                                              ::log::STATIC_MAX_LEVEL
                                                                              &&
                                                                              lvl
                                                                                  <=
                                                                                  ::log::max_level()
                                                                          {
                                                                           ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Consensus error between wasm and native block execution at block "],
                                                                                                                                  &match (&hash,)
                                                                                                                                       {
                                                                                                                                       (arg0,)
                                                                                                                                       =>
                                                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                    ::std::fmt::Display::fmt)],
                                                                                                                                   }),
                                                                                                    lvl,
                                                                                                    &("substrate_client::client",
                                                                                                      "substrate_client::client",
                                                                                                      "core/client/src/client.rs",
                                                                                                      921u32));
                                                                       }
                                                                   };
                                                                   {
                                                                       let lvl =
                                                                           ::log::Level::Warn;
                                                                       if lvl
                                                                              <=
                                                                              ::log::STATIC_MAX_LEVEL
                                                                              &&
                                                                              lvl
                                                                                  <=
                                                                                  ::log::max_level()
                                                                          {
                                                                           ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["   Header "],
                                                                                                                                  &match (&header,)
                                                                                                                                       {
                                                                                                                                       (arg0,)
                                                                                                                                       =>
                                                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                                                                   }),
                                                                                                    lvl,
                                                                                                    &("substrate_client::client",
                                                                                                      "substrate_client::client",
                                                                                                      "core/client/src/client.rs",
                                                                                                      922u32));
                                                                       }
                                                                   };
                                                                   {
                                                                       let lvl =
                                                                           ::log::Level::Warn;
                                                                       if lvl
                                                                              <=
                                                                              ::log::STATIC_MAX_LEVEL
                                                                              &&
                                                                              lvl
                                                                                  <=
                                                                                  ::log::max_level()
                                                                          {
                                                                           ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["   Native result "],
                                                                                                                                  &match (&native_result,)
                                                                                                                                       {
                                                                                                                                       (arg0,)
                                                                                                                                       =>
                                                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                                                                   }),
                                                                                                    lvl,
                                                                                                    &("substrate_client::client",
                                                                                                      "substrate_client::client",
                                                                                                      "core/client/src/client.rs",
                                                                                                      923u32));
                                                                       }
                                                                   };
                                                                   {
                                                                       let lvl =
                                                                           ::log::Level::Warn;
                                                                       if lvl
                                                                              <=
                                                                              ::log::STATIC_MAX_LEVEL
                                                                              &&
                                                                              lvl
                                                                                  <=
                                                                                  ::log::max_level()
                                                                          {
                                                                           ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["   Wasm result "],
                                                                                                                                  &match (&wasm_result,)
                                                                                                                                       {
                                                                                                                                       (arg0,)
                                                                                                                                       =>
                                                                                                                                       [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                                                                   }),
                                                                                                    lvl,
                                                                                                    &("substrate_client::client",
                                                                                                      "substrate_client::client",
                                                                                                      "core/client/src/client.rs",
                                                                                                      924u32));
                                                                       }
                                                                   };
                                                                   ::substrate_telemetry::with_logger(|l|
                                                                                                          {
                                                                                                              if ::slog::Level::Info.as_usize()
                                                                                                                     <=
                                                                                                                     ::slog::__slog_static_max_level().as_usize()
                                                                                                                 {
                                                                                                                  l.log(&{
                                                                                                                             static RS:
                                                                                                                                    ::slog::RecordStatic<'static>
                                                                                                                                    =
                                                                                                                                 {
                                                                                                                                     static LOC:
                                                                                                                                            ::slog::RecordLocation
                                                                                                                                            =
                                                                                                                                         ::slog::RecordLocation{file:
                                                                                                                                                                    "core/client/src/client.rs",
                                                                                                                                                                line:
                                                                                                                                                                    925u32,
                                                                                                                                                                column:
                                                                                                                                                                    8u32,
                                                                                                                                                                function:
                                                                                                                                                                    "",
                                                                                                                                                                module:
                                                                                                                                                                    "substrate_client::client",};
                                                                                                                                     ::slog::RecordStatic{location:
                                                                                                                                                              &LOC,
                                                                                                                                                          level:
                                                                                                                                                              ::slog::Level::Info,
                                                                                                                                                          tag:
                                                                                                                                                              SUBSTRATE_INFO,}
                                                                                                                                 };
                                                                                                                             ::slog::Record::new(&RS,
                                                                                                                                                 &::std::fmt::Arguments::new_v1(&["block.execute.consensus_failure"],
                                                                                                                                                                                &match ()
                                                                                                                                                                                     {
                                                                                                                                                                                     ()
                                                                                                                                                                                     =>
                                                                                                                                                                                     [],
                                                                                                                                                                                 }),
                                                                                                                                                 ::slog::BorrowedKV(&(::slog::SingleKV::from(("header",
                                                                                                                                                                                              ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                                            &match (&header,)
                                                                                                                                                                                                                                 {
                                                                                                                                                                                                                                 (arg0,)
                                                                                                                                                                                                                                 =>
                                                                                                                                                                                                                                 [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                                              ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                                             }))),
                                                                                                                                                                      (::slog::SingleKV::from(("origin",
                                                                                                                                                                                               ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                                             &match (&origin,)
                                                                                                                                                                                                                                  {
                                                                                                                                                                                                                                  (arg0,)
                                                                                                                                                                                                                                  =>
                                                                                                                                                                                                                                  [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                                               ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                                              }))),
                                                                                                                                                                       (::slog::SingleKV::from(("hash",
                                                                                                                                                                                                ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                                                              &match (&hash,)
                                                                                                                                                                                                                                   {
                                                                                                                                                                                                                                   (arg0,)
                                                                                                                                                                                                                                   =>
                                                                                                                                                                                                                                   [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                                                ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                                               }))),
                                                                                                                                                                        ())))))
                                                                                                                         })
                                                                                                              }
                                                                                                          });
                                                                   wasm_result
                                                               }),
                                }
                            };
                    let (_, storage_update, changes_update) =
                        self.executor.call_at_state::<_, _, _,
                                                      NeverNativeValue,
                                                      fn()
                                                          ->
                                                              _>(transaction_state,
                                                                 &mut overlay,
                                                                 "Core_execute_block",
                                                                 &<Block as
                                                                      BlockT>::new(import_headers.pre().clone(),
                                                                                   body.unwrap_or_default()).encode(),
                                                                 match origin
                                                                     {
                                                                     BlockOrigin::NetworkInitialSync
                                                                     =>
                                                                     get_execution_manager(self.execution_strategies().syncing),
                                                                     _ =>
                                                                     get_execution_manager(self.execution_strategies().importing),
                                                                 }, None,
                                                                 NeverOffchainExt::new())?;
                    overlay.commit_prospective();
                    Ok((Some(storage_update), Some(changes_update),
                        Some(overlay.into_committed().collect())))
                }
                None => Ok((None, None, None)),
            }
        }
        fn apply_finality_with_block_hash(&self,
                                          operation:
                                              &mut ClientImportOperation<Block,
                                                                         Blake2Hasher,
                                                                         B>,
                                          block: Block::Hash,
                                          justification:
                                              Option<Justification>,
                                          best_block: Block::Hash,
                                          notify: bool) -> error::Result<()> {
            let last_finalized = self.backend.blockchain().last_finalized()?;
            if block == last_finalized {
                {
                    let lvl = ::log::Level::Warn;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Possible safety violation: attempted to re-finalize last finalized block ",
                                                                                 " "],
                                                                               &match (&last_finalized,)
                                                                                    {
                                                                                    (arg0,)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Debug::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("substrate_client::client",
                                                   "substrate_client::client",
                                                   "core/client/src/client.rs",
                                                   967u32));
                    }
                };
                return Ok(());
            }
            let route_from_finalized =
                crate::blockchain::tree_route(self.backend.blockchain(),
                                              BlockId::Hash(last_finalized),
                                              BlockId::Hash(block))?;
            if let Some(retracted) = route_from_finalized.retracted().get(0) {
                {
                    let lvl = ::log::Level::Warn;
                    if lvl <= ::log::STATIC_MAX_LEVEL &&
                           lvl <= ::log::max_level() {
                        ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Safety violation: attempted to revert finalized block ",
                                                                                 " which is not in the same chain as last finalized "],
                                                                               &match (&retracted,
                                                                                       &last_finalized)
                                                                                    {
                                                                                    (arg0,
                                                                                     arg1)
                                                                                    =>
                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::std::fmt::Debug::fmt),
                                                                                     ::std::fmt::ArgumentV1::new(arg1,
                                                                                                                 ::std::fmt::Debug::fmt)],
                                                                                }),
                                                 lvl,
                                                 &("substrate_client::client",
                                                   "substrate_client::client",
                                                   "core/client/src/client.rs",
                                                   978u32));
                    }
                };
                return Err(error::Error::NotInFinalizedChain);
            }
            let route_from_best =
                crate::blockchain::tree_route(self.backend.blockchain(),
                                              BlockId::Hash(best_block),
                                              BlockId::Hash(block))?;
            if route_from_best.common_block().hash != block { }
            let enacted = route_from_finalized.enacted();
            if !(enacted.len() > 0) {
                {
                    ::std::rt::begin_panic("assertion failed: enacted.len() > 0",
                                           &("core/client/src/client.rs",
                                             998u32, 3u32))
                }
            };
            for finalize_new in &enacted[..enacted.len() - 1] {
                operation.op.mark_finalized(BlockId::Hash(finalize_new.hash),
                                            None)?;
            }
            {
                match (&enacted.last().map(|e| e.hash), &Some(block)) {
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
                                                           &("core/client/src/client.rs",
                                                             1003u32, 3u32))
                            }
                        }
                    }
                }
            };
            operation.op.mark_finalized(BlockId::Hash(block), justification)?;
            if notify {
                const MAX_TO_NOTIFY: usize = 256;
                let enacted = route_from_finalized.enacted();
                let start =
                    enacted.len() -
                        ::std::cmp::min(enacted.len(), MAX_TO_NOTIFY);
                for finalized in &enacted[start..] {
                    operation.notify_finalized.push(finalized.hash);
                }
            }
            Ok(())
        }
        fn notify_finalized(&self, notify_finalized: Vec<Block::Hash>)
         -> error::Result<()> {
            let mut sinks = self.finality_notification_sinks.lock();
            for finalized_hash in notify_finalized {
                let header =
                    self.header(&BlockId::Hash(finalized_hash))?.expect("header already known to exist in DB because it is indicated in the tree route; qed");
                ::substrate_telemetry::with_logger(|l|
                                                       {
                                                           if ::slog::Level::Info.as_usize()
                                                                  <=
                                                                  ::slog::__slog_static_max_level().as_usize()
                                                              {
                                                               l.log(&{
                                                                          static RS:
                                                                                 ::slog::RecordStatic<'static>
                                                                                 =
                                                                              {
                                                                                  static LOC:
                                                                                         ::slog::RecordLocation
                                                                                         =
                                                                                      ::slog::RecordLocation{file:
                                                                                                                 "core/client/src/client.rs",
                                                                                                             line:
                                                                                                                 1030u32,
                                                                                                             column:
                                                                                                                 4u32,
                                                                                                             function:
                                                                                                                 "",
                                                                                                             module:
                                                                                                                 "substrate_client::client",};
                                                                                  ::slog::RecordStatic{location:
                                                                                                           &LOC,
                                                                                                       level:
                                                                                                           ::slog::Level::Info,
                                                                                                       tag:
                                                                                                           SUBSTRATE_INFO,}
                                                                              };
                                                                          ::slog::Record::new(&RS,
                                                                                              &::std::fmt::Arguments::new_v1(&["notify.finalized"],
                                                                                                                             &match ()
                                                                                                                                  {
                                                                                                                                  ()
                                                                                                                                  =>
                                                                                                                                  [],
                                                                                                                              }),
                                                                                              ::slog::BorrowedKV(&(::slog::SingleKV::from(("best",
                                                                                                                                           ::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                         &match (&finalized_hash,)
                                                                                                                                                                              {
                                                                                                                                                                              (arg0,)
                                                                                                                                                                              =>
                                                                                                                                                                              [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                           ::std::fmt::Debug::fmt)],
                                                                                                                                                                          }))),
                                                                                                                   (::slog::SingleKV::from(("height",
                                                                                                                                            ::alloc::fmt::format(::std::fmt::Arguments::new_v1(&[""],
                                                                                                                                                                                               &match (&header.number(),)
                                                                                                                                                                                                    {
                                                                                                                                                                                                    (arg0,)
                                                                                                                                                                                                    =>
                                                                                                                                                                                                    [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                                 ::std::fmt::Display::fmt)],
                                                                                                                                                                                                })))),
                                                                                                                    ()))))
                                                                      })
                                                           }
                                                       });
                let notification =
                    FinalityNotification{header, hash: finalized_hash,};
                sinks.retain(|sink|
                                 sink.unbounded_send(notification.clone()).is_ok());
            }
            Ok(())
        }
        fn notify_imported(&self,
                           notify_import:
                               (Block::Hash, BlockOrigin, Block::Header, bool,
                                Option<Vec<(Vec<u8>, Option<Vec<u8>>)>>))
         -> error::Result<()> {
            let (hash, origin, header, is_new_best, storage_changes) =
                notify_import;
            if let Some(storage_changes) = storage_changes {
                self.storage_notifications.lock().trigger(&hash,
                                                          storage_changes.into_iter());
            }
            let notification =
                BlockImportNotification::<Block>{hash,
                                                 origin,
                                                 header,
                                                 is_new_best,};
            self.import_notification_sinks.lock().retain(|sink|
                                                             sink.unbounded_send(notification.clone()).is_ok());
            Ok(())
        }
        /// Apply auxiliary data insertion into an operation.
        pub fn apply_aux<'a, 'b: 'a, 'c: 'a, I: IntoIterator<Item =
                         &'a (&'c [u8], &'c [u8])>, D: IntoIterator<Item =
                         &'a &'b [u8]>>(&self,
                                        operation:
                                            &mut ClientImportOperation<Block,
                                                                       Blake2Hasher,
                                                                       B>,
                                        insert: I, delete: D)
         -> error::Result<()> {
            operation.op.insert_aux(insert.into_iter().map(|(k, v)|
                                                               (k.to_vec(),
                                                                Some(v.to_vec()))).chain(delete.into_iter().map(|k|
                                                                                                                    (k.to_vec(),
                                                                                                                     None))))
        }
        /// Mark all blocks up to given as finalized in operation. If a
        /// justification is provided it is stored with the given finalized
        /// block (any other finalized blocks are left unjustified).
        pub fn apply_finality(&self,
                              operation:
                                  &mut ClientImportOperation<Block,
                                                             Blake2Hasher, B>,
                              id: BlockId<Block>,
                              justification: Option<Justification>,
                              notify: bool) -> error::Result<()> {
            let last_best = self.backend.blockchain().info()?.best_hash;
            let to_finalize_hash =
                self.backend.blockchain().expect_block_hash_from_id(&id)?;
            self.apply_finality_with_block_hash(operation, to_finalize_hash,
                                                justification, last_best,
                                                notify)
        }
        /// Finalize a block. This will implicitly finalize all blocks up to it and
        /// fire finality notifications.
        ///
        /// Pass a flag to indicate whether finality notifications should be propagated.
        /// This is usually tied to some synchronization state, where we don't send notifications
        /// while performing major synchronization work.
        pub fn finalize_block(&self, id: BlockId<Block>,
                              justification: Option<Justification>,
                              notify: bool) -> error::Result<()> {
            self.lock_import_and_run(|operation|
                                         {
                                             let last_best =
                                                 self.backend.blockchain().info()?.best_hash;
                                             let to_finalize_hash =
                                                 self.backend.blockchain().expect_block_hash_from_id(&id)?;
                                             self.apply_finality_with_block_hash(operation,
                                                                                 to_finalize_hash,
                                                                                 justification,
                                                                                 last_best,
                                                                                 notify)
                                         })
        }
        /// Attempts to revert the chain by `n` blocks. Returns the number of blocks that were
        /// successfully reverted.
        pub fn revert(&self, n: NumberFor<Block>)
         -> error::Result<NumberFor<Block>> {
            Ok(self.backend.revert(n)?)
        }
        /// Get blockchain info.
        pub fn info(&self) -> error::Result<ClientInfo<Block>> {
            let info =
                self.backend.blockchain().info().map_err(|e|
                                                             error::Error::from_blockchain(Box::new(e)))?;
            Ok(ClientInfo{chain: info,
                          best_queued_hash: None,
                          best_queued_number: None,})
        }
        /// Get block status.
        pub fn block_status(&self, id: &BlockId<Block>)
         -> error::Result<BlockStatus> {
            if let BlockId::Hash(ref h) = id {
                if self.importing_block.read().as_ref().map_or(false,
                                                               |importing|
                                                                   h ==
                                                                       importing)
                   {
                    return Ok(BlockStatus::Queued);
                }
            }
            let hash_and_number =
                match id.clone() {
                    BlockId::Hash(hash) =>
                    self.backend.blockchain().number(hash)?.map(|n|
                                                                    (hash,
                                                                     n)),
                    BlockId::Number(n) =>
                    self.backend.blockchain().hash(n)?.map(|hash| (hash, n)),
                };
            match hash_and_number {
                Some((hash, number)) => {
                    if self.backend.have_state_at(&hash, number) {
                        Ok(BlockStatus::InChainWithState)
                    } else { Ok(BlockStatus::InChainPruned) }
                }
                None => Ok(BlockStatus::Unknown),
            }
        }
        /// Get block header by id.
        pub fn header(&self, id: &BlockId<Block>)
         -> error::Result<Option<<Block as BlockT>::Header>> {
            self.backend.blockchain().header(*id)
        }
        /// Get block body by id.
        pub fn body(&self, id: &BlockId<Block>)
         -> error::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {
            self.backend.blockchain().body(*id)
        }
        /// Get block justification set by id.
        pub fn justification(&self, id: &BlockId<Block>)
         -> error::Result<Option<Justification>> {
            self.backend.blockchain().justification(*id)
        }
        /// Get full block by id.
        pub fn block(&self, id: &BlockId<Block>)
         -> error::Result<Option<SignedBlock<Block>>> {
            Ok(match (self.header(id)?, self.body(id)?,
                      self.justification(id)?) {
                   (Some(header), Some(extrinsics), justification) =>
                   Some(SignedBlock{block: Block::new(header, extrinsics),
                                    justification,}),
                   _ => None,
               })
        }
        /// Gets the uncles of the block with `target_hash` going back `max_generation` ancestors.
        pub fn uncles(&self, target_hash: Block::Hash,
                      max_generation: NumberFor<Block>)
         -> error::Result<Vec<Block::Hash>> {
            let load_header =
                |id: Block::Hash| -> error::Result<Block::Header>
                    {
                        match self.backend.blockchain().header(BlockId::Hash(id))?
                            {
                            Some(hdr) => Ok(hdr),
                            None =>
                            Err(Error::UnknownBlock(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["Unknown block "],
                                                                                                       &match (&id,)
                                                                                                            {
                                                                                                            (arg0,)
                                                                                                            =>
                                                                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                         ::std::fmt::Debug::fmt)],
                                                                                                        })))),
                        }
                    };
            let genesis_hash = self.backend.blockchain().info()?.genesis_hash;
            if genesis_hash == target_hash { return Ok(Vec::new()); }
            let mut current_hash = target_hash;
            let mut current = load_header(current_hash)?;
            let mut ancestor_hash = *current.parent_hash();
            let mut ancestor = load_header(ancestor_hash)?;
            let mut uncles = Vec::new();
            for _generation in 0..max_generation.as_() {
                let children =
                    self.backend.blockchain().children(ancestor_hash)?;
                uncles.extend(children.into_iter().filter(|h|
                                                              h !=
                                                                  &current_hash));
                current_hash = ancestor_hash;
                if genesis_hash == current_hash { break ; }
                current = ancestor;
                ancestor_hash = *current.parent_hash();
                ancestor = load_header(ancestor_hash)?;
            }
            Ok(uncles)
        }
        fn changes_trie_config(&self)
         -> Result<Option<ChangesTrieConfiguration>, Error> {
            Ok(self.backend.state_at(BlockId::Number(self.backend.blockchain().info()?.best_number))?.storage(well_known_keys::CHANGES_TRIE_CONFIG).map_err(|e|
                                                                                                                                                                error::Error::from_state(Box::new(e)))?.and_then(|c|
                                                                                                                                                                                                                     Decode::decode(&mut &*c)))
        }
        /// Prepare in-memory header that is used in execution environment.
        fn prepare_environment_block(&self, parent: &BlockId<Block>)
         -> error::Result<Block::Header> {
            Ok(<<Block as BlockT>::Header as
                   HeaderT>::new(self.backend.blockchain().expect_block_number_from_id(parent)?
                                     + As::sa(1), Default::default(),
                                 Default::default(),
                                 self.backend.blockchain().expect_block_hash_from_id(&parent)?,
                                 Default::default()))
        }
    }
    impl <B, E, Block, RA> ChainHeaderBackend<Block> for
     Client<B, E, Block, RA> where B: backend::Backend<Block, Blake2Hasher>,
     E: CallExecutor<Block, Blake2Hasher> + Send + Sync, Block: BlockT<Hash =
     H256>, RA: Send + Sync {
        fn header(&self, id: BlockId<Block>)
         -> error::Result<Option<Block::Header>> {
            self.backend.blockchain().header(id)
        }
        fn info(&self) -> error::Result<blockchain::Info<Block>> {
            self.backend.blockchain().info()
        }
        fn status(&self, id: BlockId<Block>)
         -> error::Result<blockchain::BlockStatus> {
            self.backend.blockchain().status(id)
        }
        fn number(&self, hash: Block::Hash)
         ->
             error::Result<Option<<<Block as BlockT>::Header as
                                  HeaderT>::Number>> {
            self.backend.blockchain().number(hash)
        }
        fn hash(&self, number: NumberFor<Block>)
         -> error::Result<Option<Block::Hash>> {
            self.backend.blockchain().hash(number)
        }
    }
    impl <B, E, Block, RA> ProvideCache<Block> for Client<B, E, Block, RA>
     where B: backend::Backend<Block, Blake2Hasher>, Block: BlockT<Hash =
     H256> {
        fn cache(&self) -> Option<Arc<Cache<Block>>> {
            self.backend.blockchain().cache()
        }
    }
    impl <B, E, Block, RA> ProvideRuntimeApi for Client<B, E, Block, RA> where
     B: backend::Backend<Block, Blake2Hasher>,
     E: CallExecutor<Block, Blake2Hasher> + Clone + Send + Sync,
     Block: BlockT<Hash = H256>, RA: ConstructRuntimeApi<Block, Self> {
        type
        Api
        =
        <RA as ConstructRuntimeApi<Block, Self>>::RuntimeApi;
        fn runtime_api<'a>(&'a self) -> ApiRef<'a, Self::Api> {
            RA::construct_runtime_api(self)
        }
    }
    impl <B, E, Block, RA> CallRuntimeAt<Block> for Client<B, E, Block, RA>
     where B: backend::Backend<Block, Blake2Hasher>,
     E: CallExecutor<Block, Blake2Hasher> + Clone + Send + Sync,
     Block: BlockT<Hash = H256> {
        fn call_api_at<'a, R: Encode + Decode + PartialEq, NC: FnOnce() ->
                       result::Result<R, &'static str> + UnwindSafe,
                       C: CoreApi<Block>>(&self, core_api: &C,
                                          at: &BlockId<Block>,
                                          function: &'static str,
                                          args: Vec<u8>,
                                          changes: &RefCell<OverlayedChanges>,
                                          initialize_block:
                                              InitializeBlock<'a, Block>,
                                          native_call: Option<NC>,
                                          context: ExecutionContext,
                                          recorder:
                                              &Option<Rc<RefCell<ProofRecorder<Block>>>>)
         -> error::Result<NativeOrEncoded<R>> {
            let manager =
                match context {
                    ExecutionContext::BlockConstruction =>
                    self.execution_strategies.block_construction.get_manager(),
                    ExecutionContext::Syncing =>
                    self.execution_strategies.syncing.get_manager(),
                    ExecutionContext::Importing =>
                    self.execution_strategies.importing.get_manager(),
                    ExecutionContext::OffchainWorker(_) =>
                    self.execution_strategies.offchain_worker.get_manager(),
                    ExecutionContext::Other =>
                    self.execution_strategies.other.get_manager(),
                };
            let mut offchain_extensions =
                match context {
                    ExecutionContext::OffchainWorker(ext) => Some(ext),
                    _ => None,
                };
            self.executor.contextual_call::<_, _, fn(_, _) -> _, _,
                                            _>(||
                                                   core_api.initialize_block(at,
                                                                             &self.prepare_environment_block(at)?),
                                               at, function, &args, changes,
                                               initialize_block, manager,
                                               native_call,
                                               offchain_extensions.as_mut(),
                                               recorder)
        }
        fn runtime_version_at(&self, at: &BlockId<Block>)
         -> error::Result<RuntimeVersion> {
            self.runtime_version_at(at)
        }
    }
    impl <B, E, Block, RA> consensus::BlockImport<Block> for
     Client<B, E, Block, RA> where B: backend::Backend<Block, Blake2Hasher>,
     E: CallExecutor<Block, Blake2Hasher> + Clone + Send + Sync,
     Block: BlockT<Hash = H256> {
        type
        Error
        =
        ConsensusError;
        /// Import a checked and validated block. If a justification is provided in
        /// `ImportBlock` then `finalized` *must* be true.
        fn import_block(&self, import_block: ImportBlock<Block>,
                        new_cache: HashMap<CacheKeyId, Vec<u8>>)
         -> Result<ImportResult, Self::Error> {
            self.lock_import_and_run(|operation|
                                         {
                                             self.apply_block(operation,
                                                              import_block,
                                                              new_cache)
                                         }).map_err(|e|
                                                        ConsensusErrorKind::ClientImport(e.to_string()).into())
        }
        /// Check block preconditions.
        fn check_block(&self, hash: Block::Hash, parent_hash: Block::Hash)
         -> Result<ImportResult, Self::Error> {
            match self.block_status(&BlockId::Hash(parent_hash)).map_err(|e|
                                                                             ConsensusError::from(ConsensusErrorKind::ClientImport(e.to_string())))?
                {
                BlockStatus::InChainWithState | BlockStatus::Queued => { }
                BlockStatus::Unknown | BlockStatus::InChainPruned =>
                return Ok(ImportResult::UnknownParent),
                BlockStatus::KnownBad => return Ok(ImportResult::KnownBad),
            }
            match self.block_status(&BlockId::Hash(hash)).map_err(|e|
                                                                      ConsensusError::from(ConsensusErrorKind::ClientImport(e.to_string())))?
                {
                BlockStatus::InChainWithState | BlockStatus::Queued =>
                return Ok(ImportResult::AlreadyInChain),
                BlockStatus::Unknown | BlockStatus::InChainPruned => { }
                BlockStatus::KnownBad => return Ok(ImportResult::KnownBad),
            }
            Ok(ImportResult::imported())
        }
    }
    impl <B, E, Block, RA> CurrentHeight for Client<B, E, Block, RA> where
     B: backend::Backend<Block, Blake2Hasher>,
     E: CallExecutor<Block, Blake2Hasher>, Block: BlockT<Hash = H256> {
        type
        BlockNumber
        =
        <Block::Header as HeaderT>::Number;
        fn current_height(&self) -> Self::BlockNumber {
            self.backend.blockchain().info().map(|i|
                                                     i.best_number).unwrap_or_else(|_|
                                                                                       Zero::zero())
        }
    }
    impl <B, E, Block, RA> BlockNumberToHash for Client<B, E, Block, RA> where
     B: backend::Backend<Block, Blake2Hasher>,
     E: CallExecutor<Block, Blake2Hasher>, Block: BlockT<Hash = H256> {
        type
        BlockNumber
        =
        <Block::Header as HeaderT>::Number;
        type
        Hash
        =
        Block::Hash;
        fn block_number_to_hash(&self, n: Self::BlockNumber)
         -> Option<Self::Hash> {
            self.block_hash(n).unwrap_or(None)
        }
    }
    impl <B, E, Block, RA> BlockchainEvents<Block> for Client<B, E, Block, RA>
     where E: CallExecutor<Block, Blake2Hasher>, Block: BlockT<Hash = H256> {
        /// Get block import event stream.
        fn import_notification_stream(&self) -> ImportNotifications<Block> {
            let (sink, stream) = mpsc::unbounded();
            self.import_notification_sinks.lock().push(sink);
            stream
        }
        fn finality_notification_stream(&self)
         -> FinalityNotifications<Block> {
            let (sink, stream) = mpsc::unbounded();
            self.finality_notification_sinks.lock().push(sink);
            stream
        }
        /// Get storage changes event stream.
        fn storage_changes_notification_stream(&self,
                                               filter_keys:
                                                   Option<&[StorageKey]>)
         -> error::Result<StorageEventStream<Block::Hash>> {
            Ok(self.storage_notifications.lock().listen(filter_keys))
        }
    }
    /// Implement Longest Chain Select implementation
    /// where 'longest' is defined as the highest number of blocks
    pub struct LongestChain<B, Block> {
        backend: Arc<B>,
        import_lock: Arc<Mutex<()>>,
        _phantom: PhantomData<Block>,
    }
    impl <B, Block> Clone for LongestChain<B, Block> {
        fn clone(&self) -> Self {
            let backend = self.backend.clone();
            let import_lock = self.import_lock.clone();
            LongestChain{backend, import_lock, _phantom: Default::default(),}
        }
    }
    impl <B, Block> LongestChain<B, Block> where
     B: backend::Backend<Block, Blake2Hasher>, Block: BlockT<Hash = H256> {
        /// Instantiate a new LongestChain for Backend B
        pub fn new(backend: Arc<B>, import_lock: Arc<Mutex<()>>) -> Self {
            LongestChain{backend, import_lock, _phantom: Default::default(),}
        }
        fn best_block_header(&self)
         -> error::Result<<Block as BlockT>::Header> {
            let info: ChainInfo<Block> =
                match self.backend.blockchain().info() {
                    Ok(i) => i,
                    Err(e) =>
                    return Err(error::Error::from_blockchain(Box::new(e))),
                };
            Ok(self.backend.blockchain().header(BlockId::Hash(info.best_hash))?.expect("Best block header must always exist"))
        }
        /// Get the most recent block hash of the best (longest) chains
        /// that contain block with the given `target_hash`.
        ///
        /// The search space is always limited to blocks which are in the finalized
        /// chain or descendents of it.
        ///
        /// If `maybe_max_block_number` is `Some(max_block_number)`
        /// the search is limited to block `numbers <= max_block_number`.
        /// in other words as if there were no blocks greater `max_block_number`.
        /// Returns `Ok(None)` if `target_hash` is not found in search space.
        /// TODO: document time complexity of this, see [#1444](https://github.com/paritytech/substrate/issues/1444)
        fn best_containing(&self, target_hash: Block::Hash,
                           maybe_max_number: Option<NumberFor<Block>>)
         -> error::Result<Option<Block::Hash>> {
            let target_header =
                {
                    match self.backend.blockchain().header(BlockId::Hash(target_hash))?
                        {
                        Some(x) => x,
                        None => { return Ok(None); }
                    }
                };
            if let Some(max_number) = maybe_max_number {
                if target_header.number() > &max_number { return Ok(None); }
            }
            let (leaves, best_already_checked) =
                {
                    let _import_lock = self.import_lock.lock();
                    let info = self.backend.blockchain().info()?;
                    let canon_hash =
                        self.backend.blockchain().hash(*target_header.number())?.ok_or_else(||
                                                                                                error::Error::from(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["failed to get hash for block number "],
                                                                                                                                                                      &match (&target_header.number(),)
                                                                                                                                                                           {
                                                                                                                                                                           (arg0,)
                                                                                                                                                                           =>
                                                                                                                                                                           [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                        ::std::fmt::Display::fmt)],
                                                                                                                                                                       }))))?;
                    if canon_hash == target_hash {
                        if let Some(max_number) = maybe_max_number {
                            if let Some(header) =
                                   self.backend.blockchain().hash(max_number)?
                                   {
                                return Ok(Some(header));
                            }
                        }
                        return Ok(Some(info.best_hash));
                    } else if info.finalized_number >= *target_header.number()
                     {
                        return Ok(None);
                    }
                    (self.backend.blockchain().leaves()?, info.best_hash)
                };
            for leaf_hash in leaves {
                if leaf_hash == best_already_checked { continue ; }
                let mut current_hash = leaf_hash;
                let mut best_hash = leaf_hash;
                if let Some(max_number) = maybe_max_number {
                    loop  {
                        let current_header =
                            self.backend.blockchain().header(BlockId::Hash(current_hash.clone()))?.ok_or_else(||
                                                                                                                  error::Error::from(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["failed to get header for hash "],
                                                                                                                                                                                        &match (&current_hash,)
                                                                                                                                                                                             {
                                                                                                                                                                                             (arg0,)
                                                                                                                                                                                             =>
                                                                                                                                                                                             [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                          ::std::fmt::Display::fmt)],
                                                                                                                                                                                         }))))?;
                        if current_header.number() <= &max_number {
                            best_hash = current_header.hash();
                            break ;
                        }
                        current_hash = *current_header.parent_hash();
                    }
                }
                loop  {
                    if current_hash == target_hash {
                        return Ok(Some(best_hash));
                    }
                    let current_header =
                        self.backend.blockchain().header(BlockId::Hash(current_hash.clone()))?.ok_or_else(||
                                                                                                              error::Error::from(::alloc::fmt::format(::std::fmt::Arguments::new_v1(&["failed to get header for hash "],
                                                                                                                                                                                    &match (&current_hash,)
                                                                                                                                                                                         {
                                                                                                                                                                                         (arg0,)
                                                                                                                                                                                         =>
                                                                                                                                                                                         [::std::fmt::ArgumentV1::new(arg0,
                                                                                                                                                                                                                      ::std::fmt::Display::fmt)],
                                                                                                                                                                                     }))))?;
                    if current_header.number() < target_header.number() {
                        break ;
                    }
                    current_hash = *current_header.parent_hash();
                }
            }
            {
                let lvl = ::log::Level::Warn;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level()
                   {
                    ::log::__private_api_log(::std::fmt::Arguments::new_v1(&["Block ",
                                                                             " exists in chain but not found when following all leaves backwards. Number limit = "],
                                                                           &match (&target_hash,
                                                                                   &maybe_max_number)
                                                                                {
                                                                                (arg0,
                                                                                 arg1)
                                                                                =>
                                                                                [::std::fmt::ArgumentV1::new(arg0,
                                                                                                             ::std::fmt::Debug::fmt),
                                                                                 ::std::fmt::ArgumentV1::new(arg1,
                                                                                                             ::std::fmt::Debug::fmt)],
                                                                            }),
                                             lvl,
                                             &("substrate_client::client",
                                               "substrate_client::client",
                                               "core/client/src/client.rs",
                                               1591u32));
                }
            };
            Ok(None)
        }
        fn leaves(&self)
         -> Result<Vec<<Block as BlockT>::Hash>, error::Error> {
            self.backend.blockchain().leaves()
        }
    }
    impl <B, Block> SelectChain<Block> for LongestChain<B, Block> where
     B: backend::Backend<Block, Blake2Hasher>, Block: BlockT<Hash = H256> {
        fn leaves(&self)
         -> Result<Vec<<Block as BlockT>::Hash>, ConsensusError> {
            LongestChain::leaves(self).map_err(|e|
                                                   ConsensusErrorKind::ChainLookup(e.to_string()).into())
        }
        fn best_chain(&self)
         -> Result<<Block as BlockT>::Header, ConsensusError> {
            LongestChain::best_block_header(&self).map_err(|e|
                                                               ConsensusErrorKind::ChainLookup(e.to_string()).into())
        }
        fn finality_target(&self, target_hash: Block::Hash,
                           maybe_max_number: Option<NumberFor<Block>>)
         -> Result<Option<Block::Hash>, ConsensusError> {
            LongestChain::best_containing(self, target_hash,
                                          maybe_max_number).map_err(|e|
                                                                        ConsensusErrorKind::ChainLookup(e.to_string()).into())
        }
    }
    impl <B, E, Block, RA> BlockBody<Block> for Client<B, E, Block, RA> where
     B: backend::Backend<Block, Blake2Hasher>,
     E: CallExecutor<Block, Blake2Hasher>, Block: BlockT<Hash = H256> {
        fn block_body(&self, id: &BlockId<Block>)
         -> error::Result<Option<Vec<<Block as BlockT>::Extrinsic>>> {
            self.body(id)
        }
    }
    impl <B, E, Block, RA> backend::AuxStore for Client<B, E, Block, RA> where
     B: backend::Backend<Block, Blake2Hasher>,
     E: CallExecutor<Block, Blake2Hasher>, Block: BlockT<Hash = H256> {
        /// Insert auxiliary data into key-value store.
        fn insert_aux<'a, 'b: 'a, 'c: 'a, I: IntoIterator<Item =
                      &'a (&'c [u8], &'c [u8])>, D: IntoIterator<Item =
                      &'a &'b [u8]>>(&self, insert: I, delete: D)
         -> error::Result<()> {
            self.lock_import_and_run(|operation|
                                         {
                                             self.apply_aux(operation, insert,
                                                            delete)
                                         })
        }
        /// Query auxiliary data from key-value store.
        fn get_aux(&self, key: &[u8]) -> error::Result<Option<Vec<u8>>> {
            crate::backend::AuxStore::get_aux(&*self.backend, key)
        }
    }
}
#[cfg(feature = "std")]
mod notifications {
    //! Storage notifications
    use std::{collections::{HashSet, HashMap}, sync::Arc};
    use fnv::{FnvHashSet, FnvHashMap};
    use futures::sync::mpsc;
    use primitives::storage::{StorageKey, StorageData};
    use runtime_primitives::traits::Block as BlockT;
    /// Storage change set
    pub struct StorageChangeSet {
        changes: Arc<Vec<(StorageKey, Option<StorageData>)>>,
        filter: Option<HashSet<StorageKey>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::fmt::Debug for StorageChangeSet {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                StorageChangeSet {
                changes: ref __self_0_0, filter: ref __self_0_1 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("StorageChangeSet");
                    let _ =
                        debug_trait_builder.field("changes", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("filter", &&(*__self_0_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl StorageChangeSet {
        /// Convert the change set into iterator over storage items.
        pub fn iter<'a>(&'a self)
         -> impl Iterator<Item = &'a (StorageKey, Option<StorageData>)> + 'a {
            self.changes.iter().filter(move |&(key, _)|
                                           match self.filter {
                                               Some(ref filter) =>
                                               filter.contains(key),
                                               None => true,
                                           })
        }
    }
    /// Type that implements `futures::Stream` of storage change events.
    pub type StorageEventStream<H>
        =
        mpsc::UnboundedReceiver<(H, StorageChangeSet)>;
    type SubscriberId = u64;
    /// Manages storage listeners.
    pub struct StorageNotifications<Block: BlockT> {
        next_id: SubscriberId,
        wildcard_listeners: FnvHashSet<SubscriberId>,
        listeners: HashMap<StorageKey, FnvHashSet<SubscriberId>>,
        sinks: FnvHashMap<SubscriberId,
                          (mpsc::UnboundedSender<(Block::Hash,
                                                  StorageChangeSet)>,
                           Option<HashSet<StorageKey>>)>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <Block: ::std::fmt::Debug + BlockT> ::std::fmt::Debug for
     StorageNotifications<Block> where Block::Hash: ::std::fmt::Debug {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                StorageNotifications {
                next_id: ref __self_0_0,
                wildcard_listeners: ref __self_0_1,
                listeners: ref __self_0_2,
                sinks: ref __self_0_3 } => {
                    let mut debug_trait_builder =
                        f.debug_struct("StorageNotifications");
                    let _ =
                        debug_trait_builder.field("next_id", &&(*__self_0_0));
                    let _ =
                        debug_trait_builder.field("wildcard_listeners",
                                                  &&(*__self_0_1));
                    let _ =
                        debug_trait_builder.field("listeners",
                                                  &&(*__self_0_2));
                    let _ =
                        debug_trait_builder.field("sinks", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl <Block: BlockT> Default for StorageNotifications<Block> {
        fn default() -> Self {
            StorageNotifications{next_id: Default::default(),
                                 wildcard_listeners: Default::default(),
                                 listeners: Default::default(),
                                 sinks: Default::default(),}
        }
    }
    impl <Block: BlockT> StorageNotifications<Block> {
        /// Trigger notification to all listeners.
        ///
        /// Note the changes are going to be filtered by listener's filter key.
        /// In fact no event might be sent if clients are not interested in the changes.
        pub fn trigger(&mut self, hash: &Block::Hash,
                       changeset:
                           impl Iterator<Item = (Vec<u8>, Option<Vec<u8>>)>) {
            let has_wildcard = !self.wildcard_listeners.is_empty();
            if !has_wildcard && self.listeners.is_empty() { return; }
            let mut subscribers = self.wildcard_listeners.clone();
            let mut changes = Vec::new();
            for (k, v) in changeset {
                let k = StorageKey(k);
                let listeners = self.listeners.get(&k);
                if let Some(ref listeners) = listeners {
                    subscribers.extend(listeners.iter());
                }
                if has_wildcard || listeners.is_some() {
                    changes.push((k, v.map(StorageData)));
                }
            }
            if changes.is_empty() { return; }
            let changes = Arc::new(changes);
            for subscriber in subscribers {
                let should_remove =
                    {
                        let &(ref sink, ref filter) =
                            self.sinks.get(&subscriber).expect("subscribers returned from self.listeners are always in self.sinks; qed");
                        sink.unbounded_send((hash.clone(),
                                             StorageChangeSet{changes:
                                                                  changes.clone(),
                                                              filter:
                                                                  filter.clone(),})).is_err()
                    };
                if should_remove { self.remove_subscriber(subscriber); }
            }
        }
        fn remove_subscriber(&mut self, subscriber: SubscriberId) {
            if let Some((_, filters)) = self.sinks.remove(&subscriber) {
                match filters {
                    None => { self.wildcard_listeners.remove(&subscriber); }
                    Some(filters) => {
                        for key in filters {
                            let remove_key =
                                match self.listeners.get_mut(&key) {
                                    Some(ref mut set) => {
                                        set.remove(&subscriber);
                                        set.is_empty()
                                    }
                                    None => false,
                                };
                            if remove_key { self.listeners.remove(&key); }
                        }
                    }
                }
            }
        }
        /// Start listening for particular storage keys.
        pub fn listen(&mut self, filter_keys: Option<&[StorageKey]>)
         -> StorageEventStream<Block::Hash> {
            self.next_id += 1;
            let keys =
                match filter_keys {
                    None => {
                        self.wildcard_listeners.insert(self.next_id);
                        None
                    }
                    Some(keys) =>
                    Some(keys.iter().map(|key|
                                             {
                                                 self.listeners.entry(key.clone()).or_insert_with(Default::default).insert(self.next_id);
                                                 key.clone()
                                             }).collect()),
                };
            let (tx, rx) = mpsc::unbounded();
            self.sinks.insert(self.next_id, (tx, keys));
            rx
        }
    }
}
#[cfg(feature = "std")]
pub use crate::blockchain::Info as ChainInfo;
#[cfg(feature = "std")]
pub use crate::call_executor::{CallExecutor, LocalCallExecutor};
#[cfg(feature = "std")]
pub use crate::client::{new_with_backend, new_in_mem, BlockBody, BlockStatus,
                        ImportNotifications, FinalityNotifications,
                        BlockchainEvents, BlockImportNotification, Client,
                        ClientInfo, ExecutionStrategies, LongestChain};
#[cfg(feature = "std")]
pub use crate::notifications::{StorageEventStream, StorageChangeSet};
#[cfg(feature = "std")]
pub use state_machine::{ExecutionStrategy, NeverOffchainExt};
#[cfg(feature = "std")]
pub use crate::leaves::LeafSet;
#[doc(inline)]
pub use sr_api_macros::{decl_runtime_apis, impl_runtime_apis};
