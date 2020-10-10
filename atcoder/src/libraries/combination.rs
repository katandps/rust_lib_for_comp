#[allow(unused_imports)]
use combination::Combination;

#[allow(dead_code)]
mod combination {
    use super::mod_int::*;

    type Num = i64;

    pub struct Combination {
        stack: Vec<ModInt<Num>>,
    }

    impl Combination {
        pub fn new(number: Num) -> Self {
            let mut stack = vec![ModInt::new(1)];
            for i in 0..number {
                let t = stack[i as usize] * (number - i);
                stack.push(t / (i + 1));
            }
            Combination { stack }
        }

        pub fn get(&self, number: Num) -> ModInt<Num> {
            self.stack[number as usize]
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

        assert_eq!(1, five.get(0).get());
        assert_eq!(5, five.get(1).get());
        assert_eq!(10, five.get(2).get());
        assert_eq!(10, five.get(3).get());
        assert_eq!(5, five.get(4).get());
        assert_eq!(1, five.get(5).get());
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = Combination::new(10);
        fact.get(11);
    }
}
