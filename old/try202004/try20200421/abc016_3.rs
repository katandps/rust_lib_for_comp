use std::cmp::*;
use std::collections::HashSet;
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

fn main() {
    let (n, m) = (i::u(), i::u());
    let ab = i::uv2(m);
    let mut map = vec![HashSet::new(); n + 1];
    for i in 0..m {
        map[ab[i].0].insert(ab[i].1);
        map[ab[i].1].insert(ab[i].0);
    }

    // 自分のID
    for i in 1..(n + 1) {
        let mut ans = HashSet::new();
        // 友達
        for j in &map[i] {
            // 友達の友達
            for k in &map[*j] {
                if *k == i || map[i].contains(k) {
                    continue;
                } else {
                    ans.insert(*k);
                }
            }
        }
        println!("{}", ans.len());
    }
}
