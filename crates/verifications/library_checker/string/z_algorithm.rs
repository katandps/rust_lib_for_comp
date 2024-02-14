use rust_lib_for_comp::string::z_algorithm::z;
use rust_lib_for_comp::util::io_util::{ReadHelper, ReaderTrait};
use rust_lib_for_comp::util::string_util::JoinTrait;
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct ZAlgorithm;
impl verify::Solver for ZAlgorithm {
    const PROBLEM_ID: &'static str = "zalgorithm";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let s = reader.chars();
        writeln!(write, "{}", z(&s).join(" ")).unwrap()
    }
}
#[test]
fn test() {
    ZAlgorithm::assert("abcbcba", "7 0 0 0 0 0 1");
    ZAlgorithm::assert("mississippi", "11 0 0 0 0 0 0 0 0 0 0");
    ZAlgorithm::assert("ababacaca", "9 0 3 0 1 0 1 0 1");
    ZAlgorithm::assert("aaaaa", "5 4 3 2 1");
}
