use crate::file_utils;
use num::pow;
use std::cmp::*;

// 409 : too low

type Tuple = (usize, usize);

pub fn day14() {
    let file_path = String::from("files/day_14.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(file_path);

    let mut cave_height: usize = 0;
    let mut column_offset: usize = 500;

    let mut cave = build_cave(&lines, &mut cave_height, &mut column_offset);

    for line in &cave {
        println!("-{}", line.len());
    }

    // part 2:
    add_floor(&mut cave, &mut cave_height);

    //draw_cave(&cave, column_offset, cave_height);

    let mut p: (usize, usize) = (0, 0);

    let mut units = 0;

    while p != (usize::MAX, usize::MAX) {
        p = add_sand(500, &mut cave, cave_height, column_offset, false);
        if p != (usize::MAX, usize::MAX) {
            units += 1;
        }
    }

    add_sand(500, &mut cave, cave_height, column_offset, true);
    draw_cave(&cave, column_offset, cave_height);

    println!("UNITS : {} ", units);
}

fn add_floor(cave: &mut Vec<Vec<char>>, cave_height: &mut usize) {
    // add 2 lines :
    for line in cave {
        line.push('.');
        line.push('#');
    }

    *cave_height = *cave_height + 2;
}

fn build_cave(
    lines: &Vec<String>,
    cave_height: &mut usize,
    column_offset: &mut usize,
) -> Vec<Vec<char>> {
    let mut cave: Vec<Vec<char>> = Vec::new();

    //let mut column_offset: usize = 500;
    //let mut cave_height: usize = 0;

    cave.push(Vec::new()); // create column 500

    for line in lines {
        let points: Vec<&str> = line.split("->").collect();

        for i in 1..points.len() {
            let start = parse_point(points[i - 1]);
            let end = parse_point(points[i]);

            let from_column = min(start.0, end.0);
            let to_column = max(start.0, end.0);

            let lowest_y = max(start.1, end.1);

            // add columns if extended width:
            if from_column < *column_offset || to_column > *column_offset {
                *column_offset = add_missing_columns(
                    from_column,
                    to_column,
                    &mut cave,
                    *column_offset,
                    *cave_height,
                );
            }

            // add lines if extended height :
            if lowest_y > *cave_height {
                for column in &mut cave {
                    while column.len() <= lowest_y {
                        column.push('.');
                    }
                }
                *cave_height = lowest_y + 1;
            }

            if start.0 != end.0 {
                // horizontal line

                // draw rocks on the target line :
                let line_index = start.1;
                for y in from_column..to_column + 1 {
                    cave[y - *column_offset][line_index] = '#';
                }
            } else {
                // vertical line

                // draw rocks in the target column :
                let column_index = start.0 - *column_offset;

                let range = min(start.1, end.1)..max(start.1, end.1) + 1;

                for y in range {
                    cave[column_index][y] = '#';
                }
            }
        }
    }

    // weird ?!
    //*cave_height = *cave_height + 1;

    return cave;
}

fn add_missing_columns(
    from_column: usize,
    to_column: usize,
    cave: &mut Vec<Vec<char>>,
    column_offset: usize,
    cave_height: usize,
) -> usize {
    let mut new_column_offset = column_offset;

    // columns before
    if from_column < column_offset {
        for _ in from_column..column_offset {
            let mut new_column: Vec<char> = Vec::new();

            for _ in 0..cave_height {
                new_column.push('.');
            }

            cave.insert(0, new_column);
        }

        new_column_offset = from_column;
    }

    // columns after
    let cave_length = cave.len();
    if to_column > column_offset + cave_length {
        let nb_columns_to_add = to_column - new_column_offset - cave_length;

        for _ in 0..nb_columns_to_add + 1 {
            let mut new_column: Vec<char> = Vec::new();

            for _ in 0..cave_height {
                new_column.push('.');
            }

            cave.push(new_column);
        }
    }

    return new_column_offset;
}

fn parse_point(point: &str) -> Tuple {
    let values: Vec<&str> = point.split(",").collect();
    return (
        values[0].trim().parse().unwrap(),
        values[1].trim().parse().unwrap(),
    );
}

fn draw_cave(cave: &Vec<Vec<char>>, column_offset: usize, cave_height: usize) {
    for lvl in (1..4).rev() {
        print!("\t");
        for i in 0..cave.len() {
            let idx = i + column_offset;

            print!("{}", (idx % pow(10, lvl)) / pow(10, lvl - 1));
        }
        println!();
    }

    for j in 0..cave_height {
        print!("{}\t", j);
        for i in 0..cave.len() {
            if cave[i][j] == '.' {
                if i % 2 == 0 {
                    print!("{}", '-');
                } else {
                    print!("{}", '.');
                }
            } else {
                print!("{}", cave[i][j]);
            }
        }
        println!();
    }
}

fn add_sand(
    start_column: usize,
    cave: &mut Vec<Vec<char>>,
    cave_height: usize,
    column_offset: usize,
    draw_steps: bool,
) -> (usize, usize) {
    let mut column: i32 = start_column as i32 - column_offset as i32;

    const MAX: usize = usize::MAX;

    let mut position = (MAX, MAX);

    let mut blocked_tile: bool; // = false;

    for line in 0..cave_height {
        // current tile is empty
        if cave[column as usize][line] == '.' {
            position = (column as usize, line);
            if draw_steps {
                cave[position.0][position.1] = 'x';
                //draw_cave(&cave, column_offset, cave_height);
            }
            blocked_tile = false;
        } else {
            blocked_tile = true;
        }

        // reached bottom:
        if line == cave_height - 1 {
            position = (MAX, MAX);
            break;
        }

        // next tile is blocked
        if !(line < cave_height - 1 && cave[column as usize][line + 1] == '.') {
            // current tile is blocked:
            //try diagonal left:
            if column - 1 < 0 {
                position = (MAX, MAX);
                break;
            } else if column - 1 >= 0
                && line + 1 < cave_height
                && cave[(column - 1) as usize][line + 1] == '.'
            {
                column = column - 1;
                blocked_tile = false;
            }
            // try diagonal right:
            else if column + 1 == cave.len() as i32 {
                position = (MAX, MAX);
                break;
            } else if column + 1 < cave.len() as i32
                && line + 1 < cave_height
                && cave[(column + 1) as usize][line + 1] == '.'
            {
                column = column + 1;
                blocked_tile = false;
            } else if column < 0 || column >= cave.len() as i32 {
                position = (MAX, MAX);
                break;
            } else {
                blocked_tile = true;
            }
        }

        if blocked_tile {
            break;
        }
    }

    if position != (MAX, MAX) {
        cave[position.0][position.1] = 'o';
    }

    return position;
}

fn add_sand_v2(
    start_column: usize,
    cave: &mut Vec<Vec<char>>,
    cave_height: usize,
    column_offset: &mut usize,
    draw_steps: bool,
) -> (usize, usize) {
    let mut column: i32 = start_column as i32 - *column_offset as i32;

    const MAX: usize = usize::MAX;

    let mut position = (MAX, MAX);

    let mut blocked_tile: bool; // = false;

    for line in 0..cave_height {
        // current tile is empty
        if cave[column as usize][line] == '.' {
            position = (column as usize, line);
            if draw_steps {
                cave[position.0][position.1] = 'x';
                //draw_cave(&cave, column_offset, cave_height);
            }
            blocked_tile = false;
        } else {
            blocked_tile = true;
        }

        // reached bottom:
        if line == cave_height - 1 {
            position = (MAX, MAX);
            break;
        }

        // next tile is blocked
        if !(line < cave_height - 1 && cave[column as usize][line + 1] == '.') {
            // current tile is blocked:
            //try diagonal left:
            if column - 1 < 0 {
                add_missing_columns(
                    *column_offset - 1,
                    *column_offset,
                    cave,
                    *column_offset,
                    cave_height,
                );
                column = column - 1;
                blocked_tile = false;
            } else if column - 1 >= 0
                && line + 1 < cave_height
                && cave[(column - 1) as usize][line + 1] == '.'
            {
                column = column - 1;
                blocked_tile = false;
            }
            // try diagonal right:
            else if column + 1 == cave.len() as i32 {
                add_missing_columns(
                    *column_offset + cave.len() - 1,
                    *column_offset + cave.len(),
                    cave,
                    *column_offset,
                    cave_height,
                );
                column = column - 1;
                blocked_tile = false;
            } else if column + 1 < cave.len() as i32
                && line + 1 < cave_height
                && cave[(column + 1) as usize][line + 1] == '.'
            {
                column = column + 1;
                blocked_tile = false;
            }
            // else if column < 0 || column >= cave.len() as i32 {
            //     position = (MAX, MAX);
            //     break;
            // }
            else {
                blocked_tile = true;
            }
        }

        if blocked_tile {
            break;
        }
    }

    if position != (MAX, MAX) {
        cave[position.0][position.1] = 'o';
    }

    return position;
}
