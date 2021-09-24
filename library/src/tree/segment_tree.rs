//! セグメント木
use crate::algebra::Monoid;

///////////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use segment_tree::*;

#[allow(dead_code)]
pub mod segment_tree {
    use super::*;

    /// セグメント木
    #[derive(Clone, Debug)]
    pub struct SegmentTree<M: Monoid> {
        n: usize,
        node: Vec<M::M>,
    }

    impl<M: Monoid> SegmentTree<M> {
        /// vを初期値としてセグメント木を生成する
        /// vの長さを要素数とする
        pub fn new(v: &Vec<M::M>) -> Self {
            let n = (v.len() + 1).next_power_of_two();
            let mut node = vec![M::unit(); 2 * n - 1];
            for i in 0..v.len() {
                node[i + n - 1] = v[i].clone();
            }
            for i in (0..n - 1).rev() {
                node[i] = M::op(&node[2 * i + 1], &node[2 * i + 2]);
            }
            Self { n, node }
        }

        /// 値iをvalueに更新する
        /// 計算量 O(logN)
        pub fn update_at(&mut self, mut i: usize, value: M::M) {
            i += self.n - 1;
            self.node[i] = value;
            while i > 0 {
                i = (i - 1) / 2;
                self.node[i] = M::op(&self.node[2 * i + 1], &self.node[2 * i + 2]);
            }
        }

        /// 区間[a, b)の値を取得する
        /// 計算量 O(logN)
        pub fn get(&self, a: usize, b: usize) -> M::M {
            self.g(a, b, None, None, None)
        }

        /// k: 自分がいるノードのインデックス
        /// l, r: 対象区間 [l, r)
        fn g(
            &self,
            a: usize,
            b: usize,
            k: Option<usize>,
            l: Option<usize>,
            r: Option<usize>,
        ) -> M::M {
            let (k, l, r) = (k.unwrap_or(0), l.unwrap_or(0), r.unwrap_or(self.n));
            if r <= a || b <= l {
                M::unit()
            } else if a <= l && r <= b {
                self.node[k].clone()
            } else {
                M::op(
                    &self.g(a, b, Some(2 * k + 1), Some(l), Some((l + r) / 2)),
                    &self.g(a, b, Some(2 * k + 2), Some((l + r) / 2), Some(r)),
                )
            }
        }
    }
}

///////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::impl_monoid::max::Max;

    #[test]
    fn it_works() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut segtree: SegmentTree<Max<_>> = SegmentTree::new(&base);

        assert_eq!(3, segtree.get(0, 1));
        assert_eq!(3, segtree.get(0, 2));
        assert_eq!(4, segtree.get(0, 3));
        assert_eq!(4, segtree.get(0, 4));
        assert_eq!(5, segtree.get(0, 5));
        assert_eq!(9, segtree.get(0, 6));

        segtree.update_at(3, 8);
        assert_eq!(4, segtree.get(0, 3));
        assert_eq!(8, segtree.get(0, 4));
        assert_eq!(8, segtree.get(2, 5));
    }
}
