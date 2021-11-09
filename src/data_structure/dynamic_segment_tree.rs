//! # 動的セグメント木
//! セグメント木よりメモリアクセスが遅いが、メモリ使用量は挿入したノードの数を上界とする。
//! データの挿入が$`O( \log N)`$となっていることに注意。
//! ## verify
//! [ARC008D](https://atcoder.jp/contests/arc008/submissions/26669109)
use crate::algebra::Monoid;
use crate::prelude::*;

#[snippet(name = "dynamic-segment-tree", doc_hidden)]
#[derive(Clone)]
pub struct DynamicSegmentTree<M: Monoid> {
    root: node::Node<M>,
}

#[snippet(name = "dynamic-segment-tree", doc_hidden)]
impl<M: Monoid> Default for DynamicSegmentTree<M> {
    fn default() -> Self {
        Self {
            root: node::Node::<M>::default(),
        }
    }
}

#[snippet(name = "dynamic-segment-tree", doc_hidden)]
impl<M: Monoid> DynamicSegmentTree<M> {
    /// 最大幅を $`2^{BIT_LEN}`$ とする
    pub const BIT_LEN: i32 = 62;
    /// 値iをvalueに更新する
    /// ## 計算量
    /// $`O( \log N)`$
    pub fn set(&mut self, i: u64, value: M::M) {
        self.root.set(i, Self::BIT_LEN - 1, value);
    }
    /// 値iをvalueに更新する
    /// ## 計算量
    /// $`O( \log N)`$
    pub fn get(&self, i: u64) -> M::M {
        self.root.get(i, Self::BIT_LEN - 1)
    }
    /// Rangeで与えられた区間の値を取得する
    /// ## 計算量
    /// $`O( \log N)`$
    pub fn prod<R>(&self, range: R) -> M::M
    where
        R: RangeBounds<usize>,
    {
        let (l, r) = Self::make_lr(range);
        self.root.prod(l, r, 0, 1 << Self::BIT_LEN)
    }

    /// Range to [l, r)
    fn make_lr<R: RangeBounds<usize>>(range: R) -> (u64, u64) {
        use Bound::*;
        let l = match range.start_bound() {
            Unbounded => 0,
            Included(&s) => s,
            Excluded(&s) => s + 1,
        };
        let r = match range.end_bound() {
            Unbounded => 1 << Self::BIT_LEN,
            Included(&e) => e + 1,
            Excluded(&e) => e,
        };
        (l as u64, r as u64)
    }
}

#[snippet(name = "dynamic-segment-tree", doc_hidden)]
mod node {
    use super::Monoid;
    #[derive(Clone, Debug)]
    pub struct Node<M: Monoid> {
        value: M::M,
        child: Vec<Option<Node<M>>>,
    }

    impl<M: Monoid> Default for Node<M> {
        fn default() -> Self {
            Self {
                value: M::unit(),
                child: vec![None, None],
            }
        }
    }

    impl<M: Monoid> Node<M> {
        pub fn set(&mut self, pos: u64, bit: i32, value: M::M) {
            if bit < 0 {
                self.value = value;
                return;
            }
            let dst = (pos >> bit & 1) as usize;
            if let Some(c) = self.child[dst].as_mut() {
                c.set(pos, bit - 1, value);
            } else {
                let mut node = Node::default();
                node.set(pos, bit - 1, value);
                self.child[dst] = Some(node);
            }
            self.value = M::op(
                &self.child[0]
                    .as_ref()
                    .map_or(M::unit(), |c| c.value.clone()),
                &self.child[1]
                    .as_ref()
                    .map_or(M::unit(), |c| c.value.clone()),
            );
        }

        pub fn get(&self, pos: u64, bit: i32) -> M::M {
            if bit < 0 {
                return self.value.clone();
            }
            let dst = (pos >> bit & 1) as usize;
            if let Some(c) = &self.child[dst] {
                c.get(pos, bit - 1)
            } else {
                M::unit()
            }
        }

        /// [left, right)のうち、[lower_bound, upper_bound)の内部にあるものをprodして返す
        pub fn prod(&self, left: u64, right: u64, lower_bound: u64, upper_bound: u64) -> M::M {
            if right <= lower_bound || upper_bound <= left {
                M::unit()
            } else if left <= lower_bound && upper_bound <= right {
                self.value.clone()
            } else {
                let mid = (lower_bound + upper_bound) >> 1;
                M::op(
                    &self.child[0]
                        .as_ref()
                        .map_or(M::unit(), |c| c.prod(left, right, lower_bound, mid)),
                    &self.child[1]
                        .as_ref()
                        .map_or(M::unit(), |c| c.prod(left, right, mid, upper_bound)),
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::binary_operation::maximization::Maximization;
    use crate::algebra::Unital;

    #[test]
    fn test() {
        let mut segtree = DynamicSegmentTree::<Maximization<i64>>::default();
        segtree.set(50000000, 8);
        segtree.set(80000000000000, 10);
        assert_eq!(Maximization::<i64>::unit(), segtree.prod(0..50000000));
        assert_eq!(8, segtree.prod(0..50000001));
        assert_eq!(8, segtree.prod(0..80000000000000));
        assert_eq!(10, segtree.prod(0..80000000000001));
        assert_eq!(
            Maximization::<i64>::unit(),
            segtree.prod(50000001..80000000000000)
        );
        assert_eq!(10, segtree.prod(50000001..80000000000001));
        assert_eq!(10, segtree.prod(80000000000000..80000000000001));
        assert_eq!(10, segtree.prod(80000000000000..10000000000000000000));
        assert_eq!(
            Maximization::<i64>::unit(),
            segtree.prod(80000000000001..10000000000000000000)
        );
    }
}
