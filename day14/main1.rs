mod hash;

const INPUT: &'static str = "ljoxqyyw";
const GRID_DIMENSION: usize = 128;

fn main() {
    let mut grid = get_blank_grid();
    for i in 0..GRID_DIMENSION {
        let input_str = format!("{}-{}", INPUT, i);
        grid[i] = get_grid_values(input_str);
    }

    let mut total_filled = 0;
    for i in 0..GRID_DIMENSION {
        for j in 0..GRID_DIMENSION {
            if grid[i][j] {
                total_filled += 1;
            }
        }
    }
    println!("total filled is: {}", total_filled);
}

fn get_grid_values(input: String) -> [bool; GRID_DIMENSION] {
    let mut row = [false; GRID_DIMENSION];

    let hash = hash::get_knot_hash(&input);
    let mut i = 0;
    for ch in hash.chars() {
        let byte_arr = get_char_bytes(ch);
        for j in 0..4 {
            row[i + j] = byte_arr[j];
        }
        i += 4;
    }
    return row;
}

fn get_blank_grid() -> [[bool; GRID_DIMENSION]; GRID_DIMENSION] {
    return [[false; GRID_DIMENSION]; GRID_DIMENSION];
}

fn get_char_bytes(c: char) -> [bool; 4] {
    match c {
        '0' => [false, false, false, false],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'a' => [true, false, true, false],
        'b' => [true, false, true, true],
        'c' => [true, true, false, false],
        'd' => [true, true, false, true],
        'e' => [true, true, true, false],
        'f' => [true, true, true, true],
        _ => panic!("unknown char: {}", c),
    }
}
