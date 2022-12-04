
pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks: Vec<(Vec<u32>, Vec<u32>)> = input.lines().map(|ln| {
        let mut c1 = ln.chars().map(|ch| {
            if ch.is_uppercase() {
                (ch as u32) - 38
            } else {
                (ch as u32) - 96
            }
        }).collect::<Vec<u32>>();

        let c2 = c1.split_off(c1.len() / 2);

        (c1, c2)
    }).collect();

    let mut total_priority = 0;

    for rucksack in rucksacks {
        rucksack.0
        for item in rucksack.0 {
            if rucksack.1.contains(&item) {
                total_priority += item;
                break;
            }
        }
    }

    Some(total_priority)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<Vec<u32>> = input.lines().map(|ln| {
        ln.chars().map(|ch| {
            if ch.is_uppercase() {
                (ch as u32) - 38
            } else {
                (ch as u32) - 96
            }
        }).collect::<Vec<u32>>()
    }).collect();

    let mut total_priority = 0;

    for chunk in rucksacks.chunks(3) {
        for item in &chunk[0] {
            if chunk[1].contains(&item) && chunk[2].contains(&item) {
                total_priority += item;
                break;
            }
        }
    }
    

    Some(total_priority)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
