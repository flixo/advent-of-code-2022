


struct CPU {
    tick: i32,
    x_reg: i32,
    signal: i32,
    display: Vec<char>,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            tick: 0,
            x_reg: 1,
            signal: 0,
            display: vec![],
        }
    }

    fn output (&mut self) -> String {
        let mut screen = String::new();

        for row in self.display.chunks(40) {
            screen.push_str(&format!("{}\n", String::from_iter(row)))
        }

        screen
    }

    fn draw (&mut self) {
        let sprite = self.x_reg-1..=self.x_reg+1;
        let pixel = if sprite.contains(&(self.tick % 40)) {'#'} else {'.'}; 
        
        self.display.push(pixel);
    }

    fn tick(&mut self) {
        self.draw();
        self.tick += 1;

        if (20 - self.tick) % 40 == 0 {
            self.signal += self.tick * self.x_reg;
        }
    }

    fn noop(&mut self) {
        self.tick();
    }

    fn addx(&mut self, v: i32) {
        self.tick();
        self.tick();
        self.x_reg += v;
    }
}

fn run_cpu (input: &str) -> CPU {
    let mut cpu = CPU::new();

    for line in input.lines() {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["noop"] => cpu.noop(),
            ["addx", v] => cpu.addx(v.parse().unwrap()),
            _ => panic!("Failed to parse input")
        };
    }

    cpu
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(run_cpu(input).signal as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    Some(run_cpu(input).output())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);

        let mut result = String::new();
        result.push_str("##..##..##..##..##..##..##..##..##..##..\n");
        result.push_str("###...###...###...###...###...###...###.\n");
        result.push_str("####....####....####....####....####....\n");
        result.push_str("#####.....#####.....#####.....#####.....\n");
        result.push_str("######......######......######......####\n");
        result.push_str("#######.......#######.......#######.....\n");

        assert_eq!(part_two(&input), Some(result));
    }
}
