use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fs;

use crate::polymer;

use super::movement as movement;
use super::bingo as bingo;
use super::vents as vents;
use super::fish as fish;
use super::crabs as crabs;
use super::displays as displays;
use super::heights as heights;
use super::octopus as octopus;
use super::caves as caves;
use super::folding as folding;

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

pub fn input_as_vents(day: i8) -> Vec<vents::Vent> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| vents::Vent::from_string(&s.expect("Read failure")).unwrap()
    ).collect()
}

pub fn input_as_game(day: i8) -> bingo::Game {
    bingo::Game::from_lines(input_as_lines(day)).unwrap()
}

pub fn input_as_fish(day: i8) -> Vec<fish::Fish> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    let line = reader.lines().next().unwrap().expect("Read failure");
    line.split(',').map(
        |s| fish::Fish{ remaining : s.parse::<u64>().unwrap() }
    ).collect()
}

pub fn input_as_crabs(day: i8) -> Vec<crabs::Crab> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    let line = reader.lines().next().unwrap().expect("Read failure");
    line.split(',').map(
        |s| crabs::Crab{ pos : s.parse::<i64>().unwrap() }
    ).collect()
}

pub fn input_as_displays(day: i8) -> Vec<displays::Entry> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| displays::Entry::from_string(&s.expect("Read failure")).unwrap()
    ).collect()
}

pub fn input_as_heightmap(day: i8) -> heights::HeightMap {
    heights::HeightMap::from_lines(input_as_lines(day))
}


pub fn input_as_lines(day: i8) -> Vec<String> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| s.expect("Read failure")
    ).collect()
}

pub fn input_as_octopus_states(day: i8) -> octopus::StateMap {
    octopus::StateMap::from_lines(input_as_lines(day))
}

pub fn input_as_cave_systen(day: i8) -> caves::CaveSystem {
    caves::CaveSystem::from_lines(input_as_lines(day))
}

pub fn input_as_folding(day : i8) -> folding::Instructions {
    let filename = format!("../data/day-{}.txt", day);
    let data = fs::read_to_string(filename).expect("Read failure");
    folding::Instructions::from_string(&data).unwrap()
}

pub fn input_as_polymer(day : i8) -> polymer::PolymerProgram {
    let filename = format!("../data/day-{}.txt", day);
    let data = fs::read_to_string(filename).expect("Read failure");
    polymer::PolymerProgram::from_string(&data).unwrap()
}