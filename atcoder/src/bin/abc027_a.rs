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
    let l1: i32 = read();
    let l2: i32 = read();
    let l3: i32 = read();
    if l1 == l2 {
        println!("{}", l3);
    } else {
        if l2 == l3 {
            println!("{}", l1)
        } else {
            println!("{}", l2)
        }
    }
}
