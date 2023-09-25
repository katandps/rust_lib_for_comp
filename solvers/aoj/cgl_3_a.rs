//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_3_A>
use convex_hull::Polygon;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    io.out(polygon.area().line());
    io.flush();
}

#[test]
fn test() {
    use float_value::FValue;
    use std::str::FromStr;
    let io = io_debug::IODebug::new(
        "3
        0 0
        2 2
        -1 1",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new("2.0");
            while let Some(a) = outer.next() {
                if let Some(b) = expect.next() {
                    assert_eq!(
                        FValue::from_str(&a),
                        FValue::from_str(&b),
                        "expect '{}' but actual '{}'",
                        b,
                        a
                    );
                } else {
                    assert_eq!("expect exit but actual {}", a)
                }
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}
