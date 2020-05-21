#[allow(dead_code)]
fn main() {
    let (n, u, v) = i::u3();
    let ab = i::uv2(n - 1);

    let mut map = vec![HashSet::new(); n + 1];
    for (a, b) in ab {
        map[a].insert(b);
        map[b].insert(a);
    }

    let mut ud = vec![0; n + 1];
    let mut vd = vec![0; n + 1];

    let mut memo = HashSet::new();
    let mut q = VecDeque::new();
    memo.insert(u);
    q.push_front(u);
    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        for &c in &map[cur] {
            if memo.contains(&c) {
                continue;
            }
            memo.insert(c);
            q.push_back(c);
            ud[c] = ud[cur] + 1;
        }
    }
    let mut memo = HashSet::new();
    let mut q = VecDeque::new();
    memo.insert(v);
    q.push_front(v);
    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        for &c in &map[cur] {
            if memo.contains(&c) {
                continue;
            }
            memo.insert(c);
            q.push_back(c);
            vd[c] = vd[cur] + 1;
        }
    }

    let mut ans = 0;
    for i in 1..(n + 1) {
        if ud[i] >= vd[i] {
            continue;
        }
        ans = max(ans, vd[i] - 1);
    }
    println!("{}", ans);
    //  dbg!(ud, vd);
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
