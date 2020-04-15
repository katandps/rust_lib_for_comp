use std::io::*;
use std::str::*;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok().unwrap()
}

fn main() {
    let h: i32 = read();
    let w: i32 = read();

    let mut ans_w = '-';
    let mut ans_h = -1;
    let snuke = vec!['s', 'n', 'u', 'k', 'e'];
    for hi in 0..h {
        'inner: for wi in 0..w {
            let s: String = read();
            let v: Vec<char> = s.chars().collect();
            for i in 0..5 {
                if snuke[i] != v[i] {
                    continue 'inner;
                }
            }
            ans_w = (b'A' + wi as u8) as char;
            ans_h = hi + 1;
        }
    }
    println!("{}{}", ans_w, ans_h)
}
