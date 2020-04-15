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
    let r: i32 = read();
    let d: i32 = read();

    let pi = std::f64::consts::PI;
    println!("{}", pi * r as f64 * r as f64 * pi * 2f64 * d as f64)
}
