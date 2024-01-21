use rust_lib_for_comp::algebra::mod_int::embedded_mod_factorial::Factorial;
use rust_lib_for_comp::util::io_util::{ReadHelper, ReaderTrait};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct UnionFindSolver;
impl verify::Solver for UnionFindSolver {
    const PROBLEM_ID: &'static str = "factorial";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        for _ in 0..reader.v() {
            let n = reader.v::<i64>();
            writeln!(write, "{}", Factorial::factorial(n)).ok();
        }
    }
}
#[test]
fn test() {
    UnionFindSolver::assert(
        "5
        0
        5
        100
        1234567
        998244352",
        "1
        120
        35305197
        972177311
        998244352",
    );
}
