//! 剰余体
use crate::algebra::Zero;
use crate::prelude::*;
pub use mod1000000007::{mi, Mi};

pub mod mod1000000007;
pub mod mod1000000009;
pub mod mod998244353;

#[snippet(name = "mod-int", doc_hidden)]
pub trait Mod: Copy + Clone + Debug {
    fn get() -> i64;
}

#[snippet(name = "mod-int", doc_hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModInt<M: Mod>(i64, PhantomData<fn() -> M>);

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> ModInt<M> {
    pub fn new(mut n: i64) -> Self {
        if n < 0 || n >= M::get() {
            n = n.rem_euclid(M::get());
        }
        Self(n, PhantomData)
    }

    /// # 累乗
    /// ## 計算量
    /// $`O(\logMOD)`$
    pub fn pow(mut self, mut e: i64) -> ModInt<M> {
        let m = e < 0;
        e = e.abs();
        let mut result = Self::new(1);
        while e > 0 {
            if e & 1 == 1 {
                result *= self.0;
            }
            e >>= 1;
            self *= self.0;
        }
        if m {
            Self::new(1) / result
        } else {
            result
        }
    }

    /// # 組み合わせnCr
    /// 前計算なし
    /// ## 計算量
    /// $`O(r + \logMOD)`$
    pub fn comb(n: i64, mut r: i64) -> Mi {
        if r > n - r {
            r = n - r;
        }
        if r == 0 {
            return mi(1);
        }
        let mut ret = mi(1);
        let mut rev = mi(1);
        for k in 0..r {
            ret *= n - k;
            rev *= r - k;
        }
        ret / rev
    }

    pub fn get(self) -> i64 {
        self.0
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Add<i64> for ModInt<M> {
    type Output = Self;
    fn add(self, rhs: i64) -> Self {
        self + ModInt::new(rhs)
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Add<ModInt<M>> for ModInt<M> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> AddAssign<i64> for ModInt<M> {
    fn add_assign(&mut self, rhs: i64) {
        *self = *self + rhs
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> AddAssign<ModInt<M>> for ModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = if self.0 + rhs.0 >= M::get() {
            self.0 + rhs.0 - M::get()
        } else {
            self.0 + rhs.0
        }
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.0)
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Sub<i64> for ModInt<M> {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self {
        self - ModInt::new(rhs)
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Sub<ModInt<M>> for ModInt<M> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> SubAssign<i64> for ModInt<M> {
    fn sub_assign(&mut self, rhs: i64) {
        *self = *self - rhs
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> SubAssign<ModInt<M>> for ModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = if self.0 >= rhs.0 {
            self.0 - rhs.0
        } else {
            self.0 - rhs.0 + M::get()
        }
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Mul<i64> for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        ModInt::new(self.0 * (rhs % M::get()))
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Mul<ModInt<M>> for ModInt<M> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self * rhs.0
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> MulAssign<i64> for ModInt<M> {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> MulAssign<ModInt<M>> for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Div<i64> for ModInt<M> {
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        self * ModInt::new(rhs).pow(M::get() - 2)
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Div<ModInt<M>> for ModInt<M> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self / rhs.0
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> DivAssign<i64> for ModInt<M> {
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> DivAssign<ModInt<M>> for ModInt<M> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Display for ModInt<M> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Debug for ModInt<M> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Deref for ModInt<M> {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> DerefMut for ModInt<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |x, a| x + a)
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> From<i64> for ModInt<M> {
    fn from(i: i64) -> Self {
        Self::new(i)
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> From<ModInt<M>> for i64 {
    fn from(m: ModInt<M>) -> Self {
        m.0
    }
}

#[snippet(name = "mod-int", doc_hidden)]
impl<M: Mod> Zero for ModInt<M> {
    fn zero() -> Self {
        Self::new(0)
    }
}

#[snippet(name = "pow-table", doc_hidden)]
#[derive(Clone, Debug, Default)]
/// # 2のべき乗を都度生成するDefaultDict
pub struct PowTable(HashMap<i64, Mi>);
impl PowTable {
    pub fn pow(&mut self, e: i64) -> Mi {
        *self.0.entry(e).or_insert_with(|| mi(2).pow(e))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::distributions::{Distribution, Uniform};

    const MOD: i64 = 1_000_000_007;

    #[test]
    fn neg_test() {
        assert_eq!((Mi::new(0) - 1_000_000).get(), (Mi::new(-1_000_000)).get());
    }

    #[test]
    fn random_add_sub() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let mx = Mi::new(x);
            let my = Mi::new(y);

            assert_eq!((mx + my).get(), (x + y) % MOD);
            assert_eq!((mx + y).get(), (x + y) % MOD);
            assert_eq!((mx - my).get(), (x + MOD - y) % MOD);
            assert_eq!((mx - y).get(), (x + MOD - y) % MOD);

            let mut x = x;
            let mut mx = mx;
            x += y;
            mx += my;
            assert_eq!(mx.get(), x % MOD);

            mx += y;
            x += y;
            assert_eq!(mx.get(), x % MOD);

            mx -= my;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.get(), x);

            mx -= y;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.get(), x);
        }
    }

    #[test]
    fn random_mul() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let mx = Mi::new(x);
            let my = Mi::new(y);

            assert_eq!((mx * my).get(), (x * y) % MOD);
            assert_eq!((mx * y).get(), (x * y) % MOD);
        }
    }

    #[test]
    fn zero_test() {
        let a = Mi::new(1_000_000_000);
        let b = Mi::new(7);
        let c = a + b;
        assert_eq!(c.get(), 0);
    }

    #[test]
    fn pow_test() {
        let a = Mi::new(3);
        let a = a.pow(4);
        assert_eq!(a.get(), 81);
    }

    #[test]
    fn div_test() {
        for i in 1..100000 {
            let mut a = Mi::new(1);
            a /= i;
            a *= i;
            assert_eq!(a.get(), 1);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!((Mi::new(MOD + 1)).get(), 1);
        assert_eq!((Mi::new(std::i64::MAX) + 1).get(), 291172004);
        assert_eq!((Mi::new(1_000_000_000) * std::i64::MAX).get(), 961796000);
        assert_eq!((Mi::new(1_000_000_000) + std::i64::MAX).get(), 291171996);
        assert_eq!((Mi::new(1_000_000_000) - std::i64::MAX).get(), 708827997);
        assert_eq!(
            (Mi::new(1_000_000_000) / std::i64::MAX * std::i64::MAX).get(),
            1_000_000_000
        );

        let mut a = Mi::new(1_000_000_000);
        a *= std::i64::MAX;
        assert_eq!(a.get(), 961796000);

        let mut a = Mi::new(1_000_000_000);
        a += std::i64::MAX;
        assert_eq!(a.get(), 291171996);

        let mut a = Mi::new(1_000_000_000);
        a -= std::i64::MAX;
        assert_eq!(a.get(), 708827997);

        let mut a = Mi::new(1_000_000_000);
        a /= std::i64::MAX;
        assert_eq!((a * std::i64::MAX).get(), 1_000_000_000);
    }
}
