use std::{collections::HashMap, hash::Hash};




struct HeightMap {
    map: HashMap<Point, i32>,
    start: Point,
    end: Point
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn neighbours(&self) -> Vec<Point> {
        vec![
            Point { x: self.x + 1, y: self.y + 0 },
            Point { x: self.x - 1, y: self.y + 0 },
            Point { x: self.x + 0, y: self.y + 1 },
            Point { x: self.x + 0, y: self.y - 1 },
        ]
    }
}

impl HeightMap {
    fn from(input: &str) -> HeightMap {
        let mut map: HashMap<Point, i32> = HashMap::new();
        let mut current = Point {x: 0, y: 0};
        let mut start: Point = current.clone();
        let mut end: Point = current.clone();

        for byte in input.bytes() {
            if byte == 83 {
                start = current.clone();
                map.insert(current, 0);
            } else if byte == 69 {
                end = current.clone();
                map.insert(current, 26);
            } else if byte == 10 {
                current.y += 1;
                current.x = -1;
            } else if byte >= 97 {
                map.insert(current, (byte - 97).into());
            }
            current.x += 1
        }

        HeightMap { map, start, end }
    }

    fn traverse (&mut self, current: Point, mut visited: Vec<Point>, mut found: Vec<Vec<Point>>) -> Vec<Vec<Point>> {
        visited.push(current);
        //println!("Current: {:?}", current);

        if current == self.end {
            found.push(visited)
        } else {
            let filtered = current.neighbours().into_iter().filter(|n| {
                //println!("NB: {:?}", n);
                if let (Some(test), Some(current)) = (self.map.get(n), self.map.get(&current)) {
                    let delta = test - current;
                    let step_ok = (delta >= 0 && delta < 2);
                    let not_visited = !visited.contains(n);
                    //println!("{:?} - No visits: {} - Step ok: {} d: {}", n, not_visited, step_ok, delta);
                    not_visited && step_ok
                } else {
                    false
                }
            }).collect::<Vec<_>>();
    
            //println!("Filtered: {:?}", filtered);

            for next in filtered {
                let mut path = self.traverse(next, visited.clone(), found.clone());
                if path.len() > 0 {
                    found.append(&mut path);
                }
            }
        }

        found
    }

    fn solve(&mut self) {
        println!("MAP: {:?}", self.map);
        let paths = self.traverse(self.start, vec![], vec![]);
        println!("Paths: {:?}", paths)
    }
}



pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HeightMap::from(input);

    map.solve();

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
