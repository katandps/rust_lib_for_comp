#[allow(dead_code)]
fn main() {
    let n = i::u();
    let cmap = i::cmap(n);
    let mut b = vec![false; 366];
    for i in 0..367 {
        if i % 7 == 0 || i % 7 == 6 {
            b[i] = true;
        }
    }
    let day_index = vec![0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
    let mut set = HashSet::new();
    for day in cmap {
        if day[1] == '/' {
            let m = day[0].to_string().parse::<usize>().ok().unwrap();
            let d = if day.len() == 3 {
                day[2].to_string().parse::<usize>().ok().unwrap()
            } else {
                ("".to_string() + day[2].to_string().as_str() + day[3].to_string().as_str())
                    .parse::<usize>()
                    .ok()
                    .unwrap()
            };
            set.insert(day_index[m - 1] + d - 1);
        } else {
            let m = 10 + day[1].to_string().parse::<usize>().ok().unwrap();
            let d = if day.len() == 4 {
                day[3].to_string().parse::<usize>().ok().unwrap()
            } else {
                ("".to_string() + day[3].to_string().as_str() + day[4].to_string().as_str())
                    .parse::<usize>()
                    .ok()
                    .unwrap()
            };
            set.insert(day_index[m - 1] + d - 1);
        }
    }
    let mut holiday_count = 0;
    let mut count = 0;
    let mut ans = 0;
    for i in 0..366usize {
        if set.contains(&i) {
            holiday_count += 1;
        }
        if !b[i] && holiday_count > 0 {
            holiday_count -= 1;
            b[i] = true;
        }
        if b[i] {
            count += 1;
            ans = max(ans, count);
        } else {
            count = 0;
        }
    }
    println!("{}", ans);
    // dbg!(b, set);
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
