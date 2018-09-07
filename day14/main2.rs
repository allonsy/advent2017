mod hash;

use std::collections::HashSet;

const INPUT: &'static str = "ljoxqyyw";
const GRID_DIMENSION: usize = 128;

fn main() {
    let mut grid = get_blank_grid();
    for i in 0..GRID_DIMENSION {
        let input_str = format!("{}-{}", INPUT, i);
        grid[i] = get_grid_values(input_str);
    }

    let mut num_regions = 0;
    let mut seen_before = HashSet::new();

    for i in 0..GRID_DIMENSION {
        for j in 0..GRID_DIMENSION {
            if grid[i][j] && !seen_before.contains(&(i,j)) {
                process_region(&mut seen_before, &grid, (i,j));
                num_regions += 1;
            }
        }
    }

    println!("num regions is: {}", num_regions);
}

fn process_region(
    seen_before: &mut HashSet<(usize, usize)>,
    grid: &[[bool; GRID_DIMENSION]; GRID_DIMENSION],
    coords: (usize, usize)) {

    if seen_before.contains(&coords) {
        return;
    }

    seen_before.insert(coords);
    let neighbors = get_filled_neighbors(coords, grid);

    for neighbor in neighbors {
        if grid[neighbor.0][neighbor.1] && !seen_before.contains(&neighbor) {
            process_region(seen_before, grid, neighbor);
        }
    }
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
        _ => panic!("unknown char: {}", c)
    }
}

fn get_filled_neighbors(coords: (usize, usize), grid: &[[bool; GRID_DIMENSION]; GRID_DIMENSION]) -> Vec<(usize, usize)> {
    let (x,y) = coords;
    let mut neighbors = Vec::new();

    if x + 1 < GRID_DIMENSION && grid[x+1][y] {
        neighbors.push((x+1, y));
    }
    if x >= 1 && grid[x-1][y] {
        neighbors.push((x-1, y));
    }
    if y+1 < GRID_DIMENSION && grid[x][y+1] {
        neighbors.push((x, y+1));
    }
    if y >= 1 && grid[x][y-1] {
        neighbors.push((x, y-1));
    }
    return neighbors;
}
