use regex::Regex;

#[derive(Clone, Debug)]
pub struct Monkey {
    id: i32,
    items: Vec<i64>,
    operation: char,
    value: i32,
    divisible_by: i64,
    if_true: i32,
    if_false: i32,
    inspection_count: i64,
}

impl Monkey {
    fn catch_item(&mut self, item: i64) {
        self.items.push(item);
    }
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut lines = input.lines();
    while let (
        Some(monkey_id_line),
        Some(starting_items_line),
        Some(operation_line),
        Some(divisible_by_line),
        Some(if_true_line),
        Some(if_false_line),
    ) = (
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
    ) {
        let re_monkey_id = Regex::new(r"Monkey (\d)").unwrap();
        let caps_monkey_id = re_monkey_id.captures(monkey_id_line).unwrap();
        let id = caps_monkey_id
            .get(1)
            .map_or(0, |m| m.as_str().parse().unwrap());

        let re_starting_items = Regex::new(r"Starting items: (.+)$").unwrap();
        let caps_starting_items = re_starting_items.captures(starting_items_line).unwrap();
        let items = caps_starting_items.get(1).map_or(vec![], |m| {
            m.as_str()
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        });

        let re_operation = Regex::new(r"Operation: new = old ([+*]) (.+)$").unwrap();
        let caps_operation = re_operation.captures(operation_line).unwrap();
        let operation = caps_operation
            .get(1)
            .map_or(' ', |m| m.as_str().parse().unwrap());
        let value: i32 = caps_operation.get(2).map_or(0, |m| {
            if m.as_str() == "old" {
                -1
            } else {
                m.as_str().parse().unwrap()
            }
        });

        let re_divisible_by = Regex::new(r"Test: divisible by (\d+)$").unwrap();
        let caps_divisible_by = re_divisible_by.captures(divisible_by_line).unwrap();
        let divisible_by = caps_divisible_by
            .get(1)
            .map_or(0, |m| m.as_str().parse().unwrap());

        let re_if_true = Regex::new(r"throw to monkey (\d+)$").unwrap();
        let caps_if_true = re_if_true.captures(if_true_line).unwrap();
        let if_true = caps_if_true
            .get(1)
            .map_or(0, |m| m.as_str().parse().unwrap());

        let re_if_false = Regex::new(r"throw to monkey (\d+)$").unwrap();
        let caps_if_false = re_if_false.captures(if_false_line).unwrap();
        let if_false = caps_if_false
            .get(1)
            .map_or(0, |m| m.as_str().parse().unwrap());

        monkeys.push(Monkey {
            id,
            items,
            operation,
            value,
            divisible_by,
            if_true,
            if_false,
            inspection_count: 0,
        });
        lines.next();
    }

    monkeys
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<Monkey>) -> i64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in input {
        monkeys.push(monkey.clone());
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in &monkey.items {
                let updated_item = update_worry_level(item, monkey.operation, monkey.value);
                let round_worry_level = (updated_item / 3) as i64;
                if round_worry_level % monkey.divisible_by == 0 {
                    monkeys[monkey.if_true as usize].catch_item(round_worry_level);
                } else {
                    monkeys[monkey.if_false as usize].catch_item(round_worry_level);
                }
            }
            monkeys[i] = update_monkey_after_items_inspection(&monkey)
        }
    }

    get_monkey_business(&monkeys)
}

fn update_worry_level(item: &i64, operation: char, value: i32) -> i64 {
    let mut val = value as i64;
    if value == -1 {
        // value == old
        val = *item;
    }
    match operation {
        '+' => item + val,
        '*' => item * val,
        _ => 0,
    }
}

fn update_monkey_after_items_inspection(monkey: &Monkey) -> Monkey {
    Monkey {
        id: monkey.id,
        items: vec![],
        operation: monkey.operation,
        value: monkey.value,
        divisible_by: monkey.divisible_by,
        if_true: monkey.if_true,
        if_false: monkey.if_false,
        inspection_count: monkey.inspection_count + monkey.items.len() as i64,
    }
}

fn get_monkey_business(monkeys: &[Monkey]) -> i64 {
    let mut top_1_inspection_count = 0;
    let mut top_2_inspection_count = 0;
    for monkey in monkeys {
        if monkey.inspection_count > top_1_inspection_count {
            top_2_inspection_count = top_1_inspection_count;
            top_1_inspection_count = monkey.inspection_count;
        } else if monkey.inspection_count > top_2_inspection_count {
            top_2_inspection_count = monkey.inspection_count;
        }
    }
    top_1_inspection_count * top_2_inspection_count
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<Monkey>) -> i64 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut before_throw_should_divide_by = 1;
    for monkey in input {
        monkeys.push(monkey.clone());
        before_throw_should_divide_by *= monkey.divisible_by;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();
            for item in &monkey.items {
                let updated_item = update_worry_level(item, monkey.operation, monkey.value);
                let divided_item = updated_item % before_throw_should_divide_by;
                if updated_item % monkey.divisible_by == 0 {
                    monkeys[monkey.if_true as usize].catch_item(divided_item);
                } else {
                    monkeys[monkey.if_false as usize].catch_item(divided_item);
                }
            }
            monkeys[i] = update_monkey_after_items_inspection(&monkey)
        }
    }

    get_monkey_business(&monkeys)
}
