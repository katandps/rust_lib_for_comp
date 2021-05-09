#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let n = reader.u();
    let s = reader.s();
    let t = reader.s();

    let mut sc = 0;
    let mut st = 0;
    for i in (0..n).rev() {
        if s[i] == '1' {
            sc += 1;
        }
        if t[i] == '1' {
            st += 1;
        }
        if st > sc {
            println!("{}", -1);
            return;
        }
    }
    if (sc - st) % 2 != 0 {
        println!("{}", -1);
        return;
    }

    // 1を右側から運んでこなければならないindex
    // もっとも左側にある t[i] = 1となる iより左側にs[i]が見つかる(1)か、
    // s[i] = 0, t[i] = 1となる点
    // なるべくs[i]の左側から運んでくるのが最短 ansに足す
    // また、(1)のパターンでは消去に1回操作を消費する
    let mut before_one = n; //nは未発見を表す
    let mut before_t = VecDeque::new();
    let mut ans = 0;
    for i in 0..n {
        // dbg!(i, before_one, &before_t, ans);
        match (s[i], t[i]) {
            ('1', '0') => {
                if before_one < i {
                    // 消さなければならない1を発見済み
                    ans += i - before_one;
                    before_one = n;
                } else if !before_t.is_empty() {
                    //tの位置まで運ぶ必要がある
                    let cur = before_t.pop_front().unwrap();
                    ans += i - cur;
                } else {
                    before_one = i;
                }
            }
            ('0', '1') => {
                //ここより大きいiから今のiまで運ぶ必要がある
                before_t.push_back(i);
            }
            ('1', '1') => {
                if before_one < i {
                    ans += i - before_one;
                    before_one = n;
                    before_t.push_back(i);
                } else if !before_t.is_empty() {
                    let cur = before_t.pop_front().unwrap();
                    ans += i - cur;
                    before_t.push_back(i);
                }
            }
            _ => (),
        }
    }
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
