use itertools::Itertools;

use aoc_2024::input;

#[derive(Debug, Clone, Copy)]
enum Change {
    Pos,
    Neg,
    Bad,
}

impl Change {
    pub fn from_i32(diff: i32) -> Self {
        match diff {
            -3..=-1 => Self::Neg,
            1..=3 => Self::Pos,
            _ => Self::Bad,
        }
    }

    pub fn is_good_with(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Pos, Self::Pos) | (Self::Neg, Self::Neg)
        )
    }
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn solution_part_1(xss: &[Vec<i32>]) -> i32 {
    xss.iter()
        .filter(|xs| {
            xs.iter()
                .tuple_windows()
                .map(|(a, b)| Change::from_i32(b - a))
                .tuple_windows()
                .all(|(a, b)| a.is_good_with(b))
        })
        .count() as i32
}

fn solution_part_2(xss: &[Vec<i32>]) -> i32 {
    xss.iter()
        .filter(|xs| {
            (0..xs.len()).any(|i| {
                xs.iter()
                    .enumerate()
                    .filter_map(|(j, x)| match j == i {
                        true => None,
                        false => Some(x),
                    })
                    .tuple_windows()
                    .map(|(a, b)| Change::from_i32(b - a))
                    .tuple_windows()
                    .all(|(a, b)| a.is_good_with(b))
            })
        })
        .count() as i32
}

fn main() {
    let input = input(2);

    let xss = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&xss));
    println!("Part 2 solution: {}", solution_part_2(&xss));
}
