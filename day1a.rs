use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("day1.txt");
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
    let lines: Vec<i32> = s
        .split_whitespace()
        .map(|s| {
            println!("{}", s);
            println!("{:?}", &s[1..]);
            if s.starts_with("-") {
                -s[1..].parse::<i32>().unwrap()
            } else {
                s[1..].parse::<i32>().unwrap()
            }
        }).collect();

    let mut total = 0;
    for line in lines {
        total += line;
    }
    println!("{}", total);
}
