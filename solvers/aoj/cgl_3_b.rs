//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_3_B>

use convex_hull::Polygon;
use io_util::*;
use string_util::*;
pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    io.out(usize::from(polygon.is_convex()).line());
    io.flush();
}

#[test]
fn test() {
    let io = io_debug::IODebug::new(
        "4
        0 0
        3 1
        2 3
        0 3",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new("1");
            while let Some(a) = outer.next() {
                if let Some(b) = expect.next() {
                    assert_eq!(a, b, "expect '{}' but actual '{}'", b, a);
                } else {
                    assert_eq!("expect exit but actual {}", a)
                }
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}
#[test]
fn test2() {
    let io = io_debug::IODebug::new(
        "5
        0 0
        2 0
        1 1
        2 2
        0 2",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new("0");
            while let Some(a) = outer.next() {
                if let Some(b) = expect.next() {
                    assert_eq!(a, b, "expect '{}' but actual '{}'", b, a);
                } else {
                    assert_eq!("expect exit but actual {}", a)
                }
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}
