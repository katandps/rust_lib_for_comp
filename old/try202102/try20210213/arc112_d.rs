#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let (h, w) = reader.u2();
    let mut map = reader.bool_map(h, '.');
    map[0][0] = true;
    map[0][w - 1] = true;
    map[h - 1][0] = true;
    map[h - 1][w - 1] = true;

    let grid = Grid::new(h, w, map);

    let mut y_grand = vec![Vec::new(); h];
    let mut x_grand = vec![Vec::new(); w];
    let mut point_list = Vec::new();
    let mut x_done = vec![false; w];
    let mut y_done = vec![false; h];

    for x in 0..w {
        for y in 0..h {
            if *grid.get(grid.key(x, y)) {
                y_grand[y].push(x);
                x_grand[x].push(y);
                point_list.push(grid.key(x, y));
                x_done[x] = true;
                y_done[y] = true;
            }
        }
    }
    let mut uf = UnionFind::new(h * w);
    for x in 0..w {
        if x_grand[x].len() < 2 {
            continue;
        }
        let y1 = x_grand[x][0];
        for &y in &x_grand[x] {
            uf.unite(grid.key(x, y), grid.key(x, y1));
        }
    }
    for y in 0..h {
        if y_grand[y].len() < 2 {
            continue;
        }
        let x1 = y_grand[y][0];
        for &x in &y_grand[y] {
            uf.unite(grid.key(x, y), grid.key(x1, y));
        }
    }

    // dbg!(&point_list, &x_done, &y_done);
    let mut ans = min(
        x_done.iter().filter(|&&b| !b).count(),
        y_done.iter().filter(|&&b| !b).count(),
    );
    let mut root = HashSet::new();
    for p in point_list {
        root.insert(uf.root(p));
    }
    ans += root.len() - 1;
    println!("{}", ans);
}

#[allow(unused_imports)]
use union_find::*;

#[allow(dead_code)]
mod union_find {
    pub struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            let mut parent = vec![0; n + 1];
            let rank = vec![0; n + 1];
            for i in 1..(n + 1) {
                parent[i] = i;
            }
            UnionFind {
                parent: parent,
                rank: rank,
            }
        }

        pub fn root(&mut self, x: usize) -> usize {
            if self.parent[x] == x {
                x
            } else {
                let p = self.parent[x];
                self.parent[x] = self.root(p);
                self.parent[x]
            }
        }

        pub fn rank(&self, x: usize) -> usize {
            self.rank[x]
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn unite(&mut self, x: usize, y: usize) {
            let mut x = self.root(x);
            let mut y = self.root(y);
            if x == y {
                return;
            }
            if self.rank(x) < self.rank(y) {
                let tmp = y;
                y = x;
                x = tmp;
            }
            if self.rank(x) == self.rank(y) {
                self.rank[x] += 1;
            }
            self.parent[x] = y;
        }
    }
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
        };
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
