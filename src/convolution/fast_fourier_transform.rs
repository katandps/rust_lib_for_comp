//! # 高速フーリエ変換
//!
//! ## todo
//! - \[一般化]Garnerのアルゴリズムによる任意modでの畳み込みの実装
//!
//! ## verify
//! [ACLPracticeContestF](https://atcoder.jp/contests/practice2/submissions/35796056)

use crate::prelude::*;
#[snippet(name = "fast-fourier-transform", doc_hidden)]
pub struct FFT<T> {
    rate: Vec<T>,
    rate_inv: Vec<T>,
}

#[snippet(name = "fast-fourier-transform", doc_hidden)]
mod fast_fourier_transform_impl {
    use super::{Add, Div, DivAssign, Mul, MulAssign, One, PrimitiveRoot, Sub, Zero, FFT};

    impl<
            T: Copy
                + One
                + Add<Output = T>
                + Sub<Output = T>
                + Mul<Output = T>
                + Div<Output = T>
                + MulAssign
                + DivAssign<i64>
                + PrimitiveRoot
                + Zero
                + One,
        > FFT<T>
    {
        fn fft(&self, src: &mut Vec<T>, height: usize, rate: &[T]) {
            assert!(src.len() == 1 << height);
            for phase in 1..=height {
                let (w, p) = (1 << (phase - 1), 1 << (height - phase));
                let mut zeta = T::one();
                for s in 0..w {
                    let offset = s << (height - phase + 1);
                    for i in 0..p {
                        let (l, r) = (src[i + offset], src[i + offset + p] * zeta);
                        src[i + offset] = l + r;
                        src[i + offset + p] = l - r;
                    }
                    zeta *= rate[(!s).trailing_zeros() as usize];
                }
            }
        }
        fn ifft(&self, src: &mut Vec<T>, height: usize, rate_inv: &[T]) {
            for phase in (1..=height).rev() {
                let (w, p) = (1 << (phase - 1), 1 << (height - phase));
                let mut zeta = T::one();
                for s in 0..w {
                    let offset = s << (height - phase + 1);
                    for i in 0..p {
                        let (l, r) = (src[i + offset], src[i + offset + p]);
                        src[i + offset] = l + r;
                        src[i + offset + p] = (l - r) * zeta;
                    }
                    zeta *= rate_inv[(!s).trailing_zeros() as usize];
                }
            }
        }

        pub fn setup() -> Self {
            // root[i] ^ (2^i) == 1
            let mut root = vec![T::zero(); T::DIVIDE_LIMIT + 1];
            // root[i] * root_inv[i] == 1
            let mut root_inv = vec![T::zero(); T::DIVIDE_LIMIT + 1];
            root[T::DIVIDE_LIMIT] = T::primitive_root();
            root_inv[T::DIVIDE_LIMIT] = T::one() / root[T::DIVIDE_LIMIT];
            for i in (0..T::DIVIDE_LIMIT).rev() {
                root[i] = root[i + 1] * root[i + 1];
                root_inv[i] = root_inv[i + 1] * root_inv[i + 1];
            }
            let mut rate = vec![T::zero(); T::DIVIDE_LIMIT - 2 + 1];
            let mut rate_inv = vec![T::zero(); T::DIVIDE_LIMIT - 2 + 1];
            let (mut prod, mut prod_inv) = (T::one(), T::one());
            for i in 0..=T::DIVIDE_LIMIT - 2 {
                rate[i] = root[i + 2] * prod;
                rate_inv[i] = root_inv[i + 2] * prod_inv;
                prod *= root_inv[i + 2];
                prod_inv *= root[i + 2];
            }
            Self { rate, rate_inv }
        }

        pub fn convolution(&self, mut f: Vec<T>, mut g: Vec<T>) -> Vec<T> {
            let size = f.len() + g.len() - 1;
            let dim = size.next_power_of_two();
            let log2_dim = dim.trailing_zeros() as usize;
            f.resize(dim, T::zero());
            g.resize(dim, T::zero());
            self.fft(&mut f, log2_dim, &self.rate);
            self.fft(&mut g, log2_dim, &self.rate);
            f.iter_mut().enumerate().for_each(|(i, a)| *a *= g[i]);
            self.ifft(&mut f, log2_dim, &self.rate_inv);
            f.resize(size, T::zero());
            f.iter_mut().for_each(|c| *c /= dim as i64);
            f
        }
    }
}

#[cfg(test)]
mod test {
    use super::FFT;
    use crate::algebra::mod_int::mod998244353::mi;
    use crate::algo::xor_shift::XorShift;
    #[test]
    fn rand() {
        let mut xor_shift = XorShift::default();
        let fft = FFT::setup();
        for _ in 0..100 {
            let a_len = xor_shift.rand_range(1000..2000) as usize;
            let b_len = xor_shift.rand_range(1000..2000) as usize;
            let a = (0..a_len)
                .map(|_| xor_shift.rand_range(0..998244353).into())
                .collect::<Vec<_>>();
            let b = (0..b_len)
                .map(|_| xor_shift.rand_range(0..998244353).into())
                .collect::<Vec<_>>();
            let size = a_len + b_len - 1;
            let mut expect = vec![mi(0); size];
            for i in 0..a_len {
                for j in 0..b_len {
                    expect[i + j] += a[i] * b[j];
                }
            }
            let result = fft.convolution(a, b);
            assert_eq!(expect, result);
        }
    }

    #[test]
    fn hand() {
        let fft = FFT::setup();
        let a = (1..=4).map(mi).collect();
        let b = (5..=9).map(mi).collect();
        let result = fft.convolution(a, b);
        assert_eq!(
            result,
            vec![5, 16, 34, 60, 70, 70, 59, 36]
                .into_iter()
                .map(mi)
                .collect::<Vec<_>>()
        );
    }
}
