pub struct Permutation {
    stack: Vec<usize>,
}

impl Permutation {
    pub fn new(number: usize) -> Self {
        const MODULO: usize = 1000000007;
        let mut stack: Vec<usize> = Vec::new();
        stack.push(1);
        for i in 0..(number) {
            let k = stack[i] * (number - i) % MODULO;
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
    fn can_calc() {
        let five = Permutation::new(5);

        assert_eq!(1, five.get(0));
        assert_eq!(5, five.get(1));
        assert_eq!(20, five.get(2));
        assert_eq!(60, five.get(3));
        assert_eq!(120, five.get(4));
        assert_eq!(120, five.get(5));
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = Permutation::new(10);
        fact.get(11);
    }
}