//! # エラトステネスの篩

use crate::prelude::*;

#[snippet(name = "sieve-of-eratosthenes", doc_hidden)]
pub fn primes(m: usize) -> Vec<usize> {
    if m < 2 {
        return Vec::new();
    }
    if m == 2 {
        return vec![2];
    }
    let mut b = vec![false; m + 1];
    let mut ret = vec![2, 3];
    let mut i = 5;
    let mut f = 4;
    while i <= m {
        if !b[i] {
            ret.push(i);
            for j in i..m / i + 1 {
                b[i * j] = true;
            }
        }
        f = 6 - f;
        i += f;
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let p = primes(20);

        assert_eq!(p.len(), 8);
        assert_eq!(p[0], 2);
        assert_eq!(p[1], 3);
        assert_eq!(p[2], 5);
        assert_eq!(p[3], 7);
        assert_eq!(p[4], 11);
        assert_eq!(p[5], 13);
        assert_eq!(p[6], 17);
        assert_eq!(p[7], 19);
    }
}
