#[allow(unused_imports)]
use factorial::Factorial;

#[allow(dead_code)]
mod factorial {
    use super::mod_int::ModInt;

    pub struct Factorial {
        stack: Vec<ModInt<usize>>,
    }

    impl Factorial {
        pub fn new(number: usize) -> Self {
            let mut stack: Vec<ModInt<usize>> = Vec::new();
            stack.push(ModInt::new(1));
            for i in 1..(number + 1) as usize {
                let k = stack[i - 1] * i;
                stack.push(k);
            }

            Factorial { stack: stack }
        }

        pub fn get(&self, number: usize) -> ModInt<usize> {
            self.stack[number]
        }
    }
}

use crate::libraries::mod_int::mod_int;

#[cfg(test)]
mod tests {
    use super::factorial::*;

    #[test]
    fn can_calc() {
        let fact = Factorial::new(5);

        assert_eq!(1, fact.get(0).v);
        assert_eq!(1, fact.get(1).v);
        assert_eq!(2, fact.get(2).v);
        assert_eq!(6, fact.get(3).v);
        assert_eq!(24, fact.get(4).v);
        assert_eq!(120, fact.get(5).v);
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = Factorial::new(10);
        fact.get(11);
    }
}
