#![allow(unstable_name_collisions)]

use std::collections::HashMap;

use itertools::Itertools;

use aoc_2024::input;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

fn solution_part_1(xs: &[i32], ys: &[i32]) -> i32 {
    xs.iter()
        .sorted()
        .zip(ys.iter().sorted())
        .map(|(x, y)| x.abs_diff(*y) as i32)
        .sum()
}

fn solution_part_2(xs: &[i32], ys: &[i32]) -> i32 {
    let counts = ys.iter().fold(HashMap::new(), |mut counts, y| {
        counts.entry(y).and_modify(|count| *count += 1).or_insert(1);
        counts
    });

    xs.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum()
}

fn main() {
    let input = input(1);

    let (xs, ys) = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&xs, &ys));
    println!("Part 2 solution: {}", solution_part_2(&xs, &ys));
}