#[allow(unused_imports)]
use binary_indexed_tree::*;

#[allow(dead_code)]
mod binary_indexed_tree {
    #[derive(Clone)]
    pub struct BinaryIndexedTree {
        n: usize,
        bit: Vec<VALUE>,
    }

    type VALUE = i64;
    impl BinaryIndexedTree {
        pub fn new(n: usize) -> BinaryIndexedTree {
            let n = n + 1;
            let bit = vec![0; n + 1];
            BinaryIndexedTree { n, bit }
        }

        /// add x to i
        pub fn add(&mut self, i: usize, x: VALUE) {
            let i = i + 1; //0-indexed
            let mut idx = i as i32;
            while idx < self.n as i32 {
                self.bit[idx as usize] += x;
                idx += idx & -idx;
            }
        }

        /// sum of [0, i)
        pub fn sum(&self, i: usize) -> VALUE {
            let i = i + 1;
            let mut ret = 0;
            let mut idx = i as i32;
            while idx > 0 {
                ret += self.bit[idx as usize];
                idx -= idx & -idx;
            }
            ret
        }

        /// sum of [a, b)
        pub fn sum_ab(&self, a: usize, b: usize) -> VALUE {
            self.sum(b) - self.sum(a)
        }
    }

    impl std::fmt::Debug for BinaryIndexedTree {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use itertools::*;
            let v = (1..self.n).map(|i| self.sum(i) - self.sum(i - 1)).join(" ");
            write!(f, "{}", v)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        const LEN: usize = 1000;
        let mut v = vec![0; LEN];
        let mut bit = BinaryIndexedTree::new(LEN);

        for _ in 0..1000 {
            let left = rand::thread_rng().gen_range(0, LEN);
            let right = rand::thread_rng().gen_range(left, LEN);

            for i in left..right {
                v[i] += 1;
            }
            bit.add(left, 1);
            bit.add(right, -1);
        }

        for i in 0..LEN {
            assert_eq!(v[i], bit.sum(i));
        }
    }

    #[test]
    fn test_hand() {
        const LEN: usize = 10;
        let mut v = vec![0; LEN];
        let mut bit = BinaryIndexedTree::new(LEN);

        for i in 3..5 {
            v[i] += 1;
        }
        bit.add(3, 1);
        bit.add(5, -1);

        for i in 0..LEN {
            assert_eq!(v[i], bit.sum(i));
        }
    }
}
