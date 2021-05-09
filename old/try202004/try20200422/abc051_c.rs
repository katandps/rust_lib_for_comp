#[allow(dead_code)]
fn main() {
    let (sx, sy, tx, ty) = (i::i(), i::i(), i::i(), i::i());
    let (x, y) = (tx - sx, ty - sy);
    //行き
    for _ in 0..x {
        print!("R")
    }
    for _ in 0..y {
        print!("U")
    }
    //帰り
    for _ in 0..x {
        print!("L")
    }
    for _ in 0..y {
        print!("D")
    }
    //行き
    print!("D");
    for _ in 0..(x + 1) {
        print!("R")
    }
    for _ in 0..(y + 1) {
        print!("U")
    }
    print!("L");
    //帰り
    print!("U");
    for _ in 0..(x + 1) {
        print!("L")
    }
    for _ in 0..(y + 1) {
        print!("D")
    }
    println!("R")
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
