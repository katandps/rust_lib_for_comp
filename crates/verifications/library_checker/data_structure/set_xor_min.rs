use rust_lib_for_comp::{data_structure::binary_trie::BinaryTrie, util::io_util::*};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct SetXorMin;
impl verify::Solver for SetXorMin {
    const PROBLEM_ID: &'static str = "set_xor_min";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let mut trie = BinaryTrie::new(30);
        for _ in 0..reader.v::<usize>() {
            let (c, x) = reader.v2::<usize, u64>();
            if c == 0 {
                if !trie.contains(x) {
                    trie.insert(x);
                }
            } else if c == 1 {
                if trie.contains(x) {
                    trie.erase(x);
                }
            } else {
                trie.set_xor_val(x);
                writeln!(write, "{}", trie.min_element().unwrap() ^ x).ok();
            }
        }
    }
}
#[test]
fn test() {
    SetXorMin::assert(
        "6
        0 6
        0 7
        2 5
        1 7
        1 10
        2 7",
        "2
        1
        ",
    );
}
