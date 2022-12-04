use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "files/2015/day1.txt";

    let f = File::open(path).expect("File not found :/");
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let mut _bytes = reader.read_line(&mut line);

    let size: i32 = line.trim().len().try_into().unwrap();
    let mut nb_down = 0;

    let mut basement: i32 = 0;
    let mut i = 0;
    let mut first_entrance = true;

    for c in line.chars() {
        i += 1;

        if c == ')' {
            basement -= 1;
            nb_down += 1;
        } else {
            basement += 1;
        }

        if basement < 0 && first_entrance {
            println!("BASEMENT ! {}", i);
            first_entrance = false;
        }
    }

    let nb_up = size - nb_down;

    println!("{}", size - (nb_down * 2));
}
