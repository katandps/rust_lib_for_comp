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
    let (n, m) = (i::u(), i::u());
    let mut k = Vec::new();
    let mut s = Vec::new();
    for _ in 0..m {
        let t = i::u();
        k.push(t);
        s.push(i::uv(t));
    }
    let p = i::uv(m);

    let mut max = 1;
    for _ in 0..n {
        max *= 2;
    }
    let mut ans = 0;
    let mut i = 0;
    'case: while i < max {
        let mut l = i;
        let mut v = vec![false];
        for _ in 0..n {
            v.push(l % 2 == 0);
            l = l >> 1;
        }
        i += 1;

        for j in 0..m {
            let mut count = 0;
            for c in &s[j] {
                if v[*c] {
                    count += 1;
                }
            }
            //dbg!(&v, count);
            if count % 2 != p[j] {
                continue 'case;
            }
        }
        ans += 1;
    }
    println!("{}", ans);
}
