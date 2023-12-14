//! # 双対セグメント木
//! 区間更新/一点取得ができるセグメント木
use algebra::*;
use prelude::*;
use range_traits::*;

#[codesnip::entry("dual-segment-tree", doc_hidden)]
pub use dual_segment_tree_impl::DualSegmentTree;
#[codesnip::entry("dual-segment-tree", doc_hidden)]
mod dual_segment_tree_impl {
    use super::{MonoidOperation, RangeUpdate, ToBounds};

    #[derive(Clone, Debug)]
    pub struct DualSegmentTree<M: MonoidOperation> {
        n: usize,
        operation_monoid: M,
        log: usize,
        node: Vec<M::V>,
        lazy: Vec<M::M>,
    }

    impl<M: MonoidOperation> RangeUpdate<usize, M::M> for DualSegmentTree<M> {
        fn update_range<R: ToBounds<usize>>(&mut self, range: R, f: M::M) {
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
        }
    }

    impl<M: MonoidOperation> DualSegmentTree<M> {
        /// vを初期値としてセグメント木を生成する(完全二分木)
        /// vの長さを要素数とする
        /// ## 計算量
        /// $O(N)$
        pub fn new(src: &[M::V], operation_monoid: M) -> Self {
            let n = src.len().next_power_of_two();
            Self {
                n,
                operation_monoid,
                log: n.trailing_zeros() as usize,
                node: src.to_vec(),
                lazy: vec![M::unit(); n * 2],
            }
        }

        /// i番目の値を取得する
        pub fn get(&mut self, mut i: usize) -> M::V {
            assert!(i < self.n);
            i += self.n;
            for j in (1..=self.log).rev() {
                self.propagate(i >> j);
            }
            self.node[i - self.n].clone()
        }

        /// k番目の区間の値に作用を適用する
        fn eval(&mut self, k: usize, f: M::M) {
            if k >= self.n && k - self.n < self.node.len() {
                self.node[k - self.n] = self.operation_monoid.apply(&f, &self.node[k - self.n]);
            }
            if k < self.n {
                self.lazy[k] = M::op(&self.lazy[k], &f);
            }
        }

        /// k番目の区間に作用を適用し、その区間が含む区間に作用を伝播させる
        fn propagate(&mut self, k: usize) {
            self.eval(2 * k, self.lazy[k].clone());
            self.eval(2 * k + 1, self.lazy[k].clone());
            self.lazy[k] = M::unit();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use affine::{Affine, Composition};

    #[test]
    fn test() {
        let a = vec![1i64, 2, 3, 4, 5];
        let mut segtree = DualSegmentTree::new(&a, Composition::default());
        segtree.update_range(0..3, Affine::new(3, 2));
        assert_eq!(5, segtree.get(0));
        assert_eq!(8, segtree.get(1));
        assert_eq!(11, segtree.get(2));
        assert_eq!(4, segtree.get(3));
        assert_eq!(5, segtree.get(4));
    }
}
