//! Fenwick Tree(Binary Indexed Tree)
use crate::algebra::AbelianGroup;
use crate::*;

/// # Fenwick Tree
/// アーベル群の二項演算を載せることができる
#[derive(Clone)]
pub struct BinaryIndexedTree<A: AbelianGroup> {
    n: usize,
    bit: Vec<A::M>,
}

impl<A: AbelianGroup> BinaryIndexedTree<A> {
    pub fn new(n: usize) -> Self {
        let n = n + 1;
        let bit = vec![A::unit(); n + 1];
        BinaryIndexedTree { n, bit }
    }

    /// add x to i
    pub fn add(&mut self, i: usize, x: A::M) {
        let mut idx = i as i32 + 1;
        while idx <= self.n as i32 {
            self.bit[idx as usize] = A::op(&self.bit[idx as usize], &x);
            idx += idx & -idx;
        }
    }

    /// sum of [0, i)
    pub fn sum(&self, i: usize) -> A::M {
        let mut ret = A::unit();
        let mut idx = i as i32 + 1;
        while idx > 0 {
            ret = A::op(&ret, &self.bit[idx as usize]);
            idx -= idx & -idx;
        }
        ret
    }

    /// sum of [a, b)
    pub fn sum_ab(&self, a: usize, b: usize) -> A::M {
        if b == 0 {
            A::unit()
        } else if a == 0 {
            self.sum(b - 1)
        } else {
            A::op(&self.sum(b - 1), &A::inv(&self.sum(a - 1)))
        }
    }
}

impl<A: AbelianGroup> Debug for BinaryIndexedTree<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = (0..self.n)
            .map(|i| format!("{:?}", self.sum_ab(i, i)))
            .collect::<Vec<_>>()
            .join(" ");
        let v2 = (0..self.n)
            .map(|i| format!("{:?}", self.sum(i)))
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "\n{}\n{}", v, v2)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::binary_operation::addition::Addition;
    use rand::Rng;

    #[test]
    fn test() {
        const LEN: usize = 1000;
        let mut v = vec![0; LEN];
        let mut bit = BinaryIndexedTree::<Addition<i64>>::new(LEN);

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
        let mut bit = BinaryIndexedTree::<Addition<i64>>::new(LEN);

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
