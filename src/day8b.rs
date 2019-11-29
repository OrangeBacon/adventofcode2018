use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day8b() {
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

    let nums: Vec<usize> = s
        .split_whitespace()
        .map(|e| e.parse::<usize>().unwrap())
        .collect();

    fn node(nums: &Vec<usize>, index: usize) -> (usize, usize) {
        let mut length = index+2;
        let mut value = 0;

        let children = nums[index];
        let meta = nums[index+1];

        let mut child_val: Vec<usize> = vec![];
        for _ in 0..children {
            let (l, v) = node(nums, length);
            length = l;
            child_val.push(v);
        }

        if children == 0 {
            for i in 0..meta {
                value += nums[length + i];
            }
        } else {
            for i in 0..meta {
                let meta_val = nums[length + i];
                if meta_val <= children && meta_val >= 1 {
                    value += child_val[meta_val-1];
                }
            }
        }
        length += meta;
        return (length, value);
    }

    let (_, val) = node(&nums, 0);
    println!("{}", val);
}