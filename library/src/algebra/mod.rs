//! 代数
use crate::*;

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
pub mod subset;

/////////////////////////////////////////////////////////

/// マグマ
/// 二項演算: $`M \circ M \to M`$
pub trait Magma {
    /// マグマを構成する集合$`M`$
    type M: Clone + Debug + PartialEq;
    /// マグマを構成する演算$`op`$
    fn op(x: &Self::M, y: &Self::M) -> Self::M;
}

/// 結合則
/// $`\forall a,\forall b, \forall c \in T, (a \circ b) \circ c = a \circ (b \circ c)`$
pub trait Associative {}

/// 半群
pub trait SemiGroup {}
impl<M: Magma + Associative> SemiGroup for M {}

/// 単位的
pub trait Unital: Magma {
    /// 単位元 identity element: $`e`$
    fn unit() -> Self::M;
}

/// モノイド
/// 結合則と、単位元を持つ
pub trait Monoid {
    type M: Clone + Debug + PartialEq;
    fn op(x: &Self::M, y: &Self::M) -> Self::M;

    fn unit() -> Self::M;

    /// $`x^n = x\circ\cdots\circ x`$
    fn pow(&self, x: Self::M, n: usize) -> Self::M;
}

impl<M: SemiGroup + Unital> Monoid for M {
    type M = M::M;
    fn op(x: &M::M, y: &M::M) -> M::M {
        M::op(x, y)
    }

    fn unit() -> Self::M {
        M::unit()
    }

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

/// 可逆的
/// $`\exists e \in T, \forall a \in T, \exists b,c \in T, b \circ a = a \circ c = e`$
pub trait Invertible: Magma {
    /// $`a`$ where $`a \circ x = e`$
    fn inv(&self, x: &Self::M) -> Self::M;
}

/// 群
pub trait Group {}
impl<M: Monoid + Invertible> Group for M {}

/// 作用付きモノイド
/// 値Mono、作用Funcはモノイドで、
pub trait MapMonoid: Debug {
    /// モノイドM
    type Mono: Monoid;
    type Func: Monoid;
    /// 値xと値yを併合する
    fn op(
        x: &<Self::Mono as Monoid>::M,
        y: &<Self::Mono as Monoid>::M,
    ) -> <Self::Mono as Monoid>::M {
        Self::Mono::op(&x, &y)
    }
    fn unit() -> <Self::Mono as Monoid>::M {
        Self::Mono::unit()
    }
    /// 作用fをvalueに作用させる
    fn apply(
        f: &<Self::Func as Monoid>::M,
        value: &<Self::Mono as Monoid>::M,
    ) -> <Self::Mono as Monoid>::M;
    /// 作用fの単位元
    fn identity_map() -> <Self::Func as Monoid>::M {
        Self::Func::unit()
    }
    /// 作用fと作用gを合成する
    fn compose(
        f: &<Self::Func as Monoid>::M,
        g: &<Self::Func as Monoid>::M,
    ) -> <Self::Func as Monoid>::M {
        Self::Func::op(f, g)
    }
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

/// 整数
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
    + Display
    + Debug
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
