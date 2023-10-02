// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    library_checker_solver::factorize::solve(io_util::IO::default());
}
