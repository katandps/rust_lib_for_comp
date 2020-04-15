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
    let n: usize = read();
    let k: i32 = read();
    let mut t: Vec<i32> = Vec::new();
    for _ in 0..n {
        t.push(read())
    }

    for i in 2..n {
        if t[i] + t[i - 1] + t[i - 2] < k {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1")
}
