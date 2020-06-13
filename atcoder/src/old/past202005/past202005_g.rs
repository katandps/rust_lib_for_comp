#[allow(dead_code)]
fn main() {
    let (n, x, y) = (i::u(), i::i(), i::i());
    let xy = i::iv2(n);

    //それぞれの座標に205を足して、410 * 410の平面とする

    let (x, y) = ((205 + x) as usize, (y + 205) as usize);
    let mut dame = HashSet::new();
    for (x, y) in xy {
        dame.insert(((205 + x) as usize, (205 + y) as usize));
    }

    let mut grid = Grid::new(410, 410, vec![vec![100000usize; 410]; 410]);
    grid.set(grid.key(205, 205), 0);

    let mut q = VecDeque::new();
    q.push_back((205, 205));
    let mut memo = HashSet::new();
    memo.insert((205, 205));

    while q.len() > 0 {
        let (x, y) = q.pop_front().unwrap();

        for key in grid.run(grid.key(x, y)) {
            let next_x = grid.x(key);
            let next_y = grid.y(key);
            if memo.contains(&(next_x, next_y)) {
                continue;
            }
            if dame.contains(&(next_x, next_y)) {
                memo.insert((next_x, next_y));
                continue;
            }
            grid.set(grid.key(next_x, next_y), grid.get(grid.key(x, y)) + 1);
            q.push_back((next_x, next_y));
            memo.insert((next_x, next_y));
        }
    }
    let v = *grid.get(grid.key(x, y));
    if v == 100000 {
        println!("{}", -1);
    } else {
        println!("{}", v);
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

        pub fn run(&self, key: usize) -> Vec<usize> {
            let mut ret = self.neighbor(key);
            if self.x(key) + 1 < self.w && self.y(key) + 1 < self.h {
                ret.push(key + 1 + self.w);
            }
            if self.x(key) > 0 && self.y(key) + 1 < self.h {
                ret.push(key - 1 + self.w);
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
