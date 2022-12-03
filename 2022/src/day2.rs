use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut total_score: i32 = 0;
    for line in input.lines() {
        if line.len() == 3 {
            let (opponent_shape, my_shape) = (line.as_bytes()[0], line.as_bytes()[2]);
            total_score += get_my_round_score(char::from(my_shape), char::from(opponent_shape));
        }
    }
    return total_score
}

struct ShapeInfoPart1 {
    wins_against: char,
    score: i32
}

fn get_my_round_score(my_shape: char, opponent_shape: char) -> i32 {
    let shape_infos: HashMap<char, ShapeInfoPart1> = HashMap::from([
        ('X', ShapeInfoPart1 {wins_against: 'C', score: 1}),
        ('Y', ShapeInfoPart1 {wins_against: 'A', score: 2}),
        ('Z', ShapeInfoPart1 {wins_against: 'B', score: 3}),
        ('A', ShapeInfoPart1 {wins_against: 'Z', score: 0}),
        ('B', ShapeInfoPart1 {wins_against: 'X', score: 0}),
        ('C', ShapeInfoPart1 {wins_against: 'Y', score: 0})
    ]);

    let mut my_round_score: i32 = 0;
    if shape_infos[&my_shape].wins_against == opponent_shape {
        my_round_score += 6
    } else if shape_infos[&opponent_shape].wins_against == my_shape {
        my_round_score += 0
    } else {
        my_round_score += 3
    }
    my_round_score += shape_infos[&my_shape].score;
    return my_round_score
}

pub struct Round {
    opponent_shape: char,
    expected_outcome: char,
}

struct ShapeInfoPart2 {
    wins_against: char,
    loses_against: char,
    score: i32
}

#[aoc_generator(day2, part2)]
pub fn part2(input: &str) -> Vec<Round> {
    input.lines()
        .map(|l| {
            Round {
                opponent_shape: char::from(l.as_bytes()[0]),
                expected_outcome: char::from(l.as_bytes()[2]),
            }
        }).collect()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Round]) -> i32 {
    input.iter()
        .map(|round| {
            let win = 'Z';
            let loss = 'X';
            let shape_infos: HashMap<char, ShapeInfoPart2> = HashMap::from([
                ('A', ShapeInfoPart2 {wins_against: 'C', loses_against: 'B', score: 1}),
                ('B', ShapeInfoPart2 {wins_against: 'A', loses_against: 'C', score: 2}),
                ('C', ShapeInfoPart2 {wins_against: 'B', loses_against: 'A', score: 3})
            ]);

            let my_shape: char;
            let my_round_score: i32;
            if round.expected_outcome == win {
                my_shape = shape_infos[&round.opponent_shape].loses_against;
                my_round_score = shape_infos[&my_shape].score + 6;
            } else if round.expected_outcome == loss {
                my_shape = shape_infos[&round.opponent_shape].wins_against;
                my_round_score = shape_infos[&my_shape].score;
            } else {
                my_round_score = shape_infos[&round.opponent_shape].score + 3;
            }

            my_round_score
        })
        .sum()
}