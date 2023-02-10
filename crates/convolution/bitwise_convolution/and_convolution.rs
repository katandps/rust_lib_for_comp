//! # AND畳み込み
//!
//! ## verify
//! [Bitwise And Convolution](https://judge.yosupo.jp/submission/109188)

use super::ConvolutionType;
use mod_int::{Mod, ModInt};
use prelude::*;

#[snippet(name = "and-convolution", doc_hidden)]
#[snippet(include = "bitwise-convolution")]
pub struct AndConvolution;

#[snippet(name = "and-convolution", doc_hidden)]
impl ConvolutionType for AndConvolution {
    #[allow(clippy::many_single_char_names)]
    fn fwht<M: Mod>(src: &mut Vec<ModInt<M>>, rev: bool) {
        let n = src.len();
        let mut i = 1;
        while i < n {
            for j in 0..n {
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
    use super::{super::convolution, AndConvolution};
    use mod_int::mod998244353::{mi, Mod998_244_353};
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
            let mut expect = vec![mi(0); max!(a.len(), b.len()).next_power_of_two()];
            for i in 0..a_len {
                for j in 0..b_len {
                    expect[i & j] += a[i] * b[j];
                }
            }
            let result = convolution::<Mod998_244_353, AndConvolution>(a, b);
            assert_eq!(expect, result);
        }
    }
}
