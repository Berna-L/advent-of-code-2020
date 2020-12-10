use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn day_9() {
    let numbers = read_xmas_file();
    let preamble = 25;
    let range = (preamble..numbers.len());
    for curr_pos in range {
        let curr_number = numbers[curr_pos];
        let sum_components = &numbers[((curr_pos - preamble)..curr_pos)];
        if !is_valid(curr_number, sum_components) {
            println!("Primeiro passo encontrado: {}", curr_number);
            let breaqui = complete_break(&curr_number, &numbers[(0..curr_pos)]);
            println!("Quebra com valor: {}", breaqui);
            break;
        }
    }
}

fn is_valid(expected: usize, sum_components: &[usize]) -> bool {
    for (i, x) in sum_components.iter().enumerate() {
        for (j, y) in sum_components.iter().enumerate() {
            if (i >= j) || (x == y) {
                continue;
            }
            if x + y == expected {
                return true;
            }
        }
    }
    false
}

fn complete_break(expected_sum: &usize, previous_numbers: &[usize]) -> usize {
    for (i, x) in previous_numbers.iter().enumerate() {
        let mut min = x.clone();
        let mut max = x.clone();
        let mut curr_sum = x.clone();
        for y in previous_numbers[((i + 1)..previous_numbers.len())].iter() {
            curr_sum = (curr_sum + y);
            if curr_sum > *expected_sum {
                break;
            }
            min = std::cmp::min(min, y.clone());
            max = std::cmp::max(max, y.clone());
            if curr_sum == *expected_sum {
                return min + max;
            }
        }
    }
    0
}

fn read_xmas_file() -> Vec<usize> {
    let path = Path::new("src/day9/input.txt");

    let file = File::open(&path).expect("Could not open file");

    io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse::<usize>().expect("could not parse as number"))
        .collect()
}
