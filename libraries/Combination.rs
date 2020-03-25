pub struct Combination {
    stack: Vec<usize>,
}

impl Combination {
    pub fn new(number: usize) -> Self {
        const MODULO: usize = 1000000007;
        let mut stack: Vec<usize> = Vec::new();
        stack.push(1);
        for i in 0..number {
            let k = (stack[i] * (number - i) % MODULO) * ModInv::mod_inv(i as isize + 1, MODULO) % MODULO;
            stack.push(k);
        }

        Self {
            stack: stack,
        }
    }

    pub fn get(&self, number: usize) -> usize {
        self.stack[number]
    }
}

use std::mem::swap;

struct ModInv {}

impl ModInv {
    pub fn mod_inv(number: isize, modulo: usize) -> usize {
        let mut n = number;
        let mut b = modulo as isize;
        let mut u: isize = 1;
        let mut v: isize = 0;

        while b > 0 {
            let t: isize = n / b;
            n -= t * b;
            swap(&mut n, &mut b);
            u -= t * v;
            swap(&mut u, &mut v);
        }
        u %= modulo as isize;
        if u < 0 {
            u += modulo as isize;
        }
        u as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calc() {
        let five = Combination::new(5);

        assert_eq!(1, five.get(0));
        assert_eq!(5, five.get(1));
        assert_eq!(10, five.get(2));
        assert_eq!(10, five.get(3));
        assert_eq!(5, five.get(4));
        assert_eq!(1, five.get(5));
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = Combination::new(10);
        fact.get(11);
    }
}