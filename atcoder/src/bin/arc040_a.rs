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
    let mut r: usize = 0;
    let mut b: usize = 0;
    for _ in 0..n {
        let ss: String = read();

        r += ss
            .chars()
            .flat_map(|c| match c {
                'R' => Some(()),
                _ => None,
            })
            .count();
        b += ss
            .chars()
            .flat_map(|c| match c {
                'B' => Some(()),
                _ => None,
            })
            .count();
    }
    println!(
        "{}",
        if r == b {
            "DRAW"
        } else {
            if r > b {
                "TAKAHASHI"
            } else {
                "AOKI"
            }
        }
    )
}
