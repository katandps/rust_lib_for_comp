// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq
use io_util::*;
use minimization::Minimization;
use range_traits::RangeProduct;
use sparse_table::SparseTable;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let dst = SparseTable::<Minimization<i64>>::from(&a[..]);
    for _ in 0..q {
        let (l, r) = io.v2::<usize, usize>();
        io.out(dst.product(l..r).line());
    }
    io.flush();
}
