//! # 標準入力/標準出力のセット

use crate::prelude::*;

#[snippet(name = "in-out", doc_hidden)]
pub use io_impl::IO;
#[snippet(name = "in-out", doc_hidden)]
#[rustfmt::skip]
mod io_impl {
    use super::{ReaderFromStdin, ReaderTrait, WriterToStdout, WriterTrait};

    #[derive(Clone,Debug,Default)]
    pub struct IO {
        reader: ReaderFromStdin,
        writer: WriterToStdout,
    }

    impl ReaderTrait for IO {
        fn next(&mut self) -> Option<String> {
            self.reader.next()
        }
    }

    impl WriterTrait for IO {
        fn out<S: std::fmt::Display>(&mut self, s: S) {
            self.writer.out(s)
        }
        fn flush(&mut self) {
            self.writer.flush()
        }
    }
}
