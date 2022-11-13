mod binary_trie;

use criterion::criterion_main;
criterion_main!(binary_trie::insert);
