autohash
=========

<!-- [![Build Status](https://travis-ci.com/cuviper/autohash.svg?branch=master)](https://travis-ci.com/cuviper/autohash) -->
[![Crates.io](https://img.shields.io/crates/v/autohash.svg)](https://crates.io/crates/autohash)
[![Documentation](https://docs.rs/autohash/badge.svg)](https://docs.rs/autohash)

This crate provides `AutoHashMap` and `AutoHashSet` where the keys are
self-hashed. The implementation is built on `RawTable` from [`hashbrown`].

Keys implement the `AutoHash` trait for getting their hash value, instead of
using a generic `Hasher` in the table with `Hash` keys. This may be useful when
the type is already hash-like or has a saved hash, or if you don't want your
type to implement `Hash` for some reason.

Example wrappers are included:

- `U64Hash(u64)`: Use a direct hash value as a key.
- `AutoHashed<T, H>`: For `T: Hash, H: Hasher + Default`, this computes the hash
  automatically. This is effectively the same as using a normal hash map/set
  with `S = BuildHasherDefault<H>`, just specified on the key type instead.
- `MemoHashed<T, H>`: For `T: Hash, H: Hasher + Default`, this computes the hash
  automatically when it is constructed -- for a precomputed/memoized hash.
- `RawHashed<T>`: Pairs a value with its raw hash.

[`hashbrown`]: https://crates.io/crates/hashbrown

## [Change log](CHANGELOG.md)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
autohash = "0.1"
```

Then:

```rust
use autohash::AutoHashMap;
use autohash::wrappers::U64Hash;

let mut map = AutoHashMap::new();
map.insert(U64Hash(1), "one");
map.insert(U64Hash(u64::MAX), "max");
```

This crate has the following Cargo features:

- `serde`: Enables serde serialization support.
- `rayon`: Enables rayon parallel iterator support.
- `inline-more`: Adds inline hints to most functions, improving run-time performance at the cost
  of compilation time. (enabled by default)

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
