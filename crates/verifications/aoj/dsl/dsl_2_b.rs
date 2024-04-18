//! # Range Sum Query(RSQ)
// range add range sum

use rust_lib_for_comp::{
    data_structure::lazy_segment_tree::LazySegmentTree,
    range_traits::{PointUpdate, RangeProductMut},
    typical_problems::range_add_range_sum::AddSum,
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Dsl2B;
impl verify::Solver for Dsl2B {
    const PROBLEM_ID: &'static str = "DSL_2_B";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let mut segtree = LazySegmentTree::new(n + 1, AddSum::default());
        for _ in 0..q {
            if reader.v::<usize>() == 0 {
                let (i, x) = reader.v2::<usize, i64>();
                segtree.update_at(i, x);
            } else {
                let (x, y) = reader.v2::<usize, usize>();
                writeln!(write, "{}", segtree.product(x..=y).value).unwrap()
            }
        }
    }
}

#[test]
fn test() {
    Dsl2B::assert(
        "3 5
        0 1 1
        0 2 2
        0 3 3
        1 1 2
        1 2 2",
        "3
        2",
    );
}
