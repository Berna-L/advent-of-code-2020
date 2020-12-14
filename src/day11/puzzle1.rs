use crate::day11::puzzle1::Seat::{Empty, Floor, Occupied};
use crate::day11::puzzle1::SeatError::NoMatches;
use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::BufRead;
use std::ops::{Range, RangeInclusive};
use std::path::Path;
use std::str::FromStr;
use std::{fmt, io};

pub fn puzzle_1() {
    let mut last = read_waiting_room_layout();
    let mut current = last;
    while true {
        last = current;
        print_current_state(&last);
        current = Vec::new();
        for (i, _row) in last.iter().enumerate() {
            current.insert(i, new_row(i, &last));
        }
        if (layouts_are_equal(&last, &current)) {
            break;
        }
    }
    println!(
        "Estabilizou com {} assentos ocupados",
        current
            .iter()
            .flatten()
            .filter(|&s| s.eq(&Occupied))
            .count()
    );
}

fn print_current_state(current: &Vec<Vec<Seat>>) {
    println!("\n\nNEW ROUND!");
    for row in current {
        println!(
            "{:?}",
            row.iter().map(|s| s.to_string()).collect::<String>()
        );
    }
}

fn layouts_are_equal(v1: &Vec<Vec<Seat>>, v2: &Vec<Vec<Seat>>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for (i, row) in v1.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            if !v2[i][j].eq(seat) {
                return false;
            }
        }
    }
    true
}

fn new_row(row_index: usize, last: &Vec<Vec<Seat>>) -> Vec<Seat> {
    let mut row = Vec::new();
    for (j, _) in last[row_index].iter().enumerate() {
        row.insert(j, new_status(row_index, j, last));
    }
    row
}

fn new_status(i: usize, j: usize, last: &Vec<Vec<Seat>>) -> Seat {
    match last[i][j] {
        Floor => Floor,
        Empty => {
            if get_occupied_in_area(i, j, last) == 0 {
                Occupied
            } else {
                Empty
            }
        }
        Occupied => {
            if get_occupied_in_area(i, j, last) > 4 {
                Empty
            } else {
                Occupied
            }
        }
    }
}

fn get_occupied_in_area(i: usize, j: usize, last: &Vec<Vec<Seat>>) -> usize {
    let mut seats: Vec<Seat> = Vec::new();
    for vec in last[get_range(i, last.len() - 1)].iter() {
        vec[get_range(j, last[i].len() - 1)].iter().for_each(|s| {
            seats.push(s.clone());
        })
    }
    seats.iter().filter(|&s| s.eq(&Occupied)).count()
}

fn get_range(i: usize, max: usize) -> RangeInclusive<usize> {
    ((std::cmp::max(1, i) - 1)..=std::cmp::min(i + 1, max))
}

fn read_waiting_room_layout() -> Vec<Vec<Seat>> {
    let path = Path::new("src/day11/input.txt");

    let file = File::open(&path).expect("Could not open file");

    io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| read_waiting_room_layout_row(&l))
        .collect()
}

fn read_waiting_room_layout_row(s: &str) -> Vec<Seat> {
    s.chars()
        .map(|c| c.to_string().parse::<Seat>())
        .filter_map(Result::ok)
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Occupied,
    Empty,
    Floor,
}

impl ToString for Seat {
    fn to_string(&self) -> String {
        match self {
            Occupied => "#".to_string(),
            Empty => "L".to_string(),
            Floor => ".".to_string(),
            _ => panic!("not expected"),
        }
    }
}

impl FromStr for Seat {
    type Err = SeatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Occupied),
            "L" => Ok(Empty),
            "." => Ok(Floor),
            _ => Err(NoMatches),
        }
    }
}

enum SeatError {
    NoMatches,
}
