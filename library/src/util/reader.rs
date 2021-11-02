//!標準入力を取得するヘルパー
use crate::prelude::*;

pub struct Reader<R> {
    reader: R,
    buf: VecDeque<String>,
}

impl<R: BufRead> Iterator for Reader<R> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.buf.is_empty() {
            let mut buf = Vec::new();
            self.reader.read_to_end(&mut buf).unwrap();
            let s = from_utf8(&buf).expect("Not UTF-8 format input.");
            s.split_whitespace()
                .map(ToString::to_string)
                .for_each(|s| self.buf.push_back(s));
        }
        self.buf.pop_front()
    }
}
impl<R: BufRead> Reader<R> {
    pub fn new(reader: R) -> Reader<R> {
        Reader {
            reader,
            buf: VecDeque::new(),
        }
    }
    pub fn val<T: FromStr>(&mut self) -> T {
        self.next()
            .map(|token| token.parse().ok().expect("Failed to parse."))
            .expect("Insufficient input.")
    }
    pub fn vec<T: FromStr>(&mut self, length: usize) -> Vec<T> {
        (0..length).map(|_| self.val()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.val::<String>().chars().collect()
    }
    pub fn digits(&mut self) -> Vec<i64> {
        self.val::<String>()
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
    use std::io::Cursor;

    #[test]
    fn basics() {
        {
            let cursor = Cursor::new(b"-123 456.7 12345\nhogehoge\n");
            let mut reader = Reader::new(cursor);

            assert_eq!(-123, reader.val());
            assert_eq!(456.7, reader.val());
            assert_eq!(12345, reader.val());
            assert_eq!("hogehoge".to_string(), reader.val::<String>());
        }
        {
            let cursor = Cursor::new(b"-123 456.7 12345\rhogehoge\r");
            let mut reader = Reader::new(cursor);

            assert_eq!(-123, reader.val());
            assert_eq!(456.7, reader.val());
            assert_eq!(12345, reader.val());
            assert_eq!("hogehoge".to_string(), reader.val::<String>());
        }
        {
            let cursor = Cursor::new(b"123 456 789 012 345 678\n");
            let mut reader = Reader::new(cursor);
            assert_eq!(vec![123, 456, 789, 12, 345, 678], reader.vec(6));
        }
    }

    #[test]
    fn edge_cases() {
        {
            let cursor = Cursor::new(b"8\n");
            let mut reader = Reader::new(cursor);
            assert_eq!(8u32, reader.val());
        }
        {
            let cursor = Cursor::new(b"\n9\n");
            let mut reader = Reader::new(cursor);
            assert_eq!(9i32, reader.val());
        }
        {
            let cursor = Cursor::new(b"\n\n10\n11\n");
            let mut reader = Reader::new(cursor);
            assert_eq!(10u8, reader.val());
            assert_eq!(11u8, reader.val());
        }
    }

    #[test]
    fn map() {
        {
            let data = vec!["...#..", ".###..", "....##", ""];
            let cursor = Cursor::new(data.iter().join("\n"));
            let mut reader = Reader::new(cursor);
            let res = reader.char_map(3);
            for i in 0..3 {
                let v = data[i].chars().collect_vec();
                for j in 0..6 {
                    assert_eq!(v[j], res[i][j]);
                }
            }
        }
        {
            let data = vec!["S..#..", ".###..", "...G##", ""];
            let cursor = Cursor::new(data.iter().join("\n"));
            let mut reader = Reader::new(cursor);
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
        let cursor = Cursor::new("123456\n");
        let mut reader = Reader::new(cursor);
        let res = reader.digits();
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6]);
    }
}
