//! # 高速フーリエ変換
//! Cooley-TukeyによるSplit Radixアルゴリズムで計算する
//!
//! ## todo
//! - \[一般化]Garnerのアルゴリズムによる任意modでの畳み込みの実装
//!
use algebra::*;
use prelude::*;

#[codesnip::entry("fast-fourier-transform", doc_hidden)]
pub struct FFT<T> {
    root: Vec<T>,
    root_inv: Vec<T>,
    rate2: Vec<T>,
    rate2_inv: Vec<T>,
    rate3: Vec<T>,
    rate3_inv: Vec<T>,
}

#[codesnip::entry("fast-fourier-transform", doc_hidden)]
mod fast_fourier_transform_impl {
    use super::{Add, Div, DivAssign, Mul, MulAssign, Neg, One, PrimitiveRoot, Sub, Zero, FFT};

    impl<
            T: Copy
                + One
                + Add<Output = T>
                + Sub<Output = T>
                + Mul<Output = T>
                + Div<Output = T>
                + MulAssign
                + DivAssign<i64>
                + Neg
                + PrimitiveRoot
                + Zero
                + One,
        > FFT<T>
    {
        fn fft(&self, src: &mut Vec<T>, height: usize) {
            assert!(src.len() == 1 << height);
            let imag = self.root[2];
            let mut phase = 1;
            while phase <= height {
                let mut rot = T::one();
                if phase == height {
                    // 基数2
                    let p = 1 << (height - phase);
                    for s in 0..1 << (phase - 1) {
                        let offset = s << (height - phase + 1);
                        for i in offset..offset + p {
                            let (l, r) = (src[i], src[i + p] * rot);
                            src[i] = l + r;
                            src[i + p] = l - r;
                        }
                        rot *= self.rate2[(!s).trailing_zeros() as usize];
                    }
                    phase += 1;
                } else {
                    // 基数4
                    let p = 1 << (height - phase - 1);
                    let p2 = p + p;
                    let p3 = p2 + p;
                    for s in 0..1 << (phase - 1) {
                        let rot2 = rot * rot;
                        let rot3 = rot2 * rot;
                        let offset = s << (height - phase + 1);
                        for i in offset..offset + p {
                            let (a0, a1, a2, a3) = (
                                src[i],
                                src[i + p] * rot,
                                src[i + p2] * rot2,
                                src[i + p3] * rot3,
                            );
                            let (a0a2, a0na2, a1a3, a1na3) =
                                (a0 + a2, a0 - a2, a1 + a3, (a1 - a3) * imag);
                            src[i] = a0a2 + a1a3;
                            src[i + p] = a0a2 - a1a3;
                            src[i + p2] = a0na2 + a1na3;
                            src[i + p3] = a0na2 - a1na3;
                        }
                        rot *= self.rate3[(!s).trailing_zeros() as usize];
                    }
                    phase += 2;
                }
            }
        }
        fn ifft(&self, src: &mut Vec<T>, height: usize) {
            assert!(src.len() == 1 << height);
            let imag_inv = self.root_inv[2];
            let mut phase = height;
            while phase > 0 {
                let (mut rot_inv, p) = (T::one(), 1 << (height - phase));
                let p2 = p + p;
                let p3 = p2 + p;
                if phase == 1 {
                    // 基数2
                    for s in 0..1 << (phase - 1) {
                        let offset = s << (height - phase + 1);
                        for i in offset..offset + p {
                            let (l, r) = (src[i], src[i + p]);
                            src[i] = l + r;
                            src[i + p] = (l - r) * rot_inv;
                        }
                        rot_inv *= self.rate2_inv[(!s).trailing_zeros() as usize];
                    }
                    phase -= 1;
                } else {
                    // 基数4
                    for s in 0..1 << (phase - 2) {
                        let rot2_inv = rot_inv * rot_inv;
                        let rot3_inv = rot2_inv * rot_inv;
                        let offset = s << (height - phase + 2);
                        for i in offset..offset + p {
                            let (a0, a1, a2, a3) = (src[i], src[i + p], src[i + p2], src[i + p3]);
                            let (a0a1, a0na1, a2a3, a2na3) =
                                (a0 + a1, a0 - a1, a2 + a3, (a2 - a3) * imag_inv);
                            src[i] = a0a1 + a2a3;
                            src[i + p] = (a0na1 + a2na3) * rot_inv;
                            src[i + p2] = (a0a1 - a2a3) * rot2_inv;
                            src[i + p3] = (a0na1 - a2na3) * rot3_inv;
                        }
                        rot_inv *= self.rate3_inv[(!s).trailing_zeros() as usize];
                    }
                    phase -= 2;
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
            let (rate2, rate2_inv) = {
                let mut rate2 = vec![T::zero(); T::DIVIDE_LIMIT - 2 + 1];
                let mut rate2_inv = vec![T::zero(); T::DIVIDE_LIMIT - 2 + 1];
                let (mut prod, mut prod_inv) = (T::one(), T::one());
                for i in 0..=T::DIVIDE_LIMIT - 2 {
                    rate2[i] = root[i + 2] * prod;
                    rate2_inv[i] = root_inv[i + 2] * prod_inv;
                    prod *= root_inv[i + 2];
                    prod_inv *= root[i + 2];
                }
                (rate2, rate2_inv)
            };
            let (rate3, rate3_inv) = {
                let mut rate3 = vec![T::zero(); T::DIVIDE_LIMIT - 3 + 1];
                let mut rate3_inv = vec![T::zero(); T::DIVIDE_LIMIT - 3 + 1];
                let (mut prod, mut prod_inv) = (T::one(), T::one());
                for i in 0..=T::DIVIDE_LIMIT - 3 {
                    rate3[i] = root[i + 3] * prod;
                    rate3_inv[i] = root_inv[i + 3] * prod_inv;
                    prod *= root_inv[i + 3];
                    prod_inv *= root[i + 3];
                }
                (rate3, rate3_inv)
            };
            Self {
                root,
                root_inv,
                rate2,
                rate2_inv,
                rate3,
                rate3_inv,
            }
        }

        pub fn convolution(&self, mut f: Vec<T>, mut g: Vec<T>) -> Vec<T> {
            let size = f.len() + g.len() - 1;
            let dim = size.next_power_of_two();
            let log2_dim = dim.trailing_zeros() as usize;
            f.resize(dim, T::zero());
            g.resize(dim, T::zero());
            self.fft(&mut f, log2_dim);
            self.fft(&mut g, log2_dim);
            f.iter_mut().enumerate().for_each(|(i, a)| *a *= g[i]);
            self.ifft(&mut f, log2_dim);
            f.resize(size, T::zero());
            f.iter_mut().for_each(|c| *c /= dim as i64);
            f
        }
    }
}
