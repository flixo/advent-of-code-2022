use std::cmp::max;

#[derive(Debug, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
    fn from (src: &Vec2) -> Vec2 {
        Vec2 { x: src.x, y: src.y }
    } 
    fn cloned (&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }
    fn normalized (&self) -> Vec2 {
        Vec2 {
            x: self.x.clamp(-1, 1),
            y: self.y.clamp(-1, 1)
        }
    }
    fn add (&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
    fn sub (&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
    fn max_abs (&self) -> i32 {
        max(self.x.abs(), self.y.abs())
    }
}

struct Rope {
    segments: Vec<Vec2>,
    visited: Vec<Vec2>
}

impl Rope {
    fn new(parts: usize) -> Rope {
        let start = Vec2::new(0, 0);
        
        Rope { 
            segments: (0..parts).map(|_| Vec2::from(&start)).collect::<Vec<Vec2>>(),
            visited: vec![
                Vec2::from(&start)
            ]
        }
    }

    fn move_head(&mut self, direction: &Vec2) {
        let mut head = self.segments[0].cloned();
        let last_seg = self.segments.len()-1;

        //Update head
        head.x += direction.x;
        head.y += direction.y;
        
        self.segments[0] = head; 

        //Update trailing segments
        for i in 0..last_seg { 
            let seg_a = self.segments[i].cloned();
            let mut seg_b = self.segments[i + 1].cloned();
            
            let distance = seg_a.sub(&seg_b);
            
            if distance.max_abs() >= 2 {
                seg_b = seg_b.add(&distance.normalized());

                if i+1 == last_seg && !self.visited.contains(&seg_b) {
                    self.visited.push(Vec2::from(&seg_b))
                }
            }
            
            self.segments[i + 1] = seg_b;
        }
    }

    fn move_distance(&mut self, direction: &Vec2, steps: usize) {
        for _ in 0..steps {
            self.move_head(direction)
        }
    }

    fn simulate (&mut self, input: &str) -> &mut Rope {
        for line in input.lines() {
            let (dir, dis) = match line.split(" ").collect::<Vec<&str>>().as_slice() {
                ["R", s] => (Vec2::new( 1, 0), *s),
                ["L", s] => (Vec2::new(-1, 0), *s),
                ["D", s] => (Vec2::new( 0, 1), *s),
                ["U", s] => (Vec2::new( 0,-1), *s),
                _ => panic!("Failed to parse input")
            };

            self.move_distance(&dir, dis.parse::<usize>().unwrap());
        }

        self
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let ans = Rope::new(2).simulate(input).visited.len() as u32;

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ans = Rope::new(10).simulate(input).visited.len() as u32;

    Some(ans)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        assert_eq!(part_two(&input), Some(36));
    }
}
