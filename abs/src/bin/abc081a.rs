use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: Chars
    }
    println!(
        "{}",
        c.iter()
            .fold(0, |sum, x| sum + (if x == &'1' { 1 } else { 0 }))
    )
}
