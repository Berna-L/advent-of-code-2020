use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn day_1_read_file() -> Vec<i32> {
    let path = Path::new("src/day1/input.txt");

    let file = File::open(&path).expect("Could not open file");

    io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse::<i32>().expect("could not parse as number"))
        .collect()
}

pub fn day_1_puzzle_1() {
    let mut entries = day_1_read_file();
    entries.sort();

    for (i, min) in entries.iter().enumerate() {
        for (j, max) in entries.iter().enumerate().rev() {
            if i >= j {
                break;
            }
            if min + max == 2020 {
                println!(
                    "Resposta encontrada!\n{0} + {1} = {2}\n{0} * {1} = {3}",
                    min,
                    max,
                    min + max,
                    min * max
                );
                break;
            } else if min + max < 2020 {
                break;
            }
        }
    }
}

pub fn day_1_puzzle_2() {
    let entries = day_1_read_file();
    for (i, first) in entries.iter().enumerate() {
        for (j, second) in entries.iter().enumerate() {
            if j < i {
                continue;
            }
            for (k, third) in entries.iter().enumerate() {
                if k < j {
                    continue;
                }
                if first + second + third == 2020 {
                    println!(
                        "Resposta encontrada!\n{0} + {1} + {2} = {3}\n{0} * {1} * {2} = {4}",
                        first,
                        second,
                        third,
                        first + second + third,
                        first * second * third
                    );
                    break;
                }
            }
        }
    }
}
