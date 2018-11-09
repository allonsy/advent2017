
const NUM_ROUNDS: i32 = 50000000;
const ROUND_SIZE: i32 = 369;

struct CircularBuffer {
    current_position: i32,
    iter_num: i32,
    round_size: i32,
    zero_pos: i32,
    buf_size: i32,
    after_zero: i32,
}

impl CircularBuffer {
    fn new() -> CircularBuffer {
        CircularBuffer {
            current_position: 0,
            iter_num: 1,
            round_size: ROUND_SIZE,
            zero_pos: 0,
            buf_size: 1,
            after_zero: 0,
        }
    }

    fn perform_round(&mut self) {
        self.current_position += self.round_size;
        self.current_position %= self.buf_size;
        
        if self.current_position < self.zero_pos {
            self.zero_pos += 1
        }

        if self.current_position == self.zero_pos {
            self.after_zero = self.iter_num;
        }

        self.current_position += 1;
        self.buf_size += 1;
        self.iter_num += 1;
    }

    fn get_next_num(&self) -> i32 {
        self.after_zero
    }

}

fn main() {
    let mut buff = CircularBuffer::new();
    for _ in 0..NUM_ROUNDS {
        buff.perform_round();
    }

    println!("Number is: {}", buff.get_next_num());
}