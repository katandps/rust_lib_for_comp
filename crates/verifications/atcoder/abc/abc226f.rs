use rust_lib_for_comp::{
    algebra::{
        binary_operation::greatest_common_divisor::Gcd,
        mod_int::{const_mod_factorial::Factorial, ModInt},
        Magma,
    },
    enumerator::split_of_natural_number::SplitOfNumber,
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{AtCoder, Solver};

#[derive(AtCoder)]
pub struct ABC226F;
impl verify::Solver for ABC226F {
    const PROBLEM_ID: &'static str = "abc226_f";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, k) = reader.v2::<usize, usize>();
        let mut ans = ModInt::<998_244_353>::zero();
        let mvt: Factorial<998_244_353, 10010> = Factorial::new();
        for p in SplitOfNumber::from(n) {
            let mut score = ModInt::from(
                p.iter()
                    .fold(1, |a, x| a * *x as i64 / Gcd::op(&a, &(*x as i64))),
            )
            .pow(k as i64)
                * mvt.factorial(n as i64);
            let mut cnt = vec![0; 51];
            for pi in p {
                cnt[pi] += 1;
                score /= pi as i64;
            }
            for cnt in cnt.iter().take(50 + 1).skip(1) {
                if *cnt > 0 {
                    score /= mvt.factorial(*cnt);
                }
            }
            ans += score;
        }
        writeln!(write, "{ans}").unwrap()
    }
}

#[test]
fn test() {
    ABC226F::assert("2 2", "5");
    ABC226F::assert("3 3", "79");
    ABC226F::assert("50 10000", "77436607");
}
