// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/DSL_5_B
use binary_indexed_tree_2d::dsl_5_b;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let lr = io.vec4::<usize, usize, usize, usize>(n);
    let ans = dsl_5_b(n, &lr);
    io.out(ans.line());
    io.flush();
}
