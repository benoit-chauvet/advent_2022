use crate::file_utils;

pub fn day8() {
    let path = String::from("files/day_8.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    const HEIGHT: usize = 5; //99; // lines.len();
    const WIDTH: usize = 5; //99; //lines[0].trim().len();

    println!("h:{} w:{}", HEIGHT, WIDTH);

    let mut forest: [[i32; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];

    for i in 0..lines.len() {
        let line = &lines[i];

        let chars: Vec<char> = line.trim().chars().collect();

        for j in 0..chars.len() {
            forest[i][j] = chars[j] as i32 - 0x30;
        }
    }

    let mut visible_trees = (WIDTH * 2) + (HEIGHT * 2) - 4;
    println!("{}", visible_trees);

    for i in 1..WIDTH - 1 {
        let mut highest_top = forest[i][0];
        let mut highest_bottom = forest[i][HEIGHT - 1];

        for j in 1..HEIGHT - 1 {
            let mut highest_left = forest[0][j];
            let mut highest_right = forest[WIDTH - 1][j];

            for y in 0..j - 1 {
                if forest[i][y] > highest_top {
                    highest_top = forest[i][y];
                }
            }

            for y in (j - 1..HEIGHT - 1).rev() {
                if forest[i][y] > highest_bottom {
                    highest_bottom = forest[i][y];
                }
            }

            for x in 0..i - 1 {
                if forest[x][j] > highest_left {
                    highest_left = forest[x][j];
                }
            }

            for x in (i - 1..WIDTH - 1).rev() {
                if forest[x][j] > highest_right {
                    highest_right = forest[x][j];
                }
            }

            let mut counted = false;
            // TOP
            if forest[i][j] > highest_top {
                visible_trees += 1;
                counted = true;
                highest_top = forest[i][j];
            }
            // BOTTOM
            else if forest[i][j] > highest_bottom {
                visible_trees += 1;
                counted = true;
                highest_bottom = forest[i][j];
            }
            // LEFT
            else if forest[i][j] > highest_left {
                visible_trees += 1;
                counted = true;
                highest_left = forest[i][j];
            }
            // RIGHT
            else if forest[i][j] > highest_right {
                visible_trees += 1;
                counted = true;
                highest_right = forest[i][j];
            }
            if (counted) {
                print!("v");
            } else {
                print!("-");
            }
        }

        println!();
    }

    println!("VISIBLE TREES : {}", visible_trees);

    // x x x x x x
    // x x x x x x
    // x x x x x x
    // x x x x x x
    // x x x x x x
    // x x x x x x

    // for i in 0..height {
    //     for j in 0..width {
    //         print!("{}", forest[i][j]);
    //     }
    //     println!();
    // }
}
