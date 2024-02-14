use rust_lib_for_comp::{
    algebra::pollard_rho::PollardRho,
    util::{
        io_util::{ReadHelper, ReaderTrait},
        string_util::JoinTrait,
    },
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct Factorize;
impl verify::Solver for Factorize {
    const PROBLEM_ID: &'static str = "factorize";
    const TIME_LIMIT_MILLIS: u64 = 10000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        for _ in 0..n {
            let p = reader.v::<u64>().prime_factorize();
            writeln!(write, "{} {}\n", p.len(), p.join(" ")).unwrap();
        }
    }
}
#[test]
fn test() {
    Factorize::assert(
        "10
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10",
        "0
        1 2
        1 3
        2 2 2
        1 5
        2 2 3
        1 7
        3 2 2 2
        2 3 3
        2 2 5",
    );
}
