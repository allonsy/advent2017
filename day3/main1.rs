


const TARGET_NUMBER : i32 = 368078;

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
    let mut num_sides_left_in_square = 2;
    let mut square_length = 1;
    let mut num_numbers_left_in_side = square_length;
    let mut current_direction = Direction::Right;
    let mut current_number = 1;
    let mut right_distance = 0;
    let mut up_distance = 0;

    loop {
        if current_number == TARGET_NUMBER {
            break;
        }

        if num_numbers_left_in_side == 0 {
            current_direction = current_direction.next();
            num_sides_left_in_square -= 1;
            if num_sides_left_in_square == 0 {
                square_length += 1;
                num_sides_left_in_square = 2;
            }
            num_numbers_left_in_side = square_length;
        }
        match current_direction {
            Direction::Right => { right_distance += 1; },
            Direction::Left => { right_distance -= 1; },
            Direction::Up => { up_distance += 1; },
            Direction::Down => { up_distance -= 1; }
        }
        current_number += 1;
        num_numbers_left_in_side -= 1;
    }
    println!("distance is: {}", i32::abs(right_distance) + i32::abs(up_distance));
}
