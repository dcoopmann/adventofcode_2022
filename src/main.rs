use adventofcode_2022::days::five::DayFive;
use adventofcode_2022::days::four::DayFour;
use adventofcode_2022::days::one::DayOne;
use adventofcode_2022::days::six::DaySix;
use adventofcode_2022::days::three::DayThree;
use adventofcode_2022::days::two::DayTwo;
use adventofcode_2022::Problem;

use clap::Parser;

use std::fs;

fn select_day(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne {})),
        2 => Some(Box::new(DayTwo {})),
        3 => Some(Box::new(DayThree {})),
        4 => Some(Box::new(DayFour {})),
        5 => Some(Box::new(DayFive {})),
        6 => Some(Box::new(DaySix {})),
        _ => None,
    }
}

fn get_puzzle_input(day: usize) -> Option<String> {
    match day {
        1 => Some(fs::read_to_string("puzzle_input/day_one.txt").unwrap()),
        2 => Some(fs::read_to_string("puzzle_input/day_two.txt").unwrap()),
        3 => Some(fs::read_to_string("puzzle_input/day_three.txt").unwrap()),
        4 => Some(fs::read_to_string("puzzle_input/day_four.txt").unwrap()),
        5 => Some(fs::read_to_string("puzzle_input/day_five.txt").unwrap()),
        6 => Some(fs::read_to_string("puzzle_input/day_six.txt").unwrap()),
        _ => None,
    }
}

#[derive(Parser, Debug)]
#[command(author, version,  about, long_about=None)]
struct Args {
    ///Day
    #[arg(short, long)]
    day: usize,

    ///Part
    #[arg(short, long)]
    part: usize,
}

fn main() {
    let args = Args::parse();

    println!("Solving Day: {} Part {}", args.day, args.part);

    let day = select_day(args.day).unwrap();

    if args.part == 1 {
        println!(
            "Result Part One: {}",
            day.part_one(&get_puzzle_input(args.day).unwrap())
        );
    } else if args.part == 2 {
        println!(
            "Result Part Two: {}",
            day.part_two(&get_puzzle_input(args.day).unwrap())
        );
    } else {
        println!("There are only part one or two, not part {}", args.part)
    }
}
