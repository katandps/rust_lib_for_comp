//! printデバッグしにくいデータ構造を出力する(コピペ用)

use prelude::*;

#[snippet(name = "vec_print_2d", doc_hidden)]
pub fn format_2d_vec<T>(v: &[Vec<T>], digit: usize) -> String
where
    T: Display,
{
    let mut buf = String::new();
    for row in v {
        for cell in row {
            buf.push_str(&format!("{:digit$}", cell, digit = digit));
        }
        buf.push_str("\n");
    }
    buf
}

#[test]
fn test() {
    assert_eq!(
        format!("{}", format_2d_vec(&[vec![1, 2, 3,], vec![4, 5, 6]], 3)).as_str(),
        "  1  2  3\n  4  5  6\n"
    );
}
