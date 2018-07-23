mod util;

fn main() {
    let mut jumps = get_jumps();
    let jumps_size = jumps.len() as i32;
    let mut num_jumps = 0;

    let mut index: i32 = 0;

    loop {
        if index < 0 || index >= jumps_size {
            break;
        }
        num_jumps += 1;
        let jump_val = jumps[index as usize];
        jumps[index as usize] += 1;
        index += jump_val;
    }
    println!("number of jumps is: {}", num_jumps);
}

fn get_jumps() -> Vec<i32> {
    let lines = util::read_file_lines("input.txt");

    let mut jumps: Vec<i32> = Vec::new();

    for line in lines {
        jumps.push(line.parse::<i32>().unwrap());
    }
    return jumps;
}
