const HASH_SIZE: i32 = 256;
const INPUT: [i32; 16] = [
    212, 254, 178, 237, 2, 0, 1, 54, 167, 92, 117, 125, 255, 61, 159, 164,
];

fn main() {
    let mut hash_vec = generate_start_vec();

    let mut current_pos = 0;
    let mut skip_size = 0;
    for input in INPUT.iter() {
        let start_index = current_pos;
        let end_index = mod_index(current_pos + input);
        if *input != 0 {
            reverse_range(&mut hash_vec, start_index, end_index);
        }
        current_pos = mod_index(current_pos + input + skip_size);
        skip_size += 1;
    }

    let mult = hash_vec[0] * hash_vec[1];
    println!("multiplication is {}", mult);
}

fn generate_start_vec() -> Vec<i32> {
    let mut nums = Vec::new();
    for i in 0..HASH_SIZE {
        nums.push(i as i32);
    }
    return nums;
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
