// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A
use dinic::Dinic;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (v, e) = io.v2::<usize, usize>();
    let uvc = io.vec3::<usize, usize, i64>(e);
    let ans = solve(v, e, &uvc);
    io.out(ans.line());
    io.flush()
}

fn solve(v: usize, _e: usize, uvc: &[(usize, usize, i64)]) -> i64 {
    let mut dinic = Dinic::new(v);
    for &(u, v, c) in uvc {
        dinic.add_edge(u, v, c);
    }
    dinic.max_flow(0, v - 1)
}

#[test]
fn test() {
    let (u, v) = (4, 5);
    let uvc = vec![(0, 1, 2), (0, 2, 1), (1, 2, 1), (1, 3, 1), (2, 3, 2)];
    assert_eq!(3, solve(u, v, &uvc));
}
