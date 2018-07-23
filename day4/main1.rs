mod util;

use std::collections::HashSet;

fn main() {
    let passphrases = get_passphrases();

    let mut num_valid = 0;

    for phrase in passphrases {
        let is_valid = check_validity(phrase);
        if is_valid {
            num_valid += 1
        }
    }
    println!("number of valid phases: {}", num_valid);
}

fn check_validity(words: Vec<String>) -> bool {
    let mut words_seen = HashSet::new();

    for word in words {
        if words_seen.contains(&word) {
            return false;
        }
        words_seen.insert(word);
    }
    return true;
}

fn get_passphrases() -> Vec<Vec<String>> {
    let lines = util::read_file_lines("input.txt");

    let mut phrases: Vec<Vec<String>> = Vec::new();

    for line in lines {
        let mut words: Vec<String> = Vec::new();
        for word in line.split_whitespace() {
            words.push(word.to_string());
        }
        phrases.push(words);
    }
    return phrases;
}
