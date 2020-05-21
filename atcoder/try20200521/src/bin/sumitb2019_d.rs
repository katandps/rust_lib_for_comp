#[allow(dead_code)]
fn main() {
    let n = i::u();
    let s: Vec<usize> = i::s()
        .iter()
        .map(|&c| c as i32 - '0' as i32)
        .map(|i| i as usize)
        .collect();
    let mut left = vec![n; 10];
    let mut right = vec![0; 10];

    for i in 0..n {
        left[s[i]] = min(left[s[i]], i);
        right[s[n - i - 1]] = max(right[s[n - i - 1]], n - i - 1);
    }

    //dbg!(&left, &right);

    let mut set = HashSet::new();

    for i in 1..(n - 1) {
        let k = s[i];
        for l_i in 0..10 {
            let l = left[l_i];
            for r_i in 0..10 {
                let r = right[r_i];
                //println!("{} {} {}", l, k, r);
                if l < i && i < r {
                    let s = l_i * 100 + k * 10 + r_i;
                    // println!("{}", s);
                    set.insert(s);
                }
            }
        }
    }
    println!("{}", set.len());
    //  dbg!(set);
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
