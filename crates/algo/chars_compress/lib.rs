//! # 文字列の符号化
//! 文字列を前から現れた順で数値化する

use prelude::*;

#[snippet(name = "char-compress", doc_hidden)]
pub fn compress(src: &[char]) -> Vec<usize> {
    let mut map = vec![None; 256];
    let mut c = 0;
    let mut ret = Vec::new();
    for si in src {
        if let Some(n) = map[*si as usize] {
            ret.push(n);
        } else {
            c += 1;

            map[*si as usize] = Some(c);
            ret.push(c);
        }
    }
    ret
}
