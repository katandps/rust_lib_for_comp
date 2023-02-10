// use criterion::{criterion_group, Criterion};

// // use rust_lib_for_comp::{
// //     algo::xor_shift::XorShift,
// //     data_structure::{
// //         binary_trie::BinaryTrie, dynamic_segment_tree::DynamicSegmentTree, treap::Treap,
// //     },
// //     prelude::{binary_operation::addition::Addition, HashSet},
// // };

// fn insert_to_set(c: &mut Criterion) {
//     //     let mut xorshift = XorShift::default();
//     //     c.bench_function("Insert 100 entries to FxHashSet", |b| {
//     //         b.iter(|| {
//     //             let mut set = HashSet::default();
//     //             for _ in 0..100 {
//     //                 set.insert(xorshift.rand(1 << 60));
//     //             }
//     //         });
//     //     });
//     //     c.bench_function("Insert 100 entries to HashSet", |b| {
//     //         b.iter(|| {
//     //             let mut set = std::collections::HashSet::new();
//     //             for _ in 0..100 {
//     //                 set.insert(xorshift.rand(1 << 60));
//     //             }
//     //         });
//     //     });
//     //     c.bench_function("Insert 100 entries to 60-bits BinaryTrie", |b| {
//     //         b.iter(|| {
//     //             let mut trie = BinaryTrie::new(60);
//     //             for _ in 0..100 {
//     //                 trie.insert(xorshift.rand(1 << 60));
//     //             }
//     //         })
//     //     });
//     //     c.bench_function("Insert 100 entries to 32-bits BinaryTrie", |b| {
//     //         b.iter(|| {
//     //             let mut trie = BinaryTrie::new(32);
//     //             for _ in 0..100 {
//     //                 trie.insert(xorshift.rand(1 << 32));
//     //             }
//     //         })
//     //     });
//     //     c.bench_function("Insert 100 entries to Treap", |b| {
//     //         b.iter(|| {
//     //             let mut treap = Treap::default();
//     //             for _ in 0..100 {
//     //                 treap.insert(xorshift.rand(1 << 60));
//     //             }
//     //         })
//     //     });
//     //     c.bench_function("Add random 100 entries to Dynamic Segment Tree", |b| {
//     //         b.iter(|| {
//     //             let mut segtree = DynamicSegmentTree::<Addition<i64>>::default();
//     //             for _ in 0..100 {
//     //                 segtree.set(xorshift.rand(1 << 60) as i64, xorshift.rand(1 << 30) as i64);
//     //             }
//     //         })
//     //     });
// }

// criterion_group!(insert, insert_to_set);
