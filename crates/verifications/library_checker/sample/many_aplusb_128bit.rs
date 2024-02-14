use rust_lib_for_comp::util::io_util::{ReadHelper, ReaderTrait};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct ManyAPlusB128Bit;
impl verify::Solver for ManyAPlusB128Bit {
    const PROBLEM_ID: &'static str = "many_aplusb_128bit";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        for _ in 0..reader.v::<usize>() {
            let (a, b) = reader.v2::<i128, i128>();
            writeln!(write, "{}", a + b).ok();
        }
    }
}
#[test]
fn test() {
    ManyAPlusB128Bit::assert(
        "5
        1 2
        11 22
        -111 -222
        10000000000000000000000000000000000000 10000000000000000000000000000000000000
        1234567890123456789012345678901234567 -10000000000000000000000000000000000000",
        "3
        33
        -333
        20000000000000000000000000000000000000
        -8765432109876543210987654321098765433",
    );
}
