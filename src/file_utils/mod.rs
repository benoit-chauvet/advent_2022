use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_lines_reader(path: String) -> Vec<String> {
    let f = File::open(path).expect("File not found :/");
    let mut reader = BufReader::new(f);

    let mut lines: Vec<String> = Vec::new();

    loop {
        let mut line: String = String::new();
        let mut _bytes = reader.read_line(&mut line);

        if line == "" {
            break;
        }

        lines.push(line.trim().to_string());
    }

    return lines;
}
