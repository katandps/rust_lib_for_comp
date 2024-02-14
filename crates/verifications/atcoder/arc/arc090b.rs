use rust_lib_for_comp::{
    algebra::binary_operation::addition::Addition,
    data_structure::weighted_union_find::WeightedUnionFind,
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{AtCoder, Solver};

#[derive(AtCoder)]
pub struct ARC090B;
impl verify::Solver for ARC090B {
    const PROBLEM_ID: &'static str = "arc090_b";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m) = reader.v2::<usize, usize>();
        let mut wuf = WeightedUnionFind::<Addition<i64>>::new(n);
        for _ in 0..m {
            let (x, y, d) = reader.v3::<usize, usize, i64>();
            wuf.unite(x, y, d);
            if wuf.diff(x, y) != d {
                return writeln!(write, "No").unwrap();
            }
        }
        writeln!(write, "Yes").unwrap()
    }
}

#[test]
fn test() {
    ARC090B::assert(
        "3 3
        1 2 1
        2 3 1
        1 3 2",
        "Yes",
    );
    ARC090B::assert(
        "3 3
        1 2 1
        2 3 1
        1 3 5",
        "No",
    );
    ARC090B::assert(
        "4 3
        2 1 1
        2 3 5
        3 4 2",
        "Yes",
    );
    ARC090B::assert(
        "10 3
        8 7 100
        7 9 100
        9 8 100",
        "No",
    );
    ARC090B::assert("100 0", "Yes");
}
