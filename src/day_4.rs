use std::{borrow::Borrow, result};

use crate::file_utils;

pub fn day4() {
    let lines = file_utils::get_lines_reader("files/day_4.txt");

    let mut total = 0;

    for i in 0..lines.len() {
        let ranges: Vec<&str> = lines[i].split(",").collect();

        let mut r1 = Range::new();
        r1.init(&ranges[0]);

        let mut r2 = Range::new();
        r2.init(&ranges[1]);

        // fully containes ranges :
        if (r1.min >= r2.min && r1.max <= r2.max) || (r2.min >= r1.min && r2.max <= r1.max) {
            total += 1;
        } else {
            // ..2345..
            // ...3456..
            // .....5678..
            // ....456..
            //

            // overlapping ranges :
            if (r1.min <= r2.min && r1.max >= r2.min) || (r1.min <= r2.max && r1.max >= r2.max) {
                total += 1;
            }
        }
    }

    println!("{}", total);
}

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn new() -> Range {
        Range { min: 0, max: 0 }
    }

    fn init(&mut self, value: &str) {
        let values: Vec<&str> = value.split("-").collect();
        self.min = u32::from_str_radix(values[0], 10).unwrap();
        self.max = u32::from_str_radix(values[1], 10).unwrap();
    }

    fn get_values(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();

        for i in self.min..self.max {
            result.push(i);
        }

        return result;
    }
}
