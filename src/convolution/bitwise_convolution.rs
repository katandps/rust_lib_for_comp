//! # Xor-convolution(Fast Walsh Hadamard Transform)
//!
//! ## todo
//! verify
use crate::algebra::mod_int::{Mod, ModInt};
use crate::prelude::*;

pub trait ConvolutionType {
    fn fwht<M: Mod>(poly: &mut Vec<ModInt<M>>, rev: bool);
}

pub fn convolution<M: Mod, T: ConvolutionType>(
    mut a: Vec<ModInt<M>>,
    mut b: Vec<ModInt<M>>,
) -> Vec<ModInt<M>> {
    let sm = a.len() + b.len() - 1;
    let n = sm.next_power_of_two();
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

pub struct XorConvolution;

impl ConvolutionType for XorConvolution {
    #[allow(clippy::many_single_char_names)]
    fn fwht<M: Mod>(poly: &mut Vec<ModInt<M>>, rev: bool) {
        let n = poly.len();
        let mut i = 1;
        while i < n {
            for j in 0..i {
                let mut k = 0;
                while k < n {
                    let (u, v) = (poly[j + k], poly[j + k + i]);
                    poly[j + k] = u + v;
                    poly[j + k + i] = u - v;
                    k += i * 2;
                }
            }
            i <<= 1;
        }
        if rev {
            let inv = ModInt::one() / n as i64;
            poly.iter_mut().for_each(|p| *p *= inv);
        }
    }
}

pub struct OrConvolution;

impl ConvolutionType for OrConvolution {
    fn fwht<M: Mod>(poly: &mut Vec<ModInt<M>>, rev: bool) {
        let n = poly.len();
        let mut i = 1;
        while i < n {
            for j in 0..i {
                let mut k = 0;
                while k < n {
                    poly[j + k + i] = if rev {
                        poly[j + k + i] - poly[j + k]
                    } else {
                        poly[j + k + i] + poly[j + k]
                    };
                    k += i * 2;
                }
            }
            i <<= 1;
        }
    }
}

pub struct AndConvolution;

impl ConvolutionType for AndConvolution {
    #[allow(clippy::many_single_char_names)]
    fn fwht<M: Mod>(poly: &mut Vec<ModInt<M>>, rev: bool) {
        let n = poly.len();
        let mut i = 1;
        while i < n {
            for j in 0..i {
                let mut k = 0;
                while k < n {
                    let (u, v) = (poly[j + k], poly[j + k + i]);
                    if rev {
                        poly[j + k] = v - u;
                        poly[j + k + i] = u;
                    } else {
                        poly[j + k] = u;
                        poly[j + k + i] = u + v;
                    }
                    k += i * 2;
                }
            }
            i <<= 1;
        }
    }
}
