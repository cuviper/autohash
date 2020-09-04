//! This crate provides [`AutoHashMap`] and [`AutoHashSet`] where the keys are
//! self-hashed. The implementation is built on `RawTable` from [`hashbrown`].
//!
//! Keys implement the [`AutoHash`] trait for getting their hash value, instead of
//! using a generic `Hasher` in the table with `Hash` keys. This may be useful when
//! the type is already hash-like or has a saved hash, or if you don't want your
//! type to implement `Hash` for some reason.
//!
//! Example key types are included in the [`wrappers`] module.
//!
//! [`AutoHashMap`]: map/struct.AutoHashMap.html
//! [`AutoHashSet`]: set/struct.AutoHashSet.html
//! [`AutoHash`]: trait.AutoHash.html
//! [`hashbrown`]: https://crates.io/crates/hashbrown
//! [`wrappers`]: wrappers/index.html

#![no_std]
#![allow(
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::option_if_let_else
)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

// #[cfg(test)]
// #[macro_use]
// extern crate std;

// #[cfg_attr(test, macro_use)]
extern crate alloc;

#[cfg(feature = "nightly")]
#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod external_trait_impls;

pub mod map;
pub mod set;
pub mod wrappers;

pub use crate::map::AutoHashMap;
pub use crate::set::AutoHashSet;

pub use hashbrown::TryReserveError;

/// A self-hashed type.
///
/// Types implementing `AutoHash` are able to return their hash value independently.
///
/// If two values are equal, their hashes must also be equal, even through `Borrow`.
pub trait AutoHash {
    /// Return the hash for this value.
    fn get_hash(&self) -> u64;
}

// The forwarding impls below match the standard library's `impl<T> Borrow<T>`.

impl<T: AutoHash + ?Sized> AutoHash for &'_ T {
    #[inline]
    fn get_hash(&self) -> u64 {
        T::get_hash(*self)
    }
}

impl<T: AutoHash + ?Sized> AutoHash for &'_ mut T {
    #[inline]
    fn get_hash(&self) -> u64 {
        T::get_hash(*self)
    }
}

impl<T> AutoHash for alloc::borrow::Cow<'_, T>
where
    T: AutoHash + alloc::borrow::ToOwned + ?Sized,
{
    #[inline]
    fn get_hash(&self) -> u64 {
        T::get_hash(&**self)
    }
}

impl<T: AutoHash + ?Sized> AutoHash for alloc::boxed::Box<T> {
    #[inline]
    fn get_hash(&self) -> u64 {
        T::get_hash(&**self)
    }
}

impl<T: AutoHash + ?Sized> AutoHash for alloc::rc::Rc<T> {
    #[inline]
    fn get_hash(&self) -> u64 {
        T::get_hash(&**self)
    }
}

impl<T: AutoHash + ?Sized> AutoHash for alloc::sync::Arc<T> {
    #[inline]
    fn get_hash(&self) -> u64 {
        T::get_hash(&**self)
    }
}
