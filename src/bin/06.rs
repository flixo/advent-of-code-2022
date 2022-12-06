
fn unique_sequence(data: &[u8], sequence_length: usize) -> Option<u32> {
    for i in 0..data.len() {
        if i < sequence_length { continue; }

        let mut seen: Vec<&u8> = vec![];

        for x in &data[i-sequence_length..i] {
            if seen.contains(&x) { continue }
            seen.push(&x);
        }
        
        if seen.len() == sequence_length {
            return Some(i as u32);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    unique_sequence(input.as_bytes(), 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    unique_sequence(input.as_bytes(), 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
