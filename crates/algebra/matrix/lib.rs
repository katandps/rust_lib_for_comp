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
        max, Add, AddAssign, Debug, Display, Div, Formatter, Mul, MulAssign, Neg, One, Pow, Sub,
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
    #[derive(Clone, Copy)]
    pub struct PointerMatrix<'a, T> {
        src: &'a [Vec<T>],
        y_offset: usize,
        x_offset: usize,
        height: usize,
        width: usize,
    }

    impl<'a, T> PointerMatrix<'a, T> {
        pub fn new(src: &'a [Vec<T>], height: usize, width: usize) -> Self {
            Self {
                src,
                y_offset: 0,
                x_offset: 0,
                height,
                width,
            }
        }
    }

    impl<'a, T: PartialEq> PartialEq for PointerMatrix<'a, T> {
        fn eq(&self, other: &Self) -> bool {
            if self.width != other.width || self.height != other.height {
                return false;
            }
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.src[y + self.y_offset][x + self.x_offset]
                        != other.src[y + other.y_offset][x + other.x_offset]
                    {
                        return false;
                    }
                }
            }
            true
        }
    }

    impl<T> Matrix<T> {
        pub fn build(src: Vec<Vec<T>>) -> Option<Matrix<T>> {
            if (1..src.len()).any(|i| src[0].len() != src[i].len()) {
                None
            } else {
                let (height, width) = (src.len(), src[0].len());
                Some(Self { src, height, width })
            }
        }

        #[inline]
        pub fn pointer(&self) -> PointerMatrix<'_, T> {
            PointerMatrix::new(&self.src, self.height, self.width)
        }
    }

    impl<'a, T: Clone + Copy + Zero> PointerMatrix<'a, T> {
        #[inline]
        pub fn get(&self, y: usize, x: usize) -> T {
            if self.height <= y || self.width <= x {
                return T::zero();
            }
            *self
                .src
                .get(y + self.y_offset)
                .and_then(|v| v.get(x + self.x_offset))
                .unwrap_or(&T::zero())
        }
        /// # 列方向分割
        /// 行列を0..xとx..widthに分ける
        pub fn column_divide(&self, x: usize) -> (Self, Self) {
            let left = Self {
                src: self.src,
                height: self.height,
                y_offset: self.y_offset,
                width: x,
                x_offset: self.x_offset,
            };
            let right = Self {
                src: self.src,
                height: self.height,
                y_offset: self.y_offset,
                width: self.width - x,
                x_offset: self.x_offset + x,
            };
            (left, right)
        }
        /// # 行方向分割
        /// 行列を0..yとy..heightに分ける
        pub fn row_divide(&self, y: usize) -> (Self, Self) {
            let upper = Self {
                src: self.src,
                height: y,
                y_offset: self.y_offset,
                width: self.width,
                x_offset: self.x_offset,
            };
            let lower = Self {
                src: self.src,
                height: self.height - y,
                y_offset: self.y_offset + y,
                width: self.width,
                x_offset: self.x_offset,
            };
            (upper, lower)
        }
        /// # 列方向に結合
        pub fn combine_row(&self, lower: &Self) -> Matrix<T> {
            assert_eq!(self.width, lower.width);
            let mut v = Vec::with_capacity(self.height + lower.height);
            for y in 0..self.height {
                let mut l = Vec::with_capacity(self.width);
                for x in 0..self.width {
                    l.push(self.get(y, x));
                }
                v.push(l);
            }
            for y in 0..lower.height {
                let mut l = Vec::with_capacity(lower.width);
                for x in 0..lower.width {
                    l.push(lower.get(y, x));
                }
                v.push(l);
            }
            Matrix::build(v).unwrap()
        }
        /// # 行方向に結合
        pub fn combine_column(&self, right: &Self) -> Matrix<T> {
            assert_eq!(self.height, right.height);
            let mut v = Vec::with_capacity(self.height);
            for y in 0..self.height {
                let mut l = Vec::with_capacity(self.width + right.width);
                for x in 0..self.width {
                    l.push(self.get(y, x));
                }
                for x in 0..right.width {
                    l.push(right.get(y, x));
                }
                v.push(l);
            }
            Matrix::build(v).unwrap()
        }
        pub fn extend(&self, height: usize, width: usize) -> Matrix<T> {
            let mut v = Vec::with_capacity(self.height);
            for y in 0..height {
                let mut l = Vec::with_capacity(width);
                for x in 0..width {
                    l.push(self.get(y, x));
                }
                v.push(l)
            }
            Matrix::build(v).unwrap()
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

    impl<
            T: Clone
                + Copy
                + Zero
                + One
                + Mul<Output = T>
                + Add<Output = T>
                + Sub<Output = T>
                + ToString,
        > Pow for Matrix<T>
    {
        fn pow(mut self, mut e: i64) -> Self {
            assert_eq!(self.height, self.width);
            let mut result = Self::identity_matrix(self.height);
            while e > 0 {
                if e & 1 == 1 {
                    result = (result.pointer() * self.pointer()).unwrap();
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

    /// # 行列同士の加算
    /// 出力の行列のサイズは大きいほうに合わせられる
    impl<T: Add<Output = T> + Clone + Copy + Zero> Add<PointerMatrix<'_, T>> for PointerMatrix<'_, T> {
        type Output = Matrix<T>;
        fn add(self, rhs: PointerMatrix<T>) -> Self::Output {
            let (width, height) = (max(self.width, rhs.width), max(self.height, rhs.height));
            Matrix::build(
                (0..height)
                    .map(|y| (0..width).map(|x| self.get(y, x) + rhs.get(y, x)).collect())
                    .collect(),
            )
            .unwrap()
        }
    }

    impl<T: Sub<Output = T> + Clone + Copy + Zero> Sub<PointerMatrix<'_, T>> for PointerMatrix<'_, T> {
        type Output = Matrix<T>;
        fn sub(self, rhs: PointerMatrix<'_, T>) -> Self::Output {
            let (width, height) = (max(self.width, rhs.width), max(self.height, rhs.height));
            Matrix::build(
                (0..height)
                    .map(|y| (0..width).map(|x| self.get(y, x) - rhs.get(y, x)).collect())
                    .collect(),
            )
            .unwrap()
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

    impl<
            'a,
            T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero + Clone + ToString + Copy,
        > PointerMatrix<'a, T>
    {
        /// # 愚直積
        ///
        /// ## 計算量
        /// $O(N^3)$
        fn naive_mul(&self, rhs: &Self) -> Option<Matrix<T>> {
            let (self_row, self_col, rhs_row, rhs_col) =
                (self.height, self.width, rhs.height, rhs.width);
            if self_col != rhs_row {
                return None;
            }
            let mut ret = Matrix::zero_matrix(rhs_col, self_row);
            ret.src.iter_mut().enumerate().for_each(|(i, bufi)| {
                bufi.iter_mut().enumerate().for_each(|(j, bufij)| {
                    *bufij = (0..self_col)
                        .map(|k| self.get(i, k) * rhs.get(k, j))
                        .fold(T::zero(), |x, a| x.add(a));
                });
            });
            Some(ret)
        }

        /// # シュトラッセンのアルゴリズム
        ///
        /// ## see
        /// https://en.wikipedia.org/wiki/Strassen_algorithm
        fn strassen_mul(&self, rhs: &Self) -> Option<Matrix<T>> {
            if self.width != rhs.height {
                return None;
            }
            if self.height != self.width || rhs.height != rhs.width || self.height < 128 {
                return self.naive_mul(rhs);
            }
            let n = self.height;
            if n & 1 == 0 {
                let half = n / 2;
                let (a1, a2) = self.row_divide(half);
                let (a11, a12) = a1.column_divide(half);
                let (a21, a22) = a2.column_divide(half);
                let (b1, b2) = rhs.row_divide(half);
                let (b11, b12) = b1.column_divide(half);
                let (b21, b22) = b2.column_divide(half);
                let p1 = ((a11 + a22) * (b11 + b22)).unwrap();
                let p2 = ((a21 + a22) * b11).unwrap();
                let p3 = (a11 * (b12 - b22).pointer()).unwrap();
                let p4 = (a22 * (b21 - b11).pointer()).unwrap();
                let p5 = ((a11 + a12).pointer() * b22).unwrap();
                let p6 = ((a21 - a11) * (b11 + b12)).unwrap();
                let p7 = ((a12 - a22) * (b21 + b22)).unwrap();
                let c11 = ((p1.pointer() + p4.pointer()).pointer() - p5.pointer()).pointer()
                    + p7.pointer();
                let c12 = p3.pointer() + p5.pointer();
                let c21 = p2.pointer() + p4.pointer();
                let c22 = ((p1.pointer() + p3.pointer()).pointer() - p2.pointer()).pointer()
                    + p6.pointer();
                let c1 = c11.pointer().combine_column(&c12.pointer());
                let c2 = c21.pointer().combine_column(&c22.pointer());
                Some(c1.pointer().combine_row(&c2.pointer()))
            } else {
                let half = (n + 1) / 2;
                let (a1, a2) = self.row_divide(half);
                let (a11, a12) = a1.column_divide(half);
                let (a21, a22) = a2.column_divide(half);
                let (b1, b2) = rhs.row_divide(half);
                let (b11, b12) = b1.column_divide(half);
                let (b21, b22) = b2.column_divide(half);
                // dbg!(a11, a12, a21, a22, b11, b12, b21, b22);
                // dbg!(a11 + a22, b11 + b22);
                let p1 = ((a11 + a22) * (b11 + b22)).unwrap();
                let p2 = ((a21 + a22) * b11).unwrap();
                let p3 = (a11 * (b12 - b22).pointer()).unwrap();
                let a22e = a22.extend(half, half);
                let p4 = (a22e.pointer() * (b21 - b11).pointer()).unwrap();
                let b22e = b22.extend(half, half);
                let p5 = ((a11 + a12).pointer() * b22e.pointer()).unwrap();
                let p6 = ((a21 - a11) * (b11 + b12)).unwrap();
                let p7 = ((a12 - a22) * (b21 + b22)).unwrap();
                // dbg!(&p1, &p2, &p3, &p4, &p5, &p6, &p7);
                let c11 = ((p1.pointer() + p4.pointer()).pointer() - p5.pointer()).pointer()
                    + p7.pointer();
                let c12 = p3.pointer() + p5.pointer();
                let c21 = p2.pointer() + p4.pointer();
                let c22 = ((p1.pointer() + p3.pointer()).pointer() - p2.pointer()).pointer()
                    + p6.pointer();
                let c1 = c11.pointer().combine_column(&c12.pointer());
                let c2 = c21.pointer().combine_column(&c22.pointer());
                Some(c1.pointer().combine_row(&c2.pointer()))
            }
        }
    }

    impl<
            T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero + Clone + Copy + ToString,
        > Mul<PointerMatrix<'_, T>> for PointerMatrix<'_, T>
    {
        type Output = Option<Matrix<T>>;
        fn mul(self, rhs: PointerMatrix<'_, T>) -> Self::Output {
            self.strassen_mul(&rhs)
        }
    }

    impl<
            T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero + Clone + Copy + ToString,
        > Mul<&PointerMatrix<'_, T>> for &PointerMatrix<'_, T>
    {
        type Output = Option<Matrix<T>>;
        fn mul(self, rhs: &PointerMatrix<'_, T>) -> Self::Output {
            self.strassen_mul(rhs)
        }
    }

    impl<
            T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero + Clone + Copy + ToString,
        > Mul<PointerMatrix<'_, T>> for Matrix<T>
    {
        type Output = Option<Matrix<T>>;
        fn mul(self, rhs: PointerMatrix<'_, T>) -> Self::Output {
            self.pointer() * rhs
        }
    }

    impl<
            T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Zero + Clone + Copy + ToString,
        > Mul<Matrix<T>> for Matrix<T>
    {
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

    impl<'a, T: ToString> Debug for PointerMatrix<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "height:{} width:{}\ny_offset:{} x_offset:{}\n{}",
                self.height,
                self.width,
                self.y_offset,
                self.x_offset,
                self.src
                    .iter()
                    .skip(self.y_offset)
                    .take(self.height)
                    .map(|row| row
                        .iter()
                        .skip(self.x_offset)
                        .take(self.width)
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

    #[test]
    fn test() {
        let data = vec![
            vec![ModInt::new(3), ModInt::new(2)],
            vec![ModInt::new(5), ModInt::new(4)],
        ];
        let matrix: Matrix<ModInt> = Matrix::build(data).unwrap();
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
        let matrix: Matrix<ModInt> = Matrix::build(data).unwrap();
        let sub_matrix = matrix.sub_matrix(2, 3);
        let expect_sub_matrix: Matrix<ModInt> = Matrix::build(vec![
            vec![ModInt::new(0), ModInt::new(1), ModInt::new(3)],
            vec![ModInt::new(4), ModInt::new(5), ModInt::new(7)],
            vec![ModInt::new(8), ModInt::new(9), ModInt::new(11)],
        ])
        .unwrap();
        assert_eq!(sub_matrix, expect_sub_matrix);
        assert_eq!(sub_matrix.determinant(), Some(ModInt::new(0)));

        let lhs: Matrix<ModInt> =
            Matrix::row_vector(&vec![ModInt::new(1), ModInt::new(2), ModInt::new(3)]);
        let rhs: Matrix<ModInt> =
            Matrix::column_vector(&vec![ModInt::new(4), ModInt::new(5), ModInt::new(6)]);
        let expect: Matrix<ModInt> = Matrix::build(vec![vec![ModInt::new(32)]]).unwrap();
        assert_eq!(lhs * rhs, Some(expect));
    }

    #[test]
    fn test_column_divide() {
        let m = Matrix::build(vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 15],
        ])
        .unwrap();
        let (left, right) = m.pointer().column_divide(3);
        let expect_left_matrix = Matrix::build(vec![
            vec![0, 1, 2],
            vec![4, 5, 6],
            vec![8, 9, 10],
            vec![12, 13, 14],
        ])
        .unwrap();
        let expect_right_matrix =
            Matrix::build(vec![vec![3], vec![7], vec![11], vec![15]]).unwrap();
        assert_eq!(expect_left_matrix.pointer(), left);
        assert_eq!(expect_right_matrix.pointer(), right);
        let ex = left.extend(4, 4);
        let expect_extend_matrix = Matrix::build(vec![
            vec![0, 1, 2, 0],
            vec![4, 5, 6, 0],
            vec![8, 9, 10, 0],
            vec![12, 13, 14, 0],
        ])
        .unwrap();
        assert_eq!(expect_extend_matrix, ex);
    }

    #[test]
    fn test_row_divide() {
        let m = Matrix::build(vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 15],
        ])
        .unwrap();
        let (upper, lower) = m.pointer().row_divide(3);
        let expect_upper_matrix =
            Matrix::build(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]]).unwrap();
        let expect_lower_matrix = Matrix::build(vec![vec![12, 13, 14, 15]]).unwrap();
        assert_eq!(expect_upper_matrix.pointer(), upper);
        assert_eq!(expect_lower_matrix.pointer(), lower);
        let ex = upper.extend(4, 4);
        let expect_extend_matrix = Matrix::build(vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9, 10, 11],
            vec![0, 0, 0, 0],
        ])
        .unwrap();
        assert_eq!(expect_extend_matrix, ex);
    }
}
