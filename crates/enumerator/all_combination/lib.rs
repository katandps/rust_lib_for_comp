//! 組み合わせの全列挙
/// AtCoderではItertoolsでよい
use prelude::*;

#[snippet(name = "all_combination", doc_hidden)]
pub struct AllCombination {
    v: Vec<usize>,
}

#[snippet(name = "all_combination", doc_hidden)]
impl AllCombination {
    pub fn new(n: usize, r: usize) -> AllCombination {
        let mut c = AllCombination { v: Vec::new() };
        c.combination(n, r);
        c
    }

    fn combination(&mut self, n: usize, r: usize) {
        let p = 1usize << n;
        for i in 0..p {
            if i.count_ones() == r as u32 {
                self.v.push(i)
            }
        }
    }

    pub fn get(&self) -> &Vec<usize> {
        &self.v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let c = AllCombination::new(4, 2);
        let result = c.get();
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], 3);
        assert_eq!(result[1], 5);
        assert_eq!(result[2], 6);
        assert_eq!(result[3], 9);
        assert_eq!(result[4], 10);
        assert_eq!(result[5], 12);

        let c2 = AllCombination::new(24, 12);
        let result = c2.get();
        assert_eq!(result.len(), 2704156);
    }
}
