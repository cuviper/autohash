// This benchmark suite contains some benchmarks along a set of dimensions:
//   Hasher: std default (SipHash) and crate default (AHash).
//   Int key distribution: low bit heavy, top bit heavy, and random.
//   Task: basic functionality: insert, insert_erase, lookup, lookup_fail, iter
#![feature(test)]

extern crate test;

use ahash::AHasher;
use autohash::wrappers::{AutoHashed, MemoHashed};
use autohash::{AutoHash, AutoHashMap};
use hashbrown::HashMap;
use paste::paste;
use std::collections::hash_map::DefaultHasher;
use std::hash::BuildHasherDefault;
use test::{black_box, Bencher};

const SIZE: usize = 1000;

type AutoHashedMap<H> = AutoHashMap<AutoHashed<usize, H>, usize>;
type MemoHashedMap<H> = AutoHashMap<MemoHashed<usize, H>, usize>;
type UsizeHashMap = AutoHashMap<UsizeHash, usize>;

type HashbrownMap<H> = HashMap<usize, usize, BuildHasherDefault<H>>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UsizeHash(usize);

impl From<usize> for UsizeHash {
    #[inline]
    fn from(hash: usize) -> Self {
        UsizeHash(hash)
    }
}

impl AutoHash for UsizeHash {
    #[inline]
    fn get_hash(&self) -> u64 {
        self.0 as u64
    }
}

// A random key iterator.
#[derive(Clone, Copy)]
struct RandomKeys {
    state: usize,
}

impl RandomKeys {
    fn new() -> Self {
        RandomKeys { state: 0 }
    }
}

impl Iterator for RandomKeys {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        // Add 1 then multiply by some 32 bit prime.
        self.state = self.state.wrapping_add(1).wrapping_mul(3787392781);
        Some(self.state)
    }
}

macro_rules! bench_suite {
    (@ $bench:ident, $name:ident, $map:ty) => {
        paste! {
            $bench!([<$bench _ $name _serial>], $map, 0usize..);
            $bench!([<$bench _ $name _highbits>], $map, (0..).map(usize::swap_bytes));
            $bench!([<$bench _ $name _random>], $map, RandomKeys::new());
        }
    };
    ($bench:ident) => {
        bench_suite!(@ $bench, auto_hashed_ahash, AutoHashedMap<AHasher>);
        bench_suite!(@ $bench, auto_hashed_std, AutoHashedMap<DefaultHasher>);
        bench_suite!(@ $bench, memo_hashed_ahash, MemoHashedMap<AHasher>);
        bench_suite!(@ $bench, memo_hashed_std, MemoHashedMap<DefaultHasher>);
        bench_suite!(@ $bench, hashbrown_ahash, HashbrownMap<AHasher>);
        bench_suite!(@ $bench, hashbrown_std, HashbrownMap<DefaultHasher>);
        bench_suite!(@ $bench, usize_hash, UsizeHashMap);
    };
}

macro_rules! insert_reserved {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m: $maptype = Default::default();
            m.reserve(SIZE);
            b.iter(|| {
                m.clear();
                for i in ($keydist).take(SIZE) {
                    m.insert(i.into(), i);
                }
                black_box(&mut m);
            })
        }
    };
}
bench_suite!(insert_reserved);

macro_rules! insert_unreserved {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut m: $maptype = Default::default();
                for i in ($keydist).take(SIZE) {
                    m.insert(i.into(), i);
                }
                black_box(m);
            })
        }
    };
}
bench_suite!(insert_unreserved);

macro_rules! insert_erase {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut base: $maptype = Default::default();
            for i in ($keydist).take(SIZE) {
                base.insert(i.into(), i);
            }
            let skip = $keydist.skip(SIZE);
            b.iter(|| {
                let mut m = base.clone();
                let mut add_iter = skip.clone();
                let mut remove_iter = $keydist;
                // While keeping the size constant,
                // replace the first keydist with the second.
                for (add, remove) in (&mut add_iter).zip(&mut remove_iter).take(SIZE) {
                    m.insert(add.into(), add);
                    black_box(m.remove(&remove.into()));
                }
                black_box(m);
            })
        }
    };
}
bench_suite!(insert_erase);

macro_rules! lookup_pass {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m: $maptype = Default::default();
            for i in $keydist.take(SIZE) {
                m.insert(i.into(), i);
            }

            b.iter(|| {
                for i in $keydist.take(SIZE) {
                    black_box(m.get(&i.into()));
                }
            })
        }
    };
}
bench_suite!(lookup_pass);

macro_rules! lookup_fail {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m: $maptype = Default::default();
            let mut iter = $keydist;
            for i in (&mut iter).take(SIZE) {
                m.insert(i.into(), i);
            }

            b.iter(|| {
                for i in (&mut iter).take(SIZE) {
                    black_box(m.get(&i.into()));
                }
            })
        }
    };
}
bench_suite!(lookup_fail);

macro_rules! iter {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m: $maptype = Default::default();
            for i in ($keydist).take(SIZE) {
                m.insert(i.into(), i);
            }

            b.iter(|| {
                for i in &m {
                    black_box(i);
                }
            })
        }
    };
}
bench_suite!(iter);

macro_rules! clone_small {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m: $maptype = Default::default();
            for i in ($keydist).take(10) {
                m.insert(i.into(), i);
            }

            b.iter(|| {
                black_box(m.clone());
            })
        }
    };
}
bench_suite!(clone_small);

macro_rules! clone_from_small {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m: $maptype = Default::default();
            let mut m2: $maptype = Default::default();
            for i in ($keydist).take(10) {
                m.insert(i.into(), i);
            }

            b.iter(|| {
                m2.clone_from(&m);
                black_box(&mut m2);
            })
        }
    };
}
bench_suite!(clone_from_small);

macro_rules! clone_large {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m: $maptype = Default::default();
            for i in ($keydist).take(SIZE) {
                m.insert(i.into(), i);
            }

            b.iter(|| {
                black_box(m.clone());
            })
        }
    };
}
bench_suite!(clone_large);

macro_rules! clone_from_large {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m: $maptype = Default::default();
            let mut m2: $maptype = Default::default();
            for i in ($keydist).take(SIZE) {
                m.insert(i.into(), i);
            }

            b.iter(|| {
                m2.clone_from(&m);
                black_box(&mut m2);
            })
        }
    };
}
bench_suite!(clone_from_large);

macro_rules! grow_shrink {
    ($name:ident, $maptype:ty, $keydist:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let mut m: $maptype = Default::default();
            for i in ($keydist).take(SIZE) {
                m.insert(i.into(), i);
            }
            m.shrink_to_fit();

            b.iter(|| {
                m.reserve(10 * SIZE);
                m.shrink_to_fit();
            })
        }
    };
}
bench_suite!(grow_shrink);
