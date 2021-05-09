use input::*;
use std::cmp::*;
use std::io::*;
use std::num::*;
use std::str::*;

mod input {
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

    pub fn uv(n: usize) -> Vec<usize> {
        (0..n).map(|_| u()).collect()
    }

    pub fn fv(n: usize) -> Vec<f64> {
        (0..n).map(|_| f()).collect()
    }

    pub fn cmap(h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| s()).collect()
    }
}

fn main() {
    let n = u();
    let h = uv(n);

    let mut left = vec![0; n];
    let mut right = vec![0; n];

    let mut cur = 0;
    for i in 1..n {
        if h[i] > h[i - 1] {
            cur += 1;
            left[i] = cur;
        } else {
            cur = 0;
        }
    }
    for i in 1..n {
        let i = n - i - 1;
        if h[i] > h[i + 1] {
            cur += 1;
            right[i] = cur;
        } else {
            cur = 0;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, left[i] + right[i] + 1);
    }
    println!("{}", ans);
    // dbg!(&left, &right);
}
