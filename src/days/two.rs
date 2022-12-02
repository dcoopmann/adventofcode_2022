use crate::Problem;

pub struct DayTwo {}

/*
    A = Rock
    B = Paper
    C = Scissors

    X = Rock
    Y = Paper
    Z = Scissors

    Points:

    Rock = 1
    Paper = 2
    Scissors = 3

    Loose = 0
    Draw = 3
    Win = 6
*/

fn play(adversary: &str, you: &str) -> u32 {
    if adversary == "A" {
        if you == "Y" {
            return 6;
        } else if you == "X" {
            return 3;
        } else {
            return 0;
        };
    } else if adversary == "B" {
        if you == "Z" {
            return 6;
        } else if you == "Y" {
            return 3;
        } else {
            return 0;
        }
    } else {
        if you == "X" {
            return 6;
        } else if you == "Z" {
            return 3;
        } else {
            return 0;
        }
    }
}

fn form_check(you: &str) -> u32 {
    if you == "X" {
        return 1;
    } else if you == "Y" {
        return 2;
    } else {
        return 3;
    };
}

fn apply_tactic<'a>(tactic: &'a str, adversary: &'a str) -> &'a str {
    if tactic == "X" {
        // return "loose";
        if adversary == "A" {
            return "Z";
        } else if adversary == "B" {
            return "X";
        } else {
            return "Y";
        }
    } else if tactic == "Y" {
        // return "draw";
        if adversary == "A" {
            return "X";
        } else if adversary == "B" {
            return "Y";
        } else {
            return "Z";
        }
    } else {
        // return "win";
        if adversary == "A" {
            return "Y";
        } else if adversary == "B" {
            return "Z";
        } else {
            return "X";
        }
    }
}

impl Problem for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let binding = input.to_string();
        let c_list = binding.split_terminator('\n');

        let mut score: u32 = 0;

        for entry in c_list {
            let mut split = entry.trim().split(' ');
            let a = split.next().unwrap();
            let y = split.next().unwrap();

            score += form_check(y);
            score += play(a, y);
        }

        score.to_string()
    }
    fn part_two(&self, input: &str) -> String {
        let binding = input.to_string();
        let c_list = binding.split_terminator('\n');

        let mut score: u32 = 0;

        for entry in c_list {
            let mut split = entry.trim().split(' ');
            let a = split.next().unwrap();
            let t = split.next().unwrap();

            let y = apply_tactic(t, a);
            println!("{}", y);

            score += form_check(y);
            score += play(a, y);
        }

        score.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "A Y\n B X\n C Z\n";
        let day = DayTwo {};

        assert_eq!(day.part_one(input), "15");
    }
    #[test]
    fn test_part_two() {
        let input = "A Y\n B X\n C Z\n";
        let day = DayTwo {};

        assert_eq!(day.part_two(input), "12");
    }
}
