#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let n = reader.u();
    let a = reader.uv(n);
    let b = reader.uv(n);

    let mut ans = n; // n回ボールを書き換えれば必ずすべて食べられる
    let mut erase = 0; // ボールを消せる権利
    let mut minimum = 1_000_000_000; // 今までに見たロボが移動したときの左端

    for i in (0..n).rev() {
        if b[i] < a[i] - 1 {
            // はみ出さない
            minimum = min(minimum, a[i] - b[i]);
        } else {
            // はみ出す

            // あまりを消すパターン 今までに消した数 + 今回消すのにかかる個数
            if erase >= b[i] - (a[i] - 1) {
                ans = min(ans, erase);
            } else {
                ans = min(ans, b[i] - (a[i] - 1));
            }

            // 1つ上の値のボールを作って消すパターン
            if minimum <= a[i] {
                // すでに消えてる
                continue;
            } else {
                erase += 1;
                minimum = a[i];
            }
        }
    }
    ans = min(ans, erase);
    println!("{}", ans);
}

pub use reader::*;
#[allow(unused_imports)]
use {
    itertools::Itertools,
    std::{cmp::*, collections::*, io::*, num::*, str::*},
};

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
        ($name: ident: $method:ident: ($($T:ty),+)) => {
            pub fn $name(&mut self, n: usize) -> Vec<($($T),+)> {
                (0..n).map(|_|self.$method()).collect_vec()
            }
        };
        ($name: ident: $method:ident: $T:ty ) => {
            pub fn $name(&mut self, n: usize) -> Vec<$T> {
                (0..n).map(|_|self.$method()).collect_vec()
            }
        }
    }
    macro_rules! vec_methods {
        ($name:ident: $method:ident: ($($T:ty),+); $($rest:tt)*) => {
            vec_method!($name:$method:($($T),+));
            vec_methods!($($rest)*);
        };
        ($name:ident: $method:ident: $T:ty; $($rest:tt)*) => {
            vec_method!($name:$method:$T);
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
            uu: (usize, usize);
            ii: (i64, i64);
            uuu: (usize, usize, usize);
            uii: (usize, i64, i64);
            uuuu: (usize, usize, usize, usize);
            cuu: (char, usize, usize);
        }
        vec_methods! {
            uv: u: usize;
            uv2: uu: (usize, usize);
            uv3: uuu: (usize, usize, usize);
            iv: i: i64;
            iv2: ii: (i64, i64);
            vq: cuu: (char, usize, usize);
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
}
