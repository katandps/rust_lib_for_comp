#[allow(dead_code)]
fn main() {
    let n = i::u();
    let s = i::cmap(2);
    let m = 1_000_000_007usize;
    let mut tate = s[0][0] == s[1][0];
    let mut ans = if tate { 3usize } else { 6usize };
    let mut i = if tate { 1 } else { 2 };
    if i >= n {
        println!("{}", ans);
        return;
    }
    loop {
        if tate {
            if s[0][i] == s[1][i] {
                ans = ans * 2 % m;
                i += 1;
            } else {
                ans = ans * 2 % m;
                i += 2;
                tate = false;
            }
        } else {
            if s[0][i] == s[1][i] {
                i += 1;
                tate = true;
            } else {
                ans = ans * 3 % m;
                i += 2;
            }
        }
        if i >= n {
            break;
        }
    }
    println!("{}", ans);
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

    pub fn fv(n: usize) -> Vec<f64> {
        (0..n).map(|_| f()).collect()
    }

    pub fn cmap(h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| s()).collect()
    }
}
