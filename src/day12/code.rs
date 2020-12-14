use crate::day12::code::Compass::{E, N, S, W};
use crate::day12::code::Direction::{East, Forward, Left, North, Right, South, West};
use crate::day12::code::DirectionError::NoMatches;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Neg;
use std::path::Path;
use std::str::FromStr;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(?P<dir>\w)(?P<number>\d+)$").expect("eh");
}

pub fn puzzle_1() {
    let path = Path::new("src/day12/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let directions = read_directions(file);

    let mut north = 0;
    let mut east = 0;
    let mut pointing_to = E;

    for dir in directions {
        let result = process_direction(dir, north, east, &pointing_to);
        north = result.0;
        east = result.1;
        pointing_to = result.2;
    }
    println!("Manhattan: {}", north.abs() + east.abs());
}

pub fn puzzle_2() {
    let path = Path::new("src/day12/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let directions = read_directions(file);

    let mut waypoint = Position { north: 1, east: 10 };
    let mut ship = Position { north: 0, east: 0 };

    for dir in directions {
        let result = process_direction_with_waypoint(dir, &waypoint, &ship);
        waypoint = result.0;
        ship = result.1;
    }
    println!("Manhattan: {}", ship.north.abs() + ship.east.abs());
}

fn process_direction_with_waypoint(
    direction: Direction,
    waypoint: &Position,
    ship: &Position,
) -> (Position, Position) {
    match direction {
        North(v) => (
            Position {
                north: waypoint.north + (v as isize),
                east: waypoint.east,
            },
            ship.clone(),
        ),
        East(v) => (
            Position {
                north: waypoint.north,
                east: waypoint.east + (v as isize),
            },
            ship.clone(),
        ),
        South(v) => (
            Position {
                north: waypoint.north - (v as isize),
                east: waypoint.east,
            },
            ship.clone(),
        ),
        West(v) => (
            Position {
                north: waypoint.north,
                east: waypoint.east - (v as isize),
            },
            ship.clone(),
        ),
        Right(v) => (turn_waypoint(v, waypoint), ship.clone()),
        Left(v) => (turn_waypoint(360 - v, waypoint), ship.clone()),
        Forward(v) => (
            waypoint.clone(),
            Position {
                north: ship.north + ((v as isize) * waypoint.north),
                east: ship.east + ((v as isize) * waypoint.east),
            },
        ),
    }
}

fn turn_waypoint(angle: usize, waypoint: &Position) -> Position {
    match angle % 360 {
        0 => waypoint.clone(),
        90 => Position {
            north: waypoint.east.neg(),
            east: waypoint.north,
        },
        180 => Position {
            north: waypoint.north.neg(),
            east: waypoint.east.neg(),
        },
        270 => Position {
            north: waypoint.east,
            east: waypoint.north.neg(),
        },
        _ => panic!("not expected"),
    }
}

fn process_direction(
    curr_direction: Direction,
    north: isize,
    east: isize,
    pointing_to: &Compass,
) -> (isize, isize, Compass) {
    match curr_direction {
        North(v) => (north + (v as isize), east, pointing_to.clone()),
        East(v) => (north, east + (v as isize), pointing_to.clone()),
        South(v) => (north - (v as isize), east, pointing_to.clone()),
        West(v) => (north, east - (v as isize), pointing_to.clone()),
        Right(v) => (north, east, turn_ship(v, &pointing_to).clone()),
        Left(v) => (north, east, turn_ship(360 - v, &pointing_to).clone()),
        Forward(v) => process_direction(
            get_forward_direction(v, &pointing_to),
            north,
            east,
            &pointing_to,
        ),
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Position {
    north: isize,
    east: isize,
}

fn get_forward_direction(units: usize, pointing_to: &Compass) -> Direction {
    match pointing_to {
        N => North(units),
        E => East(units),
        S => South(units),
        W => West(units),
    }
}

fn turn_ship(angle: usize, pointing_to: &Compass) -> Compass {
    let compass = [N, E, S, W];
    let curr_pos = compass.iter().position(|c| c.eq(pointing_to)).unwrap();
    let new_compass_pos = ((angle / 90) + curr_pos) % 4;
    compass[new_compass_pos]
}

fn read_directions(file: File) -> Vec<Direction> {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .map(|l| l.parse::<Direction>())
        .map(|l| l.expect("s"))
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Compass {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl FromStr for Direction {
    type Err = DirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = RE.captures(s).unwrap();
        let num = cap["number"]
            .parse::<usize>()
            .expect("could not parse North value as number");
        match &cap["dir"] {
            "N" => Ok(North(num)),
            "S" => Ok(South(num)),
            "E" => Ok(East(num)),
            "W" => Ok(West(num)),
            "L" => Ok(Left(num)),
            "R" => Ok(Right(num)),
            "F" => Ok(Forward(num)),
            _ => Err(NoMatches),
        }
    }
}

#[derive(Debug)]
enum DirectionError {
    NoMatches,
}
