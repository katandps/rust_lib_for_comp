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
    let h: i32 = read();
    let w: i32 = read();
    let mut ans = 0;
    if h >= 2 {
        ans += w * (h - 1)
    }
    if w >= 2 {
        ans += h * (w - 1)
    }
    println!("{}", ans)
}
