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
            parent: parent,
            size: 0u128,
        }
    }

    // pub fn add_subdir(&mut self, name: &str, new_dir: Rc<RefCell<Folder>>) {
    //     self.subdirs.insert(String::from(name), new_dir);
    // }

    pub fn recurse(&self, acc: &mut u128) -> () {
        println!("{} -> {}", self.name, self.size);
        if self.size < 100000 {
            *acc += self.size;
        }
        for subdir in self.subdirs.values() {
            subdir.borrow().recurse(acc);
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
                    // .add_subdir(&dirname, new_dir);
                    .subdirs
                    .insert(dirname.clone(), Rc::clone(&new_dir));
                {
                    let mut mut_subdir = new_dir.borrow_mut();
                    mut_subdir.parent = Some(Rc::clone(&pwd));
                }
                // println!("added {:?} (theoretically)", dirname);
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
                    //while ptr.borrow().parent.is_some() {
                    if ptr.borrow().parent.is_none() {
                        break;
                    }
                    // how do I follow references back up the dir structure?!
                    let mut parent = Rc::clone(&ptr.borrow_mut().parent.as_ref().unwrap());
                    parent.borrow_mut().size += size;
                    // see if there's another level, otherwise break
                    ptr = parent;
                }
            }
        }
        // println!("{:?}, pwd is {}", line, pwd.as_ref().borrow());
    }
    dbg!(&root);
    // finally, after all this buildup, traverse from the root and gather up the data we need
    // if this dir is < 100k then add its size
    // otherwise, see if one of the subdirs is smaller than 100k and add its size
    let mut result = 0u128;
    let ptr = Rc::clone(&root);

    // Some(Rc::clone(&root).borrow().size.clone())
    ptr.borrow().recurse(&mut result);
    Some(result)
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
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
