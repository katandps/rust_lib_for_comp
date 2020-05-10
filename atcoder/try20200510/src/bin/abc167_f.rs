#[allow(dead_code)]
fn main() {
    let n = i::u();
    let s = i::cmap(n);
    // Vec<(sのインデックス, ')'の数 - '('の数 )>
    let mut stat: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut left_only = Vec::new();
    let mut right_only = Vec::new();
    for i in 0..n {
        let s_i = &s[i];
        let mut left: i32 = 0; //'('
        let mut right = 0; //')'
        for j in 0..s_i.len() {
            if s_i[j] == ')' {
                if left > 0 {
                    left -= 1;
                } else {
                    right += 1;
                }
            } else {
                left += 1;
            }
        }
        if left > 0 && right == 0 {
            left_only.push(left);
            continue;
        }
        if right > 0 && left == 0 {
            right_only.push(right);
            continue;
        }
        if right != 0 || left != 0 {
            stat.entry(right).or_insert(Vec::new()).push(left);
        }
    }

    //dbg!(&stat);
    //right, left
    let mut heap: BinaryHeap<S> = BinaryHeap::new();
    let mut cur = left_only.iter().sum();
    let mut m = cur;
    for i in 0..=cur {
        if stat.contains_key(&i) {
            for k in &stat[&i] {
                heap.push(S {
                    diff: *k - i,
                    left: *k,
                    right: i,
                });
            }
            stat.remove(&i);
        }
    }
    // for _ in 0..heap.len() {
    //     println!("{:?}", heap.pop());
    // }
    // return;
    //dbg!(&heap);
    while heap.len() > 0 {
        let c = heap.pop().unwrap();
        //dbg!(&cur, &c);
        cur -= c.right;
        if cur < 0 {
            println!("{}", "No");
            return;
        }
        cur += c.left;
        if cur > m {
            for i in (m + 1)..=cur {
                if stat.contains_key(&i) {
                    for k in &stat[&i] {
                        heap.push(S {
                            diff: *k - i,
                            left: *k,
                            right: i,
                        });
                    }
                }
                stat.remove(&i);
            }
            m = cur;
        }
    }
    if stat.len() > 0 {
        println!("{}", "No");
        return;
    }
    cur -= right_only.iter().sum::<i32>();
    println!("{}", if cur == 0 { "Yes" } else { "No" });
    //dbg!(stat, cur, left_only, right_only);
}

#[derive(PartialEq, Eq, Debug)]
struct S {
    pub diff: i32,
    pub left: i32,
    pub right: i32,
}

impl Ord for S {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.diff == other.diff {
            self.left.cmp(&other.left)
        } else {
            self.diff.cmp(&other.diff)
        }
    }
}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(unused_imports)]
use std::cmp::*;
use std::collections::BinaryHeap;
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
