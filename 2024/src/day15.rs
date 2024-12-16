use std::{collections::VecDeque, vec};

pub struct Input {
    grid: Vec<Vec<char>>,
    path: Vec<char>,
}

#[aoc_generator(day15, part1)]
pub fn parse_input(input: &str) -> Input {
    let mut split_input = input.split("\n\n");

    let grid: Vec<Vec<char>> = split_input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let path = split_input
        .next()
        .unwrap()
        .replace("\n", "")
        .chars()
        .collect();

    return Input {
        grid: grid,
        path: path,
    };
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> i32 {
    let mut current_grid = input.grid.clone();
    let (mut current_robot_x, mut current_robot_y) = find_robot(&current_grid).unwrap();
    for &step in &input.path {
        /*
        current_grid.clone().into_iter().for_each(|line| {
            println!("{:?}", line);
        });
        println!("{}", step);
        */
        if step == '<' {
            let mut box_count = 0;
            while current_grid[current_robot_x][current_robot_y - box_count - 1] == 'O' {
                box_count += 1;
            }

            if current_grid[current_robot_x][current_robot_y - box_count - 1] == '#' {
                println!("Nothing happens");
                continue; // Wall before or after boxes: nothing happens
            }

            // Move robot
            current_grid[current_robot_x][current_robot_y] = '.';
            current_grid[current_robot_x][current_robot_y - 1] = '@';
            current_robot_y -= 1;

            // Push boxes
            if box_count > 0 {
                println!("{} boxes moved", box_count);
                current_grid[current_robot_x][current_robot_y - box_count] = 'O';
            }
        }
        if step == '>' {
            let mut box_count = 0;
            while current_grid[current_robot_x][current_robot_y + box_count + 1] == 'O' {
                box_count += 1;
            }

            if current_grid[current_robot_x][current_robot_y + box_count + 1] == '#' {
                println!("Nothing happens");
                continue; // Wall before or after boxes: nothing happens
            }

            // Move robot
            current_grid[current_robot_x][current_robot_y] = '.';
            current_grid[current_robot_x][current_robot_y + 1] = '@';
            current_robot_y += 1;

            // Push boxes
            if box_count > 0 {
                println!("{} boxes moved", box_count);
                current_grid[current_robot_x][current_robot_y + box_count] = 'O';
            }
        }
        if step == '^' {
            let mut box_count = 0;
            while current_grid[current_robot_x - box_count - 1][current_robot_y] == 'O' {
                box_count += 1;
            }

            if current_grid[current_robot_x - box_count - 1][current_robot_y] == '#' {
                println!("Nothing happens");
                continue; // Wall before or after boxes: nothing happens
            }

            // Move robot
            current_grid[current_robot_x][current_robot_y] = '.';
            current_grid[current_robot_x - 1][current_robot_y] = '@';
            current_robot_x -= 1;

            // Push boxes
            if box_count > 0 {
                println!("{} boxes moved", box_count);
                current_grid[current_robot_x - box_count][current_robot_y] = 'O';
            }
        }
        if step == 'v' {
            let mut box_count = 0;
            while current_grid[current_robot_x + box_count + 1][current_robot_y] == 'O' {
                box_count += 1;
            }

            if current_grid[current_robot_x + box_count + 1][current_robot_y] == '#' {
                println!("Nothing happens");
                continue; // Wall before or after boxes: nothing happens
            }

            // Move robot
            current_grid[current_robot_x + 1][current_robot_y] = '@';
            current_grid[current_robot_x][current_robot_y] = '.';
            current_robot_x += 1;

            // Push boxes
            if box_count > 0 {
                println!("{} boxes moved", box_count);
                current_grid[current_robot_x + box_count][current_robot_y] = 'O';
            }
        }
    }

    return sum_boxes_coordinates(&current_grid);
}

fn find_robot(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '@' {
                return Some((i, j));
            }
        }
    }
    None
}

fn sum_boxes_coordinates(grid: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'O' {
                total += (100 * i) + j;
            }
        }
    }
    return total as i32;
}

#[aoc_generator(day15, part2)]
pub fn parse_input_part2(input: &str) -> Input {
    let mut split_input = input.split("\n\n");

    let grid: Vec<Vec<char>> = split_input
        .next()
        .unwrap()
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let path = split_input
        .next()
        .unwrap()
        .replace("\n", "")
        .chars()
        .collect();

    return Input {
        grid: grid,
        path: path,
    };
}

#[aoc(day15, part2)]
pub fn part2(input: &Input) -> i32 {
    let mut current_grid = input.grid.clone();
    let (mut current_robot_x, mut current_robot_y) = find_robot(&current_grid).unwrap();
    for &step in &input.path {
        println!(
            "Current robot position ({}, {}): going {}",
            current_robot_x, current_robot_y, step
        );
        if step == '<' {
            let mut box_count = 0;
            while current_grid[current_robot_x][current_robot_y - 2 * box_count - 1] == ']' {
                box_count += 1;
            }

            if current_grid[current_robot_x][current_robot_y - 2 * box_count - 1] == '#' {
                println!("Nothing happens");
                continue; // Wall before or after boxes: nothing happens
            }

            // Move robot
            current_grid[current_robot_x][current_robot_y] = '.';
            current_grid[current_robot_x][current_robot_y - 1] = '@';
            current_robot_y -= 1;

            // Push boxes
            if box_count > 0 {
                println!("{} boxes moved", box_count);
                current_grid[current_robot_x].splice(
                    current_robot_y - 2 * box_count..current_robot_y,
                    "[]".repeat(box_count).chars(),
                );
            }
        }
        if step == '>' {
            let mut box_count = 0;
            while current_grid[current_robot_x][current_robot_y + 2 * box_count + 1] == '[' {
                box_count += 1;
            }

            if current_grid[current_robot_x][current_robot_y + 2 * box_count + 1] == '#' {
                println!("Nothing happens");
                continue; // Wall before or after boxes: nothing happens
            }

            // Move robot
            current_grid[current_robot_x][current_robot_y] = '.';
            current_grid[current_robot_x][current_robot_y + 1] = '@';
            current_robot_y += 1;

            // Push boxes
            if box_count > 0 {
                println!("{} boxes moved", box_count);
                current_grid[current_robot_x][current_robot_y + box_count] = 'O';
                current_grid[current_robot_x].splice(
                    current_robot_y + 1..current_robot_y + 2 * box_count + 1,
                    "[]".repeat(box_count).chars(),
                );
            }
        }
        if step == '^' {
            let mut updated_grid = current_grid.clone();
            let mut moved_boxes: Vec<(usize, usize)> = vec![];
            let mut cells_to_check: VecDeque<(usize, usize)> =
                VecDeque::from(vec![(current_robot_x - 1, current_robot_y)]);
            let mut met_wall = false;

            while let Some((cx, cy)) = cells_to_check.pop_front() {
                println!(
                    "Checking ({}, {}), still have to check {:?}",
                    cx, cy, cells_to_check
                );
                if current_grid[cx][cy] == '#' {
                    println!("Nothing happens");
                    met_wall = true;
                    break; // Wall before or after boxes: nothing happens
                }
                if current_grid[cx][cy] == '[' {
                    updated_grid[cx][cy] = '.';
                    updated_grid[cx][cy + 1] = '.';
                    moved_boxes.push((cx, cy));
                    cells_to_check.push_back((cx - 1, cy));
                    cells_to_check.push_back((cx - 1, cy + 1));
                }
                if current_grid[cx][cy] == ']' {
                    updated_grid[cx][cy] = '.';
                    updated_grid[cx][cy - 1] = '.';
                    moved_boxes.push((cx, cy - 1));
                    cells_to_check.push_back((cx - 1, cy - 1));
                    cells_to_check.push_back((cx - 1, cy));
                }
            }

            if !met_wall {
                // Move robot
                updated_grid[current_robot_x][current_robot_y] = '.';
                updated_grid[current_robot_x - 1][current_robot_y] = '@';
                current_robot_x -= 1;

                // Push boxes
                for (bx, by) in moved_boxes {
                    updated_grid[bx - 1][by] = '[';
                    updated_grid[bx - 1][by + 1] = ']';
                }

                // Update current grid
                current_grid = updated_grid;
            }
        }
        if step == 'v' {
            let mut updated_grid = current_grid.clone();
            let mut moved_boxes: Vec<(usize, usize)> = vec![];
            let mut cells_to_check: VecDeque<(usize, usize)> =
                VecDeque::from(vec![(current_robot_x + 1, current_robot_y)]);
            let mut met_wall = false;

            while let Some((cx, cy)) = cells_to_check.pop_front() {
                if current_grid[cx][cy] == '#' {
                    println!("Nothing happens");
                    met_wall = true;
                    break; // Wall before or after boxes: nothing happens
                }
                if current_grid[cx][cy] == '[' {
                    updated_grid[cx][cy] = '.';
                    updated_grid[cx][cy + 1] = '.';
                    moved_boxes.push((cx, cy));
                    cells_to_check.push_back((cx + 1, cy));
                    cells_to_check.push_back((cx + 1, cy + 1));
                }
                if current_grid[cx][cy] == ']' {
                    updated_grid[cx][cy] = '.';
                    updated_grid[cx][cy - 1] = '.';
                    moved_boxes.push((cx, cy - 1));
                    cells_to_check.push_back((cx + 1, cy - 1));
                    cells_to_check.push_back((cx + 1, cy));
                }
            }

            if !met_wall {
                // Move robot
                updated_grid[current_robot_x][current_robot_y] = '.';
                updated_grid[current_robot_x + 1][current_robot_y] = '@';
                current_robot_x += 1;

                // Push boxes
                for (bx, by) in moved_boxes {
                    updated_grid[bx + 1][by] = '[';
                    updated_grid[bx + 1][by + 1] = ']';
                }

                // Update current grid
                current_grid = updated_grid;
            }
        }
    }

    return sum_larger_boxes_coordinates(&current_grid);
}

fn sum_larger_boxes_coordinates(grid: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '[' {
                total += (100 * i) + j;
            }
        }
    }
    return total as i32;
}
