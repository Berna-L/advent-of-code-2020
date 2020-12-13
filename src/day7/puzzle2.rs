use crate::day7::puzzle2::InnerBag::{Bag, NoBag};
use crate::day7::puzzle2::InnerBagParseError::NoMatches;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Not;
use std::path::Path;
use std::str::FromStr;

lazy_static! {
    static ref RE_OUTER: Regex = Regex::new(r"^(\w+ \w+) bag(s?)$").expect("eh");
    static ref RE_INNER: Regex =
        Regex::new(r"^(?P<qty>\d+) (?P<name>\w+ \w+) bag(s?)$").expect("eh");
    static ref NO_OTHER: String = "no other bags".to_string();
}

pub fn day_7_puzzle_2() {
    let path = Path::new("src/day7/input.txt");

    let file = File::open(&path).expect("Could not open file");

    read_bag_rules_file(file);
}

fn read_bag_rules_file(file: File) {
    let mut map = HashMap::new();
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .map(|mut l| l.replace(".", ""))
        .map(|l| split_outer_inner(l))
        .for_each(|v| {
            map.insert(
                RE_OUTER.replace_all(v[0].as_str(), r"$1").to_string(),
                get_inner_list(v[1].clone()),
            );
            return;
        });

    let total = process_bags(&map, &"shiny gold".to_string());

    println!("{:?}", total);

    // println!("{:?}", matches_set.len());
}

fn split_outer_inner(line: String) -> Vec<String> {
    line.split(" contain ").map(|s| s.to_string()).collect()
}

fn get_inner_list(inner: String) -> Vec<InnerBag> {
    inner
        .split(", ")
        .filter(|s| NO_OTHER.eq(s).not())
        .map(|s| s.parse::<InnerBag>())
        .map(|i| i.expect("could not parse innerbag"))
        .collect()
}

fn process_bags(map: &HashMap<String, Vec<InnerBag>>, desired: &String) -> usize {
    let mut count: usize = 0;
    for bag in map[desired].iter() {
        count = count
            + match bag {
                Bag(name, qty) => qty + (qty * process_bags(map, &name)),
                NoBag => 0,
            }
    }
    count
}

fn contains_desired(
    map: &HashMap<String, Vec<String>>,
    set: &mut HashSet<String>,
    current: &String,
    desired: &String,
) -> bool {
    if set.contains(current) {
        return true;
    }
    if map.contains_key(current).not() {
        return false;
    }
    for inner in map[current].iter() {
        if inner.eq(desired) {
            set.insert(current.clone());
            return true;
        }
        if contains_desired(map, set, inner, desired) {
            set.insert(current.clone());
            return true;
        }
    }
    false
}

#[derive(Debug)]
enum InnerBag {
    NoBag,
    Bag(String, usize),
}

impl FromStr for InnerBag {
    type Err = InnerBagParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if (NO_OTHER.eq(s)) {
            return Ok(NoBag);
        }
        let cap = RE_INNER.captures(s).unwrap();
        if cap.len() == 0 {
            return Err(NoMatches);
        }
        Ok(Bag(
            cap["name"].to_string(),
            cap["qty"]
                .parse::<usize>()
                .expect("could not parse qty as int"),
        ))
    }
}

#[derive(Debug)]
enum InnerBagParseError {
    NoMatches,
}
