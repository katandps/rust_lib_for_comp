//! # 動的セグメント木
//! セグメント木よりメモリアクセスが遅いが、メモリ使用量は挿入したノードの数を上界とする。
//! データの挿入が$O( \log N)$となっていることに注意。
//! ## verify
//! [ARC008D](https://atcoder.jp/contests/arc008/submissions/32453987)
use crate::prelude::*;

#[snippet(name = "dynamic-segment-tree", doc_hidden)]
#[derive(Clone, Debug, Default)]
pub struct DynamicSegmentTree<M: Monoid> {
    root: dynamic_segment_tree_impl::OptionalNode<M>,
}

#[snippet(name = "dynamic-segment-tree", doc_hidden)]
mod dynamic_segment_tree_impl {
    use super::{swap, to_lr, DynamicSegmentTree, Monoid, RangeBounds};
    impl<M: Monoid> DynamicSegmentTree<M> {
        /// 最大幅を $2^{BIT_LEN}$ とする
        const BIT_LEN: i32 = 62;
        const MAX: u64 = 1 << Self::BIT_LEN;
        /// # 値iをvalueに更新する
        /// ## 計算量
        /// $O( \log N)$
        pub fn set(&mut self, i: u64, value: M::M) {
            self.root.set(i, Self::BIT_LEN - 1, value);
        }
        /// # 値iを取得する
        /// ## 計算量
        /// $O( \log N)$
        pub fn get(&self, i: u64) -> M::M {
            self.root.get(i, Self::BIT_LEN - 1)
        }
        /// Rangeで与えられた区間の値を取得する
        /// ## 計算量
        /// $O( \log N)$
        pub fn prod<R: RangeBounds<u64>>(&self, range: R) -> M::M {
            let (l, r) = to_lr(&range, Self::MAX);
            self.root.prod(l, r, 0, Self::MAX)
        }
    }

    #[derive(Clone, Debug)]
    pub struct OptionalNode<M: Monoid>(Option<Node<M>>);

    impl<M: Monoid> Default for OptionalNode<M> {
        fn default() -> Self {
            Self(None)
        }
    }

    #[derive(Clone, Debug)]
    struct Node<M: Monoid> {
        value: M::M,
        l: Box<OptionalNode<M>>,
        r: Box<OptionalNode<M>>,
    }

    impl<M: Monoid> OptionalNode<M> {
        pub fn new(value: M::M) -> Self {
            Self(Some(Node {
                value,
                l: Box::new(Self(None)),
                r: Box::new(Self(None)),
            }))
        }

        pub fn set(&mut self, idx: u64, bit: i32, value: M::M) {
            match self.0.as_mut() {
                Some(node) => {
                    node.child_mut(idx, bit).set(idx, bit - 1, value);
                    node.value = M::op(
                        &node.l.prod(0, 1 << 62, 0, 1 << bit),
                        &node.r.prod(0, 1 << 62, 0, 1 << bit),
                    )
                }
                None if bit < 0 => swap(self, &mut Self::new(value)),
                None => {
                    swap(self, &mut Self::new(value.clone()));
                    self.set(idx, bit, value);
                }
            }
        }

        pub fn get(&self, idx: u64, bit: i32) -> M::M {
            match &self.0 {
                Some(node) if bit < 0 => node.value.clone(),
                Some(node) => node.child(idx, bit).get(idx, bit - 1),
                None => M::unit(),
            }
        }

        /// [l, r)のうち、[lb, ub)の内部にあるものをprodして返す
        pub fn prod(&self, l: u64, r: u64, lb: u64, ub: u64) -> M::M {
            match &self.0 {
                Some(node) if l <= lb && ub <= r => node.value.clone(),
                Some(node) if lb < r && l < ub => M::op(
                    &node.l.prod(l, r, lb, (lb + ub) >> 1),
                    &node.r.prod(l, r, (lb + ub) >> 1, ub),
                ),
                _ => M::unit(),
            }
        }
    }
    impl<M: Monoid> Node<M> {
        fn child_mut(&mut self, idx: u64, bit: i32) -> &mut OptionalNode<M> {
            match () {
                () if idx >> bit & 1 == 0 => self.l.as_mut(),
                _ => self.r.as_mut(),
            }
        }
        fn child(&self, idx: u64, bit: i32) -> &OptionalNode<M> {
            match () {
                () if idx >> bit & 1 == 0 => &self.l,
                _ => &self.r,
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

        assert_eq!(8, segtree.get(50000000));
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
        assert_eq!(10, segtree.prod(80000000000000..100000000000000000));
        assert_eq!(
            Maximization::<i64>::unit(),
            segtree.prod(80000000000001..100000000000000000)
        );
    }
}
