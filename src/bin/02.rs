//Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
//1 for Rock, 2 for Paper, and 3 for Scissors
//A for Rock, B for Paper, and C for Scissors
//X for Rock, Y for Paper, and Z for Scissors


pub fn part_one(input: &str) -> Option<u32> {
    let rounds: Vec<(i32, i32)> = input.lines().map(|r| {
        let c: Vec<i32> = r.chars().map(|c| c as i32).collect();
        let player  = c[2]-87;
        let oponent = c[0]-64;

        (oponent, player)
    }).collect();

    let mut score = 0;

    for round in rounds {
        let mut delta = round.1 - round.0;

        score += round.1;

        //Handle rock/sissor condition
        if delta.abs() == 2 {
            delta *= -1
        }

        if delta == 0 { //Draw
            score += 3;
        } else if delta > 0 { //Win
            score += 6;
        }
    }

    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    
    let mut score = 0;
    let rounds: Vec<(i32, i32)> = input.lines().map(|r| {
        let c: Vec<i32> = r.chars().map(|c| c as i32).collect();
        
        let strategy = c[2]-89; //-1=win, 0=draw, 1=lose
        let oponent = c[0]-64;

        let mut player = oponent + strategy;
        
        //Handle rock/sissor condition
        if player > 3 { player = 1 };
        if player < 1 { player = 3 };

        (oponent, player)
    }).collect();

    for round in rounds {
        let mut delta = round.1 - round.0;

        score += round.1;

        //Handle rock/sissor condition
        if delta.abs() == 2 {
            delta *= -1
        }

        if delta == 0 { //Draw
            score += 3;
        } else if delta > 0 { //Win
            score += 6;
        }
    }

    Some(score as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
