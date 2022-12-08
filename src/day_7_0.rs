use std::{cell::RefCell, rc::Rc};

use crate::file_utils;

pub fn day7() {
    let path = String::from("files/day_7.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let root = Directory {
        name: String::from("/"),
        children: Vec::new(),
        parent: None,
    };

    let mut current_dir = RefCell::new(Box::new(root));
    //let mut new_dir: &mut Box<Directory> = current_dir;

    let mut total = 0;

    for i in 2..lines.len() {
        let line = &lines[i].trim().to_string();

        let values: Vec<&str> = line.split(' ').collect();

        if line == "$ cd .." {
            //current_dir = current_dir.parent.unwrap();
            let s = current_dir.borrow().size();
            if s <= 100000 {
                total += s;
            }
        } else if values[0] == "dir" {
            // ...
        } else if values[1] == "cd" {
            let mut new_dir = RefCell::new(Box::new(Directory {
                name: String::from(values[2]),
                children: Vec::new(),
                parent: Some(current_dir),
            }));

            current_dir.borrow_mut().add(new_dir);
        } else if line == "$ ls" {
            // ls : nothing to do
        } else {
            // file
            let new_file: Box<dyn FSItem> = Box::new(File {
                name: String::from(values[1]),
                size: values[0].parse().unwrap(),
            });
            current_dir.borrow_mut().add(RefCell::new(new_file));
        }
    }

    //rr.display(&String::from(""));
}

trait FSItem {
    fn size(&self) -> u32;
    fn display(&self, space: &String);
}

struct File {
    name: String,
    size: u32,
}

impl FSItem for File {
    fn size(&self) -> u32 {
        return self.size;
    }

    fn display(&self, space: &String) {
        println!("{0}*{1} {2}", space, self.name, self.size)
    }
}

struct Directory {
    name: String,
    children: Vec<RefCell<Box<dyn FSItem>>>,
    parent: Option<RefCell<Box<Directory>>>,
}

impl FSItem for Directory {
    fn size(&self) -> u32 {
        let mut total: u32 = 0;
        for child in &self.children {
            total += child.borrow().size();
        }
        total
    }

    fn display(&self, space: &String) {
        println!("{0}-{1} {2}", space, self.name, self.size());
        for child in &self.children {
            child.borrow().display(&format!("{}{}", &space, " "));
        }
    }
}

impl Directory {
    fn add(&mut self, item: RefCell<Box<dyn FSItem>>) {
        //item.display(&String::from(" "));
        self.children.push(item);
    }
}
