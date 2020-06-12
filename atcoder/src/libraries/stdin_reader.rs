#[allow(unused_imports)]
use stdin_reader::StdinReader;

#[allow(dead_code)]
mod stdin_reader {
    use std::fmt::Debug;
    use std::io::*;
    use std::str::*;

    pub struct StdinReader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
        // Should never be empty
        pos: usize, // Should never be out of bounds as long as the input ends with '\n'
    }

    impl<R: BufRead> StdinReader<R> {
        pub fn new(r: R) -> StdinReader<R> {
            StdinReader {
                reader: r,
                buf: Vec::new(),
                pos: 0,
            }
        }
        pub fn next<T: FromStr>(&mut self) -> T
        where
            T::Err: Debug,
        {
            if self.buf.is_empty() {
                self._read_next_line();
            }
            let mut start = None;
            loop {
                match (self.buf[self.pos], start.is_some()) {
                    (b' ', true) | (b'\n', true) => break,
                    (_, true) | (b' ', false) => self.pos += 1,
                    (b'\n', false) => self._read_next_line(),
                    (_, false) => start = Some(self.pos),
                }
            }
            let target = &self.buf[start.unwrap()..self.pos];
            from_utf8(target).unwrap().parse().unwrap()
        }

        fn _read_next_line(&mut self) {
            self.pos = 0;
            self.buf.clear();
            if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
                panic!("Reached EOF");
            }
        }

        pub fn str(&mut self) -> String {
            self.next()
        }
        pub fn s(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }
        pub fn i(&mut self) -> i64 {
            self.next()
        }
        pub fn i2(&mut self) -> (i64, i64) {
            (self.next(), self.next())
        }
        pub fn i3(&mut self) -> (i64, i64, i64) {
            (self.next(), self.next(), self.next())
        }
        pub fn u(&mut self) -> usize {
            self.next()
        }
        pub fn u2(&mut self) -> (usize, usize) {
            (self.next(), self.next())
        }
        pub fn u3(&mut self) -> (usize, usize, usize) {
            (self.next(), self.next(), self.next())
        }
        pub fn u4(&mut self) -> (usize, usize, usize, usize) {
            (self.next(), self.next(), self.next(), self.next())
        }
        pub fn u5(&mut self) -> (usize, usize, usize, usize, usize) {
            (
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
            )
        }
        pub fn u6(&mut self) -> (usize, usize, usize, usize, usize, usize) {
            (
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
            )
        }
        pub fn f(&mut self) -> f64 {
            self.next()
        }
        pub fn f2(&mut self) -> (f64, f64) {
            (self.next(), self.next())
        }
        pub fn c(&mut self) -> char {
            self.next::<String>().pop().unwrap()
        }
        pub fn iv(&mut self, n: usize) -> Vec<i64> {
            (0..n).map(|_| self.i()).collect()
        }
        pub fn iv2(&mut self, n: usize) -> Vec<(i64, i64)> {
            (0..n).map(|_| self.i2()).collect()
        }
        pub fn iv3(&mut self, n: usize) -> Vec<(i64, i64, i64)> {
            (0..n).map(|_| self.i3()).collect()
        }
        pub fn uv(&mut self, n: usize) -> Vec<usize> {
            (0..n).map(|_| self.u()).collect()
        }
        pub fn uv2(&mut self, n: usize) -> Vec<(usize, usize)> {
            (0..n).map(|_| self.u2()).collect()
        }
        pub fn uv3(&mut self, n: usize) -> Vec<(usize, usize, usize)> {
            (0..n).map(|_| self.u3()).collect()
        }
        pub fn uv4(&mut self, n: usize) -> Vec<(usize, usize, usize, usize)> {
            (0..n).map(|_| self.u4()).collect()
        }
        pub fn fv(&mut self, n: usize) -> Vec<f64> {
            (0..n).map(|_| self.f()).collect()
        }
        pub fn cmap(&mut self, h: usize) -> Vec<Vec<char>> {
            (0..h).map(|_| self.s()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn basics() {
        let cursor = Cursor::new(b"-123 456.7 Hello, world!\n");
        let mut reader = StdinReader::new(cursor);

        assert_eq!(-123, reader.i());
        assert_eq!(456.7f64, reader.f());
        assert_eq!("Hello,".to_string(), reader.str());
        assert_eq!("world!".to_string(), reader.str());
    }

    #[test]
    fn edge_cases() {
        {
            let cursor = Cursor::new(b"8\n");
            let mut reader = StdinReader::new(cursor);
            assert_eq!(8u32, reader.next());
        }
        {
            let cursor = Cursor::new(b"\n9\n");
            let mut reader = StdinReader::new(cursor);
            assert_eq!(9i32, reader.next());
        }
        {
            let cursor = Cursor::new(b"\n\n10\n11\n");
            let mut reader = StdinReader::new(cursor);
            assert_eq!(10u8, reader.next());
            assert_eq!(11u8, reader.next());
        }
    }
}
