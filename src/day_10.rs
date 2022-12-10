use std::usize;

use crate::file_utils;

const SIZE: usize = 10;

pub fn day10() {
    let path = String::from("files/day_10.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut strengthes: Vec<i32> = Vec::new();

    for line in lines {
        if line == "noop" {
            cycle += 1;
            check_changing_cycle(cycle, x, &mut strengthes);
        } else {
            let values: Vec<&str> = line.trim().split(' ').collect();
            cycle += 1;
            check_changing_cycle(cycle, x, &mut strengthes);
            cycle += 1;
            check_changing_cycle(cycle, x, &mut strengthes);
            x += values[1].parse::<i32>().unwrap();
        }
    }

    let mut total = 0;
    for strength in strengthes {
        total += strength;
        println!("{}", strength)
    }
    println!("SUM : {}", total);
}

fn check_changing_cycle(cycle: i32, x: i32, strengthes: &mut Vec<i32>) -> bool {
    for i in (20..240).step_by(40) {
        if i == cycle {
            println!("i{}", i);
            strengthes.push(cycle * x);
            return true;
        }
    }
    return false;
}
