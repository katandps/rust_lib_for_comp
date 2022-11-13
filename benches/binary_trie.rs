use criterion::{criterion_group, Criterion};

use rust_lib_for_comp::{
    algo::xor_shift::XorShift,
    data_structure::{binary_trie::BinaryTrie, treap::Treap},
    prelude::HashSet,
};

fn insert_to_set(c: &mut Criterion) {
    let mut xorshift = XorShift::default();
    c.bench_function("Insert 100 entries to FxHashSet", |b| {
        b.iter(|| {
            let mut set = HashSet::default();
            for _ in 0..100 {
                set.insert(xorshift.rand(1 << 60));
            }
        });
    });
    c.bench_function("Insert 100 entries to HashSet", |b| {
        b.iter(|| {
            let mut set = std::collections::HashSet::new();
            for _ in 0..100 {
                set.insert(xorshift.rand(1 << 60));
            }
        });
    });
    c.bench_function("Insert 100 entries to BinaryTrie", |b| {
        b.iter(|| {
            let mut trie = BinaryTrie::default();
            for _ in 0..100 {
                trie.insert(xorshift.rand(1 << 60));
            }
        })
    });
    c.bench_function("Insert 100 entries to Treap", |b| {
        b.iter(|| {
            let mut treap = Treap::default();
            for _ in 0..100 {
                treap.insert(xorshift.rand(1 << 60));
            }
        })
    });
}

criterion_group!(insert, insert_to_set);
