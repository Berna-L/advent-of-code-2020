use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{cmp, io};

pub fn puzzle_1() {
    let mut vec = read_joltage_ratings();
    vec.sort();
    let mut map = HashMap::new();
    map.insert(3, 1);
    for (i, v) in vec.iter().enumerate() {
        let prev = match i {
            0 => 0,
            _ => vec[i - 1],
        };
        let diff = v - prev;
        match map.get(&diff) {
            Some(v) => map.insert(diff, v + 1),
            None => map.insert(diff, 1),
        };
    }
    println!(
        "Multiplicação final {}",
        map.get(&1).unwrap_or_else(|| &0) * map[&3]
    );
}

pub fn puzzle_2() {
    let mut vec = read_joltage_ratings();
    vec.push(0);
    vec.sort();
    vec.push(vec.last().unwrap() + 3);
    let mut sum_vec = Vec::new();

    for (i, x) in vec.iter().enumerate() {
        if i == 0 {
            sum_vec.push(1);
        } else {
            sum_vec.push(eh(x, &i, &vec, &sum_vec));
        }
    }
    println!("{}", sum_vec.last().expect("eh"));
}

fn eh(curr_value: &usize, curr_pos: &usize, vector: &Vec<usize>, sum_vec: &Vec<usize>) -> usize {
    let one_down = eh_2(curr_value, curr_pos, &1, vector);
    let two_down = eh_2(curr_value, curr_pos, &2, vector);
    let three_down = eh_2(curr_value, curr_pos, &3, vector);
    let mut qty = 0 as usize;
    if one_down {
        qty = qty + sum_vec[curr_pos - 1];
    }
    if two_down {
        qty = qty + sum_vec[curr_pos - 2];
    }
    if three_down {
        qty = qty + sum_vec[curr_pos - 3];
    }
    qty
}

fn eh_2(curr_value: &usize, curr_pos: &usize, offset: &usize, vector: &Vec<usize>) -> bool {
    if curr_pos < offset {
        return false;
    }
    vector
        .get(curr_pos - offset)
        .map(|v| curr_value.clone() - v.clone() <= 3)
        .unwrap_or(false)
}

fn read_joltage_ratings() -> Vec<usize> {
    let path = Path::new("src/day10/input.txt");

    let file = File::open(&path).expect("Could not open file");

    io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse::<usize>().expect("could not parse as number"))
        .collect()
}
