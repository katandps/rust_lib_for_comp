#[allow(unused_imports)]
use lucas_theorem::*;

#[allow(dead_code)]
mod lucas_theorem {
    /// nCr mod p を得る 計算量: O(logN)
    pub fn lucas_theorem(mut n: usize, mut r: usize, p: usize) -> usize {
        if p < 2 {
            return 0;
        }
        let mut ret = 1;
        while n != 0 || r != 0 {
            let (n_mod, r_mod) = (n % p, r % p);
            if n_mod >= r_mod {
                ret *= combination(n_mod, r_mod);
            } else {
                return 0;
            }
            n /= p;
            r /= p;
        }
        ret % p
    }

    pub fn combination(n: usize, k: usize) -> usize {
        if n < k {
            return 0;
        }
        if std::cmp::min(k, n - k) == 0 {
            1
        } else {
            combination(n - 1, k - 1) * n / k
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let c5_0 = lucas_theorem(5, 0, 3);
        let c5_1 = lucas_theorem(5, 1, 3);
        let c5_2 = lucas_theorem(5, 2, 3);
        let c5_3 = lucas_theorem(5, 3, 3);
        let c5_4 = lucas_theorem(5, 4, 3);
        let c5_5 = lucas_theorem(5, 5, 3);

        assert_eq!(1 % 3, c5_0);
        assert_eq!(5 % 3, c5_1);
        assert_eq!(10 % 3, c5_2);
        assert_eq!(10 % 3, c5_3);
        assert_eq!(5 % 3, c5_4);
        assert_eq!(1 % 3, c5_5);

        assert_eq!(35, combination(7, 3));

        assert_eq!(1 % 3, lucas_theorem(7, 0, 3));
        assert_eq!(7 % 3, lucas_theorem(7, 1, 3));
        assert_eq!(21 % 3, lucas_theorem(7, 2, 3));
        assert_eq!(35 % 3, lucas_theorem(7, 3, 3));
        assert_eq!(35 % 3, lucas_theorem(7, 4, 3));
        assert_eq!(21 % 3, lucas_theorem(7, 5, 3));
        assert_eq!(7 % 3, lucas_theorem(7, 6, 3));
        assert_eq!(1 % 3, lucas_theorem(7, 7, 3));
    }
}
