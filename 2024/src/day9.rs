use std::vec;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<i32> {
    let parsed_input: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut disk: Vec<i32> = vec![];
    for (i, value) in parsed_input.into_iter().enumerate() {
        let is_file: bool = i % 2 == 0;
        if is_file {
            for _ in 0..value {
                disk.push((i / 2) as i32);
            }
        } else {
            for _ in 0..value {
                disk.push(-1);
            }
        }
    }

    return disk;
}

#[aoc(day9, part1)]
pub fn part1(disk: &Vec<i32>) -> u128 {
    let mut updated_disk = disk.clone();
    let disk_total_size = updated_disk.len();
    for (i, value) in disk.into_iter().rev().enumerate() {
        let has_empty_space: bool = updated_disk.clone()[0..disk_total_size - (i + 1)]
            .into_iter()
            .any(|&v| v == -1);
        if has_empty_space {
            if *value >= 0 {
                let first_empty_index = updated_disk.iter().position(|&d| d == -1).unwrap();
                updated_disk[first_empty_index] = *value;

                updated_disk[disk_total_size - (i + 1)] = -1;
            }
        } else {
            break;
        }
    }

    return compute_checksum(&updated_disk);
}

fn compute_checksum(disk: &Vec<i32>) -> u128 {
    let mut checksum: u128 = 0;
    for (id, value) in disk
        .into_iter()
        .map(|&v| -> i32 {
            if v == -1 {
                return 0;
            }
            return v;
        })
        .enumerate()
    {
        checksum += id as u128 * value as u128;
    }
    return checksum;
}

#[aoc(day9, part2)]
pub fn part2(disk: &Vec<i32>) -> u128 {
    let mut updated_disk = disk.clone();
    let mut current_block: Vec<i32> = vec![];
    let mut moved_ids: Vec<i32> = vec![];
    for (i, &value) in disk.into_iter().rev().enumerate() {
        if moved_ids.contains(&value) {
            continue; // skip already moved blocks
        }
        if !current_block.is_empty() {
            let current_value = current_block[0];
            if current_value != value {
                if let Some(i_available_space) = updated_disk[0..updated_disk.len() - i]
                    .windows(current_block.len())
                    .position(|window| window.into_iter().all(|&value| value == -1))
                {
                    let empty_block = vec![-1; current_block.len()];
                    updated_disk.splice(
                        i_available_space..i_available_space + current_block.len(),
                        current_block,
                    );
                    updated_disk.splice(
                        updated_disk.len() - i..updated_disk.len() - i + empty_block.len(),
                        empty_block,
                    );
                    moved_ids.push(current_value);
                }

                current_block = vec![];
            }
        }
        if value != -1 {
            current_block.push(value);
        }
    }

    return compute_checksum(&updated_disk);
}
