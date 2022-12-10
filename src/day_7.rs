use crate::file_utils;
use lazy_static::lazy_static;
use regex::Regex;

const SPACE: &str = " ";

pub fn day7() {
    let path = String::from("files/day_7.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let root = Directory {
        parent: 0,
        name: String::from("/"),
        files: Vec::new(),
        dirs: Vec::new(),
    };

    let mut directories: Vec<Directory> = Vec::new();
    directories.push(root);
    let mut current_directory_index = 0;

    // fill the tree :
    for i in 0..lines.len() {
        let line = &lines[i];

        let values: Vec<&str> = line.split(' ').collect();

        lazy_static! {
            static ref CDROOT: Regex = Regex::new(r"\$ cd /").unwrap();
            static ref CDUP: Regex = Regex::new(r"\$ cd '.''.'").unwrap();
            static ref CD: Regex = Regex::new(r"\$ cd ").unwrap();
            static ref LS: Regex = Regex::new(r"\$ ls").unwrap();
            static ref DIR: Regex = Regex::new(r"dir \w+").unwrap();
            static ref FILE: Regex = Regex::new(r"\d+ [a-z]+.??[a-z]*").unwrap();
        }

        if CDROOT.is_match(line) {
            println!("ROOT");
            current_directory_index = 0;
        } else if line == "$ cd .." {
            // cd ..
            current_directory_index =
                get_directory(current_directory_index, &mut directories).parent;
        } else if CD.is_match(line) {
            // ex : $ cd bar
            let current_dir = get_directory_readonly(current_directory_index, &directories);
            let indexes = current_dir.dirs.as_slice();

            for dir in indexes {
                if get_directory_readonly(*dir, &directories).name == values[2] {
                    current_directory_index = *dir;
                }
            }
        } else if LS.is_match(line) {
        } else if DIR.is_match(line) {
            // ex: dir foo
            let new_dir = Directory {
                name: String::from(values[1]),
                parent: current_directory_index,
                files: Vec::new(),
                dirs: Vec::new(),
            };
            directories.push(new_dir);
            let new_dir_index = directories.len() - 1;

            let current = &mut get_directory(current_directory_index, &mut directories);

            current.dirs.push(new_dir_index);
        } else if FILE.is_match(line) {
            // ex : 12456 foo.bar
            let new_file = File {
                name: String::from(values[1]),
                size: values[0].parse().unwrap(),
            };
            get_directory(current_directory_index, &mut directories).add_file(new_file);
        }
    }

    directories[0].display(SPACE, &directories);

    let used_space = directories[0].size(&directories);
    let total_space = 70000000;
    let free_space = total_space - used_space;
    let necessary_space = 30000000 - free_space;

    println!("total space: {}", total_space);
    println!("used space : {}", used_space);
    println!("free space : {}", free_space);
    println!("necessary : {}", necessary_space);

    let mut smallest = total_space;
    for d in &directories {
        let size = d.size(&directories).into();
        if size >= necessary_space && size < smallest {
            smallest = size;
        }
    }
    println!("TOTAL : {}", smallest);

    // day 7 - part 1
    // let mut total = 0;
    // for d in &directories {
    //     let size: u128 = d.size(&directories).into();
    //     //println!("{}", size);
    //     if size <= 100000 {
    //         total += size;
    //     }
    // }
    // println!("TOTAL : {}", total);
}

fn get_directory_readonly(index: usize, directories: &Vec<Directory>) -> &Directory {
    return &directories[index];
}

fn get_directory(index: usize, directories: &mut Vec<Directory>) -> &mut Directory {
    return &mut directories[index];
}

struct File {
    name: String,
    size: u32,
}

struct Directory {
    name: String,
    parent: usize,
    files: Vec<File>,
    dirs: Vec<usize>,
}

impl Directory {
    fn size(&self, directories: &Vec<Directory>) -> u32 {
        let mut total: u32 = 0;
        for child in &self.files {
            total += child.size;
        }
        for child in &self.dirs {
            total += directories[*child].size(directories);
        }
        total
    }

    fn display(&self, space: &str, directories: &Vec<Directory>) {
        println!(
            "{}{}-{}",
            space,
            self.name.to_uppercase(),
            self.size(directories)
        );
        for child in &self.files {
            println!(" {}{} - {}", &space, child.name, child.size);
        }
        for child in &self.dirs {
            let dir = &directories[*child];
            dir.display(&format!("{}{}", &space, SPACE), &directories);
        }
    }

    fn add_file(&mut self, item: File) {
        self.files.push(item);
    }
}
