//! # main関数
//!
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
            let (stdin, stdout) = (stdin(), stdout());
            let (stdin_lock, stdout_lock) = (stdin.lock(), stdout.lock());
            solve(stdin_lock, stdout_lock);
        })
        .unwrap()
        .join()
        .unwrap()
}

#[codesnip::entry("solver", include("io-util", "string-util"))]
pub fn solve(read: impl Read, mut write: impl Write) {
    let (mut reader, mut writer) = (ReadHelper::default().add(read), WriteHelper::default());
    let n = reader.v::<usize>();
    writer.out(n.line());
    write!(write, "{}", writer).unwrap();
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
            let mut bytes = expect.as_bytes();
            let mut assertion: StaticAssertion = StaticAssertion {
                expect: ReadHelper::default().add(&mut bytes),
            };
            let mut writer = Vec::new();
            solve(input.as_bytes(), &mut writer);
            let mut dummy = "".as_bytes();
            assertion.assert(
                &mut WriteHelper::default(),
                &mut ReadHelper::default().add(&mut dummy),
            );
        })
        .unwrap()
        .join()
        .unwrap()
}
