//! # 文字列から入力/標準出力+読み込みに出力のセット

use io_util::*;
use prelude::*;

#[snippet(name = "io-debug", doc_hidden)]
#[rustfmt::skip]
pub use io_debug_impl::IODebug;
#[snippet(name = "io-debug", doc_hidden)]
#[rustfmt::skip]
mod io_debug_impl {
    use super::{stdout, BufWriter, Display, ReaderFromStr, ReaderTrait, Write, WriterTrait};

    pub struct IODebug<F> {
        pub reader: ReaderFromStr,
        pub test_reader: ReaderFromStr,
        pub buf: String,
        enable_stdout: bool,
        flush: F,
    }

    impl<F: FnMut(&mut ReaderFromStr, &mut ReaderFromStr)> WriterTrait for IODebug<F> {
        fn out<S: Display>(&mut self, s: S) {
            self.buf.push_str(&s.to_string());
        }
        fn flush(&mut self) {
            if self.enable_stdout {
                let stdout = stdout();
                let mut writer = BufWriter::new(stdout.lock());
                write!(writer, "{}", self.buf).expect("Failed to write.");
                let _ = writer.flush();
            }
            self.test_reader.push(&self.buf);
            self.buf.clear();
            (self.flush)(&mut self.test_reader, &mut self.reader)
        }
    }

    impl<F> ReaderTrait for IODebug<F> {
        fn next(&mut self) -> Option<String> {
            self.reader.next()
        }
    }

    impl<F> IODebug<F> {
        pub fn new(initial_input: &str, enable_stdout: bool, flush: F) -> Self {
            Self {
                reader: ReaderFromStr::new(initial_input),
                test_reader: ReaderFromStr::new(""),
                buf: String::new(),
                enable_stdout,
                flush,
            }
        }
    }
}

#[test]
fn test() {
    use string_util::*;
    let mut io = IODebug::new("", false, |_: &mut ReaderFromStr, _: &mut ReaderFromStr| ());
    io.out(123);
    io.out(456.line());
    io.out(&[1, 2, 3, 4, 5].join(" ").line());
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
        false,
        |outer: &mut ReaderFromStr, inner: &mut ReaderFromStr| {
            buf += outer.v::<i64>();
            inner.out(buf);
        },
    );
    assert_eq!(100, io.v());
    io.out(1000);
    io.flush();
    assert_eq!(io.buf, "");
    assert_eq!(1100, io.v());
}
