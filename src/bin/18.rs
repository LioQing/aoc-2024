use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_2024::input;
use glam::*;
use itertools::{FoldWhile, Itertools};
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::recognize,
    sequence::separated_pair, IResult,
};

#[derive(Debug)]
pub struct Space {
    pub positions: Vec<IVec2>,
    pub size: IVec2,
}

impl Space {
    pub fn at(&self, pos: IVec2) -> bool {
        pos.x < 0
            || pos.y < 0
            || pos.x >= self.size.x
            || pos.y >= self.size.y
            || self.positions.contains(&pos)
    }

    pub fn at_with_time(&self, pos: IVec2, time: usize) -> bool {
        pos.x < 0
            || pos.y < 0
            || pos.x >= self.size.x
            || pos.y >= self.size.y
            || self.positions[..time].contains(&pos)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instance {
    pub pos: IVec2,
    pub dist: i32,
    pub prev: Option<IVec2>,
}

impl Instance {
    pub fn new(pos: IVec2) -> Self {
        Self {
            pos,
            dist: 0,
            prev: None,
        }
    }

    pub fn with_distance(pos: IVec2, dist: i32) -> Self {
        Self {
            pos,
            dist,
            prev: None,
        }
    }

    pub fn with_prev(pos: IVec2, dist: i32, prev: IVec2) -> Self {
        Self {
            pos,
            dist,
            prev: Some(prev),
        }
    }
}

impl Ord for Instance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Instance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_input(input: &str) -> Vec<IVec2> {
    fn i32(input: &str) -> IResult<&str, i32> {
        let (input, n) = recognize(digit1)(input)?;

        Ok((input, n.parse().unwrap()))
    }

    fn pos(input: &str) -> IResult<&str, IVec2> {
        let (input, (x, y)) = separated_pair(i32, tag(","), i32)(input)?;

        Ok((input, ivec2(x, y)))
    }

    input
        .lines()
        .map(str::trim)
        .map(pos)
        .map(|pos| pos.unwrap().1)
        .collect()
}

pub fn solution_part_1(space: &Space, time: usize) -> i32 {
    std::iter::repeat(())
        .fold_while(
            (
                BinaryHeap::from([Instance::new(IVec2::ZERO)]),
                HashSet::new(),
                None,
            ),
            |(mut queue, mut visited, end), _| {
                let curr = match queue.pop() {
                    Some(curr) => curr,
                    None => return FoldWhile::Done((queue, visited, end)),
                };

                if visited.contains(&curr.pos) {
                    return FoldWhile::Continue((queue, visited, end));
                }

                visited.insert(curr.pos);

                if curr.pos == space.size - IVec2::ONE {
                    return FoldWhile::Done((queue, visited, Some(curr.dist)));
                }

                [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                    .iter()
                    .map(|&dir| curr.pos + dir)
                    .filter(|&next| !space.at_with_time(next, time) && !visited.contains(&next))
                    .for_each(|next| queue.push(Instance::with_distance(next, curr.dist + 1)));

                FoldWhile::Continue((queue, visited, end))
            },
        )
        .into_inner()
        .2
        .unwrap()
}

pub fn solution_part_2(space: &Space) -> IVec2 {
    let init_path = (0..space.size.x)
        .map(|x| IVec2::new(x, 0))
        .chain((0..space.size.y).map(|y| IVec2::new(space.size.x - 1, y)))
        .collect_vec();

    (0..space.positions.len())
        .fold_while((init_path, None), |(prev_path, blockage), time| {
            let new_pos = space.positions[time];

            if !prev_path.contains(&new_pos) {
                return FoldWhile::Continue((prev_path, blockage));
            }

            let (_, visited, end) = std::iter::repeat(())
                .fold_while(
                    (
                        BinaryHeap::from([Instance::new(IVec2::ZERO)]),
                        HashMap::<IVec2, Option<IVec2>>::new(),
                        None,
                    ),
                    |(mut queue, mut visited, end), _| {
                        let curr = match queue.pop() {
                            Some(curr) => curr,
                            None => return FoldWhile::Done((queue, visited, end)),
                        };

                        if visited.contains_key(&curr.pos) {
                            return FoldWhile::Continue((queue, visited, end));
                        }

                        visited.insert(curr.pos, curr.prev);

                        if curr.pos == space.size - IVec2::ONE {
                            return FoldWhile::Done((queue, visited, Some(curr.pos)));
                        }

                        [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                            .iter()
                            .map(|&dir| curr.pos + dir)
                            .filter(|&next| {
                                !space.at_with_time(next, time + 1) && !visited.contains_key(&next)
                            })
                            .for_each(|next| {
                                queue.push(Instance::with_prev(next, curr.dist + 1, curr.pos))
                            });

                        FoldWhile::Continue((queue, visited, end))
                    },
                )
                .into_inner();

            match end {
                Some(end) => {
                    let path =
                        std::iter::successors(Some(end), |&pos| visited.get(&pos).copied()?)
                            .collect_vec();
                    FoldWhile::Continue((path, blockage))
                }
                None => FoldWhile::Done((prev_path, Some(new_pos))),
            }
        })
        .into_inner()
        .1
        .unwrap()
}

fn main() {
    let (input, size, time) = (input(18), IVec2::splat(71), 1024);

    let positions = parse_input(&input);

    let space = Space { positions, size };

    println!("Part 1 solution: {}", solution_part_1(&space, time));
    println!("Part 2 solution: {}", {
        let solution = solution_part_2(&space);
        format!("{},{}", solution.x, solution.y)
    });
}
