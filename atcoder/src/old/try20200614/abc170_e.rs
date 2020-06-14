#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let mut reader = StdinReader::new(stdin.lock());
    let (n, q) = reader.u2();
    let ab = reader.uv2(n);
    let cd = reader.uv2(q);

    //1-indexed 幼児のパワー
    let mut power = vec![0];

    //1-indexed 幼児の所属園 （削除に使う）
    let mut shozoku = vec![0];

    //1-indexed 幼稚園に所属している園児のSET
    let mut set = vec![BTreeMap::new(); 200001];

    //i-indexed 幼稚園の中でのレートランキング の優先度付きキュー
    let mut all = BTreeMap::new();

    for i in 0..n {
        power.push(ab[i].0);
        shozoku.push(ab[i].1);

        *set[ab[i].1].entry(Reverse(ab[i].0)).or_insert(0) += 1;
    }

    for kin in set.clone() {
        match kin.iter().nth(0) {
            Some((p, num)) => {
                *all.entry(match p {
                    Reverse(p) => p.clone(),
                })
                .or_insert(0) += 1;
            }
            None => {}
        }
    }

    for (c, new) in cd {
        let old = shozoku[c];
        let p = Reverse(power[c]);
        // 移動前の幼稚園の最強園児
        let Reverse(old_max) = *set[old].iter().next().unwrap().0;
        // 移動させる園児とPowerが一致していれば全体から取り除く
        if old_max == power[c] {
            *all.entry(old_max).or_insert(0) -= 1;
            if all[&old_max] == 0 {
                all.remove(&old_max);
            }
        }
        // 移動前の幼稚園から園児を取り除く
        *set[old].entry(p).or_insert(0) -= 1;
        if set[old][&p] == 0 {
            set[old].remove(&p);
        }
        // 移動前の幼稚園に園児が残っていれば全体に加える
        if old_max == power[c] && set[old].len() > 0 {
            let Reverse(old_max) = *set[old].iter().next().unwrap().0;
            *all.entry(old_max).or_insert(0) += 1;
        }
        // 移動先の幼稚園に園児がいたら、最強園児を全体から取り除く
        if set[new].len() > 0 {
            let Reverse(new_max) = *set[new].iter().next().unwrap().0;
            *all.entry(new_max).or_insert(0) -= 1;
            if all[new_max] == 0 {
                all.remove(&new_max);
            }
        }
        // 移動先に園児を移動させる
        *set[new].entry(p).or_insert(0) += 1;
        // 移動先の最強園児を全体に加える
        let Reverse(new_max) = *set[new].iter().next().unwrap().0;
        *all.entry(new_max).or_insert(0) += 1;
        // 所属を更新
        shozoku[c] = new;

        println!("{}", all.iter().next().unwrap().0);
    }
}

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
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
