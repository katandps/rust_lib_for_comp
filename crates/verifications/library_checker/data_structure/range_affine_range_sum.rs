use rust_lib_for_comp::{
    algebra::{binary_operation::addition::Addition, mod_int::ModInt, MapMonoid},
    data_structure::lazy_segment_tree::LazySegmentTree,
    element::{
        affine::{Affine, Composition},
        section::Section,
    },
    range_traits::{RangeProductMut, RangeUpdate},
    util::io_util::*,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct RangeAffineRangeSum;
impl verify::Solver for RangeAffineRangeSum {
    const PROBLEM_ID: &'static str = "range_affine_range_sum";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<ModInt>(n);
        let mut segtree = LazySegmentTree::from_slice((
            &a.iter().map(|ai| Section::new(*ai, 1)).collect::<Vec<_>>()[..],
            Self,
        ));
        for _ in 0..q {
            if 0 == reader.v::<usize>() {
                let (l, r, b, c) = reader.v4::<usize, usize, ModInt, ModInt>();
                segtree.update_range(l..r, Affine::new(b, c));
            } else {
                let (l, r) = reader.v2::<usize, usize>();
                writeln!(write, "{}", segtree.product(l..r).value).ok();
            }
        }
    }
}
impl MapMonoid for RangeAffineRangeSum {
    type Map = Composition<ModInt, Section<ModInt>>;
    type Mono = Addition<Section<ModInt>>;
}

#[test]
fn test() {
    RangeAffineRangeSum::assert(
        "5 7
        1 2 3 4 5
        1 0 5
        0 2 4 100 101
        1 0 3
        0 1 3 102 103
        1 2 5
        0 2 5 104 105
        1 0 5",
        "15
        404
        41511
        4317767",
    );
}
