use std::vec;

use regex::Regex;

#[derive(Clone, Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

const MAX_X: i32 = 101;
const MAX_Y: i32 = 103;

impl Robot {
    pub fn step(&mut self) {
        self.x = ((self.x + self.vx) % MAX_X + MAX_X) % MAX_X;
        self.y = ((self.y + self.vy) % MAX_Y + MAX_Y) % MAX_Y;
    }

    pub fn steps(&mut self, n: i32) {
        for _ in 0..n {
            self.step();
        }
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=([-0-9]+),([-0-9]+)").unwrap();

    let mut robots: Vec<Robot> = vec![];
    re.captures_iter(input).for_each(|captures| {
        let (_, [x, y, vx, vy]) = captures.extract();
        robots.push(Robot {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            vx: vx.parse().unwrap(),
            vy: vy.parse().unwrap(),
        });
    });

    robots
}

#[aoc(day14, part1)]
pub fn part1(input_robots: &Vec<Robot>) -> i64 {
    let mut robots = input_robots.clone();

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for robot in robots.iter_mut() {
        robot.steps(100);

        if robot.x > MAX_X / 2 {
            if robot.y > MAX_Y / 2 {
                q1 += 1;
            }
            if robot.y < MAX_Y / 2 {
                q2 += 1;
            }
        }

        if robot.x < MAX_X / 2 {
            if robot.y > MAX_Y / 2 {
                q3 += 1;
            }
            if robot.y < MAX_Y / 2 {
                q4 += 1;
            }
        }
    }

    return q1 * q2 * q3 * q4;
}

#[aoc(day14, part2)]
pub fn part2(input_robots: &Vec<Robot>) -> i32 {
    let mut robots = input_robots.clone();
    let mut seconds: i32 = 0;
    let mut found_tree: bool = false;

    while !found_tree {
        for robot in robots.iter_mut() {
            robot.step();
        }

        found_tree = show_map(&robots);

        seconds += 1;
    }

    return seconds;
}

pub fn show_map(robots: &Vec<Robot>) -> bool {
    let mut map: Vec<Vec<i32>> = vec![vec![0; MAX_X as usize]; MAX_Y as usize];

    for robot in robots {
        map[robot.y as usize][robot.x as usize] += 1;
    }

    let mut found_tree = false;
    for line in map {
        let mut line_display = "".to_string();
        for n in line {
            if n == 0 {
                line_display.push('.');
            } else {
                line_display.push('\u{2588}')
            }
        }
        println!("{}", line_display);

        let tree_pattern = "\u{2588}".repeat(10);
        if line_display.contains(tree_pattern.as_str()) {
            found_tree = true;
        }
    }
    println!("");

    return found_tree;
}
