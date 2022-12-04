use std::{ops::{RangeInclusive}, cmp::{max, min}};

fn to_range(st: &str) -> RangeInclusive<i32> {
    let parts = st.split_once("-").unwrap();
    RangeInclusive::new(
        parts.0.parse().expect("Failed to parse 0"),
        parts.1.parse().expect("Failed to parse 1")
    )
}

fn to_pairs(input: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    input.lines().map(|ln| {
        let parts = ln.split_once(",").unwrap();
        let r1 = to_range(parts.0);
        let r2 = to_range(parts.1);

        if r1.start() <= r2.start() {
            (r1, r2)
        } else {
            (r2, r1)
        }
    }).collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let pairs = to_pairs(input);
    let mut ans = 0;

    for pair in pairs {
        let (r1, r2) = pair;

        if (r1.contains(&r2.start()) && r1.contains(&r2.end()))
        || (r2.contains(&r1.start()) && r2.contains(&r1.end())) {
            ans += 1;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<i32> {
    let pairs = to_pairs(input);
    let mut ans = 0;

    for pair in pairs {
        let (r1, r2) = pair;

        if max(r1.start(), r2.start()) <= min(r1.end(), r2.end()) {
            ans += 1;
        }
    }

    Some(ans)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
