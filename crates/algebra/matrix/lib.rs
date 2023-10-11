//! # 行列
//!
//!
use algebra::*;
use prelude::*;

#[snippet(name = "matrix", doc_hidden)]
pub use matrix_impl::{ColumnVector, Determinant, IdentityMatrix, Matrix, RowVector, ZeroMatrix};
#[snippet(name = "matrix", doc_hidden)]
mod matrix_impl {
    use super::{
        Add, AddAssign, Debug, Display, Div, Formatter, Mul, MulAssign, Neg, One, Pow, Sub,
        SubAssign, Zero,
    };

    #[derive(Clone, Eq, PartialEq)]
    pub struct Matrix<T>(Vec<Vec<T>>);
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
        /// # 行列のサイズ
        /// (rows, columns)
        pub fn size(&self) -> (usize, usize) {
            if self.0.is_empty() {
                (0, 0)
            } else {
                (self.0.len(), self.0[0].len())
            }
        }
    }

    /// # 零行列
    pub trait ZeroMatrix {
        fn zero_matrix(cols: usize, rows: usize) -> Self;
    }
    impl<T: Clone + Zero> ZeroMatrix for Matrix<T> {
        fn zero_matrix(cols: usize, rows: usize) -> Self {
            Self(vec![vec![T::zero(); cols]; rows])
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
                + PartialEq
                + Neg<Output = T>,
        > Determinant<T> for Matrix<T>
    {
        fn determinant(&self) -> Option<T> {
            let (rows, cols) = self.size();
            if rows != cols {
                return None;
            }
            if rows == 0 {
                return Some(T::zero());
            }

            let mut res = T::one();
            let mut buf = self.0.clone();
            for i in 0..rows {
                match (i..rows).find(|&ni| buf[ni][i] != T::zero()) {
                    Some(ni) => {
                        if i != ni {
                            buf.swap(i, ni);
                            res = -res
                        }
                    }
                    None => return Some(T::zero()),
                }
                res *= buf[i][i].clone();
                let diag = T::one() / buf[i][i].clone();
                (i..rows).for_each(|j| buf[i][j] *= diag.clone());
                for ni in (0..rows).filter(|&ni| ni != i) {
                    let c = buf[ni][i].clone();
                    for j in i..rows {
                        let d = c.clone() * buf[i][j].clone();
                        buf[ni][j] -= d;
                    }
                }
            }

            Some(res)
        }
    }

    impl<T: Clone + Zero + One + Mul<Output = T> + Add<Output = T>> Pow for Matrix<T> {
        fn pow(mut self, mut e: i64) -> Self {
            let (n, m) = self.size();
            assert_eq!(n, m);
            let mut result = Self::identity_matrix(n);
            while e > 0 {
                if e & 1 == 1 {
                    result = (result * self.clone()).unwrap();
                }
                e >>= 1;
                self = (self.clone() * self).unwrap();
            }
            result
        }
    }

    impl<T: Default + Clone + Zero + One> Matrix<T> {
        /// y行目、x列目を除いた 余因子行列を計算する
        /// x, y は 0-indexed
        pub fn sub_matrix(&self, x: usize, y: usize) -> Self {
            let (rows, cols) = self.size();
            let mut buf = vec![vec![T::default(); cols - 1]; rows - 1];
            for yi in (0..rows).filter(|&yi| yi != y) {
                for xi in (0..cols).filter(|&xi| xi != x) {
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
            assert_eq!(self.size(), rhs.size());
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
            for r in 0..self.size().0 {
                for c in 0..self.size().1 {
                    self.0[r][c] = -self.0[r][c].clone()
                }
            }
            self
        }
    }

    impl<T: Neg<Output = T> + AddAssign + Clone> Sub<Matrix<T>> for Matrix<T> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            assert_eq!(self.size(), rhs.size());
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

    impl<T: Mul<Output = T> + Add<Output = T> + Zero + Clone> Mul<Matrix<T>> for Matrix<T> {
        type Output = Option<Self>;
        fn mul(self, rhs: Self) -> Option<Self> {
            let ((self_row, self_col), (rhs_row, rhs_col)) = (self.size(), rhs.size());
            if self_col != rhs_row {
                return None;
            }
            let mut ret = Self::zero_matrix(rhs_col, self_row);
            ret.0.iter_mut().enumerate().for_each(|(i, bufi)| {
                bufi.iter_mut().enumerate().for_each(|(j, bufij)| {
                    *bufij = (0..self_col)
                        .map(|k| self.0[i][k].clone() * rhs.0[k][j].clone())
                        .fold(T::zero(), |x, a| x.add(a));
                });
            });
            Some(ret)
        }
    }

    impl<T: ToString> Display for Matrix<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(self, f)
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
    use mod_int::mod998244353::Mi;
    use std::convert::TryInto;

    #[test]
    fn test() {
        let data = vec![vec![Mi::new(3), Mi::new(2)], vec![Mi::new(5), Mi::new(4)]];
        let matrix: Matrix<Mi> = data.try_into().unwrap();
        assert_eq!(matrix.determinant(), Some(Mi::new(2)));

        let data = vec![
            vec![Mi::new(0), Mi::new(1), Mi::new(2), Mi::new(3)],
            vec![Mi::new(4), Mi::new(5), Mi::new(6), Mi::new(7)],
            vec![Mi::new(8), Mi::new(9), Mi::new(10), Mi::new(11)],
            vec![Mi::new(12), Mi::new(13), Mi::new(14), Mi::new(15)],
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
        assert_eq!(sub_matrix.determinant(), Some(Mi::new(0)));

        let lhs: Matrix<Mi> = Matrix::row_vector(&vec![Mi::new(1), Mi::new(2), Mi::new(3)]);
        let rhs: Matrix<Mi> = Matrix::column_vector(&vec![Mi::new(4), Mi::new(5), Mi::new(6)]);
        let expect: Matrix<Mi> = vec![vec![Mi::new(32)]].try_into().unwrap();
        assert_eq!(lhs * rhs, Some(expect));
    }
}
