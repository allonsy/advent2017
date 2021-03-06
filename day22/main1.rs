mod util;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&mut self) {
        let new_direction = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };

        *self = new_direction;
    }

    fn turn_left(&mut self) {
        let new_direction = match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        };

        *self = new_direction;
    }
}

struct Board {
    cells: Vec<Vec<bool>>,
    current_direction: Direction,
    current_pos: (usize, usize),
    num_infections: usize,
}

impl Board {
    fn create() -> Board {
        let lines = util::read_file_lines("input.txt");
        let mut cells = Vec::new();

        for line in lines {
            let mut row = Vec::new();
            for cell in line.chars() {
                if cell == '.' {
                    row.push(false);
                } else if cell == '#' {
                    row.push(true);
                }
            }
            cells.push(row);
        }

        let start_x = cells[0].len() / 2;
        let start_y = cells.len() / 2;

        Board {
            cells: cells,
            current_direction: Direction::Up,
            current_pos: (start_x, start_y),
            num_infections: 0,
        }
    }

    fn take_turn(&mut self) {
        let (mut cur_x, mut cur_y) = self.current_pos;

        let cur_cell = self.cells[cur_y][cur_x];
        if cur_cell {
            self.current_direction.turn_right();
        } else {
            self.current_direction.turn_left();
        }

        self.cells[cur_y][cur_x] = !cur_cell;
        if !cur_cell {
            self.num_infections += 1;
        }

        match self.current_direction {
            Direction::Up => {
                if cur_y == 0 {
                    self.add_up();
                } else {
                    cur_y -= 1;
                }
            }
            Direction::Down => {
                if cur_y == self.cells.len() - 1 {
                    self.add_down();
                }
                cur_y += 1;
            }
            Direction::Left => {
                if cur_x == 0 {
                    self.add_left();
                } else {
                    cur_x -= 1;
                }
            }
            Direction::Right => {
                if cur_x == self.cells[0].len() - 1 {
                    self.add_right();
                }
                cur_x += 1;
            }
        }

        self.current_pos = (cur_x, cur_y);
    }

    fn add_right(&mut self) {
        for row in &mut self.cells {
            row.push(false);
        }
    }

    fn add_left(&mut self) {
        for row in &mut self.cells {
            row.insert(0, false);
        }
    }

    fn add_up(&mut self) {
        if self.cells.is_empty() {
            panic!("No cells found when increasing size up");
        }

        let mut new_row = Vec::new();
        for _ in &self.cells[0] {
            new_row.push(false);
        }
        self.cells.insert(0, new_row);
    }

    fn add_down(&mut self) {
        if self.cells.is_empty() {
            panic!("No cells found when increasing size down");
        }

        let mut new_row = Vec::new();
        for _ in &self.cells[0] {
            new_row.push(false);
        }
        self.cells.push(new_row);
    }
}

fn main() {
    let mut board = Board::create();
    for _ in 0..10000 {
        board.take_turn();
    }

    println!("Number of infections: {}", board.num_infections);
}
