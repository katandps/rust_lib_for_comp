#[allow(dead_code)]
fn main() {
    let n = i::u();
    let mut a = i::uv(n);
    a.sort();
    let m = 1_000_000_007usize;
    if n % 2 == 0 {
        for i in 0..n / 2 {
            if a[i * 2] != i * 2 + 1 || a[i * 2 + 1] != i * 2 + 1 {
                println!("{}", 0);
                return;
            }
        }
    } else {
        if a[0] != 0 {
            println!("{}", 0);
            return;
        }
        for i in 1..n / 2 {
            if a[i * 2 - 1] != i * 2 || a[i * 2] != i * 2 {
                println!("{}", 0);
                return;
            }
        }
    }
    println!("{}", (0..n / 2).fold(1, |x, _| x * 2 % m));
}

#[allow(unused_imports)]
use std::cmp::*;
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
