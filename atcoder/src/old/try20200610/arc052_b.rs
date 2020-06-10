#[allow(dead_code)]
fn main() {
    let (n, q) = i::u2();
    let xrh = i::uv3(n);
    let ab = i::uv2(q);

    for (a, b) in ab {
        let mut ans = 0.0f64;
        for &(x, r, h) in &xrh {
            if b <= x || x + h <= a {
                continue;
            }
            //dbg!(&x, &h, &a, &b);
            // 内側
            if a < x && x + h < b {
                // println!("full");
                let (r, h) = (r as f64, h as f64);
                let k = r * r * h * std::f64::consts::PI / 3.0;
                ans += k;
                continue;
            }
            // 左側
            if b <= x + h && x >= a {
                // println!("left");

                let (x, r, h) = (x as f64, r as f64, h as f64);
                let k = r * r * h * std::f64::consts::PI / 3.0;
                let len = x + h - b as f64;
                let rr = r * len / h;
                let top = rr * rr * len * std::f64::consts::PI / 3.0;
                ans += k - top;
                continue;
            }

            // 右側
            if x <= a && b >= x + h {
                // println!("right");
                //
                let (x, r, h) = (x as f64, r as f64, h as f64);
                let len = h - a as f64 + x;
                let rr = r * len / h;
                let top = rr * rr * len * std::f64::consts::PI / 3.0;
                ans += top;
                continue;
            }

            // 両側カット
            // println!("center");

            let (x, r, h) = (x as f64, r as f64, h as f64);
            let len = x + h - a as f64;
            let rr = r * len / h;
            let top1 = rr * rr * len * std::f64::consts::PI / 3.0;
            // dbg!(&top1, &rr, &len, r, h);
            let len = x + h - b as f64;
            let rr = r * len / h;
            let top2 = rr * rr * len * std::f64::consts::PI / 3.0;
            // dbg!(&top1, &top2, h, len, r, rr);
            ans += top1 - top2;
            continue;
        }
        println!("{}", ans);
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

    pub fn u4() -> (usize, usize, usize, usize) {
        (read(), read(), read(), read())
    }

    pub fn u5() -> (usize, usize, usize, usize, usize) {
        (read(), read(), read(), read(), read())
    }

    pub fn u6() -> (usize, usize, usize, usize, usize, usize) {
        (read(), read(), read(), read(), read(), read())
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

    pub fn iv3(n: usize) -> Vec<(i64, i64, i64)> {
        (0..n).map(|_| iv(3)).map(|a| (a[0], a[1], a[2])).collect()
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

    pub fn uv4(n: usize) -> Vec<(usize, usize, usize, usize)> {
        (0..n)
            .map(|_| uv(4))
            .map(|a| (a[0], a[1], a[2], a[3]))
            .collect()
    }

    pub fn fv(n: usize) -> Vec<f64> {
        (0..n).map(|_| f()).collect()
    }

    pub fn cmap(h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| s()).collect()
    }
}
