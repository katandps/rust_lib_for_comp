//! # 入力ヘルパー
//! バッファリングされた入力を提供する
//!
//! ## 使い方
//! Readを引数に取り、StringのIteratorを得る
//!
//! ```
//! # use rust_lib_for_comp::util::reader::*;
//! let mut reader = Reader::new(|| &b"-123 456.7 12345\nhogehoge\r123 456  789 012  \n 345 678\n"[..]);
//! assert_eq!(-123, reader.v());
//! assert_eq!(456.7, reader.v());
//! assert_eq!(12345, reader.v());
//! assert_eq!("hogehoge".to_string(), reader.v::<String>());
//! assert_eq!(vec![123, 456, 789, 12, 345, 678], reader.vec(6));
//! ```
use crate::prelude::*;

#[snippet(name = "reader", doc_hidden)]
pub struct Reader<F> {
    init: F,
    buf: VecDeque<String>,
}

#[snippet(name = "reader", doc_hidden)]
impl<R: BufRead, F: FnMut() -> R> Iterator for Reader<F> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.buf.is_empty() {
            let reader = (self.init)();
            for l in reader.lines().flatten() {
                self.buf
                    .append(&mut l.split_whitespace().map(ToString::to_string).collect());
            }
        }
        self.buf.pop_front()
    }
}

#[snippet(name = "reader", doc_hidden)]

impl<R: BufRead, F: FnMut() -> R> Reader<F> {
    pub fn new(init: F) -> Self {
        let buf = VecDeque::new();
        Reader { init, buf }
    }

    pub fn v<T: FromStr>(&mut self) -> T {
        let s = self.next().expect("Insufficient input.");
        s.parse().ok().expect("Failed to parse.")
    }

    pub fn v2<T1: FromStr, T2: FromStr>(&mut self) -> (T1, T2) {
        (self.v(), self.v())
    }

    pub fn v3<T1: FromStr, T2: FromStr, T3: FromStr>(&mut self) -> (T1, T2, T3) {
        (self.v(), self.v(), self.v())
    }

    pub fn v4<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr>(&mut self) -> (T1, T2, T3, T4) {
        (self.v(), self.v(), self.v(), self.v())
    }

    pub fn v5<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr, T5: FromStr>(
        &mut self,
    ) -> (T1, T2, T3, T4, T5) {
        (self.v(), self.v(), self.v(), self.v(), self.v())
    }

    pub fn vec<T: FromStr>(&mut self, length: usize) -> Vec<T> {
        (0..length).map(|_| self.v()).collect()
    }

    pub fn vec2<T1: FromStr, T2: FromStr>(&mut self, length: usize) -> Vec<(T1, T2)> {
        (0..length).map(|_| self.v2()).collect()
    }

    pub fn vec3<T1: FromStr, T2: FromStr, T3: FromStr>(
        &mut self,
        length: usize,
    ) -> Vec<(T1, T2, T3)> {
        (0..length).map(|_| self.v3()).collect()
    }

    pub fn vec4<T1: FromStr, T2: FromStr, T3: FromStr, T4: FromStr>(
        &mut self,
        length: usize,
    ) -> Vec<(T1, T2, T3, T4)> {
        (0..length).map(|_| self.v4()).collect()
    }

    pub fn chars(&mut self) -> Vec<char> {
        self.v::<String>().chars().collect()
    }

    pub fn digits(&mut self) -> Vec<i64> {
        self.v::<String>()
            .chars()
            .map(|c| (c as u8 - b'0') as i64)
            .collect()
    }

    pub fn char_map(&mut self, h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| self.chars()).collect()
    }

    /// charの2次元配列からboolのmapを作る ngで指定した壁のみfalseとなる
    pub fn bool_map(&mut self, h: usize, ng: char) -> Vec<Vec<bool>> {
        self.char_map(h)
            .iter()
            .map(|v| v.iter().map(|&c| c != ng).collect())
            .collect()
    }

    /// h*w行列を取得する
    pub fn matrix<T: FromStr>(&mut self, h: usize, w: usize) -> Vec<Vec<T>> {
        (0..h).map(|_| self.vec(w)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::io::BufReader;

    #[test]
    fn edge_cases() {
        {
            let mut reader = Reader::new(|| &b"8\n"[..]);
            assert_eq!(8u32, reader.v());
        }
        {
            let mut reader = Reader::new(|| &b"\n9\n"[..]);
            assert_eq!(9i32, reader.v());
        }
    }

    #[test]
    fn map() {
        {
            let data = vec!["...#..\n", ".###..\n", "....##\n"];
            let mut iter = data.iter();
            let mut reader = Reader::new(|| BufReader::new(iter.next().unwrap().as_bytes()));
            let res = reader.char_map(3);

            for i in 0..3 {
                let v = data[i].chars().collect_vec();
                for j in 0..6 {
                    assert_eq!(v[j], res[i][j], "i:{} j:{}", i, j);
                }
            }
        }
        {
            let data = vec!["S..#..", ".###..", "...G##", ""];
            let mut iter = data.iter();
            let mut reader = Reader::new(|| BufReader::new(iter.next().unwrap().as_bytes()));
            let res = reader.bool_map(3, '#');
            for i in 0..3 {
                let v = data[i].chars().collect_vec();
                for j in 0..6 {
                    assert_eq!(v[j] != '#', res[i][j]);
                }
            }
        }
    }

    #[test]
    fn digits() {
        let mut reader = Reader::new(|| &b"123456\n"[..]);
        let res = reader.digits();
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6]);
    }
}
