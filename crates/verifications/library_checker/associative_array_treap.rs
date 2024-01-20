use rust_lib_for_comp::data_structure::treap::Treap;
use rust_lib_for_comp::util::io_util::*;
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct AssociativeArrayTreap;
impl verify::Solver for AssociativeArrayTreap {
    const PROBLEM_ID: &'static str = "associative_array";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let mut map = Treap::default();
        for _ in 0..reader.v() {
            if 0 == reader.v::<usize>() {
                let (k, v) = reader.v2::<usize, usize>();
                if map.find(&k).is_some() {
                    map.remove(&k);
                }
                map.insert(k, v);
            } else {
                let k = reader.v();
                writeln!(write, "{}", map.find(&k).unwrap_or(&0)).ok();
            }
        }
    }
}
#[test]
fn test() {
    AssociativeArrayTreap::assert(
        "8
    0 1 2
    1 1
    1 2
    0 2 3
    1 1
    1 2
    0 2 1
    1 2",
        "2
    0
    2
    3
    1",
    );
}
