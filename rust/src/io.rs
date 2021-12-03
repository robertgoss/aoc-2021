use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use super::movement as movement;

pub fn input_as_list(day: i8) -> Vec<i64> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| s.expect("Read failure").parse::<i64>().unwrap()
    ).collect()
}

pub fn input_as_commands(day: i8) -> Vec<movement::Command> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| movement::Command::from_string(&s.expect("Read failure")).unwrap()
    ).collect()
}

pub fn input_from_binary(day: i8) -> Vec<u64> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| u64::from_str_radix(&s.expect("Read failure"), 2).expect("Parse error")
    ).collect()
}