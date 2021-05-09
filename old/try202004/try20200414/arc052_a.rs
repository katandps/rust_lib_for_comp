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
    let s: String = read();
    let v: Vec<char> = s.chars().collect();
    let ans = v.iter().fold(0, |x, c| {
        if c.is_numeric() {
            x * 10 + *c as i32 - '0' as i32
        } else {
            x
        }
    });
    println!("{}", ans)
}
