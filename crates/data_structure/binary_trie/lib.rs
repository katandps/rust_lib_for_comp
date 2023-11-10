//! # Binary Trie
//! 非負整数をBit列とみなしてトライ木に載せたもの
//! set的な機能を持つ
//!
use prelude::*;
use string_util::JoinTrait;
pub mod recursive;

#[snippet(name = "binary-trie", doc_hidden)]
pub use binary_trie_impl::BinaryTrie;
#[snippet(name = "binary-trie", doc_hidden)]
mod binary_trie_impl {
    use super::{min, Debug, Formatter, JoinTrait};
    type TrieValue = u64;
    type Bit = i32;

    type NodeId = u32;
    const EMPTY_NODE: NodeId = !0;

    #[derive(Clone)]
    pub struct BinaryTrie {
        pub root: NodeId,
        pub xor_val: u64,
        pub bit_len: Bit,
        pub nodes: Vec<TrieNode>,
    }

    impl BinaryTrie {
        /// $2^{63}$ 未満の非負整数を登録できる
        pub fn new(bit_len: Bit) -> Self {
            assert!((0..=63).contains(&bit_len));
            let mut nodes = Vec::with_capacity(min(1 << bit_len, 100000));
            nodes.push(TrieNode::default());
            let (xor_val, root) = (0, 0);
            Self {
                root,
                bit_len,
                nodes,
                xor_val,
            }
        }

        fn node(&self, id: u32) -> &TrieNode {
            &self.nodes[id as usize]
        }

        fn node_mut(&mut self, id: u32) -> &mut TrieNode {
            &mut self.nodes[id as usize]
        }

        /// 今までにinsertした個数を取得する
        /// ## 計算量
        /// $O(1)$
        pub fn len(&self) -> usize {
            self.nodes[self.root as usize].count
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// # insert V
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn insert(&mut self, v: u64) {
            let mut target = self.root;
            let mut bit = self.bit_len;
            while bit >= 0 {
                self.node_mut(target).count += 1;
                if self.node(target).children[v as usize >> bit & 1] == !0 {
                    self.nodes.push(TrieNode::default());
                    self.node_mut(target).children[v as usize >> bit & 1] =
                        self.nodes.len() as NodeId - 1;
                }
                target = self.node(target).children[v as usize >> bit & 1];
                bit -= 1;
            }
            self.node_mut(target).count += 1;
        }

        /// vを一つ削除する
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn erase(&mut self, v: TrieValue) {
            let (mut target, mut bit) = (self.root, self.bit_len);
            while bit >= 0 {
                if self.node(target).count == 0 {
                    panic!("remove unexist node");
                }
                self.node_mut(target).count -= 1;
                if self.node(target).children[v as usize >> bit & 1] == !0 {
                    panic!("remove unexist node");
                }
                target = self.node(target).children[v as usize >> bit & 1];

                bit -= 1;
            }
            self.node_mut(target).count -= 1;
        }

        /// # vが含まれるか
        pub fn contains(&self, v: TrieValue) -> bool {
            let (mut target, mut bit) = (self.root, self.bit_len);
            while bit >= 0 {
                if self.node(target).children[v as usize >> bit & 1] == !0 {
                    return false;
                } else {
                    target = self.node(target).children[v as usize >> bit & 1]
                }

                bit -= 1;
            }
            self.node(target).count > 0
        }

        /// xor_valとXORをとったときに最小値となるような値を取得する
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn min_element(&self) -> Option<TrieValue> {
            if self.is_empty() {
                return None;
            }
            let (mut target, mut bit, mut ret) = (self.root, self.bit_len, self.xor_val);
            while bit >= 0 {
                let mut child = self.node(target).children[ret as usize >> bit & 1];
                if child == !0 || self.node(child).count == 0 {
                    ret ^= 1 << bit;
                    child = self.node(target).children[ret as usize >> bit & 1];
                }
                target = child;
                bit -= 1;
            }
            Some(ret)
        }

        /// biasとXORをとったときに最大値となるような値を取得する
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn max_element(&self) -> Option<TrieValue> {
            if self.is_empty() {
                return None;
            }
            let mut target = self.root;
            let mut bit = self.bit_len;
            let mut ret = self.rev_xor_val();
            while bit >= 0 {
                let mut child = self.node(target).children[ret as usize >> bit & 1];
                if child == !0 || self.node(child).count == 0 {
                    ret ^= 1 << bit;
                    child = self.node(target).children[ret as usize >> bit & 1]
                }
                target = child;
                bit -= 1;
            }

            Some(ret)
        }

        /// 小さい方からn番目の値を取得する
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn nth(&self, k: usize) -> Option<TrieValue> {
            let (mut target, mut bit, mut cnt, mut ret, k) = (self.root, self.bit_len, 0, 0, k + 1);
            while bit >= 0 {
                if self.node(target).children[0] == !0 {
                    ret += 1 << bit;
                    target = self.node(target).children[1];
                } else if cnt + self.node(self.node(target).children[0]).count < k {
                    // k番目がon側にある
                    cnt += self.node(self.node(target).children[0]).count;
                    ret += 1 << bit;
                    target = self.node(target).children[1];
                } else {
                    target = self.node(target).children[0];
                }

                if target == !0 {
                    return None;
                }
                bit -= 1;
            }
            Some(ret)
        }

        /// # 探索
        /// v以上の値をとる最小のkを返す
        pub fn lower_bound(&self, v: TrieValue) -> Option<usize> {
            if self.is_empty() {
                return None;
            }
            if v == 0 {
                return Some(0);
            }
            let (mut target, mut bit, mut ret) = (self.root, self.bit_len, 0);
            while bit >= 0 {
                if target == !0 {
                    break;
                }
                if v >> bit & 1 == 1 {
                    ret += self.count(self.node(target).children[0]);
                    target = self.node(target).children[1];
                } else {
                    target = self.node(target).children[0];
                }
                bit -= 1;
            }
            if self.len() != ret {
                Some(ret)
            } else {
                None
            }
        }

        /// # 大なる値
        /// vより大きい値のうち、最小のものを返す
        pub fn next(&self, v: TrieValue) -> Option<TrieValue> {
            if let Some(k) = self.upper_bound(v) {
                self.nth(k)
            } else {
                None
            }
        }

        /// # 小なる値
        /// vより小さい値のうち、最大のものを返す
        pub fn prev(&self, v: TrieValue) -> Option<TrieValue> {
            if let Some(k) = self.lower_bound(v) {
                if k > 0 {
                    self.nth(k - 1)
                } else {
                    None
                }
            } else {
                self.max_element()
            }
        }

        fn count(&self, node: NodeId) -> usize {
            if node == !0 {
                0
            } else {
                self.node(node).count
            }
        }

        /// # 探索
        /// vより大きい値をとる最小のkを返す
        pub fn upper_bound(&self, v: TrieValue) -> Option<usize> {
            self.lower_bound(v + 1)
        }

        /// # bias変更
        pub fn set_xor_val(&mut self, val: u64) {
            self.xor_val = val
        }

        fn rev_xor_val(&self) -> u64 {
            self.xor_val ^ ((1 << self.bit_len) - 1)
        }
    }

    impl Debug for BinaryTrie {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                (0..self.len()).filter_map(|i| self.nth(i)).join(" ")
            )
        }
    }
    #[derive(Clone, Debug)]
    pub struct TrieNode {
        count: usize,
        children: [NodeId; 2],
    }

    impl Default for TrieNode {
        fn default() -> Self {
            TrieNode {
                count: 0,
                children: [EMPTY_NODE, EMPTY_NODE],
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test() {
        let mut trie = BinaryTrie::new(4);
        trie.insert(5);
        trie.insert(9);
        trie.insert(7);
        trie.insert(6);
        trie.insert(8);
        assert_eq!(5, trie.len());
        assert_eq!(Some(5), trie.min_element());
        assert_eq!(Some(9), trie.max_element());
        assert_eq!(Some(5), trie.nth(0));
        assert_eq!(Some(6), trie.nth(1));
        assert_eq!(Some(7), trie.nth(2));
        assert_eq!(Some(8), trie.nth(3));
        assert_eq!(Some(9), trie.nth(4));

        trie.erase(5);
        trie.erase(7);

        trie.insert(6);

        assert_eq!(4, trie.len());
        assert_eq!(Some(6), trie.nth(0));
        assert_eq!(Some(6), trie.nth(1));
        assert_eq!(Some(8), trie.nth(2));
        assert_eq!(Some(9), trie.nth(3));

        assert_eq!("6 6 8 9", &format!("{:?}", trie));

        assert!(!trie.contains(5));
        assert!(trie.contains(6));
        assert!(!trie.contains(7));
        assert!(trie.contains(8));
        assert!(trie.contains(9));

        assert_eq!(Some(0), trie.lower_bound(4));
        assert_eq!(Some(0), trie.lower_bound(5));
        assert_eq!(Some(0), trie.lower_bound(6));
        assert_eq!(Some(2), trie.lower_bound(7));
        assert_eq!(Some(2), trie.lower_bound(8));
        assert_eq!(Some(3), trie.lower_bound(9));
        assert_eq!(None, trie.lower_bound(10));

        assert_eq!(Some(0), trie.upper_bound(4));
        assert_eq!(Some(0), trie.upper_bound(5));
        assert_eq!(Some(2), trie.upper_bound(6));
        assert_eq!(Some(2), trie.upper_bound(7));
        assert_eq!(Some(3), trie.upper_bound(8));
        assert_eq!(None, trie.upper_bound(9));
    }

    #[test]
    fn test_small() {
        let mut trie = BinaryTrie::new(2);
        trie.insert(1);
        trie.insert(2);
        assert_eq!(2, trie.len());
        assert_eq!(Some(1), trie.min_element());
        assert_eq!(Some(2), trie.max_element());
        assert_eq!(Some(1), trie.nth(0));
        assert_eq!(Some(2), trie.nth(1));
        assert!(!trie.contains(0));
        assert!(trie.contains(1));
        assert!(trie.contains(2));
        assert!(!trie.contains(3));
        assert_eq!(Some(0), trie.lower_bound(0));
        assert_eq!(Some(0), trie.lower_bound(1));
        assert_eq!(Some(1), trie.lower_bound(2));
        assert_eq!(None, trie.lower_bound(3));
    }
}
