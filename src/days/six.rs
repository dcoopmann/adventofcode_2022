use crate::Problem;
use std::collections::HashSet;

pub struct DaySix {}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let l = input.len();
        let mut position = 0;
        let input = input.chars().collect::<Vec<_>>();
        println!("{:?}", input);
        for i in 0..l {
            println!("Run: {}", i);
            let mut hs: HashSet<char> = HashSet::new();
            hs.insert(input[i]);
            hs.insert(input[i + 1]);
            hs.insert(input[i + 2]);
            hs.insert(input[i + 3]);

            println!("{:?}", hs);
            println!("{:?}", hs.len());
            if hs.len() == 4 {
                position = i + 4;
                break;
            }
        }

        position.to_string()
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
    fn test_part_two() {}
}
