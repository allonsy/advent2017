mod util;

use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Value {
    NUMBER(i64),
    REGISTER(char),
}

impl Value {
    fn is_number(&self) -> bool {
        match &self {
            Value::NUMBER(_) => true,
            _ => false,
        }
    }

    fn get_number(&self) -> i64 {
        match &self {
            Value::NUMBER(n) => *n,
            _ => panic!("value isn't a number!"),
        }
    }

    fn get_register(&self) -> char {
        match &self {
            Value::REGISTER(c) => *c,
            _ => panic!("Value isn't a register"),
        }
    }

    fn parse_value(word: &str) -> Value {
        let num_parse = word.parse::<i64>();
        if num_parse.is_ok() {
            return Value::NUMBER(num_parse.unwrap());
        }

        let char_parse = word.parse::<char>();
        if char_parse.is_ok() {
            return Value::REGISTER(char_parse.unwrap());
        }

        panic!("Cannot parse \"{}\" into a char or int", word);
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    SOUND(char),
    SET(char, Value),
    ADD(char, Value),
    MUL(char, Value),
    MOD(char, Value),
    RECOVER(char),
    JUMP_GREATER_ZERO(Value, Value),
}

fn get_instructions() -> Vec<Instruction> {
    let lines = util::read_file_lines("input.txt");
    let mut instructions = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        match words[0] {
            "snd" => {
                instructions.push(Instruction::SOUND(words[1].parse::<char>().unwrap()));
            }
            "set" => {
                instructions.push(Instruction::SET(
                    words[1].parse::<char>().unwrap(),
                    Value::parse_value(words[2]),
                ));
            }
            "add" => {
                instructions.push(Instruction::ADD(
                    words[1].parse::<char>().unwrap(),
                    Value::parse_value(words[2]),
                ));
            }
            "mul" => {
                instructions.push(Instruction::MUL(
                    words[1].parse::<char>().unwrap(),
                    Value::parse_value(words[2]),
                ));
            }
            "mod" => {
                instructions.push(Instruction::MOD(
                    words[1].parse::<char>().unwrap(),
                    Value::parse_value(words[2]),
                ));
            }
            "rcv" => {
                instructions.push(Instruction::RECOVER(words[1].parse::<char>().unwrap()));
            }
            "jgz" => {
                instructions.push(Instruction::JUMP_GREATER_ZERO(
                    Value::parse_value(words[1]),
                    Value::parse_value(words[2]),
                ));
            }
            _ => panic!("Unknown Instruction: {}", words[0]),
        }
    }

    return instructions;
}

struct State {
    instructions: Vec<Instruction>,
    instruction_ptr: usize,
    registers: HashMap<char, i64>,
    last_played: Option<i64>,
    last_recover: Option<i64>,
}

impl State {
    fn new(instructions: Vec<Instruction>) -> State {
        State {
            instructions: instructions,
            instruction_ptr: 0,
            registers: HashMap::new(),
            last_played: None,
            last_recover: None,
        }
    }

    fn get_register_value(&mut self, reg_name: char) -> i64 {
        return *self.registers.entry(reg_name).or_insert(0);
    }

    fn get_value(&mut self, val: Value) -> i64 {
        if val.is_number() {
            return val.get_number();
        }
        return self.get_register_value(val.get_register());
    }

    fn exec_instruction(&mut self) {
        let cur_instruction = self.instructions[self.instruction_ptr].clone();
        match cur_instruction {
            Instruction::SOUND(reg) => {
                let val = self.get_register_value(reg);
                self.last_played = Some(val);
                self.instruction_ptr += 1;
            }
            Instruction::SET(reg, val) => {
                let val = self.get_value(val);
                self.registers.insert(reg, val);
                self.instruction_ptr += 1;
            }
            Instruction::ADD(reg, val) => {
                let val = self.get_value(val);
                let old_val = self.get_register_value(reg);
                self.registers.insert(reg, old_val + val);
                self.instruction_ptr += 1;
            }
            Instruction::MUL(reg, val) => {
                let val = self.get_value(val);
                let old_val = self.get_register_value(reg);
                self.registers.insert(reg, old_val * val);
                self.instruction_ptr += 1;
            }
            Instruction::MOD(reg, val) => {
                let val = self.get_value(val);
                let old_val = self.get_register_value(reg);
                self.registers.insert(reg, old_val % val);
                self.instruction_ptr += 1;
            }
            Instruction::RECOVER(reg) => {
                let val = self.get_register_value(reg);
                if val != 0 {
                    self.last_recover = self.last_played.clone();
                }
                self.instruction_ptr += 1;
            }
            Instruction::JUMP_GREATER_ZERO(reg, val) => {
                let reg_val = self.get_value(reg);
                if reg_val > 0 {
                    let jump_val = self.get_value(val);
                    self.instruction_ptr = (self.instruction_ptr as i64 + jump_val) as usize;
                } else {
                    self.instruction_ptr += 1;
                }
            }
        }
    }
}

fn main() {
    let instructions = get_instructions();
    let mut state = State::new(instructions);

    loop {
        state.exec_instruction();
        if state.last_recover.is_some() {
            println!("Number is: {}", state.last_recover.unwrap());
            return;
        }
    }
}
