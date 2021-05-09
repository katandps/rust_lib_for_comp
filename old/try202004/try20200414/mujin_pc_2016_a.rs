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
    let mut c: String = read();
    let c = c.pop().unwrap();
    println!(
        "{}",
        if c == 'O' || c == 'P' || c == 'K' || c == 'L' {
            "Right"
        } else {
            "Left"
        }
    )
}
