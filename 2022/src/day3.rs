use std::collections::{HashMap, HashSet};

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let alphabet = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()
    ).unwrap();
    let mut priorities: i32 = 0;
    for line in input.lines() {
        for char in line[line.len() / 2..line.len()].chars() {
            if line[0..line.len() / 2].contains(char) {
                priorities += (alphabet.find(char).unwrap() as i32) + 1;
                break
            }
        }
    }
    return priorities
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let alphabet = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()
    ).unwrap();
    let mut unique_team_objects:HashMap<char, i32> = HashMap::new();
    let mut priorities: i32 = 0;

    for (i, line) in input.lines().enumerate() {
        let mut unique_personal_objects:HashSet<char> = HashSet::new();
        for c in line.chars() {
            if !unique_personal_objects.contains(&c) {
                *unique_team_objects.entry(c).or_insert(0) += 1;
            }
            unique_personal_objects.insert(c);
        }
        if i % 3 == 2 {
            for (object, nb) in unique_team_objects {
                if nb == 3 {
                    priorities += (alphabet.find(object).unwrap() as i32) + 1;
                    break
                }
            }
            unique_team_objects = HashMap::new()
        }
    }
    return priorities
}
