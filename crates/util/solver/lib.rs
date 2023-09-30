//! # main関数
//!
//! ## dependencies
//! io_util
//! prelude

use io_debug::*;
use io_util::*;
use prelude::*;
use string_util::*;

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
#[snippet(include = "float_value")]
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
    let n = io.v::<usize>();
    io.out(n.line());
}

#[snippet("tester", doc_hidden)]
#[test]
fn test_1() {
    test_helper("1", "1");
}
/// # テスト実行用ヘルパー
#[snippet("tester", doc_hidden)]
#[allow(dead_code)]
fn test_helper(input: &'static str, expect: &'static str) {
    std::thread::Builder::new()
        .name("extend stack size".into())
        .stack_size(128 * 1024 * 1024)
        .spawn(move || {
            let mut io = IODebug::static_assert(input, expect);
            solve(&mut io);
            io.flush();
        })
        .unwrap()
        .join()
        .unwrap()
}
