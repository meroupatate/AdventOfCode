#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut current_x = 1;
    let mut current_cycle = 0;
    let mut current_signal_strength = 0;
    for operation in input.lines() {
        if operation.starts_with("addx") {
            let mut s = operation.split(' ');
            let value: i32 = s.next_back().unwrap().parse().unwrap();
            // first addx cycle
            (current_signal_strength, current_cycle) =
                part1_new_cycle(current_signal_strength, current_cycle, current_x);
            // second addx cycle
            (current_signal_strength, current_cycle) =
                part1_new_cycle(current_signal_strength, current_cycle, current_x);
            // addx
            current_x += value;
        } else {
            // noop cycle
            (current_signal_strength, current_cycle) =
                part1_new_cycle(current_signal_strength, current_cycle, current_x);
        }
    }
    current_signal_strength
}

fn part1_new_cycle(current_signal_strength: i32, current_cycle: i32, current_x: i32) -> (i32, i32) {
    let new_cycle = current_cycle + 1;
    let mut new_signal_strength = current_signal_strength;
    if new_cycle == 20 || new_cycle % 40 == 20 {
        new_signal_strength += new_cycle * current_x;
    }
    (new_signal_strength, new_cycle)
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut current_x = 1;
    let mut current_cycle = 0;
    let mut current_crt = "".to_string();
    for operation in input.lines() {
        if operation.starts_with("addx") {
            let mut s = operation.split(' ');
            let value: i32 = s.next_back().unwrap().parse().unwrap();
            // first addx cycle
            current_crt = part2_write_crt(&current_crt, current_cycle, current_x);
            (current_crt, current_cycle) = part2_new_cycle(&current_crt, current_cycle);
            // second addx cycle
            current_crt = part2_write_crt(&current_crt, current_cycle, current_x);
            (current_crt, current_cycle) = part2_new_cycle(&current_crt, current_cycle);
            // addx
            current_x += value;
        } else {
            // noop cycle
            current_crt = part2_write_crt(&current_crt, current_cycle, current_x);
            (current_crt, current_cycle) = part2_new_cycle(&current_crt, current_cycle);
        }
    }
    println!("{}", current_crt);
    0
}

fn part2_write_crt(crt: &String, current_cycle: i32, current_x: i32) -> String {
    let mut pixel = ".";
    if (current_cycle % 40) == ((current_x - 1) % 40)
        || (current_cycle % 40) == ((current_x + 1) % 40)
        || (current_cycle % 40) == (current_x % 40)
    {
        pixel = "#";
    }
    crt.clone() + pixel
}

fn part2_new_cycle(crt: &String, current_cycle: i32) -> (String, i32) {
    let new_cycle = current_cycle + 1;
    let mut new_crt = crt.clone();
    if new_cycle % 40 == 0 {
        new_crt += "\n"
    }
    (new_crt, new_cycle)
}
