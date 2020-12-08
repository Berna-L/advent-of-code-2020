use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Not;
use std::path::Path;

pub fn day_6_puzzle_1() {
    let path = Path::new("src/day6/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let total_count = read_answers_file_everyone(file);

    println!("Total de respostas: {}", total_count);
}

fn read_answers_file_anyone(file: File) -> usize {
    let mut total_count = 0;
    let mut curr_group = HashSet::new();
    for line in io::BufReader::new(file).lines() {
        let actual_line = line.expect("could not parse line");
        let trimmed = actual_line.trim();
        if trimmed.is_empty() {
            total_count = total_count + curr_group.len();
            curr_group.clear();
        } else {
            trimmed.chars().for_each(|c| {
                curr_group.insert(c);
            });
        }
    }
    if curr_group.is_empty().not() {
        total_count = total_count + curr_group.len()
    }
    total_count
}

fn read_answers_file_everyone(file: File) -> usize {
    let mut total_count = 0;
    let mut curr_group_count: usize = 0;
    let mut curr_group_answers: HashMap<char, usize> = HashMap::new();
    for line in io::BufReader::new(file).lines() {
        let actual_line = line.expect("could not parse line");
        let trimmed = actual_line.trim();
        if trimmed.is_empty() {
            total_count = total_count
                + curr_group_answers
                    .iter()
                    .filter(|&e| e.1 == &curr_group_count)
                    .count();
            curr_group_count = 0;
            curr_group_answers.clear();
        } else {
            curr_group_count = curr_group_count + 1;
            trimmed.chars().for_each(|c| {
                match curr_group_answers.get(&c) {
                    Some(v) => {
                        curr_group_answers.insert(c, v + 1);
                    }
                    None => {
                        curr_group_answers.insert(c, 1);
                    }
                };
            });
        }
    }
    if curr_group_answers.is_empty().not() {
        total_count = total_count
            + curr_group_answers
                .iter()
                .filter(|&e| e.1 == &curr_group_count)
                .count();
    }
    total_count
}
