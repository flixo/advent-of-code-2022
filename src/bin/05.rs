use std::{str::Split, cmp::min};

use advent_of_code::helpers::as_parts;
use regex::Regex;

fn parse_input(input: &str) -> (Vec<Vec<char>>, &str) {
    let parts: Vec<&str> = Regex::new("\n\n").unwrap().split(input).collect();
    let mut parts_iter = parts.iter();

    let drawing_input = parts_iter.next().unwrap().lines();
    let steps_input = *parts_iter.next().unwrap();

    let mut columns: Vec<Vec<char>> = vec![];

    for line in drawing_input {
        let chars = line.chars().collect::<Vec<char>>();
        for (i, char) in chars.chunks(4).enumerate() {
            let ch = char.get(1).unwrap();
            if columns.get(i).is_none() {
                columns.insert(i, vec![]);
            }
            if ch.is_alphabetic() {
                columns[i].insert(0, *ch);
            }
        }
    }

    (columns, steps_input)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut columns, steps_input) = parse_input(input);

    for line in steps_input.lines() {
        let command = line.split(" ").collect::<Vec<&str>>();
        
        let count = command[1].parse::<u32>().unwrap() - 0;
        let from  = command[3].parse::<u32>().unwrap() - 1;
        let to    = command[5].parse::<u32>().unwrap() - 1;

        for _ in 0..count {
            let temp = columns[from as usize].pop().unwrap();
            columns[to as usize].push(temp);
        }
    }
    
    let mut top: Vec<char> = vec![];
    for col in columns {
        top.push(*col.last().unwrap())
    }

    let ans = top.iter().collect::<String>();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut columns, steps_input) = parse_input(input);

    for line in steps_input.lines() {
        let command = line.split(" ").collect::<Vec<&str>>();
        
        let count = (command[1].parse::<u32>().unwrap() - 0) as usize;
        let from  = (command[3].parse::<u32>().unwrap() - 1) as usize;
        let to    = (command[5].parse::<u32>().unwrap() - 1) as usize;

        let max = columns[from as usize].len() as usize;

        let mut temp = columns[from].drain(max-count..).collect();
        columns[to].append(&mut temp);
    }
    
    let mut top: Vec<char> = vec![];
    for col in columns {
        top.push(*col.last().unwrap())
    }

    let ans = top.iter().collect::<String>();

    Some(ans)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
