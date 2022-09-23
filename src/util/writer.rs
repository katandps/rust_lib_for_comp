//! # 出力ヘルパー
//! バッファリングされた出力を提供する
//!
//! ## 使い方
//! std::io::Writeを実装している型を渡して初期化する
//! ```
//! # use rust_lib_for_comp::util::writer::*;
//! let mut buf = Vec::new();
//! {
//!     let mut writer = Writer::new(&mut buf);
//!     writer.out(123);
//!     writer.ln(456);
//!     writer.join(&[1, 2, 3, 4, 5], " ");
//!     writer.bits(13, 5);
//! }
//! assert_eq!(Ok("123456\n1 2 3 4 5\n10110\n"), std::str::from_utf8(&buf));
//! ```
use crate::prelude::*;

#[snippet(name = "writer", doc_hidden)]
#[rustfmt::skip]
pub struct Writer<W: Write> {
    writer: BufWriter<W>,
}

#[snippet(name = "writer", doc_hidden)]
#[rustfmt::skip]
impl<W: Write> Writer<W> {
    pub fn new(write: W) -> Self {
        Self {
            writer: BufWriter::new(write),
        }
    }

    /// sを出力し、改行する
    pub fn ln<S: Display>(&mut self, s: S) {
        writeln!(self.writer, "{}", s).expect("Failed to write.")
    }

    /// sを出力する(改行しない)
    pub fn out<S: Display>(&mut self, s: S) {
        write!(self.writer, "{}", s).expect("Failed to write.")
    }

    /// vをseparatorで結合して出力し、改行する
    pub fn join<S: Display>(&mut self, v: &[S], separator: &str) {
        v.iter().fold("", |sep, arg| {
            write!(self.writer, "{}{}", sep, arg).expect("Failed to write.");
            separator
        });
        writeln!(self.writer).expect("Failed to write.");
    }

    /// iをbitごとに出力する
    pub fn bits(&mut self, i: i64, len: usize) {
        (0..len).for_each(|b| write!(self.writer, "{}", i >> b & 1).expect("Failed to write."));
        writeln!(self.writer).expect("Failed to write.")
    }

    /// バッファをクリアする
    ///
    /// 出力の後、さらに入力を要求する場合に使用する
    pub fn flush(&mut self) {
        let _ = self.writer.flush();
    }
}
