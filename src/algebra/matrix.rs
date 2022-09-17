//! # 行列
//!
//!
use crate::prelude::*;

#[snippet(name = "matrix", doc_hidden)]
#[derive(Clone, Eq, PartialEq)]
pub struct Matrix<T>(Vec<Vec<T>>);

#[snippet(name = "matrix", doc_hidden)]
mod matrix_impl {
    use super::{
        Add, AddAssign, Debug, Div, Formatter, Matrix, Mul, MulAssign, Neg, One, Sub, SubAssign,
        Sum, Zero,
    };
    impl<T> std::convert::TryFrom<Vec<Vec<T>>> for Matrix<T> {
        type Error = &'static str;
        fn try_from(buf: Vec<Vec<T>>) -> std::result::Result<Self, Self::Error> {
            if (1..buf.len()).any(|i| buf[0].len() != buf[i].len()) {
                Err("size is invalid")
            } else {
                Ok(Self(buf))
            }
        }
    }

    impl<T> Matrix<T> {
        /// (y, x)
        fn size(&self) -> (usize, usize) {
            if self.0.is_empty() {
                (0, 0)
            } else {
                (self.0.len(), self.0[0].len())
            }
        }
    }

    /// # 零行列
    pub trait ZeroMatrix {
        fn zero_matrix(x: usize, y: usize) -> Self;
    }
    impl<T: Clone + Zero> ZeroMatrix for Matrix<T> {
        fn zero_matrix(x: usize, y: usize) -> Self {
            Self(vec![vec![T::zero(); x]; y])
        }
    }

    /// # 単位行列
    /// N行N列の単位行列を生成する
    pub trait IdentityMatrix {
        fn identity_matrix(n: usize) -> Self;
    }
    impl<T: Clone + Zero + One> IdentityMatrix for Matrix<T> {
        fn identity_matrix(n: usize) -> Self {
            let mut ret = Self::zero_matrix(n, n);
            (0..n).for_each(|i| ret.0[i][i] = T::one());
            ret
        }
    }

    /// # 行行列
    pub trait RowVector<T> {
        fn row_vector(v: &[T]) -> Self;
    }
    impl<T: Clone> RowVector<T> for Matrix<T> {
        /// vをもとに行行列を生成する
        fn row_vector(v: &[T]) -> Self {
            Self(vec![v.to_vec()])
        }
    }
    /// # 列行列
    pub trait ColumnVector<T> {
        fn column_vector(v: &[T]) -> Self;
    }
    impl<T: Clone> ColumnVector<T> for Matrix<T> {
        fn column_vector(v: &[T]) -> Self {
            Self(v.iter().map(|cell| vec![cell.clone()]).collect())
        }
    }

    /// # 行列式
    /// 平方行列でない場合はNoneを返す
    /// 計算量は $O(size^3)$
    pub trait Determinant<T> {
        fn determinant(&self) -> Option<T>;
    }
    impl<
            T: Clone
                + Zero
                + One
                + MulAssign
                + Mul<Output = T>
                + SubAssign
                + Div<Output = T>
                + PartialEq,
        > Determinant<T> for Matrix<T>
    {
        fn determinant(&self) -> Option<T> {
            let (n, m) = self.size();
            if n != m {
                return None;
            }
            if n == 0 {
                return Some(T::zero());
            }

            let zero = T::zero();
            let mut res = T::one();
            let mut buf = self.0.clone();
            for i in 0..n {
                match (i..n).find(|&ni| buf[ni][i] != zero) {
                    Some(ni) => buf.swap(i, ni),
                    None => return Some(zero),
                }
                res *= buf[i][i].clone();
                let diag = T::one() / buf[i][i].clone();
                (i..n).for_each(|j| buf[i][j] *= diag.clone());
                for ni in (0..n).filter(|&ni| ni != i) {
                    let c = buf[ni][i].clone();
                    for j in i..n {
                        let d = c.clone() * buf[i][j].clone();
                        buf[ni][j] -= d;
                    }
                }
            }

            Some(res)
        }
    }

    pub trait Pow {
        fn pow(self, e: i64) -> Option<Self>
        where
            Self: Sized;
    }
    impl<T: Clone + Zero + One + Mul<Output = T> + Sum> Pow for Matrix<T> {
        fn pow(mut self, mut e: i64) -> Option<Self> {
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

    impl<T: Default + Clone + Zero + One> Matrix<T> {
        /// y行目、x列目を除いた 余因子行列を計算する
        /// x, y は 0-indexed
        pub fn sub_matrix(&self, x: usize, y: usize) -> Self {
            let (n, m) = self.size();
            let mut buf = vec![vec![T::default(); m - 1]; n - 1];
            for yi in (0..n).filter(|&yi| yi != y) {
                for xi in (0..m).filter(|&xi| xi != x) {
                    buf[yi - if yi < y { 0 } else { 1 }][xi - if xi < x { 0 } else { 1 }] =
                        self.0[yi][xi].clone();
                }
            }
            Matrix(buf)
        }
    }

    impl<T: AddAssign + Clone> Add<Matrix<T>> for Matrix<T> {
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self {
            for i in 0..self.0.len() {
                for j in 0..self.0[0].len() {
                    self.0[i][j] += rhs.0[i][j].clone()
                }
            }
            self
        }
    }
    impl<T: AddAssign + Clone> AddAssign<T> for Matrix<T> {
        fn add_assign(&mut self, rhs: T) {
            self.0
                .iter_mut()
                .for_each(|row| row.iter_mut().for_each(|cell| *cell += rhs.clone()))
        }
    }

    impl<T: Neg<Output = T> + Clone> Neg for Matrix<T> {
        type Output = Self;
        fn neg(mut self) -> Self {
            for i in 0..self.0.len() {
                for j in 0..self.0[0].len() {
                    self.0[i][j] = -self.0[i][j].clone()
                }
            }
            self
        }
    }

    impl<T: Neg<Output = T> + AddAssign + Clone> Sub<Matrix<T>> for Matrix<T> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            self + (-rhs)
        }
    }

    impl<T: MulAssign<i64>> MulAssign<i64> for Matrix<T> {
        fn mul_assign(&mut self, rhs: i64) {
            self.0
                .iter_mut()
                .for_each(|row| row.iter_mut().for_each(|cell| *cell *= rhs))
        }
    }
    impl<T: MulAssign<i64>> Mul<i64> for Matrix<T> {
        type Output = Self;
        fn mul(mut self, rhs: i64) -> Self {
            self *= rhs;
            self
        }
    }

    impl<T: Mul<Output = T> + Sum<T> + Zero + Clone> Mul<Matrix<T>> for Matrix<T> {
        type Output = Option<Self>;
        fn mul(self, rhs: Self) -> Option<Self> {
            let ((self_y, self_x), (rhs_y, rhs_x)) = (self.size(), rhs.size());
            if self_x != rhs_y {
                return None;
            }
            let mut ret = Self::zero_matrix(rhs_x, self_y);
            ret.0.iter_mut().enumerate().for_each(|(i, bufi)| {
                bufi.iter_mut().enumerate().for_each(|(j, bufij)| {
                    *bufij = (0..self_x)
                        .map(|k| self.0[i][k].clone() * rhs.0[k][j].clone())
                        .sum();
                });
            });
            Some(ret)
        }
    }

    impl<T: ToString> Debug for Matrix<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
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
}

#[cfg(test)]
mod test {
    use super::matrix_impl::{ColumnVector, Determinant, RowVector};
    use super::*;
    use crate::algebra::mod_int::mod998244353::{mi, Mi};
    use std::convert::TryInto;

    #[test]
    fn test() {
        let data = vec![vec![mi(3), mi(2)], vec![mi(5), mi(4)]];
        let matrix: Matrix<Mi> = data.try_into().unwrap();
        assert_eq!(matrix.determinant(), Some(mi(2)));

        let data = vec![
            vec![mi(0), mi(1), mi(2), mi(3)],
            vec![mi(4), mi(5), mi(6), mi(7)],
            vec![mi(8), mi(9), mi(10), mi(11)],
            vec![mi(12), mi(13), mi(14), mi(15)],
        ];
        let matrix: Matrix<Mi> = data.try_into().unwrap();
        let sub_matrix = matrix.sub_matrix(2, 3);
        let expect_sub_matrix: Matrix<Mi> = vec![
            vec![Mi::new(0), Mi::new(1), Mi::new(3)],
            vec![Mi::new(4), Mi::new(5), Mi::new(7)],
            vec![Mi::new(8), Mi::new(9), Mi::new(11)],
        ]
        .try_into()
        .unwrap();
        assert_eq!(sub_matrix, expect_sub_matrix);
        assert_eq!(sub_matrix.determinant(), Some(mi(0)));

        let lhs: Matrix<Mi> = Matrix::row_vector(&vec![mi(1), mi(2), mi(3)]);
        let rhs: Matrix<Mi> = Matrix::column_vector(&vec![mi(4), mi(5), mi(6)]);
        let expect: Matrix<Mi> = vec![vec![mi(32)]].try_into().unwrap();
        assert_eq!(lhs * rhs, Some(expect));
    }
}
