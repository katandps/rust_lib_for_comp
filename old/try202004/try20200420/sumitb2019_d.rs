use input::*;
use std::cmp::*;
use std::collections::HashSet;
use std::collections::VecDeque;
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
    let n = int() as usize;
    let s = vecchar();

    let mut indexes = Vec::new();
    for i in 0..10 {
        let mut v = VecDeque::new();
        for j in 0..n {
            if s[j] == i.to_string().chars().collect::<Vec<char>>()[0] {
                v.push_back(j)
            }
        }
        indexes.push(v);
    }

    let mut ans = 0;
    for k in 0..1000 {
        let k = format!("{:>03}", k).chars().collect::<Vec<char>>();
        let first = k[0] as usize - '0' as usize;
        let second = k[1] as usize - '0' as usize;
        let last = k[2] as usize - '0' as usize;
        let f = indexes[first].front();
        let l = indexes[last].back();
        ans += match (f, l) {
            (Some(ff), Some(ll)) => {
                let mut a = 0;
                for i in &indexes[second] {
                    if ff < i && i < ll {
                        a = 1;
                    }
                }
                a
            }
            _ => 0,
        };
    }
    println!("{}", ans)
}
