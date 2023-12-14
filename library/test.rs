pub use mod_int_impl::ModInt64;
mod mod_int_impl {
    use super::{
        Add, AddAssign, Debug, Display, Div, DivAssign, Formatter, FromStr, Mul, MulAssign, Neg,
        One, Pow, Sub, SubAssign, Sum, Zero,
    };
    use std::num::ParseIntError;
    #[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
    pub struct ModInt64<const MOD: u64 = { (1 << 61) - 1 }>(u64);
    impl<const MOD: u64> ModInt64<MOD> {
        #[doc = " # 法$N$"]
        pub const MOD: u64 = MOD;
        #[doc = " # $NN^{-1}$ \\equiv 1 \\pmod{2^32}}$ となる$N^{-1}$"]
        pub const MOD_INV: u64 = {
            let (mut n_inv, mut i) = (Self::MOD, 0);
            while i < 5 {
                n_inv = n_inv.wrapping_mul(2u64.wrapping_sub(Self::MOD.wrapping_mul(n_inv)));
                i += 1;
            }
            n_inv
        };
        #[doc = " # $2^{64} \\pmod N$"]
        #[doc = " すなわち、$1$のモンゴメリ表現"]
        pub const R: u64 = Self::MOD.wrapping_neg() % Self::MOD;
        #[doc = " # $(2^{64})^2 \\pmod N$"]
        pub const R_POW2: u64 =
            ((Self::MOD as u128).wrapping_neg() % (Self::MOD as u128)) as u64 % Self::MOD;
        #[inline]
        pub const fn new(mut n: u64) -> Self {
            if n >= Self::MOD {
                n = n.rem_euclid(Self::MOD);
            }
            Self(Self::mrmul(n, Self::R_POW2))
        }
        pub const fn one() -> Self {
            Self(Self::R)
        }
        pub const fn zero() -> Self {
            Self(0)
        }
        pub const fn add(&self, rhs: Self) -> Self {
            let mut x = self.0 + rhs.0;
            if x >= Self::MOD {
                x -= Self::MOD
            }
            Self(x)
        }
        pub const fn sub(&self, rhs: Self) -> Self {
            let x = if self.0 >= rhs.0 {
                self.0 - rhs.0
            } else {
                self.0 + Self::MOD - rhs.0
            };
            Self(x)
        }
        pub const fn mul(&self, rhs: Self) -> Self {
            Self(Self::mrmul(self.0, rhs.0))
        }
        pub const fn div(&self, rhs: Self) -> Self {
            Self::mul(self, rhs.pow(Self::MOD as i64 - 2))
        }
        pub const fn pow(mut self, mut e: i64) -> Self {
            debug_assert!(e >= 0);
            if e == 0 {
                return Self::one();
            }
            let mut t = if e & 1 == 0 { Self::R } else { self.0 };
            e >>= 1;
            while e != 0 {
                self.0 = Self::mrmul(self.0, self.0);
                if e & 1 != 0 {
                    t = Self::mrmul(t, self.0);
                }
                e >>= 1;
            }
            self.0 = t;
            self
        }
        #[doc = " # モンゴメリ表現同士の積"]
        #[doc = " $mul(ar, br) == (a * b) * r \\mod N$"]
        #[doc = ""]
        #[doc = " ## todo"]
        #[doc = " 128bit演算を使用しないようにする"]
        #[inline]
        pub const fn mrmul(ar: u64, br: u64) -> u64 {
            let t: u128 = (ar as u128) * (br as u128);
            let (t, f) = ((t >> 64) as u64).overflowing_sub(
                ((((t as u64).wrapping_mul(Self::MOD_INV) as u128) * Self::MOD as u128) >> 64)
                    as u64,
            );
            if f {
                t.wrapping_add(Self::MOD)
            } else {
                t
            }
        }
        #[doc = " # モンゴメリ表現 $AR$ から $A$の復元"]
        #[doc = " return $a \\frac R \\mod N$"]
        #[inline]
        pub const fn reduce(self) -> u64 {
            let (t, f) = (((((self.0.wrapping_mul(Self::MOD_INV)) as u128) * (Self::MOD as u128))
                >> 64) as u64)
                .overflowing_neg();
            if f {
                t.wrapping_add(Self::MOD)
            } else {
                t
            }
        }
    }
    #[doc = " # 累乗"]
    #[doc = " ## 計算量"]
    #[doc = " $M$を法として $ O(\\log M) $"]
    impl<const MOD: u64> Pow for ModInt64<MOD> {
        #[inline]
        fn pow(self, e: i64) -> Self {
            Self::pow(self, e)
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> Add<Rhs> for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn add(self, rhs: Rhs) -> Self {
            Self::add(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> AddAssign<Rhs> for ModInt64<MOD> {
        #[inline]
        fn add_assign(&mut self, rhs: Rhs) {
            self.0 = Self::add(self, rhs.into()).0
        }
    }
    impl<const MOD: u64> Neg for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn neg(self) -> Self {
            Self::zero() - self
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> Sub<Rhs> for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: Rhs) -> Self {
            Self::sub(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> SubAssign<Rhs> for ModInt64<MOD> {
        #[inline]
        fn sub_assign(&mut self, rhs: Rhs) {
            self.0 = Self::sub(self, rhs.into()).0
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> Mul<Rhs> for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: Rhs) -> Self {
            Self::mul(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> MulAssign<Rhs> for ModInt64<MOD> {
        #[inline]
        fn mul_assign(&mut self, rhs: Rhs) {
            self.0 = Self::mul(self, rhs.into()).0
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> Div<Rhs> for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn div(self, rhs: Rhs) -> Self {
            Self::div(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> DivAssign<Rhs> for ModInt64<MOD> {
        #[inline]
        fn div_assign(&mut self, rhs: Rhs) {
            self.0 = Self::div(self, rhs.into()).0
        }
    }
    impl<const MOD: u64> Display for ModInt64<MOD> {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl<const MOD: u64> Debug for ModInt64<MOD> {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl<const MOD: u64> Sum for ModInt64<MOD> {
        #[inline]
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |x, a| x + a)
        }
    }
    impl<const MOD: u64> FromStr for ModInt64<MOD> {
        type Err = ParseIntError;
        #[inline]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(s.parse::<u64>()?))
        }
    }
    macro_rules!impl_integral{($($ty:ty),*)=>{$(impl<const MOD:u64>From<$ty>for ModInt64<MOD>{#[inline]fn from(i:$ty)->Self{Self::new((i as i128).rem_euclid(Self::MOD as i128)as u64)}})*};}
    impl_integral!(i32, i64, i128, isize, u32, u64, u128, usize);
    impl<const MOD: u64> From<ModInt64<MOD>> for i64 {
        #[inline]
        fn from(m: ModInt64<MOD>) -> Self {
            m.reduce() as i64
        }
    }
    impl<const MOD: u64> Zero for ModInt64<MOD> {
        #[inline]
        fn zero() -> Self {
            Self::zero()
        }
    }
    impl<const MOD: u64> One for ModInt64<MOD> {
        #[inline]
        fn one() -> Self {
            Self::one()
        }
    }
}
// codesnip-guard: algebra
pub use algebra_traits::{
    AbelianGroup, Associative, Band, BoundedAbove, BoundedBelow, Commutative, CommutativeMonoid,
    Group, Idempotent, Integral, Invertible, LeastSignificantBit, Magma, MapMonoid, Monoid,
    MonoidOperation, One, Pow, PrimitiveRoot, SemiGroup, TrailingZeros, Unital, Zero,
};
mod algebra_traits {
    use super::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Debug,
        Display, Div, DivAssign, Mul, MulAssign, Not, Product, Rem, RemAssign, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign, Sum,
    };
    #[doc = " # マグマ"]
    #[doc = " 二項演算: $M \\circ M \\to M$"]
    pub trait Magma {
        #[doc = " マグマを構成する集合$M$"]
        type M: Clone + PartialEq + Debug;
        #[doc = " マグマを構成する演算$op$"]
        fn op(x: &Self::M, y: &Self::M) -> Self::M;
        fn op_rev(x: &Self::M, y: &Self::M) -> Self::M {
            Self::op(y, x)
        }
    }
    #[doc = " # 結合則"]
    #[doc = " $\\forall a,\\forall b, \\forall c \\in T, (a \\circ b) \\circ c = a \\circ (b \\circ c)$"]
    pub trait Associative {}
    #[doc = " # 単位的"]
    pub trait Unital: Magma {
        #[doc = " 単位元 identity element: $e$"]
        fn unit() -> Self::M;
    }
    #[doc = " # 可換"]
    pub trait Commutative: Magma {}
    #[doc = " # 可逆的"]
    #[doc = " $\\exists e \\in T, \\forall a \\in T, \\exists b,c \\in T, b \\circ a = a \\circ c = e$"]
    pub trait Invertible: Magma {
        #[doc = " $a$ where $a \\circ x = e$"]
        fn inv(x: &Self::M) -> Self::M;
    }
    #[doc = " # 冪等性"]
    pub trait Idempotent: Magma {}
    #[doc = " # 半群"]
    #[doc = " 1. 結合則"]
    pub trait SemiGroup: Magma + Associative {}
    #[doc = " # モノイド"]
    #[doc = " 1. 結合則"]
    #[doc = " 1. 単位元"]
    pub trait Monoid: Magma + Associative + Unital {
        #[doc = " $x^n = x\\circ\\cdots\\circ x$"]
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
    #[doc = " # 可換モノイド"]
    pub trait CommutativeMonoid: Magma + Associative + Unital + Commutative {}
    #[doc = " # 群"]
    #[doc = " 1. 結合法則"]
    #[doc = " 1. 単位元"]
    #[doc = " 1. 逆元"]
    pub trait Group: Magma + Associative + Unital + Invertible {}
    #[doc = " # アーベル群"]
    pub trait AbelianGroup: Magma + Associative + Unital + Commutative + Invertible {}
    #[doc = " # Band"]
    #[doc = " 1. 結合法則"]
    #[doc = " 1. 冪等律"]
    pub trait Band: Magma + Associative + Idempotent {}
    impl<M: Magma + Associative> SemiGroup for M {}
    impl<M: Magma + Associative + Unital> Monoid for M {}
    impl<M: Magma + Associative + Unital + Commutative> CommutativeMonoid for M {}
    impl<M: Magma + Associative + Unital + Invertible> Group for M {}
    impl<M: Magma + Associative + Unital + Commutative + Invertible> AbelianGroup for M {}
    impl<M: Magma + Associative + Idempotent> Band for M {}
    #[doc = " # 作用モノイド"]
    #[doc = " 作用で、その合成がモノイドをなすもの"]
    pub trait MonoidOperation: Magma + Associative + Unital {
        type V: Clone + Debug;
        fn apply(&self, f: &Self::M, value: &Self::V) -> Self::V;
    }
    #[doc = " # 作用モノイド付きモノイド"]
    pub trait MapMonoid {
        #[doc = " モノイドM"]
        type Mono: Monoid;
        type Func: Monoid;
        #[doc = " 値xと値yを併合する"]
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
        #[doc = " 作用fをvalueに作用させる"]
        fn apply(
            &self,
            f: &<Self::Func as Magma>::M,
            value: &<Self::Mono as Magma>::M,
        ) -> <Self::Mono as Magma>::M;
        #[doc = " 作用fの単位元"]
        fn identity_map() -> <Self::Func as Magma>::M {
            Self::Func::unit()
        }
        #[doc = " composition:"]
        #[doc = " $h() = f(g())$"]
        fn compose(
            &self,
            f: &<Self::Func as Magma>::M,
            g: &<Self::Func as Magma>::M,
        ) -> <Self::Func as Magma>::M {
            Self::Func::op(f, g)
        }
    }
    #[doc = " # 加算の単位元"]
    pub trait Zero {
        fn zero() -> Self;
    }
    #[doc = " # 乗算の単位元"]
    pub trait One {
        fn one() -> Self;
    }
    #[doc = " # 下に有界"]
    pub trait BoundedBelow {
        fn min_value() -> Self;
    }
    #[doc = " # 上に有界"]
    pub trait BoundedAbove {
        fn max_value() -> Self;
    }
    pub trait Pow {
        fn pow(self, exp: i64) -> Self;
    }
    #[doc = " # 原始根の存在"]
    pub trait PrimitiveRoot {
        #[doc = " # $2^{DIVIDE_LIMIT}$乗根まで存在する"]
        const DIVIDE_LIMIT: usize;
        #[doc = " # 原始根"]
        fn primitive_root() -> Self;
    }
    #[doc = " # 二進数表記したとき最後尾につく0の数"]
    pub trait TrailingZeros {
        fn trailing_zero(self) -> Self;
    }
    #[doc = " # 最下位bit"]
    pub trait LeastSignificantBit {
        fn lsb(self) -> Self;
    }
    pub trait Integral:
        'static
        + Send
        + Sync
        + Copy
        + Ord
        + Display
        + Debug
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
        + Not<Output = Self>
        + Shl<Output = Self>
        + Shr<Output = Self>
        + BitOrAssign
        + BitAndAssign
        + BitXorAssign
        + ShlAssign
        + ShrAssign
        + Zero
        + One
        + BoundedBelow
        + BoundedAbove
        + TrailingZeros
        + LeastSignificantBit
    {
    }
    macro_rules!impl_integral{($($ty:ty),*)=>{$(impl Zero for$ty{#[inline]fn zero()->Self{0}}impl One for$ty{#[inline]fn one()->Self{1}}impl BoundedBelow for$ty{#[inline]fn min_value()->Self{Self::min_value()}}impl BoundedAbove for$ty{#[inline]fn max_value()->Self{Self::max_value()}}impl TrailingZeros for$ty{#[inline]fn trailing_zero(self)->Self{self.trailing_zeros()as$ty}}impl LeastSignificantBit for$ty{#[inline]fn lsb(self)->Self{if self==0{0}else{self&!(self-1)}}}impl Integral for$ty{})*};}
    impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
}
pub use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{hash_map::RandomState, BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    convert::Infallible,
    convert::{TryFrom, TryInto},
    default::Default,
    fmt::{Debug, Display, Formatter},
    hash::{BuildHasherDefault, Hash, Hasher},
    io::{stdin, stdout, BufRead, BufWriter, Read, StdoutLock, Write},
    iter::{repeat, FromIterator, Product, Sum},
    marker::PhantomData,
    mem::swap,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Bound,
        Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Range,
        RangeBounds, RangeInclusive, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
    str::{from_utf8, FromStr},
};
