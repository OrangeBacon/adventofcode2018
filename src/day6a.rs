use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    id: i32,
}

#[derive(Debug, Clone, Copy)]
enum Loc {
    One(Coord, i32),
    More(i32),
    None,
}

pub fn day6a() {
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
    let mut bounds: (Coord, Coord) = (
        Coord {
            x: 1000,
            y: 1000,
            id: -1,
        },
        Coord { x: 0, y: 0, id: -1 },
    );
    let mut id = -1;
    for cap in re.captures_iter(&s) {
        let c = Coord {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            id: {
                id += 1;
                id
            },
        };
        if c.x < bounds.0.x {
            bounds.0.x = c.x
        }
        if c.x > bounds.1.x {
            bounds.1.x = c.x
        }
        if c.y < bounds.0.y {
            bounds.0.y = c.y
        }
        if c.y > bounds.1.x {
            bounds.1.y = c.y
        }
        coords.push(c);
    }

    bounds.1.x -= bounds.0.x;
    bounds.1.y -= bounds.0.y;

    for coord in &mut coords {
        coord.x = coord.x - bounds.0.x;
        coord.y = coord.y - bounds.0.y;
    }

    let mut coord_areas = vec![0; coords.len()];
    for y in 0..=bounds.1.y {
        for x in 0..=bounds.1.x {
            let mut val = Loc::None;
            for c in &coords {
                let dist = (c.x - x).abs() + (c.y - y).abs();
                val = match val {
                    Loc::None => Loc::One(*c, dist),
                    Loc::One(c_coord, c_dist) => {
                        if dist < c_dist {
                            Loc::One(*c, dist)
                        } else if dist == c_dist {
                            Loc::More(dist)
                        } else {
                            Loc::One(c_coord, c_dist)
                        }
                    }
                    Loc::More(c_dist) => {
                        if dist < c_dist {
                            Loc::One(*c, dist)
                        } else if dist == c_dist {
                            Loc::More(dist)
                        } else {
                            Loc::More(c_dist)
                        }
                    }
                }
            }
            if let Loc::One(coord, _) = val {
                if coord.x <= bounds.0.x
                    || coord.x >= bounds.1.x
                    || coord.y <= bounds.0.y
                    || coord.y >= bounds.1.y
                {
                    coord_areas[coord.id as usize] = -1
                }
                if coord_areas[coord.id as usize] > -1 {
                    coord_areas[coord.id as usize] += 1
                }
            }
        }
    }

    println!("{:?}", &coord_areas);
    println!("{:?}", coord_areas.iter().max().unwrap());
}
