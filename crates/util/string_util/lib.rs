use algebra::Integral;
use prelude::*;

#[snippet(name = "string-util", doc_hidden)]
#[rustfmt::skip]
pub use string_util_impl::{AddLineTrait, BitsTrait, JoinTrait};
#[snippet(name = "string-util", doc_hidden)]
#[rustfmt::skip]
mod string_util_impl {
    use super::{Display, Integral};
    pub trait AddLineTrait {
        fn ln(&self) -> String;
    }

    impl<D: Display> AddLineTrait for D {
        fn ln(&self) -> String {
            self.to_string() + "\n"
        }
    }
    pub trait JoinTrait {
        /// # separatorで結合して改行をつける
        fn join(self, separator: &str) -> String;
    }
    impl<D: Display, I: IntoIterator<Item = D>> JoinTrait for I {
        fn join(self, separator: &str) -> String {
            let mut buf = String::new();
            self.into_iter().fold("", |sep, arg| {
                buf.push_str(&format!("{}{}", sep, arg));
                separator
            });
            buf + "\n"
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
}
