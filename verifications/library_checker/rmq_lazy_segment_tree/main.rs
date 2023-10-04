// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

fn main() {
    solve(io_util::IO::default());
}
use algebra::*;
use io_util::*;
use lazy_segment_tree::LazySegmentTree;
use minimization::Minimization;
use range_traits::*;
use string_util::*;

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

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let mut dst = LazySegmentTree::from((&a[..], MinMin));
    for _ in 0..q {
        let (l, r) = io.v2::<usize, usize>();
        io.out(dst.product(l..r).line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
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
    ))
}
