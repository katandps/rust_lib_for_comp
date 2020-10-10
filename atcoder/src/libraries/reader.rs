pub use reader::*;

#[allow(dead_code)]
pub mod reader {
    #[allow(unused_imports)]
    use itertools::Itertools;
    use std::{fmt::Debug, io::*, str::*};

    pub struct Reader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
        pos: usize,
    }

    impl<R: BufRead> Reader<R> {
        pub fn new(reader: R) -> Reader<R> {
            let (buf, pos) = (Vec::new(), 0);
            Reader { reader, buf, pos }
        }

        pub fn n<T: FromStr>(&mut self) -> T
        where
            T::Err: Debug,
        {
            self.n_op().unwrap()
        }

        pub fn v<T: FromStr>(&mut self, n: usize) -> Vec<T>
        where
            T::Err: Debug,
        {
            (0..n).map(|_| self.n()).collect()
        }
        pub fn v2<T: FromStr, U: FromStr>(&mut self, n: usize) -> Vec<(T, U)>
        where
            T::Err: Debug,
            U::Err: Debug,
        {
            (0..n).map(|_| (self.n(), self.n())).collect()
        }
        pub fn v3<T: FromStr, U: FromStr, V: FromStr>(&mut self, n: usize) -> Vec<(T, U, V)>
        where
            T::Err: Debug,
            U::Err: Debug,
            V::Err: Debug,
        {
            (0..n).map(|_| (self.n(), self.n(), self.n())).collect()
        }
        pub fn v4<T: FromStr, U: FromStr, V: FromStr, W: FromStr>(
            &mut self,
            n: usize,
        ) -> Vec<(T, U, V, W)>
        where
            T::Err: Debug,
            U::Err: Debug,
            V::Err: Debug,
            W::Err: Debug,
        {
            (0..n)
                .map(|_| (self.n(), self.n(), self.n(), self.n()))
                .collect()
        }

        pub fn v5<T: FromStr, U: FromStr, V: FromStr, W: FromStr, X: FromStr>(
            &mut self,
            n: usize,
        ) -> Vec<(T, U, V, W, X)>
        where
            T::Err: Debug,
            U::Err: Debug,
            V::Err: Debug,
            W::Err: Debug,
            X::Err: Debug,
        {
            (0..n)
                .map(|_| (self.n(), self.n(), self.n(), self.n(), self.n()))
                .collect()
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
        pub fn matrix<T: FromStr>(&mut self, h: usize, w: usize) -> Vec<Vec<T>>
        where
            T::Err: Debug,
        {
            (0..h).map(|_| self.v(w)).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::io::Cursor;

    #[test]
    fn basics() {
        let cursor = Cursor::new(b"-123 456.7 Hello, world!\n");
        let mut reader = Reader::new(cursor);

        assert_eq!(-123, reader.n());
        assert_eq!(456.7f64, reader.n());
        assert_eq!("Hello,".to_string(), reader.n::<String>());
        assert_eq!("world!".to_string(), reader.n::<String>());

        let cursor = Cursor::new(b"123 456 789 012 345 678\n");
        let mut reader = Reader::new(cursor);

        assert_eq!(vec![123, 456, 789, 12, 345, 678], reader.v(6));
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
}
