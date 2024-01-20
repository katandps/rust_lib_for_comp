use rust_lib_for_comp::util::io_util::*;
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct APlusB;
impl verify::Solver for APlusB {
    const PROBLEM_ID: &'static str = "aplusb";
    const TIME_LIMIT_MILLIS: u64 = 2000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (a, b) = reader.v2::<i64, i64>();
        writeln!(write, "{}", a + b).ok();
        write.flush().ok();
    }
}
#[test]
fn test() {
    use verify::StaticAssertion;
    StaticAssertion::equals::<APlusB>("1234 5678", "6912");
    StaticAssertion::equals::<APlusB>("1000000000 1000000000", "2000000000");
}
