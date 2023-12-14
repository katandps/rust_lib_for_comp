//! 接尾辞配列

use compress::Compress;
use prelude::*;

#[codesnip::entry("suffix-array", doc_hidden)]
pub use suffix_array_impl::SuffixArray;
#[codesnip::entry("suffix-array", doc_hidden)]
mod suffix_array_impl {
    use super::{BTreeMap, Compress, Index};

    #[derive(Debug, Clone)]
    pub struct SuffixArray<T> {
        pub sa: Vec<usize>,
        pub src: Vec<T>,
    }
    impl<T: Clone + Ord> SuffixArray<T> {
        /// ## create by SA-IS. O(N)
        /// 番兵として $0$ を挿入する
        pub fn build(src: &[T]) -> Self {
            let mut sais = Sais::new(src.len() + 1);
            let mut p = src.compress(1);
            p.push(0);
            let sa = sais.suffix_array(&p);
            Self {
                sa,
                src: src.to_vec(),
            }
        }

        /// targetがsourceに含まれるとき、そのindexの一つを返す
        ///
        /// ## 計算量
        /// targetの長さをNとして、$M\logN$
        pub fn search(&self, target: &[T]) -> Option<usize> {
            let mut l = 0;
            let mut r = self.sa.len();
            while r - l > 1 {
                let mid = (l + r) / 2;
                let sl = &self.src[self.sa[mid]..];
                if sl.len() <= target.len() {
                    *if sl <= target { &mut l } else { &mut r } = mid;
                } else {
                    *if &sl[..target.len()] <= target {
                        &mut l
                    } else {
                        &mut r
                    } = mid;
                }
            }
            let sl = &self.src[self.sa[l]..];
            if sl.len() >= target.len() && &sl[..target.len()] == target {
                Some(self.sa[l])
            } else {
                None
            }
        }
    }
    impl<T> Index<usize> for SuffixArray<T> {
        type Output = usize;
        fn index(&self, i: usize) -> &usize {
            &self.sa[i]
        }
    }

    /// SA-IS法によるSuffixArrayの構築
    ///
    /// ## 計算量
    /// $O(N)$
    #[derive(Debug, Clone)]
    struct Sais {
        pos: Vec<usize>,
        lms_pos: Vec<usize>,
        reduced_text_pos: Vec<usize>,
        bucket_sizes: BTreeMap<usize, usize>,
        bucket_start: Vec<usize>,
        bucket_end: Vec<usize>,
    }

    impl Sais {
        fn new(n: usize) -> Sais {
            Sais {
                pos: Vec::with_capacity(n),
                lms_pos: Vec::with_capacity(n),
                reduced_text_pos: vec![0; n + 1],
                bucket_sizes: BTreeMap::new(),
                bucket_start: Vec::with_capacity(n),
                bucket_end: Vec::with_capacity(n),
            }
        }

        fn init_bucket_start(&mut self, source: &[usize]) {
            self.bucket_sizes.clear();
            self.bucket_start.clear();
            for &c in source {
                *self.bucket_sizes.entry(c).or_insert(0) += 1;
            }
            let mut sum = 0;
            for &size in self.bucket_sizes.values() {
                self.bucket_start.push(sum);
                sum += size
            }
        }

        fn init_bucket_end(&mut self, source: &[usize]) {
            self.bucket_end.clear();
            for &r in &self.bucket_start[1..] {
                self.bucket_end.push(r - 1);
            }
            self.bucket_end.push(source.len() - 1);
        }

        fn construct(&mut self, source: &[usize]) {
            let pos_types = PosTypes::new(source);

            self.calc_lms_pos(source, &pos_types);
            self.calc_pos(source, &pos_types);
        }

        fn suffix_array(&mut self, source: &[usize]) -> Vec<usize> {
            self.construct(source);
            self.pos.clone()
        }

        fn calc_lms_pos(&mut self, source: &[usize], pos_types: &PosTypes) {
            let n = source.len();
            self.lms_pos.clear();
            let mut i = 0;
            for r in 0..n {
                if pos_types.is_lms(r) {
                    self.lms_pos.push(r);
                    self.reduced_text_pos[r] = i;
                    i += 1;
                }
            }

            self.calc_pos(source, pos_types);
            let lms_substring_count = self.lms_pos.len();
            self.sort_lms_suffixes(source, pos_types, lms_substring_count);
        }

        fn calc_pos(&mut self, source: &[usize], pos_types: &PosTypes) {
            let n = source.len();
            self.pos.clear();
            self.init_bucket_start(source);
            self.init_bucket_end(source);
            self.pos.resize(n, n);
            for &p in self.lms_pos.iter().rev() {
                let c = source[p];
                self.pos[self.bucket_end[c]] = p;
                self.bucket_end[c] = self.bucket_end[c].wrapping_sub(1);
            }

            self.init_bucket_end(source);

            for r in 0..n {
                let p = self.pos[r];
                if p == n || p == 0 {
                    continue;
                }
                let c = source[p - 1];
                self.pos[self.bucket_start[c]] = p - 1;
                self.bucket_start[c] += 1;
            }

            for r in (0..n).rev() {
                let p = self.pos[r];
                if p == 0 {
                    continue;
                }
                if pos_types.is_s(p - 1) {
                    let c = source[p - 1];
                    self.pos[self.bucket_end[c]] = p - 1;
                    self.bucket_end[c] = self.bucket_end[c].wrapping_sub(1);
                }
            }
        }

        fn lms_substring_eq(
            &self,
            source: &[usize],
            pos_types: &PosTypes,
            i: usize,
            j: usize,
        ) -> bool {
            for k in 0.. {
                let lmsi = pos_types.is_lms(i + k);
                let lmsj = pos_types.is_lms(j + k);
                if source[i + k] != source[j + k] {
                    return false;
                }
                if lmsi != lmsj {
                    return false;
                }
                if k > 0 && lmsi && lmsj {
                    return true;
                }
            }
            false
        }

        fn sort_lms_suffixes(
            &mut self,
            source: &[usize],
            pos_types: &PosTypes,
            lms_substring_count: usize,
        ) {
            if lms_substring_count > 1 {
                let mut reduced_text = vec![0usize; lms_substring_count];
                let mut label = 0usize;
                reduced_text[self.reduced_text_pos[self.pos[0]]] = label;
                let mut prev = None;
                for &p in &self.pos {
                    if pos_types.is_lms(p) {
                        if prev.is_some()
                            && !self.lms_substring_eq(source, pos_types, prev.unwrap(), p)
                        {
                            label += 1;
                        }
                        reduced_text[self.reduced_text_pos[p]] = label;
                        prev = Some(p)
                    }
                }
                if label + 1 < lms_substring_count {
                    let lms_pos = self.lms_pos.clone();
                    self.construct(&reduced_text);
                    self.lms_pos.clear();
                    for &p in &self.pos {
                        self.lms_pos.push(lms_pos[p])
                    }
                } else {
                    self.lms_pos.clear();
                    for &p in &self.pos {
                        if pos_types.is_lms(p) {
                            self.lms_pos.push(p);
                        }
                    }
                }
            }
        }
    }
    struct PosTypes(Vec<bool>);

    impl PosTypes {
        fn new(source: &[usize]) -> Self {
            let n = source.len();
            let mut v = vec![false; n];
            v[n - 1] = true;
            for i in (0..n - 1).rev() {
                v[i] = if source[i] == source[i + 1] {
                    v[i + 1]
                } else {
                    source[i] < source[i + 1]
                }
            }
            PosTypes(v)
        }

        fn is_s(&self, pos: usize) -> bool {
            *self.0.get(pos).unwrap_or(&true)
        }
        fn is_l(&self, pos: usize) -> bool {
            !*self.0.get(pos).unwrap_or(&false)
        }
        fn is_lms(&self, pos: usize) -> bool {
            pos != 0 && self.is_s(pos) && self.is_l(pos - 1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_test() {
        let s = "mmiissiissiippii".as_bytes();
        let sa = SuffixArray::build(&s);
        assert!(sa.search(&Vec::new()).is_some());
        assert!(sa.search(&"i".as_bytes()).is_some());
        assert_eq!(Some(0), sa.search("mmiissii".as_bytes()));
        assert_eq!(Some(3), sa.search("issii".as_bytes()));
        assert_eq!(Some(12), sa.search(&sa.src[12..]));
        assert_eq!(None, sa.search(&vec![4, 4, 1, 1, 1,]));
        assert_eq!(None, sa.search(&vec![5]));
        assert_eq!(None, sa.search(&vec![1, 1, 4, 4, 1, 1, 3, 3, 3,]));
    }
}
