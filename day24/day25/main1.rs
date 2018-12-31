enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct Machine {
    tape: Vec<bool>,
    cursor: usize,
    state: State,
    num_ones: usize,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            tape: vec![false],
            cursor: 0,
            state: State::A,
            num_ones: 0,
        }
    }

    fn iterate(&mut self) {
        match &self.state {
            State::A => {
                if self.get_current_val() == false {
                    self.tape[self.cursor] = true;
                    self.num_ones += 1;
                    self.move_right();
                    self.state = State::B;
                } else {
                    self.tape[self.cursor] = false;
                    self.num_ones -= 1;
                    self.move_left();
                    self.state = State::E;
                }
            }
            State::B => {
                if self.get_current_val() == false {
                    self.tape[self.cursor] = true;
                    self.num_ones += 1;
                    self.move_left();
                    self.state = State::C;
                } else {
                    self.tape[self.cursor] = false;
                    self.num_ones -= 1;
                    self.move_right();
                    self.state = State::A;
                }
            }
            State::C => {
                if self.get_current_val() == false {
                    self.tape[self.cursor] = true;
                    self.num_ones += 1;
                    self.move_left();
                    self.state = State::D;
                } else {
                    self.tape[self.cursor] = false;
                    self.num_ones -= 1;
                    self.move_right();
                    self.state = State::C;
                }
            }
            State::D => {
                if self.get_current_val() == false {
                    self.tape[self.cursor] = true;
                    self.num_ones += 1;
                    self.move_left();
                    self.state = State::E;
                } else {
                    self.tape[self.cursor] = false;
                    self.num_ones -= 1;
                    self.move_left();
                    self.state = State::F;
                }
            }
            State::E => {
                if self.get_current_val() == false {
                    self.tape[self.cursor] = true;
                    self.num_ones += 1;
                    self.move_left();
                    self.state = State::A;
                } else {
                    self.move_left();
                    self.state = State::C;
                }
            }
            State::F => {
                if self.get_current_val() == false {
                    self.tape[self.cursor] = true;
                    self.num_ones += 1;
                    self.move_left();
                    self.state = State::E;
                } else {
                    self.move_right();
                    self.state = State::A;
                }
            }
        }
    }

    fn move_right(&mut self) {
        self.cursor += 1;
        if self.cursor == self.tape.len() {
            self.tape.push(false);
        }
    }

    fn move_left(&mut self) {
        if self.cursor == 0 {
            self.tape.insert(0, false);
        } else {
            self.cursor -= 1;
        }
    }

    fn get_current_val(&self) -> bool {
        return self.tape[self.cursor];
    }
}

fn main() {
    let mut machine = Machine::new();
    for _ in 0..12386363 {
        machine.iterate();
    }

    println!("num ones is: {}", machine.num_ones);
}
