//! # セグメント木
//! セグメント木(非再帰)
//!
//! ## verify
//! [ARC133B](https://atcoder.jp/contests/arc133/submissions/28689143)
//!
//! ## todo
//! 単調性がある場合の二分探索の実装
use algebra::*;
use prelude::*;
use range_traits::*;

#[snippet(name = "segment-tree", doc_hidden)]
pub use segment_tree_impl::SegmentTree;
#[snippet(name = "segment-tree", doc_hidden)]
mod segment_tree_impl {
    use super::{Index, Monoid, RangeProduct, ToBounds};

    #[derive(Clone, Debug)]
    pub struct SegmentTree<M: Monoid> {
        n: usize,
        node: Vec<M::M>,
    }

    impl<M: Monoid> From<&[M::M]> for SegmentTree<M> {
        fn from(v: &[M::M]) -> Self {
            let mut segtree = Self::new(v.len() + 1);
            segtree.node[segtree.n..segtree.n + v.len()].clone_from_slice(v);
            for i in (0..segtree.n - 1).rev() {
                segtree.node[i] = M::op(&segtree.node[2 * i], &segtree.node[2 * i + 1]);
            }
            segtree
        }
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(\log N)$
    impl<M: Monoid> RangeProduct<usize> for SegmentTree<M> {
        type Magma = M;
        fn product<R: ToBounds<usize>>(&self, range: R) -> M::M {
            let (mut l, mut r) = range.lr();
            l += self.n;
            r += self.n;
            let mut sml = M::unit();
            let mut smr = M::unit();
            while l < r {
                if l & 1 != 0 {
                    sml = M::op(&sml, &self.node[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = M::op(&self.node[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }
            M::op(&sml, &smr)
        }
    }

    impl<M: Monoid> SegmentTree<M> {
        /// vを初期値としてセグメント木を生成する
        /// vの長さを要素数とする
        pub fn new(n: usize) -> Self {
            let n = (n + 1).next_power_of_two();
            let node = vec![M::unit(); 2 * n];
            let mut segtree = Self { n, node };
            for i in (0..segtree.n - 1).rev() {
                segtree.node[i] = M::op(&segtree.node[2 * i], &segtree.node[2 * i + 1]);
            }
            segtree
        }

        /// 値iをvalueに更新する
        /// ## 計算量
        /// $O(\log N)$
        pub fn update_at(&mut self, mut i: usize, value: M::M) {
            i += self.n;
            self.node[i] = value;
            while i > 0 {
                i >>= 1;
                self.node[i] = M::op(&self.node[2 * i], &self.node[2 * i + 1]);
            }
        }
    }

    /// indexの値を取得する
    /// ## 計算量
    /// $O(1)$
    impl<M: Monoid> Index<usize> for SegmentTree<M> {
        type Output = M::M;

        fn index(&self, i: usize) -> &M::M {
            &self.node[i + self.n]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maximization::Maximization;

    #[test]
    fn it_works() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut segtree: SegmentTree<Maximization<i64>> = SegmentTree::from(&base[..]);

        for i in 0..base.len() {
            assert_eq!(base[i], segtree[i]);
        }
        assert_eq!(3, segtree.product(0..1));
        assert_eq!(3, segtree.product(0..2));
        assert_eq!(4, segtree.product(0..3));
        assert_eq!(4, segtree.product(0..4));
        assert_eq!(5, segtree.product(0..5));
        assert_eq!(9, segtree.product(0..6));

        segtree.update_at(3, 8);
        assert_eq!(4, segtree.product(0..3));
        assert_eq!(8, segtree.product(0..4));
        assert_eq!(8, segtree.product(2..5));
    }
}
