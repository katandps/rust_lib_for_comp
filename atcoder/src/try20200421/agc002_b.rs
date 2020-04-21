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
    let (n, m) = (u(), u());
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..m {
        let temp = uv(2);
        x.push(temp[0]);
        y.push(temp[1]);
    }
    let mut r = vec![false; n + 1];
    r[1] = true;
    let mut c = vec![1; n + 1];
    for i in 0..m {
        if r[x[i]] {
            if c[x[i]] == 1 {
                r[x[i]] = false;
            }
            r[y[i]] = true;
        }
        c[x[i]] -= 1;
        c[y[i]] += 1;
    }

    let mut ans = 0;
    for i in 1..(n + 1) {
        if r[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
