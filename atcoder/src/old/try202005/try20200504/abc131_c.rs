#[allow(dead_code)]
fn main() {
    let (a, b, c, d) = (i::u(), i::u(), i::u(), i::u());
    let total = b - a + 1;
    let cd = c * d / gcd(c, d);
    let cnum = if b / c == 0 {
        0
    } else {
        b / c - (a / c + if a % c > 0 { 1 } else { 0 }) + 1
    };
    let dnum = if b / d == 0 {
        0
    } else {
        b / d - (a / d + if a % d > 0 { 1 } else { 0 }) + 1
    };
    let cdnum = if b / cd == 0 {
        0
    } else {
        b / cd - (a / cd + if a % cd > 0 { 1 } else { 0 }) + 1
    };
    println!("{}", total - cnum - dnum + cdnum);
    //  dbg!(total, cnum, dnum, cdnum);
}

#[allow(unused_imports)]
use greatest_common_divisor::*;

#[allow(dead_code)]
mod greatest_common_divisor {
    use std::mem::swap;

    pub fn gcd(mut a: usize, mut b: usize) -> usize {
        if a < b {
            swap(&mut b, &mut a);
        }
        let mut c = 0;
        while b != 0 {
            a = a % b;
            swap(&mut a, &mut b);
            c += 1;
            //    println!("{}", c);
        }
        a
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
