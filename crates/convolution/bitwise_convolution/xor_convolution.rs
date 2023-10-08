//! # XOR畳み込み
//!
use super::ConvolutionType;
use mod_int::{Mod, ModInt};
use prelude::*;

#[snippet(name = "xor-convolution", doc_hidden)]
#[snippet(include = "bitwise-convolution")]
pub struct XorConvolution;

#[snippet(name = "xor-convolution", doc_hidden)]
impl ConvolutionType for XorConvolution {
    #[allow(clippy::many_single_char_names)]
    fn fwht<M: Mod>(src: &mut [ModInt<M>], rev: bool) {
        let n = src.len();
        let mut i = 1;
        while i < n {
            for j in 0..i {
                let mut k = 0;
                while k < n {
                    let (u, v) = (src[j + k], src[j + k + i]);
                    src[j + k] = u + v;
                    src[j + k + i] = u - v;
                    k += i * 2;
                }
            }
            i <<= 1;
        }
        if rev {
            let inv = ModInt::one() / n as i64;
            src.iter_mut().for_each(|p| *p *= inv);
        }
    }
}

#[cfg(test)]
mod test {
    use super::{super::convolution, ModInt, XorConvolution};
    use mod_int::mod998244353::Mod998_244_353;
    use xor_shift::XorShift;

    #[test]
    fn rand() {
        let mut xor_shift = XorShift::default();
        for _ in 0..100 {
            let a_len = xor_shift.rand_range(1000..2000) as usize;
            let b_len = xor_shift.rand_range(1000..2000) as usize;
            let a = (0..a_len)
                .map(|_| xor_shift.rand_range(0..998244353).into())
                .collect::<Vec<_>>();
            let b = (0..b_len)
                .map(|_| xor_shift.rand_range(0..998244353).into())
                .collect::<Vec<_>>();
            let mut expect = vec![ModInt::zero(); max!(a.len(), b.len()).next_power_of_two()];
            for i in 0..a_len {
                for j in 0..b_len {
                    expect[i ^ j] += a[i] * b[j];
                }
            }
            let result = convolution::<Mod998_244_353, XorConvolution>(a, b);
            assert_eq!(expect, result);
        }
    }
}
