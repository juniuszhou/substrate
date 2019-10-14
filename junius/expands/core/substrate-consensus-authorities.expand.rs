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

//! Authorities API.
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;


use substrate_client::decl_runtime_apis;
use runtime_primitives::traits::AuthorityIdFor;
use rstd::vec::Vec;

#[doc(hidden)]
mod sr_api_hidden_includes_DECL_RUNTIME_APIS {
    pub extern crate substrate_client as sr_api_client;
}
#[doc(hidden)]
#[allow(dead_code)]
#[allow(deprecated)]
pub mod runtime_decl_for_AuthoritiesApi {
    use super::*;
    #[doc = " Authorities API."]
    pub trait AuthoritiesApi<Block: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockT> {
        #[doc = " Returns the authorities at the given block."]
        fn authorities()
        -> Vec<AuthorityIdFor<Block>>;
    }
    pub const VERSION: u32 = 1u32;
    pub const ID: [u8; 8] =
        [120u8, 1u8, 117u8, 153u8, 25u8, 238u8, 131u8, 229u8];
    #[cfg(any(feature = "std", test))]
    fn convert_between_block_types<I: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Encode,
                                   R: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Decode>(input:
                                                                                                                              &I,
                                                                                                                          error_desc:
                                                                                                                              &'static str)
     -> ::std::result::Result<R, &'static str> {
        <R as
            self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Decode>::decode(&mut &self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(input)[..]).ok_or_else(||
                                                                                                                                                                                                                                    error_desc)
    }
    #[cfg(any(feature = "std", test))]
    pub fn authorities_native_call_generator<'a,
                                             ApiImpl: AuthoritiesApi<Block>,
                                             NodeBlock: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockT,
                                             Block: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockT +
                                             'a>()
     ->
         impl FnOnce() ->
         ::std::result::Result<Vec<AuthorityIdFor<NodeBlock>>, &'static str> +
         'a {
        move ||
            {
                let res = ApiImpl::authorities();
                convert_between_block_types(&res,
                                            "Could not convert return value from runtime to node!")
            }
    }
    #[cfg(any(feature = "std", test))]
    pub fn authorities_call_api_at<R: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Encode +
                                   self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Decode +
                                   PartialEq, NC: FnOnce() ->
                                   ::std::result::Result<R, &'static str> +
                                   ::std::panic::UnwindSafe,
                                   Block: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockT,
                                   T: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::CallRuntimeAt<Block>,
                                   C: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Core<Block>>(call_runtime_at:
                                                                                                                                   &T,
                                                                                                                               core_api:
                                                                                                                                   &C,
                                                                                                                               at:
                                                                                                                                   &self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<Block>,
                                                                                                                               args:
                                                                                                                                   Vec<u8>,
                                                                                                                               changes:
                                                                                                                                   &std::cell::RefCell<self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::OverlayedChanges>,
                                                                                                                               initialized_block:
                                                                                                                                   &std::cell::RefCell<Option<self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<Block>>>,
                                                                                                                               native_call:
                                                                                                                                   Option<NC>,
                                                                                                                               context:
                                                                                                                                   self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                                                                                               recorder:
                                                                                                                                   &Option<std::rc::Rc<std::cell::RefCell<self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::ProofRecorder<Block>>>>)
     ->
         self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<R>> {
        let version = call_runtime_at.runtime_version_at(at)?;
        use self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::InitializeBlock;
        let initialize_block =
            if false {
                InitializeBlock::Skip
            } else { InitializeBlock::Do(&initialized_block) };
        let update_initialized_block = || ();
        let ret =
            call_runtime_at.call_api_at(core_api, at,
                                        "AuthoritiesApi_authorities", args,
                                        changes, initialize_block,
                                        native_call, context, recorder)?;
        update_initialized_block();
        Ok(ret)
    }
}
#[doc = " Authorities API."]
#[cfg(any(feature = "std", test))]
pub trait AuthoritiesApi<Block: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockT>: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Core<Block> {
    #[doc = " Returns the authorities at the given block."]
    fn authorities(&self,
                   at:
                       &self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<Block>)
     ->
         ::std::result::Result<Vec<AuthorityIdFor<Block>>,
                               self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::error::Error> {
        let runtime_api_impl_params_encoded =
            self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&());
        self.AuthoritiesApi_authorities_runtime_api_impl(at,
                                                         self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext::Other,
                                                         Some(()),
                                                         runtime_api_impl_params_encoded).and_then(|r|
                                                                                                       match r
                                                                                                           {
                                                                                                           self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded::Native(n)
                                                                                                           =>
                                                                                                           {
                                                                                                               Ok(n)
                                                                                                           }
                                                                                                           self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                           =>
                                                                                                           {
                                                                                                               <Vec<AuthorityIdFor<Block>>
                                                                                                                   as
                                                                                                                   self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                                                                           self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::error::Error::CallResultDecode("authorities").into())
                                                                                                           }
                                                                                                       })
    }
    #[doc = " Returns the authorities at the given block."]
    fn authorities_with_context(&self,
                                at:
                                    &self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<Block>,
                                context:
                                    self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext)
     ->
         ::std::result::Result<Vec<AuthorityIdFor<Block>>,
                               self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::error::Error> {
        let runtime_api_impl_params_encoded =
            self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Encode::encode(&());
        self.AuthoritiesApi_authorities_runtime_api_impl(at, context,
                                                         Some(()),
                                                         runtime_api_impl_params_encoded).and_then(|r|
                                                                                                       match r
                                                                                                           {
                                                                                                           self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded::Native(n)
                                                                                                           =>
                                                                                                           {
                                                                                                               Ok(n)
                                                                                                           }
                                                                                                           self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded::Encoded(r)
                                                                                                           =>
                                                                                                           {
                                                                                                               <Vec<AuthorityIdFor<Block>>
                                                                                                                   as
                                                                                                                   self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::Decode>::decode(&mut &r[..]).ok_or_else(||
                                                                                                                                                                                                                                           self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::error::Error::CallResultDecode("authorities").into())
                                                                                                           }
                                                                                                       })
    }
    #[doc(hidden)]
    fn AuthoritiesApi_authorities_runtime_api_impl(&self,
                                                   at:
                                                       &self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockId<Block>,
                                                   context:
                                                       self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::ExecutionContext,
                                                   params: Option<()>,
                                                   params_encoded: Vec<u8>)
    ->
        self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::error::Result<self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::NativeOrEncoded<Vec<AuthorityIdFor<Block>>>>;
}
#[cfg(any(feature = "std", test))]
impl <Block: self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::BlockT>
 self::sr_api_hidden_includes_DECL_RUNTIME_APIS::sr_api_client::runtime_api::RuntimeApiInfo
 for AuthoritiesApi<Block> {
    const
    ID:
    [u8; 8]
    =
    [120u8, 1u8, 117u8, 153u8, 25u8, 238u8, 131u8, 229u8];
    const
    VERSION:
    u32
    =
    1u32;
}
