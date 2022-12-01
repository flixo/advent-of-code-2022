use advent_of_code::helpers::as_parts;


fn elf_calories (input: &str) -> Vec<u32> {
    let groups: Vec<&str> = as_parts(input);
    
    let mut calories: Vec<u32> = groups.iter().map(|group| {
        group.trim().lines().map(|cal| {
            cal.parse::<u32>().unwrap()
        }).sum()
    }).collect();

    calories.sort();

    calories
}

pub fn part_one(input: &str) -> Option<u32> {
    let calories = elf_calories(input);
    let most_calories = calories.last().expect("No index");

    Some(*most_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cals = elf_calories(input);
    let top_three_calories = &cals[cals.len()-3..].iter().sum::<u32>();
    
    Some(*top_three_calories)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
