#[macro_use]
extern crate lazy_static;
extern crate regex;

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

mod day7 {
    pub mod puzzle1;
    pub mod puzzle2;
}

mod day8 {
    pub mod code;
}

mod day9 {
    pub mod code;
}

fn main() {
    // day1::code::day_1_puzzle_1();
    // day1::code::day_1_puzzle_2();
    //
    // day2::code::day_2_puzzle_1();
    // day2::code::day_2_puzzle_2();
    //
    // day3::code::day_3_puzzle_1();
    //
    // day4::code::day_4_puzzle_1();
    //
    // day5::code::day_5_puzzle_1();
    //
    // day6::code::day_6_puzzle_1();

    // day7::code::day_7_puzzle_1();

    // day7::puzzle1::day_7_puzzle_1();
    // day7::puzzle2::day_7_puzzle_2();

    // day8::code::day_8_puzzle_2();

    day9::code::day_9();
}
