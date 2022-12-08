use std::collections::HashMap;

//#[macro_use]
//extern crate lazy_static;
use crate::file_utils;
use lazy_static::lazy_static;
use regex::Regex;

pub fn day7() {
    let path = String::from("files/day_7.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let mut items: HashMap<&str, Item> = HashMap::new();

    items.insert(
        "",
        Item {
            name: "/".to_string(),
            size: 0,
        },
    );

    parse_line("");
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

struct Item {
    name: String,
    size: i32,
}
