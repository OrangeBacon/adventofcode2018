use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day5a() {
    let path = Path::new("data/day5.txt");
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

    let mut items: Vec<&str> = s.split("").filter(|x| x != &"").collect();
    let mut mutation = true;
    let mut skip = false;
    while mutation {
        mutation = false;
        let mut deletions: Vec<usize> = vec![];
        for (i, item) in items.iter().enumerate() {
            if skip {
                skip = false;
                continue;
            }
            if i + 1 == items.len() {
                break;
            }
            let next = items[i + 1];
            if item.to_lowercase() == next.to_lowercase() && *item != next {
                deletions.push(i);
                deletions.push(i + 1);
                mutation = true;
                skip = true;
            }
        }
        for idx in deletions.iter().rev() {
            items.remove(*idx);
        }
    }

    println!("{:?}", items.len());
}
