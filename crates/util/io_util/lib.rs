//! # 入出力ヘルパー
//! バッファリングされた入力/出力を提供する
//!
//! ## 使い方
//! Readを引数に取り、StringのIteratorを得る
//!
//! ```
//! # use io_util::*;
//! let mut reader = ReaderFromStr::new("-123 456.7 12345 hogehoge 123 456  789 012   345 678");
//! assert_eq!(-123, reader.v());
//! assert_eq!(456.7, reader.v());
//! assert_eq!(12345, reader.v());
//! assert_eq!("hogehoge".to_string(), reader.v::<String>());
//! assert_eq!(vec![123, 456, 789, 12, 345, 678], reader.vec(6));
//! ```

use prelude::*;

#[snippet("io-util")]
#[rustfmt::skip]
pub use io_impl::{ReaderFromStdin, ReaderFromStr, ReaderTrait, WriterToStdout, WriterTrait, IO};
#[snippet(name = "io-util", doc_hidden)]
#[rustfmt::skip]
mod io_impl {
    use super::{stdin, stdout, BufRead, BufWriter, Display, FromStr as FS, VecDeque, Write};

    #[derive(Clone, Debug, Default)]
    pub struct IO {
        reader: ReaderFromStdin,
        writer: WriterToStdout,
    }

    pub trait ReaderTrait {
        fn next(&mut self) -> Option<String>;
        fn v<T: FS>(&mut self) -> T {
            let s = self.next().expect("Insufficient input.");
            s.parse().ok().expect("Failed to parse.")
        }
        fn v2<T1: FS, T2: FS>(&mut self) -> (T1, T2) {
            (self.v(), self.v())
        }
        fn v3<T1: FS, T2: FS, T3: FS>(&mut self) -> (T1, T2, T3) {
            (self.v(), self.v(), self.v())
        }
        fn v4<T1: FS, T2: FS, T3: FS, T4: FS>(&mut self) -> (T1, T2, T3, T4) {
            (self.v(), self.v(), self.v(), self.v())
        }
        fn v5<T1: FS, T2: FS, T3: FS, T4: FS, T5: FS>(&mut self) -> (T1, T2, T3, T4, T5) {
            (self.v(), self.v(), self.v(), self.v(), self.v())
        }
        fn vec<T: FS>(&mut self, length: usize) -> Vec<T> {
            (0..length).map(|_| self.v()).collect()
        }
        fn vec2<T1: FS, T2: FS>(&mut self, length: usize) -> Vec<(T1, T2)> {
            (0..length).map(|_| self.v2()).collect()
        }
        fn vec3<T1: FS, T2: FS, T3: FS>(&mut self, length: usize) -> Vec<(T1, T2, T3)> {
            (0..length).map(|_| self.v3()).collect()
        }
        fn vec4<T1: FS, T2: FS, T3: FS, T4: FS>(&mut self, length: usize) -> Vec<(T1, T2, T3, T4)> {
            (0..length).map(|_| self.v4()).collect()
        }
        fn chars(&mut self) -> Vec<char> {
            self.v::<String>().chars().collect()
        }
        fn split(&mut self, zero: u8) -> Vec<usize> {
            self.v::<String>()
                .chars()
                .map(|c| (c as u8 - zero) as usize)
                .collect()
        }
        /// 英小文字からなる文字列の入力を $'0' = 0$ となる数値の配列で得る
        fn digits(&mut self) -> Vec<usize> {
            self.split(b'0')
        }

        /// 英小文字からなる文字列の入力を $'a' = 1$ となる数値の配列で得る
        fn lowercase(&mut self) -> Vec<usize> {
            self.split(b'a' - 1)
        }

        /// 英大文字からなる文字列の入力を $'A' = 1$ となる数値の配列で得る
        fn uppercase(&mut self) -> Vec<usize> {
            self.split(b'A' - 1)
        }

        /// 改行された文字列の入力を2次元配列とみなし、charの2次元Vecとして得る
        fn char_map(&mut self, h: usize) -> Vec<Vec<char>> {
            (0..h).map(|_| self.chars()).collect()
        }

        /// charの2次元配列からboolのmapを作る ngで指定した壁のみfalseとなる
        fn bool_map(&mut self, h: usize, ng: char) -> Vec<Vec<bool>> {
            self.char_map(h)
                .iter()
                .map(|v| v.iter().map(|&c| c != ng).collect())
                .collect()
        }

        /// 空白区切りで $h*w$ 個の要素を行列として取得する
        fn matrix<T: FS>(&mut self, h: usize, w: usize) -> Vec<Vec<T>> {
            (0..h).map(|_| self.vec(w)).collect()
        }
    }

    pub struct ReaderFromStr {
        buf: VecDeque<String>,
    }

    impl ReaderTrait for ReaderFromStr {
        fn next(&mut self) -> Option<String> {
            self.buf.pop_front()
        }
    }

    impl ReaderFromStr {
        pub fn new(src: &str) -> Self {
            Self {
                buf: src
                    .split_whitespace()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .into(),
            }
        }

        pub fn push(&mut self, src: &str) {
            for s in src.split_whitespace().map(ToString::to_string) {
                self.buf.push_back(s);
            }
        }

        pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
            Ok(Self::new(&std::fs::read_to_string(path)?))
        }
    }

    impl WriterTrait for ReaderFromStr {
        fn out<S: Display>(&mut self, s: S) {
            self.push(&s.to_string());
        }
        fn flush(&mut self) {}
    }

    #[derive(Clone, Debug, Default)]
    pub struct ReaderFromStdin {
        buf: VecDeque<String>,
    }

    impl ReaderTrait for ReaderFromStdin {
        fn next(&mut self) -> Option<String> {
            while self.buf.is_empty() {
                let stdin = stdin();
                let mut reader = stdin.lock();
                let mut l = String::new();
                reader.read_line(&mut l).unwrap();
                self.buf
                    .append(&mut l.split_whitespace().map(ToString::to_string).collect());
            }
            self.buf.pop_front()
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_cases() {
        {
            let mut reader = ReaderFromStr::new("8");
            assert_eq!(8u32, reader.v::<u32>());
        }
        {
            let mut reader = ReaderFromStr::new("9");
            assert_eq!(9i32, reader.v::<i32>());
        }
    }

    #[test]
    fn map() {
        {
            let data = "...#..\n.###..\n....##";
            let mut reader = ReaderFromStr::new(data);
            let res = reader.char_map(3);

            let v = data
                .split_whitespace()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            for i in 0..3 {
                for j in 0..6 {
                    assert_eq!(v[i][j], res[i][j], "i:{} j:{}", i, j);
                }
            }
        }
        {
            let data = "S..#..\n.###..\n...G##";
            let mut reader = ReaderFromStr::new(data);
            let v = data
                .split_whitespace()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let res = reader.bool_map(3, '#');
            for i in 0..3 {
                for j in 0..6 {
                    assert_eq!(v[i][j] != '#', res[i][j]);
                }
            }
        }
    }

    #[test]
    fn digits() {
        let mut reader = ReaderFromStr::new("123456");
        let res = reader.digits();
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6]);
    }
}

#[test]
fn default() {
    let io = IO::default();
    let cloned = io.clone();
    let debug = format!("{:?}", cloned);
    assert_eq!(
        debug.as_str(),
        "IO { reader: ReaderFromStdin { buf: [] }, writer: WriterToStdout { buf: \"\" } }"
    );
}
