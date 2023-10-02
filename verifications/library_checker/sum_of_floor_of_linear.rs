//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_floor_of_linear

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    library_checker_solver::sum_of_floor_of_linear::solve(io_util::IO::default());
}
