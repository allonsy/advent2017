mod util;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

type Queue = Rc<RefCell<VecDeque<i64>>>;

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
    SEND(char),
    SET(char, Value),
    ADD(char, Value),
    MUL(char, Value),
    MOD(char, Value),
    RECEIVE(char),
    JUMP_GREATER_ZERO(Value, Value),
}

fn get_instructions() -> Vec<Instruction> {
    let lines = util::read_file_lines("input.txt");
    let mut instructions = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        match words[0] {
            "snd" => { instructions.push(Instruction::SEND(words[1].parse::<char>().unwrap())); },
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
                instructions.push(Instruction::RECEIVE(
                    words[1].parse::<char>().unwrap()
                ));
            },
            "jgz" => { 
                instructions.push(Instruction::JUMP_GREATER_ZERO(
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
    send_channel: Queue,
    recv_channel: Queue,
    id: usize,
    num_sends: usize,
}

impl State {
    fn new(instructions: Vec<Instruction>, sc: Queue, rc: Queue, id: usize) -> State {
        let mut st = State {
            instructions: instructions,
            instruction_ptr: 0,
            registers: HashMap::new(),
            send_channel: sc,
            recv_channel: rc,
            id: id,
            num_sends: 0
        };
        st.registers.insert('p', st.id as i64);
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

    fn is_locked(&self) -> bool {
        if self.instruction_ptr >= self.instructions.len() {
            return true;
        }
        let is_recv_inst = match self.instructions[self.instruction_ptr] {
            Instruction::RECEIVE(_) => true ,
            _ => false
        };
        return is_recv_inst && self.recv_channel.borrow().is_empty();
    }

    fn exec_instruction(&mut self) {
        let cur_instruction = self.instructions[self.instruction_ptr].clone();
        match cur_instruction {
            Instruction::SEND(reg) => {
                let val = self.get_register_value(reg);
                self.send_channel.borrow_mut().push_back(val);
                self.num_sends += 1;
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
            Instruction::RECEIVE(reg) => {
                let mut recv_queue = self.recv_channel.borrow_mut();
                match recv_queue.pop_front() {
                    Some(v) => {
                        self.registers.insert(reg, v);
                        self.instruction_ptr += 1;
                    },
                    None => { }
                };
            },
            Instruction::JUMP_GREATER_ZERO(reg, val) => {
                let reg_val = self.get_value(reg);
                if reg_val > 0 {
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

    let queue0 = Rc::new(RefCell::new(VecDeque::new()));
    let queue1 = Rc::new(RefCell::new(VecDeque::new()));

    let mut state1 = State::new(instructions.clone(), queue1.clone(), queue0.clone(), 0);
    let mut state2 = State::new(instructions.clone(), queue0.clone(), queue1.clone(), 1);

    loop {

        state1.exec_instruction();
        state2.exec_instruction();

        if state1.is_locked() && state2.is_locked() {
            println!("exiting out due to deadlock!");
            break;
        }
    }
    println!("t1 number of sends is: {}", state2.num_sends);
}