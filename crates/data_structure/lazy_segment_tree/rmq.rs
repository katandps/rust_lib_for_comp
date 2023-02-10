// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq
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

fn main() {
    let mut io = IO::default();
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let mut dst = LazySegmentTree::from((&a[..], MinMin));
    for _ in 0..q {
        let (l, r) = io.v2::<usize, usize>();
        io.out(dst.product(l..r).ln());
    }
    io.flush();
}
