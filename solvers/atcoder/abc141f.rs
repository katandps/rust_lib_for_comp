//! <https://atcoder.jp/contests/abc141/tasks/abc141_f>
//! # notice
//! atcoderでのverifyは2023/09現在不可?
use bit_matrix::*;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n: usize = io.v();
    let a: Vec<u64> = io.vec(n);
    let mut matrix = BitMatrix::new(n, 60);
    let mut b = 0;
    for &ai in &a {
        b ^= ai;
    }
    for (i, &ai) in a.iter().enumerate() {
        for j in 0..60 {
            if b >> j & 1 == 0 && ai >> j & 1 == 1 {
                matrix.set(i, j, true);
            }
        }
    }

    let mut max = 0;
    let mut all = 0;
    let _ = matrix.elimination(false);
    for (i, &ai) in a.iter().enumerate() {
        all ^= matrix.val[i].get_u64();
        max ^= ai;
    }
    io.out((all + (all ^ max)).line());
    io.flush()
}

#[test]
fn test1() {
    let io = io_debug::IODebug::new(
        "3
        3 6 5
        ",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new("12");
            while let Some(a) = outer.next() {
                assert_eq!(Some(a), expect.next())
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}

#[test]
fn test2() {
    let io = io_debug::IODebug::new(
        "4
        23 36 66 65",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new("188");
            while let Some(a) = outer.next() {
                assert_eq!(Some(a), expect.next())
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}

#[test]
fn test3() {
    let io = io_debug::IODebug::new(
        "20
        1008288677408720767 539403903321871999 1044301017184589821 215886900497862655 504277496111605629 972104334925272829 792625803473366909 972333547668684797 467386965442856573 755861732751878143 1151846447448561405 467257771752201853 683930041385277311 432010719984459389 319104378117934975 611451291444233983 647509226592964607 251832107792119421 827811265410084479 864032478037725181
        ",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new(
                "2012721721873704572",
            );
            while let Some(a) = outer.next() {
                assert_eq!(Some(a), expect.next())
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}
