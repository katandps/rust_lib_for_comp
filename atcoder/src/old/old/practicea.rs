use std::io::*;
use std::str::*;

fn read<T: FromStr>() -> Option<T> {
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok()
}

fn main() {
    let n: usize = read().unwrap();
    let mut a: Vec<i64> = Vec::new();

    for _i in 0..n {
        a.push(read().unwrap());
    }

    let mut min = 100000;
    for ai in a.iter() {
        let mut k: i64 = *ai;
        let mut c = 0;
        while k % 2 == 0 {
            k = k / 2;
            c += 1;
        }
        min = std::cmp::min(min, c);
    }
    println!("{}", min);
}
