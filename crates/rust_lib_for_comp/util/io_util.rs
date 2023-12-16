//! # 入出力ヘルパー
//! バッファリングされた入力/出力を提供する
//!
//! ## 使い方
//! Readを引数に取り、StringのIteratorを得る
//!

use crate::prelude::*;

#[codesnip::entry("io-util")]
pub use io_impl::{ReadHelper, ReaderTrait, WriteHelper};
#[codesnip::entry("io-util", include("prelude"))]
mod io_impl {
    use super::{Display, Formatter, FromStr as FS, Read, VecDeque};

    pub struct IO {
        pub reader: ReadHelper,
        pub writer: WriteHelper,
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

    #[derive(Clone, Debug, Default)]
    pub struct ReadHelper {
        pub buf: VecDeque<String>,
    }

    impl ReadHelper {
        pub fn add(mut self, mut reader: impl Read) -> Self {
            let mut l = Vec::new();
            let _ = reader.read_to_end(&mut l).unwrap();
            unsafe {
                self.buf.append(
                    &mut std::str::from_utf8_unchecked_mut(&mut l)
                        .split_ascii_whitespace()
                        .map(ToString::to_string)
                        .collect(),
                );
            }
            self
        }
    }

    impl ReaderTrait for ReadHelper {
        fn next(&mut self) -> Option<String> {
            self.buf.pop_front()
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct WriteHelper {
        pub buf: String,
    }

    impl Display for WriteHelper {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.buf)
        }
    }

    impl WriteHelper {
        pub fn out<S: Display>(&mut self, s: S) {
            self.buf.push_str(&s.to_string());
        }
    }

    impl ReaderTrait for IO {
        fn next(&mut self) -> Option<String> {
            self.reader.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{io_impl::ReadHelper, *};

    #[test]
    fn test() {
        let data = "-123 456.7 12345 hogehoge 123 456  789 012   345 678";
        let mut bytes = data.as_bytes();
        let mut reader = ReadHelper::default().add(&mut bytes);
        assert_eq!(-123, reader.v::<i16>());
        assert_eq!(456.7, reader.v::<f64>());
        assert_eq!(12345, reader.v::<i32>());
        assert_eq!("hogehoge".to_string(), reader.v::<String>());
        assert_eq!(vec![123, 456, 789, 12, 345, 678], reader.vec::<i64>(6));
    }

    #[test]
    fn map() {
        {
            let data = "...#..\n.###..\n....##";
            let mut bytes = data.as_bytes();
            let mut reader = ReadHelper::default().add(&mut bytes);
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
            let mut bytes = data.as_bytes();
            let mut reader = ReadHelper::default().add(&mut bytes);
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
        let mut bytes = "123456".as_bytes();
        let mut reader = ReadHelper::default().add(&mut bytes);
        let res = reader.digits();
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6]);
    }
}
