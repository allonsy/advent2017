mod util;
use std::collections::HashMap;

type Rulebook = HashMap<String, String>;

struct Image {
    pixels: Vec<Vec<bool>>,
}

impl Image {
    fn new(str_rep: &String) -> Image {
        Image {
            pixels: Image::get_pixels_from_string(str_rep)
        }
    }

    fn get_pixels_from_string(str_rep: &String) -> Vec<Vec<bool>> {
        let rows: Vec<&str> = str_rep.split("/").collect();

        let mut pixels = Vec::new();
        for row in rows {
            let mut pixel_row = Vec::new();
            for ch in row.chars() {
                match ch {
                    '#' => pixel_row.push(true),
                    '.' => pixel_row.push(false),
                    _ => panic!("Unknown pixel: {}", ch) 
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
                    false => rep += "."
                }
            }
            if row_num != row.len() - 1 {
                rep += "/";
            }
            row_num += 1;
        }
        return rep;
    }

    fn break_up(self) -> Vec<Image> {
        let pixel_len = self.pixels.len();
        let step_size = if pixel_len % 2 == 0 {
            2
        } else {
            3
        };

        let mut sub_images = Vec::new();
        let mut row_index = 0;
        let mut col_index = 0;

        while row_index < pixel_len {
            while col_index < pixel_len {
                let mut this_image = Vec::new();
                for row in 0..step_size {
                    let mut this_row = Vec::new();
                    for col in 0..step_size {
                        this_row.push(self.pixels[row][col]);
                    }
                    this_image.push(this_row);
                }
                sub_images.push(this_image);
                col_index += 1;
            }
            row_index += step_size;
        }

        let mut images = Vec::new();
        for sub_image in sub_images {
            images.push(Image {
                pixels: sub_image,
            });
        }
        return images;
    }

    fn rotate(self) -> Image {
        let mut pixels = self.pixels;
        let side_length = pixels.len();

        let mut new_pixels = Vec::new();

        for col in 0..side_length {
            let mut new_row = Vec::new();
            for row in 0..side_length {
                new_row.push(pixels[row][col]);
            }
            new_pixels.push(new_row);
        }

        Image {
            pixels: new_pixels,
        }
    }

    fn flip_horiz(self) -> Image {
        let mut pixels = self.pixels;
        pixels.reverse();
        Image {
            pixels: pixels,
        }
    }

    fn flip_vert(self) -> Image {
        let mut pixels = self.pixels;

        for row in &mut pixels {
            row.reverse();
        }

        Image {
            pixels: pixels,
        }
    }

    fn get_keys(self) -> Vec<String> {
        let mut keys = Vec::new();
        let mut this_image = self;
        for _ in 0..4 {
            keys.push(this_image.get_str_rep());
            
            this_image = this_image.flip_horiz();
            keys.push(this_image.get_str_rep());
            
            this_image = this_image.flip_horiz().flip_vert();
            keys.push(this_image.get_str_rep());
            
            this_image = this_image.flip_vert();
            this_image = this_image.rotate();
        }
        return keys;
    }

    fn lookup(self, rulebook: &Rulebook) -> Image {
        let keys = self.get_keys();

        for key in keys {
            if rulebook.contains_key(&key) {
                return Image::new(rulebook.get(&key).unwrap());
            }
        }

        panic!("Rulebook doesn't have a rule!")
    }

    fn iterate(self, rulebook: &Rulebook) -> Image {
        let mut new_images = Vec::new();
        let sub_images = self.break_up();

        for image in sub_images {
            new_images.push(image.lookup(rulebook));
        }
    }
}

fn main() {
    let rulebook = read_rulebook();
    println!("rulebook: {:?}", rulebook);
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