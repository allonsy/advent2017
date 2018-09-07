mod util;

use std::collections::HashMap;

#[derive(Debug)]
struct Layer {
    cur_pos: i32,
    range: i32,
    is_down: bool,
}

fn main() {
    let layers = get_layers();
    let mut delay = 1;

    loop {
        let mut caught = false;

        for cur_layer_no in 0..99 {
            let num_ticks = delay + cur_layer_no;
            let cur_layer_opt = layers.get(&cur_layer_no);
            if cur_layer_opt.is_some() {
                let cur_layer = cur_layer_opt.unwrap();
                if num_ticks % ((cur_layer.range - 1) * 2) == 0 {
                    caught = true;
                    break;
                }
            }
        }
        if caught == false {
            break;
        }
        delay += 1;
    }

    println!("delay is: {}", delay);
}

fn get_layers() -> HashMap<i32, Layer> {
    let lines = util::read_file_lines("input.txt");
    let mut layer_map = HashMap::new();

    for line in lines {
        let split: Vec<&str> = line.split(": ").collect();
        let layer_no: i32 = split[0].trim().parse().unwrap();
        let range_no: i32 = split[1].trim().parse().unwrap();

        let new_layer = Layer {
            cur_pos: 0,
            range: range_no,
            is_down: true,
        };
        layer_map.insert(layer_no, new_layer);
    }
    return layer_map;
}

fn tick_layers(layers: &mut HashMap<i32, Layer>) {
    for (_, layer) in layers {
        if layer.is_down {
            layer.cur_pos += 1;
            if layer.cur_pos == layer.range {
                layer.is_down = false;
                layer.cur_pos -= 2;
                if layer.cur_pos < 0 {
                    layer.cur_pos = 0;
                }
            }
        } else {
            layer.cur_pos -= 1;
            if layer.cur_pos < 0 {
                layer.is_down = true;
                layer.cur_pos = if layer.range > 1 { 1 } else { 0 }
            }
        }
    }
}

fn reset_layers(layers: &mut HashMap<i32, Layer>) {
    for (_, layer) in layers {
        layer.cur_pos = 0;
    }
}
