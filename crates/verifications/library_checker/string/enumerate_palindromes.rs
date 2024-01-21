use rust_lib_for_comp::{
    string::manachar::Manachar,
    util::{
        io_util::{ReadHelper, ReaderTrait},
        string_util::JoinTrait,
    },
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct EnumeratePalindromes;
impl verify::Solver for EnumeratePalindromes {
    const PROBLEM_ID: &'static str = "enumerate_palindromes";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let s = reader.chars();
        let result = Manachar::manachar(&s);
        writeln!(write, "{}", result.join(" ")).ok();
    }
}
#[test]
fn test() {
    EnumeratePalindromes::assert("abcbcba", "1 0 1 0 3 0 7 0 3 0 1 0 1");
}
