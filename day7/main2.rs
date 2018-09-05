mod util;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
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
        ProgramTower {
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
        ProgramTowerInput {
            name: name,
            weight: weight,
            children: children,
        }
    }
}

fn main() {
    let input = parse_input();
    let tower = create_tower(input);
    let (bad_node, good_weight) = analyze_children(&tower.borrow().children);
    println!(
        "new weight is: {}",
        get_new_balanced_weight(&bad_node.unwrap(), good_weight.unwrap())
    );
}

fn get_new_balanced_weight(tower: &Pointer, expected_weight: i64) -> i64 {
    let (bad_child, good_weight) = analyze_children(&tower.borrow().children);

    if bad_child.is_some() {
        if good_weight.is_some() {
            return get_new_balanced_weight(&bad_child.unwrap(), good_weight.unwrap());
        }
        let this_weight = tower.borrow().weight as i64;
        return get_new_balanced_weight(&bad_child.unwrap(), expected_weight - this_weight);
    }

    let bad_weight = get_bad_weight(&tower.borrow().children);
    if bad_weight.is_none() {
        let cur_weight = get_tower_weight(tower);
        let diff = expected_weight - cur_weight;
        return tower.borrow().weight as i64 + diff;
    }

    let (normal_weight, bad_node) = bad_weight.unwrap();
    let cur_weight = get_tower_weight(&bad_node);
    let diff = normal_weight - cur_weight;
    return bad_node.borrow().weight as i64 + diff;
}

fn get_tower_weight(tower: &Pointer) -> i64 {
    let mut sum: i64 = 0;
    sum += tower.borrow().weight as i64;

    for (_, child) in &tower.borrow().children {
        sum += get_tower_weight(&child);
    }
    return sum;
}

fn is_balanced(tower: &Pointer) -> bool {
    if tower.borrow().children.is_empty() {
        return true;
    }

    if tower.borrow().children.len() == 1 {
        for (_, child) in &tower.borrow().children {
            return is_balanced(child);
        }
    }

    let mut child_weight = None;
    for (_, child) in &tower.borrow().children {
        if !is_balanced(child) {
            return false;
        }
        let actual_child_weight = get_tower_weight(child);
        match child_weight {
            None => {
                child_weight = Some(actual_child_weight);
            }
            Some(wt) => {
                if wt != actual_child_weight {
                    return false;
                }
            }
        }
    }
    return true;
}

fn analyze_children(children: &HashMap<String, Pointer>) -> (Option<Pointer>, Option<i64>) {
    let mut bad_child = None;
    let mut bad_child_name = None;

    for (child_name, child) in children {
        if !is_balanced(child) {
            bad_child = Some(child.clone());
            bad_child_name = Some(child.borrow().name.clone());
            break;
        }
    }
    if bad_child.is_none() {
        return (None, None);
    }

    let bad_child_name_unwrap = bad_child_name.unwrap();
    let mut good_weight = None;

    for (child_name, child) in children {
        if *child_name != bad_child_name_unwrap {
            good_weight = Some(child.borrow().weight as i64);
            return (bad_child, good_weight);
        }
    }

    return (bad_child, None);
}

fn get_bad_weight(children: &HashMap<String, Pointer>) -> Option<(i64, Pointer)> {
    let mut freq_map: HashMap<i64, (i32, Pointer)> = HashMap::new();
    for (_, child) in children {
        let this_weight = get_tower_weight(child);
        freq_map.entry(this_weight).or_insert((0, child.clone())).0 += 1;
    }

    for (wt, freq) in freq_map {
        if freq.0 == 1 {
            let bad_child = freq.1;

            for (name, child) in children {
                if *name != bad_child.borrow().name {
                    return Some((get_tower_weight(child), bad_child));
                }
            }
        }
    }

    return None;
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
                this_node
                    .borrow_mut()
                    .children
                    .insert(child_name.clone(), ptr.clone());
                roots.remove(&child_name);
            } else {
                let new_child_node = ProgramTower::new(child_name.clone(), 0, HashMap::new());
                let child_ptr = Rc::new(RefCell::new(new_child_node));
                already_seen.insert(child_name.clone(), child_ptr.clone());
                this_node
                    .borrow_mut()
                    .children
                    .insert(child_name, child_ptr);
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
