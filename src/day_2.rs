use std::fs::File;
use std::io::{BufRead, BufReader};

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSOR: i32 = 3;

pub fn day2() {
    let path = "files/day_2.txt";

    let f = File::open(path).expect("File not found :/");
    let mut reader = BufReader::new(f);

    let mut total = 0;

    loop {
        let mut line = String::new();
        let _bytes = reader.read_line(&mut line);

        if line == "" {
            break;
        }

        let choices: Vec<&str> = line.split(" ").collect();

        let str_opponent = choices[0];
        let str_you = choices[1];

        //println!("{0} {1}", choices[0], choices[1]);
        println!("{0}-{1}", str_opponent, str_you);

        let mut opponent = 0;
        let mut you = 0;

        match str_opponent.trim() {
            "A" => opponent = ROCK,
            "B" => opponent = PAPER,
            "C" => opponent = SCISSOR,
            _ => opponent = -1,
        }

        //you = determine_you_v1(str_you);
        you = determine_you_v2(str_you, opponent);

        total += calculate_round(opponent, you);
    }

    println!("{0}", total);
}

fn determine_you_v2(letter: &str, opponent: i32) -> i32 {
    match letter.trim() {
        "X" => return calc_loose(opponent),
        "Y" => return calc_draw(opponent),
        "Z" => return calc_win(opponent),
        _ => return -1,
    }
}

fn calc_loose(opponent: i32) -> i32 {
    match opponent {
        ROCK => return SCISSOR,
        PAPER => return ROCK,
        SCISSOR => return PAPER,
        _ => return -1,
    }
}

fn calc_draw(opponent: i32) -> i32 {
    match opponent {
        ROCK => return ROCK,
        PAPER => return PAPER,
        SCISSOR => return SCISSOR,
        _ => return -1,
    }
}

fn calc_win(opponent: i32) -> i32 {
    match opponent {
        ROCK => return PAPER,
        PAPER => return SCISSOR,
        SCISSOR => return ROCK,
        _ => return -1,
    }
}

fn determine_you_v1(letter: &str) -> i32 {
    match letter.trim() {
        "X" => return ROCK,
        "Y" => return PAPER,
        "Z" => return SCISSOR,
        _ => return -1,
    }
}

fn calculate_round(opponent: i32, you: i32) -> i32 {
    let mut result = 0;

    if opponent == you {
        result = DRAW;
    } else if opponent == ROCK {
        if (you == PAPER) {
            result = WIN;
        } else {
            result = LOSS;
        }
    } else if opponent == PAPER {
        if (you == SCISSOR) {
            result = WIN;
        } else {
            result = LOSS;
        }
    } else if opponent == SCISSOR {
        if (you == ROCK) {
            result = WIN;
        } else {
            result = LOSS;
        }
    }

    return result + you;
}
