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
    let S: String = read();
    let s: String = read();
    let C: char = S.chars().collect::<Vec<char>>()[0];
    let c: char = s.chars().collect::<Vec<char>>()[0];
    println!(
        "{}",
        if (C as i32 - 'A' as i32) == (c as i32 - 'a' as i32) {
            "Yes"
        } else {
            "No"
        }
    )
}
