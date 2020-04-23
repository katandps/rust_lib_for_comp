#[allow(dead_code)]
fn main() {
    let n = i::u();
    let s = i::cmap(n);
    let mut ans = Vec::new();
    for w in s {
        let a: Vec<char> = w
            .iter()
            .flat_map(|c| match c {
                &'b' => Some('1'),
                &'B' => Some('1'),
                &'c' => Some('1'),
                &'C' => Some('1'),
                &'d' => Some('2'),
                &'D' => Some('2'),
                &'w' => Some('2'),
                &'W' => Some('2'),
                &'t' => Some('3'),
                &'T' => Some('3'),
                &'j' => Some('3'),
                &'J' => Some('3'),
                &'f' => Some('4'),
                &'F' => Some('4'),
                &'q' => Some('4'),
                &'Q' => Some('4'),
                &'l' => Some('5'),
                &'L' => Some('5'),
                &'v' => Some('5'),
                &'V' => Some('5'),
                &'s' => Some('6'),
                &'S' => Some('6'),
                &'x' => Some('6'),
                &'X' => Some('6'),
                &'p' => Some('7'),
                &'P' => Some('7'),
                &'m' => Some('7'),
                &'M' => Some('7'),
                &'h' => Some('8'),
                &'H' => Some('8'),
                &'k' => Some('8'),
                &'K' => Some('8'),
                &'n' => Some('9'),
                &'N' => Some('9'),
                &'g' => Some('9'),
                &'G' => Some('9'),
                &'z' => Some('0'),
                &'Z' => Some('0'),
                &'r' => Some('0'),
                &'R' => Some('0'),
                _ => None,
            })
            .map(|c| c)
            .collect();
        if a.len() > 0 {
            if ans.len() > 0 {
                ans.push(' ');
            }
            for c in a {
                ans.push(c);
            }
        }
    }
    for c in ans {
        print!("{}", c);
    }
    println!("");
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
