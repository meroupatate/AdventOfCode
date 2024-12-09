use std::vec;

pub struct Input {
    updates: Vec<Vec<i32>>,
    rules: Vec<Vec<i32>>,
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Input {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .map(|line| -> Vec<i32> {
            line.split(",")
                .map(|page| page.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let rules: Vec<Vec<i32>> = parts[0]
        .lines()
        .map(|line| -> Vec<i32> {
            let pages: Vec<i32> = line
                .split("|")
                .map(|page| page.parse::<i32>().unwrap())
                .collect();
            pages
        })
        .collect();

    Input { updates, rules }
}

#[aoc(day5, part1)]
pub fn part1(input: &Input) -> i32 {
    let mut total = 0;
    for update in &input.updates {
        if is_update_valid(input, &update) {
            total += update[update.len() / 2];
        }
    }
    return total;
}

fn is_update_valid(input: &Input, update: &Vec<i32>) -> bool {
    let mut previous_pages: Vec<i32> = vec![];
    let mut update_is_valid = true;
    for page in update {
        if is_current_page_invalid(input, page, &previous_pages) {
            update_is_valid = false;
            break;
        }
        previous_pages.push(*page);
    }
    return update_is_valid;
}

fn is_current_page_invalid(input: &Input, current_page: &i32, previous_pages: &Vec<i32>) -> bool {
    input.rules.clone().into_iter().any(|rule| -> bool {
        let mut is_invalid = false;
        if rule[0] == *current_page {
            is_invalid = previous_pages.contains(&rule[1]);
        }
        return is_invalid;
    })
}

#[aoc(day5, part2)]
pub fn part2(input: &Input) -> i32 {
    let mut total = 0;
    for update in &input.updates {
        if !is_update_valid(input, update) {
            let mut ordered_previous_pages: Vec<i32> = vec![];
            for page in update {
                ordered_previous_pages =
                    place_current_page(input, &update, page, &ordered_previous_pages);
            }
            if ordered_previous_pages != *update {
                total += ordered_previous_pages[ordered_previous_pages.len() / 2];
            }
        }
    }
    return total;
}

fn place_current_page(
    input: &Input,
    current_update: &Vec<i32>,
    current_page: &i32,
    ordered_previous_pages: &Vec<i32>,
) -> Vec<i32> {
    let mut ordered_pages: Vec<i32> = vec![];
    let mut current_page_is_placed = false;
    for page in ordered_previous_pages {
        if !current_page_is_placed
            && current_page_is_before(input, current_page, page, current_update)
        {
            ordered_pages.push(*current_page);
            current_page_is_placed = true;
        }
        ordered_pages.push(*page);
    }
    if !current_page_is_placed {
        ordered_pages.push(*current_page);
    }
    return ordered_pages;
}

fn current_page_is_before(
    input: &Input,
    current_page: &i32,
    target_page: &i32,
    current_update: &Vec<i32>,
) -> bool {
    let existing_dependencies: Vec<i32> = vec![];
    let target_page_dependencies: Vec<i32> =
        find_page_dependencies(input, target_page, current_update, &existing_dependencies);
    return target_page_dependencies.contains(current_page);
}

fn find_page_dependencies(
    input: &Input,
    page: &i32,
    current_update: &Vec<i32>,
    existing_dependencies: &Vec<i32>,
) -> Vec<i32> {
    let mut page_dependencies: Vec<i32> = vec![];
    for rule in &input.rules {
        if current_update.contains(&rule[0])
            && current_update.contains(&rule[1])
            && rule[1] == *page
            && !existing_dependencies.contains(&rule[0])
        {
            page_dependencies.push(rule[0]);
        }
    }

    let mut all_dependencies = page_dependencies.clone();
    for dependency in page_dependencies {
        let dependency_dependencies =
            find_page_dependencies(input, &dependency, current_update, &all_dependencies);
        all_dependencies.extend(dependency_dependencies);
    }

    return all_dependencies;
}
