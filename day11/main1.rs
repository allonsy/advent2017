mod util;

enum Direction {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
}

fn main() {
    let dirs = get_direction_list();
    let mut coords = (0, 0, 0);

    for dir in dirs {
        coords = move_hex(coords, dir);
    }
    println!("distance is: {}", determine_distance(coords));
}

fn move_hex(coords: (i32, i32, i32), dir: Direction) -> (i32, i32, i32) {
    let (x, y, z) = coords;
    match dir {
        Direction::N => (x, y + 1, z - 1),
        Direction::NE => (x + 1, y, z - 1),
        Direction::NW => (x - 1, y + 1, z),
        Direction::S => (x, y - 1, z + 1),
        Direction::SE => (x + 1, y - 1, z),
        Direction::SW => (x - 1, y, z + 1),
    }
}

fn determine_distance(coords: (i32, i32, i32)) -> i32 {
    let (x, y, z) = coords;
    let absx = x.abs();
    let absy = y.abs();
    let absz = z.abs();

    if absx >= absy && absx >= absz {
        return absx;
    }
    if absy >= absx && absy >= absz {
        return absy;
    }
    if absz >= absx && absz >= absy {
        return absz;
    }
    panic!("no max found!");
}

fn get_direction_list() -> Vec<Direction> {
    let contents = util::read_file_string("input.txt");
    let dir_strings = contents.split(",");
    let mut dir_vec = Vec::new();

    for dir_string in dir_strings {
        match dir_string.trim() {
            "n" => dir_vec.push(Direction::N),
            "ne" => dir_vec.push(Direction::NE),
            "nw" => dir_vec.push(Direction::NW),
            "s" => dir_vec.push(Direction::S),
            "se" => dir_vec.push(Direction::SE),
            "sw" => dir_vec.push(Direction::SW),
            _ => panic!("unknown direction in input: {}", dir_string),
        }
    }
    return dir_vec;
}
