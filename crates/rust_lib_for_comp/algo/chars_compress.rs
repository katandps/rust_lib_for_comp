//! # 文字列の符号化
//! 文字列を前から現れた順で数値化する

#[codesnip::entry("char-compress")]
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

#[test]
fn test() {
    let src: Vec<_> = "abcdebbaz".chars().collect();
    let result = compress(&src);
    assert_eq!(vec![1, 2, 3, 4, 5, 2, 2, 1, 6], result);
}
