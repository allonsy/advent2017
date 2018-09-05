mod util;

use std::collections::HashMap;

struct Instruction {
    register: String,
    command: Command,
    operand: i32,
    condition: Condition
}

enum Command {
    Inc,
    Dec
}

struct Condition {
    register: String,
    relation: Relation,
    operand: i32
}

enum Relation {
    LT, GT, LE, GE, EQ, NE
}

fn main() {
    let commands = get_commands();
    let mut cpu_state: HashMap<String, i32> = HashMap::new();
    for command in commands {
        run_command(&command, &mut cpu_state);
    }

    let mut high_value = None;
    for (_, reg_val) in cpu_state {
        match high_value {
            None => high_value = Some(reg_val),
            Some(old_val) => {
                if old_val < reg_val {
                    high_value = Some(reg_val);
                }
            }
        }
    }

    let high_num = high_value.unwrap();
    println!("value is: {}", high_num);
}

fn run_command(instruction: &Instruction, state: &mut HashMap<String, i32>) {
    let cond = &instruction.condition;
    if eval_condition(state, cond) {
        apply_tranform(
            state,
            instruction.register.clone(),
            &instruction.command,
            instruction.operand
        );
    }
}

fn eval_condition(state: &mut HashMap<String, i32>, condition: &Condition) -> bool {
    let register_value = get_register_value(state, condition.register.clone());
    let operand = condition.operand;

    match condition.relation {
        Relation::LT => return register_value < operand,
        Relation::LE => return register_value <= operand,
        Relation::GT => return register_value > operand,
        Relation::GE => return register_value >= operand,
        Relation::EQ => return register_value == operand,
        Relation::NE => return register_value != operand
    }
}

fn apply_tranform(
    state: &mut HashMap<String, i32>,
    register: String,
    command: &Command,
    operand: i32) {
    match *command {
        Command::Inc => *state.entry(register).or_insert(0) += operand,
        Command::Dec => *state.entry(register).or_insert(0) -= operand
    }
}

fn get_register_value(state: &mut HashMap<String, i32>, register: String) -> i32 {
    if state.contains_key(&register) {
        return *state.get(&register).unwrap();
    }

    state.insert(register, 0);
    return 0;
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
                operand: relation_operand
            }
        });
    }
    return commands;
}
