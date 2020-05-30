#[allow(dead_code)]
fn main() {
    let n = i::u();
    let mut a = i::uv(n);
    let mut b = i::uv(n);
    let mut c = i::uv(n);
    a.sort();
    b.sort();
    c.sort();

    let mut ans = 0usize;
    let mut a_i = 0usize;
    let mut c_i = 0usize;
    //中央を固定して、左右の個数を求める
    for b_i in 0..n {
        loop {
            if a_i == n {
                break;
            }
            if a[a_i] < b[b_i] {
                a_i += 1;
            } else {
                break;
            }
        }
        loop {
            if c_i == n {
                break;
            }
            if c[c_i] <= b[b_i] {
                c_i += 1;
            } else {
                break;
            }
        }
        ans += a_i * (n - c_i);
    }
    println!("{}", ans);
    //dbg!(a, b, c);
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
