use std::ops;

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
}

impl ops::Add<&Vec2> for &Vec2 {
    type Output = Vec2;
    fn add(self, v2: &Vec2) -> Vec2 {
        Vec2::new(self.x + v2.x, self.y + v2.y)
    }
}

#[derive(Debug)]
struct Grid {
    size: Vec2,
    cells: Vec<usize>
}


impl Grid {
    fn from(input: &str) -> Grid {
        let mut size = Vec2 { x: 0, y: 0 };
        let mut cells: Vec<usize> = vec![];

        for (y, line) in input.lines().enumerate() {
            size.y = (y as i32) + 1;
            for (x, value) in line.chars().map(|ch| ch.to_string().parse::<usize>().unwrap()).enumerate() {
                cells.push(value);
                size.x = (x as i32) + 1;
            }
        }
        
        Grid { cells, size }
    }

    fn get(&self, coord: &Vec2) -> Option<&usize> {
        if coord.y > self.size.y-1 || coord.x > self.size.x-1 || coord.x < 0 || coord.y < 0 {
            None
        } else {
            let i = coord.y * self.size.y + coord.x;
            self.cells.get(i as usize)
        }
    }

    fn visible_direction(&self, from: &Vec2, direction: &Vec2, height: &usize, acc: usize) -> (bool, usize) {
        let cell_coord = from + direction;
        if let Some(cell_height) = self.get(&cell_coord) {
            if cell_height >= height  { return (false, acc) }
            return self.visible_direction(&cell_coord, direction, height, acc + 1)
        }

        (true, acc-1)
    }

    fn visible(&self, source_coord: &Vec2) -> (bool, usize) {
        if let Some(height) = self.get(source_coord) {
            let directions = vec![
                Vec2::new( 1,  0), //Right
                Vec2::new(-1,  0), //Left
                Vec2::new( 0,  1), //Bottom
                Vec2::new( 0, -1) //Top
            ];

            let mut visible = false;
            let mut tree_count_score = 1;

            for direction in directions {
                let (visibile_from_outside, tree_count_from_inside) = self.visible_direction(source_coord, &direction, height, 1);
                
                if visibile_from_outside {
                    visible = true;
                }
                
                tree_count_score *= tree_count_from_inside;
                
            };

            (visible, tree_count_score)
        } else {
            (false, 0)
        }
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    let mut ans = 0;
    
    for x in 0..grid.size.x {
        for y in 0..grid.size.y {
            let (visible, _) = grid.visible(&Vec2::new(x, y));
            if visible {
                ans += 1;
            }
        }
    }
    
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from(input);
    let mut ans = 0;
    
    for x in 0..grid.size.x {
        for y in 0..grid.size.y {
            let (_, score) = grid.visible(&Vec2::new(x, y));
            if score > ans {
                ans = score;
            }
        }
    }

    Some(ans as u32)
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
