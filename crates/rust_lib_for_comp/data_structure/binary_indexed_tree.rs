//! # BinaryIndexedTree(Fenwick Tree)
//! アーベル群の二項演算を載せることができる
//! 計算量は各$\log(N)$
use crate::algebra::{AbelianGroup, LeastSignificantBit, Magma};
use crate::prelude::*;
use crate::range_traits::{PointUpdate, RangeProduct, ToBounds};

#[codesnip::entry("binary-indexed-tree")]
pub use binary_indexed_tree_impl::BinaryIndexedTree;
#[codesnip::entry("binary-indexed-tree", include("algebra", "prelude", "range-traits"))]
mod binary_indexed_tree_impl {
    use super::{
        AbelianGroup, Debug, Formatter, LeastSignificantBit, Magma, PointUpdate, RangeProduct,
        ToBounds,
    };
    #[derive(Clone)]
    pub struct BinaryIndexedTree<A: Magma> {
        pub n: usize,
        bit: Vec<A::M>,
    }

    /// サイズを指定して作成する
    impl<A: AbelianGroup> From<usize> for BinaryIndexedTree<A> {
        fn from(length: usize) -> Self {
            Self {
                n: length,
                bit: vec![A::unit(); length + 1],
            }
        }
    }

    /// ソースを指定して作成する
    impl<A: AbelianGroup> From<Vec<A::M>> for BinaryIndexedTree<A> {
        fn from(src: Vec<A::M>) -> Self {
            let mut bit = Self::from(src.len());
            src.into_iter()
                .enumerate()
                .for_each(|(i, item)| bit.add(i, item));
            bit
        }
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(\log N)$
    impl<A: AbelianGroup> RangeProduct<usize> for BinaryIndexedTree<A> {
        type Magma = A;
        fn product<R: ToBounds<usize>>(&self, range: R) -> A::M {
            let (a, b) = range.lr();
            if b == 0 {
                A::unit()
            } else if a == 0 {
                self.sum(b - 1)
            } else {
                A::op(&self.sum(b - 1), &A::inv(&self.sum(a - 1)))
            }
        }
    }

    /// # i番目をxに更新する
    impl<A: AbelianGroup> PointUpdate<A::M> for BinaryIndexedTree<A> {
        fn update_at(&mut self, i: usize, mut x: A::M) {
            let mut idx = i + 1;
            x = A::op(&x, &A::inv(&self.product(i..=i)));
            while idx <= self.n {
                self.bit[idx] = A::op(&self.bit[idx], &x);
                idx += idx.lsb();
            }
        }
    }

    impl<A: AbelianGroup> BinaryIndexedTree<A> {
        /// add $x$ to $i$
        pub fn add(&mut self, i: usize, x: A::M) {
            let mut idx = i + 1;
            while idx <= self.n {
                self.bit[idx] = A::op(&self.bit[idx], &x);
                idx += idx.lsb();
            }
        }

        /// sum of $[0, i)$
        pub fn sum(&self, i: usize) -> A::M {
            let mut ret = A::unit();
            let mut idx = i + 1;
            while idx > 0 {
                ret = A::op(&ret, &self.bit[idx]);
                idx -= idx.lsb();
            }
            ret
        }

        /// sumが初めてlbを超える位置を返す
        /// 二項演算の結果が単調増加するときに使える
        pub fn lower_bound(&self, lb: A::M) -> usize
        where
            A::M: PartialOrd,
        {
            if lb <= A::unit() {
                return 0;
            }
            let (mut ret, mut len) = (0, 1);
            while len < self.n {
                len <<= 1;
            }
            let mut sum = A::unit();
            while len > 0 {
                if ret + len <= self.n && A::op(&sum, &self.bit[ret + len]) < lb {
                    sum = A::op(&sum, &self.bit[ret + len]);
                    ret += len;
                }
                len >>= 1;
            }
            ret
        }

        /// sumが初めてubを下回る位置を返す
        /// 二項演算の結果が単調減少するときに使える
        pub fn upper_bound(&self, ub: A::M) -> usize
        where
            A::M: PartialOrd,
        {
            if ub >= A::unit() {
                return 0;
            }
            let (mut ret, mut len) = (0, 1);
            while len < self.n {
                len <<= 1;
            }
            let mut sum = A::unit();
            while len > 0 {
                if ret + len <= self.n && A::op(&sum, &self.bit[ret + len]) > ub {
                    sum = A::op(&sum, &self.bit[ret + len]);
                    ret += len;
                }
                len >>= 1;
            }
            ret
        }
    }

    impl<A: AbelianGroup> Debug for BinaryIndexedTree<A>
    where
        A::M: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let v = (0..self.n)
                .map(|i| format!("{:?}", self.product(i..=i)))
                .collect::<Vec<_>>()
                .join(" ");
            let v2 = (0..self.n)
                .map(|i| format!("{:?}", self.sum(i)))
                .collect::<Vec<_>>()
                .join(" ");
            write!(f, "\n{}\n{}", v, v2)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::binary_operation::addition::Addition;
    use rand::Rng;

    #[test]
    fn test() {
        const LEN: usize = 1000;
        let mut v = vec![0; LEN];
        let mut bit = BinaryIndexedTree::<Addition<i64>>::from(LEN);

        for _ in 0..1000 {
            let left = rand::thread_rng().gen_range(0..LEN);
            let right = rand::thread_rng().gen_range(left..LEN);

            for i in left..right {
                v[i] += 1;
            }
            bit.add(left, 1);
            bit.add(right, -1);
        }

        for i in 0..LEN {
            assert_eq!(v[i], bit.sum(i));
        }
    }

    #[test]
    fn test_hand() {
        const LEN: usize = 10;
        let mut v = [0; LEN];
        let mut bit = BinaryIndexedTree::<Addition<i64>>::from(LEN);

        for i in 3..5 {
            v[i] += 1;
        }
        bit.add(3, 1);
        bit.add(5, -1);

        for i in 0..LEN {
            assert_eq!(v[i], bit.sum(i));
        }
    }

    #[test]
    fn test_lower_bound() {
        const LEN: usize = 6;
        let mut bit = BinaryIndexedTree::<Addition<i64>>::from(LEN);

        for i in 2..4 {
            bit.add(i, i as i64);
        }
        for i in -3..=7 {
            let lb = bit.lower_bound(i);
            if lb == 0 {
                assert!(bit.sum(lb) >= i);
            } else if lb == bit.n {
                assert!(bit.sum(lb - 1) < i);
            } else {
                assert!(bit.sum(lb) >= i);
                assert!(bit.sum(lb - 1) < i);
            }
        }
    }

    #[test]
    fn test_upper_bound() {
        const LEN: usize = 6;
        let mut bit = BinaryIndexedTree::<Addition<i64>>::from(LEN);

        for i in 2..4 {
            bit.add(i, -(i as i64));
        }
        for i in -3..=7 {
            let lb = bit.upper_bound(i);
            if lb == 0 {
                assert!(bit.sum(lb) <= i);
            } else if lb == bit.n {
                assert!(bit.sum(lb - 1) > i);
            } else {
                assert!(bit.sum(lb) <= i);
                assert!(bit.sum(lb - 1) > i);
            }
        }
    }
}
