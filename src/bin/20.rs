use std::collections::VecDeque;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathMapNode {
    Wall,
    Path(PathMapPathNode),
}

impl PathMapNode {
    pub fn path(&self) -> Option<PathMapPathNode> {
        match self {
            Self::Wall => None,
            Self::Path(path) => Some(*path),
        }
    }

    pub fn path_mut(&mut self) -> Option<&mut PathMapPathNode> {
        match self {
            Self::Wall => None,
            Self::Path(path) => Some(path),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathMapPathNode {
    Cost(i32),
    NotVisited,
}

impl PathMapPathNode {
    pub fn cost(&self) -> Option<i32> {
        match self {
            Self::Cost(cost) => Some(*cost),
            Self::NotVisited => None,
        }
    }
}

#[derive(Debug)]
pub struct PathMap {
    path_map: Vec<Vec<PathMapNode>>,
    start: IVec2,
}

impl PathMap {
    pub fn new(map: &[Vec<u8>], start: IVec2) -> Self {
        let mut path_map =
            vec![vec![PathMapNode::Path(PathMapPathNode::NotVisited); map[0].len()]; map.len()];

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == b'#' {
                    path_map[y][x] = PathMapNode::Wall;
                }
            }
        }

        path_map[start.y as usize][start.x as usize] = PathMapNode::Path(PathMapPathNode::Cost(0));

        Self { path_map, start }
    }

    pub fn completed(map: &[Vec<u8>], start: IVec2) -> Self {
        let mut path_map = Self::new(map, start);
        path_map.complete();
        path_map
    }

    pub fn wall_at(&self, pos: IVec2) -> bool {
        self.path_map[pos.y as usize][pos.x as usize] == PathMapNode::Wall
    }

    pub fn path_at(&self, pos: IVec2) -> Option<PathMapPathNode> {
        self.path_map[pos.y as usize][pos.x as usize].path()
    }

    pub fn path_at_mut(&mut self, pos: IVec2) -> Option<&mut PathMapPathNode> {
        self.path_map[pos.y as usize][pos.x as usize].path_mut()
    }

    pub fn cost_at(&self, pos: IVec2) -> Option<i32> {
        self.path_map[pos.y as usize][pos.x as usize]
            .path()
            .and_then(|path| path.cost())
    }

    pub fn set_cost_at(&mut self, pos: IVec2, cost: i32) {
        if let PathMapNode::Path(c) = &mut self.path_map[pos.y as usize][pos.x as usize] {
            *c = PathMapPathNode::Cost(cost);
        }
    }

    pub fn complete(&mut self) {
        std::iter::repeat(()).fold_while(VecDeque::from([self.start]), |mut queue, _| {
            let curr = match queue.pop_front() {
                Some(curr) => curr,
                None => return FoldWhile::Done(queue),
            };

            [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                .into_iter()
                .map(|dir| curr + dir)
                .for_each(|pos| {
                    if self.wall_at(pos) || self.cost_at(pos).is_some() {
                        return;
                    }

                    self.set_cost_at(pos, self.cost_at(curr).unwrap() + 1);
                    queue.push_back(pos);
                });

            FoldWhile::Continue(queue)
        });
    }
}

pub fn parse_input(input: &str) -> (Vec<&[u8]>, IVec2, IVec2) {
    let map = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();

    let start = map
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

    let end = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &c)| (ivec2(x as i32, y as i32), c))
        })
        .find(|&(_, c)| c == b'E')
        .map(|(pos, _)| pos)
        .unwrap();

    (map, start, end)
}

pub fn solution_part_1(map: &[&[u8]], start: IVec2, end: IVec2, threshold: i32) -> i32 {
    let map = map.iter().map(|&row| row.to_vec()).collect_vec();
    let start_paths = PathMap::completed(&map, start);
    let end_paths = PathMap::completed(&map, end);

    let width = map[0].len();
    let height = map.len();

    let min_cost = start_paths.cost_at(end).unwrap();

    (0..height)
        .cartesian_product(0..width)
        .map(|(y, x)| ivec2(x as i32, y as i32))
        .filter(|&pos| start_paths.wall_at(pos))
        .map(|pos| {
            [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
                .into_iter()
                .map(move |dir| pos + dir)
                .filter(|&pos| {
                    (0..height).contains(&(pos.y as usize))
                        && (0..width).contains(&(pos.x as usize))
                })
                .filter(|&pos| !start_paths.wall_at(pos))
                .tuple_combinations()
                .map(|(a, b)| {
                    (start_paths.cost_at(a).unwrap() + end_paths.cost_at(b).unwrap())
                        .min(end_paths.cost_at(a).unwrap() + start_paths.cost_at(b).unwrap())
                        + 2
                })
                .map(|cost| min_cost - cost)
                .max()
                .unwrap_or(0)
        })
        .filter(|&save| save >= threshold)
        .count() as i32
}

pub fn solution_part_2(map: &[&[u8]], start: IVec2, end: IVec2, threshold: i32) -> i32 {
    const CHEAT_COUNT: i32 = 20;

    let map = map.iter().map(|&row| row.to_vec()).collect_vec();
    let start_paths = PathMap::completed(&map, start);
    let end_paths = PathMap::completed(&map, end);

    let width = map[0].len();
    let height = map.len();

    let min_cost = start_paths.cost_at(end).unwrap();

    (0..height)
        .cartesian_product(0..width)
        .map(|(y, x)| ivec2(x as i32, y as i32))
        .filter(|&pos| !start_paths.wall_at(pos))
        .tuple_combinations()
        .map(|(a, b)| (a, b, (a.x - b.x).abs() + (a.y - b.y).abs()))
        .filter(|&(_, _, cost)| cost <= CHEAT_COUNT)
        .map(|(a, b, cost)| {
            (start_paths.cost_at(a).unwrap() + end_paths.cost_at(b).unwrap())
                .min(end_paths.cost_at(a).unwrap() + start_paths.cost_at(b).unwrap())
                + cost
        })
        .map(|cost| min_cost - cost)
        .filter(|&save| save >= threshold)
        .count() as i32
}

fn main() {
    let (input, threshold) = (input(20), 100);

    let (map, start, end) = parse_input(&input);

    println!(
        "Part 1 solution: {}",
        solution_part_1(&map, start, end, threshold)
    );
    println!(
        "Part 2 solution: {}",
        solution_part_2(&map, start, end, threshold)
    );
}
