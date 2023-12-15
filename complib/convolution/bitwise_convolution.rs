//! # 高速アダマール変換 (Fast Walsh Hadamard Transform)

use crate::algebra::mod_int::ModInt;
pub(crate) use crate::min_max_macro::max;

pub mod and_convolution;
pub mod or_convolution;
pub mod xor_convolution;

#[codesnip::entry("bitwise-convolution")]
pub trait ConvolutionType {
    fn fwht<const M: u32>(poly: &mut [ModInt<M>], rev: bool);
}

#[codesnip::entry("bitwise-convolution", include("max", "mod-int"))]
pub fn convolution<T: ConvolutionType, const M: u32>(
    mut a: Vec<ModInt<M>>,
    mut b: Vec<ModInt<M>>,
) -> Vec<ModInt<M>> {
    let n = max!(a.len(), b.len()).next_power_of_two();
    a.resize(n, ModInt::zero());
    b.resize(n, ModInt::zero());
    T::fwht(&mut a, false);
    T::fwht(&mut b, false);
    for i in 0..n {
        a[i] *= b[i];
    }
    T::fwht(&mut a, true);
    a.resize(n, ModInt::zero());
    a
}
