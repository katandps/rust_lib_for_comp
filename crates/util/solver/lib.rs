//! # main関数
//!
//! ## dependencies
//! io_util
//! prelude

#[allow(unused_imports)]
use io_debug::*;
use io_util::*;
use prelude::*;

#[snippet("template")]
#[snippet(include = "io-util")]
#[snippet(include = "io-debug")]
#[snippet(include = "dbg-macro")]
#[snippet(include = "prelude")]
#[snippet(include = "algebra")]
#[snippet(include = "min_max")]
#[snippet(include = "range-traits")]
#[snippet(include = "faster-hashmap")]
#[snippet(include = "string-util")]
#[rustfmt::skip]
pub fn main() {
    std::thread::Builder::new()
        .name("extend stack size".into())
        .stack_size(128 * 1024 * 1024)
        .spawn(move || {
            let mut io = IO::default();
            solve(&mut io);
            io.flush();
        })
        .unwrap()
        .join()
        .unwrap()
}

#[snippet("solver")]
pub fn solve<IO: ReaderTrait + WriterTrait>(io: &mut IO) {
    let _n: usize = io.v();
    io.out("\n");
}

/// # テスト実行用関数
#[snippet("tester", doc_hidden)]
#[test]
fn test() {
    let test_suits = vec!["0", "1"];
    for suit in test_suits {
        std::thread::Builder::new()
            .name("extend stack size".into())
            .stack_size(128 * 1024 * 1024)
            .spawn(move || {
                let mut io = IODebug::new(
                    suit,
                    true,
                    |_outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
                        // solverの出力はこのouterで入力される
                        // innerで出力した内容がsolver側で入力として受け取れる
                        // inner.out(outer.v::<String>())
                    },
                );
                solve(&mut io);
                io.flush();
            })
            .unwrap()
            .join()
            .unwrap()
    }
}
