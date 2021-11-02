//! printデバッグしにくいデータ構造を出力する(コピペ用)
use crate::prelude::*;

pub fn vec_print_2d<T: Display>(v: &[Vec<T>], digit: usize) {
    for row in v {
        for cell in row {
            print!("{:digit$}", cell, digit = digit)
        }
        println!();
    }
}
