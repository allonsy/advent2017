use std::fs::File;
use std::io::Read;


const FILE_NAME : &str = "input.txt";

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
        let row_sum = process_row(&row);
        sum += row_sum;
    }
    println!("checksum is: {}", sum);
}

fn process_row(row : &Vec<i32>) -> i32 {
    let mut sum = 0;
    for val1 in row {
        for val2 in row {
            if *val1 % *val2 == 0 && *val1 != *val2 {
                sum += *val1 / *val2;
            }
        }
    }
    return sum;
}
