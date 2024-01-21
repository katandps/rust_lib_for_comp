use std::mem::swap;

use rust_lib_for_comp::{
    chmax, max,
    string::{longest_common_prefix_array::LCPArray, suffix_array::SuffixArray},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct LongestCommonSubString;
impl verify::Solver for LongestCommonSubString {
    const PROBLEM_ID: &'static str = "longest_common_substring";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let mut s = reader.chars();
        let sn = s.len();
        let mut t = reader.chars();
        s.push('#');
        s.append(&mut t);
        let sa = SuffixArray::build(&s);
        let lcp = LCPArray::build(&sa);
        let (mut max_size, mut sl, mut sr, mut tl, mut tr) = (0, 0, 0, 0, 0);
        // saの先頭は番兵(空文字)
        for i in 0..s.len() {
            let (mut i1, mut i2) = (sa[i], sa[i + 1]);
            if i1 > i2 {
                swap(&mut i1, &mut i2);
            }

            if i1 < sn && sn < i2 && chmax!(max_size, lcp[i + 1]) {
                let (a, b) = (i1, i2 - sn - 1);
                (sl, sr, tl, tr) = (a, a + max_size, b, b + max_size)
            }
        }
        writeln!(write, "{} {} {} {}", sl, sr, tl, tr).ok();
    }
}
#[test]
fn test() {
    LongestCommonSubString::assert(
        "abcdef
        abcxdef",
        "0 3 0 3",
    );
    LongestCommonSubString::assert(
        "aaa
    bbbb",
        "0 0 0 0",
    );
    LongestCommonSubString::assert(
        "abcabcabc
        cabcabcab",
        "0 8 1 9",
    );
    LongestCommonSubString::assert(
        "aaa
        aaaaa",
        "0 3 2 5",
    );
}
