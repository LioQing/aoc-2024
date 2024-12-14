use std::collections::HashSet;

use aoc_2024::input;
use itertools::Itertools;

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    fn parse_rules(input: &str) -> HashSet<(i32, i32)> {
        input
            .lines()
            .map(|line| line.split_once("|").unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect()
    }

    fn parse_orders(input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect_vec())
            .collect_vec()
    }

    let (rules, orders) = input.split_once("\n\n").unwrap();
    (parse_rules(rules), parse_orders(orders))
}

fn solution_part_1(rules: &HashSet<(i32, i32)>, orders: &[Vec<i32>]) -> i32 {
    orders
        .iter()
        .filter(|order| {
            order
                .iter()
                .copied()
                .sorted_by(|a, b| match rules.contains(&(*a, *b)) {
                    true => std::cmp::Ordering::Less,
                    false => std::cmp::Ordering::Greater,
                })
                .zip(order.iter().copied())
                .all(|(a, b)| a == b)
        })
        .map(|order| order.get(order.len() / 2).unwrap())
        .sum()
}

fn solution_part_2(rules: &HashSet<(i32, i32)>, orders: &[Vec<i32>]) -> i32 {
    orders
        .iter()
        .filter_map(|order| {
            match order
                .iter()
                .copied()
                .sorted_by(|a, b| match rules.contains(&(*a, *b)) {
                    true => std::cmp::Ordering::Less,
                    false => std::cmp::Ordering::Greater,
                })
                .collect_vec()
            {
                sorted if &sorted == order => None,
                sorted => Some(sorted),
            }
        })
        .map(|order| order.get(order.len() / 2).copied().unwrap())
        .sum()
}

fn main() {
    let input = input(5);

    let (rules, orders) = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&rules, &orders));
    println!("Part 2 solution: {}", solution_part_2(&rules, &orders));
}
