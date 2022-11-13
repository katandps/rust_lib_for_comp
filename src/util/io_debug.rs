//! # 文字列から入力/標準出力+読み込みに出力のセット

use crate::prelude::*;

#[snippet(name = "io-debug", doc_hidden)]
pub use io_debug_impl::IODebug;
#[snippet(name = "io-debug", doc_hidden)]
// #[rustfmt::skip]
mod io_debug_impl {
    use super::{stdout, BufWriter, Display, ReaderFromStr, ReaderTrait, Write, WriterTrait};

    pub struct IODebug<F> {
        pub reader: ReaderFromStr,
        pub test_reader: ReaderFromStr,
        pub buf: String,
        f: F,
    }

    impl<F: FnMut(&mut ReaderFromStr, &mut ReaderFromStr)> WriterTrait for IODebug<F> {
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
            (self.f)(&mut self.test_reader, &mut self.reader)
        }
    }

    impl<F> ReaderTrait for IODebug<F> {
        fn next(&mut self) -> Option<String> {
            self.reader.next()
        }
    }

    impl<F> IODebug<F> {
        pub fn new(str: &str, f: F) -> Self {
            Self {
                reader: ReaderFromStr::new(str),
                test_reader: ReaderFromStr::new(""),
                buf: String::new(),
                f,
            }
        }
    }
}

#[test]
fn test() {
    let mut io = IODebug::new("", |_: &mut ReaderFromStr, _: &mut ReaderFromStr| ());
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

#[test]
fn interactive_test() {
    let mut buf = 100i64;
    let mut io = IODebug::new(
        "100",
        |outer: &mut ReaderFromStr, inner: &mut ReaderFromStr| {
            buf += outer.v::<i64>();
            inner.out(buf);
        },
    );
    assert_eq!(100, io.v());
    io.out(1000);
    io.flush();
    dbg!(&io.buf);
    assert_eq!(1100, io.v());
}
