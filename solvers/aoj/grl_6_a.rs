//! <https://onlinejudge.u-aizu.ac.jp/problems/GRL_6_A>

use dinic::Dinic;

pub fn solve(v: usize, _e: usize, uvc: &[(usize, usize, i64)]) -> i64 {
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
