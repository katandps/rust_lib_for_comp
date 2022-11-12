//! # テスト実行用関数

#[allow(unused_imports)]
use super::solver::solve;
use crate::prelude::*;

#[snippet(name = "tester", doc_hidden)]
#[test]
fn test() {
    if let Ok(reader) = ReaderFromString::from_file("./in") {
        let stdout = stdout();
        std::thread::Builder::new()
            .name("extend stack size".into())
            .stack_size(32 * 1024 * 1024)
            .spawn(move || solve(reader, Writer::new(stdout.lock())))
            .unwrap()
            .join()
            .unwrap()
    } else {
        eprintln!("Failed to load file.")
    }
}
