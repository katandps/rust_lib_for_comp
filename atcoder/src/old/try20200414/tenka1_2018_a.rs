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
    let mut v: Vec<char> = s.chars().collect();
    if v.len() == 3 {
        v.reverse();
    }
    let a: String = v.iter().fold("".into(), |x, c| x + c.to_string().as_ref());
    println!("{}", a);
}
