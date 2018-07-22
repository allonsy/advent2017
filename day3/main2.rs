


const TARGET_NUMBER : i32 = 368078;
const ARRAY_LENGTH : usize = 1000;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(self) -> Direction {
        match self {
            Direction::Up => { Direction::Left },
            Direction::Left => { Direction::Down },
            Direction::Down => { Direction::Right },
            Direction::Right => { Direction::Up }
        }
    }
}

fn main() {
    let mut rows : [[Option<i32> ; ARRAY_LENGTH] ; ARRAY_LENGTH] =
        [[None ; ARRAY_LENGTH] ; ARRAY_LENGTH];

    let mut cur_coord = (0,0);
    let mut cur_dir = Direction::Right;

    let mut num_sides_left = 1;
    let mut num_left_in_side = 1;
    let mut side_length = 0;

    loop {
        let (actual_x, actual_y) = convert_coords(cur_coord);


        let new_value = calculate_value(cur_coord, &rows);
        if new_value > TARGET_NUMBER {
            println!("value is: {}", new_value);
            break;
        }
        rows[actual_x as usize][actual_y as usize] = Some(new_value);

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
        cur_coord = increment_coords(cur_coord, &cur_dir);
    }
}

fn increment_coords((x,y) : (i32, i32), dir : &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => { (x, y + 1) },
        Direction::Down => { (x, y - 1) },
        Direction::Left => { (x - 1, y) },
        Direction::Right => { (x + 1, y) }
    }
}

fn convert_coords((x,y) : (i32, i32)) -> (i32, i32) {
    let adjust_coord = | c | {
        if c < 0 {
            (ARRAY_LENGTH as i32) + c
        } else {
            c
        }
    };

    (adjust_coord(x),adjust_coord(y))
}

fn calculate_value((x,y) : (i32, i32), rows: &[[Option<i32> ; ARRAY_LENGTH] ; ARRAY_LENGTH]) -> i32 {
    if x == 0 && y == 0 {
        return 1;
    }

    let coords = vec![(x+1, y), (x+1, y+1), (x, y+1), (x-1, y+1), (x-1, y), (x-1, y-1), (x, y-1), (x+1, y-1)];
    let mut sum = 0;
    for coord in coords {
        let (actual_x, actual_y) = convert_coords(coord);
        match rows[actual_x as usize][actual_y as usize] {
            None => {},
            Some(val) => { sum += val; }
        }
    }

    return sum;
}
