use crate::file_utils;
use std::cmp;

type tuple = (usize, usize);

pub fn day12() {
    let path = String::from("files/day_12.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(path);

    let height = lines[0].trim().len();
    let width = lines.len();

    //println!("w:{} h:{}", width, height);

    let mut map : Vec<Vec<char>> = Vec::new();
    for line in lines{
        map.push(line.trim().chars().collect());
    }

    // identify position of E
    let target = find_finish_position(&map);
    println!("E : {} {}", target.0, target.1);
   
    let mut visited :Vec<tuple> = Vec::new();

    let mut current = (0,0);

    loop{

        visited.push(current);
        
        let top = get_top(current, &visited);
        let bottom = get_bottom(current, height, &visited);
        
        let left = get_left(current, &visited);
        let right = get_right(current, width, &visited);

        let mut candidates : Vec<tuple> = Vec::new();

        // existing, not visited yet and allowed elevation 
        if is_valid_candidate(top, current, &map){
            candidates.push(top.unwrap());
        }
        if is_valid_candidate(bottom, current, &map){
            candidates.push(bottom.unwrap());
        }
        if is_valid_candidate(left, current, &map){
            candidates.push(left.unwrap());
        }
        if is_valid_candidate(right, current, &map){
            candidates.push(right.unwrap());
        }

        if candidates.is_empty(){
            println!("NO MORE CANDIDATES");
            break;
        }     

        let mut min = 999;
        for point in candidates{
            println!("candidate : {} {}", point.0, point.1);

            let pt_min = min_distance(point, target);
            if  pt_min < min{
                min = pt_min;
            }          
        }

        break;
        // rules :
           
            //  - closest to E (horizontally or vertically)    
            //  - higher step
            // >>>> SORT ON distance, then altitude

        if current == target {
            break;
        }
    }

    println!("found !");
}

fn is_valid_candidate(opt_dest:Option<tuple>, src:tuple, map:&Vec<Vec<char>>) -> bool {
    if ! opt_dest.is_none(){
        let dest = opt_dest.unwrap();
        let diff = map[dest.0][dest.1] as u32 - map[src.0][src.1] as u32;
        if diff <= 1{
            return true;
        }
    }
    return false;
}

fn min_distance(a:tuple, b:tuple)-> usize{
    let x = a.0.abs_diff(b.0);
    let y = a.1.abs_diff(b.1);

    return cmp::min(x, y);
}

fn get_top(point:tuple, visited :&Vec<tuple>)-> Option<tuple>{
    if point.1 > 0{
        let result = (point.0, point.1 - 1);
        if visited.contains(&result){
            return None;
        }
        return Some(result);
    }
    else
    {
        return None;
    }
}

fn get_bottom(point:tuple, max:usize, visited :&Vec<tuple>)-> Option<tuple>{
    if point.1 < max - 1 {
        let result = (point.0, point.1 + 1);
        if visited.contains(&result){
            return None;
        }
        return Some(result);
    }
    else
    {
        return None;
    }
}

fn get_left(point:tuple, visited :&Vec<tuple>)-> Option<tuple>{
    if point.0 > 0{
        let result = (point.0 - 1, point.1);
        if visited.contains(&result){
            return None;
        }
        return Some(result);
    }
    else
    {
        return None;
    }
}

fn get_right(point:tuple, max:usize, visited :&Vec<tuple>)-> Option<tuple>{
    if point.0 < max - 1 {
        let result = (point.0 + 1, point.1);
        if visited.contains(&result){
            return None;
        }
        return Some(result);
    }
    else
    {
        return None;
    }
}


fn find_finish_position(map:&Vec<Vec<char>>) -> tuple{
    for i in 0..map.len(){
        let line = &map[i]; 
        for j in 0..line.len(){
            if line[j] == 'E'{
                return (i,j);
            }
        }
    }
    (0,0)
}
