use regex::Regex;

pub struct Point {
    x: i64,
    y: i64,
}

pub struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let machines: Vec<Machine> = re
        .captures_iter(input)
        .map(|captures| {
            let (_, [ax, ay, bx, by, px, py]) = captures.extract();
            Machine {
                a: Point {
                    x: ax.parse().unwrap(),
                    y: ay.parse().unwrap(),
                },
                b: Point {
                    x: bx.parse().unwrap(),
                    y: by.parse().unwrap(),
                },
                prize: Point {
                    x: px.parse().unwrap(),
                    y: py.parse().unwrap(),
                },
            }
        })
        .collect();

    machines
}

#[aoc(day13, part1)]
pub fn part1(machines: &Vec<Machine>) -> i64 {
    let mut total_cost = 0;
    for machine in machines {
        let mut cheapest_cost = i64::MAX;
        for i in 0..100 {
            let jx = (machine.prize.x - i * machine.a.x) / machine.b.x;
            let jy = (machine.prize.y - i * machine.a.y) / machine.b.y;

            if jx == jy
                && jx * machine.b.x == machine.prize.x - i * machine.a.x
                && jy * machine.b.y == machine.prize.y - i * machine.a.y
            {
                let price = i * 3 + jx;
                if price < cheapest_cost {
                    cheapest_cost = price;
                }
            }
        }
        if cheapest_cost != i64::MAX {
            total_cost += cheapest_cost;
        }
    }
    return total_cost;
}

#[aoc(day13, part2)]
pub fn part2(machines: &Vec<Machine>) -> i64 {
    let mut total_cost = 0;
    for machine in machines {
        let new_prize_x: i64 = machine.prize.x + 10000000000000;
        let new_prize_y: i64 = machine.prize.y + 10000000000000;

        let i = (new_prize_x * machine.b.y - new_prize_y * machine.b.x)
            / (machine.b.y * machine.a.x - machine.b.x * machine.a.y);
        let j = (new_prize_y - i * machine.a.y) / machine.b.y;

        if j >= 0
            && i >= 0
            && (
                j * machine.b.x + i * machine.a.x,
                j * machine.b.y + i * machine.a.y,
            ) == (new_prize_x, new_prize_y)
        {
            total_cost += 3 * i + j;
        }
    }
    return total_cost;
}
