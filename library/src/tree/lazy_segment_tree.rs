//! 遅延評価セグメント木
use crate::algebra::{Magma, MapMonoid, Unital};

#[allow(unused_imports)]
use lazy_segment_tree::*;

#[allow(dead_code)]
pub mod lazy_segment_tree {
    use super::*;

    /// 遅延評価セグメント木
    /// 区間更新、区間取得
    pub struct LazySegmentTree<M: MapMonoid> {
        n: usize,
        log: usize,
        node: Vec<<M::M as Magma>::M>,
        lazy: Vec<M::F>,
    }

    impl<M: MapMonoid> From<&Vec<<M::M as Magma>::M>> for LazySegmentTree<M> {
        fn from(v: &Vec<<M::M as Magma>::M>) -> Self {
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
            let node = vec![M::M::unit(); 2 * n - 1];
            let lazy = vec![M::identity_map(); n];
            Self { n, log, node, lazy }
        }

        /// 一点更新
        pub fn update_at(&mut self, mut i: usize, f: M::F) {
            assert!(i < self.n);
            i += self.n - 1;
            for j in (0..self.n).rev() {
                self.propagate(i >> j);
            }
            self.node[i] = M::apply(&f, &self.node[i]);
            for j in 0..self.n {
                self.calc(i >> j)
            }
        }

        /// 区間更新 [l, r)
        pub fn update_range(&mut self, mut l: usize, mut r: usize, f: M::F) {
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
        pub fn get(&mut self, i: usize) -> <M::M as Magma>::M {
            assert!(i < self.n);
            let i = i + self.n;
            for j in (1..self.log).rev() {
                self.propagate(i >> j);
            }
            self.node[i].clone()
        }

        /// 区間 $`[l, r)`$ の値を取得する
        /// $`l == r`$ のときは $`unit`$ を返す
        pub fn prod(&mut self, mut l: usize, mut r: usize) -> <M::M as Magma>::M {
            assert!(l <= r && r <= self.n);
            if l == r {
                return M::M::unit();
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
            let mut sml = M::M::unit();
            let mut smr = M::M::unit();
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
            assert!(2 * k + 2 < self.n);
            self.node[k] = M::op(&self.node[2 * k + 1], &self.node[2 * k + 2]);
        }

        /// k番目の区間に作用を適用する
        fn eval(&mut self, k: usize, f: M::F) {
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
    #[test]
    fn a() {
        // dbg!(3u32.trailing_zeros());
        // dbg!(4u32.trailing_zeros());
        // dbg!(8u32.trailing_zeros());
        // dbg!(9u32.trailing_zeros());
    }
}
