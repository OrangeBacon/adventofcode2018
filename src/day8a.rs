use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day8a() {
    let path = Path::new("data/day8.txt");
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

    let nums: Vec<i32> = s
        .split_whitespace()
        .map(|e| e.parse::<i32>().unwrap())
        .collect();

    fn node(nums: &Vec<i32>, index: usize, total: &mut i32) -> usize {
        let mut length = index+2;

        let children = nums[index];
        let meta = nums[index+1];

        for _ in 0..children {
            length = node(nums, length, total);
        }

        for i in 0..(meta as usize) {
            *total += nums[length + i];
        }
        length += meta as usize;

        return length;
    }

    let mut sum = 0;
    node(&nums, 0, &mut sum);

    println!("{}", sum);
}