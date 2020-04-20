use input::*;
use std::cmp::*;
use std::io::*;
use std::num::*;
use std::str::*;

mod input {
    use super::*;

    pub fn read<T: FromStr>() -> T {
        stdin()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<T>()
            .ok()
            .unwrap()
    }

    pub fn string() -> String {
        read()
    }

    pub fn int() -> i64 {
        read()
    }

    pub fn char() -> char {
        read::<String>().pop().unwrap()
    }

    pub fn vecchar() -> Vec<char> {
        string().chars().collect()
    }

    pub fn vecint(n: i64) -> Vec<i64> {
        let mut vec = Vec::new();
        for i in 0..n {
            vec.push(int())
        }
        vec
    }
}

fn main() {
    let (h, w) = (int() as usize, int() as usize);
    let mut a = Vec::new();
    for _ in 0..h {
        a.push(vecchar());
    }
    let mut count: Vec<usize> = vec![0; 26];
    for row in a {
        for ch in row {
            count[ch as usize - 'a' as usize] += 1;
        }
    }
    count.sort();
    if h % 2 == 0 && w % 2 == 0 {
        for c in count {
            if c % 4 != 0 {
                println!("No");
                return;
            }
        }
        println!("Yes");
    } else if h % 2 == 0 {
        let mut v = 0;
        for c in count {
            if c % 4 != 0 {
                if c % 2 != 0 {
                    println!("No");
                    return;
                }
                v += 1;
            }
        }
        if v <= h / 2 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else if w % 2 == 0 {
        let mut v = 0;
        for c in count {
            if c % 4 != 0 {
                if c % 2 != 0 {
                    println!("No");
                    return;
                }
                v += 1;
            }
        }
        if v <= w / 2 {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        let mut two = 0;
        let mut one = 0;
        for c in count {
            if c % 4 != 0 {
                match c % 4 {
                    1 => {
                        one += 1;
                    }
                    2 => {
                        two += 1;
                    }
                    3 => {
                        one += 1;
                        two += 1;
                    }
                    _ => (),
                }
            }
        }
        if one > 1 || two > (h - 1) / 2 + (w - 1) / 2 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
