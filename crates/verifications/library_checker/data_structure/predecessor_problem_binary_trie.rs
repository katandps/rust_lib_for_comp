use rust_lib_for_comp::{data_structure::binary_trie::BinaryTrie, util::io_util::*};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct PredecessorProblemBinaryTrie;
impl verify::Solver for PredecessorProblemBinaryTrie {
    const PROBLEM_ID: &'static str = "predecessor_problem";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let mut trie = BinaryTrie::new(n.next_power_of_two().trailing_zeros() as i32 + 1);
        let t = reader.digits();
        for (i, ti) in t.iter().enumerate() {
            if *ti == 1 {
                trie.insert(i as u64);
            }
        }
        for _ in 0..q {
            let (c, k) = reader.v2::<usize, u64>();
            if c == 0 {
                if !trie.contains(k) {
                    trie.insert(k);
                }
            } else if c == 1 {
                if trie.contains(k) {
                    trie.erase(k);
                }
            } else if c == 2 {
                writeln!(write, "{}", usize::from(trie.contains(k))).ok();
            } else if c == 3 {
                if trie.contains(k) {
                    writeln!(write, "{k}").ok();
                } else if let Some(ans) = trie.next(k) {
                    writeln!(write, "{ans}").ok();
                } else {
                    writeln!(write, "-1").ok();
                }
            } else if c == 4 {
                if trie.contains(k) {
                    writeln!(write, "{k}").ok();
                } else if let Some(ans) = trie.prev(k) {
                    writeln!(write, "{ans}").ok();
                } else {
                    writeln!(write, "-1").ok();
                }
            }
        }
    }
}
#[test]
fn test() {
    PredecessorProblemBinaryTrie::assert(
        "6 9
        010101
        3 3
        4 3
        4 0
        0 4
        1 3
        2 4
        2 3
        3 3
        4 3",
        "3
        3
        -1
        1
        0
        4
        1",
    );
}
