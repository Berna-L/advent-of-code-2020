// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day4_rules;
// mod day5;

mod day1 {
    pub mod code;
}

mod day2 {
    pub mod code;
}

mod day3 {
    pub mod code;
}

mod day4 {
    pub mod code;
    pub mod rules;
}

mod day5 {
    pub mod code;
}

mod day6 {
    pub mod code;
}

fn main() {
    day1::code::day_1_puzzle_1();
    day1::code::day_1_puzzle_2();

    day2::code::day_2_puzzle_1();
    day2::code::day_2_puzzle_2();

    day3::code::day_3_puzzle_1();

    day4::code::day_4_puzzle_1();

    day5::code::day_5_puzzle_1();

    day6::code::day_6_puzzle_1();
}
