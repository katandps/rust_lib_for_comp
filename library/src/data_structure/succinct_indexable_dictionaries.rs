//! 簡潔ビットベクトル(完備辞書)

/// SuccinctIndexableDictionary
///
/// ## 概要
/// ビット列に対してrank()とselect()を提供する簡潔データ構造
///
pub struct SID {
    size: usize,
    _blocks: usize,
    bits: Vec<u64>,
    sum: Vec<u64>,
}

impl SID {
    const BIT_LEN_LEN: usize = 6;
    const BIT_LEN: usize = 1 << Self::BIT_LEN_LEN;

    /// p番目のビットが立っているか
    ///
    /// ## 計算量
    /// O(1)
    pub fn access(&self, p: usize) -> bool {
        self.bits[p >> Self::BIT_LEN_LEN] >> (p & (Self::BIT_LEN - 1)) & 1 != 0
    }

    /// [0, p)に1のビットがいくつあるか
    ///
    /// ## 計算量
    /// O(1)
    pub fn rank(&self, p: usize) -> u64 {
        self.sum[p >> Self::BIT_LEN_LEN]
            + (self.bits[p >> Self::BIT_LEN_LEN] & ((1 << (p & (Self::BIT_LEN - 1))) - 1))
                .count_ones() as u64
    }

    /// 1のビットをx個含む[0, p)の区間であって、pが最小となるものを返す
    /// 存在しない場合はNone
    ///
    /// ## 計算量
    /// O(log(Size))
    pub fn select(&self, x: usize) -> Option<usize> {
        let x = x as u64;
        if self.rank(self.size) < x {
            return None;
        }
        let (mut lb, mut ub) = (-1, self.size as i64);
        while ub - lb > 1 {
            let mid = (lb + ub) >> 1;
            if self.rank(mid as usize) < x {
                lb = mid
            } else {
                ub = mid
            }
        }
        Some(ub as usize)
    }
}

pub struct SIDBuilder {
    size: usize,
    blocks: usize,
    bits: Vec<u64>,
}

impl SIDBuilder {
    pub fn new(size: usize) -> Self {
        let blocks = (size + SID::BIT_LEN) >> SID::BIT_LEN_LEN;
        Self {
            size,
            blocks,
            bits: vec![0; blocks],
        }
    }

    pub fn set(&mut self, p: usize) {
        self.bits[p >> SID::BIT_LEN_LEN] |= 1 << (p & (SID::BIT_LEN - 1))
    }

    pub fn build(self) -> SID {
        let mut sum = vec![0; self.blocks];
        for i in 1..self.blocks {
            sum[i] = sum[i - 1] + self.bits[i - 1].count_ones() as u64;
        }
        SID {
            size: self.size,
            _blocks: self.blocks,
            bits: self.bits,
            sum,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut builder = SIDBuilder::new(1024);
        builder.set(0);
        builder.set(1);
        builder.set(125);
        builder.set(250);
        builder.set(500);
        builder.set(1000);
        let dict = builder.build();
        assert!(dict.access(0));
        assert!(dict.access(1));
        assert!(!dict.access(2));
        assert!(dict.access(125));
        assert!(!dict.access(126));

        assert_eq!(0, dict.rank(0));
        assert_eq!(1, dict.rank(1));
        assert_eq!(2, dict.rank(2));
        assert_eq!(2, dict.rank(3));
        assert_eq!(2, dict.rank(125));
        assert_eq!(3, dict.rank(126));

        assert_eq!(Some(0), dict.select(0));
        assert_eq!(Some(1), dict.select(1));
        assert_eq!(Some(2), dict.select(2));
        assert_eq!(Some(126), dict.select(3));
        assert_eq!(Some(251), dict.select(4));
        assert_eq!(Some(501), dict.select(5));
        assert_eq!(Some(1001), dict.select(6));
        assert_eq!(None, dict.select(7));
    }
}
