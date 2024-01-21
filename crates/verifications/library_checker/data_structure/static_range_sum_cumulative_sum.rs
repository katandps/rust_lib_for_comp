use rust_lib_for_comp::{
    algebra::binary_operation::addition::Addition, data_structure::cumulative_sum::CumulativeSum,
    range_traits::RangeProduct, util::io_util::*,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct StaticRangeSumCumulativeSum;
impl verify::Solver for StaticRangeSumCumulativeSum {
    const PROBLEM_ID: &'static str = "static_range_sum";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<i64>(n);
        let cm = a.into_iter().collect::<CumulativeSum<Addition<i64>>>();
        for _ in 0..q {
            let (l, r) = reader.v2::<usize, usize>();
            writeln!(write, "{}", cm.product(l..r)).ok();
        }
    }
}
#[test]
fn test() {
    StaticRangeSumCumulativeSum::assert(
        "5 5
        1 10 100 1000 10000
        2 3
        0 3
        2 5
        3 4
        0 5",
        "100
        111
        11100
        1000
        11111",
    );
}
