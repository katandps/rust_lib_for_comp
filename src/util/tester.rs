//! # テスト実行用関数

#[allow(unused_imports)]
use super::solver::solve;
use crate::prelude::*;

#[snippet(name = "tester", doc_hidden)]
#[test]
fn test() {
    let test_suits = vec!["0", "1"];
    for suit in test_suits {
        std::thread::Builder::new()
            .name("extend stack size".into())
            .stack_size(32 * 1024 * 1024)
            .spawn(move || {
                let mut io = IODebug::new(
                    suit,
                    false,
                    |outer: &mut ReaderFromStr, inner: &mut ReaderFromStr| {
                        inner.out(outer.v::<String>())
                    },
                );
                solve(&mut io);
                io.flush();
            })
            .unwrap()
            .join()
            .unwrap()
    }
}
