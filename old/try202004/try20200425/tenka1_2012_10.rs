#[allow(dead_code)]
fn main() {
    let s = i::str();
    let royal = vec![
        vec!["S10", "SJ", "SQ", "SK", "SA"],
        vec!["H10", "HJ", "HQ", "HK", "HA"],
        vec!["D10", "DJ", "DQ", "DK", "DA"],
        vec!["C10", "CJ", "CQ", "CK", "CA"],
    ];
    let case: Vec<Vec<Option<usize>>> = royal
        .iter()
        .map(|suit| suit.iter().map(|c| s.find(c)).collect())
        .collect();
    let mut min_index = -1;
    let mut min_num = 200;
    for i in 0..4 {
        let c: Vec<usize> = case[i].iter().flat_map(|a| a).map(|a| a.clone()).collect();
        if c.len() < 5 {
            continue;
        }
        let mi: i32 = *c.iter().max().unwrap() as i32;
        if mi < min_num {
            min_num = mi;
            min_index = i as i32;
        }
    }
    let ss = (0..5).fold(s[0..min_num as usize].to_string(), |ans, i| {
        ans.replace(royal[min_index as usize][i], "")
    });
    if ss.len() > 0 {
        println!("{}", ss);
    } else {
        println!("{}", 0);
    }
    //dbg!(royal, case, min_num, min_index);
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
