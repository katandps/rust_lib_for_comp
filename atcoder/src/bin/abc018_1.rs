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
    let a = int();
    let b = int();
    let c = int();

    if a > b && b > c {
        println!("1\n2\n3")
    } else if a > c && c > b {
        println!("1\n3\n2")
    } else if b > a && a > c {
        println!("2\n1\n3")
    } else if b > c && c > a {
        println!("3\n1\n2")
    } else if c > a && a > b {
        println!("2\n3\n1")
    } else if c > b && b > a {
        println!("3\n2\n1")
    }
}
