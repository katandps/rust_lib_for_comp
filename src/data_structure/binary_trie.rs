//! # Binary Trie
//! 非負整数をBit列とみなしてトライ木に載せたもの
//! set的な機能を持つ
//!
//! ## verify
//! [ARC033_C](https://atcoder.jp/contests/arc033/submissions/34635956)
use crate::prelude::*;

#[snippet(name = "binary-trie", doc_hidden)]
type TrieValue = u64;

#[snippet(name = "binary-trie", doc_hidden)]
#[derive(Clone, Default)]
pub struct BinaryTrie {
    root: TrieNode,
    xor_val: u64,
}

#[snippet(name = "binary-trie", doc_hidden)]
impl BinaryTrie {
    /// $2^{60}$ 未満の非負整数を登録できる
    pub const BIT_LEN: i64 = 60;

    /// 今までにinsertした個数を取得する
    /// ## 計算量
    /// $O(1)$
    pub fn size(&self) -> usize {
        self.root.count
    }

    /// vをinsertする
    /// ## 計算量
    /// $O(\text{BIT\textunderscore LEN})$
    pub fn insert(&mut self, v: u64) {
        self.root.add(v, Self::BIT_LEN - 1);
    }

    /// vを一つ削除する
    /// ## 計算量
    /// $O(\text{BIT\textunderscore LEN})$
    pub fn erase(&mut self, v: TrieValue) {
        self.root.sub(v, Self::BIT_LEN - 1);
    }

    /// xor_valとXORをとったときに最小値となるような値を取得する
    /// ## 計算量
    /// $O(\text{BIT\textunderscore LEN})$
    pub fn min_element(&self) -> TrieValue {
        self.root.get_min(self.xor_val, Self::BIT_LEN - 1)
    }

    /// biasとXORをとったときに最大値となるような値を取得する
    /// ## 計算量
    /// $O(\text{BIT\textunderscore LEN})$
    pub fn max_element(&self) -> TrieValue {
        self.root.get_min(self.rev_xor_val(), Self::BIT_LEN - 1)
    }

    /// 小さい方からk番目の値を取得する
    /// ## 計算量
    /// $O(\text{BIT\textunderscore LEN})$
    pub fn nth(&self, k: usize) -> TrieValue {
        assert!(k <= self.size());
        self.root.get(k, Self::BIT_LEN - 1)
    }

    /// # bias変更
    pub fn set_xor_val(&mut self, val: u64) {
        self.xor_val = val
    }

    fn rev_xor_val(&self) -> u64 {
        self.xor_val ^ ((1 << Self::BIT_LEN) - 1)
    }
}

#[snippet(name = "binary-trie", doc_hidden)]
impl Debug for BinaryTrie {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.size())
                .map(|i| self.nth(i).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[snippet(name = "binary-trie", doc_hidden)]
#[derive(Clone)]
struct TrieNode {
    count: usize,
    child: Vec<Option<TrieNode>>,
}

#[snippet(name = "binary-trie", doc_hidden)]
impl Default for TrieNode {
    fn default() -> Self {
        Self {
            count: 0,
            child: vec![None, None],
        }
    }
}

#[snippet(name = "binary-trie", doc_hidden)]
impl TrieNode {
    fn add(&mut self, v: TrieValue, b: i64) {
        self.count += 1;
        if b >= 0 {
            let dst = (v >> b & 1) as usize;
            if let Some(node) = self.child[dst].as_mut() {
                node.add(v, b - 1);
            } else {
                let mut node = TrieNode::default();
                node.add(v, b - 1);
                self.child[dst] = Some(node);
            }
        }
    }

    fn sub(&mut self, v: TrieValue, b: i64) {
        self.count -= 1;
        if b >= 0 {
            let dst = (v >> b & 1) as usize;
            self.child[dst].iter_mut().for_each(|c| c.sub(v, b - 1));
        }
    }

    fn get_min(&self, bias: TrieValue, b: i64) -> TrieValue {
        if b < 0 {
            return 0;
        }
        let mut dst = bias >> b & 1;
        if self.child[dst as usize].is_none() {
            dst ^= 1;
        }
        self.child[dst as usize]
            .as_ref()
            .map_or(0, |c| c.get_min(bias, b - 1))
            | (dst << b)
    }

    fn get(&self, k: usize, b: i64) -> TrieValue {
        if b < 0 {
            return 0;
        }
        let m = self.child[0].as_ref().map_or(0, |c| c.count);
        if k < m {
            self.child[0].as_ref().unwrap().get(k, b - 1)
        } else {
            self.child[1].as_ref().map_or(0, |c| c.get(k - m, b - 1)) | (1 << b)
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        let mut trie = BinaryTrie::default();
        trie.insert(5);
        trie.insert(6);
        trie.insert(7);
        trie.insert(8);
        trie.insert(9);
        assert_eq!(5, trie.size());
        assert_eq!(5, trie.nth(0));
        assert_eq!(6, trie.nth(1));
        assert_eq!(7, trie.nth(2));
        assert_eq!(8, trie.nth(3));
        assert_eq!(9, trie.nth(4));
        assert_eq!(5, trie.min_element());
        assert_eq!(9, trie.max_element());

        trie.erase(5);
        trie.erase(7);

        assert_eq!(3, trie.size());
        assert_eq!(6, trie.nth(0));
        assert_eq!(8, trie.nth(1));
        assert_eq!(9, trie.nth(2));
    }
}
