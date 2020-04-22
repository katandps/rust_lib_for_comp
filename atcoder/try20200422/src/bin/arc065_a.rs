#[allow(dead_code)]
fn main() {
    let mut s = i::s();
    s.reverse();
    let mut i = 0;
    'all: loop {
        if s.len() >= 7 + i {
            let mut a = true;
            for j in 0..7 {
                if s[i + j] != "remaerd".chars().collect::<Vec<char>>()[j] {
                    a = false;
                }
            }
            if a {
                i += 7;
                continue 'all;
            }
        }

        if s.len() >= 6 + i {
            let mut a = true;
            for j in 0..6 {
                if s[i + j] != "resare".chars().collect::<Vec<char>>()[j] {
                    a = false;
                }
            }
            if a {
                i += 6;
                continue 'all;
            }
        }

        if s.len() >= 5 + i {
            let mut a = true;
            for j in 0..5 {
                if s[i + j] != "maerd".chars().collect::<Vec<char>>()[j] {
                    a = false;
                }
            }
            if a {
                i += 5;
                continue 'all;
            }
            let mut a = true;
            for j in 0..5 {
                if s[i + j] != "esare".chars().collect::<Vec<char>>()[j] {
                    a = false;
                }
            }
            if a {
                i += 5;
                continue 'all;
            }
        }
        if s.len() == i {
            break;
        }

        //    dbg!(i);
        println!("{}", "NO");
        return;
    }
    println!("{}", "YES");
}

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
