//! # AND畳み込み
//!

use super::ConvolutionType;
use mod_int::ModInt;
use prelude::*;

#[codesnip::entry("and-convolution", doc_hidden)]
#[snippet(include = "bitwise-convolution")]
pub struct AndConvolution;

#[codesnip::entry("and-convolution", doc_hidden)]
impl ConvolutionType for AndConvolution {
    fn fwht<const M: u32>(src: &mut [ModInt<M>], rev: bool) {
        let mut i = 1;
        while i < src.len() {
            for j in 0..src.len() {
                if i & j == 0 {
                    src[j] = if !rev {
                        src[j] + src[i | j]
                    } else {
                        src[j] - src[i | j]
                    };
                }
            }
            i <<= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{super::convolution, AndConvolution, ModInt};
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
                    expect[i & j] += a[i] * b[j];
                }
            }
            let result = convolution::<AndConvolution, 998_244_353>(a, b);
            assert_eq!(expect, result);
        }
    }
}
