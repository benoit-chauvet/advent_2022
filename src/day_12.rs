use std::collections::HashMap;

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
    let mut visited: Vec<Tuple> = Vec::new();

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

            if node.point == start {
                distances.insert(node.point, 0);
            } else {
                distances.insert(node.point, u32::MAX);
            }

            nodes.insert(point, node);
        }
    }
    visit(nodes, &mut distances, &mut visited);

    for v in visited {
        println!("v: {} {} - d:{}", v.0, v.1, distances.get(&v).unwrap());
    }

    println!("done: {}", distances.get(&end).unwrap());
}

fn visit(
    nodes: HashMap<Tuple, Node>,
    distances: &mut HashMap<Tuple, u32>,
    visited: &mut Vec<Tuple>,
) {
    let mut min = u32::MAX;
    let mut node: Option<&Node> = None;

    // find the closest non visited node :
    for candidate in &mut nodes.values() {
        if distances.get(&candidate.point).unwrap() < &min && !visited.contains(&candidate.point) {
            node = Some(candidate);
            min = *distances.get(&candidate.point).unwrap();
        }
    }

    // for ch in &node.unwrap().children {
    //     println!("chi : {} {}", ch.0, ch.1,);
    // }

    // set its distances :
    if !node.is_none() {
        let n = node.unwrap();
        let new_distance = distances.get(&n.point).unwrap() + 1;
        for c in &n.children {
            if distances.get(&c).unwrap() > &new_distance {
                //distances.remove(c);
                distances.insert(*c, new_distance);
            }
        }
        visited.push(n.point);

        visit(nodes, distances, visited);
    } else {
        // Recursive exit condition : no more unvisited nodes
        return;
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
