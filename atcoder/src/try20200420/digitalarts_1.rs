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
    let s = stdin()
        .bytes()
        .map(|c| c.unwrap() as char)
        .take_while(|c| c != &'\n')
        .collect::<Vec<char>>();
    let n = int() as usize;
    let mut t = Vec::new();
    for _ in 0..n {
        t.push(vecchar());
    }

    let mut buf = Vec::new();
    for i in 0..s.len() {
        if s[i] != ' ' {
            buf.push(s[i]);
            continue;
        }
        let mut output = true;
        for t_i in &t {
            if t_i.len() != buf.len() {
                continue;
            }
            let mut match_word = true;
            for j in 0..t_i.len() {
                if t_i[j] != '*' && t_i[j] != buf[j] {
                    match_word = false;
                }
            }
            if match_word {
                output = false;
            }
        }
        for b in buf {
            print!("{}", if output { b } else { '*' });
        }
        print!(" ");
        buf = Vec::new();
    }
    let mut output = true;
    for t_i in &t {
        if t_i.len() != buf.len() {
            continue;
        }
        let mut match_word = true;
        for j in 0..t_i.len() {
            if t_i[j] != '*' && t_i[j] != buf[j] {
                match_word = false;
            }
        }
        if match_word {
            output = false;
        }
    }
    for b in buf {
        print!("{}", if output { b } else { '*' });
    }
    println!("");
}
