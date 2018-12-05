use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day2a() {
    let path = Path::new("data/day2.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(_) => {}
    }

    let mut twos: i32 = 0;
    let mut threes: i32 = 0;
    for string in s.split_whitespace() {
        let chars: Vec<char> = string.chars().collect();
        let mut map = HashMap::new();
        for letter in chars {
            let count = map.entry(letter).or_insert(0);
            *count += 1;
        }
        let mut two = false;
        let mut three = false;
        for key in map.keys() {
            if map[key] == 2 {
                two = true
            }
            if map[key] == 3 {
                three = true
            }
        }
        if two {
            twos += 1
        }
        if three {
            threes += 1
        }
    }

    println!("{}", twos * threes);
}
