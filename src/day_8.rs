use crate::file_utils;

const HEIGHT: usize = 99;
const WIDTH: usize = 99;

pub fn day8() {
    let path = String::from("files/day_8.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let mut forest: [[i32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];

    for i in 0..lines.len() {
        let line = &lines[i];

        let chars: Vec<char> = line.trim().chars().collect();

        for j in 0..chars.len() {
            forest[i][j] = chars[j] as i32 - 0x30;
        }
    }

    //count_visible(forest);
    scenic_score(forest);
}

fn scenic_score(forest: [[i32; WIDTH]; HEIGHT]) {
    let mut max_score = 0;
    let mut max_up = -1;
    let mut max_down = -1;
    let mut max_left = -1;
    let mut max_right = -1;
    let mut max_i = 999;
    let mut max_j = 999;

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            // look down :
            let mut down = 0;
            if j < HEIGHT - 1 {
                for y in j + 1..HEIGHT {
                    down += 1;
                    if forest[i][y] >= forest[i][j] {
                        break;
                    }
                }
            }

            // look up :
            let mut up = 0;
            if j > 0 {
                for y in (0..j).rev() {
                    up += 1;
                    if forest[i][y] >= forest[i][j] {
                        break;
                    }
                }
            }

            // look right :
            let mut right = 0;
            if i < WIDTH - 1 {
                for x in i + 1..WIDTH {
                    right += 1;
                    if forest[x][j] >= forest[i][j] {
                        break;
                    }
                }
            }

            // look left :
            let mut left = 0;
            if i > 0 {
                for x in (0..i).rev() {
                    left += 1;
                    if forest[x][j] >= forest[i][j] {
                        break;
                    }
                }
            }

            let score = up * down * left * right;

            if score > max_score {
                max_score = score;
                max_down = down;
                max_left = left;
                max_right = right;
                max_up = up;
                max_i = i;
                max_j = j;
            }
        }
    }

    println!(
        "max score : {} d{} u{} l{} r{} i{} j{}",
        max_score, max_down, max_up, max_left, max_right, max_i, max_j
    );
}

fn count_visible(forest: [[i32; WIDTH]; HEIGHT]) {
    let mut counted: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
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
}
