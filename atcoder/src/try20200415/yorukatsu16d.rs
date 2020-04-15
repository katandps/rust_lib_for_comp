use std::cmp::*;
use std::collections::VecDeque;
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
    let k = int();
    let mut q = (1..10)
        .map(|i| ('0' as u8) + i)
        .map(|i| {
            let mut vec = VecDeque::new();
            vec.push_front(i as char);
            vec
        })
        .collect::<VecDeque<VecDeque<char>>>();

    let mut count = 0;
    while q.len() > 0 {
        let mut cur = q.pop_front().unwrap();
        count += 1;
        //println!("{}", cur.iter().collect::<String>());
        if count == k {
            for c in cur {
                print!("{}", c);
            }
            println!("");
            return;
        }
        let last = cur.back().unwrap();
        if last > &'0' {
            let mut minus1 = cur.clone();
            minus1.push_back((*last as u8 - 1) as char);
            q.push_back(minus1);
        }

        let mut simple = cur.clone();
        simple.push_back(last.clone());
        q.push_back(simple);

        if last < &'9' {
            let mut plus1 = cur.clone();
            plus1.push_back((*last as u8 + 1) as char);
            q.push_back(plus1);
        }
    }
}
