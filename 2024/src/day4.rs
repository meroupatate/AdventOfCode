#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
pub fn part1(grid: &Vec<Vec<char>>) -> u32 {
    let mut found = 0;
    for (i, line) in grid.into_iter().enumerate() {
        for (j, _) in line.into_iter().enumerate() {
            found += find_xmas(grid, i, j)
        }
    }
    return found;
}

fn find_xmas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut found = 0;

    if is_char(grid, i, j, 'X') {
        // Top
        if i >= 3
            && is_char(grid, i - 1, j, 'M')
            && is_char(grid, i - 2, j, 'A')
            && is_char(grid, i - 3, j, 'S')
        {
            found += 1;
        }

        // Bottom
        if i < grid.len() - 3
            && is_char(grid, i + 1, j, 'M')
            && is_char(grid, i + 2, j, 'A')
            && is_char(grid, i + 3, j, 'S')
        {
            found += 1;
        }

        // Left
        if j >= 3
            && is_char(grid, i, j - 1, 'M')
            && is_char(grid, i, j - 2, 'A')
            && is_char(grid, i, j - 3, 'S')
        {
            found += 1;
        }

        // Right
        if j < grid[0].len() - 3
            && is_char(grid, i, j + 1, 'M')
            && is_char(grid, i, j + 2, 'A')
            && is_char(grid, i, j + 3, 'S')
        {
            found += 1;
        }

        // Top left
        if i >= 3
            && j >= 3
            && is_char(grid, i - 1, j - 1, 'M')
            && is_char(grid, i - 2, j - 2, 'A')
            && is_char(grid, i - 3, j - 3, 'S')
        {
            found += 1;
        }

        // Bottom left
        if i < grid.len() - 3
            && j >= 3
            && is_char(grid, i + 1, j - 1, 'M')
            && is_char(grid, i + 2, j - 2, 'A')
            && is_char(grid, i + 3, j - 3, 'S')
        {
            found += 1;
        }

        // Top right
        if i >= 3
            && j < grid[0].len() - 3
            && is_char(grid, i - 1, j + 1, 'M')
            && is_char(grid, i - 2, j + 2, 'A')
            && is_char(grid, i - 3, j + 3, 'S')
        {
            found += 1;
        }

        // Bottom right
        if i < grid.len() - 3
            && j < grid[0].len() - 3
            && is_char(grid, i + 1, j + 1, 'M')
            && is_char(grid, i + 2, j + 2, 'A')
            && is_char(grid, i + 3, j + 3, 'S')
        {
            found += 1;
        }
    }

    found
}

fn is_char(grid: &Vec<Vec<char>>, i: usize, j: usize, c: char) -> bool {
    grid[i][j] == c
}

#[aoc(day4, part2)]
pub fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let mut found = 0;

    for (i, line) in grid.into_iter().enumerate() {
        for (j, _) in line.into_iter().enumerate() {
            if find_x_mas(grid, i, j) {
                found += 1;
            }
        }
    }
    return found;
}

fn find_x_mas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i >= 1 && i < grid.len() - 1 && j >= 1 && j < grid.len() - 1 && is_char(grid, i, j, 'A') {
        let diagonal_letters: Vec<char> = vec![
            grid[i - 1][j - 1],
            grid[i - 1][j + 1],
            grid[i + 1][j - 1],
            grid[i + 1][j + 1],
        ];
        if diagonal_letters.iter().filter(|l| **l == 'M').count() == 2
            && diagonal_letters.iter().filter(|l| **l == 'S').count() == 2
            && diagonal_letters[0] != diagonal_letters[3]
            && diagonal_letters[1] != diagonal_letters[2]
        {
            return true;
        }
    }
    return false;
}
