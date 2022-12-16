use std::collections::{HashMap, VecDeque};

use crate::file_utils;

type Tuple = (usize, usize);

pub fn day12() {
    let file_path = String::from("files/day_12.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(file_path);

    let height = lines[0].trim().len();
    let width = lines.len();

    //build the map :
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        map.push(line.trim().chars().collect());
    }

    let mut nodes: HashMap<Tuple, Node> = HashMap::new();
    let mut distances: HashMap<Tuple, u32> = HashMap::new();
    let mut visit_queue: VecDeque<Tuple> = VecDeque::new();

    // find the starting position
    let start = find_position(&map, 'S');

    // find the ending position
    let end = find_position(&map, 'E');

    println!("end : {} {}", end.0, end.1);

    map[end.0][end.1] = 'z';
    map[start.0][start.1] = 'a';

    // build the tree:
    for i in 0..width {
        for j in 0..height {
            let point = (i, j);

            let node = Node {
                point,
                children: find_children(&map, point, width, height),
            };

            nodes.insert(point, node);
        }
    }

    visit_queue.push_back(start);
    visit(&nodes, &mut distances, &mut visit_queue, 0);

    println!("done: {}", distances.get(&end).unwrap());
}

fn visit(
    nodes: &HashMap<Tuple, Node>,
    distances: &mut HashMap<Tuple, u32>,
    visit_queue: &mut VecDeque<Tuple>,
    mut depth: u32,
) {
    depth = depth + 1;

    let node = nodes.get(visit_queue.back().unwrap()).unwrap();
    for child in &node.children {
        println!(
            "p : {}-{}  ch:{}-{}",
            node.point.0, node.point.1, child.0, child.1
        );
        if !distances.contains_key(&child) {
            distances.insert(*child, depth);
            visit_queue.push_back(*child);
            visit(&nodes, distances, visit_queue, depth);
            //visit_queue.pop_front();
        }
    }
}

struct Node {
    point: Tuple,
    children: Vec<Tuple>,
}

fn find_children(map: &Vec<Vec<char>>, point: Tuple, width: usize, height: usize) -> Vec<Tuple> {
    let mut result: Vec<Tuple> = Vec::new();

    let top = get_top(point);
    let bottom = get_bottom(point, height);
    let left = get_left(point);
    let right = get_right(point, width);

    if is_valid_candidate(top, point, map) {
        result.push(top.unwrap());
    }
    if is_valid_candidate(bottom, point, map) {
        result.push(bottom.unwrap());
    }
    if is_valid_candidate(left, point, map) {
        result.push(left.unwrap());
    }
    if is_valid_candidate(right, point, map) {
        result.push(right.unwrap());
    }

    return result;
}

fn is_valid_candidate(opt_dest: Option<Tuple>, src: Tuple, map: &Vec<Vec<char>>) -> bool {
    if !opt_dest.is_none() {
        let dest = opt_dest.unwrap();

        let diff = map[dest.0][dest.1] as i32 - map[src.0][src.1] as i32;

        if diff <= 1 {
            return true;
        }
    }
    return false;
}

fn find_position(map: &Vec<Vec<char>>, value: char) -> Tuple {
    for i in 0..map.len() {
        let line = &map[i];
        for j in 0..line.len() {
            if line[j] == value {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn get_top(point: Tuple) -> Option<Tuple> {
    if point.1 > 0 {
        let result = (point.0, point.1 - 1);
        return Some(result);
    } else {
        return None;
    }
}

fn get_bottom(point: Tuple, max: usize) -> Option<Tuple> {
    if point.1 < max - 1 {
        let result = (point.0, point.1 + 1);
        return Some(result);
    } else {
        return None;
    }
}

fn get_left(point: Tuple) -> Option<Tuple> {
    if point.0 > 0 {
        let result = (point.0 - 1, point.1);
        return Some(result);
    } else {
        return None;
    }
}

fn get_right(point: Tuple, max: usize) -> Option<Tuple> {
    if point.0 < max - 1 {
        let result = (point.0 + 1, point.1);
        return Some(result);
    } else {
        return None;
    }
}