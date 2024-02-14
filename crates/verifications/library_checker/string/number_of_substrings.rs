//! # 部分文字列の種類数
use rust_lib_for_comp::{
    string::{longest_common_prefix_array::LCPArray, suffix_array::SuffixArray},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct NumberOfSubStrings;
impl verify::Solver for NumberOfSubStrings {
    const PROBLEM_ID: &'static str = "number_of_substrings";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let s = reader.lowercase();
        let sa = SuffixArray::build(&s);
        let lcp = LCPArray::build(&sa);
        let sum = lcp.lcp.iter().sum::<usize>();
        writeln!(write, "{}", s.len() * (s.len() + 1) / 2 - sum).ok();
    }
}
#[test]
fn test() {
    NumberOfSubStrings::assert("abcbcba", "21");
    NumberOfSubStrings::assert("mississippi", "53");
    NumberOfSubStrings::assert("ababacaca", "33");
    NumberOfSubStrings::assert("aaaaa", "5")
}
