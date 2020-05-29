#[allow(dead_code)]
fn main() {
    let (n, m) = i::u2();
    let uv = if m != 0 { i::uv2(m) } else { Vec::new() };
    let (s, t) = i::u2();

    let mut graph = vec![Vec::new(); n + 1];
    for (u, v) in uv {
        graph[u].push(v);
    }

    let mut dis = vec![vec![std::usize::MAX; 3]; n + 1];
    let mut q = VecDeque::new();
    q.push_front((s, 0));
    dis[s][0] = 0;
    while q.len() > 0 {
        let (from, t) = q.pop_front().unwrap();
        for &to in &graph[from] {
            if dis[to][(t + 1) % 3] == std::usize::MAX {
                dis[to][(t + 1) % 3] = dis[from][t] + 1;
                q.push_back((to, (t + 1) % 3));
            }
        }
    }
    if dis[t][0] == std::usize::MAX {
        println!("{}", -1);
        return;
    }
    println!("{}", dis[t][0] / 3);
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
