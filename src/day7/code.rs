use regex::Regex;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Map;
use std::ops::Not;
use std::path::Path;

lazy_static! {
    static ref RE_OUTER: Regex = Regex::new(r"^(\w+ \w+) bag(s?)$").expect("eh");
    static ref RE_INNER: Regex = Regex::new(r"^\d+ (\w+ \w+) bag(s?)$").expect("eh");
    static ref NO_OTHER: String = "no other bags".to_string();
}

pub fn day_7_puzzle_1() {
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

    let mut matches_set = HashSet::new();

    process_bags(&map, &mut matches_set, "shiny gold".to_string());

    println!("{:?}", map);

    println!("{:?}", matches_set.len());
}

fn split_outer_inner(line: String) -> Vec<String> {
    line.split(" contain ").map(|s| s.to_string()).collect()
}

fn get_inner_list(inner: String) -> Vec<String> {
    // let regex = Regex::new(r"^\d+ (\w+ \w+) bag(s?)$").expect("eh");
    inner
        .split(", ")
        .filter(|s| NO_OTHER.eq(s).not())
        .map(|s| RE_INNER.replace_all(s, r"$1").to_string())
        .collect()
}

fn process_bags(map: &HashMap<String, Vec<String>>, set: &mut HashSet<String>, desired: String) {
    for key in map.keys() {
        contains_desired(map, set, key, &desired);
    }
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
