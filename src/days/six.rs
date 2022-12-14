use crate::Problem;
use std::collections::HashSet;

pub struct DaySix {}

fn find_message(stream: Vec<char>, marker_length: usize) -> u32 {
    let mut position = 0;

    for i in 0..stream.len() {
        let mut hs: HashSet<char> = HashSet::new();
        hs.insert(stream[i]);
        for ii in 0..marker_length {
            hs.insert(stream[i + ii]);
        }

        println!("{:?}", hs);
        println!("{:?}", hs.len());
        if hs.len() == marker_length {
            position = i + marker_length;
            break;
        }
    }
    position as u32
}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let input = input.chars().collect::<Vec<_>>();

        let position = find_message(input, 4);
        position.to_string()
    }
    fn part_two(&self, input: &str) -> String {
        let input = input.chars().collect::<Vec<_>>();

        let position = find_message(input, 14);
        position.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let exp_result = vec!["5", "6", "10", "11"];
        let day = DaySix {};

        for i in 0..input.len() {
            assert_eq!(day.part_one(input[i]), exp_result[i])
        }
    }
    #[test]
    fn test_part_two() {
        let input = vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let exp_result = vec!["23", "23", "29", "26"];
        let day = DaySix {};

        for i in 0..input.len() {
            assert_eq!(day.part_two(input[i]), exp_result[i])
        }
    }
}
