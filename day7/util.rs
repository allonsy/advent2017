use std::fs::File;
use std::io::Read;

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
