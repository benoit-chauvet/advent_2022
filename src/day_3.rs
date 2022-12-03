use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day3() {
    day3_2();
}

fn day3_2() {
    let path = "files/day_3.txt";

    let f = File::open(path).expect("File not found :/");
    let mut reader = BufReader::new(f);

    let mut badges: Vec<char> = Vec::new();

    loop {
        let mut line1 = String::new();
        let mut _bytes = reader.read_line(&mut line1);

        if line1 == "" {
            break;
        }

        let mut line2 = String::new();
        _bytes = reader.read_line(&mut line2);

        let mut line3 = String::new();
        _bytes = reader.read_line(&mut line3);

        let mut common_items: Vec<char> = Vec::new();

        let chars1: Vec<_> = line1.trim().chars().collect();
        // identify common items between 1 & 2
        for c1 in chars1 {
            let chars2: Vec<_> = line2.trim().chars().collect();
            for c2 in chars2 {
                if c1 == c2 {
                    if !common_items.contains(&c1) {
                        common_items.push(c1);
                    }
                }
            }
        }

        let chars3: Vec<_> = line3.trim().chars().collect();
        // identify common item between 1/2 & 3
        for c3 in chars3 {
            if common_items.contains(&c3) {
                badges.push(c3);
                break;
            }
        }
    }

    // calculate the sum :
    println!("TOTAL: {}", count_priority(badges));
}

fn day3_1() {
    let path = "files/day_3.txt";

    let f = File::open(path).expect("File not found :/");
    let mut reader = BufReader::new(f);

    let mut total = 0;

    let mut common_items: Vec<char> = Vec::new();

    loop {
        let mut line = String::new();
        let _bytes = reader.read_line(&mut line);

        if line == "" {
            break;
        }

        let chars: Vec<_> = line.trim().chars().collect();

        //println!("{}", chars.count());

        let size = line.trim().len() / 2;

        let mut carry_on = true;
        for i in 0..size {
            let c1 = chars[i];
            for j in size..size * 2 {
                let c2 = chars[j];
                if c1 == c2 {
                    common_items.push(c1);
                    carry_on = false;
                    break;
                }
            }
            if carry_on == false {
                break;
            }
        }
    }

    /*
    let mut total = 0;

    for c in common_items {
        print!("{}", c);

        let mut priority: u32 = c as u32;
        if priority > 97 {
            priority = priority - 96;
        } else {
            priority = priority - 38;
        }

        print!("{}", priority);

        total += priority;
    }
    println!("");
    */

    println!("TOTAL: {}", count_priority(common_items));
}

fn count_priority(coll: Vec<char>) -> u32 {
    let mut total = 0;

    for c in coll {
        print!("{}", c);

        let mut priority: u32 = c as u32;
        if priority > 97 {
            priority = priority - 96;
        } else {
            priority = priority - 38;
        }

        print!("{}", priority);

        total += priority;
    }

    return total;
}
