use crate::day4::rules::{byr, ecl, empty_rule, eyr, hcl, hgt, iyr, pid};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Not;
use std::panic::resume_unwind;
use std::path::Path;

pub fn day_4_puzzle_1() {
    let path = Path::new("src/day4/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let passports: Vec<bool> = read_batch_file(file)
        .iter()
        .map(|s| is_passport_valid_2(s))
        .collect();

    println!("{:?}", passports.iter().filter(|b| b == &&true).count());
}

fn is_passport_valid_1(passport: &String) -> bool {
    let mut required_fields: HashSet<&str> = HashSet::new();

    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .for_each(|s| {
            required_fields.insert(s);
        });

    let fields: Vec<String> = passport
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(String::from)
        .collect();

    let regex = Regex::new(r"^(?P<key>\w{3}):.*$").unwrap();

    for field in fields {
        let cap = regex.captures(field.as_str()).unwrap();
        required_fields.remove(cap["key"].to_string().as_str());
    }

    required_fields.is_empty()
}

fn is_passport_valid_2(passport: &String) -> bool {
    let mut map = HashMap::new();

    [
        ("byr", byr as fn(String) -> bool),
        ("iyr", iyr as fn(String) -> bool),
        ("eyr", eyr as fn(String) -> bool),
        ("hgt", hgt as fn(String) -> bool),
        ("hcl", hcl as fn(String) -> bool),
        ("ecl", ecl as fn(String) -> bool),
        ("pid", pid as fn(String) -> bool),
    ]
    .iter()
    .for_each(|v| {
        map.insert(v.0.to_string(), v.1);
    });

    let fields: Vec<String> = passport
        .split(" ")
        .filter(|s| !s.trim().is_empty())
        .map(String::from)
        .collect();

    let regex = Regex::new(r"^(?P<key>\w{3}):(?P<val>.*)$").unwrap();

    for field in fields {
        let cap = regex.captures(field.as_str()).unwrap();
        let key = String::from(cap["key"].to_string());
        let validator = map.get(&key).unwrap_or(&(empty_rule as fn(String) -> bool));
        if validator(cap["val"].to_string()) {
            map.remove(key.as_str());
        }
    }

    map.is_empty()
}

fn read_batch_file(file: File) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let mut curr_passport = String::new();
    for line in io::BufReader::new(file).lines() {
        let actual_line = line.expect("could not parse line");
        let trimmed = actual_line.trim();
        if trimmed.is_empty() {
            vec.push(String::from(curr_passport.trim()));
            curr_passport.clear();
        } else {
            curr_passport = [curr_passport, trimmed.to_string()].join(" ");
        }
    }
    if curr_passport.is_empty().not() {
        vec.push(String::from(curr_passport.trim()))
    }
    vec
}
