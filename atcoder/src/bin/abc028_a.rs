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
    let n: i32 = read();
    let s = match n {
        i if i < 60 => "Bad",
        i if i < 90 => "Good",
        i if i < 100 => "Great",
        _ => "Perfect",
    };
    println!("{}", s)
}
