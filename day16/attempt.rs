mod util;

const ARR_SIZE: usize = 16;

struct Dance {
    index_arr: [u8; ARR_SIZE],
    char_arr: [char; ARR_SIZE],
}

impl Dance {
    fn new() -> Dance {
        Dance {
            index_arr: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            char_arr: [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
            ]
        }
    }

    fn rotate(&mut self, num: u8) {
        for i in 0..ARR_SIZE {
            self.index_arr[i] = (self.index_arr[i] + num) % ARR_SIZE as u8;
            self.char_arr[self.index_arr[i] as usize] = (i as u8 + 97) as char;
        }
    }

    fn swap_index(&mut self, from_index: usize, to_index: usize) {
        let temp = self.char_arr[to_index];
        self.char_arr[to_index] = self.char_arr[from_index];
        self.index_arr[self.char_arr[to_index] as usize - 97] = to_index as u8;

        self.char_arr[from_index] = temp;
        self.index_arr[temp as usize - 97] = from_index as u8;
    }

    fn swap_char(&mut self, from_char: char, to_char: char) {
        let from_char_idx: usize = from_char as usize - 97;
        let to_char_idx: usize = to_char as usize - 97;

        let temp = self.index_arr[to_char_idx];
        self.index_arr[to_char_idx] = self.index_arr[from_char_idx];
        self.char_arr[self.index_arr[to_char_idx] as usize] = to_char;

        self.index_arr[from_char_idx] = temp;
        self.char_arr[self.index_arr[from_char_idx] as usize] = from_char;
    }

    fn print_array(&self) {
        print!("array is: ");
        for i in 0..ARR_SIZE {
            print!("{}", self.char_arr[i]);
        }
        println!("");
    }
}

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn main() {
    let instructions = get_instructions();
    let mut dance = Dance::new();

    for i in 0..1000000000 {
        for instruction in &instructions {
            match instruction {
                Instruction::Spin(s) => dance.rotate(*s as u8),
                Instruction::Exchange(x, y) => dance.swap_index(*x, *y),
                Instruction::Partner(a, b) => dance.swap_char(*a, *b),
            }
        }
        println!("loop #: {}", i);
    }

    dance.print_array();
}

fn get_instructions() -> Vec<Instruction> {
    let line = util::read_file_string("input.txt");
    let mut instructions = Vec::new();

    for inst_str in line.split(",") {
        let bytes = inst_str.as_bytes();
        match bytes[0] as char {
            's' => {
                let num_str = String::from_utf8_lossy(&bytes[1..bytes.len()]).to_string();
                instructions.push(Instruction::Spin(num_str.trim().parse().unwrap()));
            }
            'x' => {
                let (slice1, slice2) = split_input(&bytes[1..bytes.len()]);
                let str1 = String::from_utf8_lossy(slice1).to_string();
                let str2 = String::from_utf8_lossy(slice2).to_string();
                instructions.push(Instruction::Exchange(
                    str1.trim().parse().unwrap(),
                    str2.trim().parse().unwrap(),
                ));
            }
            'p' => instructions.push(Instruction::Partner(bytes[1] as char, bytes[3] as char)),
            _ => panic!("unknown character: {}", bytes[0]),
        }
    }
    return instructions;
}

fn split_input(input: &[u8]) -> (&[u8], &[u8]) {
    let mut i = 0;
    while i < input.len() {
        if input[i] == '/' as u8 {
            break;
        }
        i += 1;
    }
    return (&input[0..i], &input[i + 1..input.len()]);
}
