//! セグメント木
use crate::algebra::Monoid;
use crate::*;

///////////////////////////////////////////////////////////////////

/// セグメント木(非再帰)
/// 一点更新、区間取得
///
/// 0-indexed
#[derive(Clone, Debug)]
pub struct SegmentTree<M: Monoid> {
    n: usize,
    log: usize,
    node: Vec<M::M>,
}

impl<M: Monoid> From<&Vec<M::M>> for SegmentTree<M> {
    fn from(v: &Vec<M::M>) -> Self {
        let mut segtree = Self::new(v.len() + 1);
        segtree.node[segtree.n..segtree.n + v.len()].clone_from_slice(v);
        for i in (0..segtree.n - 1).rev() {
            segtree.node[i] = M::op(&segtree.node[2 * i], &segtree.node[2 * i + 1]);
        }
        segtree
    }
}

impl<M: Monoid> SegmentTree<M> {
    /// vを初期値としてセグメント木を生成する
    /// vの長さを要素数とする
    pub fn new(n: usize) -> Self {
        let n = (n + 1).next_power_of_two();
        let log = n.trailing_zeros() as usize;
        let node = vec![M::unit(); 2 * n];
        let mut segtree = Self { n, log, node };
        for i in (0..segtree.n - 1).rev() {
            segtree.node[i] = M::op(&segtree.node[2 * i], &segtree.node[2 * i + 1]);
        }
        segtree
    }

    /// 値iをvalueに更新する
    /// ## 計算量
    /// $`O(logN)`$
    pub fn update_at(&mut self, mut i: usize, value: M::M) {
        i += self.n;
        self.node[i] = value;
        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] = M::op(&self.node[2 * i], &self.node[2 * i + 1]);
        }
    }

    /// Rangeで与えられた区間の値を取得する
    /// ## 計算量
    /// $`O(logN)`$
    pub fn prod<R>(&self, range: R) -> M::M
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r) = self.to_lr(range);
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

    /// Range to [l, r)
    fn to_lr<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
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
        assert!(l <= r && r <= self.n);
        (l, r)
    }
}

/// indexの値を取得する
/// ## 計算量
/// $`O(1)`$
impl<M: Monoid> Index<usize> for SegmentTree<M> {
    type Output = M::M;

    fn index(&self, i: usize) -> &M::M {
        &self.node[i + self.n]
    }
}

///////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::binary_operation::maximization::Maximization;

    #[test]
    fn it_works() {
        let base = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut segtree: SegmentTree<Maximization<_>> = SegmentTree::from(&base);

        for i in 0..base.len() {
            assert_eq!(base[i], segtree[i]);
        }
        assert_eq!(3, segtree.prod(0..1));
        assert_eq!(3, segtree.prod(0..2));
        assert_eq!(4, segtree.prod(0..3));
        assert_eq!(4, segtree.prod(0..4));
        assert_eq!(5, segtree.prod(0..5));
        assert_eq!(9, segtree.prod(0..6));

        segtree.update_at(3, 8);
        assert_eq!(4, segtree.prod(0..3));
        assert_eq!(8, segtree.prod(0..4));
        assert_eq!(8, segtree.prod(2..5));
    }
}
