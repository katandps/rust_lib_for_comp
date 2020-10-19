#[allow(unused_imports)]
use binary_indexed_tree::*;

#[allow(dead_code)]
mod binary_indexed_tree {
    pub struct BinaryIndexedTree {
        n: usize,
        bit: Vec<VALUE>,
    }

    type VALUE = i64;
    impl BinaryIndexedTree {
        pub fn new(n: usize) -> BinaryIndexedTree {
            BinaryIndexedTree {
                n: n + 1,
                bit: vec![0; 1000000],
            }
        }

        pub fn add(&mut self, i: usize, x: VALUE) {
            let mut idx = i as i32;
            while idx < self.n as i32 {
                self.bit[idx as usize] += x;
                idx += idx & -idx;
            }
        }

        pub fn sum(&self, i: usize) -> VALUE {
            let mut ret = 0;
            let mut idx = i as i32;
            while idx > 0 {
                ret += self.bit[idx as usize];
                idx -= idx & -idx;
            }
            ret
        }
    }
}
