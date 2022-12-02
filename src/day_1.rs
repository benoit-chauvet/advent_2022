use std::io::{BufReader, BufRead};
use std::fs::File;

// 
pub fn day1() {
    
    println!("Hello, advent!");

    let path = "files/day_1.txt";

    let f = File::open(path).expect("File not found :/");
    let mut reader = BufReader::new(f);

    let mut max_load = 0;

    let mut current_load : i32 = 0;
        
    loop {
        let mut line = String::new();
        let _bytes = reader.read_line(&mut line);
        //println!("{}", line);
        if line != "\n" && line != ""{
            current_load += line.trim().parse::<i32>().unwrap();
        }
        else
        {
            if current_load > max_load {
                max_load = current_load;
                println!("{}", max_load);
            }
            current_load = 0;
        }

        if line==""
        {
            break;
        }
    }
}


pub fn day1_2() {
    
    println!("Hello, advent!");

    let path = "files/day_1.txt";

    let f = File::open(path).expect("File not found :/");
    let mut reader = BufReader::new(f);

    let mut top1 = 0;
    let mut top2 = 0;
    let mut top3 = 0;

    let mut current_load : i32 = 0;
        
    loop {
        let mut line = String::new();
        let _bytes = reader.read_line(&mut line);
        
        if line != "\n" && line != ""{
            current_load += line.trim().parse::<i32>().unwrap();
        }
        else
        {
            if current_load > top1 {
                top3 = top2;
                top2 = top1;
                top1 = current_load;
            }
            else if current_load > top2 {
                top3 = top2;
                top2 = current_load
            } else if current_load > top3 {
                top3 = current_load;
            }
            current_load = 0;        
        }  

        if line==""
        {
            break;
        }

    }

    println!("{}", top1);
    println!("{}", top2);
    println!("{}", top3);

    println!("total: {}", top1 + top2 + top3);

}


// read next elf's load