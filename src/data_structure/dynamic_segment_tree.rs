//! # 動的セグメント木
//! セグメント木よりメモリアクセスが遅いが、メモリ使用量は挿入したノードの数を上界とする。
//! データの挿入/取得が$O( \log N)$となっていることに注意。
//!
//! この動的セグメント木はBinaryTrieをベースに実装してある。
//!
//! ## verify
//! [ARC008D](https://atcoder.jp/contests/arc008/submissions/32453987)
//! [JSC2021F](https://atcoder.jp/contests/jsc2021/submissions/35010310)
use crate::prelude::*;

#[snippet(name = "dynamic-segment-tree", doc_hidden)]
#[derive(Clone, Default)]
pub struct DynamicSegmentTree<M: Monoid> {
    root: dynamic_segment_tree_impl::OptionalNode<M>,
}

#[snippet(name = "dynamic-segment-tree", doc_hidden)]
mod dynamic_segment_tree_impl {
    use super::{swap, Debug, DynamicSegmentTree, Monoid, RangeBounds, ToLR};
    type IndexType = i64;
    impl<M: Monoid> DynamicSegmentTree<M> {
        /// 最大幅を $2^{BIT_LEN}$ とする
        const BIT_LEN: i32 = 62;
        const MAX: IndexType = 1 << Self::BIT_LEN;
        /// # 値iをvalueに更新する
        /// ## 計算量
        /// $O( \log N)$
        pub fn set(&mut self, i: IndexType, value: M::M) {
            self.root.set(i, Self::BIT_LEN - 1, value);
        }
        /// # 値iに関数fを適用する
        pub fn apply<F: Fn(M::M) -> M::M>(&mut self, i: IndexType, f: F) {
            self.root.apply(i, Self::BIT_LEN - 1, f)
        }
        /// # 値iを取得する
        /// ## 計算量
        /// $O( \log N)$
        pub fn get(&self, i: IndexType) -> M::M {
            self.root.get(i, Self::BIT_LEN - 1)
        }
        /// Rangeで与えられた区間の値を取得する
        /// ## 計算量
        /// $O( \log N)$
        pub fn prod<R: RangeBounds<IndexType>>(&self, range: R) -> M::M {
            let (l, r) = range.to_lr();
            self.root.prod(l, r, 0, Self::MAX)
        }
    }
    #[derive(Clone, Debug, Default)]
    pub struct OptionalNode<M: Monoid>(Option<Node<M>>);

    #[derive(Clone, Debug, Default)]
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
        pub fn set(&mut self, idx: IndexType, bit: i32, value: M::M) {
            match self.0.as_mut() {
                Some(node) if bit < 0 => node.value = value,
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
        pub fn apply<F: Fn(M::M) -> M::M>(&mut self, idx: IndexType, bit: i32, f: F) {
            match self.0.as_mut() {
                Some(node) if bit < 0 => node.value = f(node.value.clone()),
                Some(node) => {
                    node.child_mut(idx, bit).apply(idx, bit - 1, f);
                    node.value = M::op(
                        &node.l.prod(0, 1 << 62, 0, 1 << bit),
                        &node.r.prod(0, 1 << 62, 0, 1 << bit),
                    )
                }
                None if bit < 0 => swap(self, &mut Self::new(f(M::unit()))),
                None => {
                    swap(self, &mut Self::new(M::unit()));
                    self.apply(idx, bit, f);
                }
            }
        }
        pub fn get(&self, idx: IndexType, bit: i32) -> M::M {
            match &self.0 {
                Some(node) if bit < 0 => node.value.clone(),
                Some(node) => node.child(idx, bit).get(idx, bit - 1),
                None => M::unit(),
            }
        }
        pub fn prod(&self, l: IndexType, r: IndexType, lb: IndexType, ub: IndexType) -> M::M {
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
        fn child_mut(&mut self, idx: IndexType, bit: i32) -> &mut OptionalNode<M> {
            match () {
                () if idx >> bit & 1 == 0 => self.l.as_mut(),
                _ => self.r.as_mut(),
            }
        }
        fn child(&self, idx: IndexType, bit: i32) -> &OptionalNode<M> {
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
    use crate::algebra::binary_operation::addition::Addition;
    use crate::algebra::Unital;

    #[test]
    fn test() {
        let mut segtree = DynamicSegmentTree::<Addition<i64>>::default();

        const I1: i64 = 50000000;
        const I2: i64 = 80000000000000;
        segtree.set(I1, 8);

        assert_eq!(8, segtree.get(I1));
        segtree.apply(I2, |x| x + 2);
        segtree.apply(I2, |x| x + 8);
        assert_eq!(10, segtree.get(I2));
        assert_eq!(Addition::<i64>::unit(), segtree.prod(0..I1));
        assert_eq!(8, segtree.prod(0..=I1));
        assert_eq!(8, segtree.prod(0..I2));
        assert_eq!(18, segtree.prod(0..=I2));
        assert_eq!(Addition::<i64>::unit(), segtree.prod(I1 + 1..I2));
        assert_eq!(10, segtree.prod(I1 + 1..=I2));
        assert_eq!(10, segtree.prod(I2..=I2));
        assert_eq!(10, segtree.prod(I2..I2 * 100));
        assert_eq!(Addition::<i64>::unit(), segtree.prod(I2 + 1..I2 * 100));
    }
}
