use input::*;
use std::cmp::*;
use std::collections::HashSet;
use std::io::*;
use std::num::*;
use std::str::*;

//#[rustfmt::skip]
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

    pub fn string() -> String {
        read()
    }

    pub fn int() -> i64 {
        read()
    }

    pub fn char() -> char {
        read::<String>().pop().unwrap()
    }

    pub fn vecchar() -> Vec<char> {
        string().chars().collect()
    }

    pub fn vecint(n: i64) -> Vec<i64> {
        let mut vec = Vec::new();
        for i in 0..n {
            vec.push(int())
        }
        vec
    }
}

fn main() {
    let (a, b) = (int(), int());
    let (adiv, bdiv) = (divisor(a), divisor(b));
    let mut ans = 0;
    for ai in &adiv {
        for bi in &bdiv {
            if ai == bi {
                ans += 1
            }
        }
    }
    //println!("{:?}", adiv);
    //println!("{:?}", bdiv);
    println!("{}", ans)
}

fn divisor(mut n: i64) -> HashSet<i64> {
    let mut r = HashSet::new();
    let mut l = 2;
    while l * l <= n {
        if n % l == 0 {
            n /= l;
            r.insert(l);
            continue;
        }
        l += 1;
    }
    r.insert(1);
    r.insert(n);
    r
}
