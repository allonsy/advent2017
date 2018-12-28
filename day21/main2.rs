mod util;
use std::collections::HashMap;

type Rulebook = HashMap<String, String>;

struct Image {
    pixels: Vec<Vec<bool>>,
}

impl Image {
    fn new(str_rep: &str) -> Image {
        Image {
            pixels: Image::get_pixels_from_string(str_rep),
        }
    }

    fn get_pixels_from_string(str_rep: &str) -> Vec<Vec<bool>> {
        let rows: Vec<&str> = str_rep.split("/").collect();

        let mut pixels = Vec::new();
        for row in rows {
            let mut pixel_row = Vec::new();
            for ch in row.chars() {
                match ch {
                    '#' => pixel_row.push(true),
                    '.' => pixel_row.push(false),
                    _ => panic!("Unknown pixel: {}", ch),
                }
            }
            pixels.push(pixel_row);
        }
        return pixels;
    }

    fn get_str_rep(&self) -> String {
        let mut rep = String::new();

        let mut row_num = 0;
        for row in &self.pixels {
            for b in row {
                match b {
                    true => rep += "#",
                    false => rep += ".",
                }
            }
            if row_num != row.len() - 1 {
                rep += "/";
            }
            row_num += 1;
        }
        return rep;
    }

    fn rotate(&self) -> Image {
        let pixels = &self.pixels;
        let side_length = pixels.len();

        let mut new_pixels = Vec::new();

        for col in 0..side_length {
            let mut new_row = Vec::new();
            for row in 0..side_length {
                new_row.push(pixels[row][col]);
            }
            new_row.reverse();
            new_pixels.push(new_row);
        }

        Image { pixels: new_pixels }
    }

    fn flip_horiz(&self) -> Image {
        let mut pixels = self.pixels.clone();
        pixels.reverse();
        Image { pixels: pixels }
    }

    fn flip_vert(&self) -> Image {
        let mut pixels = self.pixels.clone();

        for row in &mut pixels {
            row.reverse();
        }

        Image { pixels: pixels }
    }

    fn lookup(self, rulebook: &Rulebook) -> Image {
        let rot1 = self.rotate();
        let rot2 = rot1.rotate();
        let rot3 = rot2.rotate();

        let keys = vec![
            self.get_str_rep(),
            rot1.get_str_rep(),
            rot2.get_str_rep(),
            rot3.get_str_rep(),
            self.flip_horiz().get_str_rep(),
            self.flip_vert().get_str_rep(),
            rot1.flip_horiz().get_str_rep(),
            rot1.flip_vert().get_str_rep(),
            rot2.flip_horiz().get_str_rep(),
            rot2.flip_vert().get_str_rep(),
            rot3.flip_horiz().get_str_rep(),
            rot3.flip_vert().get_str_rep(),
        ];

        for key in keys {
            if rulebook.contains_key(&key) {
                return Image::new(rulebook.get(&key).unwrap());
            }
        }

        self.print();

        panic!("Rulebook doesn't have a rule!")
    }

    fn get_sub_image(&self, start_row: usize, start_col: usize, sidelen: usize) -> Image {
        let mut new_pixels = Vec::new();

        for row_index in start_row..(start_row + sidelen) {
            let mut this_row = Vec::new();
            for col_index in start_col..(start_col + sidelen) {
                this_row.push(self.pixels[row_index][col_index]);
            }
            new_pixels.push(this_row);
        }

        Image { pixels: new_pixels }
    }

    fn iterate(self, rulebook: &Rulebook) -> Image {
        let side_len = self.pixels.len();
        let mut new_pixels = Vec::new();
        let stride = if side_len % 2 == 0 { 2 } else { 3 };

        let mut vert_index = 0;
        while vert_index < side_len {
            let mut horiz_index = 0;
            let mut new_image_row = if stride == 3 {
                vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()]
            } else {
                vec![Vec::new(), Vec::new(), Vec::new()]
            };

            while horiz_index < side_len {
                let sub_image = self.get_sub_image(vert_index, horiz_index, stride);
                let mut new_image = sub_image.lookup(rulebook);
                if stride == 3 {
                    new_image_row[0].append(&mut new_image.pixels[0]);
                    new_image_row[1].append(&mut new_image.pixels[1]);
                    new_image_row[2].append(&mut new_image.pixels[2]);
                    new_image_row[3].append(&mut new_image.pixels[3]);
                }
                if stride == 2 {
                    new_image_row[0].append(&mut new_image.pixels[0]);
                    new_image_row[1].append(&mut new_image.pixels[1]);
                    new_image_row[2].append(&mut new_image.pixels[2]);
                }
                horiz_index += stride;
            }
            new_pixels.append(&mut new_image_row);
            vert_index += stride
        }

        Image { pixels: new_pixels }
    }

    fn get_num_on(&self) -> u64 {
        let mut total = 0;
        for row in &self.pixels {
            for col in row {
                if *col {
                    total += 1;
                }
            }
        }
        return total;
    }

    fn print(&self) {
        for row in &self.pixels {
            for bool_val in row {
                if *bool_val {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn main() {
    let rulebook = read_rulebook();
    let mut start = Image::new(".#./..#/###");

    for _ in 0..18 {
        start = start.iterate(&rulebook);
    }
    println!("{}", start.get_num_on());
}

fn read_rulebook() -> Rulebook {
    let lines = util::read_file_lines("input.txt");

    let mut book = HashMap::new();

    for line in lines {
        let words: Vec<&str> = line.split(" => ").collect();
        book.insert(words[0].to_string(), words[1].to_string());
    }

    return book;
}
