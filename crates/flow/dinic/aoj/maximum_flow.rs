// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A
use dinic::grl_6_a;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (v, e) = io.v2::<usize, usize>();
    let uvc = io.vec3::<usize, usize, i64>(e);
    let ans = grl_6_a(v, e, &uvc);
    io.out(ans.line());
    io.flush()
}
