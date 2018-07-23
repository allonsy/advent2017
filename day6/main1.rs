use std::collections::HashSet;

const ARRAY_SIZE: usize = 16;

fn main() {
    let mut blocks: [i32; ARRAY_SIZE] = [5, 1, 10, 0, 1, 7, 13, 14, 3, 12, 8, 10, 7, 12, 0, 6];

    let mut seen_blocks: HashSet<[i32; ARRAY_SIZE]> = HashSet::new();
    seen_blocks.insert(blocks);
    let mut num_iterations = 0;

    loop {
        let max_idx = get_max_index(&blocks);
        let mut distribution = blocks[max_idx];
        blocks[max_idx] = 0;
        let mut i = (max_idx + 1) % ARRAY_SIZE;

        while distribution > 0 {
            blocks[i] += 1;
            distribution -= 1;
            i += 1;
            i %= ARRAY_SIZE;
        }
        num_iterations += 1;
        let seen_before = seen_blocks.insert(blocks);
        if seen_before == false {
            break;
        }
    }
    println!("number of iterations is: {}", num_iterations);
}

fn get_max_index(blocks: &[i32; ARRAY_SIZE]) -> usize {
    let mut max = blocks[0];
    let mut max_idx = 0;
    for i in 1..ARRAY_SIZE {
        if max < blocks[i] {
            max = blocks[i];
            max_idx = i;
        }
    }

    return max_idx;
}
