//! # 完全64分木
//! $n<2^{18} = 262143$の範囲でsetのようなInterfaceを提供する 高速・省メモリ
//!
//! 集合に存在するかどうかをBitSetの入れ子のような形で表現する
//!
//! ## todo
//! implement prev/next

use prelude::*;

#[snippet(name = "complete_64_part_tree", doc_hidden)]
pub use complete_64_part_tree_impl::Complete64PartTree;

#[snippet(name = "complete_64_part_tree", doc_hidden)]
mod complete_64_part_tree_impl {

    const WORD_SIZE: usize = 64;
    const WORD_SIZE_2: usize = WORD_SIZE * WORD_SIZE;
    const WORD_SIZE_3: usize = WORD_SIZE * WORD_SIZE * WORD_SIZE;
    const WORD_LOG: usize = 6;

    #[derive(Clone)]
    pub enum Complete64PartTree {
        Depth1(Depth1Tree),
        Depth2(Box<Depth2Tree>),
        Depth3(Box<Depth3Tree>),
        Depth4(Box<Depth4Tree>),
    }

    impl Complete64PartTree {
        pub fn new(limit: u64) -> Self {
            if limit < 1u64 << WORD_LOG {
                Self::Depth1(Depth1Tree::new())
            } else if limit < 1u64 << (WORD_LOG * 2) {
                Self::Depth2(Box::new(Depth2Tree::new()))
            } else if limit < 1u64 << (WORD_LOG * 3) {
                Self::Depth3(Box::new(Depth3Tree::new()))
            } else {
                Self::Depth4(Box::new(Depth4Tree::new()))
            }
        }
        /// # treeに $x$ を追加する
        /// すでに存在していた時はfalse
        pub fn insert(&mut self, x: u64) -> bool {
            match self {
                Self::Depth1(tree) => tree.insert(x),
                Self::Depth2(tree) => tree.insert(x),
                Self::Depth3(tree) => tree.insert(x),
                Self::Depth4(tree) => tree.insert(x),
            }
        }

        /// # keyを消す
        /// 存在していたときはtrue
        pub fn remove(&mut self, x: u64) -> bool {
            match self {
                Self::Depth1(tree) => tree.remove(x),
                Self::Depth2(tree) => tree.remove(x),
                Self::Depth3(tree) => tree.remove(x),
                Self::Depth4(tree) => tree.remove(x),
            }
        }

        /// # 値の存在判定
        pub fn contains(&self, x: u64) -> bool {
            match self {
                Self::Depth1(tree) => tree.contains(x),
                Self::Depth2(tree) => tree.contains(x),
                Self::Depth3(tree) => tree.contains(x),
                Self::Depth4(tree) => tree.contains(x),
            }
        }

        /// # 最大値を返す
        pub fn max(&self) -> Option<u64> {
            match self {
                Self::Depth1(tree) => tree.max(),
                Self::Depth2(tree) => tree.max(),
                Self::Depth3(tree) => tree.max(),
                Self::Depth4(tree) => tree.max(),
            }
        }

        /// # 最大値を消費して返す
        pub fn pop_max(&mut self) -> Option<u64> {
            match self {
                Self::Depth1(tree) => tree.pop_max(),
                Self::Depth2(tree) => tree.pop_max(),
                Self::Depth3(tree) => tree.pop_max(),
                Self::Depth4(tree) => tree.pop_max(),
            }
        }

        /// # 最小値を返す
        pub fn min(&self) -> Option<u64> {
            match self {
                Self::Depth1(tree) => tree.min(),
                Self::Depth2(tree) => tree.min(),
                Self::Depth3(tree) => tree.min(),
                Self::Depth4(tree) => tree.min(),
            }
        }

        /// # 最小値を消費して返す
        pub fn pop_min(&mut self) -> Option<u64> {
            match self {
                Self::Depth1(tree) => tree.pop_min(),
                Self::Depth2(tree) => tree.pop_min(),
                Self::Depth3(tree) => tree.pop_min(),
                Self::Depth4(tree) => tree.pop_min(),
            }
        }

        /// # xより大きい値があればその最小値を返す
        pub fn next(&self, x: u64) -> Option<u64> {
            match self {
                Self::Depth1(tree) => tree.next(x),
                Self::Depth2(tree) => tree.next(x),
                Self::Depth3(tree) => tree.next(x),
                Self::Depth4(tree) => tree.next(x),
            }
        }

        /// # xより小さい値があればその最大値を返す
        pub fn prev(&self, x: u64) -> Option<u64> {
            match self {
                Self::Depth1(tree) => tree.prev(x),
                Self::Depth2(tree) => tree.prev(x),
                Self::Depth3(tree) => tree.prev(x),
                Self::Depth4(tree) => tree.prev(x),
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
        fn insert(&mut self, x: u64) -> bool {
            assert!(x < 64);
            self.node.add(x)
        }
        fn contains(&self, x: u64) -> bool {
            assert!(x < 64);
            self.node.contains(x)
        }
        fn remove(&mut self, x: u64) -> bool {
            assert!(x < 64);
            self.node.remove(x)
        }
        fn max(&self) -> Option<u64> {
            self.node.max()
        }
        fn pop_max(&mut self) -> Option<u64> {
            let max = self.max();
            if let Some(m) = max {
                self.remove(m);
            }
            max
        }
        fn min(&self) -> Option<u64> {
            self.node.min()
        }
        fn pop_min(&mut self) -> Option<u64> {
            let min = self.min();
            if let Some(m) = min {
                self.remove(m);
            }
            min
        }
        fn next(&self, x: u64) -> Option<u64> {
            self.node.next(x)
        }
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
        fn new() -> Self {
            Self {
                top: Depth1Tree::new(),
                nodes: vec![Node::default(); WORD_SIZE],
            }
        }
        fn insert(&mut self, x: u64) -> bool {
            assert!(x < 1 << 12);
            self.top.insert(x >> WORD_LOG);
            self.nodes[x as usize >> WORD_LOG].add(x & 63)
        }
        fn contains(&self, x: u64) -> bool {
            assert!(x < 1 << 12);
            self.nodes[x as usize >> WORD_LOG].contains(x & 63)
        }
        fn remove(&mut self, x: u64) -> bool {
            assert!(x < 1 << 12);
            let del = self.nodes[x as usize >> WORD_LOG].remove(x & 63);
            if del && self.nodes[x as usize >> WORD_LOG].is_empty() {
                self.top.remove(x >> WORD_LOG);
            }
            del
        }
        fn max(&self) -> Option<u64> {
            self.top
                .max()
                .and_then(|m| self.nodes[m as usize].max().map(|m2| m2 + (m << WORD_LOG)))
        }
        fn pop_max(&mut self) -> Option<u64> {
            let max = self.max();
            if let Some(m) = max {
                self.remove(m);
            }
            max
        }
        fn min(&self) -> Option<u64> {
            self.top
                .min()
                .and_then(|m| self.nodes[m as usize].min().map(|m2| m2 + (m << WORD_LOG)))
        }
        fn pop_min(&mut self) -> Option<u64> {
            let min = self.min();
            if let Some(m) = min {
                self.remove(m);
            }
            min
        }
        fn next(&self, x: u64) -> Option<u64> {
            if let Some(a) = self.nodes[x as usize >> WORD_LOG].next(x & 63) {
                Some((x >> WORD_LOG << WORD_LOG) + a)
            } else {
                self.top
                    .next(x >> WORD_LOG)
                    .and_then(|m| self.nodes[m as usize].min().map(|m2| m2 + (m << WORD_LOG)))
            }
        }
        fn prev(&self, x: u64) -> Option<u64> {
            if let Some(a) = self.nodes[x as usize >> WORD_LOG].prev(x & 63) {
                Some((x >> WORD_LOG << WORD_LOG) + a)
            } else {
                self.top
                    .prev(x >> WORD_LOG)
                    .and_then(|m| self.nodes[m as usize].max().map(|m2| m2 + (m << WORD_LOG)))
            }
        }
    }
    #[derive(Clone)]
    pub struct Depth3Tree {
        top: Depth2Tree,
        nodes: Vec<Node>,
    }

    impl Depth3Tree {
        fn new() -> Self {
            Self {
                top: Depth2Tree::new(),
                nodes: vec![Node::default(); WORD_SIZE_2],
            }
        }
        fn insert(&mut self, x: u64) -> bool {
            assert!(x < 1 << 18);
            self.top.insert(x >> WORD_LOG);
            self.nodes[x as usize >> WORD_LOG].add(x & 63)
        }

        fn contains(&self, x: u64) -> bool {
            assert!(x < 1 << 18);
            self.nodes[x as usize >> WORD_LOG].contains(x & 63)
        }

        fn remove(&mut self, x: u64) -> bool {
            assert!(x < 1 << 18);
            let del = self.nodes[x as usize >> WORD_LOG].remove(x & 63);
            if del && self.nodes[x as usize >> WORD_LOG].is_empty() {
                self.top.remove(x >> WORD_LOG);
            }
            del
        }
        fn max(&self) -> Option<u64> {
            self.top
                .max()
                .and_then(|m| self.nodes[m as usize].max().map(|m2| m2 + (m << WORD_LOG)))
        }
        fn pop_max(&mut self) -> Option<u64> {
            let max = self.max();
            if let Some(m) = max {
                self.remove(m);
            }
            max
        }
        fn min(&self) -> Option<u64> {
            self.top
                .min()
                .and_then(|m| self.nodes[m as usize].min().map(|m2| m2 + (m << WORD_LOG)))
        }
        fn pop_min(&mut self) -> Option<u64> {
            let min = self.min();
            if let Some(m) = min {
                self.remove(m);
            }
            min
        }
        fn next(&self, x: u64) -> Option<u64> {
            if let Some(a) = self.nodes[x as usize >> WORD_LOG].next(x & 63) {
                Some((x >> WORD_LOG << WORD_LOG) + a)
            } else {
                self.top
                    .next(x >> WORD_LOG)
                    .and_then(|m| self.nodes[m as usize].min().map(|m2| m2 + (m << WORD_LOG)))
            }
        }
        fn prev(&self, x: u64) -> Option<u64> {
            if let Some(a) = self.nodes[x as usize >> WORD_LOG].prev(x & 63) {
                Some((x >> WORD_LOG << WORD_LOG) + a)
            } else {
                self.top
                    .prev(x >> WORD_LOG)
                    .and_then(|m| self.nodes[m as usize].max().map(|m2| m2 + (m << WORD_LOG)))
            }
        }
    }
    #[derive(Clone)]
    pub struct Depth4Tree {
        top: Depth3Tree,
        nodes: Vec<Node>,
    }

    impl Depth4Tree {
        fn new() -> Self {
            Self {
                top: Depth3Tree::new(),
                nodes: vec![Node::default(); WORD_SIZE_3],
            }
        }
        fn insert(&mut self, x: u64) -> bool {
            assert!(x < 1 << 24);
            self.top.insert(x >> WORD_LOG);
            self.nodes[x as usize >> WORD_LOG].add(x & 63)
        }

        fn contains(&self, x: u64) -> bool {
            assert!(x < 1 << 24);
            self.nodes[x as usize >> WORD_LOG].contains(x & 63)
        }

        fn remove(&mut self, x: u64) -> bool {
            assert!(x < 1 << 24);
            let del = self.nodes[x as usize >> WORD_LOG].remove(x & 63);
            if del && self.nodes[x as usize >> WORD_LOG].is_empty() {
                self.top.remove(x >> WORD_LOG);
            }
            del
        }

        /// # 最大値を返す
        pub fn max(&self) -> Option<u64> {
            self.top
                .max()
                .and_then(|m| self.nodes[m as usize].max().map(|m2| m2 + (m << WORD_LOG)))
        }

        /// # 最大値を消費して返す
        pub fn pop_max(&mut self) -> Option<u64> {
            let max = self.max();
            if let Some(m) = max {
                self.remove(m);
            }
            max
        }

        /// # 最小値を返す
        pub fn min(&self) -> Option<u64> {
            self.top
                .min()
                .and_then(|m| self.nodes[m as usize].min().map(|m2| m2 + (m << WORD_LOG)))
        }

        /// # 最小値を消費して返す
        pub fn pop_min(&mut self) -> Option<u64> {
            let min = self.min();
            if let Some(m) = min {
                self.remove(m);
            }
            min
        }
        fn next(&self, x: u64) -> Option<u64> {
            if let Some(a) = self.nodes[x as usize >> WORD_LOG].next(x & 63) {
                Some((x >> WORD_LOG << WORD_LOG) + a)
            } else {
                self.top
                    .next(x >> WORD_LOG)
                    .and_then(|m| self.nodes[m as usize].min().map(|m2| m2 + (m << WORD_LOG)))
            }
        }
        fn prev(&self, x: u64) -> Option<u64> {
            if let Some(a) = self.nodes[x as usize >> WORD_LOG].prev(x & 63) {
                Some((x >> WORD_LOG << WORD_LOG) + a)
            } else {
                self.top
                    .prev(x >> WORD_LOG)
                    .and_then(|m| self.nodes[m as usize].max().map(|m2| m2 + (m << WORD_LOG)))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Complete64PartTree;

    #[test]
    fn test_dim1() {
        let mut tree = Complete64PartTree::new(63);
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
        let mut tree = Complete64PartTree::new(4095);
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
        let mut tree = Complete64PartTree::new(261000);
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
        let mut tree = Complete64PartTree::new(16000000);
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
