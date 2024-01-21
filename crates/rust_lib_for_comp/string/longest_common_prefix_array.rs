//! LCP配列 Longest Common Prefix Array
use super::suffix_array::SuffixArray;
use crate::prelude::*;

#[codesnip::entry("longest-common-prefix-array")]
pub use longest_common_prefix_array_impl::LCPArray;
#[codesnip::entry("longest-common-prefix-array", include("suffix-array", "prelude"))]
mod longest_common_prefix_array_impl {
    use super::{Index, SuffixArray};

    #[derive(Clone, Debug)]
    pub struct LCPArray {
        pub lcp: Vec<usize>,
    }
    impl LCPArray {
        pub fn build<T: PartialEq>(sa: &SuffixArray<T>) -> Self {
            let n = sa.src.len();
            assert!(n > 0);

            let mut rank = vec![0; n + 1];
            sa.sa.iter().enumerate().for_each(|(i, sai)| rank[*sai] = i);

            let mut lcp = vec![0; n + 1];
            let mut h = 0usize;
            for i in 0..n {
                h = h.saturating_sub(1);
                if rank[i] == 0 {
                    continue;
                }
                let j = sa.sa[rank[i] - 1];
                while j + h < n && i + h < n {
                    if sa.src[j + h] != sa.src[i + h] {
                        break;
                    }
                    h += 1;
                }
                lcp[rank[i]] = h;
            }
            LCPArray { lcp }
        }
    }

    impl Index<usize> for LCPArray {
        type Output = usize;
        fn index(&self, i: usize) -> &usize {
            &self.lcp[i]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let sa = SuffixArray::build(
            &b"mississippi"
                .iter()
                .map(|&u| u as usize)
                .collect::<Vec<_>>(),
        );
        let lcp = LCPArray::build(&sa);
        assert_eq!(vec![11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2], sa.sa);
        assert_eq!(vec![0, 0, 1, 1, 4, 0, 0, 1, 0, 2, 1, 3], lcp.lcp);
    }
}
