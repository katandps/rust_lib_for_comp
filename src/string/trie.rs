//! # トライ木(Trie, Prefix Tree)
//!

use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Trie<T> {
    childs: HashMap<char, Trie<T>>,
    value: Option<T>,
}

impl<T: Default> Trie<T> {
    pub fn insert(&mut self, key: &[char], value: T) {
        match key.get(0) {
            Some(c) => self
                .childs
                .entry(*c)
                .or_insert_with(Trie::default)
                .insert(&key[1..], value),
            None => self.value = Some(value),
        }
    }

    pub fn get(&self, key: &[char]) -> Option<&T> {
        match key.get(0) {
            Some(c) => self.childs.get(c).and_then(|node| node.get(&key[1..])),
            None => self.value.as_ref(),
        }
    }
}

#[test]
fn test() {
    let mut trie = Trie::default();
    let key = "abcde".chars().collect::<Vec<_>>();
    trie.insert(&key, 123);

    assert_eq!(Some(&123), trie.get(&key[0..5]));
    assert_eq!(None, trie.get(&key[0..4]));
    assert_eq!(None, trie.get(&key[1..4]));
}
