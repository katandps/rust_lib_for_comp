//! # 出力ヘルパー
//! バッファリングされた出力を提供する
//!
use crate::prelude::*;
#[snippet(name = "writer", doc_hidden)]
pub use writer_impl::{AddLineTrait, BitsTrait, JoinTrait, WriterToStdout, WriterTrait};
#[snippet(name = "writer", doc_hidden)]
// #[rustfmt::skip]
mod writer_impl {
    use super::{stdout, BufWriter, Display, Integral, Write};
    pub trait WriterTrait {
        /// # Sを出力
        fn out<S: Display>(&mut self, s: S);
        /// # バッファをクリアする
        fn flush(&mut self);
    }

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

    #[derive(Clone, Debug, Default)]
    pub struct WriterToStdout {
        buf: String,
    }

    impl WriterTrait for WriterToStdout {
        fn out<S: Display>(&mut self, s: S) {
            self.buf.push_str(&s.to_string());
        }
        fn flush(&mut self) {
            let stdout = stdout();
            let mut writer = BufWriter::new(stdout.lock());
            write!(writer, "{}", self.buf).expect("Failed to write.");
            let _ = writer.flush();
            self.buf.clear();
        }
    }
}
