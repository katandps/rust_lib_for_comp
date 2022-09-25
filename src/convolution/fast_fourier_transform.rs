//! # 高速フーリエ変換
//!

use crate::{
    algebra::mod_int::{mod998244353::Mod998244353, ModInt},
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
    const DIVIDE_LIMIT: usize = 23;

    pub fn setup() -> Self {
        let mut root = vec![T::zero(); Self::DIVIDE_LIMIT + 1];
        let mut root_inv = vec![T::zero(); Self::DIVIDE_LIMIT + 1];
        root[Self::DIVIDE_LIMIT] = T::primitive_root();
        root_inv[Self::DIVIDE_LIMIT] = T::one() / root[Self::DIVIDE_LIMIT];
        for i in (0..Self::DIVIDE_LIMIT).rev() {
            root[i] = root[i + 1] * root[i + 1];
            root_inv[i] = root_inv[i + 1] * root_inv[i + 1];
        }
        Self { root, root_inv }
    }
    fn ntt(&self, f: &[T], inverse: bool, log2_f: usize, divide_cnt: usize) -> Vec<T> {
        if log2_f == 0 || divide_cnt == 0 {
            let mut ret = vec![T::zero(); f.len()];
            let mut zeta = T::one();
            for i in 0..f.len() {
                ret[i] = (0..f.len())
                    .scan(zeta, |zeta, i| {
                        let ret = f[i] * *zeta;
                        *zeta *= *zeta;
                        Some(ret)
                    })
                    .sum();
                zeta *= if inverse { &self.root } else { &self.root_inv }[0];
            }
            ret
        } else {
            let (mut f1, mut f2) = (
                Vec::with_capacity(f.len() / 2),
                Vec::with_capacity(f.len() / 2),
            );
            for i in 0..f.len() / 2 {
                f1.push(f[i * 2]);
                f2.push(f[i * 2 + 1]);
            }
            let (f1_dft, f2_dft) = (
                self.ntt(&f1, inverse, log2_f - 1, divide_cnt - 1),
                self.ntt(&f2, inverse, log2_f - 1, divide_cnt - 1),
            );
            (0..f.len())
                .scan(T::one(), |zeta, i| {
                    let ret = Some(f1_dft[i % f1_dft.len()] + *zeta * f2_dft[i % f2_dft.len()]);
                    *zeta *= if inverse { &self.root } else { &self.root_inv }[log2_f];
                    ret
                })
                .collect()
        }
    }
}

#[snippet(name = "fast-fourier-transform", doc_hidden)]
impl FFT<ModInt<Mod998244353>> {
    pub fn convolution(&self, f: &[i64], g: &[i64]) -> Vec<ModInt<Mod998244353>> {
        let dim = (f.len() + g.len()).next_power_of_two();
        let log2_dim = dim.trailing_zeros() as usize;
        let (mut nf, mut ng) = (
            f.iter().map(|fi| ModInt::new(*fi)).collect::<Vec<_>>(),
            g.iter().map(|gi| ModInt::new(*gi)).collect::<Vec<_>>(),
        );
        nf.resize(dim, ModInt::zero());
        ng.resize(dim, ModInt::zero());
        let (f_dft, g_dft) = (
            self.ntt(&nf, true, log2_dim, Self::DIVIDE_LIMIT),
            self.ntt(&ng, true, log2_dim, Self::DIVIDE_LIMIT),
        );
        let fg_dft = (0..dim).map(|i| f_dft[i] * g_dft[i]).collect::<Vec<_>>();
        let fg = self.ntt(&fg_dft, false, log2_dim, Self::DIVIDE_LIMIT);
        fg.into_iter().map(|c| c / dim as i64).collect()
    }
}
