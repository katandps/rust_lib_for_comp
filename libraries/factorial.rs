pub struct Factorial {
    stack: Vec<usize>,
}

impl Factorial {
    pub fn new(number: isize, modulo: usize) -> Self {
        let mut stack: Vec<usize> = Vec::new();
        stack.push(1);
        let mut k = 1;
        for i in 1..number + 1 {
            k = k * i as usize % modulo;
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
