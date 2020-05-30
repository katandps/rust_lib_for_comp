use input::*;
use std::cmp::*;
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
    let (n, m, v, p) = (u(), u(), u(), u());
    let a = uv(n);

    let mut b = a.clone();
    b.sort();
    b.reverse();
    let border = b[p - 1];
    let mut sum = 0;
    let mut count = 0;

    let mut ans = 0;
    for i in 0..n {
        let k = b[i];
        if k > border {
            ans += 1;
            continue;
        }
        if k + m >= border {
            if p >= v && k >= border {
                ans += 1;
                continue;
            }
            let rest = n - i - 1;
            let t = m * v;
            let cap = p * m + rest * m + count * (k + m) - sum;
            //dbg!(&t, &cap, &count, &(p * v), &(k + m), &sum);
            // println!("");
            if t <= cap {
                ans += 1;
            }

            if border <= k + m {
                sum += k;
                count += 1;
            }
        } else {
            break;
        }
    }
    println!("{}", ans);
    //dbg!(border, b, ans);
}
