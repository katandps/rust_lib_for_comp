//! # 高速フーリエ変換
//!
//! ## todo
//! - \[一般化]Garnerのアルゴリズムによる任意modでの畳み込みの実装
//!
//! ## verify
//! [ACLPracticeContestF](https://atcoder.jp/contests/practice2/submissions/35796056)

use crate::{
    algebra::mod_int::{mod998244353::mod_998_244_353_impl::Mod998_244_353, ModInt},
    prelude::*,
};
#[snippet(name = "fast-fourier-transform", doc_hidden)]
pub struct FFT<T> {
    root: Vec<T>,
    root_inv: Vec<T>,
}

#[snippet(name = "fast-fourier-transform", doc_hidden)]
mod fast_fourier_transform_impl {
    use super::{
        Add, Div, Mod998_244_353, ModInt, Mul, MulAssign, One, Pow, PrimitiveRoot, Sum, Zero, FFT,
    };

    impl<
            T: Copy
                + Zero
                + One
                + Add<Output = T>
                + Mul<Output = T>
                + Div<Output = T>
                + MulAssign
                + Sum
                + PrimitiveRoot
                + Pow,
        > FFT<T>
    {
        fn fft(&self, src: &mut Vec<T>, bit: usize, inv: bool) {
            let mut zeta = vec![T::one(); src.len()];
            let mask1 = src.len() - 1;
            assert!(src.len() == 1 << bit);
            assert!(bit <= 23);
            let root = if inv { &self.root_inv } else { &self.root }[bit];
            for i in 1..src.len() {
                zeta[i] = zeta[i - 1] * root;
            }
            for i in 0..bit {
                let mask2 = mask1 >> (i + 1);
                *src = (0..src.len())
                    .map(|j| {
                        let lower = j & mask2;
                        let upper = j ^ lower;
                        let shifted = upper << 1 & mask1;
                        src[shifted | lower] + zeta[upper] * src[shifted | (mask2 + 1) | lower]
                    })
                    .collect::<Vec<_>>();
            }
        }
    }

    impl FFT<ModInt<Mod998_244_353>> {
        const DIVIDE_LIMIT: usize = 23;

        pub fn setup() -> Self {
            let mut root = vec![ModInt::zero(); Self::DIVIDE_LIMIT + 1];
            let mut root_inv = vec![ModInt::zero(); Self::DIVIDE_LIMIT + 1];
            root[Self::DIVIDE_LIMIT] = PrimitiveRoot::primitive_root();
            root_inv[Self::DIVIDE_LIMIT] = ModInt::one() / root[Self::DIVIDE_LIMIT];
            for i in (0..Self::DIVIDE_LIMIT).rev() {
                root[i] = root[i + 1] * root[i + 1];
                root_inv[i] = root_inv[i + 1] * root_inv[i + 1];
            }
            Self { root, root_inv }
        }

        pub fn convolution(
            &self,
            mut f: Vec<ModInt<Mod998_244_353>>,
            mut g: Vec<ModInt<Mod998_244_353>>,
        ) -> Vec<ModInt<Mod998_244_353>> {
            let size = f.len() + g.len() - 1;
            let dim = size.next_power_of_two();
            let log2_dim = dim.trailing_zeros() as usize;

            f.resize(dim, ModInt::zero());
            g.resize(dim, ModInt::zero());
            self.fft(&mut f, log2_dim, false);
            self.fft(&mut g, log2_dim, false);
            f.iter_mut().enumerate().for_each(|(i, a)| *a *= g[i]);
            self.fft(&mut f, log2_dim, true);
            f.resize(size, ModInt::zero());
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
}
