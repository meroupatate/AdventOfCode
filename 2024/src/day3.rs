use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let mut result: i32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for (factor1, factor2) in re.captures_iter(input).map(|caps| -> (i32, i32) {
        let factor_1: i32 = caps[1].parse().unwrap();
        let factor_2: i32 = caps[2].parse().unwrap();
        (factor_1, factor_2)
    }) {
        result += factor1 * factor2;
    };

    return result;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut result: i32 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for part in input.split("do()") {
        let (to_do, _) = part.split_once("don't()").unwrap_or((part, ""));

        for (factor1, factor2) in re.captures_iter(to_do).map(|caps| -> (i32, i32) {
            let factor_1: i32 = caps[1].parse().unwrap();
            let factor_2: i32 = caps[2].parse().unwrap();
            (factor_1, factor_2)
        }) {
            result += factor1 * factor2;
        };
    }

    return result;
}
