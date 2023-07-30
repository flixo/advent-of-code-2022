#[derive(Debug)]
enum OP {
    ADD,
    MUL
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    worry_op: OP,
    worry_operand: i64,
    worry_test: i64,
    next_true: i64,
    next_false: i64,
    inspection_count: i64
}

impl Monkey {
    fn inspect_and_trow_items(&mut self, calm_factor: i64, use_mod: bool) -> Vec<(i64, i64)> {
        let mut to_be_trown: Vec<(i64, i64)> = vec![];

        while self.items.len() > 0 {
            let mut item = self.items.remove(0);

            let operand = match self.worry_operand == -1 {
                true => item,
                false => self.worry_operand,
            };

            //Inspect
            item = match self.worry_op {
                OP::ADD => item + operand,
                OP::MUL => item * operand,
            };

            self.inspection_count += 1;

            //Get Bored
            item = if use_mod {
                item % calm_factor
            } else {
                item / calm_factor
            };

            //Test
            if item % self.worry_test == 0 {
                to_be_trown.push((item, self.next_true))
            } else {
                to_be_trown.push((item, self.next_false))
            }
        }

        to_be_trown
    }

    fn catch_item (&mut self, item: i64) {
        self.items.push(item)
    }
}

fn extract_monkies(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|f| {
        let numbers = f.replace("old\n", "-1").replace("Operation:", "-2")
            .split(|c| c == ' ' || c == ',' || c == 0xA as char)
            .filter_map(|f| {
                if let Ok(v) = f.parse::<i64>() {
                    Some(v)
                } else {
                    None
                }
            }).collect::<Vec<i64>>();
        
        let mut numbers = numbers.split(|n| *n == -2);

        let starting_items = numbers.next().unwrap().to_vec();
        let meta = numbers.next().unwrap();
        
        Monkey {
            items: starting_items,
            worry_op: match f.contains("+") {
                true => OP::ADD,
                false => OP::MUL,
            },
            worry_operand: meta[0],
            worry_test: meta[1],
            next_true: meta[2],
            next_false: meta[3],
            inspection_count: 0
        }
    }).collect()
}

pub fn get_monkey_business(input: &str, calm_factor: i64, rounds: i64) -> i64 {
    let mut monkies = extract_monkies(input);

    let mut round = 0;
    let mut turn = 0;
    
    let (calm_factor, use_mod) = if calm_factor == 1 {
        (monkies.iter().map(|m| m.worry_test).product::<i64>(), true)
    } else {
        (calm_factor, false)
    };

    loop {
        for (item, to) in monkies.get_mut(turn).unwrap().inspect_and_trow_items(calm_factor, use_mod) {
            monkies.get_mut(to as usize).unwrap().catch_item(item);
        }
        
        turn += 1;

        if turn == monkies.len() {
            turn = 0;
            round += 1;


            if round == rounds {
                monkies.sort_by(|a, b| a.inspection_count.cmp(&b.inspection_count));
                break monkies.pop().unwrap().inspection_count * monkies.pop().unwrap().inspection_count;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(get_monkey_business(input, 3, 20) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(get_monkey_business(input, 1, 10_000) as u64)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
 