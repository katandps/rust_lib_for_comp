//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    library_checker_solver::range_kth_smallest::solve(io_util::IO::default());
}
