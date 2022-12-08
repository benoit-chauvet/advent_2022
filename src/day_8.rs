use std::num::TryFromIntError;

use crate::file_utils;

pub fn day8() {
    let path = String::from("files/day_8_prod.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    const HEIGHT: usize = 99; // lines.len();
    const WIDTH: usize = 99; //lines[0].trim().len();

    println!("h:{} w:{}", HEIGHT, WIDTH);

    let mut forest: [[i32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
    let mut counted: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

    for i in 0..lines.len() {
        let line = &lines[i];

        let chars: Vec<char> = line.trim().chars().collect();

        for j in 0..chars.len() {
            forest[i][j] = chars[j] as i32 - 0x30;
        }
    }

    let mut visible_trees = (2 * HEIGHT) + (2 * WIDTH) - 4;
    println!("{}", visible_trees);

    // RIGHT
    for j in 1..HEIGHT - 1 {
        // j : 1 2 3
        let mut max = -1;
        for i in (1..WIDTH - 1).rev() {
            // i : 3 2 1
            if forest[i + 1][j] > max {
                max = forest[i + 1][j];
            }
            if forest[i][j] > max {
                if counted[i][j] == false {
                    visible_trees += 1;
                    counted[i][j] = true;
                }
            }
        }
    }

    // LEFT
    for j in 1..HEIGHT - 1 {
        // j : 1 2 3
        let mut max = -1;
        for i in 1..WIDTH - 1 {
            // i : 1 2 3
            if forest[i - 1][j] > max {
                max = forest[i - 1][j];
            }
            if forest[i][j] > max {
                if counted[i][j] == false {
                    visible_trees += 1;
                    counted[i][j] = true;
                }
            }
        }
    }

    // TOP
    for i in 1..WIDTH - 1 {
        // i : 1 2 3
        let mut max = -1;
        for j in 1..HEIGHT - 1 {
            // j : 1 2 3
            if forest[i][j - 1] > max {
                max = forest[i][j - 1];
            }
            if forest[i][j] > max {
                if counted[i][j] == false {
                    visible_trees += 1;
                    counted[i][j] = true;
                }
            }
        }
    }

    // BOTTOM
    for i in 1..WIDTH - 1 {
        // i : 1 2 3
        let mut max = -1;
        for j in (1..HEIGHT - 1).rev() {
            // j : 3 2 1
            if forest[i][j + 1] > max {
                max = forest[i][j + 1];
            }
            if forest[i][j] > max {
                if counted[i][j] == false {
                    visible_trees += 1;
                    counted[i][j] = true;
                }
            }
        }
    }

    println!("VISIBLE TREES : {}", visible_trees);

    // for i in 0..HEIGHT {
    //     for j in 0..WIDTH {
    //         print!("{}", forest[i][j]);
    //     }
    //     println!();
    // }

    // for i in 1..HEIGHT - 1 {
    //     for j in 1..WIDTH - 1 {
    //         print!(
    //             "{}",
    //             counted[i][j].to_string().chars().collect::<Vec<char>>()[0]
    //         );
    //     }
    //     println!();
    // }

    // for i in 1..WIDTH - 1 {
    //     println!("{}", i);
    // }
    // println!("xxxxxxxxxxxxxxxxx");

    // for i in 0..HEIGHT {
    //     for j in 0..WIDTH {
    //         print!(
    //             "{}",
    //             visited[i][j].to_string().chars().collect::<Vec<char>>()[0]
    //         );
    //     }
    //     println!();
    // }

    //ttf
    //tft
    //ftf
}
