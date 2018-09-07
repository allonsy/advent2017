struct Generator {
    prev_value: u64,
    factor: u64,
}

impl Generator {
    fn new(prev_value: u64, factor: u64) -> Generator {
        Generator {
            prev_value: prev_value,
            factor: factor,
        }
    }

    fn next(&mut self) -> u64 {
        self.prev_value = (self.prev_value * self.factor) % 2147483647;
        self.prev_value
    }
}

fn main() {
    let mut gen_a = Generator::new(516, 16807);
    let mut gen_b = Generator::new(190, 48271);

    let mut num_match = 0;

    for _ in 0..40000000 {
        let val_a = gen_a.next();
        let val_b = gen_b.next();
        if val_a as u16 == val_b as u16 {
            num_match += 1;
        }
    }

    println!("number of matches is: {}", num_match);
}
