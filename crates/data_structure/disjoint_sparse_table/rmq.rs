// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq
use disjoint_sparse_table::DisjointSparseTable;
use io_util::IO;
use minimization::Minimization;
use range_traits::RangeProduct;
use reader::*;
use string_util::*;
use writer::*;

fn main() {
    let mut io = IO::default();
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let dst = DisjointSparseTable::<Minimization<i64>>::from(&a[..]);
    for _ in 0..q {
        let (l, r) = io.v2::<usize, usize>();
        io.out(dst.product(l..r).ln());
    }
    io.flush();
}
