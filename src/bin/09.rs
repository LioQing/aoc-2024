use aoc_2024::input;
use glam::*;
use itertools::Itertools;

trait FileDigits {
    fn file_digits(self) -> impl Iterator<Item = Option<i64>>;
}

impl FileDigits for &str {
    fn file_digits(self) -> impl Iterator<Item = Option<i64>> {
        self.chars()
            .filter_map(|c| c.to_digit(10).map(|x| x as i64))
            .scan((true, 0), |(is_file, id), size| {
                let data = std::iter::repeat_n(
                    match is_file {
                        true => {
                            *id += 1;
                            Some(*id - 1)
                        }
                        false => None,
                    },
                    size as usize,
                );

                *is_file = !*is_file;

                Some(data)
            })
            .flatten()
    }
}

fn solution_part_1(input: &str) -> i64 {
    let digits = input.file_digits().collect_vec();

    digits
        .iter()
        .enumerate()
        .scan(
            digits
                .iter()
                .enumerate()
                .rev()
                .filter_map(|(j, b)| b.as_ref().copied().map(|b| (j, b)))
                .peekable(),
            |rev, (i, a)| {
                if matches!(rev.peek(), Some(&(j, _)) if j < i) {
                    return None;
                }

                a.or_else(|| rev.next().map(|(_, b)| b))
            },
        )
        .enumerate()
        .map(|(i, x)| i as i64 * x)
        .sum()
}

fn solution_part_2(input: &str) -> i64 {
    let files = input
        .file_digits()
        .chunk_by(|&x| x)
        .into_iter()
        .map(|(_, xs)| xs.collect_vec())
        .collect_vec();

    files
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(j, xs)| xs.first().unwrap().as_ref().map(|&x| (j, x, xs.len())))
        .fold(files.clone(), |mut files, (j, x, n)| {
            let i = files
                .iter_mut()
                .take(j)
                .find_position(|ys| ys.iter().skip_while(|y| y.is_some()).count() >= n)
                .map(|(i, _)| i);

            if let Some(i) = i {
                files.get_mut(j).unwrap().iter_mut().for_each(|y| *y = None);
                files
                    .get_mut(i)
                    .unwrap()
                    .iter_mut()
                    .skip_while(|y| y.is_some())
                    .take(n)
                    .for_each(|y| *y = Some(x));
            }

            files
        })
        .into_iter()
        .flat_map(|xs| xs.into_iter())
        .enumerate()
        .map(|(i, x)| i as i64 * x.unwrap_or(0))
        .sum()
}

fn main() {
    let input = input(9);

    println!("Part 1 solution: {}", solution_part_1(&input));
    println!("Part 2 solution: {}", solution_part_2(&input));
}
