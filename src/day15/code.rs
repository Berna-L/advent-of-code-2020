use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn puzzle() {
    let path = Path::new("src/day15/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let numbers = get_initial_numbers(file);

    let mut map = HashMap::new();
    for (pos, v) in numbers.iter().enumerate() {
        map.insert(v.clone(), pos);
    }

    let mut last_number = numbers
        .last()
        .expect("initial numbers must not be empty")
        .clone();
    let mut curr_turn = numbers.len();
    let mut last_turn: Option<usize> = None;

    while curr_turn < 30000000 {
        match last_turn {
            Some(v) => {
                last_number = curr_turn.clone() - v.clone() - 1;
            }
            None => {
                last_number = 0;
            }
        };
        last_turn = map.get(&last_number).cloned();
        map.insert(last_number.clone(), curr_turn.clone());
        curr_turn = curr_turn + 1;
    }
    println!("{}", last_number);
}

fn get_initial_numbers(file: File) -> Vec<usize> {
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .collect();
    lines[0]
        .split(",")
        .map(|s| s.parse::<usize>())
        .map(|v| v.expect("could not parse number"))
        .collect()
}
