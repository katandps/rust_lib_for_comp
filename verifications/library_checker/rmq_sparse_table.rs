// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    library_checker_solver::rmq_sparse_table::solve(io_util::IO::default());
}
