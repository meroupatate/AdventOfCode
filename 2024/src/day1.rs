use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut total = 0;

    for line in input.lines() {
        let mut line_iter = line.split_whitespace();
        left.push(line_iter.next().unwrap().parse().unwrap());
        right.push(line_iter.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();

    for (i, left_value) in left.iter().enumerate() {
        total += (left_value - right[i]).abs()
    }

    return total
}


#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut right_count: HashMap<i32, i32> = HashMap::new();
    let mut left: Vec<i32> = Vec::new();
    let mut similarity_score = 0;

    for line in input.lines() {
        let mut line_iter = line.split_whitespace();
        left.push(line_iter.next().unwrap().parse().unwrap());

        let right = line_iter.next().unwrap().parse().unwrap();
        let right_count = right_count.entry(right).or_insert(0);
        *right_count += 1;
    }

    for (right, count) in right_count.iter() {
        if left.contains(right) {
            similarity_score += right * count;
        }
    }

    return similarity_score;
}