#[allow(unused_imports)]
use matrix::*;
use std::convert::TryInto;

#[allow(dead_code)]
mod matrix {
    use super::Mi;
    use std::fmt::Formatter;
    use std::ops::*;

    #[derive(Clone)]
    pub struct Matrix {
        buf: Vec<Vec<Mi>>,
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
        fn size(&self) -> (usize, usize) {
            if self.buf.len() == 0 {
                (0, 0)
            } else {
                (self.buf.len(), self.buf[0].len())
            }
        }

        pub fn minor_determinant(&self, x: usize, y: usize) -> Option<Mi> {
            let (n, m) = self.size();
            let mut submatrix = Matrix {
                buf: vec![vec![Mi::new(0); m - 1]; n - 1],
            };
            for yi in 0..n {
                if yi == y {
                    break;
                }
                for xi in 0..m {
                    if xi == x {
                        break;
                    }
                    submatrix.add_x_y(self.buf[yi][xi].get(), xi, yi);
                }
            }
            for yi in (0..n).rev() {
                if yi == y {
                    break;
                }
                for xi in 0..m {
                    if xi == x {
                        break;
                    }
                    submatrix.add_x_y(self.buf[yi][xi].get(), xi, yi - 1);
                }
            }
            for yi in 0..n {
                if yi == y {
                    break;
                }
                for xi in (0..m).rev() {
                    if xi == x {
                        break;
                    }
                    submatrix.add_x_y(self.buf[yi][xi].get(), xi - 1, yi);
                }
            }
            for yi in (0..n).rev() {
                if yi == y {
                    break;
                }
                for xi in (0..m).rev() {
                    if xi == x {
                        break;
                    }
                    submatrix.add_x_y(self.buf[yi][xi].get(), xi - 1, yi - 1);
                }
            }
            submatrix.determinant()
        }

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

        pub fn add_x_y(&mut self, value: i64, x: usize, y: usize) {
            self.buf[y][x] += value;
        }
    }

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

    impl Sub<Matrix> for Matrix {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            self + (-rhs)
        }
    }

    use itertools::Itertools;
    use std::fmt::*;

    impl Debug for Matrix {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "{}",
                self.buf.iter().map(|row| row.iter().join(" ")).join("\n")
            )
        }
    }
}
////////////////////////////////////////////////////////

use crate::libraries::mod_int::mod_int::*;

#[cfg(test)]
mod test {
    use super::*;

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
        assert_eq!(matrix.minor_determinant(2, 3), Some(Mi::new(0)));
    }
}
