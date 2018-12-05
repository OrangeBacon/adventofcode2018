use std::env;

enum Part {
    First,
    Second,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify the code to run (e.g. 2b)");
        return;
    }
    if args.len() > 2 {
        println!("Only 1 argument allowed");
        return;
    }
    let arg = &args[1];
    let day: u32;
    let part: Part;
    if arg.len() == 2 {
        let mut iter = arg.chars();
        let one = iter.next().unwrap().to_digit(10);
        match one {
            Some(i) => day = i,
            None => {
                println!("Day specifier not found");
                return;
            }
        }
        match iter.next() {
            Some('a') => part = Part::First,
            Some('b') => part = Part::Second,
            _ => {
                println!("Part specifier not found");
                return;
            }
        }
        run(day, part);
    } else if arg.len() == 3 {
        let mut iter = arg.chars();
        let num_str = iter.next().unwrap().to_string() + &iter.next().unwrap().to_string();
        match num_str.parse::<i32>() {
            Ok(i) => day = i as u32,
            Err(_) => {
                println!("Day specifier not found");
                return;
            }
        }
        match iter.next() {
            Some('a') => part = Part::First,
            Some('b') => part = Part::Second,
            _ => {
                println!("Part specifier not found");
                return;
            }
        }
        run(day, part);
    } else {
        println!("Invalid code specifier");
    }
}
struct ID(u32, Part);

mod day1a;
mod day1b;
mod day2a;
mod day2b;
use day1a::day1a;
use day1b::day1b;
use day2a::day2a;
use day2b::day2b;

fn run(day: u32, part: Part) {
    if day > 2 {
        println!("Day too high");
    }
    let id = ID(day, part);
    match id {
        ID(1, Part::First) => day1a(),
        ID(1, Part::Second) => day1b(),
        ID(2, Part::First) => day2a(),
        ID(2, Part::Second) => day2b(),
        ID(_, _) => println!("Could not find day/part"),
    }
}
