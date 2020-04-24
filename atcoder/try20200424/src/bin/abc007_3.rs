#[allow(dead_code)]
fn main() {
    let (r, c) = (i::u(), i::u());
    let (sy, sx) = (i::u() - 1, i::u() - 1);
    let (gy, gx) = (i::u() - 1, i::u() - 1);
    let map = i::cmap(r);

    let mut q = VecDeque::new();
    q.push_front(YX(sy, sx));
    let mut result = vec![vec![r * c; c]; r];
    result[sy][sx] = 0;
    while q.len() > 0 {
        let c = q.pop_front().unwrap();
        //dbg!(&c);
        if map[c.0 - 1][c.1] != '#' && result[c.0 - 1][c.1] > result[c.0][c.1] + 1 {
            result[c.0 - 1][c.1] = result[c.0][c.1] + 1;
            q.push_back(YX(c.0 - 1, c.1));
        }
        if map[c.0 + 1][c.1] != '#' && result[c.0 + 1][c.1] > result[c.0][c.1] + 1 {
            result[c.0 + 1][c.1] = result[c.0][c.1] + 1;
            q.push_back(YX(c.0 + 1, c.1));
        }
        if map[c.0][c.1 - 1] != '#' && result[c.0][c.1 - 1] > result[c.0][c.1] + 1 {
            result[c.0][c.1 - 1] = result[c.0][c.1] + 1;
            q.push_back(YX(c.0, c.1 - 1));
        }
        if map[c.0][c.1 + 1] != '#' && result[c.0][c.1 + 1] > result[c.0][c.1] + 1 {
            result[c.0][c.1 + 1] = result[c.0][c.1] + 1;
            q.push_back(YX(c.0, c.1 + 1));
        }
    }
    // dbg!(&result);
    println!("{}", result[gy][gx]);
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct YX(usize, usize);

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

    pub fn fv(n: usize) -> Vec<f64> {
        (0..n).map(|_| f()).collect()
    }

    pub fn cmap(h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| s()).collect()
    }
}
