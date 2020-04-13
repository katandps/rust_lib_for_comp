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
    let n: i32 = read();
    let p = n * (n + 1) / 2;
    if p == 1 {
        println!("BOWWOW");
        return;
    }
    let mut i = 2;
    while i * i <= p {
        if p % i == 0 {
            println!("BOWWOW");
            return;
        }
        i += 1;
    }
    println!("WANWAN")
}
