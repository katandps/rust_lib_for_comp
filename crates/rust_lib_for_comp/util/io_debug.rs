//! # 文字列から入力/標準出力+読み込みに出力のセット

use crate::util::io_util::*;

#[codesnip::entry("assertion", include("io-util"))]
pub struct StaticAssertion;
#[codesnip::entry("assertion", include("io-util"))]
impl StaticAssertion {
    pub fn assert(expect: impl std::io::Read, actual: impl std::io::Read) {
        let (mut expect_read, mut actual_read) = (
            ReadHelper::default().add(expect),
            ReadHelper::default().add(actual),
        );
        let (mut actual, mut expect) = (Vec::new(), Vec::new());
        while let Some(a) = actual_read.next() {
            actual.push(a);
        }
        while let Some(a) = expect_read.next() {
            expect.push(a);
        }
        assert_eq!(expect, actual);
    }
}
