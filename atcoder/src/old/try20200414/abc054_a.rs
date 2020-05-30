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

    if a == b {
        println!("Draw");
        return;
    }

    if a == 1 {
        println!("Alice");
        return;
    }
    if b == 1 {
        println!("Bob");
        return;
    }
    println!("{}", if a > b { "Alice" } else { "Bob" })
}
