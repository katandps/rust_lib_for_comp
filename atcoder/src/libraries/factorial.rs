pub struct Factorial {
    stack: Vec<usize>,
}

impl Factorial {
    pub fn new(number: isize) -> Self {
        const MODULO: usize = 1000000007;
        let mut stack: Vec<usize> = Vec::new();
        stack.push(1);
        for i in 1..(number + 1) as usize {
            let k = stack[i - 1] * i % MODULO;
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
        let fact = Factorial::new(5);

        assert_eq!(1, fact.get(0));
        assert_eq!(1, fact.get(1));
        assert_eq!(2, fact.get(2));
        assert_eq!(6, fact.get(3));
        assert_eq!(24, fact.get(4));
        assert_eq!(120, fact.get(5));
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = Factorial::new(10);
        fact.get(11);
    }
}