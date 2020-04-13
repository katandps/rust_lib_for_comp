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
    let mut v: Vec<i32> = vec![100, 100, 200];

    for i in 0..20 {
        let k = v[i];
        let l = v[i + 1];
        let m = v[i + 2];
        v.push(k + l + m);
    }
    println!("{}", v[19])
}
