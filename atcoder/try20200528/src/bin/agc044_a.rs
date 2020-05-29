#[allow(dead_code)]
fn main() {
    let t = i::u();
    for _ in 0..t {
        let (n, a, b, c, d) = (i::i(), i::i(), i::i(), i::i(), i::i());

        let mut q = VecDeque::new();
        q.push_back((n, 0)); //cur, score
        let mut memo = HashMap::new();
        memo.insert(n, 0);
        memo.insert(1, std::i64::MAX);
        memo.insert(0, std::i64::MAX);
        while q.len() > 0 {
            let cur = q.pop_front().unwrap();
            //dbg!(&cur);

            let k = cur.0;

            if k <= 1_000_000_000_000_000_000 / d {
                *memo.entry(0).or_insert(std::i64::MAX) =
                    min(cur.1 + d * k, *memo.get(&0).unwrap());
            }

            if k == 0 {
                *memo.entry(0).or_insert(std::i64::MAX) = min(cur.1, *memo.get(&0).unwrap());
                continue;
            }
            if k == 1 {
                *memo.entry(1).or_insert(std::i64::MAX) = min(cur.1, *memo.get(&1).unwrap());
                *memo.entry(0).or_insert(std::i64::MAX) = min(cur.1 + d, *memo.get(&0).unwrap());
                continue;
            }

            f(k / 5, cur.1 + c + (k % 5) * d, &mut memo, &mut q);
            if k % 5 != 0 {
                f(k / 5 + 1, cur.1 + c + (5 - k % 5) * d, &mut memo, &mut q);
            }

            f(k / 3, cur.1 + b + (k % 3) * d, &mut memo, &mut q);
            if k % 3 != 0 {
                f(k / 3 + 1, cur.1 + b + (3 - k % 3) * d, &mut memo, &mut q);
            }

            f(k / 2, cur.1 + a + (k % 2) * d, &mut memo, &mut q);
            if k % 2 != 0 {
                f(k / 2 + 1, cur.1 + a + (2 - k % 2) * d, &mut memo, &mut q);
            }
        }
        //dbg!(&memo);
        println!("{}", memo[&0]);
    }
}

fn f(s: i64, t: i64, memo: &mut HashMap<i64, i64>, q: &mut VecDeque<(i64, i64)>) {
    if memo.contains_key(&s) && memo[&s] <= t {
        return;
    }
    *memo.entry(s).or_insert(std::i64::MAX) = min(*memo.entry(s).or_insert(std::i64::MAX), t);
    q.push_front((s, t));
}

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::num::*;
#[allow(unused_imports)]
use std::str::*;

#[allow(dead_code)]
mod i {
    use super::*;

    pub fn read<T: FromStr>() -> T {
        stdin()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<T>()
            .ok()
            .unwrap()
    }

    pub fn str() -> String {
        read()
    }

    pub fn s() -> Vec<char> {
        str().chars().collect()
    }

    pub fn i() -> i64 {
        read()
    }

    pub fn u() -> usize {
        read()
    }

    pub fn u2() -> (usize, usize) {
        (read(), read())
    }

    pub fn u3() -> (usize, usize, usize) {
        (read(), read(), read())
    }

    pub fn f() -> f64 {
        read()
    }

    pub fn c() -> char {
        read::<String>().pop().unwrap()
    }

    pub fn iv(n: usize) -> Vec<i64> {
        (0..n).map(|_| i()).collect()
    }

    pub fn iv2(n: usize) -> Vec<(i64, i64)> {
        (0..n).map(|_| iv(2)).map(|a| (a[0], a[1])).collect()
    }

    pub fn uv(n: usize) -> Vec<usize> {
        (0..n).map(|_| u()).collect()
    }

    pub fn uv2(n: usize) -> Vec<(usize, usize)> {
        (0..n).map(|_| uv(2)).map(|a| (a[0], a[1])).collect()
    }

    pub fn uv3(n: usize) -> Vec<(usize, usize, usize)> {
        (0..n).map(|_| uv(3)).map(|a| (a[0], a[1], a[2])).collect()
    }

    pub fn fv(n: usize) -> Vec<f64> {
        (0..n).map(|_| f()).collect()
    }

    pub fn cmap(h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| s()).collect()
    }
}
