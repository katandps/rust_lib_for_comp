//! 代数
use crate::prelude::*;

pub mod binary_operation;
pub mod chinese_remainder_theorem;
pub mod complete_permutations;
pub mod complex_number;
pub mod lucas_theorem;
pub mod matrix;
pub mod miller_rabin;
pub mod mod_int;
pub mod mod_inv;
pub mod mod_pow;
pub mod mod_val_table;
pub mod sieve_of_eratosthenes;
pub mod xor_basis;

#[snippet(name = "algebra", doc_hidden)]
/// マグマ
/// 二項演算: $M \circ M \to M$
pub trait Magma {
    /// マグマを構成する集合$M$
    type M: Clone + PartialEq + Debug;
    /// マグマを構成する演算$op$
    fn op(x: &Self::M, y: &Self::M) -> Self::M;
}

#[snippet(name = "algebra", doc_hidden)]
/// 結合則
/// $\forall a,\forall b, \forall c \in T, (a \circ b) \circ c = a \circ (b \circ c)$
pub trait Associative {}

#[snippet(name = "algebra", doc_hidden)]
/// 単位的
pub trait Unital: Magma {
    /// 単位元 identity element: $e$
    fn unit() -> Self::M;
}

#[snippet(name = "algebra", doc_hidden)]
/// 可換
pub trait Commutative: Magma {}

#[snippet(name = "algebra", doc_hidden)]
/// 可逆的
/// $\exists e \in T, \forall a \in T, \exists b,c \in T, b \circ a = a \circ c = e$
pub trait Invertible: Magma {
    /// $a$ where $a \circ x = e$
    fn inv(x: &Self::M) -> Self::M;
}

#[snippet(name = "algebra", doc_hidden)]
/// 冪等性
pub trait Idempotent: Magma {}

#[snippet(name = "algebra", doc_hidden)]
/// 半群
/// 1. 結合則
pub trait SemiGroup: Magma + Associative {}
impl<M: Magma + Associative> SemiGroup for M {}

#[snippet(name = "algebra", doc_hidden)]
/// モノイド
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
#[snippet(name = "algebra", doc_hidden)]
impl<M: Magma + Associative + Unital> Monoid for M {}

#[snippet(name = "algebra", doc_hidden)]
/// 可換モノイド
pub trait CommutativeMonoid: Magma + Associative + Unital + Commutative {}
#[snippet(name = "algebra", doc_hidden)]
impl<M: Magma + Associative + Unital + Commutative> CommutativeMonoid for M {}

#[snippet(name = "algebra", doc_hidden)]
/// 群
/// 1. 結合法則
/// 1. 単位元
/// 1. 逆元
pub trait Group: Magma + Associative + Unital + Invertible {}
#[snippet(name = "algebra", doc_hidden)]
impl<M: Magma + Associative + Unital + Invertible> Group for M {}

#[snippet(name = "algebra", doc_hidden)]
/// アーベル群
pub trait AbelianGroup: Magma + Associative + Unital + Commutative + Invertible {}
#[snippet(name = "algebra", doc_hidden)]
impl<M: Magma + Associative + Unital + Commutative + Invertible> AbelianGroup for M {}

#[snippet(name = "algebra", doc_hidden)]
/// Band
/// 1. 結合法則
/// 1. 冪等律
pub trait Band: Magma + Associative + Idempotent {}
#[snippet(name = "algebra", doc_hidden)]
impl<M: Magma + Associative + Idempotent> Band for M {}

#[snippet(name = "algebra", doc_hidden)]
/// 作用付きモノイド
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

#[snippet(name = "algebra", doc_hidden)]
/// 加算の単位元
pub trait Zero {
    fn zero() -> Self;
}

#[snippet(name = "algebra", doc_hidden)]
/// 乗算の単位元
pub trait One {
    fn one() -> Self;
}

#[snippet(name = "algebra", doc_hidden)]
/// 下に有界
pub trait BoundedBelow {
    fn min_value() -> Self;
}

#[snippet(name = "algebra", doc_hidden)]
/// 上に有界
pub trait BoundedAbove {
    fn max_value() -> Self;
}
