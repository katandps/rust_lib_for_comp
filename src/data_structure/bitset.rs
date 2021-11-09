//! BitSet
//! bitの大きな配列に対するbit演算を省メモリかつ高速に行うデータ構造

use crate::prelude::*;

#[snippet(name = "bit-set", doc_hidden)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitSet {
    bits: Vec<u64>,
    size: usize,
}

#[snippet(name = "bit-set", doc_hidden)]
impl BitSet {
    const BLOCK_LEN: usize = 1 << Self::BLOCK_LEN_LEN;
    const BLOCK_LEN_LEN: usize = 6;
    pub fn new(size: usize) -> Self {
        Self {
            bits: vec![0; (size + Self::BLOCK_LEN - 1) / Self::BLOCK_LEN],
            size,
        }
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

    /// 立っているビットの数を数える
    pub fn count_ones(&self) -> u32 {
        self.bits.iter().map(|b| b.count_ones()).sum()
    }

    /// sizeを超えた分を切り捨てる
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

#[snippet(name = "bit-set", doc_hidden)]
impl Index<usize> for BitSet {
    type Output = bool;

    fn index(&self, index: usize) -> &bool {
        assert!(index < self.size);
        &[false, true][((self.bits[index >> Self::BLOCK_LEN_LEN]
            >> (index & (Self::BLOCK_LEN - 1)))
            & 1) as usize]
    }
}

#[snippet(name = "bit-set", doc_hidden)]
impl BitAnd for BitSet {
    type Output = BitSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        Self {
            bits: (0..self.bits.len())
                .map(|i| self.bits[i] & rhs.bits[i])
                .collect(),
            size: self.size,
        }
    }
}

#[snippet(name = "bit-set", doc_hidden)]
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

#[snippet(name = "bit-set", doc_hidden)]
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

#[snippet(name = "bit-set", doc_hidden)]
impl ShlAssign<usize> for BitSet {
    fn shl_assign(&mut self, rhs: usize) {
        *self = self.clone() << rhs;
    }
}

#[snippet(name = "bit-set", doc_hidden)]
impl Shl<usize> for BitSet {
    type Output = Self;

    fn shl(mut self, rhs: usize) -> Self::Output {
        // 全体サイズ以上シフトすると全部0になる
        if rhs >= self.size {
            self.bits.iter_mut().for_each(|b| *b = 0);
            return self;
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
        self
    }
}

#[snippet(name = "bit-set", doc_hidden)]
impl ShrAssign<usize> for BitSet {
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.clone() >> rhs;
    }
}

#[snippet(name = "bit-set", doc_hidden)]
impl Shr<usize> for BitSet {
    type Output = Self;

    fn shr(mut self, rhs: usize) -> Self::Output {
        // 全体サイズ以上シフトすると全部0になる
        if rhs >= self.size {
            self.bits.iter_mut().for_each(|b| *b = 0);
            return self;
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
        self
    }
}

#[snippet(name = "bit-set", doc_hidden)]
impl Not for BitSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            bits: self.bits.iter().map(|&i| i ^ std::u64::MAX).collect(),
            size: self.size,
        }
    }
}

#[snippet(name = "bit-set", doc_hidden)]
impl Display for BitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.size)
                .rev()
                .map(|i| (if self[i] { 1 } else { 0 }).to_string())
                .collect::<String>()
        )
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
}
