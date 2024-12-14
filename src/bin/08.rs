use std::collections::HashMap;

use aoc_2024::input;
use glam::*;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn solution_part_1(map: &[Vec<u8>]) -> i32 {
    let width = map.first().unwrap().len();
    let height = map.len();
    let size = ivec2(width as i32, height as i32);

    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &freq)| (ivec2(x as i32, y as i32), freq))
        })
        .filter(|(_, freq)| freq.is_ascii_alphanumeric())
        .fold(HashMap::<u8, Vec<_>>::new(), |mut antennas, (pos, freq)| {
            antennas
                .entry(freq)
                .and_modify(|poss| poss.push(pos))
                .or_insert(vec![pos]);
            antennas
        })
        .into_values()
        .flat_map(|poss| {
            poss.into_iter()
                .tuple_combinations()
                .flat_map(|(a, b)| [2 * b - a, 2 * a - b])
        })
        .filter(|pos| (pos.cmpge(IVec2::ZERO) & pos.cmplt(size)).all())
        .unique()
        .count() as i32
}

fn solution_part_2(map: &[Vec<u8>]) -> i32 {
    let width = map.first().unwrap().len();
    let height = map.len();
    let size = ivec2(width as i32, height as i32);

    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &freq)| (ivec2(x as i32, y as i32), freq))
        })
        .filter(|(_, freq)| freq.is_ascii_alphanumeric())
        .fold(HashMap::<u8, Vec<_>>::new(), |mut antennas, (pos, freq)| {
            antennas
                .entry(freq)
                .and_modify(|poss| poss.push(pos))
                .or_insert(vec![pos]);
            antennas
        })
        .into_values()
        .flat_map(|antennas| {
            antennas
                .into_iter()
                .tuple_combinations()
                .flat_map(|(a, b)| {
                    [b, a]
                        .into_iter()
                        .zip([b - a, a - b])
                        .flat_map(|(pos, step)| {
                            std::iter::repeat(step)
                                .enumerate()
                                .map(move |(i, step)| pos + step * i as i32)
                                .take_while(|antinode| {
                                    (antinode.cmpge(IVec2::ZERO) & antinode.cmplt(size)).all()
                                })
                        })
                })
        })
        .unique()
        .count() as i32
}

fn main() {
    let input = input(8);

    let eqs = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&eqs));
    println!("Part 2 solution: {}", solution_part_2(&eqs));
}
