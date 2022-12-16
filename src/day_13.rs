use crate::file_utils;
use json::JsonValue;
use std::cmp::Ordering;

pub fn day13() {
    part1();
    part2();
}

pub fn part2() {
    let file_path = String::from("files/day_13.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(file_path);

    let mut inputs: Vec<Item> = Vec::new();

    for i in (0..lines.len()).step_by(3) {
        inputs.push(Item{value:json::parse(&lines[i]).unwrap(), line:String::from(&lines[i])});
        inputs.push(Item{value:json::parse(&lines[i + 1]).unwrap(), line:String::from(&lines[i+1])});
    }

    inputs.sort();

    let mut index = 0;
    for i in inputs{
        index = index + 1;
        if is_divider(i.value){
            println!("{} - {}", i.line, index);
        }
    }
    
}

fn is_divider(val:JsonValue) -> bool{
    
    return val.len() ==1 && val[0].is_array() && val[0].len() == 1&&  val[0][0].is_number();
}

#[derive(Eq)]
struct Item{
    value:JsonValue,
    line:String,
}

impl Ord for Item{
    fn cmp(&self, other: &Self) -> Ordering {
        match compare(&self.value, &other.value){
            ComparisonResult::Ok => return Ordering::Less,
            ComparisonResult::Error => return Ordering::Greater,
            _ => return Ordering::Equal,
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub fn part1() {
    let file_path = String::from("files/day_13.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(file_path);

    let mut right_indexes: Vec<usize> = Vec::new();
    let mut index: usize = 0;

    for i in (0..lines.len()).step_by(3) {
        index = index + 1;

        let left_input = json::parse(&lines[i]).unwrap();
        let right_input = json::parse(&lines[i + 1]).unwrap();

        let result = compare(&left_input, &right_input);

        //println!("{:?} {}", result, index);

        if result == ComparisonResult::Ok {
            right_indexes.push(index);
        }
    }

    let mut total = 0;
    for i in right_indexes {
        //println!("{}", i);
        total += i;
    }

    println!("TOTAL: {}", total);
}

fn compare(left: &JsonValue, right: &JsonValue) -> ComparisonResult {
    if left.is_number() && right.is_number() {
        return compare_numbers(left, right);
    } else if left.is_array() && right.is_array() {
        return compare_arrays(left, right);
    } else if left.is_array() && right.is_number() {
        let r = right.as_u32();
        let mut arr = Vec::<JsonValue>::new();
        arr.push(r.into());
        return compare_arrays(left, &JsonValue::Array(arr));
    } else if left.is_number() && right.is_array() {
        let l = left.as_u32();
        let mut arr = Vec::<JsonValue>::new();
        arr.push(l.into());
        return compare_arrays(&JsonValue::Array(arr), right);
    } else if right.is_empty() {
        return ComparisonResult::Error;
    } else {
        println!("?? edge case ??");
        return ComparisonResult::Error;
    }
}

fn compare_arrays(left: &JsonValue, right: &JsonValue) -> ComparisonResult {
    if left.len() == 0 && right.len() == 0 {
        return ComparisonResult::Continue;
    }

    let mut carry_on = false;

    for i in 0..left.len() {
        // finished right before left : error :
        if right.len() < i {
            return ComparisonResult::Error;
        }

        let result = compare(&left[i], &right[i]);

        if result != ComparisonResult::Continue {
            return result;
        }
        carry_on = true;
    }

    if carry_on && left.len() == right.len() {
        return ComparisonResult::Continue;
    }

    // finished left before right : Ok :
    return ComparisonResult::Ok;
}

fn compare_numbers(left: &JsonValue, right: &JsonValue) -> ComparisonResult {
    let l = left.as_u32().unwrap();
    let r = right.as_u32().unwrap();

    // println!("{} {}", l, r);

    if l < r {
        return ComparisonResult::Ok;
    }

    if l > r {
        return ComparisonResult::Error;
    }

    return ComparisonResult::Continue;
}

#[derive(Debug, PartialEq, Eq)]
enum ComparisonResult {
    Continue,
    Ok,
    Error,
    //Unhandled,
}
