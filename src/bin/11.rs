use std::collections::HashMap;

use aoc_2024::input;
use glam::*;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solution(stones: &[i64], blink_count: usize) -> i64 {
    std::iter::repeat_n((), blink_count)
        .fold(
            stones
                .iter()
                .copied()
                .map(|x| (x, 1))
                .collect::<HashMap<_, _>>(),
            |stones, _| {
                stones
                    .into_iter()
                    .fold(HashMap::new(), |mut stones, (x, n)| {
                        match x {
                            0 => {
                                stones.entry(1).and_modify(|m| *m += n).or_insert(n);
                            }
                            _ => {
                                match x.to_string() {
                                    s if s.len() % 2 == 0 => {
                                        let (fst, snd) = s.split_at(s.len() / 2);
                                        stones
                                            .entry(fst.parse().unwrap())
                                            .and_modify(|m| *m += n)
                                            .or_insert(n);
                                        stones
                                            .entry(snd.parse().unwrap())
                                            .and_modify(|m| *m += n)
                                            .or_insert(n);
                                    }
                                    _ => {
                                        stones.entry(x * 2024).and_modify(|m| *m += n).or_insert(n);
                                    }
                                };
                            }
                        };
                        stones
                    })
            },
        )
        .values()
        .sum::<usize>() as i64
}

fn main() {
    let input = input(11);
    // let input = "125 17".to_string();

    let stones = parse_input(&input);

    println!("Part 1 solution: {}", solution(&stones, 25));
    println!("Part 2 solution: {}", solution(&stones, 75));
}
