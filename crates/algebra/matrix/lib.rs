//! # 行列
//!
//!
use algebra::*;
use prelude::*;

#[snippet(name = "matrix", doc_hidden)]
pub use matrix_impl::{ColumnVector, Determinant, Matrix, RowVector};
#[snippet(name = "matrix", doc_hidden)]
mod matrix_impl {
    use super::{
        Add, AddAssign, Debug, Display, Div, Formatter, Mul, MulAssign, Neg, One, Pow, Sub,
        SubAssign, Zero,
    };

    #[derive(Clone, Eq, PartialEq)]
    pub struct Matrix<T> {
        src: Vec<Vec<T>>,
        height: usize,
        width: usize,
    }
    /// # 参照で構成された行列
    /// 内容は不変
    pub struct PointerMatrix<'a, T> {
        src: &'a [Vec<T>],
        height: usize,
        width: usize,
    }

    impl<'a, T> PointerMatrix<'a, T> {
        pub fn new(src: &'a [Vec<T>], height: usize, width: usize) -> Self {
            Self { src, height, width }
        }
    }

    impl<T> std::convert::TryFrom<Vec<Vec<T>>> for Matrix<T> {
        type Error = &'static str;
        fn try_from(buf: Vec<Vec<T>>) -> std::result::Result<Self, Self::Error> {
            if (1..buf.len()).any(|i| buf[0].len() != buf[i].len()) {
                Err("size is invalid")
            } else {
                let (height, width) = (buf.len(), buf[0].len());
                Ok(Self {
                    src: buf,
                    height,
                    width,
                })
            }
        }
    }

    impl<T> Matrix<T> {
        pub fn pointer(&self) -> PointerMatrix<'_, T> {
            PointerMatrix::new(&self.src, self.height, self.width)
        }
    }

    impl<T: Zero + Clone> Matrix<T> {
        /// # 零行列
        pub fn zero_matrix(cols: usize, rows: usize) -> Matrix<T> {
            Matrix {
                src: vec![vec![T::zero(); cols]; rows],
                height: rows,
                width: cols,
            }
        }
    }

    impl<T: Zero + One + Clone> Matrix<T> {
        /// # 単位行列
        pub fn identity_matrix(n: usize) -> Self {
            let mut ret = Self::zero_matrix(n, n);
            (0..n).for_each(|i| ret.src[i][i] = T::one());
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
            Matrix {
                src: vec![v.to_vec()],
                height: 1,
                width: v.len(),
            }
        }
    }
    /// # 列行列
    pub trait ColumnVector<T> {
        fn column_vector(v: &[T]) -> Self;
    }
    impl<T: Clone> ColumnVector<T> for Matrix<T> {
        fn column_vector(v: &[T]) -> Self {
            Matrix {
                src: v.iter().map(|cell| vec![cell.clone()]).collect(),
                height: v.len(),
                width: 1,
            }
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
            if self.height != self.width {
                return None;
            }
            if self.height == 0 {
                return Some(T::zero());
            }

            let mut res = T::one();
            let mut buf = self.src.clone();
            for i in 0..self.height {
                match (i..self.height).find(|&ni| buf[ni][i] != T::zero()) {
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
                (i..self.height).for_each(|j| buf[i][j] *= diag.clone());
                for ni in (0..self.height).filter(|&ni| ni != i) {
                    let c = buf[ni][i].clone();
                    for j in i..self.height {
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
            assert_eq!(self.height, self.width);
            let mut result = Self::identity_matrix(self.height);
            while e > 0 {
                if e & 1 == 1 {
                    result = (result * self.pointer()).unwrap();
                }
                e >>= 1;
                self = (self.pointer() * self.pointer()).unwrap();
            }
            result
        }
    }

    impl<T: Default + Clone + Zero + One> Matrix<T> {
        /// y行目、x列目を除いた 余因子行列を計算する
        /// x, y は 0-indexed
        pub fn sub_matrix(&self, x: usize, y: usize) -> Self {
            let mut buf = vec![vec![T::default(); self.width - 1]; self.height - 1];
            for yi in (0..self.width).filter(|&yi| yi != y) {
                for xi in (0..self.height).filter(|&xi| xi != x) {
                    buf[yi - if yi < y { 0 } else { 1 }][xi - if xi < x { 0 } else { 1 }] =
                        self.src[yi][xi].clone();
                }
            }
            Matrix {
                src: buf,
                height: self.height - 1,
                width: self.width - 1,
            }
        }
    }

    impl<T: AddAssign + Clone> Add<Matrix<T>> for Matrix<T> {
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self {
            assert_eq!(self.height, rhs.width);
            for y in 0..self.height {
                for x in 0..self.width {
                    self.src[y][x] += rhs.src[y][x].clone()
                }
            }
            self
        }
    }
    impl<T: AddAssign + Clone> AddAssign<T> for Matrix<T> {
        fn add_assign(&mut self, rhs: T) {
            self.src
                .iter_mut()
                .for_each(|row| row.iter_mut().for_each(|cell| *cell += rhs.clone()))
        }
    }

    impl<T: Neg<Output = T> + Clone> Neg for Matrix<T> {
        type Output = Self;
        fn neg(mut self) -> Self {
            for r in 0..self.height {
                for c in 0..self.width {
                    self.src[r][c] = -self.src[r][c].clone()
                }
            }
            self
        }
    }

    impl<T: Neg<Output = T> + AddAssign + Clone> Sub<Matrix<T>> for Matrix<T> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            assert_eq!(self.height, rhs.width);
            self + (-rhs)
        }
    }

    impl<T: MulAssign<i64>> MulAssign<i64> for Matrix<T> {
        fn mul_assign(&mut self, rhs: i64) {
            self.src
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

    impl<'a, T: Mul<Output = T> + Add<Output = T> + Zero + Clone> PointerMatrix<'a, T> {
        /// # 愚直積
        ///
        /// ## 計算量
        /// $O(N^3)$
        fn naive_mul(self, rhs: Self) -> Option<Matrix<T>> {
            let (self_row, self_col, rhs_row, rhs_col) =
                (self.height, self.width, rhs.height, rhs.width);
            if self_col != rhs_row {
                return None;
            }
            let mut ret = Matrix::zero_matrix(rhs_col, self_row);
            ret.src.iter_mut().enumerate().for_each(|(i, bufi)| {
                bufi.iter_mut().enumerate().for_each(|(j, bufij)| {
                    *bufij = (0..self_col)
                        .map(|k| self.src[i][k].clone() * rhs.src[k][j].clone())
                        .fold(T::zero(), |x, a| x.add(a));
                });
            });
            Some(ret)
        }

        /// # シュトラッセンのアルゴリズム
        ///
        /// ## see
        /// https://en.wikipedia.org/wiki/Strassen_algorithm
        #[allow(unused)]
        fn strassen_mul(self, rhs: Self) -> Option<Matrix<T>> {
            let (self_row, self_col, rhs_row, rhs_col) =
                (self.height, self.width, rhs.height, rhs.width);
            if self_row != self_col || rhs_row != rhs_col {
                return self.naive_mul(rhs);
            }
            if self_col != rhs_row {
                return None;
            }
            unimplemented!()
        }
    }

    impl<T: Mul<Output = T> + Add<Output = T> + Zero + Clone> Mul<PointerMatrix<'_, T>>
        for PointerMatrix<'_, T>
    {
        type Output = Option<Matrix<T>>;
        fn mul(self, rhs: PointerMatrix<'_, T>) -> Self::Output {
            self.naive_mul(rhs)
        }
    }

    impl<T: Mul<Output = T> + Add<Output = T> + Zero + Clone> Mul<PointerMatrix<'_, T>> for Matrix<T> {
        type Output = Option<Matrix<T>>;
        fn mul(self, rhs: PointerMatrix<'_, T>) -> Self::Output {
            self.pointer() * rhs
        }
    }

    impl<T: Mul<Output = T> + Add<Output = T> + Zero + Clone> Mul<Matrix<T>> for Matrix<T> {
        type Output = Option<Matrix<T>>;
        fn mul(self, rhs: Self) -> Self::Output {
            self.pointer() * rhs.pointer()
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
                self.src
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
    use mod_int::ModInt;
    use std::convert::TryInto;

    #[test]
    fn test() {
        let data = vec![
            vec![ModInt::new(3), ModInt::new(2)],
            vec![ModInt::new(5), ModInt::new(4)],
        ];
        let matrix: Matrix<ModInt> = data.try_into().unwrap();
        assert_eq!(matrix.determinant(), Some(ModInt::new(2)));

        let data = vec![
            vec![
                ModInt::new(0),
                ModInt::new(1),
                ModInt::new(2),
                ModInt::new(3),
            ],
            vec![
                ModInt::new(4),
                ModInt::new(5),
                ModInt::new(6),
                ModInt::new(7),
            ],
            vec![
                ModInt::new(8),
                ModInt::new(9),
                ModInt::new(10),
                ModInt::new(11),
            ],
            vec![
                ModInt::new(12),
                ModInt::new(13),
                ModInt::new(14),
                ModInt::new(15),
            ],
        ];
        let matrix: Matrix<ModInt> = data.try_into().unwrap();
        let sub_matrix = matrix.sub_matrix(2, 3);
        let expect_sub_matrix: Matrix<ModInt> = vec![
            vec![ModInt::new(0), ModInt::new(1), ModInt::new(3)],
            vec![ModInt::new(4), ModInt::new(5), ModInt::new(7)],
            vec![ModInt::new(8), ModInt::new(9), ModInt::new(11)],
        ]
        .try_into()
        .unwrap();
        assert_eq!(sub_matrix, expect_sub_matrix);
        assert_eq!(sub_matrix.determinant(), Some(ModInt::new(0)));

        let lhs: Matrix<ModInt> =
            Matrix::row_vector(&vec![ModInt::new(1), ModInt::new(2), ModInt::new(3)]);
        let rhs: Matrix<ModInt> =
            Matrix::column_vector(&vec![ModInt::new(4), ModInt::new(5), ModInt::new(6)]);
        let expect: Matrix<ModInt> = vec![vec![ModInt::new(32)]].try_into().unwrap();
        assert_eq!(lhs * rhs, Some(expect));
    }
}
