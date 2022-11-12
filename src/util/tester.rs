//! # テスト実行用関数

#[allow(unused_imports)]
use super::solver::solve;
use crate::prelude::*;

#[snippet(name = "tester", doc_hidden)]
#[test]
fn test() {
    let test_suits = vec!["0", "1"];
    for suit in test_suits {
        let reader = ReaderFromString::new(suit);
        let stdout = stdout();
        std::thread::Builder::new()
            .name("extend stack size".into())
            .stack_size(32 * 1024 * 1024)
            .spawn(move || solve(reader, Writer::new(stdout.lock())))
            .unwrap()
            .join()
            .unwrap()
    }
}
