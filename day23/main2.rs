mod util;

use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
enum Value {
    NUMBER(i64),
    REGISTER(char)
}

impl Value {

    fn is_number(&self) -> bool {
        match &self {
            Value::NUMBER(_) => true,
            _ => false
        }
    }

    fn get_number(&self) -> i64 {
        match &self {
            Value::NUMBER(n) => *n,
            _ => panic!("value isn't a number!")
        }
    }

    fn get_register(&self) -> char {
        match &self {
            Value::REGISTER(c) => *c,
            _ => panic!("Value isn't a register")
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

#[derive(Clone)]
#[derive(Debug)]
enum Instruction {
    SET(char, Value),
    SUB(char, Value),
    MUL(char, Value),
    JUMP_NOT_ZERO(Value, Value),
}

fn get_instructions() -> Vec<Instruction> {
    let lines = util::read_file_lines("input.txt");
    let mut instructions = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        match words[0] {
            "set" => { 
                    instructions.push(Instruction::SET(
                        words[1].parse::<char>().unwrap(),
                        Value::parse_value(words[2])
                    ));
                },
            "sub" => { 
                instructions.push(Instruction::SUB(
                    words[1].parse::<char>().unwrap(),
                    Value::parse_value(words[2])
                ));
            },
            "mul" => { 
                instructions.push(Instruction::MUL(
                    words[1].parse::<char>().unwrap(),
                    Value::parse_value(words[2])
                ));
            },
            "jnz" => { 
                instructions.push(Instruction::JUMP_NOT_ZERO(
                    Value::parse_value(words[1]),
                    Value::parse_value(words[2])
                ));
            },
            _ => panic!("Unknown Instruction: {}", words[0])
        }
    }

    return instructions;
}

struct State {
    instructions: Vec<Instruction>,
    instruction_ptr: usize,
    registers: HashMap<char, i64>,
    num_muls: usize,
    first: usize,
}

impl State {
    fn new(instructions: Vec<Instruction>) -> State {
        let mut st = State {
            instructions: instructions,
            instruction_ptr: 0,
            registers: HashMap::new(),
            num_muls: 0,
            first: 0,
        };
        st.registers.insert('a', 1);
        st
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
            Instruction::SET(reg, val) => {
                let val = self.get_value(val);
                self.registers.insert(reg, val);
                self.instruction_ptr += 1;
            },
            Instruction::SUB(reg, val) => {
                let val = self.get_value(val);
                let old_val = self.get_register_value(reg);
                self.registers.insert(reg, old_val - val);
                self.instruction_ptr += 1;

                if reg == 'h' {
                    println!("hsub: {:?}", self.registers);
                }
            },
            Instruction::MUL(reg, val) => {
                let val = self.get_value(val);
                let old_val = self.get_register_value(reg);
                self.registers.insert(reg, old_val * val);
                self.instruction_ptr += 1;
                self.num_muls += 1;
            },
            Instruction::JUMP_NOT_ZERO(reg, val) => {
                if self.instruction_ptr == 23 {
                    println!("jnz23: {:?}", self.registers);
                    if self.first <= 3 {
                        self.first += 1;
                        let diff = self.registers.get(&'d').unwrap() - self.registers.get(&'g').unwrap();
                        self.registers.insert('g', 0);
                        self.registers.insert('d', diff);
                    }
                } else if self.instruction_ptr == 31 {
                    println!("jnz31: {:?}", self.registers);
                } else if self.instruction_ptr == 28 {
                    println!("jnz28: {:?}", self.registers);
                }
                let reg_val = self.get_value(reg);
                if reg_val != 0 {
                    let mut jump_val = self.get_value(val);
                    let new_ptr = self.instruction_ptr as i64 + jump_val;
                    if new_ptr < 0 {
                        jump_val = self.instructions.len() as i64;
                    }
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

    let mut state = State::new(instructions.clone());

    loop {

        state.exec_instruction();
        if state.instruction_ptr == instructions.len() {
            break;
        }
    }
    println!("value in h is: {}", state.registers.get(&'h').unwrap());
}