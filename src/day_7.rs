use std::{cell::RefCell, rc::Rc};

//#[macro_use]
//extern crate lazy_static;
use crate::file_utils;
use lazy_static::lazy_static;
use regex::Regex;

pub fn day7() {
    let path = String::from("files/day_7.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let root = Directory {
        name: String::from("/"),
        files: Vec::new(),
        dirs: Vec::new(),
    };

    let mut dirs: Vec<Rc<Directory>> = Vec::new();
    dirs.push(Rc::new(root));

    for i in 2..lines.len() {
        let line = &lines[i];

        let values: Vec<&str> = line.split(' ').collect();

        lazy_static! {
            static ref CDUP: Regex = Regex::new(r"\$ cd ..").unwrap();
            static ref CD: Regex = Regex::new(r"\$ cd ").unwrap();
            static ref LS: Regex = Regex::new(r"\$ ls").unwrap();
            static ref DIR: Regex = Regex::new(r"dir \w+").unwrap();
            static ref FILE: Regex = Regex::new(r"\d+ [a-z]+.??[a-z]*").unwrap();
        }

        if CDUP.is_match(line) {
            dirs.pop();
        } else if CD.is_match(line) {
            // ex : $ cd bar
            let new_dir = Directory {
                name: String::from(values[2]),
                files: Vec::new(),
                dirs: Vec::new(),
            };
            let current = dirs.last_mut().unwrap();
            current.add_dir(new_dir);
            dirs.push(Rc::new(new_dir));
        } else if LS.is_match(line) {
        } else if DIR.is_match(line) {
            // ex: dir foo
            //     let new_dir = Directory {
            //         name: String::from(values[1]),
            //         files: Vec::new(),
            //         dirs: Vec::new(),
            //     };
            //     dirs.last_mut().unwrap().get_mut().add_dir(new_dir);
        } else if FILE.is_match(line) {
            // ex : 12456 foo.bar
            let new_file = File {
                name: String::from(values[1]),
                size: values[0].parse().unwrap(),
            };
            dirs.last_mut().unwrap().add_file(new_file);
        }
    }
}

fn parse_line(line: &str) {
    lazy_static! {
        static ref CD: Regex = Regex::new(r"\$ cd ").unwrap();
        static ref LS: Regex = Regex::new(r"\$ ls").unwrap();
        static ref DIR: Regex = Regex::new(r"dir \w+").unwrap();
        static ref FILE: Regex = Regex::new(r"\d+ [a-z]+.??[a-z]*").unwrap();
    }

    if CD.is_match(line) {
    } else if LS.is_match(line) {
    } else if DIR.is_match(line) {
    } else if FILE.is_match(line) {
    }
}

trait FSItem {
    fn size(&self) -> u32;
    fn display(&self, space: &String);
    fn name(&self) -> &str;
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

    fn name(&self) -> &str {
        return &self.name;
    }
}

struct Directory {
    name: String,
    //children: Vec<Box<dyn FSItem>>,
    files: Vec<File>,
    dirs: Vec<Rc<Directory>>,
}

impl FSItem for Directory {
    fn size(&self) -> u32 {
        let mut total: u32 = 0;
        for child in &self.files {
            total += child.size();
        }
        for child in &self.dirs {
            total += child.size();
        }
        total
    }

    fn display(&self, space: &String) {
        println!("{0}-{1} {2}", space, self.name, self.size());
        for child in &self.files {
            child.display(&format!("{}{}", &space, " "));
        }
        for child in &self.dirs {
            child.display(&format!("{}{}", &space, " "));
        }
    }

    fn name(&self) -> &str {
        return &self.name;
    }
}

impl Directory {
    //fn add(&mut self, item: Box<dyn FSItem>) {
    //    self.children.push(item);
    //}

    fn add_dir(&mut self, item: Directory) {
        self.dirs.push(Rc::new(item));
    }

    fn add_file(&mut self, item: File) {
        self.files.push(item);
    }
}
