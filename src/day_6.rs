use crate::file_utils;

pub fn day6() {
    let path = String::from("files/day_6.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let line = &lines[0];

    let position = find_sequence(&line);

    println!("{}", position);
}

fn find_sequence(line: &String) -> usize {
    const SIZE: usize = 14;

    let mut chars: [char; SIZE] = [' '; SIZE];
    //let mut seq = 0;

    let line_chars: Vec<char> = line.trim().chars().collect();

    for index in 0..line_chars.len() {
        let c = line_chars[index];
        // initial filling :
        if index < 4 {
            chars[index] = c;
        } else {
            let mut found = true;
            for i in 0..SIZE {
                for j in 0..SIZE {
                    if i != j && chars[i] == chars[j] {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                return index;
            } else {
                // insert the new char :  abcd >> bcde
                for i in 0..SIZE - 1 {
                    chars[i] = chars[i + 1];
                }
                chars[SIZE - 1] = c;

                for i in 0..SIZE {
                    print!("{}", chars[i]);
                }
                println!();
            }
        }
    }

    999999999
}
