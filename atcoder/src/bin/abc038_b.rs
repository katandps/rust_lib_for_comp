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

fn main() {
    let h1: i32 = read();
    let w1: i32 = read();
    let h2: i32 = read();
    let w2: i32 = read();

    println!(
        "{}",
        if h1 == h2 || h1 == w2 || w1 == h2 || w1 == w2 {
            "YES"
        } else {
            "NO"
        }
    )
}
