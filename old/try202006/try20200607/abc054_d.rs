#[allow(dead_code)]
fn main() {
    let (n, ma, mb) = i::u3();
    let abc = i::uv3(n);

    let size = 400;
    let size2 = size * size;

    let score_c: Vec<_> = abc
        .iter()
        .map(|abc| (abc.0 * size + abc.1, abc.2))
        .collect();
    let mut dp = vec![std::usize::MAX; size2 + 1];
    dp[0] = 0;
    for &(s, c) in &score_c {
        for i in 1..size2 + 1 {
            let i = size2 - i;
            if i + s <= size2 && dp[i] != std::usize::MAX {
                dp[i + s] = min(dp[i + s].clone(), dp[i].clone() + c);
            }
        }
    }
    let m = ma * size + mb;
    let mut ans = std::usize::MAX;
    let mut target = m;
    while target <= size2 {
        //println!("{} {}", dp[target], target);
        ans = min(ans, dp[target]);
        target += m;
    }
    if ans == std::usize::MAX {
        println!("{}", -1);
        return;
    }
    println!("{}", ans);
    // dbg!(dp);
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

    pub fn u4() -> (usize, usize, usize, usize) {
        (read(), read(), read(), read())
    }

    pub fn u5() -> (usize, usize, usize, usize, usize) {
        (read(), read(), read(), read(), read())
    }

    pub fn u6() -> (usize, usize, usize, usize, usize, usize) {
        (read(), read(), read(), read(), read(), read())
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

    pub fn iv3(n: usize) -> Vec<(i64, i64, i64)> {
        (0..n).map(|_| iv(3)).map(|a| (a[0], a[1], a[2])).collect()
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

    pub fn uv4(n: usize) -> Vec<(usize, usize, usize, usize)> {
        (0..n)
            .map(|_| uv(4))
            .map(|a| (a[0], a[1], a[2], a[3]))
            .collect()
    }

    pub fn fv(n: usize) -> Vec<f64> {
        (0..n).map(|_| f()).collect()
    }

    pub fn cmap(h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| s()).collect()
    }
}
