#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let (n, k) = (reader.u128(), reader.u32());
    let s = reader.s().iter().map(|c| c.into()).collect::<Vec<RPS>>();

    let mut p = 0;
    let mut len = 1;
    // 幅2^pのトーナメントの開始位置のindex % n, 勝者
    let mut map = HashMap::new();
    const WIDTH: u32 = 5;
    while k > p + WIDTH {
        let before = len;
        len *= 2u128.pow(WIDTH);
        p += WIDTH;
        for i in 0..n {
            map.insert((i, len), search(i, i + len, &s, &map, before));
        }
    }
    println!("{}", search(0, 2u128.pow(k), &s, &map, len));
}

fn search(
    left: u128,
    right: u128,
    s: &Vec<RPS>,
    map: &HashMap<(u128, u128), RPS>,
    before: u128,
) -> RPS {
    if right - left == before {
        match map.get(&(left % s.len() as u128, before)) {
            Some(c) => *c,
            _ => s[(left % s.len() as u128) as usize],
        }
    } else {
        let center = left + (right - left) / 2;
        let (l, r) = (
            search(left, center, s, map, before),
            search(center, right, s, map, before),
        );
        l.battle(r)
    }
}

#[allow(unused_imports)]
use rps::*;

///じゃんけん
#[allow(dead_code)]
mod rps {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum RPS {
        Rock,
        Paper,
        Scissors,
    }

    impl RPS {
        pub fn battle(self, rhs: Self) -> RPS {
            use RPS::*;
            match (self, rhs) {
                (Rock, Rock) => Rock,
                (Rock, Paper) => Paper,
                (Rock, Scissors) => Rock,
                (Paper, Rock) => Paper,
                (Paper, Paper) => Paper,
                (Paper, Scissors) => Scissors,
                (Scissors, Rock) => Rock,
                (Scissors, Paper) => Scissors,
                (Scissors, Scissors) => Scissors,
            }
        }
    }

    use std::fmt;

    impl fmt::Display for RPS {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let c: char = (*self).into();
            write!(f, "{}", c)
        }
    }

    impl From<char> for RPS {
        fn from(c: char) -> Self {
            use RPS::*;
            match c {
                'R' => Rock,
                'P' => Paper,
                'S' => Scissors,
                c => panic!(format!("変換できません: {}", c)),
            }
        }
    }

    impl From<&char> for RPS {
        fn from(c: &char) -> Self {
            Self::from(*c)
        }
    }

    impl From<RPS> for char {
        fn from(rps: RPS) -> Self {
            use RPS::*;
            match rps {
                Rock => 'R',
                Paper => 'P',
                Scissors => 'S',
            }
        }
    }
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
        ($name: ident: ($($T:ty),+)) => {
            pub fn $name(&mut self, n: usize) -> Vec<($($T),+)> {
                (0..n).map(|_|($(replace_expr!($T self.n())),+)).collect_vec()
            }
        };
        ($name: ident: $T:ty) => {
            pub fn $name(&mut self, n: usize) -> Vec<$T> {
                (0..n).map(|_|self.n()).collect_vec()
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
