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
    let n = int();

    let mut divisor = HashSet::new();

    let mut l = 1;
    while l * l <= n {
        if n % l == 0 {
            divisor.insert(l);
            divisor.insert(n / l);
        }
        l += 1;
    }

    let mut mdivisor = HashSet::new();
    let m = n - 1;
    let mut l = 1;
    while l * l <= m {
        if m % l == 0 {
            mdivisor.insert(l);
            mdivisor.insert(m / l);
        }
        l += 1;
    }

    divisor.remove(&1);
    //println!("{:?}", divisor);

    mdivisor.remove(&1);
    //println!("{:?}", mdivisor);

    for d in divisor {
        let mut k = n;
        while k % d == 0 {
            k = k / d;
        }
        if k == 1 || k % d == 1 {
            mdivisor.insert(d);
        }
    }
    //println!("{:?}", mdivisor);
    println!("{}", mdivisor.len())
}
