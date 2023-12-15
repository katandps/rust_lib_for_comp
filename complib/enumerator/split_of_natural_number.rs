//! # 自然数の分割
//! 降順に次の分割を得るIteratorの実装
//! ## 使用方法
//! - `From<usize>`: 入力した数の分割をすべて得る
//! - `From<&[usize]>`: 入力したスライスの合計の分割を途中から得る

#[codesnip::entry("split-of-natural-number")]
pub use split_of_natural_number_impl::SplitOfNumber;
#[codesnip::entry("split-of-natural-number")]
mod split_of_natural_number_impl {
    #[derive(Clone, Debug)]
    pub struct SplitOfNumber(Option<Vec<usize>>);
    impl Iterator for SplitOfNumber {
        type Item = Vec<usize>;
        fn next(&mut self) -> Option<Vec<usize>> {
            let ret = self.0.clone();
            if let Some(v) = &mut self.0 {
                match v.iter().rposition(|&x| x != 1) {
                    None => self.0 = None,
                    Some(i) => {
                        let others = v.split_off(i);
                        let mut rest = others.iter().sum::<usize>();
                        let max = others[0] - 1;
                        while rest > 0 {
                            let next = rest.min(max);
                            v.push(next);
                            rest -= next;
                        }
                    }
                }
            } else {
                self.0 = None
            };
            ret
        }
    }
    impl From<usize> for SplitOfNumber {
        fn from(n: usize) -> Self {
            SplitOfNumber(Some(vec![n]))
        }
    }
    impl From<&[usize]> for SplitOfNumber {
        fn from(src: &[usize]) -> Self {
            SplitOfNumber(Some(src.to_vec()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res: Vec<_> = SplitOfNumber::from(5).collect();
        assert_eq!(
            res,
            vec![
                vec![5],
                vec![4, 1],
                vec![3, 2],
                vec![3, 1, 1],
                vec![2, 2, 1],
                vec![2, 1, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]
        );

        assert_eq!(204226, SplitOfNumber::from(50).count());
    }
}
