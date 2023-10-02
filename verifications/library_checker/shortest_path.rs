//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    library_checker_solver::shortest_path::solve(io_util::IO::default());
}
