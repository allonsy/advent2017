const TARGET_NUMBER: i32 = 368078;
const ARRAY_LENGTH: usize = 1000;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

fn main() {
    let mut rows: [[Option<i32>; ARRAY_LENGTH]; ARRAY_LENGTH] =
        [[None; ARRAY_LENGTH]; ARRAY_LENGTH];

    let mut cur_num = 1;
    let mut cur_coord = (0, 0);
    let mut cur_dir = Direction::Right;

    let mut num_sides_left = 1;
    let mut num_left_in_side = 1;
    let mut side_length = 0;

    while cur_num < TARGET_NUMBER {
        let (actual_x, actual_y) = convert_coords(cur_coord);

        rows[actual_x as usize][actual_y as usize] = Some(cur_num);

        if num_left_in_side < 1 {
            if num_sides_left < 1 {
                side_length += 1;
                num_sides_left = 1;
            } else {
                num_sides_left -= 1;
            }
            cur_dir = cur_dir.next();
            num_left_in_side = side_length;
        } else {
            num_left_in_side -= 1;
        }
        cur_num += 1;
        cur_coord = increment_coords(cur_coord, &cur_dir);
    }

    println!("distance is: {}", get_distance(cur_coord));
}

fn get_distance((x, y): (i32, i32)) -> i32 {
    i32::abs(x) + i32::abs(y)
}

fn increment_coords((x, y): (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn convert_coords((x, y): (i32, i32)) -> (i32, i32) {
    let adjust_coord = |c| {
        if c < 0 {
            (ARRAY_LENGTH as i32) + c
        } else {
            c
        }
    };

    (adjust_coord(x), adjust_coord(y))
}
