use std::collections::{HashSet, VecDeque};

use aoc_2024::input;
use glam::*;
use itertools::{FoldWhile, Itertools};

pub trait MapAt {
    fn at(&self, pos: IVec2) -> u8;
    fn set_at(&mut self, pos: IVec2, c: u8);
}

impl MapAt for Vec<Vec<u8>> {
    fn at(&self, pos: IVec2) -> u8 {
        self[pos.y as usize][pos.x as usize]
    }

    fn set_at(&mut self, pos: IVec2, c: u8) {
        self[pos.y as usize][pos.x as usize] = c;
    }
}

pub fn parse_input(input: &str) -> (Vec<Vec<u8>>, IVec2, Vec<IVec2>) {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut map = map
        .lines()
        .map(|row| row.trim().as_bytes().to_vec())
        .collect_vec();

    let pos = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| (ivec2(x as i32, y as i32), c))
        })
        .find(|&(_, c)| c == b'@')
        .map(|(pos, _)| pos)
        .unwrap();

    map.set_at(pos, b'.');

    let deltas = moves
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '^' => ivec2(0, -1),
            '>' => ivec2(1, 0),
            'v' => ivec2(0, 1),
            '<' => ivec2(-1, 0),
            _ => unreachable!(),
        })
        .collect_vec();

    (map, pos, deltas)
}

pub fn solution_part_1(map: &[Vec<u8>], pos: IVec2, deltas: &[IVec2]) -> i32 {
    let mut map = map.iter().map(|row| row.to_vec()).collect_vec();
    map.set_at(pos, b'.');

    deltas
        .iter()
        .fold((map, pos), |(mut map, pos), delta| {
            let (end, end_c) = (1..)
                .map(|step| pos + delta * step)
                .find(|&offset| matches!(map.at(offset), b'#' | b'.'))
                .map(|end| (end, map.at(end)))
                .unwrap();

            if end_c == b'#' {
                return (map, pos);
            }

            (1..)
                .map(|step| pos + delta * step)
                .take_while(|&offset| offset - delta != end)
                .for_each(|offset| {
                    map.set_at(offset, if offset == pos + delta { b'.' } else { b'O' })
                });

            (map, pos + delta)
        })
        .0
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(x, c)| (ivec2(x as i32, y as i32), c))
        })
        .filter(|&(_, c)| c == b'O')
        .map(|(pos, _)| pos.y * 100 + pos.x)
        .sum()
}

pub fn solution_part_2(map: &[Vec<u8>], pos: IVec2, deltas: &[IVec2]) -> i32 {
    let mut map = map
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|&c| if c == b'O' { [b'[', b']'] } else { [c, c] })
                .collect_vec()
        })
        .collect_vec();

    let pos = pos * ivec2(2, 1);

    map.set_at(pos, b'.');

    deltas
        .iter()
        .fold((map, pos), |(mut map, pos), delta| {
            if delta.x != 0 {
                let (end, end_c) = (1..)
                    .map(|step| pos + delta * step)
                    .find(|&offset| matches!(map.at(offset), b'#' | b'.'))
                    .map(|end| (end, map.at(end)))
                    .unwrap();

                if end_c == b'#' {
                    return (map, pos);
                }

                (1..)
                    .map(|step| pos + delta * step)
                    .take_while(|&offset| offset - delta != end)
                    .collect_vec()
                    .into_iter()
                    .rev()
                    .for_each(|offset| map.set_at(offset, map.at(offset - delta)));
            } else {
                let (_, boxes, blocked) = std::iter::repeat(())
                    .fold_while(
                        (VecDeque::from([pos]), HashSet::new(), false),
                        |(mut queue, mut visited, blocked), _| {
                            let curr = match queue.pop_front() {
                                Some(curr) => curr,
                                None => return FoldWhile::Done((queue, visited, blocked)),
                            };

                            if visited.contains(&curr) {
                                return FoldWhile::Continue((queue, visited, blocked));
                            }

                            match map.at(curr) {
                                b'[' => {
                                    queue.push_back(curr + IVec2::X);
                                    visited.insert(curr);
                                }
                                b']' => queue.push_back(curr - IVec2::X),
                                _ => {}
                            }

                            match map.at(curr + delta) {
                                b'[' | b']' => queue.push_back(curr + delta),
                                b'#' => return FoldWhile::Done((queue, visited, true)),
                                _ => {}
                            }

                            FoldWhile::Continue((queue, visited, blocked))
                        },
                    )
                    .into_inner();

                if blocked {
                    return (map, pos);
                }

                boxes.iter().for_each(|&b| {
                    map.set_at(b, b'.');
                    map.set_at(b + IVec2::X, b'.');
                });

                boxes.into_iter().map(|b| b + delta).for_each(|b| {
                    map.set_at(b, b'[');
                    map.set_at(b + IVec2::X, b']');
                });
            }

            (map, pos + delta)
        })
        .0
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(x, c)| (ivec2(x as i32, y as i32), c))
        })
        .filter(|&(_, c)| c == b'[')
        .map(|(pos, _)| pos.y * 100 + pos.x)
        .sum()
}

fn main() {
    let input = input(15);

    let (map, pos, deltas) = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&map, pos, &deltas));
    println!("Part 2 solution: {}", solution_part_2(&map, pos, &deltas));
}
