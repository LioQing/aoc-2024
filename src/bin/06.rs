use std::collections::HashSet;

use aoc_2024::input;
use itertools::{FoldWhile, Itertools};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up = b'^',
    Right = b'>',
    Down = b'<',
    Left = b'v',
}

impl Dir {
    fn next(self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
        }
    }

    fn apply(self, (y, x): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::Up => match y {
                0 => None,
                _ => Some((y - 1, x)),
            },
            Self::Right => Some((y, x + 1)),
            Self::Down => Some((y + 1, x)),
            Self::Left => match x {
                0 => None,
                _ => Some((y, x - 1)),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Instance {
    dir: Dir,
    pos: (usize, usize),
}

impl Instance {
    fn new(pos: (usize, usize)) -> Self {
        Self { dir: Dir::Up, pos }
    }

    fn next_pos(self) -> Option<(usize, usize)> {
        self.dir.apply(self.pos)
    }

    fn turn(self) -> Self {
        Self {
            dir: self.dir.next(),
            ..self
        }
    }

    fn with_pos(self, pos: (usize, usize)) -> Self {
        Self { pos, ..self }
    }

    /// Returns the trace and the instance of repeat if any
    fn trace(
        self,
        map: &[Vec<u8>],
        obstruction: Option<(usize, usize)>,
    ) -> (HashSet<Instance>, Option<Instance>) {
        let width = map.first().unwrap().len();
        let height = map.len();

        let result = std::iter::repeat(())
            .fold_while(
                (HashSet::from([self]), self, None),
                |(mut trace, instance, _), _| {
                    let (y, x) = match instance.next_pos() {
                        Some((y, x)) if y < height && x < width => (y, x),
                        _ => return FoldWhile::Done((trace, instance, None)),
                    };

                    if let Some(obstruction) = obstruction {
                        if (y, x) == obstruction {
                            return FoldWhile::Continue((trace, instance.turn(), None));
                        }
                    }

                    FoldWhile::Continue(match map[y][x] {
                        b'#' => (trace, instance.turn(), None),
                        _ => {
                            let instance = instance.with_pos((y, x));

                            if trace.contains(&instance) {
                                return FoldWhile::Done((trace, instance, Some(instance)));
                            }

                            trace.insert(instance);
                            (trace, instance, None)
                        }
                    })
                },
            )
            .into_inner();

        (result.0, result.2)
    }
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn solution_part_1(map: &[Vec<u8>]) -> i32 {
    let pos = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| (y, x, cell)))
        .find_map(|(y, x, cell)| match cell {
            b'^' => Some((y, x)),
            _ => None,
        })
        .unwrap();

    Instance::new(pos)
        .trace(map, None)
        .0
        .into_iter()
        .map(|instance| instance.pos)
        .unique()
        .count() as i32
}

fn solution_part_2(map: &[Vec<u8>]) -> i32 {
    let pos = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, cell)| (y, x, cell)))
        .find_map(|(y, x, cell)| match cell {
            b'^' => Some((y, x)),
            _ => None,
        })
        .unwrap();

    Instance::new(pos)
        .trace(map, None)
        .0
        .into_iter()
        .map(|instance| instance.pos)
        .unique()
        .filter(|&obstruction| Instance::new(pos).trace(map, Some(obstruction)).1.is_some())
        .count() as i32
}

fn main() {
    let input = input(6);

    let map = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&map));
    println!("Part 2 solution: {}", solution_part_2(&map));
}
