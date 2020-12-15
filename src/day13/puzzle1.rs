use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn puzzle_1() {
    let path = Path::new("src/day13/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let (arrival_time, buses) = read_personal_notes(file);

    let mut bus_id = 0 as usize;
    let mut minimum_arrival = usize::max_value();

    for bus in buses {
        if arrival_time % bus == 0 {
            bus_id = bus;
            minimum_arrival = arrival_time;
            break;
        }
        let curr_arrival = ((arrival_time / bus) + 1) * bus;
        if curr_arrival < minimum_arrival {
            bus_id = bus;
            minimum_arrival = curr_arrival;
        }
    }
    println!("{}", bus_id * (minimum_arrival - arrival_time));
}

fn read_personal_notes(file: File) -> (usize, Vec<usize>) {
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .collect();
    let arrival_time = lines[0]
        .parse::<usize>()
        .expect("could not parse arrival_time");
    let buses = lines[1]
        .split(",")
        .filter(|s| s.ne(&"x"))
        .map(|s| s.parse::<usize>())
        .map(|v| v.expect("could not parse bus value"))
        .collect();
    (arrival_time, buses)
}
