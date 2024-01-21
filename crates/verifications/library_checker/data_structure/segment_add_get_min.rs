use rust_lib_for_comp::{
    data_structure::dynamic_li_chao_tree::DynamicLiChaoTree, util::io_util::*,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct SegmentAddGetMin;
impl verify::Solver for SegmentAddGetMin {
    const PROBLEM_ID: &'static str = "segment_add_get_min";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let mut dlct = DynamicLiChaoTree::default();
        for _ in 0..n {
            let (l, r, a, b) = reader.v4::<i64, i64, i64, i64>();
            dlct.add_segment(l..r, a, b);
        }
        for _ in 0..q {
            if reader.v::<i64>() == 0 {
                let (l, r, a, b) = reader.v4::<i64, i64, i64, i64>();
                dlct.add_segment(l..r, a, b);
            } else {
                let p = reader.v::<i64>();
                let ans = dlct.query(p);
                if ans == DynamicLiChaoTree::INF {
                    writeln!(write, "INFINITY").ok();
                } else {
                    writeln!(write, "{ans}").ok();
                }
            }
        }
    }
}
#[test]
fn test() {
    SegmentAddGetMin::assert(
        "2 8
        -3 3 -1 -1
        0 7 0 1
        1 -1
        1 -2
        1 0
        1 2
        0 -4 2 0 -10
        1 -2
        1 0
        1 2",
        "0
        1
        -1
        -3
        -10
        -10
        -3",
    );
    SegmentAddGetMin::assert(
        "0 1
        1 0",
        "INFINITY",
    );
}
