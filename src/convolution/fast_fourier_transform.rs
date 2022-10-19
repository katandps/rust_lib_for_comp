//! # 高速フーリエ変換
//!
//! ## todo
//! - \[高速化]バタフライ演算を使用した非再帰での実装
//! - \[一般化]Garnerのアルゴリズムによる任意modでの畳み込みの実装
//!
//! ## verify
//! [ACLPracticeContestF](https://atcoder.jp/contests/practice2/submissions/35782670)

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
    fn ntt(&self, f: &mut [T], inverse: bool, log2_f: usize, divide_cnt: usize) {
        debug_assert!(log2_f < self.root.len());
        if log2_f == 0 || divide_cnt == 0 {
            let mut zeta = T::one();
            for i in 0..f.len() {
                f[i] = (0..f.len())
                    .scan(zeta, |zeta, i| {
                        let ret = f[i] * *zeta;
                        *zeta *= *zeta;
                        Some(ret)
                    })
                    .sum();
                zeta *= if inverse { &self.root } else { &self.root_inv }[0];
            }
        } else {
            let (mut f1, mut f2) = (
                Vec::with_capacity(f.len() / 2),
                Vec::with_capacity(f.len() / 2),
            );
            for i in 0..f.len() / 2 {
                f1.push(f[i * 2]);
                f2.push(f[i * 2 + 1]);
            }
            self.ntt(&mut f1, inverse, log2_f - 1, divide_cnt - 1);
            self.ntt(&mut f2, inverse, log2_f - 1, divide_cnt - 1);
            let mut zeta = T::one();
            for i in 0..f.len() {
                f[i] = f1[i % f1.len()] + zeta * f2[i % f2.len()];
                zeta *= if inverse { &self.root } else { &self.root_inv }[log2_f];
            }
        }
    }
}

#[snippet(name = "fast-fourier-transform", doc_hidden)]
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
        self.ntt(&mut f, true, log2_dim, Self::DIVIDE_LIMIT);
        self.ntt(&mut g, true, log2_dim, Self::DIVIDE_LIMIT);
        f.iter_mut().enumerate().for_each(|(i, a)| *a *= g[i]);
        self.ntt(&mut f, false, log2_dim, Self::DIVIDE_LIMIT);
        f.resize(size, ModInt::zero());
        f.iter_mut().for_each(|c| *c /= dim as i64);
        f
    }
}
