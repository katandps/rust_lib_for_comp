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
    let h: usize = read();
    let w: usize = read();
    let mut s = Vec::new();
    for _ in 0..h {
        s.push(vecchar());
    }

    let mut dp = Vec::new();
    for _ in 0..h {
        dp.push(vec![0usize; w])
    }
    if s[0][0] == '#' {
        dp[0][0] = 1;
    }
    for x in 1..w {
        dp[0][x] = if (dp[0][x - 1] % 2 == 0) ^ (s[0][x] == '.') {
            dp[0][x - 1] + 1
        } else {
            dp[0][x - 1]
        }
    }
    for y in 1..h {
        dp[y][0] = if (dp[y - 1][0] % 2 == 0) ^ (s[y][0] == '.') {
            dp[y - 1][0] + 1
        } else {
            dp[y - 1][0]
        }
    }
    // dbg!(&dp);

    for y in 1..h {
        for x in 1..w {
            let yy = if (dp[y - 1][x] % 2 == 0 && s[y][x] == '#')
                || (dp[y - 1][x] % 2 == 1 && s[y][x] == '.')
            {
                dp[y - 1][x] + 1
            } else {
                dp[y - 1][x]
            };
            let xx = if (dp[y][x - 1] % 2 == 0) && (s[y][x] == '#')
                || (dp[y][x - 1] % 2 == 1) && (s[y][x] == '.')
            {
                dp[y][x - 1] + 1
            } else {
                dp[y][x - 1]
            };
            // dbg!(&y, &x, &yy, &xx);
            dp[y][x] = min(xx, yy)
        }
    }

    //   dbg!(&s);
    // dbg!(&dp);
    println!("{}", (dp[h - 1][w - 1] + 1) / 2)
}
