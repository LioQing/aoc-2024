use aoc_2024::input;
use glam::*;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::multispace1,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    fn pattern(input: &str) -> IResult<&str, &str> {
        take_while1(|c| c == 'w' || c == 'u' || c == 'b' || c == 'r' || c == 'g')(input)
    }

    let (_, (patterns, designs)) = (separated_pair(
        separated_list1(tag(", "), pattern),
        multispace1,
        separated_list1(multispace1, pattern),
    )(input))
    .unwrap();

    (patterns, designs)
}

pub fn solution_part_1(patterns: &[&str], designs: &[&str]) -> i32 {
    designs
        .iter()
        .filter(|&design| {
            let completes = (0..=design.len()).map(|i| i == 0).collect_vec();

            (1..=design.len())
                .map(|len| (len, &design.as_bytes()[..len]))
                .fold(completes, |mut completes, (i, subdesign)| {
                    completes[i] = patterns.iter().any(|&pattern| {
                        subdesign.ends_with(pattern.as_bytes())
                            && subdesign.len() >= pattern.len()
                            && completes[subdesign.len() - pattern.len()]
                    });
                    completes
                })
                .last()
                .copied()
                .unwrap()
        })
        .count() as i32
}

pub fn solution_part_2(patterns: &[&str], designs: &[&str]) -> i64 {
    designs
        .iter()
        .map(|&design| {
            let counts = (0..=design.len())
                .map(|i| if i == 0 { 1 } else { 0 })
                .collect_vec();

            (1..=design.len())
                .map(|len| (len, &design.as_bytes()[..len]))
                .fold(counts, |mut counts, (i, subdesign)| {
                    counts[i] = patterns
                        .iter()
                        .flat_map(|&pattern| {
                            (subdesign.ends_with(pattern.as_bytes())
                                && subdesign.len() >= pattern.len())
                            .then(|| counts[subdesign.len() - pattern.len()])
                        })
                        .sum();
                    counts
                })
                .last()
                .copied()
                .unwrap()
        })
        .sum::<i64>()
}

fn main() {
    let input = input(19);

    let (patterns, designs) = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&patterns, &designs));
    println!("Part 2 solution: {}", solution_part_2(&patterns, &designs));
}
