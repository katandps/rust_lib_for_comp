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
    let m: i32 = read();
    let vv: String = match m {
        k if k < 100 => "00".into(),
        k if k < 1000 => format!("0{}", 10 * k / 1000),
        k if k <= 5000 => format!("{}", 10 * k / 1000),
        k if k <= 30000 => format!("{}", k / 1000 + 50),
        k if k <= 70000 => format!("{}", (k / 1000 - 30) / 5 + 80),
        _ => "89".into(),
    };
    println!("{}", vv)
}
