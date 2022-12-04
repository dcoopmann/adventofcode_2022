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

    if item.is_uppercase(){
        binding as u32 - 38
    } else {
        binding as u32 - 96 
    }
}

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let binding = input.to_string();
        let backpacks = binding.split('\n').collect::<Vec<_>>();

        let mut sum = 0;

        println!("a {}", 'a' as u32 - 96);
        println!("z {}", 'z' as u32 - 96);
        println!("A {}", 'A' as u32 - 38);
        println!("Z {}", 'Z' as u32 - 38);

        println!("uppercase? A {}", 'A'.is_uppercase());
        println!("uppercase? a {}", 'a'.is_uppercase());

        for backpack in backpacks {
            let b = backpack.trim(); // sanitize mostly for test input
            println!("{}", b);
            let item = find_double_item(b);
            sum += assign_value(&item);
        }


        sum.to_string()
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
    fn test_part_two() {}
}
