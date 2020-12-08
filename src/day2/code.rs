use crate::day2::code::DatabaseEntryParseErr::NoMatches;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

pub fn day_2_puzzle_1() {
    let path = Path::new("src/day2/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let entries = get_entries(file);
    let mut correct_passwords = 0;
    for e in entries {
        let count = e.password.chars().filter(|c| c.eq(&e.letter)).count();
        if count <= e.second_param && count >= e.first_param {
            correct_passwords = correct_passwords + 1;
        }
    }
    println!("Correct passwords: {}", correct_passwords);
}

fn get_entries(file: File) -> Vec<DatabaseEntry> {
    io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            l.parse::<DatabaseEntry>()
                .expect("could not parse as DatabaseEntry")
        })
        .collect()
}

pub fn day_2_puzzle_2() {
    let path = Path::new("src/day2/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let entries: Vec<DatabaseEntry> = get_entries(file);
    let mut correct_passwords = 0;
    for e in entries {
        let (first, second) = get_chars_from_password(&e);
        if (first == e.letter && second != e.letter) || (first != e.letter && second == e.letter) {
            correct_passwords = correct_passwords + 1;
        }
    }
    println!("Correct passwords: {}", correct_passwords);
}

fn get_chars_from_password(e: &DatabaseEntry) -> (char, char) {
    let first = e
        .password
        .chars()
        .nth(e.first_param - 1)
        .expect("index oob");
    let second = e
        .password
        .chars()
        .nth(e.second_param - 1)
        .expect("index oob");
    (first, second)
}

#[derive(Debug)]
struct DatabaseEntry {
    first_param: usize,
    second_param: usize,
    letter: char,
    password: String,
}

impl FromStr for DatabaseEntry {
    type Err = DatabaseEntryParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex =
            Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)$").unwrap();
        let cap = regex.captures(s).unwrap();
        if cap.len() == 0 {
            return Err(NoMatches);
        }
        Ok(DatabaseEntry {
            first_param: cap["min"]
                .parse::<usize>()
                .expect("could not parse min as int"),
            second_param: cap["max"]
                .parse::<usize>()
                .expect("could not parse max as int"),
            letter: cap["letter"]
                .parse::<char>()
                .expect("could not parse letter as char"),
            password: cap["password"].to_string(),
        })
    }
}

#[derive(Debug)]
enum DatabaseEntryParseErr {
    NoMatches,
}
