use rust_lib_for_comp::algebra::MapMonoid;
use rust_lib_for_comp::range_traits::RangeProductMut;
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::{
    algebra::binary_operation::minimization::Minimization,
    data_structure::lazy_segment_tree::LazySegmentTree,
};
use verify::{LibraryChecker, Solver};

#[derive(Clone, Debug, Default)]
struct MinMin {
    map: Minimization<i64>,
    mono: Minimization<i64>,
}
impl MapMonoid for MinMin {
    type Mono = Minimization<i64>;
    type Map = Minimization<i64>;
    fn map(&mut self) -> &mut Self::Map {
        &mut self.map
    }
    fn monoid(&mut self) -> &mut Self::Mono {
        &mut self.mono
    }
}

#[derive(LibraryChecker)]
pub struct StaticRmqLazySegmentTree;
impl verify::Solver for StaticRmqLazySegmentTree {
    const PROBLEM_ID: &'static str = "staticrmq";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<i64>(n);
        let mut segtree = LazySegmentTree::build(&a[..], MinMin::default());
        for _ in 0..q {
            let (l, r) = reader.v2::<usize, usize>();
            writeln!(write, "{}", segtree.product(l..r)).ok();
        }
    }
}
#[test]
fn test() {
    StaticRmqLazySegmentTree::assert(
        "4 10
        2 10 1 100
        0 1
        0 2
        0 3
        0 4
        1 2
        1 3
        1 4
        2 3
        2 4
        3 4",
        "2
        2
        1
        1
        10
        1
        1
        1
        1
        100",
    );
}
