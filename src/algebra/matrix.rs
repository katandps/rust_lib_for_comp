//! # 行列
//!
//! ## dependency
//! [mod-int](Mi)
//!
use crate::algebra::mod_int::mod998244353::Mi;
use crate::prelude::*;

#[snippet(name = "matrix", doc_hidden)]
#[derive(Clone, Eq, PartialEq)]
pub struct Matrix {
    pub buf: Vec<Vec<Mi>>,
}

#[snippet(name = "matrix", doc_hidden)]
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

#[snippet(name = "matrix", doc_hidden)]
impl Matrix {
    /// N行N列の単位行列を生成する
    pub fn identity_matrix(n: usize) -> Self {
        let mut buf = vec![vec![Mi::new(0); n]; n];

        buf.iter_mut()
            .enumerate()
            .for_each(|(i, bufi)| bufi[i] += 1);
        Matrix { buf }
    }

    /// vをもとに行行列を生成する
    pub fn row_vector(v: &[Mi]) -> Self {
        Matrix {
            buf: vec![v.to_vec()],
        }
    }

    /// vをもとに列行列を生成する
    pub fn column_vector(v: &[Mi]) -> Self {
        Matrix {
            buf: v.iter().map(|mi| vec![*mi]).collect(),
        }
    }

    /// (y, x)
    fn size(&self) -> (usize, usize) {
        if self.buf.is_empty() {
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

#[snippet(name = "matrix", doc_hidden)]
impl Add<Matrix> for Matrix {
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

#[snippet(name = "matrix", doc_hidden)]
impl Neg for Matrix {
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

#[snippet(name = "matrix", doc_hidden)]
impl Sub<Matrix> for Matrix {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

#[snippet(name = "matrix", doc_hidden)]
impl Mul<i64> for Matrix {
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

#[snippet(name = "matrix", doc_hidden)]
impl Mul<Matrix> for Matrix {
    type Output = Option<Matrix>;
    fn mul(self, rhs: Matrix) -> Option<Matrix> {
        let ((self_y, self_x), (rhs_y, rhs_x)) = (self.size(), rhs.size());
        if self_x != rhs_y {
            return None;
        }
        let mut buf = vec![vec![Mi::new(0); rhs_x]; self_y];
        buf.iter_mut().enumerate().for_each(|(i, bufi)| {
            bufi.iter_mut().enumerate().for_each(|(j, bufij)| {
                *bufij = (0..self_x).map(|k| self.buf[i][k] * rhs.buf[k][j]).sum();
            });
        });
        Some(buf.try_into().unwrap())
    }
}

#[snippet(name = "matrix", doc_hidden)]
impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.buf
                .iter()
                .map(|row| row
                    .iter()
                    .map(|mi| mi.to_string())
                    .collect::<Vec<_>>()
                    .join(" "))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::mod_int::mod998244353::mi;
    use std::convert::TryInto;

    #[test]
    fn test() {
        let data = vec![vec![mi(3), mi(2)], vec![mi(5), mi(4)]];
        let matrix: Matrix = data.try_into().unwrap();
        assert_eq!(matrix.determinant(), Some(mi(2)));

        let data = vec![
            vec![mi(0), mi(1), mi(2), mi(3)],
            vec![mi(4), mi(5), mi(6), mi(7)],
            vec![mi(8), mi(9), mi(10), mi(11)],
            vec![mi(12), mi(13), mi(14), mi(15)],
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
        assert_eq!(sub_matrix.determinant(), Some(mi(0)));

        let lhs: Matrix = Matrix::row_vector(&vec![mi(1), mi(2), mi(3)]);
        let rhs: Matrix = Matrix::column_vector(&vec![mi(4), mi(5), mi(6)]);
        let expect: Matrix = vec![vec![mi(32)]].try_into().unwrap();
        assert_eq!(lhs * rhs, Some(expect));
    }
}
