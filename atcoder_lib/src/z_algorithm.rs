#[allow(unused_imports)]
use z_algorithm::*;

#[allow(dead_code)]
mod z_algorithm {
    /// 文字列SとSのi文字目から始まる文字列の共通部分列の長さをO(N)で求める
    pub fn z(s: &Vec<char>) -> Vec<usize> {
        let mut c = 0;
        let n = s.len();
        let mut z = vec![0; n];
        for i in 1..n {
            let l = i - c;
            if i + z[l] < c + z[c] {
                z[i] = z[l];
            } else {
                let mut j = if c + z[c] > i { c + z[c] - i } else { 0 };
                while i + j < n && s[j] == s[i + j] {
                    j += 1;
                }
                z[i] = j;
                c = i;
            }
        }
        z[0] = n;
        z
    }
}
