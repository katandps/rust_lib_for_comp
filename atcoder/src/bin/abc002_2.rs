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
    let w: String = read();
    let v: Vec<char> = w.chars().collect();
    let a: String = v
        .iter()
        .filter(|c| **c != 'a' && **c != 'i' && **c != 'u' && **c != 'e' && **c != 'o')
        .fold("".into(), |x, c| x + c.to_string().as_ref());

    println!("{}", a)
}
