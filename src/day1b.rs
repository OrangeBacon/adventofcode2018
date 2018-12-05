use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day1b() {
    let path = Path::new("data/day1.txt");
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
    let lines: Vec<i32> = s
        .split_whitespace()
        .map(|s| {
            if s.starts_with("-") {
                -s[1..].parse::<i32>().unwrap()
            } else {
                s[1..].parse::<i32>().unwrap()
            }
        }).collect();

    let mut total = 0;
    let mut freq: Vec<i32> = vec![];
    let mut i: usize = 0;
    loop {
        total += lines[i % lines.len()];
        if freq.contains(&total) {
            break;
        }
        freq.push(total);
        i += 1;
    }
    //println!("{:?}", freq);
    println!("{}", total);
}
