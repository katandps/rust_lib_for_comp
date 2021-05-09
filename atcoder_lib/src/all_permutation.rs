#[allow(unused_imports)]
use all_permutation::*;

#[allow(dead_code)]
mod all_permutation {
    pub struct AllPermutation {
        v: Vec<Vec<usize>>,
    }

    impl AllPermutation {
        pub fn new(n: usize) -> Self {
            let mut p = AllPermutation { v: Vec::new() };
            p.permutate((0..n).collect(), Vec::with_capacity(n));
            p
        }

        fn permutate(&mut self, rest: Vec<usize>, mut current: Vec<usize>) {
            if rest.len() == 1 {
                current.push(rest[0]);
                self.v.push(current);
            } else {
                if rest.len() == 2 {
                    let mut next = current.clone();
                    next.push(rest[0]);
                    next.push(rest[1]);
                    self.v.push(next);
                    current.push(rest[1]);
                    current.push(rest[0]);
                    self.v.push(current);
                } else {
                    for rest_i in 0..rest.len() {
                        let mut next = current.clone();
                        let mut next_rest = rest.clone();
                        next.push(next_rest.remove(rest_i));
                        self.permutate(next_rest, next)
                    }
                }
            }
        }

        pub fn get(&self) -> &Vec<Vec<usize>> {
            &self.v
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let p = AllPermutation::new(3);
        let result = p.get();
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], vec![0, 1, 2]);
        assert_eq!(result[1], vec![0, 2, 1]);
        assert_eq!(result[2], vec![1, 0, 2]);
        assert_eq!(result[3], vec![1, 2, 0]);
        assert_eq!(result[4], vec![2, 0, 1]);
        assert_eq!(result[5], vec![2, 1, 0]);

        let p2 = AllPermutation::new(9);
        assert_eq!(p2.get().len(), 362880);
    }
}
