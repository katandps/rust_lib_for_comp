//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_C>
use io_util::*;
use plane_float::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    for _ in 0..io.v::<usize>() {
        let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
        let (x3, y3, x4, y4) = io.v4::<f64, f64, f64, f64>();
        if let Some(result) = Segment::cross_point(
            Segment::new((x1, y1).into(), (x2, y2).into()),
            Segment::new((x3, y3).into(), (x4, y4).into()),
        ) {
            io.out(format!("{} {}\n", result.x, result.y));
        } else {
            panic!("line is parallel")
        }
    }
    io.flush();
}

#[test]
fn test() {
    use float_value::FValue;
    use std::str::FromStr;

    let io = io_debug::IODebug::new(
        "3
        0 0 2 0 1 1 1 -1
        0 0 1 1 0 1 1 0
        0 0 1 1 1 0 0 1",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new(
                "1.0000000000 0.0000000000
                0.5000000000 0.5000000000
                0.5000000000 0.5000000000",
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
        },
    );
    solve(io)
}
