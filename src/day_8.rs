use crate::file_utils;

pub fn day8() {
    let path = String::from("files/day_8.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    const HEIGHT: usize = 5;// 99; // lines.len();
    const WIDTH: usize = 5;//99; //lines[0].trim().len();

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

    let mut visible_trees = 0;

    let mut max = -1;
    for i in 0..WIDTH {
        max = -1;
        for j in 0..HEIGHT {
            if forest[i][j] > max {
                if !counted[i][j]{
                    visible_trees+=1;
                    counted[i][j] = true;
                }
                max = forest[i][j];
            }
            // else
            // {
            //     break;
            // }
        }
    }

    for i in (0..WIDTH).rev() {
        max = -1;
        for j in 0..HEIGHT {
            if forest[i][j] > max {
                if !counted[i][j]{
                    visible_trees+=1;
                    counted[i][j] = true;
                }
                max = forest[i][j];
            }
            // else
            // {
            //     break;
            // }
        }
    }

    for i in (0..WIDTH).rev() {
        max = -1;
        for j in (0..HEIGHT).rev() {
            if forest[i][j] > max {
                if !counted[i][j]{
                    visible_trees+=1;
                    counted[i][j] = true;
                }
                max = forest[i][j];
            }
            // else
            // {
            //     break;
            // }
        }
    }

    for i in 0..WIDTH {
        max = -1;
        for j in (0..HEIGHT).rev() {
            if forest[i][j] > max {
                if !counted[i][j]{
                    visible_trees+=1;
                    counted[i][j] = true;
                }
                max = forest[i][j];
            }
            // else
            // {
            //     break;
            // }
        }
    }




    println!("VISIBLE TREES : {}", visible_trees);

    // for i in 0..height {
    //     for j in 0..width {
    //         print!("{}", forest[i][j]);
    //     }
    //     println!();
    // }
}
