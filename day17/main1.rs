
const NUM_ROUNDS: i32 = 2017;
const ROUND_SIZE: i32 = 369;

struct CircularBuffer {
    buffer: Vec<i32>,
    current_position: i32,
    iter_num: i32,
    round_size: i32,
}

impl CircularBuffer {
    fn new() -> CircularBuffer {
        CircularBuffer {
            buffer: vec![0],
            current_position: 0,
            iter_num: 1,
            round_size: ROUND_SIZE
        }
    }

    fn perform_round(&mut self) {
        self.current_position += self.round_size;
        self.current_position %= self.buffer.len() as i32;
        if self.current_position == self.buffer.len() as i32 - 1 {
            self.buffer.push(self.iter_num);
        } else {
            self.buffer.insert(self.current_position as usize + 1, self.iter_num);
        }
        self.current_position += 1;
        self.iter_num += 1;
    }

    fn get_next_num(&self) -> i32 {
        let idx = (self.current_position + 1) % self.buffer.len() as i32;
        self.buffer[idx as usize]
    }

    fn print_buf(&self) {
        print!("Buffer is: [");
        for i in 0..self.buffer.len(){
            if i as i32 == self.current_position {
                print!(" (");
            }
            print!(" {} ", self.buffer[i]);
            if i as i32 == self.current_position {
                print!(") ");
            }
        }
        println!("] ");
    }


}

fn main() {
    let mut buff = CircularBuffer::new();
    for _ in 0..NUM_ROUNDS {
        buff.perform_round();
        //buff.print_buf();
    }

    println!("Number is: {}", buff.get_next_num());
}