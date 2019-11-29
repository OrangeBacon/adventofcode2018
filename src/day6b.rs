use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day6b() {
    let path = Path::new("data/day6.txt");
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

    let re = Regex::new(r"(\d+), (\d+)").unwrap();
    let mut coords = vec![];

    let (mut x, mut y, mut width, mut height) = (0,0,0,0);
    let mut bounds_defined = false;
    for cap in re.captures_iter(&s) {
        let c = (
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap()
        );
        coords.push(c);
        if !bounds_defined {
            x = c.0;
            y = c.1;
            width = c.0;
            height = c.1;
            bounds_defined = true;
        }
        if c.0 < x {
            x = c.0;
        }
        if c.1 < y {
            y = c.1;
        }
        if c.0 > width {
            width = c.0;
        }
        if c.1 > height {
            height = c.1;
        }
    }
    
    for coord in &mut coords {
        coord.0 -= x;
        coord.1 -= y;
    }
    width -= x;
    height -= y;

    let mut count = 0;
    for x in 0..=width {
        for y in 0..=height {
            let mut dist = 0;
            for coord in &coords {
                dist += (coord.0 as i32 - x as i32).abs() as u32
                    + (coord.1 as i32 - y as i32).abs() as u32;
            }
            if dist < 10000 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
