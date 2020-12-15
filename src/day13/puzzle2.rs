use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn puzzle_2() {
    let path = Path::new("src/day13/input.txt");

    let file = File::open(&path).expect("Could not open file");

    let buses = read_personal_notes(file);

    let mut buses_with_pos: Vec<(usize, &usize)> = buses
        .iter()
        .enumerate()
        .filter(|x| x.1 != &(0 as usize))
        .collect();

    buses_with_pos.sort_by(|x, y| y.1.cmp(x.1));

    let mut curr_incr = buses_with_pos[0].1.clone();
    let max_pos = buses_with_pos[0].0;

    let mut curr_value = 0 as usize;
    let mut found = false;

    while !found {
        curr_value = curr_value + curr_incr;
        found = true;
        for (pos, bus) in &buses_with_pos {
            if ((curr_value as isize) + ((*pos as isize) - (max_pos as isize)))
                % (*bus.clone() as isize)
                != 0
            {
                found = false;
                break;
            } else {
                curr_incr = num_integer::lcm(curr_incr, **bus);
            }
        }
    }

    println!("{}", curr_value - max_pos);
}

fn read_personal_notes(file: File) -> Vec<usize> {
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .collect();
    let buses = lines[1]
        .split(",")
        .map(|s| if s.eq("x") { Ok(0) } else { s.parse::<usize>() })
        .map(|v| v.expect("could not parse bus value"))
        .collect();
    buses
}
