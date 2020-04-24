#[allow(dead_code)]
fn main() {
    let (h, w) = (i::u(), i::u());
    let s = i::cmap(h);
    let mut ans = vec![vec!['.'; w]; h];
    for y in 0..h {
        for x in 0..w {
            let mut can = s[y][x] == '#';
            if x > 0 {
                can &= s[y][x - 1] == '#';
                if y > 0 {
                    can &= s[y - 1][x] == '#';
                    can &= s[y - 1][x - 1] == '#';
                }
                if y < h - 1 {
                    can &= s[y + 1][x] == '#';
                    can &= s[y + 1][x - 1] == '#';
                }
            }
            if x < w - 1 {
                can &= s[y][x + 1] == '#';
                if y > 0 {
                    can &= s[y - 1][x] == '#';
                    can &= s[y - 1][x + 1] == '#';
                }
                if y < h - 1 {
                    can &= s[y + 1][x] == '#';
                    can &= s[y + 1][x + 1] == '#';
                }
            }
            if can {
                ans[y][x] = '#';
            }
        }
    }
    let mut check = vec![vec!['.'; w]; h];
    for y in 0..h {
        for x in 0..w {
            if ans[y][x] == '.' {
                continue;
            }
            check[y][x] = '#';
            if x > 0 {
                check[y][x - 1] = '#';
                if y > 0 {
                    check[y - 1][x] = '#';
                    check[y - 1][x - 1] = '#';
                }
                if y < h - 1 {
                    check[y + 1][x] = '#';
                    check[y + 1][x - 1] = '#';
                }
            }
            if x < w - 1 {
                check[y][x + 1] = '#';
                if y > 0 {
                    check[y - 1][x] = '#';
                    check[y - 1][x + 1] = '#';
                }
                if y < h - 1 {
                    check[y + 1][x] = '#';
                    check[y + 1][x + 1] = '#';
                }
            }
        }
    }
    if s != check {
        println!("{}", "impossible");
        return;
    }
    println!("{}", "possible");
    for y in 0..h {
        for x in 0..w {
            print!("{}", ans[y][x]);
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
