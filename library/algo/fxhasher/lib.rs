//! # 高速ハッシュ
//! 暗号学的に安全ではないが、高速なHash
//!
//! ## 出典
//! [cbreeden/fxhash](https://github.com/cbreeden/fxhash)

use prelude::*;

#[snippet(name = "faster-hashmap")]
#[rustfmt::skip]
pub use self::fxhasher_impl::{FxHashMap as HashMap, FxHashSet as HashSet};

#[snippet(name = "faster-hashmap")]
#[rustfmt::skip]
mod fxhasher_impl {
    use super::{BitXor, BuildHasherDefault, Hasher, TryInto};
    use std::collections::{HashMap, HashSet};

    #[derive(Default)]
    pub struct FxHasher {
        hash: u64,
    }

    type BuildHasher = BuildHasherDefault<FxHasher>;
    pub type FxHashMap<K, V> = HashMap<K, V, BuildHasher>;
    pub type FxHashSet<V> = HashSet<V, BuildHasher>;

    const ROTATE: u32 = 5;
    const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

    impl Hasher for FxHasher {
        #[inline]
        fn finish(&self) -> u64 {
            self.hash
        }
        #[inline]
        fn write(&mut self, mut bytes: &[u8]) {
            while bytes.len() >= 8 {
                self.add_to_hash(u64::from_ne_bytes(bytes[..8].try_into().unwrap()));
                bytes = &bytes[8..];
            }
            while bytes.len() >= 4 {
                self.add_to_hash(u64::from(u32::from_ne_bytes(
                    bytes[..4].try_into().unwrap(),
                )));
                bytes = &bytes[4..];
            }
            while bytes.len() >= 2 {
                self.add_to_hash(u64::from(u16::from_ne_bytes(
                    bytes[..2].try_into().unwrap(),
                )));
            }
            if let Some(&byte) = bytes.first() {
                self.add_to_hash(u64::from(byte));
            }
        }
    }

    impl FxHasher {
        #[inline]
        pub fn add_to_hash(&mut self, i: u64) {
            self.hash = self.hash.rotate_left(ROTATE).bitxor(i).wrapping_mul(SEED);
        }
    }
}

#[test]
fn test() {
    let mut k = HashSet::default();
    for i in 0..100 {
        k.insert(i);
    }
    for i in 0..100 {
        assert!(k.contains(&i));
    }
}
