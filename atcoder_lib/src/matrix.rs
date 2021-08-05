#[allow(unused_imports)]
use matrix::*;

#[allow(dead_code)]
mod matrix {
    use super::*;
    use itertools::Itertools;
    use std::convert::TryInto;
    use std::fmt;
    use std::ops;

    #[derive(Clone, Eq, PartialEq)]
    pub struct Matrix {
        pub buf: Vec<Vec<Mi>>,
    }

    impl std::convert::TryFrom<Vec<Vec<Mi>>> for Matrix {
        type Error = &'static str;
        fn try_from(buf: Vec<Vec<Mi>>) -> std::result::Result<Self, Self::Error> {
            if (1..buf.len()).any(|i| buf[0].len() != buf[i].len()) {
                Err("size is invalid")
            } else {
                Ok(Self { buf })
            }
        }
    }

    impl Matrix {
        fn identity_matrix(n: usize) -> Self {
            let mut buf = vec![vec![Mi::new(0); n]; n];
            for i in 0..n {
                buf[i][i] += 1;
            }
            Matrix { buf }
        }

        /// (y, x)
        fn size(&self) -> (usize, usize) {
            if self.buf.len() == 0 {
                (0, 0)
            } else {
                (self.buf.len(), self.buf[0].len())
            }
        }

        /// y行目、x列目を除いた 余因子行列を計算する
        /// x, y は 0-indexed
        pub fn sub_matrix(&self, x: usize, y: usize) -> Self {
            let (n, m) = self.size();
            let mut sub = vec![vec![Mi::new(0); m - 1]; n - 1];
            for yi in (0..n).filter(|&yi| yi != y) {
                for xi in (0..m).filter(|&xi| xi != x) {
                    sub[yi - if yi < y { 0 } else { 1 }][xi - if xi < x { 0 } else { 1 }] =
                        self.buf[yi][xi];
                }
            }
            Matrix { buf: sub }
        }

        /// 行列式detを計算する
        /// 平方行列でない場合はNoneを返す
        /// 計算量は O(size^3)
        pub fn determinant(&self) -> Option<Mi> {
            let (n, m) = self.size();
            let zero = Mi::new(0);
            if n != m {
                return None;
            }
            if n == 0 {
                return Some(zero);
            }

            let mut res = Mi::new(1);
            let mut buf = self.buf.clone();
            for i in 0..n {
                match (i..n).find(|&ni| buf[ni][i] != zero) {
                    Some(ni) => buf.swap(i, ni),
                    None => return Some(zero),
                }
                res *= buf[i][i];
                let diag = Mi::new(1) / buf[i][i];
                (i..n).for_each(|j| buf[i][j] *= diag);
                for ni in (0..n).filter(|&ni| ni != i) {
                    let c = buf[ni][i];
                    for j in i..n {
                        let d = c * buf[i][j];
                        buf[ni][j] -= d;
                    }
                }
            }

            Some(res)
        }

        pub fn pow(mut self, mut e: i64) -> Option<Self> {
            let (n, m) = self.size();
            if n != m {
                return None;
            }
            let mut result = Self::identity_matrix(n);
            while e > 0 {
                if e & 1 == 1 {
                    result = (result * self.clone()).unwrap();
                }
                e >>= 1;
                self = (self.clone() * self).unwrap();
            }
            Some(result)
        }
    }

    impl ops::Add<Matrix> for Matrix {
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self {
            for i in 0..self.buf.len() {
                for j in 0..self.buf[0].len() {
                    self.buf[i][j] += rhs.buf[i][j]
                }
            }
            self
        }
    }

    impl ops::Neg for Matrix {
        type Output = Self;
        fn neg(mut self) -> Self {
            for i in 0..self.buf.len() {
                for j in 0..self.buf[0].len() {
                    self.buf[i][j] = -self.buf[i][j]
                }
            }
            self
        }
    }

    impl ops::Sub<Matrix> for Matrix {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            self + (-rhs)
        }
    }

    impl ops::Mul<i64> for Matrix {
        type Output = Self;
        fn mul(mut self, rhs: i64) -> Self {
            let (n, m) = self.size();
            for i in 0..n {
                for j in 0..m {
                    self.buf[i][j] *= rhs;
                }
            }
            self
        }
    }

    impl ops::Mul<Matrix> for Matrix {
        type Output = Option<Matrix>;
        fn mul(self, rhs: Matrix) -> Option<Matrix> {
            let ((self_y, self_x), (rhs_y, rhs_x)) = (self.size(), rhs.size());
            if self_x != rhs_y {
                return None;
            }
            let mut buf = vec![vec![Mi::new(0); rhs_x]; self_y];
            for i in 0..self_y {
                for j in 0..rhs_x {
                    for k in 0..self_x {
                        buf[i][j] += self.buf[i][k] * rhs.buf[k][j];
                    }
                }
            }
            Some(buf.try_into().unwrap())
        }
    }

    impl fmt::Debug for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                self.buf.iter().map(|row| row.iter().join(" ")).join("\n")
            )
        }
    }
}
////////////////////////////////////////////////////////

use crate::mod_int::mod_int::*;

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test() {
        let data = vec![vec![Mi::new(3), Mi::new(2)], vec![Mi::new(5), Mi::new(4)]];
        let matrix: Matrix = data.try_into().unwrap();
        assert_eq!(matrix.determinant(), Some(Mi::new(2)));

        let data = vec![
            vec![Mi::new(0), Mi::new(1), Mi::new(2), Mi::new(3)],
            vec![Mi::new(4), Mi::new(5), Mi::new(6), Mi::new(7)],
            vec![Mi::new(8), Mi::new(9), Mi::new(10), Mi::new(11)],
            vec![Mi::new(12), Mi::new(13), Mi::new(14), Mi::new(15)],
        ];
        let matrix: Matrix = data.try_into().unwrap();
        let sub_matrix = matrix.sub_matrix(2, 3);
        let expect_sub_matrix: Matrix = vec![
            vec![Mi::new(0), Mi::new(1), Mi::new(3)],
            vec![Mi::new(4), Mi::new(5), Mi::new(7)],
            vec![Mi::new(8), Mi::new(9), Mi::new(11)],
        ]
        .try_into()
        .unwrap();
        assert_eq!(sub_matrix, expect_sub_matrix);
        assert_eq!(sub_matrix.determinant(), Some(Mi::new(0)));

        let lhs: Matrix = vec![vec![Mi::new(1), Mi::new(2), Mi::new(3)]]
            .try_into()
            .unwrap();
        let rhs: Matrix = vec![vec![Mi::new(4)], vec![Mi::new(5)], vec![Mi::new(6)]]
            .try_into()
            .unwrap();
        let expect: Matrix = vec![vec![Mi::new(32)]].try_into().unwrap();
        assert_eq!(lhs * rhs, Some(expect));
    }
}
