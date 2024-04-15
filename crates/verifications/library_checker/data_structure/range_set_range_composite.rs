// use rust_lib_for_comp::{
//     algebra::{
//         binary_operation::overwrite_operation::OverwriteOperation, mod_int::ModInt, Magma,
//         MapMonoid,
//     },
//     data_structure::lazy_segment_tree::LazySegmentTree,
//     element::{
//         affine::{Affine, Composition},
//         section::Section,
//     },
//     range_traits::{RangeProductMut, RangeUpdate},
//     util::io_util::*,
// };
// use verify::{LibraryChecker, Solver};

// #[derive(LibraryChecker)]
// pub struct RangeSetRangeComposite;
// impl verify::Solver for RangeSetRangeComposite {
//     const PROBLEM_ID: &'static str = "range_set_range_composite";
//     const TIME_LIMIT_MILLIS: u64 = 5000;
//     fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
//         let mut reader = ReadHelper::new(read);
//         let (n, q) = reader.v2::<usize, usize>();
//         let ab = reader.vec2::<ModInt, ModInt>(n);
//         let mut segtree = LazySegmentTree::from_slice((
//             &ab.into_iter()
//                 .map(|(a, b)| Affine::new(a, b))
//                 .collect::<Vec<_>>()[..],
//             Self,
//         ));
//         for _ in 0..q {
//             let t = reader.v::<usize>();
//             if t == 0 {
//                 let (l, r, c, d) = reader.v4::<usize, usize, ModInt, ModInt>();
//                 segtree.update_range(l..r, Some(Affine::new(c, d)));
//             } else {
//                 let (l, r, x) = reader.v3::<usize, usize, ModInt>();
//                 writeln!(write, "{}", segtree.product(l..r).apply(x)).unwrap();
//             }
//         }
//         for i in 0..n {
//             dbg!(i, segtree.product(i..=i));
//         }
//         dbg!(segtree.product(0..1));
//         dbg!(segtree.product(0..2));
//         dbg!(segtree.product(0..3));
//         dbg!(segtree.product(0..4));
//         dbg!(segtree.product(0..5));
//     }
// }

// // todo: 区間代入区間和を遅延セグメント木で解くときと同じようにSectionでCompositionを管理する
// // Compositionの設計は再考の余地あり

// impl MapMonoid for RangeSetRangeComposite {
//     type Map = OverwriteOperation<Section<Affine<ModInt>>>;
//     type Mono = Composition<ModInt>;
//     fn apply(
//         &self,
//         f: &<Self::Map as Magma>::M,
//         value: &<Self::Mono as Magma>::M,
//     ) -> <Self::Mono as Magma>::M {
//         f.clone().unwrap_or(value.clone())
//     }
// }

// #[test]
// fn test() {
//     RangeSetRangeComposite::assert(
//         "5 7
// 1 2
// 3 4
// 5 6
// 7 8
// 9 10
// 1 0 5 11
// 1 2 4 12
// 0 1 2 13 14
// 1 0 4 15
// 1 2 5 16
// 0 0 5 10 1
// 1 0 5 1",
//         "14005
// 470
// 8275
// 5500
// 111111",
//     )
// }
