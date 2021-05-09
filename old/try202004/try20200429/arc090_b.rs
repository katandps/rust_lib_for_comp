#[allow(dead_code)]
fn main() {
    let (n, m) = i::u2();
    let lrd = i::uv3(m);

    let mut map: Vec<HashMap<usize, i64>> = vec![HashMap::new(); n + 1];
    let mut uf = UnionFind::new(n + 1);
    for i in 0..m {
        let (l, r, d) = lrd[i];
        map[l].insert(r, d as i64);
        map[r].insert(l, -(d as i64));
        uf.unite(l, r);
    }

    let mut q = VecDeque::new();
    let mut d: HashMap<usize, i64> = HashMap::new();

    let mut s = HashSet::new();
    for i in 1..n + 1 {
        let r = uf.root(i);
        if !s.contains(&r) {
            s.insert(r);
            q.push_front(r);
            d.insert(r, 0);
        }
    }
    //dbg!(&d, &map);
    while q.len() > 0 {
        let c = q.pop_front().unwrap();
        //dbg!(&c);
        for i in &map[c] {
            if d.contains_key(&i.0) && d[&c] + *i.1 != d[i.0] {
                //dbg!(d, i, c);
                println!("{}", "No");
                return;
            }
            if !d.contains_key(&i.0) {
                let t = d[&c] + *i.1;
                d.insert(*i.0, t);
                q.push_back(*i.0);
            }
        }
    }

    //dbg!(d);
    println!("{}", "Yes");
}

#[allow(unused_imports)]
use union_find::*;

#[allow(dead_code)]
mod union_find {
    pub struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            let mut parent = vec![0; n + 1];
            let rank = vec![0; n + 1];
            for i in 1..(n + 1) {
                parent[i] = i;
            }
            UnionFind {
                parent: parent,
                rank: rank,
            }
        }

        pub fn root(&mut self, x: usize) -> usize {
            if self.parent[x] == x {
                x
            } else {
                let p = self.parent[x];
                self.parent[x] = self.root(p);
                self.parent[x]
            }
        }

        pub fn rank(&self, x: usize) -> usize {
            self.rank[x]
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }

        pub fn unite(&mut self, x: usize, y: usize) {
            let mut x = self.root(x);
            let mut y = self.root(y);
            if x == y {
                return;
            }
            if self.rank(x) < self.rank(y) {
                let tmp = y;
                y = x;
                x = tmp;
            }
            if self.rank(x) == self.rank(y) {
                self.rank[x] += 1;
            }
            self.parent[x] = y;
        }
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
