use std::collections::VecDeque;

use crate::Problem;

pub struct DayFive {}

fn find_stack_numbers(number_line: &&str) -> Vec<char> {
    number_line
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<_>>()
}

fn find_instruction_numbers<'a>(number_line: &&'a str) -> Vec<&'a str> {
    number_line
        .split(' ')
        .filter(|c| c.chars().count() <= 2 && c != &"to")
        .collect::<Vec<_>>()
}

fn find_number_positions(number_line: &&str, numbers: Vec<char>) -> Vec<usize> {
    let mut positions = Vec::new();
    for number in numbers {
        positions.push(number_line.find(number).unwrap())
    }

    positions
}

fn build_stack<'a>(plan: &'a [&'a str]) -> Vec<VecDeque<&'a str>> {
    let number_line = plan.last().unwrap();
    let numbers = find_stack_numbers(number_line);
    // println!("{:?}", numbers);

    let positions = find_number_positions(number_line, numbers);
    // println!("{:?}", positions);
    let mut base = Vec::new();

    for position in positions {
        let mut t = VecDeque::new();

        for line in plan {
            if line != number_line {
                let t2 = &line[position..position + 1];
                if !t2.trim().is_empty() {
                    t.push_back(t2)
                }
            }
        }

        base.push(t)
    }

    // println!("{:?}", base);
    base
}

#[derive(Debug)]
struct Instruction {
    count: u32,
    from: u32,
    to: u32,
}

impl Instruction {
    fn new(count: u32, from: u32, to: u32) -> Instruction {
        Instruction {
            count,
            from: from - 1,
            to: to - 1,
        }
    }
}

impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let i = input.split('\n').collect::<Vec<_>>();
        let s = i.split(|line| line.is_empty()).collect::<Vec<_>>();
        let plan = s[0];
        let instructions = s[1];

        // println!("Plan:\n{:?}", plan);
        let mut plan = build_stack(plan);
        // println!("Build Plan:\n{:?}", plan);

        // println!("Instructions:\n{:?}", instructions);
        let mut instruction_que = VecDeque::new();
        for i in instructions {
            let t = find_instruction_numbers(i);
            println!("{:?}", t);
            let inst = Instruction::new(
                t[0].to_string().parse::<u32>().unwrap(),
                t[1].to_string().parse::<u32>().unwrap(),
                t[2].to_string().parse::<u32>().unwrap(),
            );
            println!("Instruction: {:?}", inst);
            instruction_que.push_back(inst);
        }

        println!("{:?}", instruction_que);
        // let mut cn = 0;
        for i in instruction_que {
            // cn += 1;
            // println!("Run: {}", cn);
            for _ in 0..i.count {
                let to = i.to as usize;
                let from = i.from as usize;
                // println!("From: {:?}",plan[from]);
                // println!("To: {:?}",plan[to]);
                let t = plan[from].pop_front().unwrap();
                plan[to].push_front(t)
            }
        }

        println!("Restructured Plan:\n{:?}", plan);

        let mut result = "".to_string();
        for mut p in plan {
            result += p.pop_front().unwrap();
        }
        result
    }

    fn part_two(&self, input: &str) -> String {
        input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let day = DayFive {};
        assert_eq!(day.part_one(input), "CMZ")
    }
    #[test]
    fn test_part_two() {}
}
