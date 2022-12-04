pub struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    fn fully_contains(&self, a: &Assignment) -> bool {
        return self.start <= a.start && self.end >= a.end
    }
    fn overlaps(&self, a: &Assignment) -> bool {
        return (self.start <= a.start && a.start <= self.end) || (self.start <= a.end && a.end <= self.end)
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<(Assignment, Assignment)> {
    input.lines()
        .map(|l| {
            let mut assignments = l.split(',');
            let mut assignment_a = assignments.next().unwrap().split('-');
            let mut assignment_b = assignments.next().unwrap().split('-');
            (
                Assignment {
                    start: assignment_a.next().unwrap().parse::<i32>().unwrap(),
                    end: assignment_a.next().unwrap().parse::<i32>().unwrap(),
                },
                Assignment {
                    start: assignment_b.next().unwrap().parse::<i32>().unwrap(),
                    end: assignment_b.next().unwrap().parse::<i32>().unwrap(),
                },
            )
        }).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[(Assignment, Assignment)]) -> i32 {
    input.iter()
        .map(|assignments| {
            let (assignment_a, assignment_b) = assignments;
             (assignment_a.fully_contains(&assignment_b) || assignment_b.fully_contains(&assignment_a)) as i32
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[(Assignment, Assignment)]) -> i32 {
    input.iter()
        .map(|assignments| {
            let (assignment_a, assignment_b) = assignments;
             (assignment_a.overlaps(&assignment_b) || assignment_b.overlaps(&assignment_a)) as i32
        })
        .sum()
}
