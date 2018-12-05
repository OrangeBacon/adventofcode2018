use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("day2.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => println!("Read file"),
    }

    'outer: for current in s.split_whitespace() {
        for check in s.split_whitespace() {
            if current == check {
                break;
            }
            let mut differs = 0i32;
            let current_chars: Vec<char> = current.chars().collect();
            let check_chars: Vec<char> = check.chars().collect();
            for (i, ch) in current_chars.iter().enumerate() {
                if ch != &check_chars[i] {
                    differs += 1;
                }
            }
            if differs == 1 {
                for (i, ch) in current_chars.iter().enumerate() {
                    if ch == &check_chars[i] {
                        print!("{}", ch);
                    }
                }
                break 'outer;
            }
        }
    }
}
