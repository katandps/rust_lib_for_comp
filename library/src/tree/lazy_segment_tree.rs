//! 遅延評価セグメント木
use crate::algebra::{MapMonoid, Monoid};

#[allow(unused_imports)]
use lazy_segment_tree::*;

#[allow(dead_code)]
pub mod lazy_segment_tree {
    use super::*;

    /// 遅延評価セグメント木
    /// 区間更新、区間取得
    ///
    /// 実装内部は1-indexed
    #[derive(Debug, Clone)]
    pub struct LazySegmentTree<M: MapMonoid> {
        n: usize,
        log: usize,
        node: Vec<<<M as MapMonoid>::Mono as Monoid>::M>,
        lazy: Vec<M::Func>,
    }

    impl<M: MapMonoid> From<&Vec<<M::Mono as Monoid>::M>> for LazySegmentTree<M> {
        fn from(v: &Vec<<M::Mono as Monoid>::M>) -> Self {
            let mut segtree = Self::new(v.len());
            segtree.node[segtree.n - 1..2 * segtree.n - 1].clone_from_slice(&v);
            for i in (0..segtree.n - 1).rev() {
                segtree.calc(i);
            }
            segtree
        }
    }

    impl<M: MapMonoid> LazySegmentTree<M> {
        pub fn new(n: usize) -> Self {
            let n = (n + 1).next_power_of_two();
            let log = n.trailing_zeros() as usize;
            let node = vec![M::unit(); 2 * n];
            let lazy = vec![M::identity_map(); n];
            let mut segtree = Self { n, log, node, lazy };
            for i in (1..n).rev() {
                segtree.calc(i)
            }
            segtree
        }

        /// 一点更新
        pub fn update_at(&mut self, mut i: usize, f: M::Func) {
            assert!(i < self.n);
            i += self.n;
            for j in (1..=self.log).rev() {
                self.propagate(i >> j);
            }
            self.node[i] = M::apply(&f, &self.node[i]);
            for j in 1..=self.log {
                self.calc(i >> j)
            }
        }

        /// 区間更新 [l, r)
        pub fn update_range(&mut self, mut l: usize, mut r: usize, f: M::Func) {
            assert!(l <= r && r <= self.n);
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
                let l2 = l;
                let r2 = r;
                while l < r {
                    if l & 1 != 0 {
                        self.eval(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.eval(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
                l = l2;
                r = r2;
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
        pub fn get(&mut self, mut i: usize) -> <M::Mono as Monoid>::M {
            assert!(i < self.n);
            i += self.n;
            for j in (1..self.log).rev() {
                self.propagate(i >> j);
            }
            self.node[i].clone()
        }

        /// 区間 $`[l, r)`$ の値を取得する
        /// $`l == r`$ のときは $`unit`$ を返す
        pub fn prod(&mut self, mut l: usize, mut r: usize) -> <M::Mono as Monoid>::M {
            assert!(l <= r && r <= self.n);
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

        /// k番目の区間を内包する区間の値から計算する
        fn calc(&mut self, k: usize) {
            assert!(2 * k + 1 < self.node.len());
            self.node[k] = M::op(&self.node[2 * k], &self.node[2 * k + 1]);
        }

        /// k番目の区間の値に作用を適用する
        fn eval(&mut self, k: usize, f: M::Func) {
            self.node[k] = M::apply(&f, &self.node[k]);
            if k < self.n {
                self.lazy[k] = M::compose(&f, &self.lazy[k]);
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
    use crate::algebra::impl_map_monoid::add_sum::AddSum;
    use crate::tree::lazy_segment_tree::lazy_segment_tree::LazySegmentTree;

    #[test]
    fn a() {
        let n = 5;
        let mut segtree = LazySegmentTree::<AddSum>::new(n);

        for i in 1..n {
            assert_eq!(0, segtree.prod(i - 1, i).value);
        }

        // [0, 0, 3, 0, 0]
        segtree.update_at(2, 3);
        assert_eq!(3, segtree.prod(2, 3).value);

        // [0, 2, 5, 2, 0]
        segtree.update_range(1, 4, 2);
        assert_eq!(7, segtree.prod(0, 3).value);
    }
}
