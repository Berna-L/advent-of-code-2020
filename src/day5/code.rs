use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use std::path::Path;

pub fn day_5_puzzle_1() {
    let path = Path::new("src/day5/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let mut seat_ids = read_seat_ids(file);
    println!("{:?}", seat_ids.iter().max());

    seat_ids.sort();

    for (pos, curr_val) in seat_ids.iter().enumerate() {
        if pos + 1 < seat_ids.len() {
            let next_val = seat_ids.get(pos + 1).unwrap();
            if (next_val - curr_val) == 2 {
                println!("O seu assento tem ID {}", next_val - 1);
            }
        }
    }
}

fn read_seat_ids(file: File) -> Vec<i32> {
    io::BufReader::new(file)
        .lines()
        .map(|e| e.expect("Could not parse line"))
        .map(get_seat_id)
        .collect()
}

fn get_seat_id(s: String) -> i32 {
    let value: Vec<i32> = s
        .chars()
        .map(|c| match c {
            'B' => 1,
            'R' => 1,
            _ => 0,
        })
        .collect();
    let split = value.split_at(7);
    let row = convert_binary_to_number(split.0);
    let column = convert_binary_to_number(split.1);
    (row * 8) + column
}

fn convert_binary_to_number(i: &[i32]) -> i32 {
    i.iter()
        .rev()
        .enumerate()
        .filter(|(pos, &v)| v.eq(&1))
        .fold(0, |acc, (pos, v)| acc + (v * (2_i32.pow(pos as u32))))
}
