#[allow(dead_code)]
fn main() {
    let (h, w) = i::u2();
    let a = i::cmap(h);

    let mut grid = Grid::new(h, w, a);
    let mut q = VecDeque::new();
    for i in 0..h * w {
        let t = grid.get(i);
        if t == &'#' {
            q.push_front((i, 0));
        }
    }

    let mut map = vec![100000; h * w];
    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        map[cur.0] = min(map[cur.0], cur.1);
        for n in grid.neighbor(cur.0) {
            if grid.get(n) == &'.' {
                q.push_back((n, cur.1 + 1));
                grid.set(n, '#');
            }
        }
    }

    let mut ans = 0;
    for i in 0..h * w {
        ans = max(ans, map[i]);
    }
    println!("{}", ans);
}

#[allow(unused_imports)]
use grid::*;

#[allow(dead_code)]
mod grid {
    #[derive(Debug)]
    pub struct Grid<T> {
        pub h: usize,
        pub w: usize,
        pub map: Vec<T>,
    }

    impl<T: Clone> Grid<T> {
        pub fn new(h: usize, w: usize, map: Vec<Vec<T>>) -> Grid<T> {
            let mut flat = Vec::new();
            for r in map {
                for c in r {
                    flat.push(c);
                }
            }
            Grid {
                h: h,
                w: w,
                map: flat,
            }
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

        pub fn left(&self, key: usize) -> Option<usize> {
            if self.x(key) > 0 {
                Some(key - 1)
            } else {
                None
            }
        }
        pub fn right(&self, key: usize) -> Option<usize> {
            if self.x(key) < self.w - 1 {
                Some(key + 1)
            } else {
                None
            }
        }
        pub fn up(&self, key: usize) -> Option<usize> {
            if self.y(key) > 0 {
                Some(key - self.w)
            } else {
                None
            }
        }
        pub fn down(&self, key: usize) -> Option<usize> {
            if self.y(key) < self.h - 1 {
                Some(key + self.w)
            } else {
                None
            }
        }

        pub fn neighbor(&self, key: usize) -> Vec<usize> {
            vec![
                self.up(key),
                self.down(key),
                self.left(key),
                self.right(key),
            ]
            .iter()
            .flat_map(|i| *i)
            .collect()
        }
    }
}

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::num::*;
#[allow(unused_imports)]
use std::str::*;

#[allow(dead_code)]
mod i {
    use super::*;

    pub fn read<T: FromStr>() -> T {
        stdin()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<T>()
            .ok()
            .unwrap()
    }

    pub fn str() -> String {
        read()
    }

    pub fn s() -> Vec<char> {
        str().chars().collect()
    }

    pub fn i() -> i64 {
        read()
    }

    pub fn u() -> usize {
        read()
    }

    pub fn u2() -> (usize, usize) {
        (read(), read())
    }

    pub fn u3() -> (usize, usize, usize) {
        (read(), read(), read())
    }

    pub fn f() -> f64 {
        read()
    }

    pub fn c() -> char {
        read::<String>().pop().unwrap()
    }

    pub fn iv(n: usize) -> Vec<i64> {
        (0..n).map(|_| i()).collect()
    }

    pub fn iv2(n: usize) -> Vec<(i64, i64)> {
        (0..n).map(|_| iv(2)).map(|a| (a[0], a[1])).collect()
    }

    pub fn uv(n: usize) -> Vec<usize> {
        (0..n).map(|_| u()).collect()
    }

    pub fn uv2(n: usize) -> Vec<(usize, usize)> {
        (0..n).map(|_| uv(2)).map(|a| (a[0], a[1])).collect()
    }

    pub fn uv3(n: usize) -> Vec<(usize, usize, usize)> {
        (0..n).map(|_| uv(3)).map(|a| (a[0], a[1], a[2])).collect()
    }

    pub fn fv(n: usize) -> Vec<f64> {
        (0..n).map(|_| f()).collect()
    }

    pub fn cmap(h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| s()).collect()
    }
}
