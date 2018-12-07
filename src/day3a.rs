use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum Section {
    None,
    One,
    More,
}

pub fn day3a() {
    let path = Path::new("data/day3.txt");
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

    let regex = Regex::new(r"(:?#(\d+) @ (\d+),(\d+): (\d+)x(\d+))+").unwrap();
    let mut claims: Vec<Claim> = vec![];
    for cap in regex.captures_iter(&s) {
        claims.push(Claim {
            id: cap[2].parse().unwrap(),
            x: cap[3].parse().unwrap(),
            y: cap[4].parse().unwrap(),
            width: cap[5].parse().unwrap(),
            height: cap[6].parse().unwrap(),
        })
    }

    let mut fabric: Vec<Vec<Section>> = vec![vec![Section::None; 1100]; 1100];
    let mut count = 0i32;
    for claim in &claims {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                if fabric[x as usize][y as usize] == Section::None {
                    fabric[x as usize][y as usize] = Section::One;
                } else if fabric[x as usize][y as usize] == Section::One {
                    fabric[x as usize][y as usize] = Section::More;
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
