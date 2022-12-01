use crate::Problem;

pub struct DayOne {}

impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let binding = input.to_string();
        let c_list = binding.split('\n');
        println!("{:?}", c_list);

        let mut largest = 0;
        let mut temp = 0;

        for item in c_list {
            if item.trim().is_empty() {
                println!("is empty matched, got new elv");
                if temp > largest {
                    largest = temp;

                    println!("Got new highest cals {}", largest);
                }
                temp = 0;
            } else {
                temp += item.trim().parse::<i32>().unwrap();
            }
            println!("Item: {:?}", item.trim());
            println!("temp: {}", temp);
        }

        largest.to_string()
    }
    fn part_two(&self, input: &str) -> String {
        let binding = input.to_string();
        let c_list = binding.split('\n');

        let mut temp = 0;
        let mut elves = Vec::new();

        for item in c_list {
            if item.trim().is_empty() {
                elves.push(temp);
                temp = 0;
            } else {
                temp += item.trim().parse::<i32>().unwrap();
            }
        }

        elves.sort();
        println!("{:?}", elves);
        let result = elves.pop().unwrap() + elves.pop().unwrap() + elves.pop().unwrap();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000";

        let day = DayOne {};
        assert_eq!(day.part_one(input), "24000");
    }
    #[test]
    fn test_part_two() {
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
        ";

        let day = DayOne {};
        assert_eq!(day.part_two(input), "45000");
    }
}
