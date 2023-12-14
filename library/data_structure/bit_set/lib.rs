//! # BitSet
//! bitの大きな配列に対するbit演算を省メモリかつ高速に行うデータ構造
use prelude::*;

#[codesnip::entry("bit-set", doc_hidden)]
pub use bitset_impl::BitSet;
#[codesnip::entry("bit-set", doc_hidden)]
mod bitset_impl {
    use super::{
        BitAnd, BitOr, BitXor, Debug, Display, Formatter, FromIterator, Index, Not, Shl, ShlAssign,
        Shr, ShrAssign,
    };
    #[derive(Clone, Eq, PartialEq)]
    pub struct BitSet {
        bits: Vec<u64>,
        size: usize,
    }

    impl BitSet {
        /// bitの長さ $2^6 = 64$
        const BLOCK_LEN: usize = u64::BITS as usize;
        /// # bitの長さの長さ
        /// indexからu64単位の場所を割り出すのに使う
        const BLOCK_LEN_LEN: usize = Self::BLOCK_LEN.trailing_zeros() as usize;
        pub fn new(size: usize) -> Self {
            let bits = vec![0; (size + Self::BLOCK_LEN - 1) / Self::BLOCK_LEN];
            Self { bits, size }
        }

        /// indexビット目のビットをbに変える
        pub fn set(&mut self, index: usize, b: bool) {
            assert!(index < self.size);
            if b {
                self.bits[index >> Self::BLOCK_LEN_LEN] |= 1 << (index & (Self::BLOCK_LEN - 1));
            } else {
                self.bits[index >> Self::BLOCK_LEN_LEN] &= !(1 << (index & (Self::BLOCK_LEN - 1)));
            }
        }
        /// indexビット目のbitを反転する
        pub fn rev(&mut self, index: usize) {
            assert!(index < self.size);
            self.bits[index >> Self::BLOCK_LEN_LEN] ^= 1 << (index & (Self::BLOCK_LEN - 1));
        }

        /// 立っているビットの数を数える
        pub fn count_ones(&self) -> u32 {
            self.bits.iter().map(|b| b.count_ones()).sum()
        }
        /// 下位64bitをu64として取り出す
        pub fn get_u64(&self) -> u64 {
            self.bits[0]
        }
        // sizeを超えた分を切り捨てる
        fn chomp(&mut self) {
            let r = self.size & (Self::BLOCK_LEN - 1);
            if r != 0 {
                let d = Self::BLOCK_LEN - r;
                if let Some(x) = self.bits.last_mut() {
                    *x = (*x << d) >> d;
                }
            }
        }
    }

    impl Debug for BitSet {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            Display::fmt(self, f)
        }
    }

    impl Index<usize> for BitSet {
        type Output = bool;
        fn index(&self, index: usize) -> &bool {
            assert!(index < self.size);
            &[false, true][((self.bits[index >> Self::BLOCK_LEN_LEN]
                >> (index & (Self::BLOCK_LEN - 1)))
                & 1) as usize]
        }
    }

    impl BitAnd for BitSet {
        type Output = BitSet;
        fn bitand(self, rhs: Self) -> Self::Output {
            &self & &rhs
        }
    }
    impl BitAnd for &BitSet {
        type Output = BitSet;
        fn bitand(self, rhs: Self) -> Self::Output {
            assert_eq!(self.size, rhs.size);
            BitSet {
                bits: self
                    .bits
                    .iter()
                    .zip(rhs.bits.iter())
                    .map(|(l, r)| l & r)
                    .collect(),
                size: self.size,
            }
        }
    }

    impl BitOr for BitSet {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            assert_eq!(self.size, rhs.size);
            Self {
                bits: (0..self.bits.len())
                    .map(|i| self.bits[i] | rhs.bits[i])
                    .collect(),
                size: self.size,
            }
        }
    }

    impl BitXor for BitSet {
        type Output = Self;

        fn bitxor(self, rhs: Self) -> Self::Output {
            assert_eq!(self.size, rhs.size);
            Self {
                bits: (0..self.bits.len())
                    .map(|i| self.bits[i] ^ rhs.bits[i])
                    .collect(),
                size: self.size,
            }
        }
    }

    impl ShlAssign<usize> for BitSet {
        fn shl_assign(&mut self, rhs: usize) {
            // 全体サイズ以上シフトすると全部0になる
            if rhs >= self.size {
                self.bits.iter_mut().for_each(|b| *b = 0);
                return;
            }
            // ブロック単位のシフト量
            let block = rhs >> Self::BLOCK_LEN_LEN;
            // ブロック内部のシフト量
            let inner = rhs & (Self::BLOCK_LEN - 1);
            if inner == 0 {
                (block..self.bits.len())
                    .rev()
                    .for_each(|i| self.bits[i] = self.bits[i - block])
            } else {
                (block + 1..self.bits.len()).rev().for_each(|i| {
                    self.bits[i] = (self.bits[i - block] << inner)
                        | (self.bits[i - block - 1] >> (Self::BLOCK_LEN - inner))
                });
                self.bits[block] = self.bits[0] << inner;
            }
            self.bits[..block].iter_mut().for_each(|b| *b = 0);
            self.chomp();
        }
    }

    impl Shl<usize> for BitSet {
        type Output = Self;
        fn shl(mut self, rhs: usize) -> Self::Output {
            self <<= rhs;
            self
        }
    }

    impl ShrAssign<usize> for BitSet {
        fn shr_assign(&mut self, rhs: usize) {
            // 全体サイズ以上シフトすると全部0になる
            if rhs >= self.size {
                self.bits.iter_mut().for_each(|b| *b = 0);
                return;
            }
            // ブロック単位のシフト量
            let block = rhs >> Self::BLOCK_LEN_LEN;
            // ブロック内部のシフト量
            let inner = rhs & (Self::BLOCK_LEN - 1);

            let len = self.bits.len();
            if inner == 0 {
                (0..len - block).for_each(|i| self.bits[i] = self.bits[i + block])
            } else {
                (0..len - block - 1).for_each(|i| {
                    self.bits[i] = (self.bits[i + block] >> inner)
                        | (self.bits[i + block + 1] << (Self::BLOCK_LEN - inner))
                });
                self.bits[len - block - 1] = self.bits[len - 1] >> inner;
            }
            self.bits[len - block..].iter_mut().for_each(|b| *b = 0);
        }
    }

    impl Shr<usize> for BitSet {
        type Output = Self;
        fn shr(mut self, rhs: usize) -> Self::Output {
            self >>= rhs;
            self
        }
    }

    impl Not for BitSet {
        type Output = Self;
        fn not(self) -> Self::Output {
            Self {
                bits: self.bits.iter().map(|&i| i ^ std::u64::MAX).collect(),
                size: self.size,
            }
        }
    }

    impl Display for BitSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                (0..self.size)
                    .rev()
                    .map(|i| usize::from(self[i]).to_string())
                    .collect::<String>()
            )
        }
    }

    impl<Item: Into<bool>> FromIterator<Item> for BitSet {
        fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
            let mut bits = Vec::new();
            let mut size = 0;
            let mut cur = 0;
            for item in iter.into_iter() {
                if item.into() {
                    cur += 1 << (size % 64);
                }
                size += 1;
                if size % u64::BITS as usize == 0 {
                    bits.push(cur);
                    cur = 0;
                }
            }
            if size % u64::BITS as usize != 0 {
                bits.push(cur)
            }
            Self { bits, size }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut bitset = BitSet::new(100);
        bitset.set(10, true);
        assert!(bitset.index(10));
        bitset.set(11, true);
        assert!(bitset.index(11));
        bitset.set(10, false);
        assert!(!bitset.index(10));
        bitset <<= 88;
        assert!(bitset.index(99));
        assert_eq!(1, bitset.count_ones());
        bitset <<= 1;
        assert_eq!(0, bitset.count_ones());
    }

    #[test]
    fn from_iterator() {
        let bitset = (0..10usize).map(|i| i % 2 == 1).collect::<BitSet>();
        let mut expect = BitSet::new(10);
        expect.set(1, true);
        expect.set(3, true);
        expect.set(5, true);
        expect.set(7, true);
        expect.set(9, true);
        assert_eq!(bitset, expect);
        assert_eq!(bitset.clone(), expect);

        let mut expect = BitSet::new(11);
        expect.set(1, true);
        expect.set(3, true);
        expect.set(5, true);
        expect.set(7, true);
        expect.set(9, true);
        assert_ne!(bitset, expect);

        assert_eq!("1010101010", format!("{:?}", bitset));
        assert_eq!("01010101010", format!("{:?}", expect));
    }

    #[test]
    fn shift() {
        let mut bitset = (0..100).map(|i| i % 2 == 1).collect::<BitSet>();
        assert_eq!("1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010", format!("{:?}", bitset));
        bitset >>= 3;
        assert_eq!("0001010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101", format!("{:?}", bitset));
        bitset = bitset >> 90;
        assert_eq!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001010101", format!("{:?}", bitset));
        bitset = bitset << 90;
        assert_eq!("0001010101000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", format!("{:?}", bitset));
        let mut bitset = (0..10).map(|i| i % 2 == 1).collect::<BitSet>();
        bitset <<= 10;
        assert_eq!(BitSet::new(10), bitset);
        let mut bitset = (0..10).map(|i| i % 2 == 1).collect::<BitSet>();
        bitset >>= 10;
        assert_eq!(BitSet::new(10), bitset);
    }

    #[test]
    fn bit_operator() {
        let bitset = (0..100).map(|i| i % 2 == 1).collect::<BitSet>();
        assert_eq!("1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010", format!("{:?}", bitset));
        let not_bitset = !bitset.clone();
        assert_eq!("0101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101", format!("{:?}", not_bitset));
        let and_bitset1 = bitset.clone() & not_bitset.clone();
        let and_bitset2 = bitset.clone() & bitset.clone();
        assert_eq!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", format!("{:?}", and_bitset1));
        assert_eq!(bitset, and_bitset2);
        let or_bitset = bitset.clone() | not_bitset;
        let or_bitset2 = bitset.clone() | bitset.clone();
        assert_eq!("1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111", format!("{:?}", or_bitset));
        assert_eq!(bitset, or_bitset2);
    }
}
