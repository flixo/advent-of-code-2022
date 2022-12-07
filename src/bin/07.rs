use std::{rc::Rc, sync::{Arc, Mutex}};

#[derive(Debug)]
struct Node<'a> {
    is_folder: bool,
    name: String,
    children: Vec<Arc<Mutex<Node<'a>>>>,
    parent: Option<Arc<Mutex<&'a Node<'a>>>>
}


impl<'a> Node<'a> {
    fn new(name: String, folder: bool, parent: Option<Arc<Mutex<&'a Node<'a>>>>) -> Node<'a> {
        Node {
            is_folder: folder, 
            name: name,
            children: vec![],
            parent: parent
        }
    }

    fn add_child(&'a mut self, name: String, folder: bool) -> () {
        let child = Node::new(name, folder, 
            Some(Arc::new(Mutex::new(self)))
        );
        let _ = &self.children.push(Arc::new(Mutex::new(child)));
    }

    fn get_parent(&self) -> Option<&Node<'a>> {
        if let Some(result) = self.parent {
            Some(&result.lock().unwrap())
        } else {
            None
        }
    }
    fn get_child(&self, name: String) -> Option<&Node<'a>> {
        let res = self.children.iter().find(|c| {
            c.lock().unwrap().name == name
        });

        if let Some(result) = res {
            Some(&result.lock().unwrap())
        } else {
            None
        }
    }
}



pub fn part_one(input: &str) -> Option<u32> {
    let fs = Node::new(String::from("root"), true, None);
    let mut current_node = &fs;



    for line in input.lines() {
        match line.split(" ").collect::<Vec<&str>>().as_slice() {
            ["$", "cd", arg] => {
                println!("CMD CD: {line}");
                match *arg {
                    "/" => current_node = &fs, //Root
                    ".." => {
                        if let Some(node) = current_node.get_parent() {
                            current_node = node;
                        }
                    }
                    name  => {
                        if let Some(node) = current_node.get_child(String::from(name)) {
                            current_node = node;
                        }
                    },
                }
            }
            ["$", "ls", arg] => {
                println!("CMD LS: {line}")
            }
            ["dir", name] => {
                //current_node.add_child(String::from(*name), true);
                println!("Dir: {line}")
            }
            [size, name] => {
                //current_node.add_child(String::from(*name), false);
                println!("File: {line}")
            }
            _ => {}
        }
    }


    //fs.add_child(String::from("Test.exe"), false);
    //fs.add_child(String::from("file.exe"), false);
    //fs.add_child(String::from("mac.exe"), false);

    //println!("{:#?}", fs);

    Some(1)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
