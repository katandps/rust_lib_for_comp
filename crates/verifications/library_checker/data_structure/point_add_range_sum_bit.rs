use rust_lib_for_comp::{
    algebra::binary_operation::addition::Addition,
    data_structure::binary_indexed_tree::BinaryIndexedTree, range_traits::RangeProductMut,
    util::io_util::*,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct PointAddRangeSumByBit;
impl verify::Solver for PointAddRangeSumByBit {
    const PROBLEM_ID: &'static str = "point_add_range_sum";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<i64>(n);
        let mut bit = BinaryIndexedTree::<Addition<i64>>::build(a, Addition::default());
        for _ in 0..q {
            if 0 == reader.v::<usize>() {
                let (p, x) = reader.v2::<usize, i64>();
                bit.add(p, x);
            } else {
                let (l, r) = reader.v2::<usize, usize>();
                writeln!(write, "{}", bit.product(l..r)).ok();
            }
        }
    }
}
#[test]
fn test() {
    PointAddRangeSumByBit::assert(
        "5 5
        1 2 3 4 5
        1 0 5
        1 2 4
        0 3 10
        1 0 5
        1 0 3",
        "15
        7
        25
        6",
    );
}
