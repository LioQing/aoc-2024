use std::collections::{HashSet, VecDeque};

use aoc_2024::input;
use glam::*;
use itertools::{FoldWhile, Itertools};

trait MapAt {
    type Item;

    fn at(&self, pos: IVec2) -> Option<&Self::Item>;
}

impl MapAt for &[&[u8]] {
    type Item = u8;

    fn at(&self, pos: IVec2) -> Option<&Self::Item> {
        self.get(usize::try_from(pos.y).ok()?)?
            .get(usize::try_from(pos.x).ok()?)
    }
}

impl<T> MapAt for Vec<Vec<T>> {
    type Item = T;

    fn at(&self, pos: IVec2) -> Option<&Self::Item> {
        self.get(usize::try_from(pos.y).ok()?)?
            .get(usize::try_from(pos.x).ok()?)
    }
}

trait Regions {
    fn regions(&self) -> Vec<HashSet<IVec2>>;
}

impl Regions for &[&[u8]] {
    fn regions(&self) -> Vec<HashSet<IVec2>> {
        self.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, &plant)| (ivec2(x as i32, y as i32), plant))
            })
            .fold(
                (Vec::new(), HashSet::new()),
                |(mut regions, mut visited), (pos, plant)| {
                    if visited.contains(&pos) {
                        return (regions, visited);
                    }

                    regions.push(
                        std::iter::repeat(())
                            .fold_while(
                                (VecDeque::from([pos]), HashSet::new()),
                                |(mut queue, mut visited), _| match queue.pop_front() {
                                    Some(pos) => {
                                        if visited.contains(&pos) {
                                            return FoldWhile::Continue((queue, visited));
                                        }

                                        visited.insert(pos);

                                        queue.extend(
                                            [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                                                .iter()
                                                .map(move |offset| pos + offset)
                                                .filter(|&pos| self.at(pos) == Some(&plant)),
                                        );

                                        FoldWhile::Continue((queue, visited))
                                    }
                                    _ => FoldWhile::Done((queue, visited)),
                                },
                            )
                            .into_inner()
                            .1,
                    );

                    visited.extend(regions.last().unwrap().iter().copied());

                    (regions, visited)
                },
            )
            .0
    }
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.trim().as_bytes()).collect()
}

fn solution_part_1(map: &[&[u8]]) -> i32 {
    map.regions()
        .into_iter()
        .map(|region| {
            let area = region.len() as i32;
            let perimeter = region
                .iter()
                .map(|&pos| {
                    [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                        .iter()
                        .map(move |offset| pos + offset)
                        .filter(|&pos| !region.contains(&pos))
                        .count()
                })
                .sum::<usize>() as i32;

            area * perimeter
        })
        .sum()
}

fn solution_part_2(map: &[&[u8]]) -> i32 {
    map.regions()
        .into_iter()
        .map(|region| {
            let area = region.len() as i32;
            let side_count = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                .into_iter()
                .map(|dir| {
                    let sides = region
                        .iter()
                        .filter(|&pos| !region.contains(&(pos + dir)))
                        .collect::<HashSet<_>>();

                    let cross_axis = match dir {
                        IVec2::X | IVec2::NEG_X => IVec2::Y,
                        IVec2::Y | IVec2::NEG_Y => IVec2::X,
                        _ => unreachable!(),
                    };

                    sides
                        .iter()
                        .map(|&pos| pos + cross_axis)
                        .filter(|&pos| !sides.contains(&pos))
                        .count() as i32
                })
                .sum::<i32>();

            area * side_count
        })
        .sum()
}

fn main() {
    let input = input(12);

    let map = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&map));
    println!("Part 2 solution: {}", solution_part_2(&map));
}
