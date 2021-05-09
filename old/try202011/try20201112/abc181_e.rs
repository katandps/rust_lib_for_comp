#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let (h, w, n, m) = reader.uuuu();
    let ab = reader.uv2(n);
    let cd = reader.uv2(m);

    let mut yoko_bt = vec![BinaryIndexedTree::new(h + 1); w + 1];
    let mut tate_bt = vec![BinaryIndexedTree::new(w + 1); h + 1];

    // 横一列に切り取ったときに、どの位置にブロックがあるか
    let mut yoko_b = vec![BTreeSet::new(); w + 1];
    // 縦
    let mut tate_b = vec![BTreeSet::new(); h + 1];

    // 番兵
    for i in 0..=w {
        yoko_b[i].insert(0);
        yoko_b[i].insert(h + 1);
    }
    for i in 0..=h {
        tate_b[i].insert(0);
        tate_b[i].insert(w + 1);
    }

    for (c, d) in cd {
        yoko_b[d].insert(c);
        tate_b[c].insert(d);
    }

    for (a, b) in ab {
        //区間を探す
        let &tate_min = tate_b[a].range(..=b).rev().next().unwrap();
        let &tate_max = tate_b[a].range(b..).next().unwrap();

        let &yoko_min = yoko_b[b].range(..=a).rev().next().unwrap();
        let &yoko_max = yoko_b[b].range(a..).next().unwrap();

        tate_bt[a].add(tate_min + 1, 1);
        tate_bt[a].add(tate_max, -1);

        yoko_bt[b].add(yoko_min + 1, 1);
        yoko_bt[b].add(yoko_max, -1);
    }

    let mut ans = h * w;
    for i in 1..=h {
        for j in 1..=w {
            if tate_bt[i].sum(j) + yoko_bt[j].sum(i) == 0 {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}

#[allow(unused_imports)]
use binary_indexed_tree::*;

#[allow(dead_code)]
mod binary_indexed_tree {
    #[derive(Clone)]
    pub struct BinaryIndexedTree {
        n: usize,
        bit: Vec<VALUE>,
    }

    type VALUE = i64;

    impl BinaryIndexedTree {
        pub fn new(n: usize) -> BinaryIndexedTree {
            let mut p = 1;
            while p <= n {
                p <<= 1;
            }
            BinaryIndexedTree {
                n: n + 1,
                bit: vec![0; p],
            }
        }

        pub fn add(&mut self, i: usize, x: VALUE) {
            let mut idx = i as i32;
            while idx < self.n as i32 {
                self.bit[idx as usize] += x;
                idx += idx & -idx;
            }
        }

        pub fn sum(&self, i: usize) -> VALUE {
            let mut ret = 0;
            let mut idx = i as i32;
            while idx > 0 {
                ret += self.bit[idx as usize];
                idx -= idx & -idx;
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
