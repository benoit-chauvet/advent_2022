use std::usize;

use crate::file_utils;

const SIZE: usize = 10;

pub fn day9() {
    let path = String::from("files/day_9.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let mut rope: [(i32, i32); SIZE] = [(0, 0); SIZE];

    let mut visited = Vec::<(i32, i32)>::new();

    visited.push((0, 0));

    for i in 0..lines.len() {
        let line = &lines[i];

        let values: Vec<&str> = line.split(' ').collect();

        let direction = values[0];
        let steps = values[1].parse::<i32>().unwrap();

        for _i in 0..steps {
            // move head :
            let mut h = rope[0];
            match direction {
                "U" => rope[0] = (h.0, h.1 + 1),
                "D" => rope[0] = (h.0, h.1 - 1),
                "R" => rope[0] = (h.0 + 1, h.1),
                "L" => rope[0] = (h.0 - 1, h.1),
                _ => rope[0] = h,
            }

            for n in 1..SIZE {
                move_rope(&mut rope, n);
            }

            if !visited.contains(&rope[SIZE - 1]) {
                visited.push(rope[SIZE - 1]);
            }
        }
    }

    println!("{}", visited.len());
}

fn move_rope(rope: &mut [(i32, i32); SIZE], index: usize) {
    let h = rope[index - 1];
    let mut t = rope[index];

    let same = h == t;
    let adjacent_x = h.0 == t.0 && h.1.abs_diff(t.1) == 1;
    let adjacent_y = h.1 == t.1 && h.0.abs_diff(t.0) == 1;
    let diagonal = h.0.abs_diff(t.0) == 1 && h.1.abs_diff(t.1) == 1;

    if !(same || adjacent_x || adjacent_y || diagonal) {
        if h.0 != t.0 && h.1 == t.1 {
            // X
            if h.0 > t.0 {
                t.0 += 1;
            }

            if h.0 < t.0 {
                t.0 -= 1;
            }
        }

        if h.0 == t.0 && h.1 != t.1 {
            // Y
            if h.1 > t.1 {
                t.1 += 1;
            }

            if h.1 < t.1 {
                t.1 -= 1;
            }
        }

        if h.0 != t.0 && h.1 != t.1 {
            // X
            if h.0 > t.0 {
                t.0 += 1;
            }

            if h.0 < t.0 {
                t.0 -= 1;
            }
            // Y
            if h.1 > t.1 {
                t.1 += 1;
            }

            if h.1 < t.1 {
                t.1 -= 1;
            }
        }
        rope[index] = t;
    }
}
