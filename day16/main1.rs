mod util;

const ARR_SIZE: usize = 16;

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn main() {
    let instructions = get_instructions();
    let mut arr = get_start_arr();

    for instruction in instructions {
        match instruction {
            Instruction::Spin(s) => arr = rotate(s, arr),
            Instruction::Exchange(x, y) => swap_index(&mut arr, x, y),
            Instruction::Partner(a, b) => swap_chars(&mut arr, a, b),
        }
    }

    print!("Array is: ");
    for i in 0..ARR_SIZE {
        print!("{}", arr[i]);
    }
    println!("");
}

fn get_start_arr() -> [char; ARR_SIZE] {
    [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]
}

fn rotate(num: usize, arr: [char; ARR_SIZE]) -> [char; ARR_SIZE] {
    let mut new_arr = ['\0'; ARR_SIZE];
    for i in 0..ARR_SIZE {
        let new_index = (i + num) % ARR_SIZE;
        new_arr[new_index] = arr[i];
    }
    return new_arr;
}

fn swap_index(arr: &mut [char; ARR_SIZE], from_index: usize, to_index: usize) {
    let temp = arr[to_index];
    arr[to_index] = arr[from_index];
    arr[from_index] = temp;
}

fn swap_chars(arr: &mut [char; ARR_SIZE], from_char: char, to_char: char) {
    let mut from_index = ARR_SIZE;
    let mut to_index = ARR_SIZE;

    for i in 0..ARR_SIZE {
        if arr[i] == from_char {
            from_index = i;
        } else if arr[i] == to_char {
            to_index = i;
        }
    }
    swap_index(arr, from_index, to_index);
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
