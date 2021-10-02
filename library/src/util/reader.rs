//!標準入力を取得するヘルパー
use crate::*;

pub struct Reader<R: BufRead> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
}

macro_rules! prim_method {
        ($name:ident: $T: ty) => {
            pub fn $name(&mut self) -> $T {
                self.n::<$T>()
            }
        };
        ($name:ident) => {
            prim_method!($name: $name);
        }
    }
macro_rules! prim_methods {
        ($name:ident: $T:ty; $($rest:tt)*) => {
            prim_method!($name:$T);
            prim_methods!($($rest)*);
        };
        ($name:ident; $($rest:tt)*) => {
            prim_method!($name);
            prim_methods!($($rest)*);
        };
        () => ()
    }

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}
macro_rules! tuple_method {
        ($name: ident: ($($T:ident),+)) => {
            pub fn $name(&mut self) -> ($($T),+) {
                ($(replace_expr!($T self.n())),+)
            }
        }
    }
macro_rules! tuple_methods {
        ($name:ident: ($($T:ident),+); $($rest:tt)*) => {
            tuple_method!($name:($($T),+));
            tuple_methods!($($rest)*);
        };
        () => ()
    }
macro_rules! vec_method {
        ($name: ident: ($($T:ty),+)) => {
            pub fn $name(&mut self, n: usize) -> Vec<($($T),+)> {
                (0..n).map(|_|($(replace_expr!($T self.n())),+)).collect()
            }
        };
        ($name: ident: $T:ty) => {
            pub fn $name(&mut self, n: usize) -> Vec<$T> {
                (0..n).map(|_|self.n()).collect()
            }
        };
    }
macro_rules! vec_methods {
        ($name:ident: ($($T:ty),+); $($rest:tt)*) => {
            vec_method!($name:($($T),+));
            vec_methods!($($rest)*);
        };
        ($name:ident: $T:ty; $($rest:tt)*) => {
            vec_method!($name:$T);
            vec_methods!($($rest)*);
        };
        () => ()
    }
impl<R: BufRead> Reader<R> {
    pub fn new(reader: R) -> Reader<R> {
        let (buf, pos) = (Vec::new(), 0);
        Reader { reader, buf, pos }
    }
    prim_methods! {
        u: usize; i: i64; f: f64; str: String; c: char; string: String;
        u8; u16; u32; u64; u128; usize; i8; i16; i32; i64; i128; isize; f32; f64; char;
    }
    tuple_methods! {
        u2: (usize, usize); u3: (usize, usize, usize); u4: (usize, usize, usize, usize);
        i2: (i64, i64); i3: (i64, i64, i64); i4: (i64, i64, i64, i64);
        cuu: (char, usize, usize);
    }
    vec_methods! {
        uv: usize; uv2: (usize, usize); uv3: (usize, usize, usize);
        iv: i64; iv2: (i64, i64); iv3: (i64, i64, i64);
        vq: (char, usize, usize);
    }

    pub fn n<T: FromStr>(&mut self) -> T
    where
        T::Err: Debug,
    {
        self.n_op().unwrap()
    }

    pub fn n_op<T: FromStr>(&mut self) -> Option<T>
    where
        T::Err: Debug,
    {
        if self.buf.is_empty() {
            self._read_next_line();
        }
        let mut start = None;
        while self.pos != self.buf.len() {
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) => self.pos += 1,
                (b'\n', false) => self._read_next_line(),
                (_, false) => start = Some(self.pos),
            }
        }
        start.map(|s| from_utf8(&self.buf[s..self.pos]).unwrap().parse().unwrap())
    }

    fn _read_next_line(&mut self) {
        self.pos = 0;
        self.buf.clear();
        self.reader.read_until(b'\n', &mut self.buf).unwrap();
    }
    pub fn s(&mut self) -> Vec<char> {
        self.n::<String>().chars().collect()
    }
    pub fn digits(&mut self) -> Vec<i64> {
        self.n::<String>()
            .chars()
            .map(|c| (c as u8 - b'0') as i64)
            .collect()
    }
    pub fn char_map(&mut self, h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| self.s()).collect()
    }
    /// charの2次元配列からboolのmapを作る ngで指定した壁のみfalseとなる
    pub fn bool_map(&mut self, h: usize, ng: char) -> Vec<Vec<bool>> {
        self.char_map(h)
            .iter()
            .map(|v| v.iter().map(|&c| c != ng).collect())
            .collect()
    }
    /// h*w行列を取得する
    pub fn matrix(&mut self, h: usize, w: usize) -> Vec<Vec<i64>> {
        (0..h).map(|_| self.iv(w)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::io::Cursor;

    #[test]
    fn basics() {
        let cursor = Cursor::new(b"-123 456.7 12345 Hello, world!\n");
        let mut reader = Reader::new(cursor);

        assert_eq!(-123, reader.i());
        assert_eq!(456.7, reader.f());
        assert_eq!(12345, reader.u());
        assert_eq!("Hello,".to_string(), reader.str());
        assert_eq!("world!".to_string(), reader.str());

        let cursor = Cursor::new(b"123 456 789 012 345 678\n");
        let mut reader = Reader::new(cursor);

        assert_eq!(vec![123, 456, 789, 12, 345, 678], reader.uv(6));
    }

    #[test]
    fn edge_cases() {
        {
            let cursor = Cursor::new(b"8\n");
            let mut reader = Reader::new(cursor);
            assert_eq!(8u32, reader.n());
        }
        {
            let cursor = Cursor::new(b"\n9\n");
            let mut reader = Reader::new(cursor);
            assert_eq!(9i32, reader.n());
        }
        {
            let cursor = Cursor::new(b"\n\n10\n11\n");
            let mut reader = Reader::new(cursor);
            assert_eq!(10u8, reader.n());
            assert_eq!(11u8, reader.n());
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
