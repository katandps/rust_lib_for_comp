//! # 双対セグメント木
//! 区間更新/一点取得ができるセグメント木
use crate::algebra::*;
use crate::range_traits::*;

#[codesnip::entry("dual-segment-tree")]
pub use dual_segment_tree_impl::DualSegmentTree;
#[codesnip::entry("dual-segment-tree", include("algebra", "range-traits"))]
mod dual_segment_tree_impl {
    use super::{MonoidMapping, RangeUpdate, ToBounds};

    #[derive(Clone, Debug)]
    pub struct DualSegmentTree<M: MonoidMapping> {
        n: usize,
        log: usize,
        node: Vec<M::Domain>,
        lazy: Vec<M::Mapping>,
    }

    impl<M: MonoidMapping> RangeUpdate<usize, M::M> for DualSegmentTree<M> {
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

    impl<M: MonoidMapping> DualSegmentTree<M> {
        /// vを初期値としてセグメント木を生成する(完全二分木)
        /// vの長さを要素数とする
        /// ## 計算量
        /// $O(N)$
        pub fn new(src: &[M::Domain]) -> Self {
            let n = src.len().next_power_of_two();
            Self {
                n,
                log: n.trailing_zeros() as usize,
                node: src.to_vec(),
                lazy: vec![M::unit(); n * 2],
            }
        }

        /// i番目の値を取得する
        pub fn get(&mut self, mut i: usize) -> M::Codomain {
            assert!(i < self.n);
            i += self.n;
            for j in (1..=self.log).rev() {
                self.propagate(i >> j);
            }
            M::apply(&self.lazy[i], &self.node[i - self.n])
        }

        /// k番目の区間の値に作用を適用する
        fn eval(&mut self, k: usize, f: M::M) {
            self.lazy[k] = M::op(&self.lazy[k], &f);
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
    use crate::element::affine::{Affine, Composition};

    #[test]
    fn test() {
        let a = vec![1i64, 2, 3, 4, 5];
        let mut segtree = DualSegmentTree::<Composition<i64>>::new(&a);
        segtree.update_range(0..3, Affine::new(3, 2));
        assert_eq!(5, segtree.get(0));
        assert_eq!(8, segtree.get(1));
        assert_eq!(11, segtree.get(2));
        assert_eq!(4, segtree.get(3));
        assert_eq!(5, segtree.get(4));
    }
}
