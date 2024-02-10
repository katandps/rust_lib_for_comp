use rust_lib_for_comp::{
    algo::{fxhasher::HashMap, union_of_ranges::unite},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{AtCoder, Solver};

#[derive(AtCoder)]
pub struct ABC147F;
impl verify::Solver for ABC147F {
    const PROBLEM_ID: &'static str = "abc147_f";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, mut x, mut d) = reader.v3::<i64, i64, i64>();
        if d == 0 {
            return writeln!(write, "{}", if x == 0 { 1 } else { n + 1 }).unwrap();
        }
        if d < 0 {
            x = -x;
            d = -d;
        }

        let mut map = HashMap::default();
        for i in 0..=n {
            map.entry((x * i).rem_euclid(d)).or_insert(Vec::new()).push(
                (x * i).div_euclid(d) + i * (i - 1) / 2
                    ..=(x * i).div_euclid(d) + n * (n - 1) / 2 - (n - i) * (n - i - 1) / 2,
            );
        }
        let mut ans = 0;
        for (_, v) in map {
            let v = unite(&v);
            for r in v {
                ans += r.count();
            }
        }
        writeln!(write, "{ans}").unwrap()
    }
}

#[test]
fn test() {
    ABC147F::assert("3 4 2", "8");
    ABC147F::assert("2 3 -3", "2");
    ABC147F::assert("100 14 20", "49805");
    ABC147F::assert("3 0 0", "1");
    ABC147F::assert("3 1 0", "4");
}
