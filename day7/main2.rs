
mod util;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type Pointer = Rc<RefCell<ProgramTower>>;

#[derive(Debug)]
struct ProgramTower {
    name: String,
    weight: i32,
    children: HashMap<String, Pointer>,
}

impl ProgramTower {
    fn new(name: String, weight: i32, children: HashMap<String, Pointer>) -> ProgramTower {
        ProgramTower{
            name: name,
            weight: weight,
            children: children,
        }
    }
}

struct ProgramTowerInput {
    name: String,
    weight: i32,
    children: Vec<String>,
}

impl ProgramTowerInput {
    fn new(name: String, weight: i32, children: Vec<String>) -> ProgramTowerInput {
        ProgramTowerInput{
            name: name,
            weight: weight,
            children: children,
        }
    }
}

fn main() {
    let input = parse_input();
    let tower = create_tower(input));
    println!("root is: {}, {}", tower.name, is_balanced(&tower));
}

fn find_balanced_weight(tower: Pointer, whole_tower_ptr: Pointer) -> i32 {
    if is_balanced(tower.borrow()) {
        return 0;
    }
    if tower.borrow().children.is_empty() {
        return 0;
    }
    if tower.borrow().children.len() == 1 {
        for child in tower.borrow().children.values() {
            return find_balanced_weight(child.borrow_mut(), whole_tower_ptr);
        }
    }

    for child in tower.borrow().children.values() {
        if !is_balanced(child.borrow()) {
            return find_balanced_weight(child.borrow_mut(), whole_tower_ptr);
        }
    }
    if tower.borrow().children.len() == 2 {
        return process_two(tower, whole_tower_ptr);
    } else {
        return process_many(tower);
    }

}

fn process_many(tower: Pointer) -> i32 {
    HashMap<i32, Vec<Pointer>> frequency = HashMap::new();

    for child in tower.borrow().children() {
        let tower_weight = get_tower_weight(child.borrow());
        if frequency.contains_key(tower_weight) {
            frequency.get(tower_weight).unwrap().push(child.clone());
        } else {
            frequency.insert(tower_weight, child.clone());
        }
    }
    if frequency.len() != 2 {
        panic!("frequency has length: {}", frequency.len());
    }
    let freq_keys: Vec<i32> = frequency.keys().collect();
    if frequency.get(freq_keys[0]).unwrap().len() == 2 {
        return freq_keys[0];
    } else {
        return freq_keys[1];
    }
}

fn process_two(tower: Pointer, whole_tower_ptr: Pointer) -> i32 {
    let children: Vec<Pointer> = tower.borrow().values().collect();
}

fn is_balanced(tower: &ProgramTower) -> bool {
    if tower.children.is_empty() {
        return true;
    } else {
        let mut child_weight = None;
        for child in tower.children.values() {
            let new_weight = get_tower_weight(&child.borrow());

            if child_weight.is_none() {
                child_weight = Some(new_weight);
            } else {
                if new_weight != child_weight.unwrap() {
                    return false;
                }
            }
        }
    }

    true
}

fn get_tower_weight(tower: &ProgramTower) -> i32 {
    let mut total = tower.weight;
    for child in tower.children.values() {
        total += get_tower_weight(&child.borrow());
    }

    total
}

fn create_tower(input: Vec<ProgramTowerInput>) -> Pointer {
    let mut roots: HashMap<String, Pointer> = HashMap::new();
    let mut already_seen: HashMap<String, Pointer> = HashMap::new();

    for node in input {
        let this_node: Pointer = if already_seen.contains_key(&node.name) {
            let val = already_seen.get(&node.name).unwrap();
            val.borrow_mut().weight = node.weight;
            val.clone()
        } else {
            let new_node = ProgramTower::new(node.name.clone(), node.weight, HashMap::new());
            let new_pointer = Rc::new(RefCell::new(new_node));
            already_seen.insert(node.name.clone(), new_pointer.clone());
            roots.insert(node.name, new_pointer.clone());
            new_pointer
        };

        for child_name in node.children {
            if already_seen.contains_key(&child_name) {
                let ptr = already_seen.get(&child_name).unwrap();
                this_node.borrow_mut().children.insert(child_name.clone(), ptr.clone());
                roots.remove(&child_name);
            } else {
                let new_child_node = ProgramTower::new(child_name.clone(), 0, HashMap::new());
                let child_ptr = Rc::new(RefCell::new(new_child_node));
                already_seen.insert(child_name.clone(), child_ptr.clone());
                this_node.borrow_mut().children.insert(child_name, child_ptr);
            }
        }
    }
    if roots.len() != 1 {
        panic!("failing out: {}", roots.len());
    }
    roots.values().next().unwrap().clone()
}

fn parse_input() -> Vec<ProgramTowerInput> {
    let input_lines = util::read_file_lines("input.txt");
    let mut tower_input = Vec::new();
    for line in input_lines {
        let words: Vec<&str> = line.split(" ").collect();
        let prog_name: String = words[0].to_owned();
        let weight_full_str: String = words[1].to_owned();
        let weight_length = weight_full_str.len();
        let weight: String = (&weight_full_str[1..weight_length - 1]).to_owned();
        let weight_num = weight.parse::<i32>().unwrap();
        if words.len() == 2 {
            // no children
            tower_input.push(ProgramTowerInput::new(prog_name, weight_num, Vec::new()));
        } else {
            let mut children: Vec<String> = Vec::new();
            let child_names = &words[3..];
            for child in child_names {
                let child_str = child.to_owned().to_string();
                let child_len = child_str.len();
                if last_char_comma(&child_str) {
                    children.push(child_str[..child_len - 1].to_string());
                } else {
                    children.push(child_str);
                }
            }
            tower_input.push(ProgramTowerInput::new(prog_name, weight_num, children));
        }
    }
    return tower_input;
}

fn last_char_comma(name: &String) -> bool {
    let bytes = name.clone().into_bytes();
    let bytes_len = bytes.len();
    bytes[bytes_len - 1] == ',' as u8
}
