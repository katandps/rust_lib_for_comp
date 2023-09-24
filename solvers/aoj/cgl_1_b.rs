//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_1_B>

use io_util::*;
use plane_float::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
    let line = Line::new(Vector::new(x1, y1), Vector::new(x2, y2));
    for _ in 0..io.v::<usize>() {
        let (x, y) = io.v2::<f64, f64>();
        let p = Vector::new(x, y);
        let result = line.reflection(p);
        io.out(format!("{} {}\n", result.x, result.y))
    }
    io.flush();
}

#[test]
fn test() {
    use float_value::FValue;
    use std::str::FromStr;
    let io = io_debug::IODebug::new(
        "0 0 3 4
        3
        2 5
        1 4
        0 3",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new(
                "4.24 3.32
            3.56 2.08
            2.88 0.84",
            );
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
#[test]
fn test2() {
    let io = io_debug::IODebug::new(
        "0 0 2 0
        3
        -1 1
        0 1
        1 1",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new(
                "-1 -1
                0 -1
                1 -1",
            );
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
