use rust_lib_for_comp::{
    algebra::mod_int::ModInt,
    data_structure::segment_tree::SegmentTree,
    element::affine::{Affine, Composition},
    range_traits::{PointUpdate, RangeProductMut},
    util::io_util::*,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct PointSetRangeComposite;
impl verify::Solver for PointSetRangeComposite {
    const PROBLEM_ID: &'static str = "point_set_range_composite";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let ab = reader
            .vec2::<ModInt<998_244_353>, ModInt>(n)
            .into_iter()
            .map(|(a, b)| Affine::new(a, b))
            .collect::<Vec<_>>();
        let mut segtree = SegmentTree::<Composition<ModInt>>::build(ab, Composition::default());
        for _ in 0..q {
            if 0 == reader.v::<usize>() {
                let (p, c, d) = reader.v3::<usize, ModInt, ModInt>();
                segtree.update_at(p, Affine::new(c, d));
            } else {
                let (l, r, x) = reader.v3::<usize, usize, ModInt>();
                writeln!(write, "{}", segtree.product(l..r).apply(x)).ok();
            }
        }
    }
}
#[test]
fn test() {
    PointSetRangeComposite::assert(
        "5 5
        1 2
        3 4
        5 6
        7 8
        9 10
        1 0 5 11
        1 2 4 12
        0 1 13 14
        1 0 4 15
        1 2 5 16",
        "14005
        470
        8275
        5500",
    );
}
