//! # Binary Trie
//! 非負整数をBit列とみなしてトライ木に載せたもの
//! set的な機能を持つ
//!
//! ## verify
//! [ARC033_C](https://atcoder.jp/contests/arc033/submissions/34635956)
use prelude::*;
use string_util::JoinTrait;

pub mod non_recursive;

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
        pub fn len(&self) -> usize {
            self.root.count()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
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

        /// # vが含まれるか
        pub fn contains(&mut self, v: TrieValue) -> bool {
            self.root.contains(v, self.bit_len)
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
            assert!(k <= self.len());
            self.root.get(k as u64, self.bit_len)
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

            let c = self.root.count_lower(v, self.bit_len - 1);
            if self.len() != c {
                Some(c)
            } else {
                None
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
            write!(f, "{}", (0..self.len()).map(|i| self.nth(i)).join(" "))
        }
    }

    #[derive(Clone, Default, Debug)]
    struct OptionalNode(Option<TrieNode>);

    impl OptionalNode {
        fn is_none(&self) -> bool {
            self.0.is_none()
        }

        /// # 加算
        /// $v$に$1$加える
        /// bは現在の階層
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

        /// # 減算
        /// $v$から$1$引く
        /// bは現在の階層
        /// 0未満にすることはできない
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

        /// # 存在判定
        /// $v$が1以上か調べる
        /// bは現在の階層
        fn contains(&self, v: TrieValue, b: Bit) -> bool {
            if let Some(node) = &self.0 {
                // b == 0の場合はこのノードが存在することが答え
                if b == 0 {
                    node.count > 0
                } else {
                    node.child(v, b - 1).contains(v, b - 1)
                }
            } else {
                false
            }
        }

        /// # 最小値
        fn get_min(&self, mut bias: TrieValue, b: Bit) -> TrieValue {
            if b == 0 {
                return bias;
            }
            if let Some(node) = &self.0 {
                let mut child = node.child(bias, b - 1);
                if child.is_none() {
                    bias ^= 1 << (b - 1);
                    child = node.child(bias, b - 1);
                }
                child.get_min(bias, b - 1)
            } else {
                0
            }
        }

        /// # 最大値
        fn get(&self, k: u64, b: Bit) -> TrieValue {
            if b == 0 {
                return 0;
            }
            if let Some(node) = &self.0 {
                let m = node.child(k, b).count() as u64;
                if k < m {
                    node.off.get(k, b - 1)
                } else {
                    node.on.get(k - m, b - 1) | (1 << (b - 1))
                }
            } else {
                unreachable!("k-th element does not exist.")
            }
        }

        /// # v未満の値の個数
        fn count_lower(&self, v: TrieValue, b: Bit) -> usize {
            if let Some(node) = &self.0 {
                if b < 0 {
                    0
                } else if v >> b & 1 == 1 {
                    node.off.count() + node.on.count_lower(v, b - 1)
                } else {
                    node.off.count_lower(v, b - 1)
                }
            } else {
                0
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

    #[derive(Clone, Debug, Default)]
    pub struct TrieNode {
        count: usize,
        on: Box<OptionalNode>,
        off: Box<OptionalNode>,
    }

    impl TrieNode {
        #[inline]
        fn child_mut(&mut self, idx: TrieValue, bit: Bit) -> &mut OptionalNode {
            match () {
                () if idx >> bit & 1 == 0 => &mut self.off,
                _ => &mut self.on,
            }
        }
        #[inline]
        fn child(&self, idx: TrieValue, bit: Bit) -> &OptionalNode {
            match () {
                () if idx >> bit & 1 == 0 => &self.off,
                _ => &self.on,
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
        assert_eq!(5, trie.min_element());
        assert_eq!(9, trie.max_element());
        assert_eq!(5, trie.nth(0));
        assert_eq!(6, trie.nth(1));
        assert_eq!(7, trie.nth(2));
        assert_eq!(8, trie.nth(3));
        assert_eq!(9, trie.nth(4));

        trie.erase(5);
        trie.erase(7);

        trie.insert(6);

        assert_eq!(4, trie.len());
        assert_eq!(6, trie.nth(0));
        assert_eq!(6, trie.nth(1));
        assert_eq!(8, trie.nth(2));
        assert_eq!(9, trie.nth(3));

        assert_eq!("6 6 8 9", &format!("{:?}", trie));

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
        assert_eq!(1, trie.min_element());
        assert_eq!(2, trie.max_element());
        assert_eq!(1, trie.nth(0));
        assert_eq!(2, trie.nth(1));
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
