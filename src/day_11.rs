use num::BigUint;

use crate::file_utils;

const NB_MONKEYS: usize = 4;

pub fn day11() {
    let path = String::from("files/day_11t.txt");
    let mut lines: Vec<String> = file_utils::get_lines_reader(path);

    let mut monkeys: Vec<Monkey> = Vec::new();
    let nb_rounds = 20;

    for i in (0..lines.len()).step_by(7) {
        monkeys.push(parse_monkey(&mut lines[i..i + 7]));
    }

    let mut inspections: [i32; NB_MONKEYS] = [0; NB_MONKEYS];

    for _i in 0..nb_rounds {
        let mut transfers: [Vec<BigUint>; NB_MONKEYS] = Default::default();

        for m in 0..NB_MONKEYS {
            {
                let monkey = &mut monkeys[m];
                for item in &monkey.items {
                    let worry = monkey.operation.get_worry_lvl(item.clone());

                    let dest = monkey.test.get_dest(worry.clone()) as usize;

                    transfers[dest].push(worry.clone());
                    inspections[m] += 1;
                }
                monkey.items = Vec::new();
            }

            for t in 0..NB_MONKEYS {
                for item in &transfers[t] {
                    let mdest = &mut monkeys[t];
                    mdest.items.push(item.clone());
                }
                transfers[t] = Vec::new();
            }
        }
    }

    // for monkey in monkeys {
    //     monkey.show();
    // }

    for inspect in inspections {
        println!("{}", inspect)
    }
}

fn parse_monkey(lines: &mut [String]) -> Monkey {
    Monkey {
        items: get_items(&mut lines[1]),
        operation: get_operation(&mut lines[2]),
        test: get_test(&mut lines[3..6]),
    }
}

fn get_items(line: &mut String) -> Vec<BigUint> {
    let mut items: Vec<BigUint> = Vec::new();

    *line = line.replace("Starting items: ", "");

    let values: Vec<&str> = line.split(',').collect();

    for value in values {
        items.push(value.trim().parse().unwrap());
    }
    return items;
}

fn get_operation(line: &mut String) -> Operation {
    *line = line.replace("Operation: new = ", "");

    let values: Vec<&str> = line.split(' ').collect();

    Operation {
        a: get_operand(values[0]),
        b: get_operand(values[2]),
        operator: values[1].chars().next().unwrap(),
    }
}

fn get_operand(value: &str) -> u32 {
    if value == "old" {
        return 0;
    } else {
        return value.parse().unwrap();
    }
}

fn get_test(lines: &mut [String]) -> Test {
    Test {
        value: lines[0]
            .replace("Test: divisible by", "")
            .trim()
            .parse()
            .unwrap(),
        dest_if_true: lines[1]
            .replace("If true: throw to monkey", "")
            .trim()
            .parse()
            .unwrap(),
        dest_if_false: lines[2]
            .replace("If false: throw to monkey ", "")
            .trim()
            .parse()
            .unwrap(),
    }
}

struct Test {
    value: u128,
    dest_if_true: u32,
    dest_if_false: u32,
}

impl Test {
    fn get_dest(&self, worry: BigUint) -> u32 {
        if worry % self.value == BigUint::from(0u32) {
            return self.dest_if_true;
        } else {
            return self.dest_if_false;
        }
    }
}

struct Operation {
    a: u32,
    b: u32,
    operator: char,
}

impl Operation {
    fn get_worry_lvl(&self, value: BigUint) -> BigUint {
        let a: BigUint = if self.a == 0 {
            value.clone()
        } else {
            BigUint::from(self.a)
        };
        let b: BigUint = if self.b == 0 {
            value.clone()
        } else {
            BigUint::from(self.b)
        };

        let mut result: BigUint = BigUint::from(0u32);

        match self.operator {
            '+' => result = a + b,
            '-' => result = a - b,
            '*' => result = a * b,
            '/' => result = a / b,
            _ => result = BigUint::from(0u32),
        }

        return result / BigUint::from(3u32);
    }
}

struct Monkey {
    items: Vec<BigUint>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn show(&self) {
        print!("items : ");
        for item in &self.items {
            print!("{} ", item);
        }
        println!();

        // println!(
        //     "operation : {} {} {}",
        //     &self.operation.a, &self.operation.operator, &self.operation.b
        // );

        // print!(
        //     "div by {} ? to {} : to {}",
        //     &self.test.value, &self.test.dest_if_true, &self.test.dest_if_false
        // );

        // println!();
    }
}
