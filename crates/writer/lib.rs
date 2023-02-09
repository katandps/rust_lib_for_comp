//! # 出力ヘルパー
//! バッファリングされた出力を提供する
//!
use prelude::*;

#[snippet(name = "writer", doc_hidden)]
pub use writer_impl::{WriterToStdout, WriterTrait};

#[snippet(name = "writer", doc_hidden)]
#[rustfmt::skip]
mod writer_impl {
    use super::{stdout, BufWriter, Display, Write};
    pub trait WriterTrait {
        /// # Sを出力
        fn out<S: Display>(&mut self, s: S);
        /// # バッファをクリアする
        fn flush(&mut self);
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
            if !self.buf.is_empty() {
                let stdout = stdout();
                let mut writer = BufWriter::new(stdout.lock());
                write!(writer, "{}", self.buf).expect("Failed to write.");
                let _ = writer.flush();
                self.buf.clear();
            }
        }
    }
}
