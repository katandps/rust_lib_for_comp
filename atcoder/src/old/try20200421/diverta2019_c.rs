#[allow(dead_code)]
fn main() {
    let n = i::u();
    let s = i::cmap(n);

    let native: usize = s
        .iter()
        .map(|s| {
            let mut a = false;
            let mut count: usize = 0;
            for c in s {
                if a && *c == 'B' {
                    count += 1;
                }
                a = *c == 'A'
            }
            count
        })
        .sum();
    let xa = s
        .iter()
        .filter(|s| s[0] != 'B' && s[s.len() - 1] == 'A')
        .count();
    let bx = s
        .iter()
        .filter(|s| s[0] == 'B' && s[s.len() - 1] != 'A')
        .count();
    let ba = s
        .iter()
        .filter(|s| s[0] == 'B' && s[s.len() - 1] == 'A')
        .count();
    println!(
        "{}",
        native
            + match (xa, bx) {
                (0, 0) =>
                    if ba > 0 {
                        ba - 1
                    } else {
                        0
                    },
                (a, b) => min(a + ba, b + ba),
            }
    );
    //dbg!(native, xa, bx, ba);
}

use i::cmap;
#[allow(unused_imports)]
use std::cmp::*;
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
