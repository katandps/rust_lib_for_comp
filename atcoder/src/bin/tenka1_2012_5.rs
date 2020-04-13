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
    let a: i32 = read();
    let b: i32 = read();
    let c: i32 = read();

    let mut i = 1;
    while i < 128 {
        if i % 3 == a && i % 5 == b && i % 7 == c {
            println!("{}", i)
        }
        i += 1;
    }
}
