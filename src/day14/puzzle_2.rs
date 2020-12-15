use crate::day14::puzzle_2::MaskValue::{Floating, One};
use crate::day14::puzzle_2::Operation::{Assign, Mask};
use crate::day14::puzzle_2::OperationError::NoMatches;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::{BitAnd, BitOr, BitXor};
use std::path::Path;
use std::str::FromStr;

lazy_static! {
    static ref RE_MASK: Regex = Regex::new(r"^mask = (?P<mask>.*)$").expect("eh");
    static ref RE_ASSIGN: Regex =
        Regex::new(r"^mem\[(?P<addr>\d+)] = (?P<value>\d+)$").expect("eh");
}

pub fn puzzle_2() {
    let path = Path::new("src/day14/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let operations = read_operations(file);

    let mut memory: HashMap<usize, usize> = HashMap::new();

    let mut current_mask = Mask(HashMap::new());

    for op in operations {
        match op {
            Mask(_) => {
                current_mask = op;
            }
            Assign(base_addr, value) => {
                get_addresses(base_addr, &current_mask)
                    .iter()
                    .for_each(|addr| {
                        memory.insert(*addr, value);
                    });
            }
        };
    }
    println!("{:?}", memory.values().sum::<usize>());
}

fn get_addresses(base_addr: usize, op: &Operation) -> Vec<usize> {
    let mut addr = base_addr.clone();
    let map = match op {
        Mask(m) => m,
        _ => panic!("chamou get_addresses com um valor que não é mask"),
    };
    map.iter()
        .filter(|(k, v)| v.eq(&&One))
        .map(|(k, v)| k)
        .for_each(|k| addr = overwrite_bit_with_one(addr, k));
    let floating_positions: Vec<&usize> = map
        .iter()
        .filter(|(k, v)| v.eq(&&Floating))
        .map(|(k, v)| k)
        .collect();
    apply_floating(addr, floating_positions.as_slice())
}

fn apply_floating(address: usize, bit_vec: &[&usize]) -> Vec<usize> {
    let mut return_vec = Vec::new();
    if bit_vec.is_empty() {
        return_vec.push(address);
    } else {
        let bit = bit_vec[0];
        for addr in apply_floating(address, &bit_vec[(1..bit_vec.len())]) {
            return_vec.push(overwrite_bit_with_one(addr, &bit));
            return_vec.push(overwrite_bit_with_zero(addr, &bit));
        }
    }
    return_vec
}

fn overwrite_bit_with_one(addr: usize, bit: &usize) -> usize {
    addr.bitor(2_usize.pow(*bit as u32))
}

fn overwrite_bit_with_zero(addr: usize, bit: &usize) -> usize {
    addr.bitand((2_usize.pow(36) - 1).bitxor(2_usize.pow(*bit as u32)))
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
    Mask(HashMap<usize, MaskValue>),
    Assign(usize, usize),
}

#[derive(Debug, Eq, PartialEq)]
enum MaskValue {
    One,      // Sobrescrever bit com valor 1
    Floating, // Escrever usando o bit como 0 e como 1
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
                    .filter(|(pos, c)| c != &'0')
                    .for_each(|(pos, c)| {
                        map.insert(pos, process_mask_bit(c));
                    });
                return Ok(Mask(map));
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

fn process_mask_bit(c: char) -> MaskValue {
    match c {
        '1' => One,
        'X' => Floating,
        _ => panic!("ué"),
    }
}

#[derive(Debug)]
enum OperationError {
    NoMatches,
}
