use std::cmp::*;
use std::io::*;
use std::num::*;
use std::str::*;

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
    let (n, p) = (i::u(), i::u());
    let a = i::uv(n);
    let odd = a.iter().filter(|k| *k % 2 == 1).count();
    let even = a.iter().filter(|k| *k % 2 == 0).count();

    let mut s = 1usize;
    for _ in 0..even {
        s *= 2;
    }
    let mut ans = 0;
    //oddをi個選ぶ
    for i in 0..(odd + 1) {
        if i % 2 != p {
            continue;
        }
        let mut combination = 1;
        for j in 0..i {
            combination *= odd - j;
            combination /= j + 1
        }
        ans += combination * s;
        //dbg!(ans, i, p);
    }
    println!("{}", ans);
}
