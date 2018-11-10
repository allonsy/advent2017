mod util;

use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

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
    SOUND(char),
    SET(char, Value),
    ADD(char, Value),
    MUL(char, Value),
    MOD(char, Value),
    RECOVER(char),
    JUMP_GREATER_ZERO(char, Value),
}

fn get_instructions() -> Vec<Instruction> {
    let lines = util::read_file_lines("input.txt");
    let mut instructions = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        match words[0] {
            "snd" => { instructions.push(Instruction::SOUND(words[1].parse::<char>().unwrap())); },
            "set" => { 
                    instructions.push(Instruction::SET(
                        words[1].parse::<char>().unwrap(),
                        Value::parse_value(words[2])
                    ));
                },
            "add" => { 
                instructions.push(Instruction::ADD(
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
            "mod" => { 
                instructions.push(Instruction::MOD(
                    words[1].parse::<char>().unwrap(),
                    Value::parse_value(words[2])
                ));
            },
            "rcv" => { 
                instructions.push(Instruction::RECOVER(
                    words[1].parse::<char>().unwrap()
                ));
            },
            "jgz" => { 
                instructions.push(Instruction::JUMP_GREATER_ZERO(
                    words[1].parse::<char>().unwrap(),
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
    last_played: Option<i64>,
    last_recover: Option<i64>,
    send_channel: Sender<i64>,
    recv_channel: Receiver<i64>,
    id: usize,
    num_sends: usize,
}

impl State {
    fn new(instructions: Vec<Instruction>, sc: Sender<i64>, rc: Receiver<i64>, id: usize) -> State {
        let mut st = State {
            instructions: instructions,
            instruction_ptr: 0,
            registers: HashMap::new(),
            last_played: None,
            last_recover: None,
            send_channel: sc,
            recv_channel: rc,
            id: id,
            num_sends: 0,
        };
        st.registers.insert('p', id as i64);
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

    fn exec_instruction(&mut self) -> Option<()> {
        if self.instruction_ptr >= self.instructions.len() {
            return Some(());
        }
        let cur_instruction = self.instructions[self.instruction_ptr].clone();
        match cur_instruction {
            Instruction::SOUND(reg) => {
                let val = self.get_register_value(reg);
                self.last_played = Some(val);
                self.instruction_ptr += 1;
            },
            Instruction::SET(reg, val) => {
                let val = self.get_value(val);
                self.registers.insert(reg, val);
                self.instruction_ptr += 1;
            },
            Instruction::ADD(reg, val) => {
                let val = self.get_value(val);
                let old_val = self.get_register_value(reg);
                self.registers.insert(reg, old_val + val);
                self.instruction_ptr += 1;
            },
            Instruction::MUL(reg, val) => {
                let val = self.get_value(val);
                let old_val = self.get_register_value(reg);
                self.registers.insert(reg, old_val * val);
                self.instruction_ptr += 1;
            },
            Instruction::MOD(reg, val) => {
                let val = self.get_value(val);
                let old_val = self.get_register_value(reg);
                self.registers.insert(reg, old_val % val);
                self.instruction_ptr += 1;
            },
            Instruction::RECOVER(reg) => {
                let val = self.get_register_value(reg);
                if val != 0 {
                    self.last_recover = self.last_played.clone();
                }
                self.instruction_ptr += 1;
            },
            Instruction::JUMP_GREATER_ZERO(reg, val) => {
                let reg_val = self.get_register_value(reg);
                if reg_val > 0 {
                    let jump_val = self.get_value(val);
                    self.instruction_ptr = (self.instruction_ptr as i64 + jump_val) as usize;
                } else {
                    self.instruction_ptr += 1;
                }
            }
        }
        None
    }
}

fn main() {
    let instructions = get_instructions();

    let (sender1, recv1) = channel();
    let (sender2, recv2) = channel();

    let mut state1 = State::new(instructions.clone(), sender2, recv1, 0);
    let mut state2 = State::new(instructions.clone(), sender1, recv2, 1);

    loop {
        state1.exec_instruction();
        if state1.last_recover.is_some() {
            println!("Number is: {}", state1.last_recover.unwrap());
            return;
        }
    }
}