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

fn string() -> String {
    read()
}

fn int() -> i64 {
    read()
}

fn char() -> char {
    read::<String>().pop().unwrap()
}

fn vecchar() -> Vec<char> {
    string().chars().collect()
}

fn vecint(n: i64) -> Vec<i64> {
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(int())
    }
    vec
}

fn main() {
    let (n, m) = (int() as i32, int() as i32);
    let mut a = Vec::new();
    for _ in 0..n {
        a.push((int() as i32, int() as i32));
    }
    let mut c = Vec::new();
    for _ in 0..m {
        c.push((int() as i32, int() as i32));
    }

    for (x, y) in a {
        let min = c
            .iter()
            .zip((1..(m + 1)))
            .map(|(c, i)| ((x - c.0).abs() + (y - c.1).abs(), i))
            .fold((0, -1), |(xdis, xi), (dis, i)| {
                if xi == -1 {
                    (dis, i)
                } else {
                    if dis < xdis {
                        (dis, i)
                    } else {
                        (xdis, xi)
                    }
                }
            });
        println!("{}", min.1)
    }
}
