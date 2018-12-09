use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn react(mut items: Vec<&str>) -> usize {
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
    items.len()
}

pub fn day5b() {
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

    let items: Vec<&str> = s.split("").filter(|x| x != &"").collect();
    let mut set = HashSet::new();
    for item in &items {
        set.insert(item.to_lowercase());
    }
    let mut lens = vec![];
    for unit in set {
        let upper = unit.to_uppercase();
        let chain = s.as_str();
        let chain_vec = chain
            .split("")
            .map(|x| match x {
                _ if x == unit => "",
                _ if x == upper => "",
                x => x,
            }).filter(|x| x != &"")
            .collect();
        lens.push(react(chain_vec));
    }
    println!("{:?}", lens.iter().min().unwrap());
}
