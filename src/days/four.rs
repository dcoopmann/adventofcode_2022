use crate::Problem;

pub struct DayFour {}

fn check_containing(area_one: &str, area_two: &str) -> bool {
    let a1 = convert_to_range(area_one);
    let a2 = convert_to_range(area_two);

    // accommodate for rust stile ranges which do not include last digit -> +1
    let area_1 = a1.0..a1.1+1; 
    let area_2 = a2.0..a2.1+1;

    if area_2.contains(&a1.0) && area_2.contains(&a1.1) {
        println!("Area one contained in two");
        return true;
    };

    if area_1.contains(&a2.0) && area_1.contains(&a2.1) {
        println!("Area two contained in one");
        return true;
    };

    false
}

fn convert_to_range(area: &str) -> (u32, u32) {
    let r = area.split('-').collect::<Vec<_>>();

    (r[0].parse::<u32>().unwrap(), r[1].parse::<u32>().unwrap())
}

impl Problem for DayFour {
    fn part_one(&self, input: &str) -> String {
        let binding = input.to_string();
        let ranges_vec = binding.split("\n").collect::<Vec<_>>();

        let mut result = 0;

        for ranges in ranges_vec {
            let areas = ranges.trim().split(",").collect::<Vec<_>>();
            println!("Area Vec: {:?}", areas);
            if check_containing(areas[0], areas[1]) {
                println!("Area contained in other");
                result += 1
            }
        }

        result.to_string()
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
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";
        let day = DayFour {};

        assert_eq!(day.part_one(input), "2")
    }
    #[test]
    fn test_part_two() {}
}
