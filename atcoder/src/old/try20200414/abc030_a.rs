use std::io::*;
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
    let d: i32 = read();
    if b * c == d * a {
        println!("DRAW");
        return;
    }
    println!("{}", if b * c > d * a { "TAKAHASHI" } else { "AOKI" })
}
