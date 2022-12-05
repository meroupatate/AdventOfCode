use regex::Regex;
use std::collections::VecDeque;

pub struct Action {
    number: usize,
    from: usize,
    to: usize,
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<Action>) {
    const SIZE: i32 = 9;
    let lines = input.lines();
    let mut actions: Vec<Action> = Vec::new();
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..SIZE {
        stacks.push(VecDeque::new());
    }

    for line in lines {
        if line.starts_with("move") {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            let captured = re.captures(line).unwrap();
            actions.push(Action {
                number: captured.get(1).unwrap().as_str().parse().unwrap(),
                from: captured.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                to: captured.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            });
        } else if line.starts_with("[") {
            for i in 0..SIZE {
                let c = line.chars().nth((i * 4 + 1) as usize).unwrap();
                if c != ' ' {
                    stacks[i as usize].push_front(c)
                }
            }
        }
    }
    (stacks, actions)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Vec<VecDeque<char>>, Vec<Action>)) -> String {
    let (stacks, actions) = input;
    let mut current_stacks = stacks.clone();
    for action in actions.iter() {
        for _ in 0..action.number {
            let mut new_stacks: Vec<VecDeque<char>> = Vec::new();
            let from_stack = current_stacks.get(action.from).unwrap();
            let moved = from_stack.back().unwrap();
            for (i, stack) in current_stacks.iter().enumerate() {
                if i == action.from {
                    let mut new_from_stack: VecDeque<char> = stack.clone();
                    new_from_stack.pop_back();
                    new_stacks.push(new_from_stack);
                } else if i == action.to {
                    let mut new_to_stack: VecDeque<char> = stack.clone();
                    new_to_stack.push_back(*moved);
                    new_stacks.push(new_to_stack);
                } else {
                    new_stacks.push(stack.clone())
                }
            }
            current_stacks = new_stacks.clone();
        }
    }

    let mut result: String = String::new();
    for stack in current_stacks {
        result.push_str(&stack.back().unwrap().to_string());
    }
    result
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &(Vec<VecDeque<char>>, Vec<Action>)) -> String {
    let (stacks, actions) = input;
    let mut current_stacks = stacks.clone();
    for action in actions.iter() {
        let mut new_stacks: Vec<VecDeque<char>> = Vec::new();
        let from_stack = current_stacks.get(action.from).unwrap();
        let mut moved: VecDeque<char> = VecDeque::new();
        let mut from_stack_iter = from_stack.iter();
        for _ in 0..action.number {
            moved.push_front(*from_stack_iter.next_back().unwrap());
        }
        println!("{:?}", moved);
        for (i, stack) in current_stacks.iter().enumerate() {
            if i == action.from {
                let mut new_from_stack: VecDeque<char> = stack.clone();
                new_from_stack.truncate(stack.len().saturating_sub(action.number));
                new_stacks.push(new_from_stack);
            } else if i == action.to {
                let mut new_to_stack: VecDeque<char> = stack.clone();
                new_to_stack.append(&mut moved);
                new_stacks.push(new_to_stack);
            } else {
                new_stacks.push(stack.clone())
            }
        }
        current_stacks = new_stacks.clone();
    }

    let mut result: String = String::new();
    for stack in current_stacks {
        result.push_str(&stack.back().unwrap().to_string());
    }
    result
}
