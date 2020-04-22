use std::cmp::*;
use std::collections::HashSet;
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
    let (a, b) = (int(), int());
    let (adiv, bdiv) = (div(a), div(b));

    let mut ans = 0;
    for ad in adiv {
        if bdiv.contains(&ad) {
            ans += 1;
        }
    }
    println!("{}", ans)
}

fn div(n: i64) -> HashSet<i64> {
    let mut div = HashSet::new();
    let mut l = 2;
    let mut k = n;
    while l * l <= k {
        if k % l == 0 {
            div.insert(l);
            k /= l;
            continue;
        }
        l += 1
    }
    div.insert(k);
    div.insert(1);
    div
}
