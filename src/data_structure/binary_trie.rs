//! # Binary Trie
//! 非負整数をBit列とみなしてトライ木に載せたもの
//! set的な機能を持つ
//!
//! ## verify
//! [ARC033_C](https://atcoder.jp/contests/arc033/submissions/34635956)
use crate::prelude::*;

#[snippet(name = "binary-trie", doc_hidden)]
pub use binary_trie_impl::BinaryTrie;
#[snippet(name = "binary-trie", doc_hidden)]
mod binary_trie_impl {
    use super::{swap, Debug, Formatter, JoinTrait};
    type TrieValue = u64;
    type Bit = i32;

    #[derive(Clone, Default)]
    pub struct BinaryTrie {
        root: OptionalNode,
        xor_val: u64,
        bit_len: Bit,
    }

    impl BinaryTrie {
        /// $2^{63}$ 未満の非負整数を登録できる
        pub fn new(bit_len: Bit) -> Self {
            assert!((0..=63).contains(&bit_len));
            Self {
                bit_len,
                ..Default::default()
            }
        }

        /// 今までにinsertした個数を取得する
        /// ## 計算量
        /// $O(1)$
        pub fn size(&self) -> usize {
            self.root.count()
        }

        /// # insert V
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn insert(&mut self, v: u64) {
            self.root.add(v, self.bit_len);
        }

        /// vを一つ削除する
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn erase(&mut self, v: TrieValue) {
            self.root.sub(v, self.bit_len);
        }

        /// xor_valとXORをとったときに最小値となるような値を取得する
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn min_element(&self) -> TrieValue {
            self.root.get_min(self.xor_val, self.bit_len)
        }

        /// biasとXORをとったときに最大値となるような値を取得する
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn max_element(&self) -> TrieValue {
            self.root.get_min(self.rev_xor_val(), self.bit_len)
        }

        /// 小さい方からn番目の値を取得する
        /// ## 計算量
        /// $O(\text{BIT\textunderscore LEN})$
        pub fn nth(&self, k: usize) -> TrieValue {
            assert!(k <= self.size());
            self.root.get(k as u64, self.bit_len)
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
            write!(f, "{}", (0..self.size()).map(|i| self.nth(i)).join(" "))
        }
    }

    #[derive(Clone, Default)]
    struct OptionalNode(Option<TrieNode>);

    impl OptionalNode {
        fn is_none(&self) -> bool {
            self.0.is_none()
        }

        fn add(&mut self, v: TrieValue, b: Bit) {
            match self.0.as_mut() {
                Some(node) => {
                    node.count += 1;
                    if b > 0 {
                        node.child_mut(v, b - 1).add(v, b - 1)
                    }
                }
                None => {
                    self.0 = Some(TrieNode::default());
                    self.add(v, b);
                }
            }
        }

        fn sub(&mut self, v: TrieValue, b: Bit) {
            match self.0.as_mut() {
                Some(node) => {
                    if node.count == 1 {
                        swap(&mut self.0, &mut None);
                    } else {
                        node.count -= 1;
                        if b > 0 {
                            node.child_mut(v, b - 1).sub(v, b - 1);
                        }
                    }
                }
                None => panic!("Sub to unexisted node."),
            }
        }

        fn get_min(&self, mut bias: TrieValue, b: Bit) -> TrieValue {
            if b == 0 {
                return bias;
            }
            match &self.0 {
                Some(node) => {
                    let mut child = node.child(bias, b - 1);
                    if child.is_none() {
                        bias ^= 1 << b - 1;
                        child = node.child(bias, b - 1);
                    }
                    child.get_min(bias, b - 1)
                }
                None => 0,
            }
        }

        fn get(&self, k: u64, b: Bit) -> TrieValue {
            if b == 0 {
                return 0;
            }
            if let Some(node) = &self.0 {
                let m = node.child(k, b).count() as u64;
                if k < m {
                    node.left().get(k, b - 1)
                } else {
                    node.right().get(k - m, b - 1) | (1 << b - 1)
                }
            } else {
                unreachable!("k-th element does not exist.")
            }
        }

        fn count(&self) -> usize {
            if let Some(node) = &self.0 {
                node.count
            } else {
                0
            }
        }
    }

    #[derive(Clone)]
    pub struct TrieNode {
        count: usize,
        children: Vec<OptionalNode>,
    }

    impl Default for TrieNode {
        fn default() -> Self {
            Self {
                count: 0,
                children: vec![OptionalNode::default(), OptionalNode::default()],
            }
        }
    }

    impl TrieNode {
        #[inline]
        fn child_mut(&mut self, idx: TrieValue, bit: Bit) -> &mut OptionalNode {
            match () {
                () if idx >> bit & 1 == 0 => self.left_mut(),
                _ => self.right_mut(),
            }
        }
        #[inline]
        fn child(&self, idx: TrieValue, bit: Bit) -> &OptionalNode {
            match () {
                () if idx >> bit & 1 == 0 => self.left(),
                _ => self.right(),
            }
        }
        #[inline]
        fn left(&self) -> &OptionalNode {
            unsafe { self.children.get_unchecked(0) }
        }
        #[inline]
        fn left_mut(&mut self) -> &mut OptionalNode {
            unsafe { self.children.get_unchecked_mut(0) }
        }
        #[inline]
        fn right(&self) -> &OptionalNode {
            unsafe { self.children.get_unchecked(1) }
        }
        #[inline]
        fn right_mut(&mut self) -> &mut OptionalNode {
            unsafe { self.children.get_unchecked_mut(1) }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        let mut trie = BinaryTrie::new(5);
        trie.insert(5);
        trie.insert(9);
        trie.insert(7);
        trie.insert(6);
        trie.insert(8);
        assert_eq!(5, trie.size());
        assert_eq!(5, trie.min_element());
        assert_eq!(9, trie.max_element());
        assert_eq!(5, trie.nth(0));
        assert_eq!(6, trie.nth(1));
        assert_eq!(7, trie.nth(2));
        assert_eq!(8, trie.nth(3));
        assert_eq!(9, trie.nth(4));

        trie.erase(5);
        trie.erase(7);

        assert_eq!(3, trie.size());
        assert_eq!(6, trie.nth(0));
        assert_eq!(8, trie.nth(1));
        assert_eq!(9, trie.nth(2));
    }
}
