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
    let n = int() as usize;
    let mut a = Vec::new();
    a.push(0usize);
    for _ in 0..n {
        a.push(int() as usize)
    }

    let mut memo = HashSet::new();
    let mut cur = 1;
    let mut ans = 1;
    memo.insert(1 as usize);
    loop {
        if memo.contains(&a[cur]) {
            println!("-1");
            return;
        } else {
            let target = a[cur];
            if target == 2 {
                println!("{}", ans);
                return;
            }

            memo.insert(target);
            cur = target;
            ans += 1;
        }
    }
}
