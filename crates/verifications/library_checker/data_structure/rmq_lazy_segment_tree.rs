use rust_lib_for_comp::algebra::{Magma, MapMonoid};
use rust_lib_for_comp::range_traits::RangeProductMut;
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::{
    algebra::binary_operation::minimization::Minimization,
    data_structure::lazy_segment_tree::LazySegmentTree,
};
use verify::{LibraryChecker, Solver};

struct MinMin;
impl MapMonoid for MinMin {
    type Mono = Minimization<i64>;
    type Func = Minimization<i64>;

    fn apply(
        &self,
        f: &<Self::Func as Magma>::M,
        value: &<Self::Mono as Magma>::M,
    ) -> <Self::Mono as Magma>::M {
        std::cmp::min(*f, *value)
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
        let mut segtree = LazySegmentTree::from_slice((&a[..], MinMin));
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
