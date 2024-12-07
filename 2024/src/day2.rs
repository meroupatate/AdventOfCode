#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut safe_count: i32 = 0;

    for report in input.lines() {
        let levels: Vec<i32> = report.split_whitespace().map(|l|->i32{l.parse().unwrap()}).collect();
        if is_report_safe(&levels) {
            safe_count += 1;
        }
    }

    return safe_count
}


#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut safe_count: i32 = 0;

    for report in input.lines() {
        let levels: Vec<i32> = report.split_whitespace().map(|l|->i32{l.parse().unwrap()}).collect();
        if is_report_safe(&levels) {
            safe_count += 1;
        }
        else {
            for i in 0..levels.len() {
                let mut truncated_levels = levels.clone();
                _ = truncated_levels.remove(i);
                if is_report_safe(&truncated_levels) {
                    safe_count += 1;
                    break
                }
            }
        }
    }

    return safe_count
}

fn is_report_safe(levels: &Vec<i32>) -> bool {
    let mut last_level: i32 = levels[0];
    let mut has_increase = false;
    let mut has_decrease = false;
    for level in &levels[1..] {
        let diff = last_level - level;
        if diff.abs() > 3 || diff == 0 {
            return false;
        }
        if diff < 0 {
            has_increase = true;
        } else if diff > 0 {
            has_decrease = true;
        }
        last_level = *level;
    }
    if has_increase && has_decrease {
        return false;
    }
    return true
}