use crate::day16::code::ParseError::NoMatches;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::path::Path;
use std::str::FromStr;

lazy_static! {
    static ref RE_RULE: Regex =
        Regex::new(r"^(?P<name>.*): (?P<r1b>\d+)-(?P<r1e>\d+) or (?P<r2b>\d+)-(?P<r2e>\d+)$")
            .expect("eh");
    static ref RE_ASSIGN: Regex =
        Regex::new(r"^mem\[(?P<addr>\d+)] = (?P<value>\d+)$").expect("eh");
}

pub fn puzzles() {
    let path = Path::new("src/day16/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let notes = read_notes(file);

    let mut error_rate = 0;

    let mut invalid_tickets = HashSet::new();

    let mut map: HashMap<usize, Vec<Rule>> = HashMap::new();

    for (tpos, ticket) in notes.other_tickets.iter().enumerate() {
        for value in ticket.iter() {
            if notes
                .rules
                .iter()
                .flat_map(|r| r.ranges.as_slice())
                .all(|r| !r.contains(value))
            {
                error_rate = error_rate + value;
                invalid_tickets.insert(tpos);
            }
        }
    }

    println!("Error rate: {}", error_rate);

    for (_tpos, ticket) in notes
        .other_tickets
        .iter()
        .enumerate()
        .filter(|v| !invalid_tickets.contains(&v.0))
    {
        for (pos, value) in ticket.iter().enumerate() {
            let rules = match map.get(&pos) {
                Some(r) => r
                    .iter()
                    .filter(|r| rule_contains_value(r, value))
                    .map(|r| r.clone())
                    .collect(),
                None => notes
                    .rules
                    .iter()
                    .filter(|r| rule_contains_value(r, value))
                    .map(|r| r.clone())
                    .collect(),
            };
            map.insert(pos, rules);
        }
    }

    let mut final_map = HashMap::new();

    while final_map.len() < map.len() {
        let (k, v) = map
            .iter()
            .map(|(k, v)| {
                (
                    k,
                    is_there_only_one_unprocessed_rule(v, final_map.values().collect()),
                )
            })
            .find(|(k, v)| v.is_some())
            .expect("eh");
        let rule = v.unwrap();
        final_map.insert(k.clone(), rule.clone());
    }

    let departures = final_map
        .iter()
        .filter(|(pos, r)| r.name.contains("departure"))
        .map(|(pos, r)| notes.your_ticket[pos.clone()])
        .fold(1, |acc, x| acc * x);

    println!("Departures multiplied: {:?}", departures);
}

fn is_there_only_one_unprocessed_rule(rules: &Vec<Rule>, processed: Vec<&Rule>) -> Option<Rule> {
    let vec: Vec<&Rule> = rules.iter().filter(|r| !processed.contains(r)).collect();
    if vec.len() == 1 {
        return Some(vec[0].clone());
    }
    None
}

fn rule_contains_value(rule: &Rule, value: &usize) -> bool {
    rule.ranges.iter().any(|range| range.contains(value))
}

fn read_notes(file: File) -> Notes {
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .collect();
    let separators: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(pos, v)| v.is_empty())
        .map(|(pos, v)| pos)
        .collect();

    let rules = read_rules(&lines[(0..separators[0])]);

    let your_ticket = read_ticket(&lines[separators[0] + 2]);

    let other_tickets: Vec<Vec<usize>> = lines[(separators[1] + 2..lines.len())]
        .iter()
        .map(|l| read_ticket(l))
        .collect();

    Notes {
        rules,
        your_ticket,
        other_tickets,
    }
}

fn read_ticket(string: &String) -> Vec<usize> {
    string
        .split(",")
        .map(|s| s.parse::<usize>().expect("could not read ticket"))
        .collect()
}

fn read_rules(lines: &[String]) -> Vec<Rule> {
    lines
        .iter()
        .map(|l| l.parse::<Rule>().expect("could not parse rule"))
        .collect()
}

#[derive(Debug)]
struct Notes {
    rules: Vec<Rule>,
    your_ticket: Vec<usize>,
    other_tickets: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match RE_RULE.captures(s) {
            Some(cap) => Ok(Rule {
                name: cap["name"].to_string(),
                ranges: read_ranges_from_rule(&cap),
            }),
            None => Err(NoMatches),
        }
    }
}

fn read_ranges_from_rule(cap: &Captures) -> Vec<RangeInclusive<usize>> {
    let range_1 = (cap["r1b"].parse::<usize>().unwrap()..=cap["r1e"].parse::<usize>().unwrap());
    let range_2 = (cap["r2b"].parse::<usize>().unwrap()..=cap["r2e"].parse::<usize>().unwrap());
    [range_1, range_2].to_vec()
}

#[derive(Debug)]
enum ParseError {
    NoMatches,
}
