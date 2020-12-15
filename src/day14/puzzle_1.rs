use crate::day14::puzzle_1::Operation::{Assign, Mask};
use crate::day14::puzzle_1::OperationError::NoMatches;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::{BitAnd, BitXor};
use std::path::Path;
use std::str::FromStr;

lazy_static! {
    static ref RE_MASK: Regex = Regex::new(r"^mask = (?P<mask>.*)$").expect("eh");
    static ref RE_ASSIGN: Regex =
        Regex::new(r"^mem\[(?P<addr>\d+)] = (?P<value>\d+)$").expect("eh");
}

pub fn puzzle_1() {
    let path = Path::new("src/day14/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let operations = read_operations(file);

    let mut memory: HashMap<usize, usize> = HashMap::new();

    let mut current_mask = Mask(HashMap::new()); // trocar

    for op in operations {
        match op {
            Mask(_) => {
                current_mask = op;
            }
            Assign(addr, value) => {
                memory.insert(addr, calculate_value(value, &current_mask));
            }
        };
    }
    println!("{:?}", memory.values().sum::<usize>());
}

fn calculate_value(v: usize, op: &Operation) -> usize {
    let mut ret = v.clone();
    match op {
        Mask(map) => {
            for (k, v) in map {
                ret = ret.bitand(((2_usize.pow(36)) - 1) - 2_usize.pow(*k as u32));
                if *v {
                    ret = ret + 2_usize.pow(*k as u32);
                }
            }
        }
        _ => (),
    };
    ret
}

fn read_operations(file: File) -> Vec<Operation> {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .map(|l| l.parse::<Operation>())
        .map(|l| l.expect("could not parse line as operation"))
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Mask(HashMap<usize, bool>),
    Assign(usize, usize),
}

impl FromStr for Operation {
    type Err = OperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match RE_MASK.captures(s) {
            Some(cap) => {
                let mut map = HashMap::new();
                cap["mask"]
                    .chars()
                    .rev()
                    .enumerate()
                    .filter(|(pos, c)| c != &'X')
                    .for_each(|(pos, c)| {
                        map.insert(pos, process_mask_bit(c));
                    });
                return Ok(Mask(map.clone()));
            }
            _ => (),
        };
        match RE_ASSIGN.captures(s) {
            Some(cap) => {
                return Ok(Assign(
                    cap["addr"]
                        .parse::<usize>()
                        .expect("could not parse addr as usize"),
                    cap["value"]
                        .parse::<usize>()
                        .expect("could not parse value as usize"),
                ));
            }
            None => Err(NoMatches),
        }
    }
}

fn process_mask_bit(c: char) -> bool {
    match c {
        '1' => true,
        '0' => false,
        _ => panic!("ué"),
    }
}

#[derive(Debug)]
enum OperationError {
    NoMatches,
}
