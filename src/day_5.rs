use crate::file_utils;

pub fn day5() {
    let path = String::from("files/day_5.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);
    let cargo_height: usize = 8;

    let mut ship = build_ship(&lines, cargo_height);

    process_changes(&lines, cargo_height, &mut ship);

    for i in 0..9 {
        print!("{}", ship[i].pop().unwrap());
    }
}

fn build_ship(lines: &Vec<String>, height: usize) -> [Vec<char>; 9] {
    let mut ship: [Vec<char>; 9] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    for i in (0..height).rev() {
        let line = &lines[i];

        let item_length = 3;
        let mut index = 0;
        let mut cursor = 0;
        loop {
            let chars: Vec<char> = line.trim().chars().collect();

            if cursor + item_length <= line.len() {
                let item: char = chars[cursor + 1];
                if item != '.' {
                    ship[index].push(item);
                }
                cursor = cursor + item_length + 1;
                index += 1;
            } else {
                //index = 0;
                //cursor = 0;
                break;
            }
        }
    }
    for i in 0..9 {
        println!("{}", ship[i].len());
        for j in 0..ship[i].len() {
            print!("{}", ship[i][j]);
        }
        println!("");
    }

    ship
}

fn process_changes(lines: &Vec<String>, height: usize, ship: &mut [Vec<char>; 9]) {
    for i in height + 2..lines.len() {
        let mut line = lines[i].to_string();

        line = line
            .replace("move ", "")
            .replace(" from ", ".")
            .replace(" to ", ".");

        let values: Vec<&str> = line.split('.').collect();

        let nb_items: usize = values[0].parse().unwrap();
        let from: usize = values[1].parse().unwrap();
        let to: usize = values[2].parse().unwrap();

        /*
        // V1
        for _i in 0..nb_items {
            let stack_from = &mut ship[from - 1];
            let item = stack_from.pop().unwrap();

            let stack_to = &mut ship[to - 1];
            stack_to.push(item);
        }
        println!();
        */

        // V2
        let stack_from = &mut ship[from - 1];
        let mut items = stack_from.split_off(stack_from.len() - nb_items);

        let stack_to = &mut ship[to - 1];
        stack_to.append(&mut items);

        println!("move {} from {} to {}", nb_items, from, to);
    }
}
