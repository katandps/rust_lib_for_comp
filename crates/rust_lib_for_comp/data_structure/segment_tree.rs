//! # セグメント木
//! セグメント木(非再帰)
//!
use crate::algebra::*;
use crate::prelude::*;
use crate::range_traits::*;

#[codesnip::entry("segment-tree")]
pub use segment_tree_impl::SegmentTree;
#[codesnip::entry("segment-tree", include("algebra", "prelude", "range-traits"))]
mod segment_tree_impl {
    use super::{Index, Monoid, PointUpdate, RangeProductMut, ToBounds};

    #[derive(Clone, Debug)]
    pub struct SegmentTree<M: Monoid, const REV: bool = false> {
        n: usize,
        monoid: M,
        node: Vec<M::M>,
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(\log N)$
    impl<M: Monoid, const REV: bool> RangeProductMut<usize> for SegmentTree<M, REV> {
        type Magma = M;
        fn product<R: ToBounds<usize>>(&mut self, range: R) -> M::M {
            let (mut l, mut r) = range.lr();
            l += self.n;
            r += self.n;
            let (mut sum_l, mut sum_r) = (M::unit(), M::unit());
            while l < r {
                if l & 1 != 0 {
                    sum_l = self.op(&sum_l, &self.node[l].clone());
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    sum_r = self.op(&self.node[r].clone(), &sum_r);
                }
                l >>= 1;
                r >>= 1;
            }
            self.op(&sum_l, &sum_r)
        }
    }

    /// # 値iをvalueに更新する
    /// ## 計算量
    /// $O(\log N)$
    impl<M: Monoid, const REV: bool> PointUpdate<M::M> for SegmentTree<M, REV> {
        fn update_at(&mut self, mut i: usize, value: M::M) {
            i += self.n;
            self.node[i] = value;
            i >>= 1;
            while i > 0 {
                self.node[i] = self.op(&self.node[i << 1].clone(), &self.node[i << 1 | 1].clone());
                i >>= 1;
            }
        }
    }

    impl<M: Monoid, const REV: bool> SegmentTree<M, REV> {
        #[inline]
        fn op(&mut self, a: &M::M, b: &M::M) -> M::M {
            if REV {
                self.monoid.op_rev(a, b)
            } else {
                self.monoid.op(a, b)
            }
        }

        /// vを初期値としてセグメント木を生成する
        /// vの長さを要素数とする
        pub fn new(n: usize, monoid: M) -> Self {
            let node = vec![M::unit(); n << 1];
            let mut segtree = Self { n, monoid, node };
            for i in (1..segtree.n).rev() {
                segtree.node[i] = segtree.op(
                    &segtree.node[i << 1].clone(),
                    &segtree.node[i << 1 | 1].clone(),
                );
            }
            segtree
        }

        /// vを初期値としてセグメント木を生成する
        /// vの長さを要素数とする
        /// ## 計算量
        /// $O(N)$
        pub fn build(v: Vec<M::M>, monoid: M) -> Self {
            let mut segtree = Self::new(v.len(), monoid);
            segtree.node[segtree.n..segtree.n + v.len()].clone_from_slice(&v);
            for i in (1..segtree.n).rev() {
                segtree.node[i] = segtree.op(
                    &segtree.node[i << 1].clone(),
                    &segtree.node[i << 1 | 1].clone(),
                );
            }
            segtree
        }

        /// # [l, r)のモノイド積を取るときに使用するノードを列挙する
        /// 元の要素の順で左から右に並ぶ
        /// ## 計算量
        /// $O(\log N)$
        fn top_nodes(&self, l: usize, r: usize) -> Vec<usize> {
            let (mut l, mut r) = (l + self.n, r + self.n);
            let (mut l_nodes, mut r_nodes) = (Vec::new(), Vec::new());
            while l < r {
                if l & 1 != 0 {
                    l_nodes.push(l);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    r_nodes.push(r);
                }
                l >>= 1;
                r >>= 1;
            }
            r_nodes.reverse();
            l_nodes.append(&mut r_nodes);
            l_nodes
        }

        /// # 関数fが初めてtrueになる[l..r)のrを返す
        /// モノイド積に単調性がある必要がある
        /// ## 計算量
        /// $O(\log N)$
        pub fn upper_bound<F: Fn(&M::M) -> bool>(&mut self, l: usize, f: F) -> Option<usize> {
            if f(&M::unit()) {
                return Some(l);
            }
            let top_nodes = self.top_nodes(l, self.n);
            let mut cur = M::unit();
            for mut top in top_nodes {
                let t = self.op(&cur, &self.node[top].clone());
                if !f(&t) {
                    cur = t;
                } else {
                    while top < self.n {
                        top <<= 1;
                        let t = self.op(&cur, &self.node[top].clone());
                        if !f(&t) {
                            cur = t;
                            top += 1;
                        }
                    }
                    if !f(&cur) {
                        cur = self.op(&cur, &self.node[top].clone());
                        top += 1;
                    }
                    assert!(f(&cur));
                    return Some(top - self.n);
                }
            }
            None
        }

        /// # 関数fが初めてtrueになる[l..r)のlを返す
        /// モノイド積に単調性がある必要がある
        /// ## 計算量
        /// $O(\log N)$
        pub fn lower_bound<F: Fn(&M::M) -> bool>(&mut self, r: usize, f: F) -> Option<usize> {
            if f(&M::unit()) {
                return Some(r);
            }
            let top_nodes = self.top_nodes(0, r);
            let mut cur = M::unit();
            for mut top in top_nodes.into_iter().rev() {
                let t = self.op(&self.node[top].clone(), &cur);
                if !f(&t) {
                    cur = t;
                } else {
                    while top < self.n {
                        top <<= 1;
                        let t = self.op(&self.node[top].clone(), &cur);
                        if f(&t) {
                            top += 1;
                        } else {
                            cur = t;
                        }
                    }
                    return Some(top - self.n);
                }
            }
            None
        }
    }

    /// # 一点取得
    /// ## 計算量
    /// $O(1)$
    impl<M: Monoid, const REV: bool> Index<usize> for SegmentTree<M, REV> {
        type Output = M::M;
        fn index(&self, i: usize) -> &M::M {
            &self.node[i + self.n]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::binary_operation::{addition::Addition, maximization::Maximization};
    use crate::element::sequence::Sequence;

    #[test]
    fn test_non_commutative() {
        let v = (0..100).map(Sequence::new).collect::<Vec<_>>();
        let monoid = Addition::default();
        let mut segtree: SegmentTree<Addition<Sequence<i64>>> =
            SegmentTree::build(v.clone(), monoid);
        for i in 0..v.len() {
            assert_eq!(v[i], segtree[i]);
        }
        // 順序が正しいモノイド積が得られる
        for l in 0..v.len() {
            for r in l + 1..=v.len() {
                assert_eq!(
                    v[l..r].iter().fold(Sequence::zero(), |s, x| s + x.clone()),
                    segtree.product(l..r)
                )
            }
        }
    }

    #[test]
    fn test_rev_non_commutative() {
        let v = (0..100).map(Sequence::new).collect::<Vec<_>>();
        let monoid = Addition::default();
        let mut segtree: SegmentTree<Addition<Sequence<i64>>, true> =
            SegmentTree::build(v.clone(), monoid);
        for i in 0..v.len() {
            assert_eq!(v[i], segtree[i]);
        }
        // 順序が正しいモノイド積が得られる
        for l in 0..v.len() {
            for r in l + 1..=v.len() {
                assert_eq!(
                    v[l..r]
                        .iter()
                        .rev()
                        .fold(Sequence::zero(), |s, x| s + x.clone()),
                    segtree.product(l..r)
                )
            }
        }
    }

    #[test]
    fn product_test() {
        let mut base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut monoid = Maximization::default();
        let mut segtree: SegmentTree<Maximization<i64>> =
            SegmentTree::build(base.clone(), monoid.clone());

        for i in 0..base.len() {
            assert_eq!(base[i], segtree[i]);
        }

        for l in 0..base.len() {
            for r in l + 1..base.len() {
                let p = (l..r).fold(0, |x, i: usize| monoid.op(&x, &base[i]));
                assert_eq!(p, segtree.product(l..r));
            }
        }

        base[3] = 8;
        segtree.update_at(3, 8);
        for l in 0..base.len() {
            for r in l + 1..base.len() {
                let p = (l..r).fold(0, |x, i: usize| monoid.op(&x, &base[i]));
                assert_eq!(p, segtree.product(l..r));
            }
        }
    }

    #[test]
    fn binary_search_test() {
        let n = 200;
        let base = vec![1; n];
        let monoid = Addition::default();
        let mut segtree: SegmentTree<Addition<i32>> = SegmentTree::build(base.clone(), monoid);

        for l in 0..200 {
            for c in 0..200 {
                let expect = if l + c <= 200 { Some(l + c) } else { None };
                assert_eq!(expect, segtree.upper_bound(l, |&k| k >= c as i32));
            }
        }

        for r in 0..200 {
            for c in 0..200 {
                let expect = if c <= r { Some(r - c) } else { None };
                assert_eq!(expect, segtree.lower_bound(r, |&k| k >= c as i32),);
            }
        }
    }

    #[test]
    fn max_test() {
        // 不正な値を作ってしまい落ちることがあった
        let n = 524288;
        let base = vec![1_000_000_000_000; n];
        let monoid = Addition::default();
        let mut segtree: SegmentTree<Addition<i64>> = SegmentTree::build(base, monoid);
        let q = 524288;
        for _ in 0..q {
            segtree.update_at(0, 1_000_000_000_000);
        }
    }
}
