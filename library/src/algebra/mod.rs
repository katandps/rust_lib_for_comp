//! 代数

pub mod all_combination;
pub mod all_permutation;
pub mod chinese_remainder_theorem;
pub mod greatest_common_divisor;
pub mod impl_monoid;
pub mod lucas_theorem;
pub mod matrix;
pub mod mod_int;
pub mod mod_inv;
pub mod mod_pow;
pub mod mod_val_table;
pub mod sieve_of_eratosthenes;

/////////////////////////////////////////////////////////

pub use algebra::*;
pub mod algebra {
    use std::fmt;
    use std::iter::{Product, Sum};
    use std::ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    };

    /// マグマ
    /// 二項演算: $`M \circ M \to M`$
    pub trait Magma {
        /// マグマを構成する集合$`M`$
        type M: Clone + PartialEq;
        /// マグマを構成する演算$`op`$
        fn op(x: &Self::M, y: &Self::M) -> Self::M;
    }

    /// 結合則
    /// $`\forall a,\forall b, \forall c \in T, (a \circ b) \circ c = a \circ (b \circ c)`$
    pub trait Associative {}

    /// 半群
    pub trait SemiGroup: Magma + Associative {}

    /// 単位的
    pub trait Unital: Magma {
        /// 単位元 identity element: $`e`$
        fn unit() -> Self::M;
    }

    /// モノイド
    /// 結合則と、単位元を持つ
    pub trait Monoid: SemiGroup + Unital {
        /// $`x^n = x\circ\cdots\circ x`$
        fn pow(&self, x: Self::M, mut n: usize) -> Self::M {
            let mut res = Self::unit();
            let mut base = x;
            while n > 0 {
                if n & 1 == 1 {
                    res = Self::op(&res, &base);
                }
                base = Self::op(&base, &base);
                n >>= 1;
            }
            res
        }
    }

    impl<M: SemiGroup + Unital> Monoid for M {}

    /// 可逆的
    /// $`\exists e \in T, \forall a \in T, \exists b,c \in T, b \circ a = a \circ c = e`$
    pub trait Invertible: Magma {
        /// $`a`$ where $`a \circ x = e`$
        fn inv(&self, x: &Self::M) -> Self::M;
    }

    /// 群
    pub trait Group: Monoid + Invertible {}

    /// 作用付きモノイド
    pub trait MapMonoid {
        /// モノイドM
        type M: Monoid;
        type F: Clone;
        /// 値xと値yを併合する
        fn op(x: &<Self::M as Magma>::M, y: &<Self::M as Magma>::M) -> <Self::M as Magma>::M {
            Self::M::op(&x, &y)
        }
        /// 作用fをvalueに作用させる
        fn apply(f: &Self::F, value: &<Self::M as Magma>::M) -> <Self::M as Magma>::M;
        /// 作用fの単位元
        fn identity_map() -> Self::F;
        /// 作用fと作用gを合成する
        fn compose(f: &Self::F, g: &Self::F) -> Self::F;
    }

    /// 加算の単位元
    pub trait Zero {
        fn zero() -> Self;
    }

    /// 乗算の単位元
    pub trait One {
        fn one() -> Self;
    }

    /// 下に有界
    pub trait BoundedBelow {
        fn min_value() -> Self;
    }

    /// 上に有界
    pub trait BoundedAbove {
        fn max_value() -> Self;
    }

    pub trait Integral:
        'static
        + Send
        + Sync
        + Copy
        + Ord
        + Not<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Sum
        + Product
        + BitOr<Output = Self>
        + BitAnd<Output = Self>
        + BitXor<Output = Self>
        + BitOrAssign
        + BitAndAssign
        + BitXorAssign
        + Shl<Output = Self>
        + Shr<Output = Self>
        + ShlAssign
        + ShrAssign
        + fmt::Display
        + fmt::Debug
        + Zero
        + One
        + BoundedBelow
        + BoundedAbove
    {
    }

    macro_rules! impl_integral {
        ($($ty:ty),*) => {
            $(
                impl Zero for $ty {
                    fn zero() -> Self {
                        0
                    }
                }

                impl One for $ty {
                    fn one() -> Self {
                        1
                    }
                }

                impl BoundedBelow for $ty {
                    fn min_value() -> Self {
                        Self::min_value()
                    }
                }

                impl BoundedAbove for $ty {
                    fn max_value() -> Self {
                        Self::max_value()
                    }
                }

                impl Integral for $ty {}
            )*
        };
    }
    impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
}
