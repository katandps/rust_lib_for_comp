#[allow(dead_code)]
fn main() {
    let (n, m) = (i::u(), i::u());
    let mut map = i::cmap(n)
        .iter()
        .map(|row| {
            row.iter()
                .map(|ch| ch.to_string().parse::<usize>().ok().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut ans = vec![vec![0; m]; n];
    let mut up = 0;
    while map.iter().fold(0usize, |count, row| {
        count + row.iter().fold(0usize, |count, c| count + *c)
    }) > 0
    {
        // 上端
        for i in 0..m {
            if map[up][i] > 0 {
                ans[up + 1][i] = map[up][i];
                map[up][i] = 0;
                map[up + 1][i - 1] -= ans[up + 1][i];
                map[up + 1][i + 1] -= ans[up + 1][i];
                map[up + 2][i] -= ans[up + 1][i];
            }
        }
        up += 1;
    }
    for i in 0..n {
        for j in 0..m {
            print!("{}", ans[i][j]);
        }
        println!("");
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
