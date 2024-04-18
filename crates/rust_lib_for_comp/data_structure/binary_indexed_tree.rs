//! # BinaryIndexedTree(Fenwick Tree)
//! アーベル群の二項演算を載せることができる
//! 計算量は各$\log(N)$
use crate::algebra::{AbelianGroup, LeastSignificantBit, Magma};
use crate::range_traits::{PointUpdate, RangeProductMut, ToBounds};

#[codesnip::entry("binary-indexed-tree")]
pub use binary_indexed_tree_impl::BinaryIndexedTree;
#[codesnip::entry("binary-indexed-tree", include("algebra", "prelude", "range-traits"))]
mod binary_indexed_tree_impl {
    use super::{AbelianGroup, LeastSignificantBit, Magma, PointUpdate, RangeProductMut, ToBounds};
    #[derive(Clone)]
    pub struct BinaryIndexedTree<A: Magma> {
        pub n: usize,
        bit: Vec<A::M>,
        abelian_group: A,
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(\log N)$
    impl<A: AbelianGroup> RangeProductMut<usize> for BinaryIndexedTree<A> {
        type Magma = A;
        fn product<R: ToBounds<usize>>(&mut self, range: R) -> A::M {
            let (a, b) = range.lr();
            if b == 0 {
                A::unit()
            } else if a == 0 {
                self.sum(b - 1)
            } else {
                //   a   b
                //   |   |
                let a = self.sum(a - 1);
                let b = self.sum(b - 1);
                self.abelian_group.op(&b, &A::inv(&a))
            }
        }
    }

    /// # i番目をxに更新する
    impl<A: AbelianGroup> PointUpdate<A::M> for BinaryIndexedTree<A> {
        fn update_at(&mut self, i: usize, mut x: A::M) {
            let mut idx = i + 1;
            let t = self.product(i..=i);
            x = self.abelian_group.op(&x, &A::inv(&t));
            while idx <= self.n {
                self.bit[idx] = self.abelian_group.op(&self.bit[idx], &x);
                idx += idx.lsb();
            }
        }
    }

    impl<A: AbelianGroup> BinaryIndexedTree<A> {
        /// サイズを指定して作成する
        pub fn new(length: usize, abelian_group: A) -> Self {
            Self {
                n: length,
                bit: vec![A::unit(); length + 1],
                abelian_group,
            }
        }

        /// ソースを指定して作成する
        pub fn build(src: Vec<A::M>, abelian_group: A) -> Self {
            let mut bit = Self::new(src.len(), abelian_group);
            src.into_iter()
                .enumerate()
                .for_each(|(i, item)| bit.add(i, item));
            bit
        }

        /// add $x$ to $i$
        pub fn add(&mut self, i: usize, x: A::M) {
            let mut idx = i + 1;
            while idx <= self.n {
                self.bit[idx] = self.abelian_group.op(&self.bit[idx], &x);
                idx += idx.lsb();
            }
        }

        /// sum of $[0, i)$
        pub fn sum(&mut self, i: usize) -> A::M {
            let mut ret = A::unit();
            let mut idx = i + 1;
            while idx > 0 {
                ret = self.abelian_group.op(&ret, &self.bit[idx]);
                idx -= idx.lsb();
            }
            ret
        }

        /// sumが初めてlbを超える位置を返す
        /// 二項演算の結果が単調増加するときに使える
        pub fn lower_bound(&mut self, lb: A::M) -> usize
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
                if ret + len <= self.n && self.abelian_group.op(&sum, &self.bit[ret + len]) < lb {
                    sum = self.abelian_group.op(&sum, &self.bit[ret + len]);
                    ret += len;
                }
                len >>= 1;
            }
            ret
        }

        /// sumが初めてubを下回る位置を返す
        /// 二項演算の結果が単調減少するときに使える
        pub fn upper_bound(&mut self, ub: A::M) -> usize
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
                if ret + len <= self.n && self.abelian_group.op(&sum, &self.bit[ret + len]) > ub {
                    sum = self.abelian_group.op(&sum, &self.bit[ret + len]);
                    ret += len;
                }
                len >>= 1;
            }
            ret
        }

        pub fn into_string(mut self) -> String {
            let mut buf = String::new();
            (0..self.n).for_each(|i| buf.push_str(&format!("{:?}", self.product(i..=i))));
            (0..self.n).for_each(|i| buf.push_str(&format!("{:?}", self.sum(i))));
            buf
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
        let mut v = vec![0i64; LEN];
        let mut bit = BinaryIndexedTree::new(LEN, Addition::default());

        for _ in 0..1000 {
            let left = rand::thread_rng().gen_range(0..LEN);
            let right = rand::thread_rng().gen_range(left..LEN);

            v[left..right].iter_mut().for_each(|vi| *vi += 1);
            bit.add(left, 1);
            bit.add(right, -1);
        }

        (0..LEN).for_each(|i: usize| assert_eq!(v[i], bit.sum(i)));
    }

    #[test]
    fn test_hand() {
        const LEN: usize = 10;
        let mut v = [0; LEN];
        let mut bit = BinaryIndexedTree::new(LEN, Addition::default());

        v[3] += 1;
        v[4] += 1;
        bit.add(3, 1);
        bit.add(5, -1);
        (0..LEN).for_each(|i: usize| assert_eq!(v[i], bit.sum(i)));
    }

    #[test]
    fn test_lower_bound() {
        const LEN: usize = 6;
        let mut bit = BinaryIndexedTree::new(LEN, Addition::default());

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
        let mut bit = BinaryIndexedTree::new(LEN, Addition::default());

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
