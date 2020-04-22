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
    let mut i: i32 = read();
    let h = i / 3600;
    i = i % 3600;
    let m = i / 60;
    i = i % 60;
    println!("{:>02}:{:>02}:{:>02}", h, m, i)
}
