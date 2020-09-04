//! Simple wrappers that implement `AutoHash`.

use crate::AutoHash;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

/// A wrapper for an existing `u64` hash value.
///
/// This is a simple example of a key type that knows its own hash value,
/// without implementing the `Hash` trait at all.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct U64Hash(pub u64);

impl AutoHash for U64Hash {
    #[inline]
    fn get_hash(&self) -> u64 {
        self.0
    }
}

impl fmt::Debug for U64Hash {
    #[cfg_attr(feature = "inline-more", inline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A wrapper that automatically uses a default hasher.
///
/// Using this with `AutoHashMap`/`Set` is effectively the same as a normal
/// `HashMap`/`Set` with `S = BuildHasherDefault<H>`, just specified on the
/// key type instead.
#[repr(transparent)]
pub struct AutoHashed<T, H> {
    /// The wrapped value
    pub value: T,
    hasher: PhantomData<H>,
}

impl<T, H> From<T> for AutoHashed<T, H> {
    #[inline]
    fn from(value: T) -> Self {
        Self {
            value,
            hasher: PhantomData,
        }
    }
}

impl<T: Clone, H> Clone for AutoHashed<T, H> {
    #[inline]
    fn clone(&self) -> Self {
        Self::from(self.value.clone())
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.value.clone_from(&other.value);
    }
}

impl<T: Copy, H> Copy for AutoHashed<T, H> {}

impl<T: fmt::Debug, H> fmt::Debug for AutoHashed<T, H> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: PartialEq, H> PartialEq for AutoHashed<T, H> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq, H> Eq for AutoHashed<T, H> {}

impl<T, H> AutoHash for AutoHashed<T, H>
where
    T: Hash,
    H: Hasher + Default,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn get_hash(&self) -> u64 {
        let hasher = &mut H::default();
        self.value.hash(hasher);
        hasher.finish()
    }
}

/// A wrapper that caches the hash using a default hasher.
///
/// Using this with `AutoHashMap`/`Set` is similar to a normal `HashMap`/`Set`
/// with `S = BuildHasherDefault<H>`, but the hash is computed and saved as soon
/// as the key is wrapped.
pub struct MemoHashed<T, H> {
    /// The wrapped value
    pub value: T,
    hash: u64,
    hasher: PhantomData<H>,
}

impl<T, H> From<T> for MemoHashed<T, H>
where
    T: Hash,
    H: Hasher + Default,
{
    #[inline]
    fn from(value: T) -> Self {
        let hasher = &mut H::default();
        value.hash(hasher);
        let hash = hasher.finish();
        Self {
            value,
            hash,
            hasher: PhantomData,
        }
    }
}

impl<T: Clone, H> Clone for MemoHashed<T, H> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            hash: self.hash,
            hasher: PhantomData,
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.value.clone_from(&other.value);
        self.hash = other.hash;
    }
}

impl<T: Copy, H> Copy for MemoHashed<T, H> {}

impl<T: fmt::Debug, H> fmt::Debug for MemoHashed<T, H> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: PartialEq, H> PartialEq for MemoHashed<T, H> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq, H> Eq for MemoHashed<T, H> {}

impl<T, H> AutoHash for MemoHashed<T, H> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn get_hash(&self) -> u64 {
        self.hash
    }
}

/// A wrapper that uses a provided hash.
///
/// The raw hash is still required to uphold the contract that if two values are
/// equal, their hashes must also be equal.
pub struct RawHashed<T> {
    /// The wrapped value
    pub value: T,
    hash: u64,
}

impl<T> RawHashed<T> {
    /// Pairs a value with its raw hash
    pub fn new(hash: u64, value: T) -> Self {
        RawHashed { hash, value }
    }
}

impl<T: Clone> Clone for RawHashed<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            hash: self.hash,
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.value.clone_from(&other.value);
        self.hash = other.hash;
    }
}

impl<T: Copy> Copy for RawHashed<T> {}

impl<T: fmt::Debug> fmt::Debug for RawHashed<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: PartialEq> PartialEq for RawHashed<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for RawHashed<T> {}

impl<T> AutoHash for RawHashed<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn get_hash(&self) -> u64 {
        self.hash
    }
}
