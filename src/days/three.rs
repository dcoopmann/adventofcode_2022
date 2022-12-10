use crate::Problem;

pub struct DayThree {}

fn find_double_item(backpack: &str) -> char {
    let middle = backpack.len() / 2;
    let (comp1, comp2) = backpack.split_at(middle);
    println!("Split at: {}", middle);
    println!("Compartment 1: {}", comp1);
    println!("Compartment 2: {}", comp2);

    let mut singularity = char::default();
    'outer: for cp1 in comp1.chars() {
        for cp2 in comp2.chars() {
            if cp1 == cp2 {
                println!("Singularity: {:?}", cp1);
                singularity = cp1;
                break 'outer;
            }
        }
    }

    singularity
}

fn assign_value(item: &char) -> u32 {
    let binding = item.clone();
    println!("Binding: {}", binding);

    if item.is_uppercase() {
        binding as u32 - 38
    } else {
        binding as u32 - 96
    }
}

fn find_doubles(needle: &str, stack: &str) -> String {
    let mut doubles = String::new();
    for n in needle.chars() {
        for s in stack.chars() {
            if n == s {
                doubles += &s.to_string();
            }
        }
    }
    doubles
}

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let binding = input.to_string();
        let backpacks = binding.split('\n').collect::<Vec<_>>();

        let mut sum = 0;

        for backpack in backpacks {
            let b = backpack.trim(); // sanitize mostly for test input
            println!("{}", b);
            let item = find_double_item(b);
            sum += assign_value(&item);
        }

        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let binding = input.to_string();
        let backpacks = binding.split('\n').collect::<Vec<_>>();

        let mut sum = 0;

        let mut counter = 0;
        while counter <= backpacks.len() - 3 {
            // let mut s = String::new();
            println!("Counter: {}", counter);

            let s1 = backpacks.get(counter).unwrap().trim();
            let s2 = backpacks.get(counter + 1).unwrap().trim();
            let s3 = backpacks.get(counter + 2).unwrap().trim();

            let c1 = find_doubles(&s1, &s2);
            println!("c1: {}", c1);
            let c2 = find_doubles(&c1, &s3);
            println!("c2: {}", c2);

            let item = c2.trim()[0..1].parse::<char>().unwrap();
            sum += assign_value(&item);

            counter += 3;
        }
        println!("Length backpacks: {}", backpacks.len());

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let day = DayThree {};
        assert_eq!(day.part_one(input), "157");
    }
    #[test]
    fn test_part_two() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let day = DayThree {};
        assert_eq!(day.part_two(input), "70");
    }
}
