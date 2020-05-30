#[allow(dead_code)]
fn main() {
    let n = i::u();
    let mut p = i::uv(n * n);

    p.reverse();
    let mut map = vec![vec![0usize; n]; n];
    let mut grid = Grid::new(n, n, map);
    let mut ans = 0;
    for i in p {
        let k = i - 1; //gridに合わせる
        let value = *grid.get(k);
        dbg!(&grid.map, i, value, ans);
        ans += value;

        grid.set(k, value + 1);

        let mut q = VecDeque::new();
        let mut memo = HashSet::new();
        memo.insert(k);
        q.push_front(k);
        while q.len() > 0 {
            let c = q.pop_front().unwrap();
            for ne in grid.neighbor(c) {
                let nene = grid.neighbor(ne);
                if nene.len() == 4 && nene.iter().min() == nene.iter().max() {}
            }
            let ne = grid.neighbor(c);
            let m: Vec<usize> = ne.iter().map(|ne| *grid.get(*ne)).collect();
            if m.iter().max() == m.iter().min() {}

            for neighbor in ne {
                if !memo.contains(&neighbor) && *grid.get(neighbor) > m + 1 {
                    grid.set(neighbor, m);
                    memo.insert(neighbor);
                    q.push_back(neighbor);
                }
            }
        }
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
        pub max: usize,
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
                max: h * w,
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
