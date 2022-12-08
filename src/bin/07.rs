use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self},
    rc::Rc,
};

#[derive(PartialEq)]
struct Folder {
    name: String,
    files: HashMap<String, u128>,
    subdirs: HashMap<String, Rc<RefCell<Folder>>>,
    parent: Option<Rc<RefCell<Folder>>>,
    size: u128, // size of all the files in this folder and all subfolders
}

impl Folder {
    pub fn new(name: &str, parent: Option<Rc<RefCell<Folder>>>) -> Folder {
        Folder {
            name: name.into(),
            files: HashMap::new(),
            subdirs: HashMap::new(),
            parent,
            size: 0u128,
        }
    }

    pub fn find_less_than_100k(&self, acc: &mut u128) -> () {
        println!("{} -> {}", self.name, self.size);
        if self.size < 100000 {
            *acc += self.size;
        }
        for subdir in self.subdirs.values() {
            subdir.borrow().find_less_than_100k(acc);
        }
    }

    pub fn find_all_sizes(&self, acc: &mut Vec<u128>) -> () {
        println!("{} -> {}", self.name, self.size);
        acc.push(self.size);
        for subdir in self.subdirs.values() {
            subdir.borrow().find_all_sizes(acc);
        }
    }
}

impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {:?}), files: {:?}, acc size: {:?}",
            self.name,
            self.subdirs.keys(),
            self.files,
            self.size
        )
    }
}

impl fmt::Debug for Folder {
    // can't print the parent or we stackoverflow
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {:?}), files: {:?}, acc size: {:?}",
            self.name,
            self.subdirs.keys(),
            self.files,
            self.size
        )
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let root = Rc::new(RefCell::new(Folder::new("/", None)));
    let mut pwd = Rc::clone(&root);

    for line in input.lines() {
        if line.starts_with("$ cd ") {
            let cmd = line.replace("$ cd ", "");
            let name = cmd.as_str();
            match name {
                "/" => {
                    pwd = Rc::clone(&root);
                }
                ".." => {
                    let parent = Rc::clone(&pwd.borrow().parent.as_ref().unwrap());
                    pwd = Rc::clone(&parent);
                }
                _ => {
                    let d = Rc::clone(&pwd.borrow().subdirs.get(name).unwrap());
                    pwd = Rc::clone(&d);
                }
            }
        } else if line.contains("$ ls") {
            continue;
        } else {
            // reading files in the current dir
            if line.starts_with("dir ") {
                // add to subdirs
                let dirname = line.replace("dir ", "");
                let new_dir = Rc::new(RefCell::new(Folder::new(&dirname.clone(), None)));
                pwd.borrow_mut()
                    .subdirs
                    .insert(dirname.clone(), Rc::clone(&new_dir));
                {
                    let mut mut_subdir = new_dir.borrow_mut();
                    mut_subdir.parent = Some(Rc::clone(&pwd));
                }
            } else {
                // add to files
                let mut split = line.split(" ");
                let size = split.next().unwrap().parse::<u128>().unwrap();
                let filename = split.next().unwrap();
                pwd.borrow_mut()
                    .files
                    .entry(filename.to_string())
                    .and_modify(|val| *val += size)
                    .or_insert(size);
                // every time we add a file, let's traverse the dir structure and update all the ancestors so we have an easier time later
                pwd.borrow_mut().size += size;
                let mut ptr = Rc::clone(&pwd);
                loop {
                    if ptr.borrow().parent.is_none() {
                        break;
                    }
                    let parent = Rc::clone(&ptr.borrow_mut().parent.as_ref().unwrap());
                    parent.borrow_mut().size += size;
                    ptr = parent;
                }
            }
        }
    }

    // finally, after all this buildup, traverse from the root and gather up the data we need
    let mut result = 0u128;
    let ptr = Rc::clone(&root);
    ptr.borrow().find_less_than_100k(&mut result);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u128> {
    // uhm, yes, refactor this C+P mess, building the structure is the same each time, only the fn changes
    let root = Rc::new(RefCell::new(Folder::new("/", None)));
    let mut pwd = Rc::clone(&root);

    for line in input.lines() {
        if line.starts_with("$ cd ") {
            let cmd = line.replace("$ cd ", "");
            let name = cmd.as_str();
            match name {
                "/" => {
                    pwd = Rc::clone(&root);
                }
                ".." => {
                    let parent = Rc::clone(&pwd.borrow().parent.as_ref().unwrap());
                    pwd = Rc::clone(&parent);
                }
                _ => {
                    let d = Rc::clone(&pwd.borrow().subdirs.get(name).unwrap());
                    pwd = Rc::clone(&d);
                }
            }
        } else if line.contains("$ ls") {
            continue;
        } else {
            // reading files in the current dir
            if line.starts_with("dir ") {
                // add to subdirs
                let dirname = line.replace("dir ", "");
                let new_dir = Rc::new(RefCell::new(Folder::new(&dirname.clone(), None)));
                pwd.borrow_mut()
                    .subdirs
                    .insert(dirname.clone(), Rc::clone(&new_dir));
                {
                    let mut mut_subdir = new_dir.borrow_mut();
                    mut_subdir.parent = Some(Rc::clone(&pwd));
                }
            } else {
                // add to files
                let mut split = line.split(" ");
                let size = split.next().unwrap().parse::<u128>().unwrap();
                let filename = split.next().unwrap();
                pwd.borrow_mut()
                    .files
                    .entry(filename.to_string())
                    .and_modify(|val| *val += size)
                    .or_insert(size);
                // every time we add a file, let's traverse the dir structure and update all the ancestors so we have an easier time later
                pwd.borrow_mut().size += size;
                let mut ptr = Rc::clone(&pwd);
                loop {
                    if ptr.borrow().parent.is_none() {
                        break;
                    }
                    let parent = Rc::clone(&ptr.borrow_mut().parent.as_ref().unwrap());
                    parent.borrow_mut().size += size;
                    ptr = parent;
                }
            }
        }
    }

    // finally, after all this buildup, traverse from the root and gather up the data we need
    let ptr = Rc::clone(&root);
    let mut all_sizes: Vec<u128> = vec![];
    ptr.borrow().find_all_sizes(&mut all_sizes);
    all_sizes.sort(); // to make it easy to find the value we want
    let total_fs = root.borrow().size;
    let unused = 70000000u128 - total_fs;
    let goal = 30000000u128 - unused;
    println!("attempting to find first folder giving us > {}", &goal);
    for size in all_sizes {
        if size > goal {
            return Some(size);
        }
    }
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
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
