use crate::file_utils;
use json::JsonValue;

//5563 : too low
// 5710: ko
//5819 : too high

pub fn day13() {
    part1();
    part2();
}

pub fn part2() {
    let file_path = String::from("files/day_13.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(file_path);

    let mut inputs: Vec<JsonValue> = Vec::new();

    for i in (0..lines.len()).step_by(3) {
        inputs.push(json::parse(&lines[i]).unwrap());
        inputs.push(json::parse(&lines[i + 1]).unwrap());
    }

    Sort(inputs);
}

pub fn Sort(inputs: Vec<JsonValue>) {}

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
