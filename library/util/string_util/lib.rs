use algebra::Integral;
use prelude::*;

#[codesnip::entry("string-util", doc_hidden)]
pub use string_util_impl::{AddLineTrait, BitsTrait, JoinTrait, YesTrait};
#[codesnip::entry("string-util", doc_hidden)]
mod string_util_impl {
    use super::{Display, Integral};
    pub trait AddLineTrait {
        fn line(&self) -> String;
    }

    impl<D: Display> AddLineTrait for D {
        fn line(&self) -> String {
            self.to_string() + "\n"
        }
    }
    pub trait JoinTrait {
        /// # separatorで結合する
        fn join(self, separator: &str) -> String;
    }
    impl<D: Display, I: IntoIterator<Item = D>> JoinTrait for I {
        fn join(self, separator: &str) -> String {
            let mut buf = String::new();
            self.into_iter().fold("", |sep, arg| {
                buf.push_str(&format!("{}{}", sep, arg));
                separator
            });
            buf
        }
    }

    pub trait BitsTrait {
        fn bits(self, length: Self) -> String;
    }

    impl<I: Integral> BitsTrait for I {
        fn bits(self, length: Self) -> String {
            let mut buf = String::new();
            let mut i = I::zero();
            while i < length {
                buf.push_str(&format!("{}", self >> i & I::one()));
                i += I::one();
            }
            buf + "\n"
        }
    }

    pub trait YesTrait {
        fn yes(self) -> String;
        fn no(self) -> String;
    }

    impl YesTrait for bool {
        #[inline]
        fn yes(self) -> String {
            if self { "Yes" } else { "No" }.to_string()
        }
        #[inline]
        fn no(self) -> String {
            if self { "No" } else { "Yes" }.to_string()
        }
    }
}
