//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    library_checker_solver::vertex_add_path_sum::solve(io_util::IO::default())
}
