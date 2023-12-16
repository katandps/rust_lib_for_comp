//! # 文字列から入力/標準出力+読み込みに出力のセット

use crate::element::float_value::FValue;
use crate::util::io_util::*;

#[codesnip::entry("io-debug")]
pub use io_debug_impl::{Assertion, FValueAssertion, NoAssertion, StaticAssertion};
#[codesnip::entry("io-debug", include("float-value", "io-util"))]
mod io_debug_impl {
    use super::{FValue, ReadHelper, ReaderTrait, WriteHelper};

    pub trait Assertion {
        fn assert(&mut self, output: &mut WriteHelper, re_input: &mut ReadHelper);
    }

    pub struct NoAssertion;
    impl Assertion for NoAssertion {
        fn assert(&mut self, _: &mut WriteHelper, _: &mut ReadHelper) {}
    }

    pub struct StaticAssertion {
        pub expect: ReadHelper,
    }
    impl Assertion for StaticAssertion {
        fn assert(&mut self, output: &mut WriteHelper, _: &mut ReadHelper) {
            let (mut actual, mut expect) = (Vec::new(), Vec::new());
            let mut read = ReadHelper::default().add(&mut output.buf.as_bytes());
            while let Some(a) = read.next() {
                actual.push(a);
            }
            while let Some(a) = self.expect.next() {
                expect.push(a);
            }
            assert_eq!(expect, actual);
        }
    }
    pub struct FValueAssertion {
        pub expect: ReadHelper,
    }
    impl Assertion for FValueAssertion {
        fn assert(&mut self, output: &mut WriteHelper, _: &mut ReadHelper) {
            use std::str::FromStr;
            let mut read = ReadHelper::default().add(&mut output.buf.as_bytes());
            while let Some(a) = read.next() {
                if let Some(b) = self.expect.next() {
                    assert_eq!(FValue::from_str(&a), FValue::from_str(&b));
                } else {
                    panic!("expect exit but actual {}", a);
                }
            }
            assert_eq!(None, self.expect.next())
        }
    }
}

#[codesnip::entry("custom-assert")]
pub use custom_assertion_impl::ClosureAssertion;
#[codesnip::entry("custom-assert", include("io-util", "io-debug"))]
mod custom_assertion_impl {
    use super::{Assertion, ReadHelper, ReaderTrait, WriteHelper};
    pub struct ClosureAssertion {
        /// add status for assertions
        pub buf: i64,
    }
    impl Assertion for ClosureAssertion {
        fn assert(&mut self, output: &mut WriteHelper, re_input: &mut ReadHelper) {
            let mut read = ReadHelper::default().add(&mut output.buf.as_bytes());
            self.buf += read.v::<i64>();
            re_input.buf.push_back(self.buf.to_string());
        }
    }
}
