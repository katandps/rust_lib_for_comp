use rust_lib_for_comp::{
    algebra::mod_int::ModInt,
    data_structure::dual_segment_tree::DualSegmentTree,
    element::affine::{Affine, Composition},
    range_traits::RangeUpdate,
    util::io_util::*,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct RangeAffinePointGet;
impl verify::Solver for RangeAffinePointGet {
    const PROBLEM_ID: &'static str = "range_affine_point_get";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<ModInt>(n);
        let mut segtree = DualSegmentTree::new(&a, Composition::default());
        for _ in 0..q {
            if 0 == reader.v::<usize>() {
                let (l, r, b, c) = reader.v4::<usize, usize, ModInt, ModInt>();
                segtree.update_range(l..r, Affine::new(b, c));
            } else {
                let i = reader.v::<usize>();
                writeln!(write, "{}", segtree.get(i)).ok();
            }
        }
    }
}
#[test]
fn test() {
    RangeAffinePointGet::assert(
        "5 12
        1 2 3 4 5
        0 2 4 100 101
        1 0
        1 1
        1 2
        1 3
        1 4
        0 1 3 102 103
        1 0
        1 1
        1 2
        1 3
        1 4",
        "1
        2
        401
        501
        5
        1
        307
        41005
        501
        5
        ",
    );
}
