//! 2次元BIT
use crate::*;
/// verified by https://atcoder.jp/contests/typical90/tasks/typical90_ab

pub struct BinaryIndexedTree2 {
    h: usize,
    w: usize,
    bit: Vec<Vec<i64>>,
}

impl BinaryIndexedTree2 {
    pub fn new(h: usize, w: usize) -> BinaryIndexedTree2 {
        let (h, w) = (h + 1, w + 1);
        let bit = vec![vec![0; w]; h];
        BinaryIndexedTree2 { h, w, bit }
    }

    pub fn add(&mut self, y: usize, x: usize, v: i64) {
        let mut idx = x as i32 + 1;
        while idx < self.w as i32 {
            let mut idy = y as i32 + 1;
            while idy < self.h as i32 {
                self.bit[idy as usize][idx as usize] += v;
                idy += idy & -idy;
            }
            idx += idx & -idx;
        }
    }

    /// sum of 0 <= y <= h & 0 <= x <= w
    pub fn sum(&self, y: usize, x: usize) -> i64 {
        let mut ret = 0;
        let mut idx = x as i32 + 1;
        while idx > 0 {
            let mut idy = y as i32 + 1;
            while idy > 0 {
                ret += self.bit[idy as usize][idx as usize];
                idy -= idy & -idy;
            }
            idx -= idx & -idx;
        }
        ret
    }

    pub fn sum_ab(&self, (y1, x1): (usize, usize), (y2, x2): (usize, usize)) -> i64 {
        self.sum(y2, x2) - self.sum(y2, x1) - self.sum(y1, x2) + self.sum(y1, x1)
    }
}

impl Debug for BinaryIndexedTree2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        buf += "\n";
        for y in 0..self.h - 1 {
            for x in 0..self.w - 1 {
                if x > 0 {
                    buf += " ";
                }
                buf += self.sum(y, x).to_string().as_str();
            }
            buf += "\n";
        }
        write!(f, "{}", buf)
    }
}
