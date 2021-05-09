use input::*;
use std::cmp::*;
use std::collections::HashMap;
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
    let (S, T) = (str(), str());
    let mut A = u();
    let mut B = u();
    let U = str();
    let mut map = HashMap::new();
    if S == U {
        A -= 1
    } else {
        B -= 1;
    };
    map.insert(S.clone(), A);
    map.insert(T.clone(), B);
    println!("{} {}", map[&S], map[&T]);
}
