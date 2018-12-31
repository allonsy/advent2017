mod util;

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Instruction {
    Vert,
    Horiz,
    Junction,
    Space,
    Letter(char),
}

fn main() -> () {
    let instructions = get_input();
    let (mut row, mut col) = get_starting_point(&instructions);
    let mut dir = Direction::Down;
    let mut letters = Vec::new();
    let mut cont = true;
    let mut num_steps = 0;

    loop {
        if !cont {
            break;
        }
        let this_inst = &instructions[row as usize][col as usize];
        match this_inst {
            Instruction::Vert => {
                let (new_row, new_col) = proceed((row, col), &dir);
                row = new_row;
                col = new_col;
            }
            Instruction::Horiz => {
                let (new_row, new_col) = proceed((row, col), &dir);
                row = new_row;
                col = new_col;
            }
            Instruction::Letter(c) => {
                letters.push(*c);
                let (new_row, new_col) = proceed((row, col), &dir);
                row = new_row;
                col = new_col;
            }
            Instruction::Space => {
                cont = false;
            }
            Instruction::Junction => {
                let (new_dir, new_row, new_col) = compute_junction(&instructions, row, col, &dir);
                dir = new_dir;
                row = new_row;
                col = new_col;
            }
        }
        num_steps += 1;
    }
    println!("num_steps is: {}", num_steps - 1);
}

fn proceed((row, col): (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Down => (row + 1, col),
        Direction::Up => (row - 1, col),
        Direction::Left => (row, col - 1),
        Direction::Right => (row, col + 1),
    }
}

fn get_resultant_direction((old_r, old_c): (i32, i32), (new_r, new_c): (i32, i32)) -> Direction {
    if old_r > new_r {
        Direction::Up
    } else if old_r < new_r {
        Direction::Down
    } else {
        if old_c < new_c {
            Direction::Right
        } else if old_c > new_c {
            Direction::Left
        } else {
            panic!("old and new are the same!");
        }
    }
}

fn compute_junction(
    instructions: &Vec<Vec<Instruction>>,
    row: i32,
    col: i32,
    cur_dir: &Direction,
) -> (Direction, i32, i32) {
    let checks = match cur_dir {
        Direction::Down => vec![(row, col - 1), (row + 1, col), (row, col + 1)],
        Direction::Up => vec![(row, col + 1), (row - 1, col), (row, col - 1)],
        Direction::Left => vec![(row - 1, col), (row, col - 1), (row + 1, col)],
        Direction::Right => vec![(row - 1, col), (row, col + 1), (row + 1, col)],
    };

    let mut valid_checks = Vec::new();
    for (r, c) in checks {
        if r >= 0 && r < instructions.len() as i32 {
            if c >= 0 && c < instructions[r as usize].len() as i32 {
                valid_checks.push((r, c));
            }
        }
    }

    for (r, c) in valid_checks {
        match &instructions[r as usize][c as usize] {
            Instruction::Space => {}
            _ => {
                let new_dir = get_resultant_direction((row, col), (r, c));
                return (new_dir, r, c);
            }
        }
    }
    panic!("Multiple choices possible at row: {}, col: {}", row, col);
}

fn get_input() -> Vec<Vec<Instruction>> {
    let mut instructions = Vec::new();
    let lines = util::read_file_lines("input.txt");
    for line in lines {
        let mut row = Vec::new();
        for ch in line.chars() {
            match ch {
                '|' => row.push(Instruction::Vert),
                '-' => row.push(Instruction::Horiz),
                '+' => row.push(Instruction::Junction),
                ' ' => row.push(Instruction::Space),
                _ => row.push(Instruction::Letter(ch)),
            }
        }
        instructions.push(row);
    }

    instructions
}

fn get_starting_point(instructions: &Vec<Vec<Instruction>>) -> (i32, i32) {
    let mut i = 0;
    for ch in &instructions[0] {
        match ch {
            Instruction::Space => {}
            _ => return (0, i),
        }
        i += 1;
    }
    panic!("no stating point found")
}
