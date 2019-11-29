use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn empty(map: &mut HashMap<String, Vec<String>>) -> Option<Vec<String>> {
    //println!("{:?}\n", map);
    let mut ret = vec![];
    for (key, val) in &*map {
        if val.len() == 0 {
            ret.push(key.to_string());
        }
    }
    if ret.len() == 0 {
        return None;
    }
    ret.sort();
    map.remove(&*ret[0]);
    for (key, _) in map.clone() {
        map.entry(key.to_string())
            .or_insert(vec![])
            .retain(|x| *x != ret[0]);
    }
    Some(ret)
}

pub fn day7a() {
    let path = Path::new("data/day7.txt");
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

    let re = Regex::new(r"(\w) must be finished before step (\w)").unwrap();
    let mut tasks: HashMap<String, Vec<String>> = HashMap::new();
    for cap in re.captures_iter(&s) {
        tasks
            .entry(cap[2].to_string())
            .or_insert(vec![])
            .push(cap[1].to_string());
        tasks.entry(cap[1].to_string()).or_insert(vec![]);
    }

    while let Some(ref letter) = empty(&mut tasks) {
        print!("{}", letter[0]);
    }
}
