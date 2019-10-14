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

//! Lowest-abstraction level for the Substrate runtime: just exports useful primitives from std
//! or core/alloc to be used with any code that depends on the runtime.


#![doc =
       "Substrate runtime standard library as compiled when linked with Rust's standard library."]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

#[macro_export]
macro_rules! map(( $ ( $ name : expr => $ value : expr ) , * ) => (
                 vec ! [ $ ( ( $ name , $ value ) ) , * ] . into_iter (  ) .
                 collect (  ) ));




// Re-export `vec!` macro here, but not in `std` mode, since
// std's prelude already brings `vec!` into the scope.
pub use std::borrow;
pub use std::boxed;
pub use std::cell;
pub use std::clone;
pub use std::cmp;
pub use std::fmt;
pub use std::hash;
pub use std::iter;
pub use std::marker;
pub use std::mem;
pub use std::num;
pub use std::ops;
pub use std::ptr;
pub use std::rc;
pub use std::slice;
pub use std::vec;
pub use std::default;
pub use std::result;
pub use std::convert;
pub mod collections {
    pub use std::collections::btree_map;
    pub use std::collections::btree_set;
}
/// Prelude of common useful imports.
///
/// This should include only things which are in the normal std prelude.
pub mod prelude {
    pub use crate::vec::Vec;
    pub use crate::boxed::Box;
    pub use crate::cmp::{Eq, PartialEq};
    pub use crate::clone::Clone;
}
