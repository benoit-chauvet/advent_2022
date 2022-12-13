use crate::file_utils;
use json::JsonValue;

//5563 : too low

//5819 : too high

pub fn day13() {
    let file_path = String::from("files/day_13.txt");
    let lines: Vec<String> = file_utils::get_lines_reader(file_path);

    let mut right_indexes: Vec<usize> = Vec::new();
    let mut index: usize = 0;

    for i in (0..lines.len()).step_by(3) {
        index = index + 1;

        let left_input = json::parse(&lines[i]).unwrap();
        let right_input = json::parse(&lines[i + 1]).unwrap();

        let result = compare_arrays(&left_input, &right_input);

        //if result == ComparisonResult::Unhandled {
        println!("{:?} index {}", result, index);
        println!("left : {}", left_input);
        println!("right : {}", right_input,);
        //}

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

fn compare_arrays(left_arr: &JsonValue, right_arr: &JsonValue) -> ComparisonResult {
    let mut result = ComparisonResult::Unhandled;

    if left_arr.len() == 0 {
        return ComparisonResult::Ok;
    }

    for i in 0..left_arr.len() {
        let left = &left_arr[i];

        if right_arr.len() == 0 {
            return ComparisonResult::Error;
        }

        let right = &right_arr[i];

        // println!("left : {} - right : {}", left, right);

        if left.is_number() && right.is_number() {
            result = compare_numbers(left, right);
        }
        if left.is_array() && right.is_array() {
            result = compare_arrays(left, right);
        }
        if left.is_array() && right.is_number() {
            let r = right.as_u32();
            let mut arr = Vec::<JsonValue>::new();
            arr.push(r.into());
            result = compare_arrays(left, &JsonValue::Array(arr));
        }
        if left.is_number() && right.is_array() {
            let l = left.as_u32();
            let mut arr = Vec::<JsonValue>::new();
            arr.push(l.into());
            result = compare_arrays(&JsonValue::Array(arr), right);
        }

        if result != ComparisonResult::Continue {
            return result;
        }
    }

    if left_arr.len() > right_arr.len() {
        return ComparisonResult::Error;
    } else {
        return ComparisonResult::Ok;
    }

    //return result;
}

fn compare_numbers(left: &JsonValue, right: &JsonValue) -> ComparisonResult {
    let l = left.as_u32().unwrap();
    let r = right.as_u32().unwrap();

    //println!("compare numbers : {} {}", l, r);

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
    Unhandled,
}
