#[allow(dead_code)]
fn main() {
    let (x, y, w) = (i::u(), i::u(), i::str());
    let c = i::cmap(9);
    let mut ans = Vec::new();
    let mut x = x - 1;
    let mut y = y - 1;
    let mut w = w.as_str();

    ans.push(c[y][x]);
    while ans.len() < 4 {
        match w {
            "R" => {
                if x >= 8 {
                    w = "L";
                    continue;
                }
                x += 1;
            }
            "L" => {
                if x <= 0 {
                    w = "R";
                    continue;
                }
                x -= 1;
            }
            "U" => {
                if y <= 0 {
                    w = "D";
                    continue;
                }
                y -= 1;
            }
            "D" => {
                if y >= 8 {
                    w = "U";
                    continue;
                }
                y += 1;
            }
            "RU" => {
                if x >= 8 {
                    w = "LU";
                    continue;
                }
                if y <= 0 {
                    w = "RD";
                    continue;
                }
                x += 1;
                y -= 1;
            }
            "RD" => {
                if x >= 8 {
                    w = "LD";
                    continue;
                }
                if y >= 8 {
                    w = "RU";
                    continue;
                }
                x += 1;
                y += 1;
            }
            "LU" => {
                if x <= 0 {
                    w = "RU";
                    continue;
                }
                if y <= 0 {
                    w = "LD";
                    continue;
                }
                x -= 1;
                y -= 1;
            }
            "LD" => {
                if x <= 0 {
                    w = "RD";
                    continue;
                }
                if y >= 8 {
                    w = "LU";
                    continue;
                }
                x -= 1;
                y += 1;
            }
            _ => {}
        }
        ans.push(c[y][x]);
    }
    for a in ans {
        print!("{}", a);
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
