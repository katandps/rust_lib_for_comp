use rust_lib_for_comp::{
    data_structure::dynamic_li_chao_tree::DynamicLiChaoTree, util::io_util::*,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct LineAddGetMin;
impl verify::Solver for LineAddGetMin {
    const PROBLEM_ID: &'static str = "line_add_get_min";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let mut dlct = DynamicLiChaoTree::default();
        for _ in 0..n {
            let (a, b) = reader.v2::<i64, i64>();
            dlct.add_line(a, b);
        }
        for _ in 0..q {
            if reader.v::<i64>() == 0 {
                let (a, b) = reader.v2::<i64, i64>();
                dlct.add_line(a, b);
            } else {
                let p = reader.v::<i64>();
                writeln!(write, "{}", dlct.query(p)).ok();
            }
        }
    }
}
#[test]
fn test() {
    LineAddGetMin::assert(
        "2 8
        -1 -1
        0 1
        1 -1
        1 -2
        1 0
        1 2
        0 0 -10
        1 -2
        1 0
        1 2",
        "0
        1
        -1
        -3
        -10
        -10
        -10",
    );
}
