//! # 動的セグメント木
//! セグメント木よりメモリアクセスが遅いが、メモリ使用量は挿入したノードの数を上界とする。
//! データの挿入/取得が$O( \log N)$となっていることに注意。
//!
//! この動的セグメント木はBinaryTrieをベースに実装してある。
//!
//! ## verify
//! [ARC008D](https://atcoder.jp/contests/arc008/submissions/32453987)
//! [JSC2021F](https://atcoder.jp/contests/jsc2021/submissions/35010310)
use algebra::*;
use prelude::*;
use range_traits::*;

#[snippet(name = "dynamic-segment-tree", doc_hidden)]
pub use dynamic_segment_tree_impl::DynamicSegmentTree;
#[snippet(name = "dynamic-segment-tree", doc_hidden)]
mod dynamic_segment_tree_impl {
    use super::{swap, Debug, Monoid, RangeBounds, RangeProduct, ToLR};
    type IndexType = i64;
    type Bit = i32;

    #[derive(Clone, Default)]
    pub struct DynamicSegmentTree<M: Monoid> {
        root: OptionalNode<M>,
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(\log N)$
    impl<M: Monoid> RangeProduct<IndexType> for DynamicSegmentTree<M> {
        type Magma = M;
        fn product<R: RangeBounds<IndexType>>(&self, range: R) -> M::M {
            let (l, r) = range.to_lr();
            self.root.prod(l, r, 0, Self::MAX)
        }
    }

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
    }
    #[derive(Clone, Debug, Default)]
    pub struct OptionalNode<M: Monoid>(Option<Node<M>>);

    impl<M: Monoid> OptionalNode<M> {
        fn new(value: M::M) -> Self {
            Self(Some(Node {
                value,
                children: vec![Self(None), Self(None)],
            }))
        }
        fn set(&mut self, idx: IndexType, bit: i32, value: M::M) {
            match self.0.as_mut() {
                Some(node) if bit < 0 => node.value = value,
                Some(node) => {
                    node.child_mut(idx, bit).set(idx, bit - 1, value);
                    node.value = M::op(
                        &node.left().prod(0, 1 << 62, 0, 1 << bit),
                        &node.right().prod(0, 1 << 62, 0, 1 << bit),
                    )
                }
                None if bit < 0 => swap(self, &mut Self::new(value)),
                None => {
                    swap(self, &mut Self::new(value.clone()));
                    self.set(idx, bit, value);
                }
            }
        }
        fn apply<F: Fn(M::M) -> M::M>(&mut self, idx: IndexType, bit: i32, f: F) {
            match self.0.as_mut() {
                Some(node) if bit < 0 => node.value = f(node.value.clone()),
                Some(node) => {
                    node.child_mut(idx, bit).apply(idx, bit - 1, f);
                    node.value = M::op(
                        &node.left().prod(0, 1 << 62, 0, 1 << bit),
                        &node.right().prod(0, 1 << 62, 0, 1 << bit),
                    )
                }
                None if bit < 0 => swap(self, &mut Self::new(f(M::unit()))),
                None => {
                    swap(self, &mut Self::new(M::unit()));
                    self.apply(idx, bit, f);
                }
            }
        }
        fn get(&self, idx: IndexType, bit: i32) -> M::M {
            match &self.0 {
                Some(node) if bit < 0 => node.value.clone(),
                Some(node) => node.child(idx, bit).get(idx, bit - 1),
                None => M::unit(),
            }
        }
        fn prod(&self, l: IndexType, r: IndexType, lb: IndexType, ub: IndexType) -> M::M {
            match &self.0 {
                Some(node) if l <= lb && ub <= r => node.value.clone(),
                Some(node) if lb < r && l < ub => M::op(
                    &node.left().prod(l, r, lb, (lb + ub) >> 1),
                    &node.right().prod(l, r, (lb + ub) >> 1, ub),
                ),
                _ => M::unit(),
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    struct Node<M: Monoid> {
        value: M::M,
        children: Vec<OptionalNode<M>>,
    }
    impl<M: Monoid> Node<M> {
        #[inline]
        fn child_mut(&mut self, idx: IndexType, bit: Bit) -> &mut OptionalNode<M> {
            match () {
                () if idx >> bit & 1 == 0 => self.left_mut(),
                _ => self.right_mut(),
            }
        }
        #[inline]
        fn child(&self, idx: IndexType, bit: Bit) -> &OptionalNode<M> {
            match () {
                () if idx >> bit & 1 == 0 => self.left(),
                _ => self.right(),
            }
        }
        #[inline]
        fn left(&self) -> &OptionalNode<M> {
            unsafe { self.children.get_unchecked(0) }
        }
        #[inline]
        fn left_mut(&mut self) -> &mut OptionalNode<M> {
            unsafe { self.children.get_unchecked_mut(0) }
        }
        #[inline]
        fn right(&self) -> &OptionalNode<M> {
            unsafe { self.children.get_unchecked(1) }
        }
        #[inline]
        fn right_mut(&mut self) -> &mut OptionalNode<M> {
            unsafe { self.children.get_unchecked_mut(1) }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use addition::Addition;

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
        assert_eq!(Addition::<i64>::unit(), segtree.product(0..I1));
        assert_eq!(8, segtree.product(0..=I1));
        assert_eq!(8, segtree.product(0..I2));
        assert_eq!(18, segtree.product(0..=I2));
        assert_eq!(Addition::<i64>::unit(), segtree.product(I1 + 1..I2));
        assert_eq!(10, segtree.product(I1 + 1..=I2));
        assert_eq!(10, segtree.product(I2..=I2));
        assert_eq!(10, segtree.product(I2..I2 * 100));
        assert_eq!(Addition::<i64>::unit(), segtree.product(I2 + 1..I2 * 100));
    }
}
