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

//! # Example Module
//!
//! <!-- Original author of paragraph: @gavofyork --> 
//! The Example: A simple example of a runtime module demonstrating
//! concepts, APIs and structures common to most runtime modules.
//!
//! Run `cargo doc --package srml-example --open` to view this module's documentation.
//!
//! ### Documentation Guidelines:
//!
//! <!-- Original author of paragraph: Various. Based on collation of review comments to PRs addressing issues with -->
//! <!-- label 'S3-SRML' in https://github.com/paritytech/substrate-developer-hub/issues -->
//! <ul>
//!		<li>Documentation comments (i.e. <code>/// comment</code>) - should accompany module functions and be
//!         restricted to the module interface, not the internals of the module implementation. Only state inputs,
//!         outputs, and a brief description that mentions whether calling it requires root, but without repeating
//!         the source code details. Capitalise the first word of each documentation comment and end it with a full
//!         stop. See <a href="https://github.com/paritytech/substrate#72-contributing-to-documentation-for-substrate-packages"
//!         target="_blank">Generic example of annotating source code with documentation comments</a></li>
//! 	<li>Self-documenting code - Try to refactor code to be self-documenting.</li>
//!		<li>Code comments - Supplement complex code with a brief explanation, not every line of code.</li>
//!		<li>Identifiers - surround by backticks (i.e. <code>INHERENT_IDENTIFIER</code>, <code>InherentType</code>,
//!         <code>u64</code>)</li>
//!		<li>Usage scenarios - should be simple doctests. The compiler should ensure they stay valid.</li>
//!		<li>Extended tutorials - should be moved to external files and refer to.</li>
//!		<!-- Original author of paragraph: @AmarRSingh -->
//!		<li>Mandatory - include all of the sections/subsections where <b>MUST</b> is specified.</li>
//!		<li>Optional - optionally include sections/subsections where <b>CAN</b> is specified.</li>
//! </ul>
//!
//! ### Documentation Template:<br>
//!
//! Copy and paste this template from srml/example/src/lib.rs into file srml/<INSERT_CUSTOM_MODULE_NAME>/src/lib.rs of
//! your own custom module and complete it.
//! <details><p><pre>
//! // Add heading with custom module name
//!
//! \# <INSERT_CUSTOM_MODULE_NAME> Module
//!
//! // Add simple description
//!
//! // Include the following links that shows what trait needs to be implemented to use the module
//! // and the supported dispatchables that are documented in the Call enum.
//!
//! - \[`<INSERT_CUSTOM_MODULE_NAME>::Trait`](./trait.Trait.html)
//! - \[`Call`](./enum.Call.html)
//! - \[`Module`](./struct.Module.html)
//!
//! \## Overview
//!
//! <!-- Original author of paragraph: Various. See https://github.com/paritytech/substrate-developer-hub/issues/44 --> 
//! // Short description of module purpose.
//! // Links to Traits that should be implemented.
//! // What this module is for.
//! // What functionality the module provides.
//! // When to use the module (use case examples).
//! // How it is used.
//! // Inputs it uses and the source of each input.
//! // Outputs it produces.
//!
//! <!-- Original author of paragraph: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//! <!-- and comment https://github.com/paritytech/substrate-developer-hub/issues/44#issuecomment-471982710 -->
//!
//! \## Terminology
//!
//! // Add terminology used in the custom module. Include concepts, storage items, or actions that you think
//! // deserve to be noted to give context to the rest of the documentation or module usage. The author needs to
//! // use some judgment about what is included. We don't want a list of every storage item nor types - the user
//! // can go to the code for that. For example, "transfer fee" is obvious and should not be included, but
//! // "free balance" and "reserved balance" should be noted to give context to the module.
//! // Please do not link to outside resources. The reference docs should be the ultimate source of truth.
//!
//! <!-- Original author of heading: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! \## Goals
//!
//! // Add goals that the custom module is designed to achieve.
//!
//! <!-- Original author of heading: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! \### Scenarios
//!
//! <!-- Original author of paragraph: @Kianenigma. Based on PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! \#### <INSERT_SCENARIO_NAME>
//!
//! // Describe requirements prior to interacting with the custom module.
//! // Describe the process of interacting with the custom module for this scenario and public API functions used.
//!
//! \## Interface
//!
//! \### Supported Origins
//!
//! // What origins are used and supported in this module (root, signed, none)
//! // i.e. root when <code>\`ensure_root\`</code> used
//! // i.e. none when <code>\`ensure_none\`</code> used
//! // i.e. signed when <code>\`ensure_signed\`</code> used
//!
//! <code>\`inherent\`</code> <INSERT_DESCRIPTION>
//!
//! <!-- Original author of paragraph: @Kianenigma in comment -->
//! <!-- https://github.com/paritytech/substrate-developer-hub/issues/44#issuecomment-471982710 -->
//!
//! \### Types
//!
//! // Type aliases. Include any associated types and where the user would typically define them.
//!
//! <code>\`ExampleType\`</code> <INSERT_DESCRIPTION>
//!
//! <!-- Original author of paragraph: ??? -->
//!
//! // Reference documentation of aspects such as `storageItems` and `dispatchable` functions should only be
//! // included in the https://docs.rs Rustdocs for Substrate and not repeated in the README file.
//!
//! \### Dispatchable Functions
//!
//! <!-- Original author of paragraph: @AmarRSingh & @joepetrowski -->
//!
//! // A brief description of dispatchable functions and a link to the rustdoc with their actual documentation.
//!
//! // <b>MUST</b> have link to Call enum
//! // <b>MUST</b> have origin information included in function doc
//! // <b>CAN</b> have more info up to the user
//!
//! \### Public Functions
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! // A link to the rustdoc and any notes about usage in the module, not for specific functions.
//! // For example, in the balances module: "Note that when using the publicly exposed functions,
//! // you (the runtime developer) are responsible for implementing any necessary checks
//! // (e.g. that the sender is the signer) before calling a function that will affect storage."
//!
//! <!-- Original author of paragraph: @AmarRSingh -->
//!
//! // It is up to the writer of the respective module (with respect to how much information to provide).
//!
//! \#### Public Inspection functions - Immutable (getters)
//!
//! // Insert a subheading for each getter function signature
//!
//! \##### <code>\`example_getter_name()\`</code>
//!
//! // What it returns
//! // Why, when, and how often to call it
//! // When it could panic or error
//! // When safety issues to consider
//!
//! \#### Public Mutable functions (changing state)
//!
//! // Insert a subheading for each setter function signature
//!
//! \##### <code>\`example_setter_name(origin, parameter_name: T::ExampleType)\`</code>
//!
//! // What state it changes
//! // Why, when, and how often to call it
//! // When it could panic or error
//! // When safety issues to consider
//! // What parameter values are valid and why
//!
//! \### Storage Items
//!
//! // Explain any storage items included in this module
//!
//! \### Digest Items
//!
//! // Explain any digest items included in this module
//!
//! \### Inherent Data
//!
//! // Explain what inherent data (if any) is defined in the module and any other related types
//!
//! \### Events:
//!
//! // Insert events for this module if any
//!
//! \### Errors:
//!
//! // Explain what generates errors
//!
//! \## Usage
//!
//! // Insert 2-3 examples of usage and code snippets that show how to use <INSERT_CUSTOM_MODULE_NAME> module in a custom module.
//!
//! \### Prerequisites
//!
//! // Show how to include necessary imports for <INSERT_CUSTOM_MODULE_NAME> and derive
//! // your module configuration trait with the `INSERT_CUSTOM_MODULE_NAME` trait.
//!
//! \```rust
//! use <INSERT_CUSTOM_MODULE_NAME>;
//! 
//! pub trait Trait: <INSERT_CUSTOM_MODULE_NAME>::Trait { }
//! \```
//!
//! \### Simple Code Snippet
//!
//! // Show a simple example (e.g. how to query a public getter function of <INSERT_CUSTOM_MODULE_NAME>)
//!
//! \### Example from SRML
//!
//! // Show a usage example in an actual runtime
//!
//! // See:
//! // - Substrate TCR https://github.com/parity-samples/substrate-tcr
//! // - Substrate Kitties https://shawntabrizi.github.io/substrate-collectables-workshop/#/
//!
//! \## Genesis Config
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! \## Dependencies
//!
//! // Dependencies on other SRML modules and the genesis config should be mentioned,
//! // but not the Rust Standard Library.
//! // Genesis configuration modifications that may be made to incorporate this module
//! // Interaction with other modules
//!
//! <!-- Original author of heading: @AmarRSingh -->
//!
//! \## Related Modules
//!
//! // Interaction with other modules in the form of a bullet point list
//!
//! \## References
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! // Links to reference material, if applicable. For example, Phragmen, W3F research, etc.
//! // that the implementation is based on.
//! </pre></p></details>
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

// Ensure we're `no_std` when compiling for Wasm.

use srml_support::{StorageValue, dispatch::Result, decl_module, decl_storage,
                   decl_event};
use system::ensure_signed;

/// Our module's configuration trait. All our types and consts go in here. If the
/// module is dependent on specific other modules, then their configuration traits
/// should be added to our implied traits list.
///
/// `system::Trait` should always be included in our implied traits.
pub trait Trait: balances::Trait {
    /// The overarching event type.
    type
    Event: From<Event<Self>> +
    Into<<Self as system::Trait>::Event>;
}

#[doc(hidden)]
mod sr_api_hidden_includes_decl_storage {
    pub extern crate srml_support as hidden_include;
    // A macro for the Storage trait, and its implementation, for this module.
    // This allows for type-safe usage of the Substrate storage database, so you can
    // keep things around between blocks.
    // Any storage declarations of the form:
    //   `pub? Name get(getter_name)? [config()|config(myname)] [build(|_| {...})] : <type> (= <new_default_value>)?;`
    // where `<type>` is either:
    //   - `Type` (a basic value item); or
    //   - `map KeyType => ValueType` (a map item).
    //
    // Note that there are two optional modifiers for the storage type declaration.
    // - `Foo: Option<u32>`:
    //   - `Foo::put(1); Foo::get()` returns `Some(1)`;
    //   - `Foo::kill(); Foo::get()` returns `None`.
    // - `Foo: u32`:
    //   - `Foo::put(1); Foo::get()` returns `1`;
    //   - `Foo::kill(); Foo::get()` returns `0` (u32::default()).
    // e.g. Foo: u32;
    // e.g. pub Bar get(bar): map T::AccountId => Vec<(T::Balance, u64)>;
    //
    // For basic value items, you'll get a type which implements
    // `support::StorageValue`. For map items, you'll get a type which
    // implements `support::StorageMap`.
    //
    // If they have a getter (`get(getter_name)`), then your module will come
    // equipped with `fn getter_name() -> Type` for basic value items or
    // `fn getter_name(key: KeyType) -> ValueType` for map items.

    // A map that has enumerable entries.

    // this one uses the default, we'll demonstrate the usage of 'mutate' API.
}
struct Dummy<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>
 for Dummy<T> {
    type
    Query
    =
    Option<T::Balance>;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Example Dummy".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).or_else(||
                                                                                                                                                             Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(storage);
        let ret = f(&mut val);
        match val {
            Some(ref val) =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&val,
                                                                                                                                      storage),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::kill(storage),
        };
        ret
    }
}
#[doc = r" Linkage data of an element (it's successor and predecessor)"]
pub(crate) struct __LinkageForBarDoNotUse<Key> {
    #[doc = r" Previous element key in storage (None for the first element)"]
    pub previous: Option<Key>,
    #[doc = r" Next element key in storage (None for the last element)"]
    pub next: Option<Key>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR___LinkageForBarDoNotUse: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Key> _parity_codec::Encode for __LinkageForBarDoNotUse<Key>
         where Option<Key>: _parity_codec::Encode,
         Option<Key>: _parity_codec::Encode,
         Option<Key>: _parity_codec::Encode,
         Option<Key>: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                dest.push(&self.previous);
                dest.push(&self.next);
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR___LinkageForBarDoNotUse: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <Key> _parity_codec::Decode for __LinkageForBarDoNotUse<Key>
         where Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode,
         Option<Key>: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                Some(__LinkageForBarDoNotUse{previous:
                                                 _parity_codec::Decode::decode(input)?,
                                             next:
                                                 _parity_codec::Decode::decode(input)?,})
            }
        }
    };
mod __linked_map_details_for_bar_do_not_use {
    use super::*;
    #[doc =
          r" Re-exported version of linkage to overcome proc-macro derivation issue."]
    pub(crate) use super::__LinkageForBarDoNotUse as Linkage;
    impl <Key> Default for Linkage<Key> {
        fn default() -> Self { Self{previous: None, next: None,} }
    }
    #[doc = r" A key-value pair iterator for enumerable map."]
    pub(crate) struct Enumerator<'a, S, K, V> {
        pub storage: &'a S,
        pub next: Option<K>,
        pub _data: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<V>,
    }
    impl <'a,
          S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>,
          T: Trait> Iterator for
     Enumerator<'a, S, T::AccountId, (T::Balance, T)> where T: 'a {
        type
        Item
        =
        (T::AccountId, T::Balance);
        fn next(&mut self) -> Option<Self::Item> {
            let next = self.next.take()?;
            let key_for =
                <super::Bar<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      T::Balance>>::key_for(&next);
            let (val, linkage): (T::Balance, Linkage<T::AccountId>) =
                self.storage.get(&*key_for).expect("previous/next only contain existing entires; we enumerate using next; entry exists; qed");
            self.next = linkage.next;
            Some((next, val))
        }
    }
    pub(crate) trait Utils<T: Trait> {
        #[doc = r" Update linkage when this element is removed."]
        #[doc = r""]
        #[doc = r" Takes care of updating previous and next elements points"]
        #[doc = r" as well as updates head if the element is first or last."]
        fn remove_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(linkage:
                                                                                                                                                                                  Linkage<T::AccountId>,
                                                                                                                                                                              storage:
                                                                                                                                                                                  &S);
        #[doc = r" Read the contained data and it's linkage."]
        fn read_with_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                     &S,
                                                                                                                                                                                 key:
                                                                                                                                                                                     &[u8])
        -> Option<(T::Balance, Linkage<T::AccountId>)>;
        #[doc = r" Generate linkage for newly inserted element."]
        #[doc = r""]
        #[doc = r" Takes care of updating head and previous head's pointer."]
        fn new_head_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                    &S,
                                                                                                                                                                                key:
                                                                                                                                                                                    &T::AccountId)
        -> Linkage<T::AccountId>;
        #[doc = r" Read current head pointer."]
        fn read_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                             &S)
        -> Option<T::AccountId>;
        #[doc = r" Overwrite current head pointer."]
        #[doc = r""]
        #[doc = r" If `None` is given head is removed from storage."]
        fn write_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                              &S,
                                                                                                                                                                          head:
                                                                                                                                                                              Option<&T::AccountId>);
    }
}
struct Bar<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait> self::__linked_map_details_for_bar_do_not_use::Utils<T> for
 Bar<T> {
    fn remove_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(linkage:
                                                                                                                                                                              self::__linked_map_details_for_bar_do_not_use::Linkage<T::AccountId>,
                                                                                                                                                                          storage:
                                                                                                                                                                              &S) {
        use self::__linked_map_details_for_bar_do_not_use::Utils;
        let next_key =
            linkage.next.as_ref().map(|x|
                                          <Self as
                                              self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                                T::Balance>>::key_for(x));
        let prev_key =
            linkage.previous.as_ref().map(|x|
                                              <Self as
                                                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                                    T::Balance>>::key_for(x));
        if let Some(prev_key) = prev_key {
            let mut res =
                Self::read_with_linkage(storage,
                                        &*prev_key).expect("Linkage is updated in case entry is removed; it always points to existing keys; qed");
            res.1.next = linkage.next;
            storage.put(&*prev_key, &res);
        } else { Self::write_head(storage, linkage.next.as_ref()); }
        if let Some(next_key) = next_key {
            let mut res =
                Self::read_with_linkage(storage,
                                        &*next_key).expect("Linkage is updated in case entry is removed; it always points to existing keys; qed");
            res.1.previous = linkage.previous;
            storage.put(&*next_key, &res);
        }
    }
    fn read_with_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                 &S,
                                                                                                                                                                             key:
                                                                                                                                                                                 &[u8])
     ->
         Option<(T::Balance,
                 self::__linked_map_details_for_bar_do_not_use::Linkage<T::AccountId>)> {
        storage.get(key)
    }
    fn new_head_linkage<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                                &S,
                                                                                                                                                                            key:
                                                                                                                                                                                &T::AccountId)
     -> self::__linked_map_details_for_bar_do_not_use::Linkage<T::AccountId> {
        use self::__linked_map_details_for_bar_do_not_use::Utils;
        if let Some(head) = Self::read_head(storage) {
            {
                let head_key =
                    <Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                          T::Balance>>::key_for(&head);
                let (data, linkage) =
                    Self::read_with_linkage(storage,
                                            &*head_key).expect(r#"
								head is set when first element is inserted and unset when last element is removed;
								if head is Some then it points to existing key; qed
							"#);
                storage.put(&*head_key,
                            &(data,
                              self::__linked_map_details_for_bar_do_not_use::Linkage{next:
                                                                                         linkage.next.as_ref(),
                                                                                     previous:
                                                                                         Some(key),}));
            }
            Self::write_head(storage, Some(key));
            let mut linkage =
                self::__linked_map_details_for_bar_do_not_use::Linkage::default();
            linkage.next = Some(head);
            linkage
        } else {
            Self::write_head(storage, Some(key));
            self::__linked_map_details_for_bar_do_not_use::Linkage::default()
        }
    }
    fn read_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                         &S)
     -> Option<T::AccountId> {
        storage.get("head of Example Bar".as_bytes())
    }
    fn write_head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                          &S,
                                                                                                                                                                      head:
                                                                                                                                                                          Option<&T::AccountId>) {
        match head {
            Some(head) => storage.put("head of Example Bar".as_bytes(), head),
            None => storage.kill("head of Example Bar".as_bytes()),
        }
    }
}
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                   T::Balance>
 for Bar<T> {
    type
    Query
    =
    T::Balance;
    type
    Hasher
    =
    self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256;
    #[doc = r" Get the prefix key in storage."]
    fn prefix() -> &'static [u8] { "Example Bar".as_bytes() }
    #[doc =
          r" Get the storage key used to fetch a value corresponding to a specific key."]
    fn key_for(key: &T::AccountId)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        let mut key_for =
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::Balance>>::prefix().to_vec();
        self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode::encode_to(&key,
                                                                                            &mut key_for);
        key_for
    }
    #[doc = r" Load the value associated with the given key from the map."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                   &T::AccountId,
                                                                                                                                                               storage:
                                                                                                                                                                   &S)
     -> Self::Query {
        storage.get(&*<Self as
                          self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                            T::Balance>>::key_for(key)).unwrap_or_else(||
                                                                                                                                                                           Default::default())
    }
    #[doc = r" Take the value, reading and removing it."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                    &T::AccountId,
                                                                                                                                                                storage:
                                                                                                                                                                    &S)
     -> Self::Query {
        use self::__linked_map_details_for_bar_do_not_use::Utils;
        let res:
                Option<(T::Balance,
                        self::__linked_map_details_for_bar_do_not_use::Linkage<T::AccountId>)> =
            storage.take(&*<Self as
                               self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                                 T::Balance>>::key_for(key));
        match res {
            Some((data, linkage)) => {
                Self::remove_linkage(linkage, storage);
                data
            }
            None => Default::default(),
        }
    }
    #[doc = r" Remove the value under a key."]
    fn remove<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::AccountId,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S) {
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::Balance>>::take(key,
                                                                                                                                 storage);
    }
    #[doc =
          r" Store a value to be associated with the given key from the map."]
    fn insert<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::AccountId,
                                                                                                                                                                  val:
                                                                                                                                                                      &T::Balance,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S) {
        use self::__linked_map_details_for_bar_do_not_use::Utils;
        let key_for =
            &*<Self as
                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                    T::Balance>>::key_for(key);
        let linkage =
            match Self::read_with_linkage(storage, key_for) {
                Some((_data, linkage)) => linkage,
                None => Self::new_head_linkage(storage, key),
            };
        storage.put(key_for, &(val, linkage))
    }
    #[doc = r" Mutate the value under a key"]
    fn mutate<R, F: FnOnce(&mut Self::Query) -> R,
              S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(key:
                                                                                                                                                                      &T::AccountId,
                                                                                                                                                                  f:
                                                                                                                                                                      F,
                                                                                                                                                                  storage:
                                                                                                                                                                      &S)
     -> R {
        use self::__linked_map_details_for_bar_do_not_use::Utils;
        let key_for =
            &*<Self as
                  self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                    T::Balance>>::key_for(key);
        let (mut val, linkage) =
            Self::read_with_linkage(storage,
                                    key_for).map(|(data, linkage)|
                                                     (data,
                                                      Some(linkage))).unwrap_or_else(||
                                                                                         (Default::default(),
                                                                                          None));
        let ret = f(&mut val);
        match linkage {
            Some(linkage) => storage.put(key_for, &(val, linkage)),
            None =>
            <Self as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                  T::Balance>>::insert(key,
                                                                                                                                       &val,
                                                                                                                                       storage),
        };
        ret
    }
}
impl <T: 'static + Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::EnumerableStorageMap<T::AccountId,
                                                                                                             T::Balance>
 for Bar<T> {
    fn head<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                    &S)
     -> Option<T::AccountId> {
        use self::__linked_map_details_for_bar_do_not_use::Utils;
        Self::read_head(storage)
    }
    fn enumerate<'a,
                 S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Blake2_256>>(storage:
                                                                                                                                                                         &'a S)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::boxed::Box<dyn Iterator<Item
                                                                                     =
                                                                                     (T::AccountId,
                                                                                      T::Balance)> +
                                                                                     'a>
     where T::AccountId: 'a, T::Balance: 'a {
        use self::__linked_map_details_for_bar_do_not_use::{Utils,
                                                            Enumerator};
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::boxed::Box::new(Enumerator{next:
                                                                                                        Self::read_head(storage),
                                                                                                    storage,
                                                                                                    _data:
                                                                                                        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData::<(T::Balance,
                                                                                                                                                                                                T)>::default(),})
    }
}
struct Foo<T: Trait>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>
 for Foo<T> {
    type
    Query
    =
    T::Balance;
    #[doc = r" Get the storage key."]
    fn key() -> &'static [u8] { "Example Foo".as_bytes() }
    #[doc = r" Load the value from the provided storage instance."]
    fn get<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                &S)
     -> Self::Query {
        storage.get(<Self as
                        self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
                                                                                                                                                                    Default::default())
    }
    #[doc = r" Take a value from storage, removing it afterwards."]
    fn take<S: self::sr_api_hidden_includes_decl_storage::hidden_include::HashedStorage<self::sr_api_hidden_includes_decl_storage::hidden_include::Twox128>>(storage:
                                                                                                                                                                 &S)
     -> Self::Query {
        storage.take(<Self as
                         self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::key()).unwrap_or_else(||
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
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(storage);
        let ret = f(&mut val);
        <Self as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&val,
                                                                                                                                  storage);
        ret
    }
}
trait Store {
    type
    Dummy;
    type
    Bar;
    type
    Foo;
}
#[doc(hidden)]
pub struct __GetByteStructDummy<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Dummy:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructDummy<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Dummy.get_or_init(||
                                                      {
                                                          let def_val:
                                                                  Option<T::Balance> =
                                                              Default::default();
                                                          <Option<T::Balance>
                                                              as
                                                              Encode>::encode(&def_val)
                                                      }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructBar<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Bar:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructBar<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Bar.get_or_init(||
                                                    {
                                                        let def_val:
                                                                T::Balance =
                                                            Default::default();
                                                        <T::Balance as
                                                            Encode>::encode(&def_val)
                                                    }).clone()
    }
}
#[doc(hidden)]
pub struct __GetByteStructFoo<T>(pub self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Foo:
       self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>>
       =
    self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
#[cfg(feature = "std")]
impl <T: Trait>
 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
 for __GetByteStructFoo<T> {
    fn default_byte(&self)
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
        use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Foo.get_or_init(||
                                                    {
                                                        let def_val:
                                                                T::Balance =
                                                            Default::default();
                                                        <T::Balance as
                                                            Encode>::encode(&def_val)
                                                    }).clone()
    }
}
impl <T: Trait> Store for Module<T> {
    type
    Dummy
    =
    Dummy<T>;
    type
    Bar
    =
    Bar<T>;
    type
    Foo
    =
    Foo<T>;
}
impl <T: 'static + Trait> Module<T> {
    pub fn dummy() -> Option<T::Balance> {
        <Dummy<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    pub fn bar<K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<T::AccountId>>(key:
                                                                                                                     K)
     -> T::Balance {
        <Bar<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                              T::Balance>>::get(key.borrow(),
                                                                                                                                &self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    pub fn foo() -> T::Balance {
        <Foo<T> as
            self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::get(&self::sr_api_hidden_includes_decl_storage::hidden_include::storage::RuntimeStorage)
    }
    #[doc(hidden)]
    pub fn store_metadata()
     ->
         self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata{functions:
                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode({
                                                                                                                                                                                                  &[self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Dummy"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDummy::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Bar"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                                                                                                                                                                                                                   key:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                                                                                                                                                                                                                   value:
                                                                                                                                                                                                                                                                                                                                                                                                       self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                                                                                                                                                                                                                   is_linked:
                                                                                                                                                                                                                                                                                                                                                                                                       true,},
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBar::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                                                                                                                                                                                                                 documentation:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
                                                                                                                                                                                                    self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Foo"),
                                                                                                                                                                                                                                                                                                 modifier:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                                                                                                                                                                                                                 ty:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                                                                                                                                                                                                                 default:
                                                                                                                                                                                                                                                                                                     self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructFoo::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
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
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Dummy"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Optional,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructDummy::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Bar"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Map{hasher:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageHasher::Blake2_256,
                                                                                                                                                                                                             key:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::AccountId"),
                                                                                                                                                                                                             value:
                                                                                                                                                                                                                 self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance"),
                                                                                                                                                                                                             is_linked:
                                                                                                                                                                                                                 true,},
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructBar::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),},
              self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionMetadata{name:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("Foo"),
                                                                                                           modifier:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionModifier::Default,
                                                                                                           ty:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageFunctionType::Plain(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode("T::Balance")),
                                                                                                           default:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByteGetter(&__GetByteStructFoo::<T>(self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData))),
                                                                                                           documentation:
                                                                                                               self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DecodeDifferent::Encode(&[]),}]
        }
    }
    #[doc(hidden)]
    pub fn store_metadata_name() -> &'static str { "Example" }
}
#[cfg(feature = "std")]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(bound(serialize =
                  "Option < T :: Balance > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: AccountId : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "))]
#[serde(bound(deserialize =
                  "Option < T :: Balance > : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: AccountId : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, T :: Balance : self :: sr_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "))]
pub struct GenesisConfig<T: Trait> {
    pub dummy: T::Balance,
    pub bar: Vec<(T::AccountId, T::Balance)>,
    pub foo: T::Balance,
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
         Option<T::Balance>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::AccountId: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::Serialize
         {
            fn serialize<__S>(&self, __serializer: __S)
             -> _serde::export::Result<__S::Ok, __S::Error> where
             __S: _serde::Serializer {
                let mut __serde_state =
                    match _serde::Serializer::serialize_struct(__serializer,
                                                               "GenesisConfig",
                                                               false as usize
                                                                   + 1 + 1 +
                                                                   1) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "dummy",
                                                                    &self.dummy)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "bar",
                                                                    &self.bar)
                    {
                    _serde::export::Ok(__val) => __val,
                    _serde::export::Err(__err) => {
                        return _serde::export::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state,
                                                                    "foo",
                                                                    &self.foo)
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
         Option<T::Balance>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::AccountId: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
         T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
         {
            fn deserialize<__D>(__deserializer: __D)
             -> _serde::export::Result<Self, __D::Error> where
             __D: _serde::Deserializer<'de> {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, __field2, }
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
                            2u64 => _serde::export::Ok(__Field::__field2),
                            _ =>
                            _serde::export::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value),
                                                                                 &"field index 0 <= i < 3")),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str)
                     -> _serde::export::Result<Self::Value, __E> where
                     __E: _serde::de::Error {
                        match __value {
                            "dummy" => _serde::export::Ok(__Field::__field0),
                            "bar" => _serde::export::Ok(__Field::__field1),
                            "foo" => _serde::export::Ok(__Field::__field2),
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
                            b"dummy" => _serde::export::Ok(__Field::__field0),
                            b"bar" => _serde::export::Ok(__Field::__field1),
                            b"foo" => _serde::export::Ok(__Field::__field2),
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
                       Option<T::Balance>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::AccountId: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                       T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned {
                    marker: _serde::export::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl <'de, T: Trait> _serde::de::Visitor<'de> for
                 __Visitor<'de, T> where
                 Option<T::Balance>: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::AccountId: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned,
                 T::Balance: self::sr_api_hidden_includes_decl_storage::hidden_include::serde::de::DeserializeOwned
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
                            match match _serde::de::SeqAccess::next_element::<T::Balance>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(0usize,
                                                                                                 &"struct GenesisConfig with 3 elements"));
                                }
                            };
                        let __field1 =
                            match match _serde::de::SeqAccess::next_element::<Vec<(T::AccountId,
                                                                                   T::Balance)>>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(1usize,
                                                                                                 &"struct GenesisConfig with 3 elements"));
                                }
                            };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<T::Balance>(&mut __seq)
                                      {
                                      _serde::export::Ok(__val) => __val,
                                      _serde::export::Err(__err) => {
                                          return _serde::export::Err(__err);
                                      }
                                  } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(2usize,
                                                                                                 &"struct GenesisConfig with 3 elements"));
                                }
                            };
                        _serde::export::Ok(GenesisConfig{dummy: __field0,
                                                         bar: __field1,
                                                         foo: __field2,})
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A)
                     -> _serde::export::Result<Self::Value, __A::Error> where
                     __A: _serde::de::MapAccess<'de> {
                        let mut __field0: _serde::export::Option<T::Balance> =
                            _serde::export::None;
                        let mut __field1:
                                _serde::export::Option<Vec<(T::AccountId,
                                                            T::Balance)>> =
                            _serde::export::None;
                        let mut __field2: _serde::export::Option<T::Balance> =
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
                                                                       _serde::de::Error>::duplicate_field("dummy"));
                                    }
                                    __field0 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Balance>(&mut __map)
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
                                                                       _serde::de::Error>::duplicate_field("bar"));
                                    }
                                    __field1 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<Vec<(T::AccountId,
                                                                                                            T::Balance)>>(&mut __map)
                                                                 {
                                                                 _serde::export::Ok(__val)
                                                                 => __val,
                                                                 _serde::export::Err(__err)
                                                                 => {
                                                                     return _serde::export::Err(__err);
                                                                 }
                                                             });
                                }
                                __Field::__field2 => {
                                    if _serde::export::Option::is_some(&__field2)
                                       {
                                        return _serde::export::Err(<__A::Error
                                                                       as
                                                                       _serde::de::Error>::duplicate_field("foo"));
                                    }
                                    __field2 =
                                        _serde::export::Some(match _serde::de::MapAccess::next_value::<T::Balance>(&mut __map)
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
                                match _serde::private::de::missing_field("dummy")
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
                                match _serde::private::de::missing_field("bar")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        let __field2 =
                            match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None =>
                                match _serde::private::de::missing_field("foo")
                                    {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                            };
                        _serde::export::Ok(GenesisConfig{dummy: __field0,
                                                         bar: __field1,
                                                         foo: __field2,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["dummy", "bar", "foo"];
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
        GenesisConfig{dummy: Default::default(),
                      bar: Default::default(),
                      foo: Default::default(),}
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
                ((|config: &GenesisConfig<T>| config.dummy.clone()))(&self);
            <Dummy<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&v,
                                                                                                                                      &storage);
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let data =
                ((|config: &GenesisConfig<T>| config.bar.clone()))(&self);
            for (k, v) in data.into_iter() {
                <Bar<T> as
                    self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageMap<T::AccountId,
                                                                                                                      T::Balance>>::insert(&k,
                                                                                                                                           &v,
                                                                                                                                           &storage);
            }
        }
        {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::{cell::RefCell,
                                                                                  marker::PhantomData};
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::{Encode,
                                                                                   Decode};
            let v = ((|config: &GenesisConfig<T>| config.foo.clone()))(&self);
            <Foo<T> as
                self::sr_api_hidden_includes_decl_storage::hidden_include::storage::hashed::generator::StorageValue<T::Balance>>::put(&v,
                                                                                                                                      &storage);
        }
        let r = storage.into_inner();
        (|_, _, _| { })(r, c, &self);
        Ok(())
    }
}

// Just a normal `enum`, here's a dummy event to ensure it compiles.

// The module declaration. This states the entry points that we handle. The
// macro takes care of the marshalling of arguments and dispatch.
//
// Anyone can have these functions execute by signing and submitting
// an extrinsic. Ensure that calls into each of these execute in a time, memory and
// using storage space proportional to any costs paid for by the caller or otherwise the
// difficulty of forcing the call to happen.
//
// Generally you'll want to split these into three groups:
// - Public calls that are signed by an external account.
// - Root calls that are allowed to be made only by the governance system.
// - Unsigned calls that can be of two kinds:
//   * "Inherent extrinsics" that are opinions generally held by the block authors that build child blocks.
//   * Unsigned Transactions that are of intrinsic recognisable utility to the network, and are validated by the runtime.
//
// Information about where this dispatch initiated from is provided as the first argument
// "origin". As such functions must always look like:
//
// `fn foo(origin, bar: Bar, baz: Baz) -> Result;`
//
// The `Result` is required as part of the syntax (and expands to the conventional dispatch
// result of `Result<(), &'static str>`).
//
// When you come to `impl` them later in the module, you must specify the full type for `origin`:
//
// `fn foo(origin: T::Origin, bar: Bar, baz: Baz) { ... }`
//
// There are three entries in the `system::Origin` enum that correspond
// to the above bullets: `::Signed(AccountId)`, `::Root` and `::None`. You should always match
// against them as the first thing you do in your function. There are three convenience calls
// in system that do the matching for you and return a convenient result: `ensure_signed`,
// `ensure_root` and `ensure_none`.
// Simple declaration of the `Module` type. Lets the macro know what its working on.
// This just increases the value of `Dummy` by `increase_by`.
//
// Since this is a dispatched function there are two extremely important things to
// remember:
//
// - MUST NOT PANIC: Under no circumstances (save, perhaps, storage getting into an
// irreparably damaged state) must this function panic.
// - NO SIDE-EFFECTS ON ERROR: This function must either complete totally (and return
// `Ok(())` or it must have no side-effects on storage and return `Err('Some reason')`.
//
// The first is relatively easy to audit for - just ensure all panickers are removed from
// logic that executes in production (which you do anyway, right?!). To ensure the second
// is followed, you should do all tests for validity at the top of your function. This
// is stuff like checking the sender (`origin`) or that state is such that the operation
// makes sense.
//
// Once you've determined that it's all good, then enact the operation and change storage.
// If you can't be certain that the operation will succeed without substantial computation
// then you have a classic blockchain attack scenario. The normal way of managing this is
// to attach a bond to the operation. As the first major alteration of storage, reserve
// some value from the sender's account (`Balances` module has a `reserve` function for
// exactly this scenario). This amount should be enough to cover any costs of the
// substantial execution in case it turns out that you can't proceed with the operation.
//
// If it eventually transpires that the operation is fine and, therefore, that the
// expense of the checks should be borne by the network, then you can refund the reserved
// deposit. If, however, the operation turns out to be invalid and the computation is
// wasted, then you can burn it or repatriate elsewhere.
//
// Security bonds ensure that attackers can't game it by ensuring that anyone interacting
// with the system either progresses it or pays for the trouble of faffing around with
// no progress.
//
// If you don't respect these rules, it is likely that your chain will be attackable.
// This is a public call, so we ensure that the origin is some signed account.

// Read the value of dummy from storage.
// let dummy = Self::dummy();
// Will also work using the `::get` on the storage item type itself:
// let dummy = <Dummy<T>>::get();

// Calculate the new value.
// let new_dummy = dummy.map_or(increase_by, |dummy| dummy + increase_by);

// Put the new value into storage.
// <Dummy<T>>::put(new_dummy);
// Will also work with a reference:
// <Dummy<T>>::put(&new_dummy);

// Here's the new one of read and then modify the value.

// Let's deposit an event to let the outside world know this happened.

// All good.

// Implementation of a privileged call. This doesn't have an `origin` parameter because
// it's not (directly) from an extrinsic, but rather the system as a whole has decided
// to execute it. Different runtimes have different reasons for allow privileged
// calls to be executed - we don't need to care why. Because it's privileged, we can
// assume it's a one-off operation and substantial processing/storage/memory can be used
// without worrying about gameability or attack scenarios.
// If you not specify `Result` explicitly as return value, it will be added automatically
// for you and `Ok(())` will be returned.
// Put the new value into storage.

// The signature could also look like: `fn on_initialize()`
// Anything that needs to be done at the start of the block.
// We don't do anything here.

// The signature could also look like: `fn on_finalize()`
// Anything that needs to be done at the end of the block.
// We just kill our dummy storage item.

// A runtime code run after every block and have access to extended set of APIs.
//
// For instance you can generate extrinsics for the upcoming produced block.
// We don't do anything here.
// but we could dispatch extrinsic (transaction/unsigned/inherent) using
// runtime_io::submit_extrinsic

// The main implementation block for the module. Functions here fall into three broad
// categories:
// - Public interface. These are functions that are `pub` and generally fall into inspector
// functions that do not write to storage and operation functions that do.
// - Private functions. These are your usual private utilities unavailable to other modules.
// Add public immutables and private mutables.

// Because Foo has 'default', the type of 'foo' in closure is the raw type instead of an Option<> type.



// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are requried.


// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
// We use default for brevity, but you can configure as desired if needed.
// we configure the map with (key, value) pairs.

// Check that GenesisBuilder works properly.

// Check that accumulate works when we have Some value in Dummy already.

// Check that finalizing the block removes Dummy from storage.

// Check that accumulate works when we Dummy has None in it.

/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T> = RawEvent<<T as balances::Trait>::Balance>;
/// Events for this module.
///
#[doc = r" Events are a simple means of reporting specific conditions and"]
#[doc =
      r" circumstances that have happened that users, Dapps and/or chain explorers would find"]
#[doc = r" interesting and otherwise difficult to detect."]
#[structural_match]
pub enum RawEvent<B> {

    #[doc = r" Dummy event, just here so there's a generic type that's used."]
    Dummy(B),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <B: ::std::clone::Clone> ::std::clone::Clone for RawEvent<B> {
    #[inline]
    fn clone(&self) -> RawEvent<B> {
        match (&*self,) {
            (&RawEvent::Dummy(ref __self_0),) =>
            RawEvent::Dummy(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <B: ::std::cmp::PartialEq> ::std::cmp::PartialEq for RawEvent<B> {
    #[inline]
    fn eq(&self, other: &RawEvent<B>) -> bool {
        match (&*self, &*other) {
            (&RawEvent::Dummy(ref __self_0), &RawEvent::Dummy(ref __arg_1_0))
            => (*__self_0) == (*__arg_1_0),
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<B>) -> bool {
        match (&*self, &*other) {
            (&RawEvent::Dummy(ref __self_0), &RawEvent::Dummy(ref __arg_1_0))
            => (*__self_0) != (*__arg_1_0),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl <B: ::std::cmp::Eq> ::std::cmp::Eq for RawEvent<B> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        { let _: ::std::cmp::AssertParamIsEq<B>; }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_RawEvent: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <B> _parity_codec::Encode for RawEvent<B> where
         B: _parity_codec::Encode, B: _parity_codec::Encode {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    RawEvent::Dummy(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    _ => (),
                }
            }
        }
    };
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_RawEvent: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <B> _parity_codec::Decode for RawEvent<B> where
         B: _parity_codec::Decode, B: _parity_codec::Decode {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(RawEvent::Dummy(_parity_codec::Decode::decode(input)?))
                    }
                    _ => None,
                }
            }
        }
    };
#[automatically_derived]
#[allow(unused_qualifications)]
impl <B: ::std::fmt::Debug> ::std::fmt::Debug for RawEvent<B> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&RawEvent::Dummy(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Dummy");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl <B> From<RawEvent<B>> for () {
    fn from(_: RawEvent<B>) -> () { () }
}
impl <B> RawEvent<B> {
    #[allow(dead_code)]
    pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
        &[::srml_support::event::EventMetadata{name:
                                                   ::srml_support::event::DecodeDifferent::Encode("Dummy"),
                                               arguments:
                                                   ::srml_support::event::DecodeDifferent::Encode(&["B"]),
                                               documentation:
                                                   ::srml_support::event::DecodeDifferent::Encode(&[r" Dummy event, just here so there's a generic type that's used."]),}]
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
    fn on_initialize(_n: T::BlockNumber) { }
}
impl <T: Trait>
 ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber> for
 Module<T> {
    fn on_finalize(_n: T::BlockNumber) { <Dummy<T>>::kill(); }
}
impl <T: Trait>
 ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
 for Module<T> {
    fn generate_extrinsics(_n: T::BlockNumber) { }
}
impl <T: Trait> Module<T> {
    fn deposit_event(event: Event<T>) {
        <system::Module<T>>::deposit_event(<T as Trait>::from(event).into());
    }
}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl <T: Trait> Module<T> {
    #[doc = r" This is your public interface. Be extremely careful."]
    #[doc =
          r" This is just a simple example of how to interact with the module from the external"]
    #[doc = r" world."]
    fn accumulate_dummy(origin: T::Origin, increase_by: T::Balance)
     -> Result {
        let _sender = ensure_signed(origin)?;
        <Dummy<T>>::mutate(|dummy|
                               {
                                   let new_dummy =
                                       dummy.map_or(increase_by,
                                                    |dummy|
                                                        dummy + increase_by);
                                   *dummy = Some(new_dummy);
                               });
        Self::deposit_event(RawEvent::Dummy(increase_by));
        Ok(())
    }
    #[doc =
          r" A privileged call; in this case it resets our dummy value to something new."]
    fn set_dummy(new_value: T::Balance) -> ::srml_support::dispatch::Result {
        { <Dummy<T>>::put(new_value); }
        Ok(())
    }
}
pub enum Call<T: Trait> {

    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(::srml_support::rstd::marker::PhantomData<(T)>,
                  ::srml_support::dispatch::Never),

    #[allow(non_camel_case_types)]
    accumulate_dummy(T::Balance),

    #[allow(non_camel_case_types)]
    set_dummy(
              #[codec(compact)]
              T::Balance),
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Call: () =
    {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl <T: Trait> _parity_codec::Encode for Call<T> where
         T::Balance: _parity_codec::Encode, T::Balance: _parity_codec::Encode,
         T::Balance: _parity_codec::HasCompact {
            fn encode_to<EncOut: _parity_codec::Output>(&self,
                                                        dest: &mut EncOut) {
                match *self {
                    Call::accumulate_dummy(ref aa) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                    }
                    Call::set_dummy(ref aa) => {
                        dest.push_byte(1usize as u8);
                        {
                            dest.push(&<<T::Balance as
                                        _parity_codec::HasCompact>::Type as
                                           _parity_codec::EncodeAsRef<'_,
                                                                      T::Balance>>::from(aa));
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
         T::Balance: _parity_codec::Decode, T::Balance: _parity_codec::Decode,
         T::Balance: _parity_codec::HasCompact {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn)
             -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => {
                        Some(Call::accumulate_dummy(_parity_codec::Decode::decode(input)?))
                    }
                    x if x == 1usize as u8 => {
                        Some(Call::set_dummy(<<T::Balance as
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
            Call::accumulate_dummy(ref increase_by) =>
            Call::accumulate_dummy((*increase_by).clone()),
            Call::set_dummy(ref new_value) =>
            Call::set_dummy((*new_value).clone()),
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/example/src/lib.rs",
                                             346u32, 1u32))
                }
            }
        }
    }
}
impl <T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::accumulate_dummy(ref increase_by) => {
                let self_params = (increase_by,);
                if let Call::accumulate_dummy(ref increase_by) = *_other {
                    self_params == (increase_by,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/example/src/lib.rs",
                                                         346u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            Call::set_dummy(ref new_value) => {
                let self_params = (new_value,);
                if let Call::set_dummy(ref new_value) = *_other {
                    self_params == (new_value,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            {
                                ::std::rt::begin_panic("internal error: entered unreachable code",
                                                       &("srml/example/src/lib.rs",
                                                         346u32, 1u32))
                            }
                        }
                        _ => false,
                    }
                }
            }
            _ => {
                {
                    ::std::rt::begin_panic("internal error: entered unreachable code",
                                           &("srml/example/src/lib.rs",
                                             346u32, 1u32))
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
            Call::accumulate_dummy(ref increase_by) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"accumulate_dummy",
                                                               &(increase_by.clone(),))
                                                            {
                                                            (arg0, arg1) =>
                                                            [::std::fmt::ArgumentV1::new(arg0,
                                                                                         ::std::fmt::Display::fmt),
                                                             ::std::fmt::ArgumentV1::new(arg1,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        })),
            Call::set_dummy(ref new_value) =>
            _f.write_fmt(::std::fmt::Arguments::new_v1(&["", ""],
                                                       &match (&"set_dummy",
                                                               &(new_value.clone(),))
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
                                           &("srml/example/src/lib.rs",
                                             346u32, 1u32))
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
            Call::accumulate_dummy(increase_by) => {
                <Module<T>>::accumulate_dummy(_origin, increase_by)
            }
            Call::set_dummy(new_value) => {
                {
                    system::ensure_root(_origin)?;
                    <Module<T>>::set_dummy(new_value)
                }
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
                                                       &("srml/example/src/lib.rs",
                                                         346u32, 1u32))
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
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("accumulate_dummy"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("increase_by"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("T::Balance"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" This is your public interface. Be extremely careful.",
                                                                                                             r" This is just a simple example of how to interact with the module from the external",
                                                                                                             r" world."]),},
          ::srml_support::dispatch::FunctionMetadata{name:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode("set_dummy"),
                                                     arguments:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[::srml_support::dispatch::FunctionArgumentMetadata{name:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("new_value"),
                                                                                                                                                                ty:
                                                                                                                                                                    ::srml_support::dispatch::DecodeDifferent::Encode("Compact<T::Balance>"),}]),
                                                     documentation:
                                                         ::srml_support::dispatch::DecodeDifferent::Encode(&[r" A privileged call; in this case it resets our dummy value to something new."]),}]
    }
}
impl <T: Trait> Module<T> {
    #[allow(dead_code)]
    fn accumulate_foo(origin: T::Origin, increase_by: T::Balance) -> Result {
        let _sender = ensure_signed(origin)?;
        let prev = <Foo<T>>::get();
        let result =
            <Foo<T>>::mutate(|foo| { *foo = *foo + increase_by; *foo });
        if !(prev + increase_by == result) {
            {
                ::std::rt::begin_panic("assertion failed: prev + increase_by == result",
                                       &("srml/example/src/lib.rs", 476u32,
                                         3u32))
            }
        };
        Ok(())
    }
}
