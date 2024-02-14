//! # 完全64分木
//! $n<2^{18} = 262143$の範囲でsetのようなInterfaceを提供する 高速・省メモリ
//!
//! 集合に存在するかどうかをBitSetの入れ子のような形で表現する
//!

#[codesnip::entry("complete_64_part_tree")]
pub use complete_64_part_tree_impl::{Complete64PartTree, WordAryTree};

#[codesnip::entry("complete_64_part_tree")]
mod complete_64_part_tree_impl {

    const WORD_SIZE: usize = 64;
    const WORD_SIZE_2: usize = WORD_SIZE * WORD_SIZE;
    const WORD_LOG: usize = 6;

    #[derive(Clone)]
    pub struct Complete64PartTree();

    pub trait WordAryTree {
        fn is_empty(&self) -> bool;
        fn insert(&mut self, x: u64) -> bool;
        /// # keyを消す
        /// 存在していたときはtrue
        fn remove(&mut self, x: u64) -> bool;
        /// # 値の存在判定
        fn contains(&self, x: u64) -> bool;
        /// # 最大値を返す
        fn max(&self) -> Option<u64>;
        /// # 最大値を消費して返す
        fn pop_max(&mut self) -> Option<u64> {
            let max = self.max();
            if let Some(m) = max {
                self.remove(m);
            }
            max
        }
        /// # 最小値を返す
        fn min(&self) -> Option<u64>;
        /// # 最小値を消費して返す
        fn pop_min(&mut self) -> Option<u64> {
            let min = self.min();
            if let Some(m) = min {
                self.remove(m);
            }
            min
        }
        /// # xより大きい値があればその最小値を返す
        fn next(&self, x: u64) -> Option<u64>;
        /// # xより小さい値があればその最大値を返す
        fn prev(&self, x: u64) -> Option<u64>;
    }

    impl Complete64PartTree {
        pub fn build(limit: u64) -> Box<dyn WordAryTree> {
            if limit < 1u64 << WORD_LOG {
                Box::new(Depth1Tree::new())
            } else if limit < 1u64 << (WORD_LOG * 2) {
                Box::new(Depth2Tree::new(limit))
            } else if limit < 1u64 << (WORD_LOG * 3) {
                Box::new(Depth3Tree::new(limit))
            } else {
                Box::new(Depth4Tree::new(limit))
            }
        }
    }

    trait HasNodes {
        fn nodes(&self) -> &[Node];
        fn nodes_mut(&mut self) -> &mut [Node];
        fn top(&self) -> &dyn WordAryTree;
        fn top_mut(&mut self) -> &mut dyn WordAryTree;
    }

    impl<T: HasNodes> WordAryTree for T {
        #[inline]
        fn is_empty(&self) -> bool {
            self.top().is_empty()
        }
        #[inline]
        fn insert(&mut self, x: u64) -> bool {
            self.top_mut().insert(x >> WORD_LOG);
            self.nodes_mut()[(x >> WORD_LOG) as usize].add(x & 63)
        }
        #[inline]
        fn contains(&self, x: u64) -> bool {
            self.nodes()[(x >> WORD_LOG) as usize].contains(x & 63)
        }
        #[inline]
        fn remove(&mut self, x: u64) -> bool {
            let del = self.nodes_mut()[x as usize >> WORD_LOG].remove(x & 63);
            if del && self.nodes()[x as usize >> WORD_LOG].is_empty() {
                self.top_mut().remove(x >> WORD_LOG);
            }
            del
        }
        #[inline]
        fn max(&self) -> Option<u64> {
            self.top().max().and_then(|m| {
                self.nodes()[m as usize]
                    .max()
                    .map(|m2| m2 + (m << WORD_LOG))
            })
        }
        #[inline]
        fn pop_max(&mut self) -> Option<u64> {
            let max = self.max();
            if let Some(m) = max {
                self.remove(m);
            }
            max
        }
        #[inline]
        fn min(&self) -> Option<u64> {
            self.top().min().and_then(|m| {
                self.nodes()[m as usize]
                    .min()
                    .map(|m2| m2 + (m << WORD_LOG))
            })
        }
        #[inline]
        fn pop_min(&mut self) -> Option<u64> {
            let min = self.min();
            if let Some(m) = min {
                self.remove(m);
            }
            min
        }
        #[inline]
        fn next(&self, x: u64) -> Option<u64> {
            if let Some(a) = self.nodes()[x as usize >> WORD_LOG].next(x & 63) {
                Some((x >> WORD_LOG << WORD_LOG) + a)
            } else {
                self.top().next(x >> WORD_LOG).and_then(|m| {
                    self.nodes()[m as usize]
                        .min()
                        .map(|m2| m2 + (m << WORD_LOG))
                })
            }
        }
        #[inline]
        fn prev(&self, x: u64) -> Option<u64> {
            if let Some(a) = self.nodes()[x as usize >> WORD_LOG].prev(x & 63) {
                Some((x >> WORD_LOG << WORD_LOG) + a)
            } else {
                self.top().prev(x >> WORD_LOG).and_then(|m| {
                    self.nodes()[m as usize]
                        .max()
                        .map(|m2| m2 + (m << WORD_LOG))
                })
            }
        }
    }
    #[derive(Clone, Copy, Default)]
    struct Node(u64);

    impl Node {
        #[inline]
        fn is_empty(&self) -> bool {
            self.0 == 0
        }
        #[inline]
        fn contains(&self, x: u64) -> bool {
            self.0 >> x & 1 == 1
        }
        #[inline]
        fn add(&mut self, x: u64) -> bool {
            if self.contains(x) {
                false
            } else {
                self.0 |= 1 << x;
                true
            }
        }
        #[inline]
        fn remove(&mut self, x: u64) -> bool {
            if self.contains(x) {
                self.0 ^= 1 << x;
                true
            } else {
                false
            }
        }
        #[inline]
        fn max(&self) -> Option<u64> {
            if self.0 == 0 {
                None
            } else {
                Some(WORD_SIZE as u64 - 1 - self.0.leading_zeros() as u64)
            }
        }
        #[inline]
        fn min(&self) -> Option<u64> {
            if self.0 == 0 {
                None
            } else {
                Some(self.0.trailing_zeros() as u64)
            }
        }
        #[inline]
        fn next(&self, x: u64) -> Option<u64> {
            let mask = !((1 << x) - 1) - (1 << x);
            Self(self.0 & mask).min()
        }
        #[inline]
        fn prev(&self, x: u64) -> Option<u64> {
            let mask = (1 << x) - 1;
            Self(self.0 & mask).max()
        }
    }
    #[derive(Clone)]
    pub struct Depth1Tree {
        node: Node,
    }
    impl Depth1Tree {
        fn new() -> Self {
            Self {
                node: Node::default(),
            }
        }
    }

    impl WordAryTree for Depth1Tree {
        #[inline]
        fn is_empty(&self) -> bool {
            self.node.is_empty()
        }
        #[inline]
        fn insert(&mut self, x: u64) -> bool {
            assert!(x < 64);
            self.node.add(x)
        }
        #[inline]
        fn contains(&self, x: u64) -> bool {
            assert!(x < 64);
            self.node.contains(x)
        }
        #[inline]
        fn remove(&mut self, x: u64) -> bool {
            assert!(x < 64);
            self.node.remove(x)
        }
        #[inline]
        fn max(&self) -> Option<u64> {
            self.node.max()
        }
        #[inline]
        fn min(&self) -> Option<u64> {
            self.node.min()
        }
        #[inline]
        fn next(&self, x: u64) -> Option<u64> {
            self.node.next(x)
        }
        #[inline]
        fn prev(&self, x: u64) -> Option<u64> {
            self.node.prev(x)
        }
    }
    #[derive(Clone)]
    pub struct Depth2Tree {
        top: Depth1Tree,
        nodes: Vec<Node>,
    }

    impl Depth2Tree {
        fn new(limit: u64) -> Self {
            Self {
                top: Depth1Tree::new(),
                nodes: vec![Node::default(); limit as usize >> (WORD_LOG - 1)],
            }
        }
    }
    #[derive(Clone)]
    pub struct Depth3Tree {
        top: Depth2Tree,
        nodes: Vec<Node>,
    }

    impl Depth3Tree {
        fn new(limit: u64) -> Self {
            Self {
                top: Depth2Tree::new(limit >> (WORD_LOG - 1)),
                nodes: vec![Node::default(); WORD_SIZE_2],
            }
        }
    }
    #[derive(Clone)]
    pub struct Depth4Tree {
        top: Depth3Tree,
        nodes: Vec<Node>,
    }
    impl Depth4Tree {
        fn new(limit: u64) -> Self {
            Self {
                top: Depth3Tree::new(limit >> (WORD_LOG - 1)),
                nodes: vec![Node::default(); limit as usize >> (WORD_LOG - 1)],
            }
        }
    }

    macro_rules! impl_has_nodes {
        ($($ty:ty),*) => {
            $(
                impl HasNodes for $ty {
                    #[inline] fn nodes(&self) -> &[Node] { &self.nodes }
                    #[inline] fn nodes_mut(&mut self) -> &mut [Node] { &mut self.nodes }
                    #[inline] fn top(&self) -> &dyn WordAryTree { &self.top }
                    #[inline] fn top_mut(&mut self) -> &mut dyn WordAryTree { &mut self.top }
                }
            )*
        };
    }
    impl_has_nodes!(Depth2Tree, Depth3Tree, Depth4Tree);
}

#[cfg(test)]
mod test {
    use super::Complete64PartTree;

    #[test]
    fn test_dim1() {
        let mut tree = Complete64PartTree::build(63);
        assert!(tree.insert(5));
        assert_eq!(Some(5), tree.max());
        tree.remove(5);
        assert_eq!(None, tree.max());
        tree.insert(5);
        assert!(!tree.insert(5));
        assert!(tree.insert(7));
        assert_eq!(None, tree.next(7));
        assert_eq!(Some(7), tree.next(6));

        assert!(tree.insert(15));
        assert_eq!(Some(15), tree.max());
        assert!(tree.insert(3));
        assert_eq!(Some(15), tree.max());
        assert!(tree.remove(3));
        assert!(!tree.remove(3));
        assert!(tree.remove(5));
        assert!(tree.remove(15));
        assert_eq!(Some(7), tree.max());
        assert!(tree.remove(7));
        assert!(!tree.remove(15));
        assert_eq!(None, tree.max());
    }

    #[test]
    fn test_dim2() {
        let mut tree = Complete64PartTree::build(4095);
        assert!(tree.insert(4000));
        assert!(tree.contains(4000));
        assert!(tree.insert(255));
        assert!(tree.contains(255));
        assert_eq!(Some(4000), tree.max());
        assert_eq!(Some(255), tree.min());
        assert_eq!(None, tree.next(4000));
        assert_eq!(Some(4000), tree.next(255));
        assert_eq!(Some(255), tree.next(254));
        assert_eq!(None, tree.prev(255));
        assert_eq!(Some(255), tree.prev(256));
        assert_eq!(Some(4000), tree.prev(4001));
        assert!(tree.remove(4000));
        assert_eq!(Some(255), tree.max());
        assert!(tree.remove(255));
        assert!(!tree.remove(4000));
        assert_eq!(None, tree.max());
    }

    #[test]
    fn test_dim3() {
        let mut tree = Complete64PartTree::build(261000);
        assert!(tree.insert(40000));
        assert!(tree.contains(40000));
        assert!(tree.insert(2550));
        assert!(tree.contains(2550));
        assert_eq!(Some(40000), tree.max());
        assert_eq!(Some(2550), tree.min());
        assert!(tree.remove(40000));
        assert_eq!(Some(2550), tree.max());
        assert!(tree.remove(2550));
        assert!(!tree.remove(40000));
        assert_eq!(None, tree.max());
    }

    #[test]
    fn test_dim4() {
        let mut tree = Complete64PartTree::build(16000000);
        assert!(tree.insert(4000000));
        assert!(tree.contains(4000000));
        assert!(tree.insert(255000));
        assert!(tree.contains(255000));
        assert_eq!(Some(4000000), tree.max());
        assert_eq!(Some(255000), tree.min());
        assert!(tree.remove(4000000));
        assert_eq!(Some(255000), tree.max());
        assert!(tree.remove(255000));
        assert!(!tree.remove(4000000));
        assert_eq!(None, tree.max());
    }
}
