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
    let x: i32 = read();
    let y: i32 = read();

    if x == 1 && y == 1 {
        println!("1000000")
    } else {
        let mut ans = 0;
        ans += match x {
            1 => 300000,
            2 => 200000,
            3 => 100000,
            _ => 0,
        };
        ans += match y {
            1 => 300000,
            2 => 200000,
            3 => 100000,
            _ => 0,
        };
        println!("{}", ans)
    }
}
