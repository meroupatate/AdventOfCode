#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(grid: &Vec<Vec<u32>>) -> i32 {
    let max_x = grid.len();
    let max_y = grid[0].len();
    let mut visible_trees = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            // Try top
            let mut i = x;
            let mut visible_from_top = true;
            while i > 0 {
                i -= 1;
                if grid[i][y] >= *tree {
                    visible_from_top = false;
                    break;
                }
            }
            if visible_from_top {
                visible_trees += 1;
                continue;
            }

            // Try bottom
            i = x;
            let mut visible_from_bottom = true;
            while i < max_x - 1 {
                i += 1;
                if grid[i][y] >= *tree {
                    visible_from_bottom = false;
                    break;
                }
            }
            if visible_from_bottom {
                visible_trees += 1;
                continue;
            }

            // Try left
            let mut j = y;
            let mut visible_from_left = true;
            while j > 0 {
                j -= 1;
                if grid[x][j] >= *tree {
                    visible_from_left = false;
                    break;
                }
            }
            if visible_from_left {
                visible_trees += 1;
                continue;
            }

            // Try right
            j = y;
            let mut visible_from_right = true;
            while j < max_y - 1 {
                j += 1;
                if grid[x][j] >= *tree {
                    visible_from_right = false;
                    break;
                }
            }
            if visible_from_right {
                visible_trees += 1;
                continue;
            }
        }
    }

    visible_trees
}

#[aoc(day8, part2)]
pub fn solve_part2(grid: &Vec<Vec<u32>>) -> i32 {
    let max_x = grid.len();
    let max_y = grid[0].len();
    let mut max_scenic_score = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, tree) in row.iter().enumerate() {
            // Try top
            let mut i = x;
            let mut visible_from_top = 0;
            while i > 0 {
                i -= 1;
                if grid[i][y] >= *tree {
                    visible_from_top += 1;
                    break;
                }
                visible_from_top += 1;
            }

            // Try bottom
            i = x;
            let mut visible_from_bottom = 0;
            while i < max_x - 1 {
                i += 1;
                if grid[i][y] >= *tree {
                    visible_from_bottom += 1;
                    break;
                }
                visible_from_bottom += 1
            }

            // Try left
            let mut j = y;
            let mut visible_from_left = 0;
            while j > 0 {
                j -= 1;
                if grid[x][j] >= *tree {
                    visible_from_left += 1;
                    break;
                }
                visible_from_left += 1;
            }

            // Try right
            j = y;
            let mut visible_from_right = 0;
            while j < max_y - 1 {
                j += 1;
                if grid[x][j] >= *tree {
                    visible_from_right += 1;
                    break;
                }
                visible_from_right += 1;
            }

            let scenic_score =
                visible_from_top * visible_from_bottom * visible_from_left * visible_from_right;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}
