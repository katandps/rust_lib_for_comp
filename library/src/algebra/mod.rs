//! 代数

pub mod all_combination;
pub mod all_permutation;
pub mod chinese_remainder_theorem;
pub mod greatest_common_divisor;
pub mod lucas_theorem;
pub mod matrix;
pub mod mod_int;
pub mod mod_inv;
pub mod mod_pow;
pub mod mod_val_table;
pub mod sieve_of_eratosthenes;

/// マグマ
/// 二項演算: $`M \circ M \to M`$
pub trait Magma {
    /// マグマを構成する集合$`M`$
    type M: Clone + PartialEq;
    /// マグマを構成する演算$`op`$
    fn op(&self, x: &Self::M, y: &Self::M) -> Self::M;
}
/// 結合則
/// $`\forall a,\forall b, \forall c \in T, (a \circ b) \circ c = a \circ (b \circ c)`$
pub trait Associative {}
/// 半群
pub trait SemiGroup: Magma + Associative {}
/// 単位的
pub trait Unital: Magma {
    /// 単位元 identity element: $`e`$
    fn unit(&self) -> Self::M;
}
/// モノイド
/// 結合則と、単位元を持つ
pub trait Monoid: SemiGroup + Unital {
    /// $`x^n = x\circ\cdots\circ x`$
    fn pow(&self, x: Self::M, mut n: usize) -> Self::M {
        let mut res = self.unit();
        let mut base = x;
        while n > 0 {
            if n & 1 == 1 {
                res = self.op(&res, &base);
            }
            base = self.op(&base, &base);
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
