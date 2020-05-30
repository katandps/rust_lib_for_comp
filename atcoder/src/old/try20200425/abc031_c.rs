#[allow(dead_code)]
fn main() {
    let n = i::u();
    let a = i::iv(n);
    let mut ans = std::i32::MIN as i64;
    let mut ao_max = std::i32::MIN as i64;
    for t in 0..n {
        let mut tk = std::i32::MIN as i64;
        for ao in 0..n {
            if t == ao {
                continue;
            }
            let mut score = 0;
            let mut ao_score = 0;
            let l = min(t, ao);
            let r = max(t, ao);
            let m = r - l + 1;
            for i in 0..m {
                if i % 2 == 0 {
                    score += a[l + i];
                }
                if i % 2 == 1 {
                    ao_score += a[l + i];
                }
            }
            //dbg!(ao_score, ao_max, score);
            if ao_score > ao_max {
                ao_max = ao_score;
                tk = score;
            }
        }
        ans = max(ans, tk);
        ao_max = std::i32::MIN as i64;
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
