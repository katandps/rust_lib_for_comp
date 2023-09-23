//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/5/CGL_5_A>
use closest_pair::ClosestPair;
use io_util::*;
use plane_float::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let (dist, _pair) = ClosestPair::closest_pair(points);
    io.out(dist.line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::new(
        "2
        0.0 0.0
        1.0 0.0",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new("1");
            while let Some(a) = outer.next() {
                assert_eq!(Some(a), expect.next())
            }
            assert_eq!(None, expect.next())
        },
    ))
}
