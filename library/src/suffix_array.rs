pub mod suffix_array {
    use std::collections::BTreeMap;

    #[derive(Debug, Clone)]
    pub struct SuffixArray {
        sa: Vec<usize>,
        source: Vec<u8>,
    }

    impl SuffixArray {
        /// create by SA-IS. O(N)
        pub fn create(source: &[u8]) -> Self {
            let mut sais = SAIS::new(source.len());
            let source = compress(source);
            let sa = sais.suffix_array(&source);
            Self { sa, source }
        }
    }

    fn compress(source: &[u8]) -> Vec<u8> {
        let mut source: Vec<_> = source.iter().cloned().collect();
        source.push(0);
        let n = source.len();
        let mut idx: Vec<_> = (0..n).collect();
        idx.sort_by_key(|&i| &source[i]);
        let mut s2 = vec![0; n];
        let mut now = 0;
        for i in 0..n {
            if i > 0 && source[idx[i - 1]] != source[idx[i]] {
                now += 1;
            }
            s2[idx[i]] = now;
        }
        s2
    }

    #[derive(Debug, Clone)]
    struct SAIS {
        pos: Vec<usize>,
        lms_pos: Vec<usize>,
        reduced_text_pos: Vec<usize>,
        bucket_sizes: BTreeMap<u8, usize>,
        bucket_start: Vec<usize>,
        bucket_end: Vec<usize>,
    }
    impl SAIS {
        fn new(n: usize) -> SAIS {
            SAIS {
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

        fn lms_substring_eq(
            &self,
            source: &[u8],
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
                        if prev.is_some()
                            && !self.lms_substring_eq(source, pos_types, prev.unwrap(), p)
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
}

#[cfg(test)]
mod test {
    use crate::suffix_array::suffix_array::SuffixArray;

    #[test]
    fn suffix_array_test() {
        let s: Vec<_> = "mmiissiissiippii".chars().map(|c| c as u8).collect();
        let sa = SuffixArray::create(&s);
        dbg!(sa);
    }
}
