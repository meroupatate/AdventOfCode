use std::collections::HashSet;

pub struct Action {
    direction: char,
    step: i32,
}

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|l| {
            let mut action = l.split(' ');
            Action {
                direction: action.next().unwrap().parse().unwrap(),
                step: action.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(actions: &[Action]) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut current_hx = 0;
    let mut current_hy = 0;
    let mut current_tx = 0;
    let mut current_ty = 0;
    visited.insert((current_tx, current_ty));
    for action in actions {
        for _ in 0..action.step {
            // move H
            match action.direction {
                'R' => current_hy += 1,
                'L' => current_hy -= 1,
                'U' => current_hx += 1,
                'D' => current_hx -= 1,
                _ => {}
            }

            // move T if necessary
            let x_diff = current_hx - current_tx;
            let y_diff = current_hy - current_ty;
            if current_hx == current_tx && y_diff.abs() >= 2 {
                current_ty += y_diff / y_diff.abs();
            } else if current_hy == current_ty && x_diff.abs() >= 2 {
                current_tx += x_diff / x_diff.abs();
            } else {
                if (x_diff.abs() >= 2 && y_diff.abs() >= 1)
                    || (y_diff.abs() >= 2 && x_diff.abs() >= 1)
                {
                    current_tx += x_diff / x_diff.abs();
                    current_ty += y_diff / y_diff.abs();
                }
            }

            visited.insert((current_tx, current_ty));
        }
    }
    visited.len()
}

#[aoc(day9, part2)]
pub fn solve_part2(actions: &[Action]) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut current_positions: Vec<(i32, i32)> = vec![(0, 0); 10];
    visited.insert((0, 0));
    for action in actions {
        for _ in 0..action.step {
            // move H
            match action.direction {
                'R' => current_positions[0].1 += 1,
                'L' => current_positions[0].1 -= 1,
                'U' => current_positions[0].0 += 1,
                'D' => current_positions[0].0 -= 1,
                _ => {}
            }

            // make other knots follow if necessary
            for knot in 1..10 {
                let previous_knot_x = current_positions[knot - 1].0;
                let previous_knot_y = current_positions[knot - 1].1;
                let current_knot_x = current_positions[knot].0;
                let current_knot_y = current_positions[knot].1;
                let x_diff = previous_knot_x - current_knot_x;
                let y_diff = previous_knot_y - current_knot_y;
                if previous_knot_x == current_knot_x && y_diff.abs() >= 2 {
                    current_positions[knot].1 += y_diff / y_diff.abs();
                } else if previous_knot_y == current_knot_y && x_diff.abs() >= 2 {
                    current_positions[knot].0 += x_diff / x_diff.abs();
                } else {
                    if (x_diff.abs() >= 2 && y_diff.abs() >= 1)
                        || (y_diff.abs() >= 2 && x_diff.abs() >= 1)
                    {
                        current_positions[knot].0 += x_diff / x_diff.abs();
                        current_positions[knot].1 += y_diff / y_diff.abs();
                    }
                }
            }

            visited.insert((current_positions[9].0, current_positions[9].1));
        }
    }
    visited.len()
}
