use aoc_2024::input;
use itertools::Itertools;

struct Extractor<'a> {
    vec: Vec<&'a [u8]>,
}

impl Extractor<'_> {
    fn horizontal<const N: usize>(&self, row: usize, col: usize) -> Option<[u8; N]> {
        self.vec.get(row)?.get(col..col + N)?.try_into().ok()
    }

    fn vertical<const N: usize>(&self, row: usize, col: usize) -> Option<[u8; N]> {
        self.vec
            .get(row..row + N)?
            .iter()
            .filter_map(|row| row.get(col))
            .copied()
            .collect_vec()
            .try_into()
            .ok()
    }

    fn back_diag<const N: usize>(&self, row: usize, col: usize) -> Option<[u8; N]> {
        self.vec
            .get(row..row + N)?
            .iter()
            .enumerate()
            .filter_map(|(i, row)| row.get(col + i))
            .copied()
            .collect_vec()
            .try_into()
            .ok()
    }

    fn forward_diag<const N: usize>(&self, row: usize, col: usize) -> Option<[u8; N]> {
        self.vec
            .get(row..row + N)?
            .iter()
            .enumerate()
            .filter_map(|(i, row)| row.get(col + N - i - 1))
            .copied()
            .collect_vec()
            .try_into()
            .ok()
    }
}

impl<'a, I: IntoIterator<Item = &'a [u8]>> From<I> for Extractor<'a> {
    fn from(value: I) -> Self {
        Self {
            vec: value.into_iter().collect_vec(),
        }
    }
}

fn solution_part_1(input: &str) -> i32 {
    let width = input.lines().map(|l| l.len()).max().unwrap();
    let height = input.lines().count();

    (0..height)
        .cartesian_product(0..width)
        .scan(
            Extractor::from(input.lines().map(str::as_bytes)),
            |extractor, (row, col)| {
                [
                    Extractor::horizontal,
                    Extractor::vertical,
                    Extractor::back_diag,
                    Extractor::forward_diag,
                ]
                .iter()
                .filter(|extract| {
                    matches!(
                        extract(extractor, row, col).as_ref(),
                        Some(b"XMAS") | Some(b"SAMX")
                    )
                })
                .count()
                .into()
            },
        )
        .sum::<usize>() as i32
}

fn solution_part_2(input: &str) -> i32 {
    let width = input.lines().map(|l| l.len()).max().unwrap();
    let height = input.lines().count();

    (0..height)
        .cartesian_product(0..width)
        .scan(
            Extractor::from(input.lines().map(str::as_bytes)),
            |extractor, (row, col)| {
                [Extractor::back_diag, Extractor::forward_diag]
                    .iter()
                    .all(|extract| {
                        matches!(
                            extract(extractor, row, col).as_ref(),
                            Some(b"MAS") | Some(b"SAM")
                        )
                    })
                    .into()
            },
        )
        .filter(|x| *x)
        .count() as i32
}

fn main() {
    let input = input(4);

    println!("Part 1 solution: {}", solution_part_1(&input));
    println!("Part 2 solution: {}", solution_part_2(&input));
}
