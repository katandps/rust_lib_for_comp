//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    library_checker_solver::strongly_connected_components::solve(io_util::IO::default());
}
