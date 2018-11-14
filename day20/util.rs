use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn read_file_lines(fname: &str) -> Vec<String> {
    let mut f = File::open(fname).unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut rows = Vec::new();

    for fline in contents.lines() {
        if !fline.is_empty() {
            rows.push(fline.to_owned());
        }
    }
    return rows;
}

pub fn read_file_string(fname: &str) -> String {
    let mut f = File::open(fname).unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    return contents;
}

pub fn pause() {
    print!("continue? ");
    std::io::stdout().flush();
    let mut garbage = String::new();
    std::io::stdin().read_line(&mut garbage).unwrap();
}
