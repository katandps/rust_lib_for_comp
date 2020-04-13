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
    let mut a: String = read();
    let mut b: String = read();
    let aa: bool = a.pop().unwrap() == 'H';
    let bb: bool = b.pop().unwrap() == 'H';
    println!("{}", if !aa ^ bb { "H" } else { "D" })
}
