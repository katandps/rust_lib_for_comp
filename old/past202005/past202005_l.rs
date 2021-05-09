#[allow(dead_code)]
fn main() {
    let n = i::u();
    let mut k = Vec::new();
    let mut t = Vec::new();

    let mask: usize = 1_000_000_000_000;

    for _ in 0..n {
        let k_i = i::u();
        k.push(k_i);
        let mut t_i = i::uv(k_i);
        t_i.push(0);
        t_i.push(0);
        t.push(t_i);
    }
    let m = i::u();
    let a = i::uv(m);

    let mut first = BTreeMap::new();
    let mut second = BTreeMap::new();

    // 一番目に並べてある商品の消費期限
    let mut first_tana = vec![0; n + 1];
    let mut second_tana = vec![0; n + 1];

    let mut indexes = vec![2; n + 1];

    for i in 0..n {
        first.insert(mask - t[i][0], i);
        first_tana[i] = mask - t[i][0];
        second.insert(mask - t[i][1], i);
        second_tana[i] = mask - t[i][1];
    }

    //dbg!(&first, &second);
    for customer_i in 0..m {
        match a[customer_i] {
            1 => {
                let buy = first.iter().next().unwrap();
                let buy = (*buy.0, *buy.1);
                println!("{}", mask - buy.0);

                let tana = buy.1;
                first.remove(&first_tana[tana]);
                first.insert(mask - t[tana][indexes[tana] - 1], tana);
                first_tana[tana] = second_tana[tana];
                second.remove(&second_tana[tana]);
                second.insert(mask - t[tana][indexes[tana]], tana);
                second_tana[tana] = mask - t[tana][indexes[tana]];
                indexes[tana] += 1;
                //dbg!(&first, &second, &tana);
            }
            2 => {
                let buy1 = first.iter().next().unwrap();
                let buy2 = second.iter().next().unwrap();
                // 小さい方を優先する(正負反転済み)
                if buy1.0 < buy2.0 {
                    let buy = first.iter().next().unwrap();
                    let buy = (*buy.0, *buy.1);
                    println!("{}", mask - buy.0);

                    let tana = buy.1;
                    first.remove(&first_tana[tana]);
                    first.insert(mask - t[tana][indexes[tana] - 1], tana);
                    first_tana[tana] = second_tana[tana];
                    second.remove(&second_tana[tana]);
                    second.insert(mask - t[tana][indexes[tana]], tana);
                    second_tana[tana] = mask - t[tana][indexes[tana]];
                    indexes[tana] += 1;
                //dbg!(&first, &second, &tana);
                } else {
                    let buy = second.iter().next().unwrap();
                    let buy = (*buy.0, *buy.1);
                    println!("{}", mask - buy.0);

                    let tana = buy.1;
                    second.remove(&second_tana[tana]);
                    second.insert(mask - t[tana][indexes[tana]], tana);
                    second_tana[tana] = mask - t[tana][indexes[tana]];
                    indexes[tana] += 1;
                    //dbg!(&first, &second, &tana);
                }
            }
            _ => unreachable!(),
        }
    }

    //dbg!(first, second);
}

#[derive(Eq, PartialEq)]
struct Item {
    pub tana: usize,
    pub limit: usize,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.limit.cmp(&other.limit)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
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
