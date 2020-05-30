#[allow(dead_code)]
fn main() {
    let n = i::u();
    let mut f = Vec::new();
    for _ in 0..n {
        f.push(i::uv(10));
    }
    let mut p = Vec::new();
    for _ in 0..n {
        p.push(i::iv(11));
    }

    let mut ans = std::i32::MIN as i64;

    let case = (0..10).fold(1usize, |x, _| x * 2);
    //0は店を営業していないので除外
    for i in 1..case {
        let mut l = i;
        // joisinoの店が営業している時間帯
        let open: Vec<bool> = (0..10)
            .map(|_| {
                let r = l % 2 == 1;
                l >>= 1;
                r
            })
            .collect();
        // 各ショップと営業時間帯が一致する数
        let match_open: Vec<usize> = f
            .iter()
            .map(|v| {
                v.iter()
                    .zip(open.clone())
                    .filter(|fo| *fo.0 == 1 && fo.1)
                    .count()
            })
            .collect();
        let mut a = 0;
        for j in 0..n {
            a += p[j][match_open[j]];
        }
        ans = max(ans, a);
    }
    println!("{}", ans);
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
