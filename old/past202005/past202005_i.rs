#[allow(dead_code)]
fn main() {
    let (n, q) = i::u2();
    let mut xy: Vec<Vec<usize>> = vec![(0..n).collect(), (0..n).map(|i| n * i).collect()];
    let mut k = 1;
    for _ in 0..q {
        match i::u() {
            1 => {
                let (a, b) = i::u2();
                let t = xy[k][a - 1];
                xy[k][a - 1] = xy[k][b - 1];
                xy[k][b - 1] = t;
            }
            2 => {
                let (a, b) = i::u2();
                let t = xy[1 - k][a - 1];
                xy[1 - k][a - 1] = xy[1 - k][b - 1];
                xy[1 - k][b - 1] = t;
            }
            3 => {
                k = 1 - k;
            }
            4 => {
                let (a, b) = i::u2();
                //dbg!(&(a - 1), &(b - 1), &k, &xy[k][a - 1], &xy[1 - k][b - 1]);
                println!("{}", xy[k][a - 1] + xy[1 - k][b - 1]);
            }
            _ => unreachable!(),
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
