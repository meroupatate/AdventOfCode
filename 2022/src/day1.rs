#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut max: i32 = 0;
    let mut cur: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if cur > max {
                max = cur;
            }
            cur = 0;
        } else {
            let value: i32 = line.parse().unwrap();
            cur += value;
        }
    }
    if cur > max {
        max = cur
    }
    return max
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut top3: [i32; 3] = [0, 0, 0];
    let mut cur: i32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            if top3.len() < 3 {
                top3[top3.len()-1] = cur;
            } else if cur > top3[0] {
                top3[0] = cur;
                top3.sort();
            }
            cur = 0;
        } else {
            let value: i32 = line.parse().unwrap();
            cur += value;
        }
    }
    return top3.iter().sum()
}
