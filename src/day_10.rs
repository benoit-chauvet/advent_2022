use std::{ops::Index, usize};

use crate::file_utils;

const SIZE: usize = 10;
const WIDTH: i32 = 40;

pub fn day10() {
    let path = String::from("files/day_10.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let mut display: Vec<char> = Vec::new();

    let mut x: i32 = 1;
    let mut cycle: i32 = 0;

    for line in lines {
        if line == "noop" {
            execute_cycle(&mut cycle, x, &mut display);
        } else {
            let values: Vec<&str> = line.trim().split(' ').collect();
            execute_cycle(&mut cycle, x, &mut display);
            execute_cycle(&mut cycle, x, &mut display);
            x += values[1].parse::<i32>().unwrap();
        }
    }

    for i in 0..display.len() {
        if i % 40 == 0 {
            println!();
        }
        print!("{}", display.index(i));
    }
    println!();
}

fn execute_cycle(cycle: &mut i32, x: i32, display: &mut Vec<char>) {
    let m_cycle = *cycle % WIDTH;
    let m_x = x % WIDTH;
    *cycle = *cycle + 1;
    if m_cycle.abs_diff(m_x) <= 1 {
        display.push('#');
    } else {
        display.push('.');
    }
}

fn day10_part1() {
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
