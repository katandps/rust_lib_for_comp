//! # 出力ヘルパー
//! バッファリングされた出力を提供する
//!
use crate::prelude::*;
#[snippet(name = "writer", doc_hidden)]
pub use writer_impl::{WriterToStdout, WriterTrait};
#[snippet(name = "writer", doc_hidden)]
#[rustfmt::skip]
mod writer_impl {
    use super::{stdout, BufWriter, Display, Write};
    pub trait WriterTrait {
        /// # Sを出力して改行
        fn ln<S: Display>(&mut self, s: S);
        /// # Sを出力
        fn out<S: Display>(&mut self, s: S);
        /// # iをseparatorで結合して出力し、改行する
        fn join<S: Display, I: IntoIterator<Item = S>>(&mut self, i: I, separator: &str);
        /// iをbitごとに出力し、改行する
        fn bits(&mut self, i: i64, len: usize);
        /// # バッファをクリアする
        fn flush(&mut self);
    }

    #[derive(Clone, Debug, Default)]
    pub struct WriterToStdout {
        buf: String,
    }

    impl WriterTrait for WriterToStdout {
        fn ln<S: Display>(&mut self, s: S) {
            self.out(s);
            self.buf.push('\n')
        }
        fn out<S: Display>(&mut self, s: S) {
            self.buf.push_str(&s.to_string());
        }
        fn join<S: Display, I: IntoIterator<Item = S>>(&mut self, i: I, separator: &str) {
            i.into_iter().fold("", |sep, arg| {
                self.buf.push_str(&format!("{}{}", sep, arg));
                separator
            });
            self.buf.push('\n');
        }
        fn bits(&mut self, i: i64, len: usize) {
            (0..len).for_each(|b| self.buf.push_str(&format!("{}", i >> b & 1)));
            self.buf.push('\n');
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
