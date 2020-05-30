#[allow(dead_code)]
fn main() {
    let (n, m) = i::u2();
    let abt = i::uv3(m);
    let wf = warshall_floyd(
        n,
        m,
        abt.iter().map(|t| t.0).collect(),
        abt.iter().map(|t| t.1).collect(),
        abt.iter().map(|t| t.2).collect(),
    );

    let mut min_dis = std::usize::MAX;
    for i in 1..n + 1 {
        let mut max_dis = 0;
        for j in 1..n + 1 {
            max_dis = max(max_dis, wf[i][j]);
        }
        min_dis = min(min_dis, max_dis);
    }
    println!("{}", min_dis);
}

#[allow(unused_imports)]
use warshall_floyd::*;

#[allow(dead_code)]
mod warshall_floyd {
    use std::cmp::min;

    pub fn warshall_floyd(
        vertex_n: usize,
        edge_n: usize,
        a: Vec<usize>,
        b: Vec<usize>,
        cost: Vec<usize>,
    ) -> Vec<Vec<usize>> {
        let mut ret = vec![vec![1_000_000_000usize; vertex_n + 1]; vertex_n + 1];
        for i in 0..vertex_n + 1 {
            ret[i][i] = 0;
        }
        for i in 0..edge_n {
            ret[a[i]][b[i]] = min(ret[a[i]][b[i]], cost[i]);
            ret[b[i]][a[i]] = min(ret[b[i]][a[i]], cost[i]); //有向グラフの場合はコメントアウト
        }
        for i in 0..vertex_n + 1 {
            for j in 0..vertex_n + 1 {
                for k in 0..vertex_n + 1 {
                    ret[j][k] = min(ret[j][k], ret[j][i] + ret[i][k])
                }
            }
        }
        ret
    }
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
