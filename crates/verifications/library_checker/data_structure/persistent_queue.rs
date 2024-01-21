// verification-helper: PROBLEM https://judge.yosupo.jp/problem/persistent_queue
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use persistent_queue::PersistentQueue;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let mut q = PersistentQueue::default();
    let mut timeline = Vec::new();
    for _ in 0..io.v::<usize>() {
        let p = if 0 == io.v() {
            let (t, x) = io.v2::<i64, usize>();
            if t == -1 {
                q.push_rear(x, 0)
            } else {
                q.push_rear(x, timeline[t as usize])
            }
        } else {
            let t: usize = io.v::<usize>();
            let Some((v, p)) = q.pop_front(timeline[t]) else {
                unreachable!()
            };
            io.out(v.line());
            p
        };
        timeline.push(p);
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "6
        0 -1 6
        0 0 7
        1 0
        0 -1 8
        1 3
        1 1",
        "6
        8
        6",
    ));
}
