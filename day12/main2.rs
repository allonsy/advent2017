mod util;

use std::collections::HashMap;
use std::collections::HashSet;

struct Node {
    name: i32,
    connections: HashSet<i32>,
}

fn main() {
    let mut node_map = get_nodes();
    let mut num_groups = 0;
    let mut cur_node = get_any_node(&node_map);

    while cur_node.is_some() {
        let mut seen_before = HashSet::new();
        {
            let this_node = node_map.get(&cur_node.unwrap()).unwrap();
            get_connected_nodes(&mut seen_before, &node_map, this_node);
        }
        num_groups += 1;

        for name in seen_before {
            node_map.remove(&name);
        }
        cur_node = get_any_node(&node_map);
    }

    println!("number of groups is: {}", num_groups);
}

fn get_connected_nodes(
    seen_before: &mut HashSet<i32>,
    node_map: &HashMap<i32, Node>,
    cur_node: &Node,
) {
    seen_before.insert(cur_node.name);

    for conn in &cur_node.connections {
        if !seen_before.contains(&conn) {
            get_connected_nodes(seen_before, node_map, node_map.get(conn).unwrap());
        }
    }
}

fn get_nodes() -> HashMap<i32, Node> {
    let mut node_args = Vec::new();
    let lines = util::read_file_lines("input.txt");

    for line in lines {
        let node_desc: Vec<&str> = line.split("<->").collect();
        let prefix = node_desc[0];
        let suffix = node_desc[1];
        let node_name: i32 = prefix.trim().parse().unwrap();
        let conn_names: Vec<&str> = suffix.split(",").collect();

        let mut connections = Vec::new();
        for conn in conn_names {
            connections.push(conn.trim().parse::<i32>().unwrap());
        }
        node_args.push((node_name, connections))
    }
    return gen_node_map(node_args);
}

fn gen_node_map(nodes: Vec<(i32, Vec<i32>)>) -> HashMap<i32, Node> {
    let mut node_map = HashMap::new();

    for node in nodes {
        for conn in node.1 {
            create_connection(&mut node_map, node.0, conn);
        }
    }
    return node_map;
}

fn create_connection(node_map: &mut HashMap<i32, Node>, from: i32, to: i32) {
    if from == to {
        if !node_map.contains_key(&from) {
            let new_node = Node {
                name: from,
                connections: HashSet::new(),
            };
            node_map.insert(from, new_node);
        }
    }

    if node_map.contains_key(&from) {
        node_map.get_mut(&from).unwrap().connections.insert(to);
    } else {
        let mut new_conns = HashSet::new();
        new_conns.insert(to);
        let new_node = Node {
            name: from,
            connections: new_conns,
        };
        node_map.insert(from, new_node);
    }

    if node_map.contains_key(&to) {
        node_map.get_mut(&to).unwrap().connections.insert(from);
    } else {
        let mut new_conns = HashSet::new();
        new_conns.insert(from);
        let new_node = Node {
            name: to,
            connections: new_conns,
        };
        node_map.insert(to, new_node);
    }
}

fn get_any_node(node_map: &HashMap<i32, Node>) -> Option<i32> {
    for (key, _) in node_map {
        return Some(*key);
    }

    None
}
