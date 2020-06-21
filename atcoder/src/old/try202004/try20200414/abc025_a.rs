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
    let n: i32 = read();

    let v: Vec<char> = s.chars().collect();

    let mut count = 1;
    for a in &v {
        for b in &v {
            if count == n {
                println!("{}{}", a, b);
                return;
            }
            count += 1;
        }
    }
}
