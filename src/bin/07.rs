use std::collections::HashSet;

use aoc_2024::input;

fn parse_input(input: &str) -> Vec<(i64, Vec<(i64, usize)>)> {
    input
        .lines()
        .map(|line| line.split_once(":").unwrap())
        .map(|(y, xs)| {
            (
                y.trim().parse().unwrap(),
                xs.trim()
                    .split(" ")
                    .map(|x| (x.parse().unwrap(), x.len()))
                    .collect(),
            )
        })
        .collect()
}

fn solution_part_1(eqs: &[(i64, Vec<(i64, usize)>)]) -> i64 {
    eqs.iter()
        .filter(|(y, xs)| {
            let x = xs.first().copied().unwrap().0;

            xs.iter()
                .skip(1)
                .map(|(x, _)| x)
                .fold(HashSet::from([x]), |results, x| {
                    results
                        .into_iter()
                        .flat_map(|result: i64| [result * x, result + x])
                        .filter(|result| result <= y)
                        .collect()
                })
                .contains(y)
        })
        .map(|(y, _)| y)
        .sum()
}

fn solution_part_2(eqs: &[(i64, Vec<(i64, usize)>)]) -> i64 {
    eqs.iter()
        .filter(|(y, xs)| {
            let x = xs.first().copied().unwrap().0;

            xs.iter()
                .skip(1)
                .map(|&(x, concat)| (x, 10i64.pow(concat as u32)))
                .fold(HashSet::from([x]), |results, (x, pow)| {
                    results
                        .into_iter()
                        .flat_map(|result: i64| [result * x, result + x, result * pow + x])
                        .filter(|result| result <= y)
                        .collect()
                })
                .contains(y)
        })
        .map(|(y, _)| y)
        .sum()
}

fn main() {
    let input = input(7);

    let eqs = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&eqs));
    println!("Part 2 solution: {}", solution_part_2(&eqs));
}
