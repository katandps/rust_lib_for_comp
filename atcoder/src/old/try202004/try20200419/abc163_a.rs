use input::*;
use std::cmp::*;
use std::f64::consts::PI;
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
    let r = int() as f64;
    println!("{}", r * 2.0 * PI)
}
