use crate::day8::code::Oper::{Acc, Jmp, Nop};
use crate::day8::code::OperParseError::NoMatches;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Not;
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(?P<oper>\w{3}) (?P<number>(\+|-)\d+)$").expect("eh");
}

pub(crate) fn day_8_puzzle_1() {
    let path = Path::new("src/day8/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let boot_code = read_boot_code(file);
    let mut acc: isize = 0;
    let mut ptr: isize = 0;
    let mut processed = HashSet::new();

    while processed.contains(&ptr.clone()).not() {
        processed.insert(ptr.clone());
        match boot_code[ptr.clone() as usize] {
            Acc(v) => {
                acc = acc + v;
                ptr = ptr + 1;
            }
            Jmp(v) => ptr = ptr + v,
            Nop(_) => ptr = ptr + 1,
        };
    }

    println!("{}", acc);
}

pub(crate) fn day_8_puzzle_2() {
    let path = Path::new("src/day8/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let boot_code = read_boot_code(file);
    let value: Vec<isize> = boot_code
        .iter()
        .enumerate()
        .filter(|o| match o.1 {
            Jmp(_) => true,
            Nop(_) => true,
            _ => false,
        })
        .map(|(pos, v)| pos)
        .map(|pos| process_altered_boot_code(&boot_code, pos))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect();
    if value.len() != 1 {
        panic!("ué");
    }
    println!("{:?}", value[0]);
}

fn process_altered_boot_code(boot_code: &Vec<Oper>, ptr_to_alter: usize) -> Option<isize> {
    let mut acc: isize = 0;
    let mut ptr: isize = 0;
    let mut processed = HashSet::new();

    while true {
        if processed.contains(&ptr) {
            return None;
        }
        if (ptr as usize).eq(&boot_code.len()) {
            return Some(acc);
        }
        processed.insert(ptr.clone());
        let mut oper = boot_code[ptr.clone() as usize];
        if (ptr as usize).eq(&ptr_to_alter) {
            oper = match boot_code[ptr.clone() as usize] {
                Jmp(v) => Nop(v),
                Nop(v) => Jmp(v),
                Acc(_) => panic!("acc operations are incorruptible"),
            }
        }
        match oper {
            Acc(v) => {
                acc = acc + v;
                ptr = ptr + 1;
            }
            Jmp(v) => ptr = ptr + v,
            Nop(_) => ptr = ptr + 1,
        };
    }
    None
}

fn read_boot_code(file: File) -> Vec<Oper> {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .map(|l| l.parse::<Oper>())
        .map(|l| l.expect("could not convert into operation"))
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum Oper {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Oper {
    type Err = OperParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = RE.captures(s).unwrap();
        match &cap["oper"] {
            "acc" => Ok(Acc(cap["number"]
                .parse::<isize>()
                .expect("could not parse acc value as number"))),
            "jmp" => Ok(Jmp(cap["number"]
                .parse::<isize>()
                .expect("could not parse jmp value as number"))),
            "nop" => Ok(Nop(cap["number"]
                .parse::<isize>()
                .expect("could not parse nop value as number"))),
            _ => Err(NoMatches),
        }
    }
}

#[derive(Debug)]
enum OperParseError {
    NoMatches,
}
