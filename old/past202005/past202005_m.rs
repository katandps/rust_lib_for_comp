#[allow(dead_code)]
fn main() {
    let (n, m) = i::u2();
    let uv = i::uv2(m);
    let s = i::u();
    let k = i::u();
    let mut t = vec![s];
    t.append(i::uv(k).as_mut());

    let mut map = vec![HashSet::new(); n + 1];
    for (u, v) in uv {
        map[u].insert(v);
        map[v].insert(u);
    }

    // distance[i][j] = i から jまでに移動する必要のある距離
    let mut distance = vec![vec![10000000usize; k + 1]; k + 1];
    for i in 0..=k {
        distance[i][i] = 0;
    }

    for i in 0..=k {
        let t_i = t[i];
        let mut q = VecDeque::new();
        q.push_front(t_i);
        let mut memo = HashSet::new();
        memo.insert(t_i);
        let mut dist = vec![10000000usize; n + 1];
        dist[t_i] = 0;
        while q.len() > 0 {
            let cur = q.pop_front().unwrap();
            for &c in &map[cur] {
                if memo.contains(&c) {
                    continue;
                }
                memo.insert(c);
                dist[c] = dist[cur] + 1;
                q.push_back(c);
            }
        }

        for j in 0..=k {
            if t_i == t[j] {
                continue;
            }
            distance[i][j] = dist[t[j]];
            distance[j][i] = dist[t[j]];
        }
    }

    let nn = distance.len();
    let mut dp = vec![vec![1_000_000_000_000usize; nn]; 1 << nn];

    dp[(1 << nn) - 1][0] = 0;

    for i in 0..=(1 << nn) - 2 {
        let s = (1 << nn) - 2 - i;
        for v in 0..nn {
            for u in 0..nn {
                if (s >> u & 1) == 0 {
                    dp[s][v] = min(dp[s][v], dp[s | 1 << u][u] + distance[v][u]);
                }
            }
        }
    }

    let mut ans = 1_000_000_000_000;
    for i in 1..dp[0].len() {
        ans = min(ans, dp[0][i]);
    }
    println!("{}", ans);
    //   dbg!(dp, distance);
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
