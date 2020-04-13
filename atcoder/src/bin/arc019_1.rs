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
    println!(
        "{}",
        s.replace("O", "0")
            .replace("D", "0")
            .replace("I", "1")
            .replace("Z", "2")
            .replace("S", "5")
            .replace("B", "8")
    )
}
