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
    let mut s: String = read();
    let mut c = 'C';
    while !s.is_empty() {
        let char = s.pop().unwrap();
        if c == char {
            if c == 'C' {
                c = 'A';
                continue;
            } else {
                println!("Yes");
                return;
            }
        }
        c = 'C';
    }
    println!("No")
}
