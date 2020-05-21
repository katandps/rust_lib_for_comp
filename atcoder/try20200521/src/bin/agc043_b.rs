#[allow(dead_code)]
fn main() {
    let n = i::u();
    let a: Vec<i32> = i::s()
        .iter()
        .map(|&c| (c as i32) - ('0' as i32) - 1)
        .collect();

    let mut b = false;
    for i in 0..n {
        let a_i = a[i] % 2;
        if a_i == 0 {
            continue;
        }
        let k = lucas_theorem(n - 1, i, 2);
        b = b ^ (k == 1);
    }

    if b {
        println!("{}", 1);
        return;
    }

    for i in 0..n {
        if a[i] == 1 {
            println!("{}", 0);
            return;
        }
    }

    let a: Vec<i32> = a.iter().map(|k| k / 2).collect();
    let mut b = false;
    for i in 0..n {
        let a_i = a[i] % 2;
        if a_i == 0 {
            continue;
        }
        let k = lucas_theorem(n - 1, i, 2);
        b = b ^ (k == 1);
    }

    println!("{}", if b { 2 } else { 0 });
}

#[allow(unused_imports)]
use lucas_theorem::*;

#[allow(dead_code)]
mod lucas_theorem {
    use super::*;

    /// nCr mod p を得る 計算量: O(log n)
    pub fn lucas_theorem(mut n: usize, mut r: usize, p: usize) -> usize {
        if p < 2 {
            return 0;
        }
        let mut ret = 1;
        while n != 0 || r != 0 {
            let (n_mod, r_mod) = (n % p, r % p);
            if n_mod >= r_mod {
                ret *= combination(n_mod, r_mod);
            } else {
                return 0;
            }
            n /= p;
            r /= p;
        }
        ret % p
    }

    fn combination(n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        if std::cmp::min(k, n - k) == 0 {
            1
        } else {
            combination(n - 1, k - 1) * n / k
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
