use rust_lib_for_comp::{
    algo::compress::Compress,
    data_structure::complete_64_part_tree::Complete64PartTree,
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
/// # PriorityQueueの問題を座圧とw-ary-treeで解く
pub struct DoubleEndedPriorityQueueWAryTree;
impl verify::Solver for DoubleEndedPriorityQueueWAryTree {
    const PROBLEM_ID: &'static str = "double_ended_priority_queue";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let mut s = reader.vec::<i64>(n);
        let mut queries = Vec::new();
        for _ in 0..q {
            let q = reader.v::<usize>();
            if q == 0 {
                let x = reader.v();
                s.push(x);
                queries.push(x);
            } else if q == 1 {
                queries.push(std::i64::MIN + 1);
            } else {
                queries.push(std::i64::MIN + 2);
            }
        }
        let (comp, rev) = s[..].compress_with_reverse();
        let mut tree = Complete64PartTree::build(comp.len() as u64);
        let mut cnt = vec![0; comp.len()];
        for &si in comp.iter().take(n) {
            if cnt[si] == 0 {
                tree.insert(si as u64);
            }
            cnt[si] += 1;
        }
        let mut i = n;
        for q in queries {
            if q == std::i64::MIN + 1 {
                let p = tree.min().unwrap();
                cnt[p as usize] -= 1;
                if cnt[p as usize] == 0 {
                    tree.remove(p);
                }
                writeln!(write, "{}", s[rev[p as usize]]).ok();
            } else if q == std::i64::MIN + 2 {
                let p = tree.max().unwrap();
                cnt[p as usize] -= 1;
                if cnt[p as usize] == 0 {
                    tree.remove(p);
                }
                writeln!(write, "{}", s[rev[p as usize]]).ok();
            } else {
                if cnt[comp[i]] == 0 {
                    tree.insert(comp[i] as u64);
                }
                cnt[comp[i]] += 1;

                i += 1;
            }
        }
    }
}
#[test]
fn test() {
    DoubleEndedPriorityQueueWAryTree::assert(
        "4 10
        -3 0 1 3
        0 3
        2
        2
        0 -2
        0 1
        1
        1
        2
        1
        2",
        "3
        3
        -3
        -2
        1
        0
        1
        ",
    );
}
