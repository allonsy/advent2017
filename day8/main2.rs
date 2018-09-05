mod util;

use std::collections::HashMap;

struct Instruction {
    register: String,
    command: Command,
    operand: i32,
    condition: Condition,
}

enum Command {
    Inc,
    Dec,
}

struct Condition {
    register: String,
    relation: Relation,
    operand: i32,
}

enum Relation {
    LT,
    GT,
    LE,
    GE,
    EQ,
    NE,
}

struct State {
    cpu: HashMap<String, i32>,
    highest_value: Option<i32>,
}

fn main() {
    let commands = get_commands();
    let mut cpu_state = State {
        cpu: HashMap::new(),
        highest_value: None,
    };

    for command in commands {
        run_command(&command, &mut cpu_state);
    }
    println!("value is: {}", cpu_state.highest_value.unwrap());
}

fn run_command(instruction: &Instruction, state: &mut State) {
    let cond = &instruction.condition;
    if eval_condition(state, cond) {
        apply_tranform(
            state,
            instruction.register.clone(),
            &instruction.command,
            instruction.operand,
        );
    }
}

fn eval_condition(state: &mut State, condition: &Condition) -> bool {
    let register_value = get_register_value(state, condition.register.clone());
    let operand = condition.operand;

    match condition.relation {
        Relation::LT => return register_value < operand,
        Relation::LE => return register_value <= operand,
        Relation::GT => return register_value > operand,
        Relation::GE => return register_value >= operand,
        Relation::EQ => return register_value == operand,
        Relation::NE => return register_value != operand,
    }
}

fn apply_tranform(state: &mut State, register: String, command: &Command, operand: i32) {
    let new_val;
    {
        let entry = state.cpu.entry(register).or_insert(0);
        match *command {
            Command::Inc => *entry += operand,
            Command::Dec => *entry -= operand,
        }
        new_val = *entry;
    }
    update_highest_value(state, new_val);
}

fn get_register_value(state: &mut State, register: String) -> i32 {
    if state.cpu.contains_key(&register) {
        return *state.cpu.get(&register).unwrap();
    }

    state.cpu.insert(register, 0);
    update_highest_value(state, 0);
    return 0;
}

fn update_highest_value(state: &mut State, new_val: i32) {
    match state.highest_value {
        None => state.highest_value = Some(new_val),
        Some(old_val) => {
            state.highest_value = if old_val < new_val {
                Some(new_val)
            } else {
                Some(old_val)
            }
        }
    };
}

fn get_commands() -> Vec<Instruction> {
    let lines = util::read_file_lines("input.txt");
    let mut commands = Vec::new();
    for line in lines {
        let words = line.split(" ").collect::<Vec<&str>>();
        let register = words[0];
        let command = match words[1] {
            "inc" => Command::Inc,
            "dec" => Command::Dec,
            _ => panic!("Unknown command"),
        };

        let operand: i32 = words[2].parse().unwrap();
        let relation_register = words[4];

        let relation = match words[5] {
            ">" => Relation::GT,
            "<" => Relation::LT,
            "<=" => Relation::LE,
            ">=" => Relation::GE,
            "==" => Relation::EQ,
            "!=" => Relation::NE,
            _ => panic!("unknown relation: "),
        };

        let relation_operand: i32 = words[6].parse().unwrap();

        commands.push(Instruction {
            register: register.to_string(),
            command: command,
            operand: operand,
            condition: Condition {
                register: relation_register.to_string(),
                relation: relation,
                operand: relation_operand,
            },
        });
    }
    return commands;
}
