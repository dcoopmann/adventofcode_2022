use crate::Problem;

pub struct DaySix {}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        input.to_string()
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
