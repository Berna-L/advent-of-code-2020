use std::collections::hash_map::Keys;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::{RangeBounds, RangeInclusive};
use std::path::Path;

pub fn puzzle_2() {
    let path = Path::new("src/day17/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let mut last_state = convert_initial_to_map(read_initial_state(file));

    let cycles = (0..6);

    for _ in cycles {
        let mut new_state = HashSet::new();

        let coords_to_be_checked: HashSet<Coord> = last_state
            .iter()
            .map(|c| c.get_neighbors())
            .flatten()
            .collect();

        for coord in coords_to_be_checked {
            if new_state_is_active(&last_state, coord) {
                new_state.insert(coord);
            }
        }

        last_state = new_state;
    }

    println!("{}", last_state.len());
}

fn new_state_is_active(last_state: &HashSet<Coord>, c: Coord) -> bool {
    let is_active = last_state.contains(&c);
    let count = c
        .get_neighbors()
        .iter()
        .filter(|n| n.ne(&&c) && last_state.contains(n))
        .count();
    if is_active {
        (2..=3).contains(&count)
    } else {
        count == 3
    }
}

fn read_initial_state(file: File) -> Vec<Vec<bool>> {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .map(|l| read_line(l))
        .collect()
}

fn convert_initial_to_map(initial_state: Vec<Vec<bool>>) -> HashSet<Coord> {
    let mut set = HashSet::new();
    for (x, vec) in initial_state.iter().enumerate() {
        for (y, val) in vec.iter().enumerate() {
            if *val {
                set.insert(Coord {
                    x: x as isize,
                    y: y as isize,
                    z: 0,
                    w: 0,
                });
            }
        }
    }
    set
}

fn read_line(line: String) -> Vec<bool> {
    line.chars().map(|c| c.eq(&'#')).collect()
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Coord {
    fn get_neighbors(&self) -> HashSet<Coord> {
        let mut set = HashSet::new();
        for x in (self.x - 1)..=(self.x + 1) {
            for y in (self.y - 1)..=(self.y + 1) {
                for z in (self.z - 1)..=(self.z + 1) {
                    for w in (self.w - 1)..=(self.w + 1) {
                        set.insert(Coord { x, y, z, w });
                    }
                }
            }
        }
        set
    }
}
