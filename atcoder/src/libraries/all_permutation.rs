#[allow(unused_imports)]
use all_permutation::*;

#[allow(dead_code)]
mod all_permutation {
    pub struct AllPermutation<T> {
        v: Vec<Vec<T>>,
    }

    impl<T: Eq + Clone> AllPermutation<T> {
        pub fn new(v: Vec<T>) -> AllPermutation<T> {
            let mut p = AllPermutation { v: Vec::new() };
            p.permutate(v, Vec::new());
            p
        }

        fn permutate(&mut self, rest: Vec<T>, mut current: Vec<T>) {
            if rest.len() == 1 {
                current.push(rest[0].clone());
                self.v.push(current.clone());
            } else {
                if rest.len() == 2 {
                    let mut next = current.clone();
                    next.push(rest[0].clone());
                    next.push(rest[1].clone());
                    self.v.push(next);
                    current.push(rest[1].clone());
                    current.push(rest[0].clone());
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

        pub fn get(&self) -> &Vec<Vec<T>> {
            &self.v
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let p = AllPermutation::new(vec![1, 2, 3]);
        let result = p.get();
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], vec![1, 2, 3]);
        assert_eq!(result[1], vec![1, 3, 2]);
        assert_eq!(result[2], vec![2, 1, 3]);
        assert_eq!(result[3], vec![2, 3, 1]);
        assert_eq!(result[4], vec![3, 1, 2]);
        assert_eq!(result[5], vec![3, 2, 1]);

        let p2 = AllPermutation::new(vec![0; 9]);
        assert_eq!(p2.get().len(), 362880);
    }
}
