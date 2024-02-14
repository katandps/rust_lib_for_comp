use rust_lib_for_comp::{
    geometry::floor_sum::floor_sum,
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct SumOfFloorOfLinear;
impl verify::Solver for SumOfFloorOfLinear {
    const PROBLEM_ID: &'static str = "sum_of_floor_of_linear";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        for _ in 0..reader.v::<usize>() {
            let (n, m, a, b) = reader.v4::<i64, i64, i64, i64>();
            writeln!(write, "{}", floor_sum(n, m, a, b)).ok();
        }
    }
}
#[test]
fn test() {
    SumOfFloorOfLinear::assert(
        "5
    4 10 6 3
    6 5 4 3
    1 1 0 0
    31415 92653 58979 32384
    1000000000 1000000000 999999999 999999999",
        "3
    13
    0
    314095480
    499999999500000000",
    );
}
