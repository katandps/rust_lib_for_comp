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
    let x1: i32 = read();
    let y1: i32 = read();
    let x2: i32 = read();
    let y2: i32 = read();

    println!("{}", (x2 - x1).abs() + (y2 - y1).abs() + 1)
}
