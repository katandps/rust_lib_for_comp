//! # main関数
//!
//! ## dependencies
//! io_util
//! prelude

use crate::prelude::*;
use crate::util::io_debug::*;
use crate::util::io_util::*;
use crate::util::string_util::*;

#[codesnip::entry(
    "template",
    include(
        "io-util",
        "io-debug",
        "dbg-macro",
        "prelude",
        "algebra",
        "chmin",
        "chmax",
        "range-traits",
        "faster-hashmap",
        "string-util",
        "float-value",
        "solver"
    )
)]
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

#[codesnip::entry("solver", include("io-util", "string-util"))]
pub fn solve<IO: ReaderTrait + WriterTrait>(io: &mut IO) {
    let n = io.v::<usize>();
    io.out(n.line());
}

#[codesnip::entry("tester", include("solver", "io-debug"))]
#[test]
fn test_1() {
    test_helper("1", "1");
}
/// # テスト実行用ヘルパー
#[codesnip::entry("tester", include("solver", "io-debug"))]
#[allow(dead_code)]
pub fn test_helper(input: &'static str, expect: &'static str) {
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
