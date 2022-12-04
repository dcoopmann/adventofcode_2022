use adventofcode_2022::Problem;
use adventofcode_2022::days::one::DayOne;
use adventofcode_2022::days::two::DayTwo;
use adventofcode_2022::days::three::DayThree;
use std::{env, fs, process};

fn select_day(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne {})),
        2 => Some(Box::new(DayTwo {})),
        3 => Some(Box::new(DayThree {})),
        _ => None,
    }
}

fn get_puzzle_input(day: usize) -> Option<String> {
    match day {
        1 => Some(fs::read_to_string("puzzle_input/day_one.txt").unwrap()),
        2 => Some(fs::read_to_string("puzzle_input/day_two.txt").unwrap()),
        3 => Some(fs::read_to_string("puzzle_input/day_three.txt").unwrap()),
        _ => None,
    }
}

fn main() {
    let mut args = env::args();
    args.next(); // Skip Path

    let day_number = match args.next() {
        Some(arg) => arg.parse().unwrap(),
        None => {
            eprintln!("Did not get day number.");
            process::exit(1)
        }
    };

    let day = select_day(day_number).unwrap();

    let input = get_puzzle_input(day_number).unwrap();

    println!("Result Part One: {}", day.part_one(&input));
    println!("Result Part Two: {}", day.part_two(&input));
}
