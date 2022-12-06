use std::cmp::max;
use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut result = 0;
    for end in 4..input.len() {
        let start = max(0, end - 4);
        let mut set = HashSet::new();
        for c in input[start..end].chars() {
            set.insert(c);
        }
        if set.len() == 4 {
            result = end as i32;
            break;
        }
    }
    result
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut result = 0;
    for end in 14..input.len() {
        let start = max(0, end - 14);
        let mut set = HashSet::new();
        for c in input[start..end].chars() {
            set.insert(c);
        }
        if set.len() == 14 {
            result = end as i32;
            break;
        }
    }
    result
}
