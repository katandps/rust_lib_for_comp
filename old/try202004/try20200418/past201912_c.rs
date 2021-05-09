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
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());

    let n = int() as usize;
    let a = vecint(n as i64);
    for i in 1..n {
        writeln!(
            out,
            "{}",
            if a[i - 1] == a[i] {
                "stay".to_string()
            } else {
                if a[i - 1] > a[i] {
                    "down ".to_string() + (a[i - 1] - a[i]).to_string().as_ref()
                } else {
                    "up ".to_string() + (a[i] - a[i - 1]).to_string().as_ref()
                }
            }
        );
    }
}
