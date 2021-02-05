#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

const INF: usize = std::usize::MAX / 100000;

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let (n, m) = reader.u2();
    let ab = reader.uv2(m);
    let k = reader.u();
    let c = reader.uv(k);

    let mut g = vec![Vec::new(); n + 1];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut dist = vec![vec![INF; k + 1]; k + 1];

    // c_i からc_jへの最短距離を求める
    for i in 1..=k {
        let start = c[i - 1];
        let mut d = vec![INF; n + 1];
        d[start] = 0;

        let mut q = VecDeque::new();
        let mut memo = HashSet::new();
        q.push_front(start);
        memo.insert(start);
        while !q.is_empty() {
            let from = q.pop_front().unwrap();
            for &to in &g[from] {
                if memo.contains(&to) {
                    continue;
                }
                d[to] = d[from] + 1;
                q.push_back(to);
                memo.insert(to);
            }
        }
        for j in 1..=k {
            dist[i][j] = d[c[j - 1]];
        }
    }

    let mut ans = INF;
    // iスタートでそれぞれの巡回セールスマンを解く
    let mut dp = vec![vec![INF; k + 1]; (1 << k) + 1];
    for i in 0..k {
        let d = dfs(1 << i, i, k, &mut dp, &dist);
        ans = min(ans, d);
    }
    if ans == INF {
        println!("{}", -1);
        return;
    }
    println!("{}", ans + 1);
}

fn dfs(s: usize, v: usize, n: usize, dp: &mut Vec<Vec<usize>>, dist: &Vec<Vec<usize>>) -> usize {
    if dp[s][v] != INF {
        dp[s][v]
    } else if s == (1 << n) - 1 {
        dp[s][v] = 0;
        0
    } else {
        let mut ans = INF;
        for u in 0..n {
            // sにuが含まれないなら、uに移動する計算をする
            if (s >> u) & 1 == 0 && dist[v + 1][u + 1] != INF {
                ans = min(ans, dfs(s | (1 << u), u, n, dp, dist) + dist[v + 1][u + 1]);
            }
        }
        dp[s][v] = ans;
        ans
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
