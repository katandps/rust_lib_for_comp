#[allow(dead_code)]
fn main() {
    let n = i::u();
    let mut map = HashMap::new();
    for k in 1..(n + 1) {
        let mut l = 2;
        let mut k = k;
        while l * l <= k {
            if k % l == 0 {
                if !map.contains_key(&l) {
                    map.insert(l, 0);
                }
                *map.get_mut(&l).unwrap() += 1;
                k /= l;

                if l == 1 {
                    l += 1;
                }
                continue;
            }
            l += 1;
        }
        if !map.contains_key(&k) {
            map.insert(k, 0);
        }
        *map.get_mut(&k).unwrap() += 1;
        //dbg!(&map);
    }

    let MOD = 1_000_000_007usize;
    let mut ans = 1;
    for k in &map {
        if *k.0 == 1 {
            continue;
        }
        ans = ans * (*k.1 + 1) % MOD;
    }
    println!("{}", ans);
    //dbg!(ans, map);
}

#[allow(unused_imports)]
use std::cmp::*;
use std::collections::HashMap;
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
