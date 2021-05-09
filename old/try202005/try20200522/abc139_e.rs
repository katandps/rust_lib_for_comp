#[allow(dead_code)]
fn main() {
    let n = i::u();
    let mut a = Vec::new();
    for _ in 0..n {
        a.push(i::uv(n - 1).iter().map(|a| a - 1).collect::<Vec<usize>>());
    }

    let mut i = vec![0; n];

    let mut ans = 0;
    let mut rest = n * (n - 1) / 2;
    let mut ready = HashSet::new();
    let mut pool = HashSet::new();
    for i in 0..n {
        if pool.contains(&(a[i][0], i)) {
            ready.insert((i, a[i][0]));
        } else {
            pool.insert((i, a[i][0]));
        }
    }
    while rest > 0 {
        let mut c = HashSet::new();
        let mut remove = HashSet::new();
        for r in &ready {
            let (t, u) = (&r.0, &r.1);
            if c.contains(u) || c.contains(t) {
                continue;
            }
            c.insert(*t);
            c.insert(*u);
            remove.insert((*u, *t));
            remove.insert((*t, *u));
        }
        for r in remove {
            ready.remove(&r);
        }
        if c.len() == 0 {
            // dbg!(pool, ready);
            println!("{}", -1);
            return;
        }
        rest -= c.len() / 2;
        ans += 1;
        for j in c {
            i[j] += 1;
            if i[j] < n - 1 {
                if pool.contains(&(a[j][i[j]], j)) {
                    ready.insert((j, a[j][i[j]]));
                } else {
                    pool.insert((j, a[j][i[j]]));
                }
            }
        }
    }
    println!("{}", ans);
    //dbg!(a, i);
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
