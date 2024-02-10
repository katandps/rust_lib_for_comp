use rust_lib_for_comp::{
    algo::slice_bounds::SliceBounds,
    data_structure::partially_persistent_union_find::PartiallyPersistentUnionFind,
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{AtCoder, Solver};

#[derive(AtCoder)]
pub struct ABC235E;
impl verify::Solver for ABC235E {
    const PROBLEM_ID: &'static str = "abc235_e";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m, q) = reader.v3::<usize, usize, usize>();
        let mut abc = reader.vec3::<usize, usize, i64>(m);
        let uvw = reader.vec3::<usize, usize, i64>(q);
        let mut uf = PartiallyPersistentUnionFind::new(n);
        abc.sort_by_key(|(_a, _b, c)| *c);
        let mut time = vec![0];
        for (a, b, c) in abc {
            uf.unite(a, b);
            time.push(c);
        }
        for (u, v, w) in uvw {
            let t = time.upper_bound(&w) - 1;
            if uf.same(u, v, t) {
                writeln!(write, "No").unwrap()
            } else {
                writeln!(write, "Yes").unwrap()
            }
        }
    }
}

#[test]
fn test() {
    ABC235E::assert(
        "5 6 3
        1 2 2
        2 3 3
        1 3 6
        2 4 5
        4 5 9
        3 5 8
        1 3 1
        3 4 7
        3 5 7",
        "Yes
        No
        Yes",
    );
    ABC235E::assert(
        "2 3 2
        1 2 100
        1 2 1000000000
        1 1 1
        1 2 2
        1 1 5",
        "Yes
        No",
    );
}
