use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
enum SleepState {
    Start,
    End,
}

#[derive(Debug)]
enum Action {
    Begin(i32),
    Sleep(SleepState),
}

#[derive(Debug)]
struct Item {
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    action: Action,
}

pub fn day4b() {
    let path = Path::new("data/day4.txt");
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

    let main_re = Regex::new(r"\[1518-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] ([^\n\r]+)").unwrap();
    let guard_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let mut dates = vec![];
    for cap in main_re.captures_iter(&s) {
        let action = match &cap[5] {
            "falls asleep" => Action::Sleep(SleepState::Start),
            "wakes up" => Action::Sleep(SleepState::End),
            _ if guard_re.is_match(&cap[5]) => Action::Begin(
                guard_re
                    .captures(&cap[5])
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            ),
            _ => panic!("Should not get here, {}", &cap[0]),
        };
        dates.push(Item {
            month: cap[1].parse().unwrap(),
            day: cap[2].parse().unwrap(),
            hour: cap[3].parse().unwrap(),
            min: cap[4].parse().unwrap(),
            action: action,
        })
    }
    dates.sort_by(|a, b| {
        let a_cmp = a.month * 1000000 + a.day * 10000 + a.hour * 100 + a.min;
        let b_cmp = b.month * 1000000 + b.day * 10000 + b.hour * 100 + b.min;
        if a_cmp > b_cmp {
            return Ordering::Greater;
        } else if a_cmp < b_cmp {
            return Ordering::Less;
        }
        return Ordering::Equal;
    });

    let mut guards: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut current: i32 = -1;
    let mut start: i32 = -1;
    for item in dates {
        match item.action {
            Action::Begin(id) => current = id,
            Action::Sleep(SleepState::Start) => start = item.min,
            Action::Sleep(SleepState::End) => {
                for i in start..item.min {
                    guards.entry(current).or_insert(vec![0; 60])[i as usize] += 1;
                }
            }
        }
    }

    let mut times_slept = 0;
    let mut min = 0;
    let mut id = 0;
    for i in 0..60 {
        for (key, value) in &guards {
            if std::cmp::max(value[i], times_slept) == value[i] {
                times_slept = value[i];
                min = i;
                id = *key;
            }
        }
    }
    println!("{}", id * min as i32);
}
