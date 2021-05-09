use input::*;
use std::cmp::*;
use std::collections::{HashMap, HashSet};
use std::io::*;
use std::num::*;
use std::str::*;

mod input {
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
    let (h, w, k) = (u(), u(), u());
    let a = cmap(h);

    let mut sum = vec![vec![0; w]; h];

    let mut before = 1;
    for i in 0..h {
        if !a[i].contains(&'#') {
            continue;
        }

        let mut c = 0;
        for j in 0..w {
            if a[i][j] == '#' {
                c += 1;
                if c > 1 {
                    before += 1;
                }
            }
            sum[i][j] = before;
        }
        before += 1;
    }

    let mut stack = 0;
    let mut last = vec![0; w];
    for i in 0..h {
        if sum[i][0] == 0 {
            if last[0] == 0 {
                stack += 1;
            } else {
                for j in 0..w {
                    print!("{} ", &last[j]);
                }
                println!("");
            }
        } else {
            for j in 0..w {
                print!("{} ", &sum[i][j]);
            }
            println!("");
            last = (*sum[i]).to_owned();
            while stack > 0 {
                for j in 0..w {
                    print!("{} ", &last[j]);
                }
                println!("");
                stack -= 1;
            }
        }
    }
}
