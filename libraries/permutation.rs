pub struct Permutation {
    stack: Vec<usize>,
}

impl Permutation {
    pub fn new(number: usize, modulo: usize) -> Self {
        let mut stack: Vec<usize> = Vec::new();
        stack.push(1);
        for i in 0..(number) {
            let k = stack[i] * (number - i) % modulo;
            stack.push(k);
        }

        Self {
            stack,
        }
    }

    pub fn get(&self, number: isize) -> usize {
        self.stack[number as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calc_factorial() {
        let fact = Permutation::new(5, 1000000007);

        assert_eq!(5, fact.get(1));
        assert_eq!(20, fact.get(2));
        assert_eq!(60, fact.get(3));
        assert_eq!(120, fact.get(4));
        assert_eq!(120, fact.get(5));
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = Permutation::new(10, 1000000007);
        fact.get(11);
    }
}