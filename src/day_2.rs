use std::io::{BufReader, BufRead};
use std::fs::File;

const loss : i32 = 0;
const draw : i32 = 3;
const win : i32 = 6;

const rock : i32 = 1;
const paper : i32 = 2;
const scissor : i32 = 3;

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

        let mut opponent=0;
        let mut you=0;

        match str_opponent.trim(){
            "A"=> opponent = rock,
            "B"=> opponent = paper,
            "C"=> opponent = scissor,
            _=>opponent = -1
        }

        //you = determine_you_v1(str_you);
        you = determine_you_v2(str_you, opponent);

        total += calculate_round(opponent, you);

    }

    println!("{0}", total);

}

fn determine_you_v2(letter:&str, opponent:i32) ->  i32 {
    match letter.trim(){
        "X"=> return calc_loose(opponent),
        "Y"=> return calc_draw(opponent),
        "Z"=> return calc_win(opponent),
        _=> return -1
    }
}

fn calc_loose(opponent:i32) -> i32 {
    match opponent{
        rock=> return scissor,
        paper=> return rock,
        scissor=> return paper,
        _ => return -1
    }
}

fn calc_draw(opponent:i32) -> i32 {
    match opponent{
        rock=> return rock,
        paper=> return paper,
        scissor=> return scissor,
        _ => return -1
    }
}

fn calc_win(opponent:i32) -> i32 {
    match opponent{
        rock=> return paper,
        paper=> return scissor,
        scissor=> return rock,
        _ => return -1
    }
}


fn determine_you_v1(letter:&str) ->  i32 {
    match letter.trim(){
        "X"=> return rock,
        "Y"=> return paper,
        "Z"=> return scissor,
        _=> return -1
    }
}

fn calculate_round(opponent:i32, you:i32) -> i32 {

    let mut result = 0;
    
    if opponent == you {
        result = draw;
    }
    else if  opponent == rock {
        if (you == paper) {
            result = win;
        }
        else{
            result = loss;
        }
    }
    else if  opponent == paper {
        if (you == scissor) {
            result = win;
        }
        else{
            result = loss;
        }
    }
    else if  opponent == scissor {
        if (you == rock) {
            result = win;
        }
        else{
            result = loss;
        }
    }
    
    return result + you;   
}