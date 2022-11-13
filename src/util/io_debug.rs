//! # 文字列から入力/標準出力+読み込みに出力のセット

use crate::prelude::*;

#[snippet(name = "io-debug", doc_hidden)]
pub use io_debug_impl::IODebug;
#[snippet(name = "io-debug", doc_hidden)]
#[rustfmt::skip]
mod io_debug_impl {
    use super::{stdout, BufWriter, Display, ReaderFromStr, ReaderTrait, Write, WriterTrait};

    pub struct IODebug {
        pub reader: ReaderFromStr,
        pub test_reader: ReaderFromStr,
        buf: String,
    }

    impl WriterTrait for IODebug {
        fn out<S: Display>(&mut self, s: S) {
            self.buf.push_str(&s.to_string());
        }
        fn flush(&mut self) {
            let stdout = stdout();
            let mut writer = BufWriter::new(stdout.lock());
            self.test_reader.push(&self.buf);
            write!(writer, "{}", self.buf).expect("Failed to write.");
            let _ = writer.flush();
            self.buf.clear();
        }
    }

    impl ReaderTrait for IODebug {
        fn next(&mut self) -> Option<String> {
            self.reader.next()
        }
    }

    impl IODebug {
        pub fn new(str: &str) -> Self {
            Self {
                reader: ReaderFromStr::new(str),
                test_reader: ReaderFromStr::new(""),
                buf: String::new(),
            }
        }
    }
}

#[test]
fn test() {
    let mut io = IODebug::new("");
    io.out(123);
    io.out(456.ln());
    io.out(&[1, 2, 3, 4, 5].join(" "));
    io.out(13.bits(5));
    io.flush();
    for &expect in &["123456", "1", "2", "3", "4", "5", "10110"] {
        assert_eq!(Some(expect.to_string()), io.test_reader.next());
    }
    assert!(io.next().is_none());
}
