mod util;

#[derive(Clone)]
#[derive(PartialEq)]
struct Bridge {
    port_a: u64,
    port_b: u64
}

fn get_strongest_bridge(bridges: &Vec<Bridge>, starting_num: u64) -> i64 {
    let mut strongest = -1;
    for bridge in bridges {
        if bridge.port_a == starting_num || bridge.port_b == starting_num {
            let mut new_bridges = bridges.clone();
            remove_bridge(&mut new_bridges, &bridge);
            let other_port = get_other_port(&bridge, starting_num);
            let bridge_strength = starting_num + other_port;
            let chain_strength = get_strongest_bridge(&new_bridges, other_port);
            let possible_strongest = if chain_strength > -1 {
                bridge_strength as i64 + chain_strength
            } else {
                bridge_strength as i64
            };
            if possible_strongest > strongest {
                strongest = possible_strongest;
            }
        }
    }
    return strongest;
}

fn get_other_port(bridge: &Bridge, starting_num: u64) -> u64 {
    if bridge.port_a == starting_num {
        return bridge.port_b;
    } else {
        return bridge.port_a;
    }
}

fn main() {
    let bridges = read_bridges();
    println!("strongest bridge: {}", get_strongest_bridge(&bridges, 0));
}

fn read_bridges() -> Vec<Bridge> {
    let lines = util::read_file_lines("input.txt");
    let mut bridges = Vec::new();

    for line in lines {
        let ports: Vec<&str> = line.split("/").collect();
        let first_port = ports[0].parse::<u64>().unwrap();
        let second_port = ports[1].parse::<u64>().unwrap();
        bridges.push(Bridge {
            port_a: first_port,
            port_b: second_port
        });
    }
    return bridges;
}

fn remove_bridge(bridges: &mut Vec<Bridge>, bridge: &Bridge) {
    for i in 0..bridges.len() {
        if bridges[i] == *bridge {
            bridges.remove(i);
            return;
        }
    }
    panic!("Bridge not found for removal!");
}