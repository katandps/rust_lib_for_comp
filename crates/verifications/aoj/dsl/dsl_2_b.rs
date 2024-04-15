//! # Range Sum Query(RSQ)
// range add range sum

use rust_lib_for_comp::{
    algebra::{binary_operation::addition::Addition, mapping::add_mapping::AddMapping, MapMonoid},
    data_structure::lazy_segment_tree::LazySegmentTree,
    element::section::Section,
    range_traits::{PointUpdate, RangeProductMut},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{AizuOnlineJudge, Solver};

pub struct AddSum;
impl MapMonoid for AddSum {
    type Mono = Addition<Section<i64>>;
    type Map = AddMapping<i64, Section<i64>, Section<i64>>;
}

#[derive(AizuOnlineJudge)]
pub struct Dsl2B;
impl verify::Solver for Dsl2B {
    const PROBLEM_ID: &'static str = "DSL_2_B";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let mut segtree = LazySegmentTree::from_length((n + 1, AddSum));
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
