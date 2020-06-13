#[allow(dead_code)]
fn main() {
    let (n, l) = i::u2();
    let x = i::uv(n);
    let x_set: HashSet<usize> = x.iter().cloned().collect();
    let (t1, t2, t3) = i::u3();

    let mut time = vec![std::usize::MAX / 2; l + 5];
    time[0] = 0;

    for i in 0..l {
        time[i + 1] = min(
            time[i + 1],
            time[i] + t1 + if x_set.contains(&i) { t3 } else { 0 },
        );
        time[i + 2] = min(
            time[i + 2],
            time[i] + t1 + t2 + if x_set.contains(&i) { t3 } else { 0 },
        );
        time[i + 4] = min(
            time[i + 4],
            time[i] + t1 + t2 * 3 + if x_set.contains(&i) { t3 } else { 0 },
        );
    }

    let mut ans = vec![
        time[l],
        time[l - 1] + t1 / 2 + t2 / 2 + if x_set.contains(&(l - 1)) { t3 } else { 0 },
        time[l - 2] + t1 / 2 + t2 / 2 + t2 + if x_set.contains(&(l - 2)) { t3 } else { 0 },
    ];
    if l > 2 {
        ans.push(
            time[l - 3] + t1 / 2 + t2 / 2 + t2 * 2 + if x_set.contains(&(l - 3)) { t3 } else { 0 },
        );
    }
    println!("{}", ans.iter().min().unwrap());
    // dbg!(time, ans);
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
