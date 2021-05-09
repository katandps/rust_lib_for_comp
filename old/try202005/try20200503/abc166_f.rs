#[allow(dead_code)]
fn main() {
    let (n, a, b, c) = i::u4();
    let s = i::cmap(n);

    let mut ans = Vec::new();
    let mut a = a;
    let mut b = b;
    let mut c = c;

    let mut stack = "";
    for i in 0..n {
        let cur: String = s[i].iter().collect();
        let cur = cur.as_str();
        if stack == cur {
            if cur == "AB" {
                if a == 0 {
                    ans.push("A");
                    ans.push("B");
                } else {
                    ans.push("B");
                    ans.push("A");
                }
            }
            if cur == "AC" {
                if a == 0 {
                    ans.push("A");
                    ans.push("C");
                } else {
                    ans.push("C");
                    ans.push("A");
                }
            }
            if cur == "BC" {
                if b == 0 {
                    ans.push("B");
                    ans.push("C");
                } else {
                    ans.push("C");
                    ans.push("B");
                }
            }
            stack = "";
            continue;
        }

        if stack == "AB" {
            if cur == "BC" {
                ans.push("B");
                b += 1;
                a -= 1;
            } else {
                //AC
                ans.push("A");
                a += 1;
                b -= 1;
            }
        }
        if stack == "BC" {
            if cur == "AB" {
                ans.push("B");
                b += 1;
                c -= 1;
            } else {
                //AC
                ans.push("C");
                c += 1;
                b -= 1;
            }
        }
        if stack == "AC" {
            if cur == "BC" {
                ans.push("C");
                c += 1;
                a -= 1;
            } else {
                //AB
                ans.push("A");
                a += 1;
                c -= 1;
            }
        }
        stack = "";

        if cur == "AB" {
            if a == 0 && b == 0 {
                println!("{}", "No");
                return;
            }
            if a == 0 {
                ans.push("A");
                a += 1;
                b -= 1;
            } else if b == 0 {
                ans.push("B");
                a -= 1;
                b += 1;
            } else if c == 0 {
                stack = "AB";
            } else {
                ans.push("A");
                a += 1;
                b -= 1;
            }
        } else if cur == "BC" {
            if b == 0 && c == 0 {
                println!("{}", "No");
                return;
            }
            if b == 0 {
                ans.push("B");
                b += 1;
                c -= 1;
            } else if c == 0 {
                ans.push("C");
                b -= 1;
                c += 1;
            } else if a == 0 {
                stack = "BC";
            } else {
                ans.push("B");
                b += 1;
                c -= 1;
            }
        } else if cur == "AC" {
            if a == 0 && c == 0 {
                println!("{}", "No");
                return;
            }
            if a == 0 {
                ans.push("A");
                a += 1;
                c -= 1;
            } else if c == 0 {
                ans.push("C");
                a -= 1;
                c += 1;
            } else if b == 0 {
                stack = "AC";
            } else {
                ans.push("A");
                a += 1;
                c -= 1;
            }
        }
    }
    if stack == "BC" {
        ans.push("B");
    } else if stack == "AB" {
        ans.push("A");
    } else if stack == "AC" {
        ans.push("C");
    }
    println!("{}", "Yes");
    for k in ans {
        println!("{}", k);
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
