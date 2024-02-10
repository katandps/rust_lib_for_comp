use rust_lib_for_comp::{
    algebra::miller_rabin::MillerRabin,
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct PrimalityTest;
impl verify::Solver for PrimalityTest {
    const PROBLEM_ID: &'static str = "primality_test";
    const TIME_LIMIT_MILLIS: u64 = 10000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        for _ in 0..reader.v::<usize>() {
            let n = reader.v::<u64>();
            writeln!(write, "{}", if n.is_prime() { "Yes" } else { "No" }).unwrap()
        }
    }
}
#[test]
fn test() {
    PrimalityTest::assert(
        "6
        1
        2
        3
        4
        998244353
        1000000000000000000",
        "No
        Yes
        Yes
        No
        Yes
        No",
    );
}
