use rust_lib_for_comp::util::io_util::{ReadHelper, ReaderTrait};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct ManyAPlusB128Bit;
impl verify::Solver for ManyAPlusB128Bit {
    const PROBLEM_ID: &'static str = "many_aplusb";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        for _ in 0..reader.v::<usize>() {
            let (a, b) = reader.v2::<i64, i64>();
            writeln!(write, "{}", a + b).ok();
        }
    }
}
#[test]
fn test() {
    ManyAPlusB128Bit::assert(
        "3
        1 2
        11 22
        1000000000000000000 1000000000000000000",
        "3
        33
        2000000000000000000",
    );
}
