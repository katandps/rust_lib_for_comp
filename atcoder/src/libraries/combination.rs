#[allow(unused_imports)]
use combination::Combination;

#[allow(dead_code)]
mod combination {
    use super::mod_int::*;

    pub struct Combination {
        stack: Vec<ModInt<usize>>,
    }

    impl Combination {
        pub fn new(number: usize) -> Self {
            let mut stack = vec![ModInt::new(1)];
            for i in 0..number {
                let t = stack[i] * (number - i);
                stack.push(t / (i + 1));
            }
            Combination { stack: stack }
        }

        pub fn get(&self, number: usize) -> ModInt<usize> {
            self.stack[number]
        }
    }
}

use crate::libraries::mod_int::mod_int;
#[cfg(test)]
mod tests {
    use super::combination::*;

    #[test]
    fn can_calc() {
        let five = Combination::new(5);

        assert_eq!(1, five.get(0).v);
        assert_eq!(5, five.get(1).v);
        assert_eq!(10, five.get(2).v);
        assert_eq!(10, five.get(3).v);
        assert_eq!(5, five.get(4).v);
        assert_eq!(1, five.get(5).v);
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = Combination::new(10);
        fact.get(11);
    }
}
