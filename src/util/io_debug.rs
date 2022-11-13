//! # 文字列から入力/標準出力+読み込みに出力のセット

use crate::prelude::*;

#[snippet(name = "io-debug", doc_hidden)]
pub use io_debug_impl::IODebug;
#[snippet(name = "io-debug", doc_hidden)]
#[rustfmt::skip]
mod io_debug_impl {
    use super::{stdout, BufWriter, Display, ReaderFromStr, ReaderTrait, Write, WriterTrait};

    pub struct IODebug {
        reader: ReaderFromStr,
        buf: String,
    }

    impl WriterTrait for IODebug {
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
            self.reader.push(&self.buf);
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
                buf: String::new(),
            }
        }
    }
}

#[test]
fn test() {
    let mut io = IODebug::new("");
    io.out(123);
    io.ln(456);
    io.join(&[1, 2, 3, 4, 5], " ");
    io.bits(13, 5);
    io.flush();
    for &expect in &["123456", "1", "2", "3", "4", "5", "10110"] {
        assert_eq!(Some(expect.to_string()), io.next());
    }
    assert!(io.next().is_none());
}
