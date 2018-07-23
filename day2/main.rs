use std::fs::File;
use std::i32;
use std::io::Read;

const FILE_NAME: &str = "input.txt";

fn read_input() -> Vec<Vec<i32>> {
    let mut f = File::open(FILE_NAME).unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut rows = Vec::new();

    for fline in contents.lines() {
        if !fline.is_empty() {
            let mut row = Vec::new();
            for num_str in fline.split_whitespace() {
                row.push(num_str.parse::<i32>().unwrap());
            }
            rows.push(row);
        }
    }
    return rows;
}

fn main() {
    let rows = read_input();
    let mut sum = 0;

    for row in rows {
        let (min, max) = get_max_min(&row);
        let diff = max - min;
        sum += diff;
    }
    println!("checksum is: {}", sum);
}

fn get_max_min(row: &Vec<i32>) -> (i32, i32) {
    let mut max = i32::MIN;
    let mut min = i32::MAX;
    for val in row {
        if *val >= max {
            max = *val;
        }
        if *val <= min {
            min = *val;
        }
    }
    return (min, max);
}
