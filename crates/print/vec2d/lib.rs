//! printデバッグしにくいデータ構造を出力する(コピペ用)

use prelude::*;

#[snippet(name = "vec_print_2d", doc_hidden)]
pub fn vec_print_2d<T: Display>(v: &[Vec<T>], digit: usize) {
    for row in v {
        for cell in row {
            print!("{:digit$}", cell, digit = digit)
        }
        println!();
    }
}
