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
    use super::{Debug, Formatter, Monoid, RangeProduct, ToBounds};
    type IndexType = i64;

    type NodeId = u32;
    const EMPTY_NODE: NodeId = !0;
    /// 最大幅を $2^{BIT_LEN}$ とする
    const BIT_LEN: i8 = 62;
    #[derive(Clone)]
    pub struct DynamicSegmentTree<M: Monoid> {
        nodes: Vec<Node<M>>,
        bit_len: i8,
        root: usize,
    }

    impl<M: Monoid> Default for DynamicSegmentTree<M> {
        fn default() -> Self {
            Self {
                nodes: vec![Node::default()],
                bit_len: BIT_LEN,
                root: 0,
            }
        }
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(\log N)$
    impl<M: Monoid> RangeProduct<IndexType> for DynamicSegmentTree<M> {
        type Magma = M;
        fn product<R: ToBounds<IndexType>>(&self, range: R) -> M::M {
            let (l, r) = range.lr();
            // stack[(node, nodeのlower_bound, nodeのupper_bound)]
            let mut stack = vec![(self.root as NodeId, 0, 1 << self.bit_len)];
            let mut ret = M::unit();
            while let Some((node_id, lb, ub)) = stack.pop() {
                let Some(node) = self.node(node_id) else {
                    continue;
                };
                // todo 非可換クエリに対応する
                if l <= lb && ub <= r {
                    ret = M::op(&ret, &node.value);
                } else if lb < r && l < ub {
                    stack.push((node.children[0], lb, (lb + ub) >> 1));
                    stack.push((node.children[1], (lb + ub) >> 1, ub));
                }
            }
            ret
        }
    }

    impl<M: Monoid> DynamicSegmentTree<M> {
        pub fn new(max: i64) -> Self {
            let bit_len = ((max as u64).next_power_of_two()).ilog2() as i8;
            Self {
                nodes: vec![Node::default()],
                bit_len,
                root: 0,
            }
        }
        fn node(&self, id: NodeId) -> Option<&Node<M>> {
            self.nodes.get(id as usize)
        }
        fn node_mut(&mut self, id: NodeId) -> Option<&mut Node<M>> {
            self.nodes.get_mut(id as usize)
        }

        /// # 値iをvalueに更新する
        /// ## 計算量
        /// $O( \log N)$
        pub fn set(&mut self, i: IndexType, value: M::M) {
            self.apply(i, |_| value.clone())
        }
        /// # 値iに関数fを適用する
        pub fn apply<F: Fn(M::M) -> M::M>(&mut self, i: IndexType, f: F) {
            let mut stack = Vec::new();
            stack.push((self.root as NodeId, self.bit_len));
            while let Some((v, b)) = stack.pop() {
                if v < !v {
                    stack.push((!v, b));
                    if b > 0 {
                        let child = i as usize >> (b - 1) & 1;
                        let child_is_empty = if let Some(node) = self.node(v) {
                            node.children[child] == EMPTY_NODE
                        } else {
                            false
                        };
                        if child_is_empty {
                            let id = self.nodes.len() as NodeId;
                            if let Some(node) = self.node_mut(v) {
                                node.children[child] = id;
                            }
                            self.nodes.push(Node::default());
                        }
                        if let Some(node) = self.node(v) {
                            stack.push((node.children[child], b - 1))
                        }
                    } else {
                        if let Some(node) = self.node_mut(v) {
                            node.value = f(node.value.clone());
                        } else {
                            unreachable!()
                        }
                    }
                } else {
                    let v = !v;
                    let value = if let Some(node) = self.node(v) {
                        match (self.node(node.children[0]), self.node(node.children[1])) {
                            (Some(l), Some(r)) => M::op(&l.value, &r.value),
                            (Some(l), None) => l.value.clone(),
                            (_, Some(r)) => r.value.clone(),
                            (_, _) => node.value.clone(),
                        }
                    } else {
                        M::unit()
                    };
                    if let Some(node) = self.node_mut(v) {
                        node.value = value
                    }
                }
            }
        }
        /// # 値iを取得する
        /// ## 計算量
        /// $O( \log N)$
        pub fn get(&self, i: IndexType) -> M::M {
            let mut current = self.root as NodeId;
            let mut b = self.bit_len;
            while b > 0 {
                current = match self.node(current) {
                    Some(node) => node.children[i as usize >> (b - 1) & 1],
                    None => return M::unit(),
                };
                b -= 1;
            }
            self.node(current).unwrap_or(&Node::default()).value.clone()
        }
    }
    impl<M: Monoid> Debug for DynamicSegmentTree<M> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            let mut q = vec![(self.root as u32, 0, 1 << self.bit_len)];
            let mut result = Vec::new();
            while let Some((node_id, lb, ub)) = q.pop() {
                if let Some(node) = self.node(node_id) {
                    result.push((lb, ub, node.value.clone()));
                    q.push((node.children[0], lb, (lb + ub) >> 1));
                    q.push((node.children[1], (lb + ub) >> 1, ub));
                }
            }
            result.sort_by_key(|(lb, ub, _value)| (*lb, *ub));
            write!(
                f,
                "{}",
                result
                    .into_iter()
                    .fold("".to_string(), |a, (lb, ub, value)| format!(
                        "{}\n{}..{}: {:?}",
                        a, lb, ub, value
                    ))
            )
        }
    }
    #[derive(Clone, Debug)]
    struct Node<M: Monoid> {
        value: M::M,
        children: [NodeId; 2],
    }
    impl<M: Monoid> Default for Node<M> {
        fn default() -> Self {
            Node {
                value: M::unit(),
                children: [EMPTY_NODE, EMPTY_NODE],
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use addition::Addition;

    #[test]
    fn test() {
        let mut segtree = DynamicSegmentTree::<Addition<i64>>::new(10);

        const I1: i64 = 5;
        const I2: i64 = 8;
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
