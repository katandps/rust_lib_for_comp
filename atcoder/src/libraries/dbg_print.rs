#[allow(unused_imports)]
use dbg_print::*;

#[allow(dead_code)]
mod dbg_print {
    pub fn vec_print_2d<T: std::fmt::Display>(v: &Vec<Vec<T>>, digit: usize) {
        for row in v {
            for cell in row {
                print!("{:digit$}", cell, digit = digit)
            }
            println!();
        }
    }
}
