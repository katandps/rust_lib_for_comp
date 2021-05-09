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
    let (h, w) = (int() as usize, int() as usize);
    let mut a = Vec::new();
    for _ in 0..h {
        a.push(vecchar())
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                count += 1
            }
        }
    }
    println!(
        "{}",
        if count == h + w - 1 {
            "Possible"
        } else {
            "Impossible"
        }
    )
}
