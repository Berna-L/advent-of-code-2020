use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn day_3_puzzle_1() {
    let path = Path::new("src/day3/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let map = &get_map(file);

    let result = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|t| process_slope(map, t.0, t.1))
        .fold(1, |a, e| a * e);
    println!("Valor final multiplicado: {}", result);
}

fn process_slope(map: &Vec<Vec<bool>>, right_steps: usize, down_steps: usize) -> usize {
    let mut cur_x: usize = 0;
    let mut cur_y: usize = 0;

    let mut tree_count = 0;
    let map_length = map.get(0).expect("empty map").len();

    while cur_y < map.len() {
        if *map
            .get(cur_y)
            .expect("line expected but does not exist")
            .get(cur_x % map_length)
            .expect("coordinate without bool value")
        {
            tree_count = tree_count + 1;
        }
        cur_x = cur_x + right_steps;
        cur_y = cur_y + down_steps;
    }
    tree_count
}

fn get_map(file: File) -> Vec<Vec<bool>> {
    io::BufReader::new(file)
        .lines()
        .map(|e| (e.expect("could not parse line")))
        .map(convert_line)
        .collect()
}

fn convert_line(p: String) -> Vec<bool> {
    p.chars().map(|c| c == '#').collect()
}
