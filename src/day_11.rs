use crate::file_utils;

const NB_MONKEYS: usize = 8;
type Mval = u128;

pub fn day11() {
    let path = String::from("files/day_11.txt");
    let mut lines: Vec<String> = file_utils::get_lines_reader(path);

    let mut monkeys: Vec<Monkey> = Vec::new();
    let nb_rounds = 10000;

    for i in (0..lines.len()).step_by(7) {
        monkeys.push(parse_monkey(&mut lines[i..i + 7]));
    }

    let mut inspections: [i32; NB_MONKEYS] = [0; NB_MONKEYS];

    let modulo: u128 = monkeys.iter().map(|m| m.test.value).product();

    for _i in 0..nb_rounds {
        let mut transfers: [Vec<Mval>; NB_MONKEYS] = Default::default();

        for m in 0..NB_MONKEYS {
            {
                let monkey = &mut monkeys[m];
                for item in &monkey.items {
                    let worry = monkey.operation.get_worry_lvl(*item, modulo);

                    let dest = monkey.test.get_dest(worry) as usize;

                    transfers[dest].push(worry);
                    inspections[m] += 1;
                }
                monkey.items = Vec::new();
            }

            for t in 0..NB_MONKEYS {
                for item in &transfers[t] {
                    let mdest = &mut monkeys[t];
                    mdest.items.push(*item);
                }
                transfers[t] = Vec::new();
            }
        }
    }

    for monkey in monkeys {
        monkey.show();
    }

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

fn get_items(line: &mut String) -> Vec<Mval> {
    let mut items: Vec<Mval> = Vec::new();

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
    value: Mval,
    dest_if_true: u32,
    dest_if_false: u32,
}

impl Test {
    fn get_dest(&self, worry: Mval) -> u32 {
        if worry % self.value == 0 {
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
    fn get_worry_lvl(&self, value: Mval, modulo: u128) -> Mval {
        let a: Mval = if self.a == 0 { value } else { self.a as Mval };
        let b: Mval = if self.b == 0 { value } else { self.b as Mval };

        let mut result: Mval = 0;

        println!("a {} b {}", a, b);

        match self.operator {
            '+' => result = a + b,
            '-' => result = a - b,
            '*' => result = a * b,
            '/' => result = a / b,
            _ => result = 0,
        }

        return result % modulo;

        //return result; // / 3;
    }
}

struct Monkey {
    items: Vec<Mval>,
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
