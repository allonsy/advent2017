const HASH_SIZE: i32 = 256;

pub fn get_knot_hash(input_string: &str) -> String {
    let mut hash_vec = generate_start_vec();

    let mut current_pos = 0;
    let mut skip_size = 0;
    let input_vec = get_input(input_string);
    for _ in 0..64 {
        for input in &input_vec {
            let start_index = current_pos;
            let end_index = mod_index(current_pos + input);
            if *input != 0 {
                reverse_range(&mut hash_vec, start_index, end_index);
            }
            current_pos = mod_index(current_pos + input + skip_size);
            skip_size += 1;
        }
    }

    let dense = get_dense_hash(&hash_vec);
    get_str_representation(&dense)
}

fn generate_start_vec() -> Vec<i32> {
    let mut nums = Vec::new();
    for i in 0..HASH_SIZE {
        nums.push(i as i32);
    }
    return nums;
}

fn get_dense_hash(hash_vec: &Vec<i32>) -> Vec<i32> {
    let mut dense_vec = Vec::new();

    let mut xor = 0;
    for i in 0..HASH_SIZE {
        xor ^= hash_vec[i as usize];
        if i % 16 == 15 {
            dense_vec.push(xor);
            xor = 0;
        }
    }
    return dense_vec;
}

fn get_str_representation(hash_vec: &Vec<i32>) -> String {
    let mut hash = String::new();

    for byte in hash_vec {
        let mut hex_byte: String = format!("{:x}", byte);
        if hex_byte.len() == 1 {
            hex_byte = "0".to_string() + &hex_byte;
        }
        hash += &hex_byte;
    }
    return hash;
}

fn reverse_range(hash_vec: &mut Vec<i32>, start_index: i32, end_index: i32) {
    let range = if end_index > start_index {
        end_index - start_index
    } else {
        HASH_SIZE - start_index + end_index
    };

    let actual_end_index = mod_index(end_index - 1);
    for i in 0..(range / 2) {
        let swap_start = mod_index(start_index + i);
        let swap_end = mod_index(actual_end_index - i);
        swap(hash_vec, swap_start, swap_end);
    }
}

fn swap(hash_vec: &mut Vec<i32>, start_index: i32, end_index: i32) {
    let temp = hash_vec[end_index as usize];
    hash_vec[end_index as usize] = hash_vec[start_index as usize];
    hash_vec[start_index as usize] = temp;
}

fn mod_index(index: i32) -> i32 {
    if index >= 0 {
        index % HASH_SIZE
    } else {
        HASH_SIZE + (index % HASH_SIZE)
    }
}

fn get_input(input_str: &str) -> Vec<i32> {
    let mut bytes = Vec::new();
    for byte in input_str.to_string().as_bytes() {
        bytes.push(*byte as i32);
    }

    bytes.push(17);
    bytes.push(31);
    bytes.push(73);
    bytes.push(47);
    bytes.push(23);
    return bytes;
}
