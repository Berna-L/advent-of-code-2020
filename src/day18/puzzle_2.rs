use crate::day18::puzzle_2::Expression::{
    CloseParentheses, Multiply, Number, OpenParentheses, Sum,
};
use std::borrow::BorrowMut;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn puzzle_2() {
    let path = Path::new("src/day18/input.txt");

    let file = File::open(&path).expect("Could not open file");

    println!("{}", read_homework(file));
}

fn read_homework(file: File) -> usize {
    io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("could not read line"))
        .map(|l| parse_expression(split_line(l)))
        .map(|mut e| calculate(e.borrow_mut()))
        .sum()
}

fn calculate(exp: &mut Vec<Expression>) -> usize {
    // (54 * 210 + 6) + 2 + 4 * 2
    // 1 + 6 + (4 * (5 + 6))
    let mut preceding: Vec<Expression> = Vec::new();
    while !exp.is_empty() {
        if let Some(o) = exp.pop() {
            match o {
                Number(v1) => {
                    process_number(v1, preceding.borrow_mut());
                }
                OpenParentheses => {
                    preceding.push(Number(calculate(exp.borrow_mut())));
                }
                CloseParentheses => {
                    break;
                }
                _ => {
                    preceding.push(o);
                }
            }
        }
    }
    if preceding.len() == 1 {
        if let Number(v1) = preceding[0] {
            return v1;
        }
    } else {
        if (preceding.contains(&Sum)) {
            let summed_value = calculate(preceding.borrow_mut());
            preceding.push(Number(summed_value));
        }
        return process_multiplies(preceding.borrow_mut());
    }
    panic!("chegou ao final da função");
}

fn process_number(v1: usize, preceding: &mut Vec<Expression>) {
    if preceding.is_empty() || preceding.last().map_or_else(|| true, |s| s.eq(&Multiply)) {
        preceding.push(Number(v1));
    } else if let Some(o) = preceding.pop() {
        if let Number(v2) = preceding.pop().unwrap() {
            match o {
                Sum => {
                    preceding.push(Number(v1 + v2));
                }
                _ => panic!("número colado de outro número ou de abre/fecha parênteses"),
            }
        }
    }
}

fn process_multiplies(exp: &mut Vec<Expression>) -> usize {
    let mut preceding = Vec::new();
    while !exp.is_empty() {
        if let Some(o) = exp.pop() {
            match o {
                Number(v1) => {
                    if preceding.is_empty() {
                        preceding.push(Number(v1));
                    } else if let Multiply = preceding.pop().unwrap() {
                        if let Number(v2) = preceding.pop().unwrap() {
                            preceding.push(Number(v1 * v2));
                        }
                    }
                }
                Multiply => {
                    preceding.push(o);
                }
                _ => panic!("não é para ter nada além de multiplicação e número aqui"),
            }
        }
    }
    if let Number(v) = preceding[0] {
        return v;
    }
    panic!("ué");
}

fn calculate_immediate_next(exp: &mut Vec<Expression>) -> usize {
    if let Some(e) = exp.last() {
        if let Number(v) = e {
            return v.clone();
        }
    }
    calculate(exp)
}

fn split_line(l: String) -> Vec<String> {
    l.replace("(", "( ")
        .replace(")", " )")
        .split(' ')
        .map(|u| u.to_string())
        .rev()
        .collect()
}

fn parse_expression(units: Vec<String>) -> Vec<Expression> {
    let mut vec = Vec::new();
    for unit in units.iter() {
        if let Ok(num) = unit.parse::<usize>() {
            vec.push(Number(num));
        } else {
            match unit.as_str() {
                "+" => {
                    vec.push(Sum);
                }
                "*" => {
                    vec.push(Multiply);
                }
                "(" => {
                    vec.push(OpenParentheses);
                }
                ")" => {
                    vec.push(CloseParentheses);
                }
                _ => panic!("ué"),
            }
        }
    }
    vec
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Expression {
    Number(usize),
    Sum,
    Multiply,
    OpenParentheses,
    CloseParentheses,
}
