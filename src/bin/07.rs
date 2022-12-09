#[derive(Debug)]
struct Node {
    name: String,
    is_directory: bool,
    index: usize,
    children: Vec<usize>,
    parent: Option<usize>,
    size: usize
}

#[derive(Debug)]
struct FileSystem {
    nodes: Vec<Node>
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            nodes: vec![]
        }
    }
    fn create_node(&mut self, name: &str, is_directory: bool, size: usize) -> usize {
        let index = self.nodes.len();
        let node = Node {
            name: String::from(name),
            index: index,
            is_directory: is_directory,
            children: vec![],
            parent: None,
            size: size
        };
        self.nodes.push(node);
        index
    }

    fn update_parent_size(&mut self, node_idx: usize, size: usize) {
        let node = self.nodes.get_mut(node_idx).unwrap();
        node.size += size;

        if let Some(node_idx) = node.parent {
            self.update_parent_size(node_idx, size)
        }
    }

    fn add_to(&mut self, parent_idx: usize, child_idx: usize) -> () {
        let child_size = self.get_node(child_idx).unwrap().size;

        let parent_node = self.nodes.get_mut(parent_idx).unwrap();
            parent_node.children.push(child_idx);
            
        let parent_idx = parent_node.index;
        
        let child_node = self.nodes.get_mut(child_idx).unwrap();
            child_node.parent = Some(parent_idx);

        self.update_parent_size(parent_idx, child_size)
    }

    fn get_parent(&self, node_idx: usize) -> Option<usize> {
        self.nodes.get(node_idx).unwrap().parent
    }

    fn get_by_name(&self, parent_idx: usize, name: &str) -> Option<usize> {
        let parent_node = self.nodes.get(parent_idx).unwrap();
        let mut child: Option<&Node>;

        for child_idx in &parent_node.children {
            child = self.nodes.get(*child_idx);
            if let Some(child) = child {
                if child.name == name { 
                    return Some(child.index);
                }
            }
        }

        None
    }

    
    fn get_node(&self, node_idx: usize) -> Option<&Node> {
        self.nodes.get(node_idx)
    }
    

    //For debugging, not used in solution
    fn _tree(&self, node_idx: usize, depth: usize) {
        let node_idxs = &self.nodes.get(node_idx).unwrap().children;
        for node_idx in node_idxs {
            let node = self.get_node(*node_idx).unwrap();
            let label = if node.is_directory {"DIR"} else {"FIL"};

            println!("{}[{}] <{}> {}", " ".repeat(3 * depth), node.name, label, node.size);

            self._tree(node.index, depth + 1);
        }
    }
}


fn create_fs(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    let root = fs.create_node("root", true, 0);
    
    let mut current_node_idx = root;
    
    for line in input.lines() {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "cd", arg] => {
                match *arg {
                    "/" => current_node_idx = root, //Root
                    ".." => {
                        if let Some(node_idx) = fs.get_parent(current_node_idx) {
                            current_node_idx = node_idx;
                        }
                    }
                    name  => {
                        if let Some(node_idx) = fs.get_by_name(current_node_idx, name) {
                            current_node_idx = node_idx;
                        }
                    },
                }
            }
            ["$", "ls"] => {
                //
            }
            ["dir", name] => {
                let node_idx = fs.create_node(name, true, 0);
                fs.add_to(current_node_idx, node_idx);
            }
            [size, name] => {
                let node_idx = fs.create_node(name, false, size.parse().unwrap());
                fs.add_to(current_node_idx, node_idx);
            }
            _ => {}
        }
    }

    fs
}

pub fn part_one(input: &str) -> Option<u32> {
    let fs = create_fs(input);
    let ans = fs.nodes
        .into_iter()
        .filter(|node| node.is_directory && node.size <= 100_000)
        .map(|c| c.size)
        .sum::<usize>();

    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let fs = create_fs(input);
    
    let required_space = 30000000;
    let total_space = 70000000;
    let root_size = fs.nodes.get(0).unwrap().size;
    let available_space = total_space - root_size;
    let mut ans: usize = total_space;

    fs.nodes.into_iter()
        .filter(|node| node.is_directory)
        .for_each(|d| {
            if available_space + d.size >= required_space && d.size < ans {
                ans = d.size
            }
        });
        
    Some(ans as u32)
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
