use std::cmp::*;
use std::io::*;
use std::num::*;
use std::str::*;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok().unwrap()
}

fn string() -> String {
    read()
}

fn int() -> i64 {
    read()
}

fn char() -> char {
    read::<String>().pop().unwrap()
}

fn vecchar() -> Vec<char> {
    string().chars().collect()
}

fn vecint(n: i64) -> Vec<i64> {
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(int())
    }
    vec
}

fn main() {
    let (r, g, b) = (string(), string(), string());
    let s = r + &g + &b;
    println!(
        "{}",
        if s.parse::<i32>().unwrap() % 4 == 0 {
            "YES"
        } else {
            "NO"
        }
    );
}
