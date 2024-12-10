use std::collections::HashSet;

use aoc_2024::input;
use glam::*;
use itertools::Itertools;

trait MapAt {
    fn at(&self, pos: IVec2) -> Option<i32>;
}

impl MapAt for &[Vec<i32>] {
    fn at(&self, pos: IVec2) -> Option<i32> {
        self.get(usize::try_from(pos.y).ok()?)?
            .get(usize::try_from(pos.x).ok()?)
            .copied()
    }
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect()
}

fn solution_part_1(map: &[Vec<i32>]) -> i32 {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &h)| (ivec2(x as i32, y as i32), h))
        })
        .filter(|&(_, h)| h == 0)
        .map(|(pos, _)| {
            (1..=9)
                .fold(HashSet::from([pos]), |xs, h| {
                    xs.into_iter()
                        .flat_map(|x| {
                            [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                                .iter()
                                .map(move |offset| x + offset)
                                .filter(|&x| map.at(x) == Some(h))
                        })
                        .collect()
                })
                .len() as i32
        })
        .sum()
}

fn solution_part_2(map: &[Vec<i32>]) -> i32 {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &h)| (ivec2(x as i32, y as i32), h))
        })
        .filter(|&(_, h)| h == 0)
        .map(|(pos, _)| {
            (1..=9)
                .fold(vec![pos], |xs, h| {
                    xs.into_iter()
                        .flat_map(|x| {
                            [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                                .iter()
                                .map(move |offset| x + offset)
                                .filter(|&x| map.at(x) == Some(h))
                        })
                        .collect()
                })
                .len() as i32
        })
        .sum()
}

fn main() {
    let input = input(10);

    let map = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&map));
    println!("Part 2 solution: {}", solution_part_2(&map));
}
