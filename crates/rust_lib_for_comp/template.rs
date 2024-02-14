//! # main関数
//!
use crate::prelude::*;
use crate::util::io_debug::*;
use crate::util::io_util::*;

#[codesnip::entry(
    "template",
    include(
        "io-util",
        "dbg-macro",
        "prelude",
        "algebra",
        "chmin",
        "chmax",
        "chclamp",
        "clamp",
        "range-traits",
        "faster-hashmap",
        "string-util",
        "float-value",
        "zzz-solver"
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

#[codesnip::entry("zzz-solver", include("io-util", "string-util"))]
pub fn solve(read: impl Read, mut write: impl Write) {
    let mut reader = ReadHelper::new(read);
    let n = reader.v::<usize>();
    writeln!(write, "{}", n).ok();
    write.flush().ok();
}

#[codesnip::entry("tester")]
#[test]
fn test_1() {
    test_helper("1", "1");
}
/// # テスト実行用ヘルパー
#[codesnip::entry("tester", include("zzz-solver", "assertion"))]
#[allow(dead_code)]
pub fn test_helper(input: &str, expect: &str) {
    let mut writer = Vec::new();
    solve(input.as_bytes(), &mut writer);
    StaticAssertion::assert(expect.as_bytes(), &writer[..]);
}
