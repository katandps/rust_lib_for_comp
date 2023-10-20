//! # 完全64分木
//! $n<2^{18} = 262143$の範囲でsetのようなInterfaceを提供する 高速・省メモリ
//!
//! 集合に存在するかどうかをBitSetの入れ子のような形で表現する

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
        Depth2(Depth2Tree),
        Depth3(Depth3Tree),
        Depth4(Depth4Tree),
    }

    impl Complete64PartTree {
        pub fn new(limit: u64) -> Self {
            if limit < 1u64 << WORD_LOG {
                Self::Depth1(Depth1Tree {
                    node: Node::default(),
                })
            } else if limit < 1u64 << WORD_LOG * 2 {
                Self::Depth2(Depth2Tree {
                    nodes: [Node::default(); 1 + WORD_SIZE],
                })
            } else if limit < 1u64 << WORD_LOG * 3 {
                Self::Depth3(Depth3Tree {
                    nodes: [Node::default(); 1 + WORD_SIZE + WORD_SIZE_2],
                })
            } else {
                Self::Depth4(Depth4Tree {
                    nodes: [Node::default(); 1 + WORD_SIZE + WORD_SIZE_2 + WORD_SIZE_3],
                })
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
            // let ret = self.max();
            // if let Some(r) = ret {
            //     self.erase(r);
            // }
            // ret
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
            // let ret = self.min();
            // if let Some(r) = ret {
            //     self.erase(r);
            // }
            // ret
            match self {
                Self::Depth1(tree) => tree.pop_min(),
                Self::Depth2(tree) => tree.pop_min(),
                Self::Depth3(tree) => tree.pop_min(),
                Self::Depth4(tree) => tree.pop_min(),
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
    }
    #[derive(Clone)]
    pub struct Depth1Tree {
        node: Node,
    }

    impl Depth1Tree {
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
    }
    #[derive(Clone)]
    pub struct Depth2Tree {
        nodes: [Node; 1 + WORD_SIZE],
    }

    impl Depth2Tree {
        fn insert(&mut self, x: u64) -> bool {
            assert!(x < 1 << 12);
            self.nodes[0].add(x >> WORD_LOG);
            self.nodes[1 + (x >> WORD_LOG) as usize].add(x & 63)
        }
        fn contains(&self, x: u64) -> bool {
            assert!(x < 1 << 12);
            self.nodes[1 + (x >> WORD_LOG) as usize].contains(x & 63)
        }
        fn remove(&mut self, x: u64) -> bool {
            assert!(x < 1 << 12);
            let xi = x as usize;
            let del = self.nodes[1 + (xi >> WORD_LOG)].remove(x & WORD_SIZE as u64 - 1);
            if del && self.nodes[1 + (xi >> WORD_LOG)].is_empty() {
                self.nodes[0].remove(x >> WORD_LOG);
            }
            del
        }
        fn max(&self) -> Option<u64> {
            self.nodes[0].max().and_then(|m1| {
                self.nodes[1 + m1 as usize]
                    .max()
                    .map(|m2| m2 + (m1 << WORD_LOG))
            })
        }
        fn pop_max(&mut self) -> Option<u64> {
            let max = self.max();
            if let Some(m) = max {
                self.remove(m);
            }
            max
        }
        fn min(&self) -> Option<u64> {
            if let Some(m1) = self.nodes[0].min() {
                self.nodes[1 + m1 as usize]
                    .min()
                    .map(|m2| m2 + (m1 << WORD_LOG))
            } else {
                None
            }
        }
        fn pop_min(&mut self) -> Option<u64> {
            let min = self.min();
            if let Some(m) = min {
                self.remove(m);
            }
            min
        }
    }
    #[derive(Clone)]
    pub struct Depth3Tree {
        nodes: [Node; 1 + WORD_SIZE + WORD_SIZE_2],
    }

    impl Depth3Tree {
        fn insert(&mut self, x: u64) -> bool {
            assert!(x < 1 << 18);
            self.nodes[0].add(x >> WORD_LOG * 2);
            self.nodes[1 + (x >> WORD_LOG * 2) as usize].add(x >> WORD_LOG & 63);
            self.nodes[1 + WORD_SIZE + (x >> WORD_LOG) as usize].add(x & 63)
        }

        fn contains(&self, x: u64) -> bool {
            assert!(x < 1 << 18);
            self.nodes[1 + WORD_SIZE + (x >> WORD_LOG) as usize].contains(x & 63)
        }

        fn remove(&mut self, x: u64) -> bool {
            assert!(x < 1 << 18);
            let dim2_index = 1 + WORD_SIZE + (x >> WORD_LOG) as usize;
            let del = self.nodes[dim2_index].remove(x & 63);
            if del && self.nodes[dim2_index].is_empty() {
                let dim1_index: usize = 1 + (x >> WORD_LOG * 2) as usize;
                let del = self.nodes[dim1_index].remove(x >> WORD_LOG & 63);
                if del && self.nodes[dim1_index].is_empty() {
                    self.nodes[0].remove(x >> WORD_LOG * 2);
                }
            }
            del
        }
        fn max(&self) -> Option<u64> {
            self.nodes[0].max().and_then(|m1| {
                self.nodes[1 + m1 as usize].max().and_then(|m2| {
                    self.nodes[1 + WORD_SIZE + m1 as usize * WORD_SIZE + m2 as usize]
                        .max()
                        .map(|m3| m3 + (m2 << WORD_LOG) + (m1 << WORD_LOG * 2))
                })
            })
        }
        fn pop_max(&mut self) -> Option<u64> {
            let max = self.max();
            if let Some(m) = max {
                self.remove(m);
            }
            max
        }
        fn min(&self) -> Option<u64> {
            self.nodes[0].min().and_then(|m1| {
                self.nodes[1 + m1 as usize].min().and_then(|m2| {
                    self.nodes[1 + WORD_SIZE + m1 as usize * WORD_SIZE + m2 as usize]
                        .min()
                        .map(|m3| m3 + (m2 << WORD_LOG) + (m1 << WORD_LOG * 2))
                })
            })
        }
        fn pop_min(&mut self) -> Option<u64> {
            let min = self.min();
            if let Some(m) = min {
                self.remove(m);
            }
            min
        }
    }
    #[derive(Clone)]
    pub struct Depth4Tree {
        nodes: [Node; 1 + WORD_SIZE + WORD_SIZE_2 + WORD_SIZE_3],
    }

    impl Depth4Tree {
        fn insert(&mut self, x: u64) -> bool {
            assert!(x < 1 << 24);
            self.nodes[0].add(x >> WORD_LOG * 3);
            self.nodes[1 + (x >> WORD_LOG * 2 & 63) as usize].add(x >> WORD_LOG * 2 & 63);
            self.nodes[1 + WORD_SIZE + (x >> WORD_LOG & 63) as usize].add(x >> WORD_LOG & 63);
            self.nodes[1 + WORD_SIZE + WORD_SIZE_2 + (x & 63) as usize].add(x & 63)
        }

        fn contains(&self, x: u64) -> bool {
            assert!(x < 1 << 24);
            self.nodes[1 + WORD_SIZE + WORD_SIZE_2 + (x & 63) as usize].contains(x & 63)
        }

        fn remove(&self, x: u64) -> bool {
            assert!(x < 1 << 24);
            unimplemented!()
        }

        /// # 最大値を返す
        pub fn max(&self) -> Option<u64> {
            unimplemented!()
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
            unimplemented!()
        }

        /// # 最小値を消費して返す
        pub fn pop_min(&mut self) -> Option<u64> {
            let min = self.min();
            if let Some(m) = min {
                self.remove(m);
            }
            min
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
}
