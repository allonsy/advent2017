mod util;

use std::collections::LinkedList;

struct Group {
    items: Vec<GroupItem>
}

enum GroupItem {
    Garbage(Vec<u8>),
    Group(Group)
}

fn main() {
    let mut queue = get_stream();
    consume_char(&mut queue);
    let group = parse_group(&mut queue);
    let score = calc_score(&group, 1);
    println!("score is: {}", score);
}

fn calc_score(group: &Group, this_group_score: i32) -> i32 {
    let mut sum = this_group_score;

    for item in &group.items {
        match item {
            GroupItem::Group(gp) => sum += calc_score(&gp, this_group_score + 1),
            _ => { }
        }
    }

    return sum;
}

fn parse_group(queue: &mut LinkedList<u8>) -> Group {
    let mut items = Vec::new();

    loop {
        let this_char = consume_char(queue);
        match this_char as char {
            '}' => break,
            '<' => items.push(GroupItem::Garbage(parse_garbage(queue))),
            '{' => items.push(GroupItem::Group(parse_group(queue))),
            _ => panic!("invalid character!")
        }
        let next_char = consume_char(queue);
        match next_char as char {
            '}' => break,
            ',' => { },
            _ => panic!("invalid character!")
        }
    }
    return Group { items: items };
}

fn parse_garbage(queue: &mut LinkedList<u8>) -> Vec<u8> {
    let mut chars = Vec::new();
    loop {
        let this_char = consume_char(queue);
        match this_char as char {
            '>' => break,
            '!' => {
                consume_char(queue);
            },
            _ => chars.push(this_char)
        }
    }
    return chars;
}

fn consume_char(queue: &mut LinkedList<u8>) -> u8 {
    return queue.pop_front().unwrap();
}

fn get_stream() -> LinkedList<u8> {
    let stream = util::read_file_string("input.txt");
    let mut queue: LinkedList<u8> = LinkedList::new();

    for byte in stream.into_bytes() {
        queue.push_back(byte);
    }

    return queue;
}
