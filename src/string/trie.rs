//! # トライ木(Trie, Prefix Tree)
//!

use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Trie<K: Hash, T> {
    childs: HashMap<K, Trie<K, T>>,
    value: Option<T>,
}

impl<K: Copy + Hash + Eq + Default, T: Default> Trie<K, T> {
    pub fn insert(&mut self, key: &[K], value: T) {
        match key.first() {
            Some(c) => self
                .childs
                .entry(*c)
                .or_insert_with(Trie::default)
                .insert(&key[1..], value),
            None => self.value = Some(value),
        }
    }

    pub fn get(&self, key: &[K]) -> Option<&T> {
        match key.first() {
            Some(c) => self.childs.get(c).and_then(|node| node.get(&key[1..])),
            None => self.value.as_ref(),
        }
    }

    pub fn get_mut(&mut self, key: &[K]) -> Option<&mut T> {
        match key.first() {
            Some(c) => self
                .childs
                .get_mut(c)
                .and_then(|node| node.get_mut(&key[1..])),
            None => self.value.as_mut(),
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

    if let Some(v) = trie.get_mut(&key[..]) {
        *v = 150;
    }
    assert_eq!(Some(&150), trie.get(&key[0..5]));
}
