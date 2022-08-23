//! # KMP法(クヌース・モリス・プラット法)
//! $O(N)$で単語検索を行う
//!
//! 未完成

use crate::prelude::*;

pub struct KMP<T> {
    word: Vec<T>,
    table: Vec<Option<usize>>,
}

mod kmp_impl {
    use super::KMP;
    impl<T: PartialEq + Clone> KMP<T> {
        pub fn new(word: &[T]) -> Self {
            let l = word.len();
            let mut table = vec![Some(0); l + 1];
            table[0] = None;
            let mut c = Some(0);
            for p in 0..l {
                if c.is_some() && word[p] == word[c.unwrap()] {
                    table[p] = table[c.unwrap()];
                } else {
                    table[p] = c;
                    if let Some(v) = c {
                        c = table[v];
                    }
                    while let Some(v) = c {
                        if word[p] != word[v] {
                            c = table[v]
                        }
                    }
                }
                c = Some(c.map_or(0, |c| c + 1));
            }
            table[l] = c;
            Self {
                table,
                word: word.to_vec(),
            }
        }
    }
}
