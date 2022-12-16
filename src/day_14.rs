use crate::file_utils;
use std::cmp::*;

type Tuple  = (usize, usize);

pub fn day14() {

    let file_path = String::from("files/day_14.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(file_path);




}


fn build_cave(lines: &Vec<String>) -> Vec<Vec<char>> {

let mut cave: Vec<Vec<char>> = Vec::new();

let mut column_offset :usize = 500;
let mut cave_height:usize = 0;


cave.push(Vec::new()); // create column 500

for line in lines{

    let points: Vec<&str> = line.split("->").collect();

    for i in 1..points.len(){

        let start = parse_point(points[i-1]);
        let end = parse_point(points[i]);


        if start.0 != end.0{ // horizontal line

            let from_column = min(start.0, end.0);
            let to_column = max(start.0, end.0);

            let min_column = column_offset;
            let max_column = column_offset + cave.len();

            // add columns if extended width :
            // columns after
            
            // columns before

            // draw rocks on the target line :  

        }
        else {  // vertical line

            let lowest_y = max(start.1, end.1);

            // add lines if extended height :
            if lowest_y > cave_height{
                cave_height = lowest_y;
                let new_lines = lowest_y - cave_height;
                for column in &mut cave{
                    for _ in 0..new_lines{
                        column.push('.');
                    }
                }
            }

            // draw rocks in the target column :
            let column_index = start.0 - column_offset;

            let range = min(start.1,end.1) .. max(start.1,end.1);

            for y in range{
                cave[column_index][y] = '#';
            }

        }



    }

}


return cave;

}

fn parse_point(point:&str) -> Tuple{

    let values : Vec<&str> = point.split(",").collect();
    return (values[0].parse().unwrap(), values[1].parse().unwrap());
}