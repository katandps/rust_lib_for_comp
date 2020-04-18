use std::cmp::*;
use std::collections::{HashSet, VecDeque};
use std::io::*;
use std::num::*;
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

fn string() -> String {
    read()
}

fn int() -> i64 {
    read()
}

fn char() -> char {
    read::<String>().pop().unwrap()
}

fn vecchar() -> Vec<char> {
    string().chars().collect()
}

fn vecint(n: i64) -> Vec<i64> {
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(int())
    }
    vec
}

fn main() {
    let (h, w) = (int() as usize, int() as usize);
    let mut a = Vec::new();
    for _ in 0..h {
        let mut ai = Vec::new();
        for _ in 0..w {
            ai.push(int())
        }
        a.push(ai)
    }

    let mut ah1 = hw(h, w, 250_000_000i64);
    let mut ahw = hw(h, w, 250_000_000i64);
    let mut a1w = hw(h, w, 250_000_000i64);
    ah1[h - 1][0] = 0;
    ahw[h - 1][w - 1] = 0;
    a1w[0][w - 1] = 0;

    ah1 = bfs(&a, h, w, ah1, (h - 1, 0usize));
    ahw = bfs(&a, h, w, ahw, (h - 1, w - 1));
    a1w = bfs(&a, h, w, a1w, (0usize, w - 1));

    let mut ans = 250_000_000i64;
    for y in 0..h {
        for x in 0..w {
            ans = min(ans, ah1[y][x] + ahw[y][x] + a1w[y][x] - 2 * a[y][x])
        }
    }
    println!("{}", ans)
}

fn bfs(
    a: &Vec<Vec<i64>>,
    h: usize,
    w: usize,
    mut res: Vec<Vec<i64>>,
    start: (usize, usize),
) -> Vec<Vec<i64>> {
    let mut queue = VecDeque::new();
    queue.push_front(start);
    while queue.len() > 0 {
        let (y, x) = queue.pop_front().unwrap();
        if x > 0 && res[y][x - 1] > res[y][x] + a[y][x - 1] {
            res[y][x - 1] = res[y][x] + a[y][x - 1];
            queue.push_back((y, x - 1))
        }
        if x < w - 1 && res[y][x + 1] > res[y][x] + a[y][x + 1] {
            res[y][x + 1] = res[y][x] + a[y][x + 1];
            queue.push_back((y, x + 1))
        }
        if y > 0 && res[y - 1][x] > res[y][x] + a[y - 1][x] {
            res[y - 1][x] = res[y][x] + a[y - 1][x];
            queue.push_back((y - 1, x))
        }
        if y < h - 1 && res[y + 1][x] > res[y][x] + a[y + 1][x] {
            res[y + 1][x] = res[y][x] + a[y + 1][x];
            queue.push_back((y + 1, x))
        }
    }
    res
}

fn hw(h: usize, w: usize, initial: i64) -> Vec<Vec<i64>> {
    let mut d = Vec::new();
    for _ in 0..h {
        let mut di = Vec::new();
        for _ in 0..w {
            di.push(initial)
        }
        d.push(di)
    }
    d
}
