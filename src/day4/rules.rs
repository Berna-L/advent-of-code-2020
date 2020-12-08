use regex::Regex;
use std::collections::HashSet;
use std::ops::Not;

pub fn byr(s: String) -> bool {
    let parse_result = s.parse::<usize>();
    match parse_result {
        Err(_) => false,
        Ok(v) => v <= 2002 && v >= 1920,
    }
}

pub fn iyr(s: String) -> bool {
    let parse_result = s.parse::<usize>();
    match parse_result {
        Err(_) => false,
        Ok(v) => v <= 2020 && v >= 2010,
    }
}

pub fn eyr(s: String) -> bool {
    let parse_result = s.parse::<usize>();
    match parse_result {
        Err(_) => false,
        Ok(v) => v <= 2030 && v >= 2020,
    }
}

pub fn hgt(s: String) -> bool {
    let regex = Regex::new(r"^(?P<value>\d+)(?P<unit>cm|in)$").unwrap();

    match regex.captures(s.as_str()) {
        None => false,
        Some(v) => {
            let value_result = v["value"].parse::<usize>();

            if value_result.is_err() {
                return false;
            }

            let value = value_result.unwrap();

            match &v["unit"] {
                "cm" => value >= 150 && value <= 193,
                "in" => value >= 59 && value <= 76,
                _ => false,
            }
        }
    }
}

pub fn hcl(s: String) -> bool {
    let regex = Regex::new(r"^#(?P<hex>[0-9a-fA-F]{6})$").unwrap();
    regex.captures(s.as_str()).is_some()
}

pub fn ecl(s: String) -> bool {
    let mut set = HashSet::new();
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .map(|s| s.to_string())
        .for_each(|s| {
            set.insert(s);
        });
    set.contains(&s)
}

pub fn pid(s: String) -> bool {
    let regex = Regex::new(r"^[0-9]{9}$").unwrap();
    regex.captures(s.as_str()).is_some()
}

pub fn empty_rule(s: String) -> bool {
    true
}
