use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_2024::input;
use glam::*;
use itertools::{FoldWhile, Itertools};

pub trait MapAt {
    fn at(&self, pos: IVec2) -> u8;
}

impl MapAt for &[&[u8]] {
    fn at(&self, pos: IVec2) -> u8 {
        self[pos.y as usize][pos.x as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dir {
    X = 0,
    Y = 1,
    NegX = 2,
    NegY = 3,
}

impl Dir {
    pub fn from_u8(value: u8) -> Self {
        match value.rem_euclid(4) {
            0 => Self::X,
            1 => Self::Y,
            2 => Self::NegX,
            3 => Self::NegY,
            _ => unreachable!(),
        }
    }

    pub fn turns(self) -> [Self; 2] {
        let value = self as u8;
        [value.wrapping_sub(1), value.wrapping_add(1)].map(Self::from_u8)
    }
}

impl From<Dir> for IVec2 {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::X => IVec2::X,
            Dir::Y => IVec2::Y,
            Dir::NegX => IVec2::NEG_X,
            Dir::NegY => IVec2::NEG_Y,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instance {
    pub score: i32,
    pub pos: IVec2,
    pub dir: Dir,
    pub prev: Option<Dir>,
}

impl Instance {
    pub fn new(score: i32, pos: IVec2, dir: Dir) -> Self {
        Self {
            score,
            pos,
            dir,
            prev: None,
        }
    }

    pub fn with_prev(score: i32, pos: IVec2, dir: Dir, prev: Dir) -> Self {
        Self {
            score,
            pos,
            dir,
            prev: Some(prev),
        }
    }
}

impl Ord for Instance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Instance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_input(input: &str) -> (Vec<&[u8]>, IVec2) {
    let map = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();

    let pos = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| (ivec2(x as i32, y as i32), c))
        })
        .find(|&(_, c)| c == b'S')
        .map(|(pos, _)| pos)
        .unwrap();

    (map, pos)
}

pub fn solution_part_1(map: &[&[u8]], pos: IVec2) -> i32 {
    std::iter::repeat(())
        .fold_while(
            (
                BinaryHeap::from([Instance::new(0, pos, Dir::X)]),
                HashSet::new(),
                None,
            ),
            |(mut queue, mut visited, score), _| {
                let curr = queue.pop().unwrap();

                if visited.contains(&curr.pos) {
                    return FoldWhile::Continue((queue, visited, score));
                }

                if map.at(curr.pos) == b'E' {
                    return FoldWhile::Done((queue, visited, Some(curr.score)));
                }

                let neighbors = curr
                    .dir
                    .turns()
                    .into_iter()
                    .map(|dir| (1000, dir))
                    .chain(std::iter::once((0, curr.dir)))
                    .map(|(score, dir)| (score + 1, dir, IVec2::from(dir)))
                    .flat_map(|(score, dir, delta)| {
                        let next_pos = curr.pos + delta;
                        let next_score = curr.score + score;

                        if map.at(next_pos) == b'#' {
                            return None;
                        }

                        Some(Instance::new(next_score, next_pos, dir))
                    });

                queue.extend(neighbors);
                visited.insert(curr.pos);

                FoldWhile::Continue((queue, visited, score))
            },
        )
        .into_inner()
        .2
        .unwrap()
}

pub fn solution_part_2(map: &[&[u8]], pos: IVec2) -> i32 {
    let (_, visited, end) = std::iter::repeat(())
        .fold_while(
            (
                BinaryHeap::from([Instance::new(0, pos, Dir::X)]),
                HashMap::<(IVec2, Dir), (i32, Vec<(IVec2, Dir)>)>::new(),
                None,
            ),
            |(mut queue, mut visited, mut end), _| {
                let curr = match queue.pop() {
                    Some(curr) => curr,
                    None => return FoldWhile::Done((queue, visited, end)),
                };

                if let Some((score, prevs)) = visited.get_mut(&(curr.pos, curr.dir)) {
                    if *score == curr.score {
                        prevs.push((curr.pos - IVec2::from(curr.dir), curr.prev.unwrap()));
                    }
                    return FoldWhile::Continue((queue, visited, end));
                }

                if map.at(curr.pos) != b'E' {
                    let neighbors = curr
                        .dir
                        .turns()
                        .into_iter()
                        .map(|dir| (1000, dir))
                        .chain(std::iter::once((0, curr.dir)))
                        .map(|(score, dir)| (score + 1, dir, IVec2::from(dir)))
                        .flat_map(|(score, dir, delta)| {
                            let next_pos = curr.pos + delta;
                            let next_score = curr.score + score;

                            if map.at(next_pos) == b'#' {
                                return None;
                            }

                            Some(Instance::with_prev(next_score, next_pos, dir, curr.dir))
                        });

                    queue.extend(neighbors);
                }

                visited.insert(
                    (curr.pos, curr.dir),
                    (
                        curr.score,
                        curr.prev
                            .map(|prev| (curr.pos - IVec2::from(curr.dir), prev))
                            .into_iter()
                            .collect_vec(),
                    ),
                );

                if map.at(curr.pos) == b'E' {
                    end = Some(curr.pos);
                }

                FoldWhile::Continue((queue, visited, end))
            },
        )
        .into_inner();

    let backtrack_start = [Dir::X, Dir::NegY].map(|dir| (end.unwrap(), dir)).to_vec();

    let min_score = backtrack_start
        .iter()
        .map(|&(pos, dir)| visited.get(&(pos, dir)).unwrap().0)
        .min()
        .unwrap();

    let backtrack_start = backtrack_start
        .into_iter()
        .filter(|&(pos, dir)| visited.get(&(pos, dir)).unwrap().0 == min_score)
        .collect_vec();

    std::iter::repeat(())
        .fold_while(
            (
                HashSet::from_iter(backtrack_start),
                HashSet::from([end.unwrap()]),
            ),
            |(next, paths), _| {
                let next: HashSet<_> = next
                    .into_iter()
                    .flat_map(|(pos, dir)| visited.get(&(pos, dir)))
                    .flat_map(|(_, prevs)| prevs)
                    .copied()
                    .collect();

                if next.is_empty() {
                    return FoldWhile::Done((next, paths));
                }

                let paths = paths
                    .union(&next.iter().map(|&(pos, _)| pos).collect())
                    .copied()
                    .collect();

                FoldWhile::Continue((next, paths))
            },
        )
        .into_inner()
        .1
        .len() as i32
}

fn main() {
    let input = input(16);

    let (map, pos) = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&map, pos));
    println!("Part 2 solution: {}", solution_part_2(&map, pos));
}
