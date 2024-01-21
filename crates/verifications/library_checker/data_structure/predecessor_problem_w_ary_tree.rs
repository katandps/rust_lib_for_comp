use rust_lib_for_comp::{
    data_structure::complete_64_part_tree::Complete64PartTree, util::io_util::*,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct PredecessorProblemWAryTree;
impl verify::Solver for PredecessorProblemWAryTree {
    const PROBLEM_ID: &'static str = "predecessor_problem";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (_n, q) = reader.v2::<usize, usize>();
        let mut tree = Complete64PartTree::build(10000000);
        let t = reader.digits();
        for (i, ti) in t.iter().enumerate() {
            if *ti == 1 {
                tree.insert(i as u64);
            }
        }
        for _ in 0..q {
            let (c, k) = reader.v2::<usize, u64>();
            if c == 0 {
                tree.insert(k);
            } else if c == 1 {
                tree.remove(k);
            } else if c == 2 {
                writeln!(write, "{}", usize::from(tree.contains(k))).ok();
            } else if c == 3 {
                if tree.contains(k) {
                    writeln!(write, "{k}").ok();
                } else if let Some(ans) = tree.next(k) {
                    writeln!(write, "{ans}").ok();
                } else {
                    writeln!(write, "-1").ok();
                }
            } else if c == 4 {
                if tree.contains(k) {
                    writeln!(write, "{k}").ok();
                } else if let Some(ans) = tree.prev(k) {
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
    PredecessorProblemWAryTree::assert(
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
