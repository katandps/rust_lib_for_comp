#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let mut reader = StdinReader::new(stdin.lock());
    let n = reader.u();
    let xy = reader.uv2(n - 1);
    let q = reader.u();
    let ab = reader.uv2(q);

    let mut g = vec![vec![]; n];
    for &(x, y) in &xy {
        g[a].push(b);
        g[b].push(a);
    }
    let mut lca = LowestCommonAncestor::new(&g, 0);

    for (a, b) in ab {
        println!("{}", lca.get_dist(a - 1, b - 1) + 1);
    }
}

#[allow(dead_code)]
mod lowest_common_ancestor {
    use std::mem::swap;

    /// LowestCommonAncestor(LCA)を求めるライブラリ
    /// 事前処理NlogN、クエリlogN
    pub struct LowestCommonAncestor {
        parent: Vec<Vec<usize>>,
        dist: Vec<usize>,
    }

    impl LowestCommonAncestor {
        pub fn new(graph: &Vec<Vec<usize>>, root: usize) -> LowestCommonAncestor {
            let v = graph.len();
            let mut k = 1;
            while (1 << k) < v {
                k += 1;
            }
            let mut lca = LowestCommonAncestor {
                parent: vec![vec![std::usize::MAX; v]; k],
                dist: vec![std::usize::MAX; v],
            };
            lca.dfs(graph, root, std::usize::MAX, 0);
            for k in 0..k - 1 {
                for v in 0..v {
                    if lca.parent[k][v] == std::usize::MAX {
                        lca.parent[k + 1][v] = 1;
                    } else {
                        lca.parent[k + 1][v] = lca.parent[k][lca.parent[k][v]];
                    }
                }
            }
            lca
        }

        /// graph: グラフ
        /// v: 今見ている頂点
        /// p: parent
        /// d: 根からの距離
        fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, p: usize, d: usize) {
            self.parent[0][v] = p;
            self.dist[v] = d;
            for &to in &graph[v] {
                if to != p {
                    self.dfs(graph, to, v, d + 1);
                }
            }
        }

        /// u,vの最近共通祖先(LCA)を求める(O(logN))
        pub fn query(&mut self, mut u: usize, mut v: usize) -> usize {
            // uの深さはvの深さ以上
            if self.dist[u] < self.dist[v] {
                swap(&mut u, &mut v);
            }
            let k = self.parent.len();
            // LCAまでの距離を揃える
            for k in 0..k {
                if (self.dist[u] - self.dist[v]) >> k & 1 == 1 {
                    u = self.parent[k][u];
                }
            }
            if u == v {
                u
            } else {
                for i in 0..k {
                    let k = k - i - 1;
                    if self.parent[k][u] != self.parent[k][v] {
                        u = self.parent[k][u];
                        v = self.parent[k][v];
                    }
                }
                self.parent[0][u]
            }
        }

        /// 2頂点u,v間の距離を求める
        pub fn get_dist(&mut self, u: usize, v: usize) -> usize {
            let lca = self.query(u, v);
            self.dist[u] + self.dist[v] - 2 * self.dist[lca]
        }

        /// u,vを結ぶパス上に頂点aが存在するかどうか
        pub fn on_path(&mut self, u: usize, v: usize, a: usize) -> bool {
            self.get_dist(u, a) + self.get_dist(a, v) == self.get_dist(u, v)
        }
    }
}

#[allow(unused_imports)]
use lowest_common_ancestor::LowestCommonAncestor;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::{cmp::*, collections::*, io::*, num::*, str::*};
#[allow(unused_imports)]
use stdin_reader::StdinReader;

#[allow(dead_code)]
mod stdin_reader {
    use std::fmt::Debug;
    use std::io::*;
    use std::str::*;

    pub struct StdinReader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
        // Should never be empty
        pos: usize, // Should never be out of bounds as long as the input ends with '\n'
    }

    impl<R: BufRead> StdinReader<R> {
        pub fn new(r: R) -> StdinReader<R> {
            StdinReader {
                reader: r,
                buf: Vec::new(),
                pos: 0,
            }
        }
        pub fn next<T: FromStr>(&mut self) -> T
        where
            T::Err: Debug,
        {
            if self.buf.is_empty() {
                self._read_next_line();
            }
            let mut start = None;
            loop {
                if self.pos == self.buf.len() {
                    break;
                }
                match (self.buf[self.pos], start.is_some()) {
                    (b' ', true) | (b'\n', true) => break,
                    (_, true) | (b' ', false) => self.pos += 1,
                    (b'\n', false) => self._read_next_line(),
                    (_, false) => start = Some(self.pos),
                }
            }
            let target = &self.buf[start.unwrap()..self.pos];
            from_utf8(target).unwrap().parse().unwrap()
        }

        fn _read_next_line(&mut self) {
            self.pos = 0;
            self.buf.clear();
            if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
                panic!("Reached EOF");
            }
        }

        pub fn str(&mut self) -> String {
            self.next()
        }
        pub fn s(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }
        pub fn i(&mut self) -> i64 {
            self.next()
        }
        pub fn i2(&mut self) -> (i64, i64) {
            (self.next(), self.next())
        }
        pub fn i3(&mut self) -> (i64, i64, i64) {
            (self.next(), self.next(), self.next())
        }
        pub fn u(&mut self) -> usize {
            self.next()
        }
        pub fn u2(&mut self) -> (usize, usize) {
            (self.next(), self.next())
        }
        pub fn u3(&mut self) -> (usize, usize, usize) {
            (self.next(), self.next(), self.next())
        }
        pub fn u4(&mut self) -> (usize, usize, usize, usize) {
            (self.next(), self.next(), self.next(), self.next())
        }
        pub fn u5(&mut self) -> (usize, usize, usize, usize, usize) {
            (
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
            )
        }
        pub fn u6(&mut self) -> (usize, usize, usize, usize, usize, usize) {
            (
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
            )
        }
        pub fn f(&mut self) -> f64 {
            self.next()
        }
        pub fn f2(&mut self) -> (f64, f64) {
            (self.next(), self.next())
        }
        pub fn c(&mut self) -> char {
            self.next::<String>().pop().unwrap()
        }
        pub fn iv(&mut self, n: usize) -> Vec<i64> {
            (0..n).map(|_| self.i()).collect()
        }
        pub fn iv2(&mut self, n: usize) -> Vec<(i64, i64)> {
            (0..n).map(|_| self.i2()).collect()
        }
        pub fn iv3(&mut self, n: usize) -> Vec<(i64, i64, i64)> {
            (0..n).map(|_| self.i3()).collect()
        }
        pub fn uv(&mut self, n: usize) -> Vec<usize> {
            (0..n).map(|_| self.u()).collect()
        }
        pub fn uv2(&mut self, n: usize) -> Vec<(usize, usize)> {
            (0..n).map(|_| self.u2()).collect()
        }
        pub fn uv3(&mut self, n: usize) -> Vec<(usize, usize, usize)> {
            (0..n).map(|_| self.u3()).collect()
        }
        pub fn uv4(&mut self, n: usize) -> Vec<(usize, usize, usize, usize)> {
            (0..n).map(|_| self.u4()).collect()
        }
        pub fn fv(&mut self, n: usize) -> Vec<f64> {
            (0..n).map(|_| self.f()).collect()
        }
        pub fn cmap(&mut self, h: usize) -> Vec<Vec<char>> {
            (0..h).map(|_| self.s()).collect()
        }
    }
}
