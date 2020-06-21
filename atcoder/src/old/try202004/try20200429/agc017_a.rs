#[allow(dead_code)]
fn main() {
    let (n, p) = i::u2();
    let a = i::uv(n);
    let odd = a.iter().filter(|&i| i % 2 == 1).count();
    let even = a.iter().filter(|&i| i % 2 == 0).count();

    let mut odd_comb = Vec::new();
    odd_comb.push(1);
    for i in 1..odd + 1 {
        let k = odd_comb[i - 1] * (odd - i + 1) / i;
        odd_comb.push(k);
    }
    //  dbg!(&odd_comb, &odd);

    if p == 1 {
        if odd < 1 {
            println!("{}", 0);
            return;
        }
        let mut odd_p = 0;
        let mut i = 1;
        while i <= odd {
            odd_p += odd_comb[i];
            i += 2;
        }
        let mut even_p = (0..even).fold(1usize, |u, _| u * 2);
        println!("{}", odd_p * even_p);
        //  dbg!(odd_p, even_p);
        return;
    }

    let mut odd_p = 0;
    let mut i = 0;
    while i <= odd {
        odd_p += odd_comb[i];
        i += 2;
    }
    let mut even_p = (0..even).fold(1usize, |u, _| u * 2);

    println!("{}", odd_p * even_p);
    // dbg!(odd_p, even_p);
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
