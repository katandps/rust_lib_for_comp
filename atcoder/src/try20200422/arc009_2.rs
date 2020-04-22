#[allow(dead_code)]
fn main() {
    let b: Vec<usize> = (0..10).map(|_| i::read()).collect();
    let n = i::u();
    let a = i::cmap(n);
    let mut b_rev = vec![0; 10];
    for i in 0..10 {
        b_rev[b[i].to_string().parse::<usize>().ok().unwrap()] = i;
    }
    let mut aa: Vec<usize> = a
        .iter()
        .map(|s| {
            s.iter()
                .map(|c| b_rev[*c as usize - '0' as usize])
                .fold("".to_string(), |x, c| x + &c.to_string())
        })
        .map(|s| s.parse::<usize>().ok().unwrap())
        .collect();
    aa.sort();

    for ans in &aa {
        let s = ans.to_string();
        for c in s.chars() {
            print!("{}", b[c as usize - '0' as usize]);
        }
        println!("");
    }
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
