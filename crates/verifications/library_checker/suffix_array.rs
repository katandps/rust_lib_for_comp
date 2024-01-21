use rust_lib_for_comp::string::suffix_array::SuffixArray;
use rust_lib_for_comp::util::io_util::{ReadHelper, ReaderTrait};
use rust_lib_for_comp::util::string_util::JoinTrait;
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct SuffixArraySolver;
impl verify::Solver for SuffixArraySolver {
    const PROBLEM_ID: &'static str = "suffixarray";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let s = reader.lowercase();
        let result = SuffixArray::build(&s);
        writeln!(write, "{}", result.sa[1..].join(" ")).ok();
    }
}
#[test]
fn test() {
    SuffixArraySolver::assert("abcbcba", "6 0 5 3 1 4 2");
    SuffixArraySolver::assert("mississippi", "10 7 4 1 0 9 8 6 3 5 2");
    SuffixArraySolver::assert("ababacaca", "8 0 2 6 4 1 3 7 5");
    SuffixArraySolver::assert("aaaaa", "4 3 2 1 0");
}
