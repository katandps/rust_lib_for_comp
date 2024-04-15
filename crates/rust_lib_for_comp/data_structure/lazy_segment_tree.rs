//! # 遅延評価セグメント木
//! 区間更新 $O(\log N)$、区間取得 $O(\log N)$
//!

use crate::algebra::{Magma, MapMonoid, Mapping};
use crate::range_traits::{PointUpdate, RangeProductMut, RangeUpdate, ToBounds};
use crate::util::string_util::JoinTrait;

#[codesnip::entry("lazy-segment-tree")]
pub use lazy_segment_tree_impl::LazySegmentTree;
#[codesnip::entry("lazy-segment-tree", include("algebra", "range-traits", "string-util"))]
mod lazy_segment_tree_impl {
    use super::{
        JoinTrait, Magma, MapMonoid, Mapping, PointUpdate, RangeProductMut, RangeUpdate, ToBounds,
    };

    #[derive(Clone)]
    pub struct LazySegmentTree<M: MapMonoid> {
        map: M,
        n: usize,
        log: usize,
        node: Vec<<M::Map as Mapping>::Domain>,
        lazy: Vec<<M::Map as Mapping>::Mapping>,
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(\log N)$
    impl<M: MapMonoid> RangeProductMut<usize> for LazySegmentTree<M> {
        type Magma = M::Mono;
        fn product<R: ToBounds<usize>>(&mut self, range: R) -> <M::Mono as Magma>::M {
            let (mut l, mut r) = range.lr();
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
                    sml = self.map.op(&sml, &self.node[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = self.map.op(&self.node[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }
            self.map.op(&sml, &smr)
        }
    }

    impl<M: MapMonoid> PointUpdate<<M::Map as Magma>::M> for LazySegmentTree<M> {
        fn update_at(&mut self, mut i: usize, f: <M::Map as Magma>::M) {
            assert!(i < self.n);
            i += self.n;
            (1..=self.log).rev().for_each(|j| self.propagate(i >> j));
            self.node[i] = self.map.apply(&f, &self.node[i]);
            (1..=self.log).for_each(|j| self.calc(i >> j));
        }
    }

    impl<M: MapMonoid> RangeUpdate<usize, <M::Map as Magma>::M> for LazySegmentTree<M> {
        fn update_range<R: ToBounds<usize>>(&mut self, range: R, f: <M::Map as Magma>::M) {
            let (mut l, mut r) = range.lr();
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
    }

    impl<M: MapMonoid> LazySegmentTree<M> {
        /// 0-indexedで配列の内容を詰めたセグメント木を生成する
        pub fn from_slice((v, m): (&[<M::Mono as Magma>::M], M)) -> Self {
            let mut segtree = Self::from_length((v.len() + 1, m));
            segtree.node[segtree.n..segtree.n + v.len()].clone_from_slice(v);
            (0..segtree.n - 1).rev().for_each(|i| segtree.calc(i));
            segtree
        }

        pub fn from_length((length, map): (usize, M)) -> Self {
            let n = (length + 1).next_power_of_two();
            let log = n.trailing_zeros() as usize;
            let node = vec![M::unit(); 2 * n];
            let lazy = vec![M::identity_map(); n];
            let mut tree = Self {
                map,
                n,
                log,
                node,
                lazy,
            };
            (1..n).rev().for_each(|i| tree.calc(i));
            tree
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
            self.node[k] = self.map.op(&self.node[2 * k], &self.node[2 * k + 1]);
        }

        /// k番目の区間の値に作用を適用する
        fn eval(&mut self, k: usize, f: <M::Map as Magma>::M) {
            self.node[k] = self.map.apply(&f, &self.node[k]);
            if k < self.n {
                self.lazy[k] = self.map.compose(&self.lazy[k], &f);
            }
        }

        /// k番目の区間に作用を適用し、その区間が含む区間に作用を伝播させる
        fn propagate(&mut self, k: usize) {
            self.eval(2 * k, self.lazy[k].clone());
            self.eval(2 * k + 1, self.lazy[k].clone());
            self.lazy[k] = M::identity_map();
        }

        pub fn debug(&mut self) -> String {
            (0..self.n).for_each(|i| {
                self.get(i);
            });
            (0..self.n).map(|i| format!("{:?}", self.get(i))).join(" ")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::binary_operation::addition::Addition;
    use crate::algebra::mapping::add_mapping::AddMapping;
    use crate::element::section::Section;
    use crate::element::sequence::Sequence;

    // これは毎回書く(モノイドとモノイドから作用付きモノイドを作る)
    pub struct AddSum;
    impl MapMonoid for AddSum {
        type Mono = Addition<Section<i64>>;
        type Map = AddMapping<i64, Section<i64>, Section<i64>>;
    }

    #[test]
    fn a() {
        let n = 5;
        let m = AddSum;
        let mut segtree = LazySegmentTree::<AddSum>::from_length((n, m));

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

    struct RangeAddRangeSum;
    impl MapMonoid for RangeAddRangeSum {
        type Mono = Addition<Sequence<i64>>;
        type Map = Addition<Sequence<i64>>;
        fn apply(&self, f: &Sequence<i64>, value: &Sequence<i64>) -> Sequence<i64> {
            value.clone() + f.clone()
        }
    }

    #[test]
    fn seq_test() {
        let n = 5;
        let m = RangeAddRangeSum;
        let mut segtree = LazySegmentTree::<RangeAddRangeSum>::from_length((n, m));
        segtree.update_range(2..4, Sequence::new(1));
        segtree.update_range(1..3, Sequence::new(2));
        segtree.update_range(3..5, Sequence::new(3));

        assert_eq!("[] [2] [1, 2] [1, 3] [3] [] [] []", &segtree.debug())
    }
}
