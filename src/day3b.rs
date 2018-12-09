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
enum SectionState {
    None,
    One,
    More,
}
#[derive(Debug, Clone, PartialEq)]
struct Section {
    state: SectionState,
    id: i32,
}

pub fn day3b() {
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

    let mut fabric: Vec<Vec<Section>> = vec![
        vec![
            Section {
                state: SectionState::None,
                id: -1
            };
            1100
        ];
        1100
    ];
    let mut overlaps: Vec<bool> = vec![false; claims.len()];
    for claim in &claims {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                if fabric[x as usize][y as usize].state == SectionState::None {
                    fabric[x as usize][y as usize] = Section {
                        state: SectionState::One,
                        id: claim.id,
                    };
                } else if fabric[x as usize][y as usize].state == SectionState::One {
                    overlaps[fabric[x as usize][y as usize].id as usize - 1 as usize] = true;
                    overlaps[claim.id as usize - 1 as usize] = true;
                    fabric[x as usize][y as usize] = Section {
                        state: SectionState::More,
                        id: -1,
                    };
                } else {
                    overlaps[claim.id as usize - 1 as usize] = true;
                }
            }
        }
    }

    let result = overlaps.iter().position(|s| !s).unwrap() + 1;
    println!("{}", result);
}
