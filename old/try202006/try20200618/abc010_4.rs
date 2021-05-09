#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let mut reader = StdinReader::new(stdin.lock());
    let (n, g, e) = reader.u3();
    let p = reader.uv(g);
    let ab = reader.uv2(e);

    let source = 0;
    let sink = n;
    let mut dinitz = Dinitz::new(n + 1);

    for (a, b) in ab {
        dinitz.add_edge(a, b, 1);
        dinitz.add_edge(b, a, 1);
    }
    for g in p {
        dinitz.add_edge(g, sink, 1);
    }
    let flow = dinitz.max_flow(source, sink);
    println!("{}", flow);
}

#[allow(dead_code)]
mod dinitz {
    use std::collections::VecDeque;

    struct Edge {
        pub to: usize,
        pub rev: usize,
        pub cap: i64,
    }

    pub(crate) struct Dinitz {
        g: Vec<Vec<Edge>>,
        level: Vec<i32>,
        iter: Vec<usize>,
    }

    impl Dinitz {
        pub fn new(v: usize) -> Dinitz {
            let mut g: Vec<Vec<Edge>> = Vec::new();
            for _ in 0..v {
                g.push(Vec::new());
            }
            Dinitz {
                g: g,
                level: vec![0; v],
                iter: vec![0; v],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64) {
            let to_len = self.g[to].len();
            let from_len = self.g[from].len();
            self.g[from].push(Edge {
                to: to,
                rev: to_len,
                cap: cap,
            });
            self.g[to].push(Edge {
                to: from,
                rev: from_len,
                cap: 0,
            });
        }

        fn dfs(&mut self, v: usize, t: usize, f: i64) -> i64 {
            if v == t {
                return f;
            }
            while self.iter[v] < self.g[v].len() {
                let (e_cap, e_to, e_rev);
                {
                    let ref e = self.g[v][self.iter[v]];
                    e_cap = e.cap;
                    e_to = e.to;
                    e_rev = e.rev;
                }
                if e_cap > 0 && self.level[v] < self.level[e_to] {
                    let d = self.dfs(e_to, t, std::cmp::min(f, e_cap));
                    if d > 0 {
                        {
                            let ref mut e = self.g[v][self.iter[v]];
                            e.cap -= d;
                        }
                        {
                            let ref mut rev_edge = self.g[e_to][e_rev];
                            rev_edge.cap += d;
                        }
                        return d;
                    }
                }
                self.iter[v] += 1;
            }

            return 0;
        }

        fn bfs(&mut self, s: usize) {
            let v = self.level.len();
            self.level = vec![-1; v];
            self.level[s] = 0;
            let mut deque = VecDeque::new();
            deque.push_back(s);
            while !deque.is_empty() {
                let v = deque.pop_front().unwrap();
                for e in &self.g[v] {
                    if e.cap > 0 && self.level[e.to] < 0 {
                        self.level[e.to] = self.level[v] + 1;
                        deque.push_back(e.to);
                    }
                }
            }
        }

        pub fn max_flow(&mut self, s: usize, t: usize) -> i64 {
            let v = self.level.len();
            let mut flow: i64 = 0;
            loop {
                self.bfs(s);
                if self.level[t] < 0 {
                    return flow;
                }
                self.iter = vec![0; v];
                loop {
                    let f = self.dfs(s, t, std::i64::MAX);
                    if f == 0 {
                        break;
                    }
                    flow += f;
                }
            }
        }
    }
}

#[allow(unused_imports)]
use dinitz::*;

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::num::*;
#[allow(unused_imports)]
use std::str::*;
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
