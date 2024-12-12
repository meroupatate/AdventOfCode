use std::{collections::HashMap, hash::Hash, vec};

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|stone| -> u64 { stone.parse().unwrap() })
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(stones: &Vec<u64>) -> usize {
    let mut stones = stones.clone();

    for _ in 0..25 {
        let mut updated_stones: Vec<u64> = vec![];
        for stone in stones {
            let evolved_stones = evolve_stone(stone);
            updated_stones.extend(evolved_stones);
        }
        stones = updated_stones;
    }

    return stones.len();
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct RecurseCountEvolvedStonesParams {
    current_stone: u64,
    remaining_iterations: i32,
}

#[aoc(day11, part2)]
pub fn part2(stones: &Vec<u64>) -> u64 {
    let mut total_stones_count = 0;
    let mut cached_recurse_count_evolved_stones: HashMap<RecurseCountEvolvedStonesParams, u64> =
        HashMap::new();

    for stone in stones {
        total_stones_count +=
            recurse_count_evolved_stones(*stone, 75, &mut cached_recurse_count_evolved_stones);
    }

    return total_stones_count;
}

fn recurse_count_evolved_stones(
    current_stone: u64,
    remaining_iterations: i32,
    cache: &mut HashMap<RecurseCountEvolvedStonesParams, u64>,
) -> u64 {
    if remaining_iterations == 0 {
        return 1;
    }

    let params = RecurseCountEvolvedStonesParams {
        current_stone,
        remaining_iterations,
    };
    if let Some(&cached_count) = cache.get(&params) {
        return cached_count;
    }

    let stones = evolve_stone(current_stone);
    let mut stones_count = 0;

    for stone in stones {
        stones_count += recurse_count_evolved_stones(stone, remaining_iterations - 1, cache);
    }

    cache.insert(params, stones_count);

    return stones_count;
}

fn evolve_stone(current_stone: u64) -> Vec<u64> {
    let mut evolved_stones: Vec<u64> = vec![];
    if current_stone == 0 {
        evolved_stones.push(1);
        return evolved_stones;
    }

    let value_str = current_stone.to_string();
    if value_str.len() % 2 == 0 {
        let left: u64 = value_str[..value_str.len() / 2].parse().unwrap();
        let right: u64 = value_str[value_str.len() / 2..].parse().unwrap();
        evolved_stones.push(left);
        evolved_stones.push(right);
        return evolved_stones;
    }

    evolved_stones.push(2024 * current_stone);
    return evolved_stones;
}
