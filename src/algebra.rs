//! # 代数
use crate::prelude::*;

pub mod binary_operation;
pub mod chinese_remainder_theorem;
pub mod lucas_theorem;
pub mod miller_rabin;
pub mod mod_inv;
pub mod mod_inv_u64;
pub mod mod_pow;
pub mod pollard_rho;
pub mod sieve_of_eratosthenes;
pub mod xor_basis;

pub use algebra_traits::{
    AbelianGroup, Associative, Band, BoundedAbove, BoundedBelow, Commutative, CommutativeMonoid,
    Group, Idempotent, Invertible, Magma, MapMonoid, Monoid, One, Pow, PrimitiveRoot, SemiGroup,
    TrailingZeros, Unital, Zero,
};

mod algebra_traits {
    use super::Debug;
    /// # マグマ
    /// 二項演算: $M \circ M \to M$
    pub trait Magma {
        /// マグマを構成する集合$M$
        type M: Clone + PartialEq + Debug;
        /// マグマを構成する演算$op$
        fn op(x: &Self::M, y: &Self::M) -> Self::M;
    }

    /// # 結合則
    /// $\forall a,\forall b, \forall c \in T, (a \circ b) \circ c = a \circ (b \circ c)$
    pub trait Associative {}

    /// # 単位的
    pub trait Unital: Magma {
        /// 単位元 identity element: $e$
        fn unit() -> Self::M;
    }

    /// # 可換
    pub trait Commutative: Magma {}

    /// # 可逆的
    /// $\exists e \in T, \forall a \in T, \exists b,c \in T, b \circ a = a \circ c = e$
    pub trait Invertible: Magma {
        /// $a$ where $a \circ x = e$
        fn inv(x: &Self::M) -> Self::M;
    }

    /// # 冪等性
    pub trait Idempotent: Magma {}

    /// # 半群
    /// 1. 結合則
    pub trait SemiGroup: Magma + Associative {}

    /// # モノイド
    /// 1. 結合則
    /// 1. 単位元
    pub trait Monoid: Magma + Associative + Unital {
        /// $x^n = x\circ\cdots\circ x$
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

    /// # 可換モノイド
    pub trait CommutativeMonoid: Magma + Associative + Unital + Commutative {}

    /// # 群
    /// 1. 結合法則
    /// 1. 単位元
    /// 1. 逆元
    pub trait Group: Magma + Associative + Unital + Invertible {}

    /// # アーベル群
    pub trait AbelianGroup: Magma + Associative + Unital + Commutative + Invertible {}

    /// # Band
    /// 1. 結合法則
    /// 1. 冪等律
    pub trait Band: Magma + Associative + Idempotent {}

    impl<M: Magma + Associative> SemiGroup for M {}
    impl<M: Magma + Associative + Unital> Monoid for M {}
    impl<M: Magma + Associative + Unital + Commutative> CommutativeMonoid for M {}
    impl<M: Magma + Associative + Unital + Invertible> Group for M {}
    impl<M: Magma + Associative + Unital + Commutative + Invertible> AbelianGroup for M {}
    impl<M: Magma + Associative + Idempotent> Band for M {}

    /// # 作用付きモノイド
    pub trait MapMonoid {
        /// モノイドM
        type Mono: Monoid;
        type Func: Monoid;
        /// 値xと値yを併合する
        fn op(
            &self,
            x: &<Self::Mono as Magma>::M,
            y: &<Self::Mono as Magma>::M,
        ) -> <Self::Mono as Magma>::M {
            Self::Mono::op(x, y)
        }
        fn unit() -> <Self::Mono as Magma>::M {
            Self::Mono::unit()
        }
        /// 作用fをvalueに作用させる
        fn apply(
            &self,
            f: &<Self::Func as Magma>::M,
            value: &<Self::Mono as Magma>::M,
        ) -> <Self::Mono as Magma>::M;
        /// 作用fの単位元
        fn identity_map() -> <Self::Func as Magma>::M {
            Self::Func::unit()
        }
        /// composition:
        /// $h() = f(g())$
        fn compose(
            &self,
            f: &<Self::Func as Magma>::M,
            g: &<Self::Func as Magma>::M,
        ) -> <Self::Func as Magma>::M {
            Self::Func::op(f, g)
        }
    }

    /// # 加算の単位元
    pub trait Zero {
        fn zero() -> Self;
    }

    /// # 乗算の単位元
    pub trait One {
        fn one() -> Self;
    }

    /// # 下に有界
    pub trait BoundedBelow {
        fn min_value() -> Self;
    }

    /// # 上に有界
    pub trait BoundedAbove {
        fn max_value() -> Self;
    }

    pub trait Pow {
        fn pow(self, exp: i64) -> Self;
    }

    /// # 原始根の存在
    pub trait PrimitiveRoot {
        /// # $2^{DIVIDE_LIMIT}$乗根まで存在する
        const DIVIDE_LIMIT: usize;
        /// # 原始根
        fn primitive_root() -> Self;
    }

    pub trait TrailingZeros {
        fn trailing_zero(self) -> Self;
    }
}
