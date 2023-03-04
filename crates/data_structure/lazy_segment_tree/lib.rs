//! # 遅延評価セグメント木
//! 区間更新 $O(\log N)$、区間取得 $O(\log N)$
//!
//! ## verify
//! [ABL E](https://atcoder.jp/contests/abl/submissions/26979080)

use algebra::{Magma, MapMonoid};
use prelude::*;
use range_traits::{RangeProductMut, ToLR};

#[snippet(name = "lazy-segment-tree", doc_hidden)]
pub use lazy_segment_tree_impl::LazySegmentTree;
#[snippet(name = "lazy-segment-tree", doc_hidden)]
mod lazy_segment_tree_impl {
    use super::{Magma, MapMonoid, RangeProductMut, ToLR};

    #[derive(Clone)]
    pub struct LazySegmentTree<M: MapMonoid> {
        m: M,
        n: usize,
        log: usize,
        node: Vec<<M::Mono as Magma>::M>,
        lazy: Vec<<M::Func as Magma>::M>,
    }

    impl<M: MapMonoid> From<(usize, M)> for LazySegmentTree<M> {
        fn from((length, m): (usize, M)) -> Self {
            let n = (length + 1).next_power_of_two();
            let log = n.trailing_zeros() as usize;
            let node = vec![M::unit(); 2 * n];
            let lazy = vec![M::identity_map(); n];
            let mut tree = Self {
                m,
                n,
                log,
                node,
                lazy,
            };
            (1..n).rev().for_each(|i| tree.calc(i));
            tree
        }
    }

    /// 1-indexedで配列の内容を詰めたセグメント木を生成する
    impl<M: MapMonoid> From<(&[<M::Mono as Magma>::M], M)> for LazySegmentTree<M> {
        fn from((v, m): (&[<M::Mono as Magma>::M], M)) -> Self {
            let mut segtree = Self::from((v.len() + 1, m));
            segtree.node[segtree.n..segtree.n + v.len()].clone_from_slice(v);
            (0..segtree.n - 1).rev().for_each(|i| segtree.calc(i));
            segtree
        }
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(\log N)$
    impl<M: MapMonoid> RangeProductMut<usize> for LazySegmentTree<M> {
        type Magma = M::Mono;
        fn product<R: ToLR<usize>>(&mut self, range: R) -> <M::Mono as Magma>::M {
            let (mut l, mut r) = range.to_lr();
            if l == r {
                return M::unit();
            }
            l += self.n;
            r += self.n;
            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.propagate(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.propagate(r >> i);
                }
            }
            let (mut sml, mut smr) = (M::unit(), M::unit());
            while l < r {
                if l & 1 != 0 {
                    sml = self.m.op(&sml, &self.node[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = self.m.op(&self.node[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }
            self.m.op(&sml, &smr)
        }
    }

    impl<M: MapMonoid> LazySegmentTree<M> {
        /// 一点更新
        pub fn update_at(&mut self, mut i: usize, f: <M::Func as Magma>::M) {
            assert!(i < self.n);
            i += self.n;
            (1..=self.log).rev().for_each(|j| self.propagate(i >> j));
            self.node[i] = self.m.apply(&f, &self.node[i]);
            (1..=self.log).for_each(|j| self.calc(i >> j));
        }

        /// 区間更新 [l, r)
        pub fn update_range<R: ToLR<usize>>(&mut self, range: R, f: <M::Func as Magma>::M) {
            let (mut l, mut r) = range.to_lr();
            if l == r {
                return;
            }
            l += self.n;
            r += self.n;
            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.propagate(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.propagate((r - 1) >> i);
                }
            }
            {
                let (mut l2, mut r2) = (l, r);
                while l2 < r2 {
                    if l2 & 1 != 0 {
                        self.eval(l2, f.clone());
                        l2 += 1;
                    }
                    if r2 & 1 != 0 {
                        r2 -= 1;
                        self.eval(r2, f.clone());
                    }
                    l2 >>= 1;
                    r2 >>= 1;
                }
            }
            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.calc(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.calc((r - 1) >> i);
                }
            }
        }

        /// i番目の値を取得する
        pub fn get(&mut self, mut i: usize) -> <M::Mono as Magma>::M {
            assert!(i < self.n);
            i += self.n;
            for j in (1..=self.log).rev() {
                self.propagate(i >> j);
            }
            self.node[i].clone()
        }

        /// k番目の区間を内包する区間の値から計算する
        fn calc(&mut self, k: usize) {
            assert!(2 * k + 1 < self.node.len());
            self.node[k] = self.m.op(&self.node[2 * k], &self.node[2 * k + 1]);
        }

        /// k番目の区間の値に作用を適用する
        fn eval(&mut self, k: usize, f: <M::Func as Magma>::M) {
            self.node[k] = self.m.apply(&f, &self.node[k]);
            if k < self.n {
                self.lazy[k] = self.m.compose(&self.lazy[k], &f);
            }
        }

        /// k番目の区間に作用を適用し、その区間が含む区間に作用を伝播させる
        fn propagate(&mut self, k: usize) {
            self.eval(2 * k, self.lazy[k].clone());
            self.eval(2 * k + 1, self.lazy[k].clone());
            self.lazy[k] = M::identity_map();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use addition::Addition;
    use segment::Segment;

    // これは毎回書く(モノイドとモノイドから作用付きモノイドを作る)
    pub struct AddSum;
    impl MapMonoid for AddSum {
        type Mono = Addition<Segment<i64>>;
        type Func = Addition<i64>;

        fn apply(
            &self,
            f: &<Self::Func as Magma>::M,
            value: &<Self::Mono as Magma>::M,
        ) -> <Self::Mono as Magma>::M {
            value.clone() + *f
        }
    }

    #[test]
    fn a() {
        let n = 5;
        let m = AddSum;
        let mut segtree = LazySegmentTree::<AddSum>::from((n, m));

        for i in 1..n {
            assert_eq!(0, segtree.product(i - 1..i).value);
        }

        // [0, 0, 3, 0, 0]
        segtree.update_at(2, 3);
        assert_eq!(3, segtree.product(2..3).value);

        // [0, 2, 5, 2, 0]
        segtree.update_range(1..4, 2);
        assert_eq!(7, segtree.product(0..3).value);
    }
}
