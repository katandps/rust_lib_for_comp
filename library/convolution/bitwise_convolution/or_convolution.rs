//! # OR畳み込み

use super::ConvolutionType;
use crate::algebra::mod_int::ModInt;

#[codesnip::entry("or-convolution")]
pub struct OrConvolution;

#[codesnip::entry("or-convolution", include("bitwise-convolution", "mod-int"))]
impl ConvolutionType for OrConvolution {
    fn fwht<const M: u32>(src: &mut [ModInt<M>], rev: bool) {
        let n = src.len();
        let mut i = 1;
        while i < n {
            for j in 0..i {
                let mut k = 0;
                while k < n {
                    src[j + k + i] = if rev {
                        src[j + k + i] - src[j + k]
                    } else {
                        src[j + k + i] + src[j + k]
                    };
                    k += i * 2;
                }
            }
            i <<= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{super::convolution, ModInt, OrConvolution};
    use crate::algo::xor_shift::XorShift;
    pub(crate) use crate::min_max_macro::max;

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
                    expect[i | j] += a[i] * b[j];
                }
            }
            let result = convolution::<OrConvolution, 998_244_353>(a, b);
            assert_eq!(expect, result);
        }
    }
}
