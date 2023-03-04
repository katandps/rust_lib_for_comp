//! # BinaryIndexedTree(Fenwick Tree)
//! アーベル群の二項演算を載せることができる
//! 計算量は各$\log(N)$
use algebra::{AbelianGroup, Magma};
use prelude::*;
use range_traits::{RangeProduct, ToLR};

#[snippet(name = "binary-indexed-tree", doc_hidden)]
#[derive(Clone)]
pub struct BinaryIndexedTree<A: Magma> {
    n: usize,
    bit: Vec<A::M>,
}

#[snippet(name = "binary-indexed-tree", doc_hidden)]
mod binary_indexed_tree_impl {
    use super::{AbelianGroup, BinaryIndexedTree, Debug, Formatter, RangeProduct, ToLR};

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
    impl<A: AbelianGroup> From<&[A::M]> for BinaryIndexedTree<A> {
        fn from(src: &[A::M]) -> Self {
            let mut bit = Self::from(src.len());
            src.iter()
                .enumerate()
                .for_each(|(i, item)| bit.add(i + 1, item.clone()));
            bit
        }
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(\log N)$
    impl<A: AbelianGroup> RangeProduct<usize> for BinaryIndexedTree<A> {
        type Magma = A;
        fn product<R: ToLR<usize>>(&self, range: R) -> A::M {
            let (a, b) = range.to_lr();
            if b == 0 {
                A::unit()
            } else if a == 0 {
                self.sum(b - 1)
            } else {
                A::op(&self.sum(b - 1), &A::inv(&self.sum(a - 1)))
            }
        }
    }

    impl<A: AbelianGroup> BinaryIndexedTree<A> {
        /// add $x$ to $i$
        pub fn add(&mut self, i: usize, x: A::M) {
            let mut idx = i as i32 + 1;
            while idx <= self.n as i32 {
                self.bit[idx as usize] = A::op(&self.bit[idx as usize], &x);
                idx += idx & -idx;
            }
        }

        /// update $i$ to $x$
        pub fn update(&mut self, i: usize, mut x: A::M) {
            let mut idx = i as i32 + 1;
            x = A::op(&x, &A::inv(&self.product(i..=i)));
            while idx <= self.n as i32 {
                self.bit[idx as usize] = A::op(&self.bit[idx as usize], &x);
                idx += idx & -idx;
            }
        }

        /// sum of $[0, i)$
        pub fn sum(&self, i: usize) -> A::M {
            let mut ret = A::unit();
            let mut idx = i as i32 + 1;
            while idx > 0 {
                ret = A::op(&ret, &self.bit[idx as usize]);
                idx -= idx & -idx;
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
    use addition::Addition;
    use rand::Rng;

    #[test]
    fn test() {
        const LEN: usize = 1000;
        let mut v = vec![0; LEN];
        let mut bit = BinaryIndexedTree::<Addition<i64>>::from(LEN);

        for _ in 0..1000 {
            let left = rand::thread_rng().gen_range(0, LEN);
            let right = rand::thread_rng().gen_range(left, LEN);

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
        let mut v = vec![0; LEN];
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
