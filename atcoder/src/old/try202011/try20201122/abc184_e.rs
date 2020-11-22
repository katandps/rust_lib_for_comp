#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let (h, w) = reader.uu();
    let a = reader.char_map(h);
    let grid = Grid::new(h, w, a);

    let mut start = 0;
    let mut goal = 0;
    let mut alpha = HashMap::new();
    for i in 0..h * w {
        if grid.get(i) == &'S' {
            start = i;
        }
        if grid.get(i) == &'G' {
            goal = i;
        }
        alpha.entry(grid.get(i)).or_insert(Vec::new()).push(i);
    }
    let mut q = VecDeque::new();
    let mut dest = vec![std::usize::MAX / 10000000; h * w];
    dest[start] = 0;
    let mut used_alpha = HashSet::new();
    used_alpha.insert('.');
    used_alpha.insert('#');
    q.push_back(start);
    while !q.is_empty() {
        let from = q.pop_front().unwrap();
        for to in grid.neighbor(from) {
            if grid.get(to) == &'#' {
                continue;
            }
            if dest[to] > dest[from] + 1 {
                dest[to] = dest[from] + 1;
                q.push_back(to);
            }
        }
        let c = grid.get(from);
        if !used_alpha.contains(c) {
            used_alpha.insert(*c);
            for &to in &alpha[c] {
                if from == to {
                    continue;
                }
                if dest[to] > dest[from] + 1 {
                    dest[to] = dest[from] + 1;
                    q.push_back(to);
                }
            }
        }
    }
    if dest[goal] == 1844674407370 {
        println!("{}", -1);
        return;
    }
    println!("{}", dest[goal]);
}

#[allow(unused_imports)]
use grid::*;

#[allow(dead_code)]
mod grid {
    #[derive(Debug)]
    pub struct Grid<T> {
        pub h: usize,
        pub w: usize,
        pub max: usize,
        pub map: Vec<T>,
    }

    impl<T: Clone> Grid<T> {
        pub fn new(h: usize, w: usize, input: Vec<Vec<T>>) -> Grid<T> {
            let mut map = Vec::new();
            for r in input {
                for c in r {
                    map.push(c);
                }
            }
            let max = h * w;
            Grid { h, w, max, map }
        }
        pub fn key(&self, x: usize, y: usize) -> usize {
            y * self.w + x
        }
        pub fn xy(&self, k: usize) -> (usize, usize) {
            (self.x(k), self.y(k))
        }
        pub fn x(&self, k: usize) -> usize {
            k % self.w
        }
        pub fn y(&self, k: usize) -> usize {
            k / self.w
        }
        pub fn get(&self, key: usize) -> &T {
            &self.map[key]
        }
        pub fn set(&mut self, key: usize, value: T) {
            self.map[key] = value;
        }
        pub fn neighbor(&self, key: usize) -> Vec<usize> {
            let mut ret = self.one_way(key);
            if self.x(key) > 0 {
                ret.push(key - 1);
            }
            if self.y(key) > 0 {
                ret.push(key - self.w);
            }
            ret
        }
        pub fn one_way(&self, key: usize) -> Vec<usize> {
            let mut ret = Vec::new();
            if self.x(key) + 1 < self.w {
                ret.push(key + 1);
            }
            if self.y(key) + 1 < self.h {
                ret.push(key + self.w);
            }
            ret
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
            uu: (usize, usize);
            ii: (i64, i64);
            uuu: (usize, usize, usize);
            uii: (usize, i64, i64);
            uuuu: (usize, usize, usize, usize);
            cuu: (char, usize, usize);
        }
        vec_methods! {
            uv: usize;
            uv2: (usize, usize);
            uv3: (usize, usize, usize);
            iv: i64;
            iv2: (i64, i64);
            iv3: (i64, i64, i64);
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
