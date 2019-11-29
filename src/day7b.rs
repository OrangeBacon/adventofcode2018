use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn empty(map: &mut HashMap<String, Vec<String>>) -> Option<Vec<String>> {
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
pub fn day7b() {
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

    let mut tasks_num = HashMap::new();
    for (key, value) in &tasks {
        let mut vec = vec![];
        for item in value {
            vec.push(item.bytes().next().clone().unwrap() - 64);
        }
        tasks_num.insert(key.bytes().next().clone().unwrap() as i32 - 64, vec);
    }

    let mut order = vec![];
    while let Some(letter) = empty(&mut tasks) {
        order.push(letter[0].bytes().next().clone().unwrap() as i32 - 64)
    }
    let workers: &mut [i32] = &mut [-1; 2];
    let mut time: i32 = 0;
    println!("{:?}, {:?}", tasks_num, order);
    'out: loop {
        for worker in workers.iter_mut() {
            if *worker == time || *worker == -1 {
                if order.len() == 0 {
                    break 'out;
                }
                for pop in &order {
                    if order.iter().position(|x| x == pop) == None {
                        *worker = pop + 0 + time;
                    } else {
                        *worker = -1;
                    }
                }
            }
        }
        time += 1;
        //println!("{:?}, {}", workers, time);
    }
    println!("{:?}, {}", workers, time);
    println!("{:?}", workers.iter().max());
}
