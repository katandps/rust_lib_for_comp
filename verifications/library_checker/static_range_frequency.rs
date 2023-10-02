//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    library_checker_solver::static_range_frequency::solve(io_util::IO::default());
}
