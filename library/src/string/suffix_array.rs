//! 接尾辞配列

use crate::util::compress::compress;
use crate::*;

#[derive(Debug, Clone)]
pub struct SuffixArray {
    pub sa: Vec<usize>,
    pub source: Vec<u8>,
}

impl SuffixArray {
    /// create by SA-IS. O(N)
    pub fn build(source: &[u8]) -> Self {
        let mut source: Vec<_> = source.to_vec();
        source.push(0);
        let mut sais = Sais::new(source.len());
        let sa = sais.suffix_array(
            &compress(&source)
                .iter()
                .map(|&u| u as u8)
                .collect::<Vec<_>>(),
        );
        Self { sa, source }
    }

    /// targetがsourceに含まれるとき、そのindexの一つを返す
    pub fn search(&self, target: &[u8]) -> Option<usize> {
        let mut l = 0;
        let mut r = self.sa.len();
        while r - l > 1 {
            let mid = (l + r) / 2;
            if &self.source[self.sa[mid]..min(self.sa[mid] + target.len(), self.sa.len())] <= target
            {
                l = mid;
            } else {
                r = mid;
            }
        }
        if &self.source[self.sa[l]..min(self.sa[l] + target.len(), self.sa.len())] == target {
            Some(self.sa[l])
        } else {
            None
        }
    }
}

/// SA-IS法によるSuffixArrayの構築
///
/// ## 計算量
/// `$O(N)$`
#[derive(Debug, Clone)]
struct Sais {
    pos: Vec<usize>,
    lms_pos: Vec<usize>,
    reduced_text_pos: Vec<usize>,
    bucket_sizes: BTreeMap<u8, usize>,
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

    fn init_bucket_start(&mut self, source: &[u8]) {
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

    fn init_bucket_end(&mut self, source: &[u8]) {
        self.bucket_end.clear();
        for &r in &self.bucket_start[1..] {
            self.bucket_end.push(r - 1);
        }
        self.bucket_end.push(source.len() - 1);
    }

    fn construct(&mut self, source: &[u8]) {
        let pos_types = PosTypes::new(source);

        self.calc_lms_pos(source, &pos_types);
        self.calc_pos(source, &pos_types);
    }

    fn suffix_array(&mut self, source: &[u8]) -> Vec<usize> {
        self.construct(source);
        self.pos.clone()
    }

    fn calc_lms_pos(&mut self, source: &[u8], pos_types: &PosTypes) {
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

    fn calc_pos(&mut self, source: &[u8], pos_types: &PosTypes) {
        let n = source.len();
        self.pos.clear();
        self.init_bucket_start(source);
        self.init_bucket_end(source);
        self.pos.resize(n, n);
        for &p in self.lms_pos.iter().rev() {
            let c = source[p] as usize;
            self.pos[self.bucket_end[c]] = p;
            self.bucket_end[c] = self.bucket_end[c].wrapping_sub(1);
        }

        self.init_bucket_end(source);

        for r in 0..n {
            let p = self.pos[r];
            if p == n || p == 0 {
                continue;
            }
            let c = source[p - 1] as usize;
            self.pos[self.bucket_start[c]] = p - 1;
            self.bucket_start[c] += 1;
        }

        for r in (0..n).rev() {
            let p = self.pos[r];
            if p == 0 {
                continue;
            }
            if pos_types.is_s(p - 1) {
                let c = source[p - 1] as usize;
                self.pos[self.bucket_end[c]] = p - 1;
                self.bucket_end[c] = self.bucket_end[c].wrapping_sub(1);
            }
        }
    }

    fn lms_substring_eq(&self, source: &[u8], pos_types: &PosTypes, i: usize, j: usize) -> bool {
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
        source: &[u8],
        pos_types: &PosTypes,
        lms_substring_count: usize,
    ) {
        if lms_substring_count > 1 {
            let mut reduced_text = vec![0u8; lms_substring_count];
            let mut label = 0;
            reduced_text[self.reduced_text_pos[self.pos[0]]] = label;
            let mut prev = None;
            for &p in &self.pos {
                if pos_types.is_lms(p) {
                    if prev.is_some() && !self.lms_substring_eq(source, pos_types, prev.unwrap(), p)
                    {
                        label += 1;
                    }
                    reduced_text[self.reduced_text_pos[p]] = label;
                    prev = Some(p)
                }
            }
            if label as usize + 1 < lms_substring_count {
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
    fn new(source: &[u8]) -> Self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_test() {
        assert!(build_u8_vec("abc") <= build_u8_vec("abcd"));
        assert!(build_u8_vec("abc") <= build_u8_vec("abc"));
        assert!(!(build_u8_vec("abc") <= build_u8_vec("ab")));
        assert!(build_u8_vec("abc") <= build_u8_vec("abd"));

        let s: Vec<_> = build_u8_vec("mmiissiissiippii");
        let sa = SuffixArray::build(&s);
        assert!(sa.search(&build_u8_vec("")).is_some());
        assert!(sa.search(&build_u8_vec("i")).is_some());
        assert_eq!(Some(0), sa.search(&build_u8_vec("mmiissiissiippii")));
        assert_eq!(Some(3), sa.search(&build_u8_vec("issiis")));
        assert_eq!(Some(12), sa.search(&build_u8_vec("ppii")));
        assert_eq!(None, sa.search(&build_u8_vec("issiisss")));
        assert_eq!(None, sa.search(&build_u8_vec("ppiii")));
        assert_eq!(None, sa.search(&build_u8_vec("b")));
    }

    fn build_u8_vec(s: &str) -> Vec<u8> {
        s.chars().map(|c| c as u8).collect()
    }
}
