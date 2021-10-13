//! セグメント木
use crate::algebra::Monoid;
use crate::*;

///////////////////////////////////////////////////////////////////

/// セグメント木
/// 一点更新、区間取得
#[derive(Clone, Debug)]
pub struct SegmentTree<M: Monoid> {
    n: usize,
    node: Vec<M::M>,
}

impl<M: Monoid> SegmentTree<M> {
    /// vを初期値としてセグメント木を生成する
    /// vの長さを要素数とする
    pub fn new(v: &[M::M]) -> Self {
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
    /// ## 計算量
    /// $`O(logN)`$
    pub fn update_at(&mut self, mut i: usize, value: M::M) {
        i += self.n - 1;
        self.node[i] = value;
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] = M::op(&self.node[2 * i + 1], &self.node[2 * i + 2]);
        }
    }

    /// Rangeで与えられた区間の値を取得する
    /// ## 計算量
    /// $`O(logN)`$
    pub fn get_by_range<R>(&self, range: R) -> M::M
    where
        R: RangeBounds<usize>,
    {
        use Bound::*;
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(&s) => s,
            Excluded(&s) => s + 1,
        };
        let r = match range.end_bound() {
            Unbounded => self.n,
            Included(&e) => e + 1,
            Excluded(&e) => e,
        };
        self.g(l, r, None, None, None)
    }

    /// iの値を取得する
    /// ## 計算量
    /// $`O(logN)`$
    pub fn get(&self, i: usize) -> M::M {
        self.g(i, i + 1, None, None, None)
    }

    /// [start, end)の値を求める
    /// k: 自分がいるノードのインデックス
    /// l, r: 対象区間 [l, r)
    fn g(
        &self,
        start: usize,
        end: usize,
        k: Option<usize>,
        l: Option<usize>,
        r: Option<usize>,
    ) -> M::M {
        let (k, l, r) = (k.unwrap_or(0), l.unwrap_or(0), r.unwrap_or(self.n));
        if r <= start || end <= l {
            M::unit()
        } else if start <= l && r <= end {
            self.node[k].clone()
        } else {
            M::op(
                &self.g(start, end, Some(2 * k + 1), Some(l), Some((l + r) / 2)),
                &self.g(start, end, Some(2 * k + 2), Some((l + r) / 2), Some(r)),
            )
        }
    }
}

///////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::impl_monoid::max_monoid::MaxMonoid;

    #[test]
    fn it_works() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut segtree: SegmentTree<MaxMonoid<_>> = SegmentTree::new(&base);

        assert_eq!(3, segtree.get_by_range(0..1));
        assert_eq!(3, segtree.get_by_range(0..2));
        assert_eq!(4, segtree.get_by_range(0..3));
        assert_eq!(4, segtree.get_by_range(0..4));
        assert_eq!(5, segtree.get_by_range(0..5));
        assert_eq!(9, segtree.get_by_range(0..6));

        segtree.update_at(3, 8);
        assert_eq!(4, segtree.get_by_range(0..3));
        assert_eq!(8, segtree.get_by_range(0..4));
        assert_eq!(8, segtree.get_by_range(2..5));
    }
}
