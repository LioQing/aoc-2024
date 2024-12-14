use aoc_2024::input;
use glam::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{opt, recognize},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Robot {
    p: IVec2,
    v: IVec2,
}

fn parse_input(input: &str) -> Vec<Robot> {
    fn i32(input: &str) -> IResult<&str, i32> {
        let (input, n) = recognize(preceded(opt(tag("-")), recognize(digit1)))(input)?;

        Ok((input, n.parse().unwrap()))
    }

    fn p(input: &str) -> IResult<&str, IVec2> {
        let (input, _) = tag("p=")(input)?;
        let (input, (x, y)) = separated_pair(i32, tag(","), i32)(input)?;

        Ok((input, ivec2(x, y)))
    }

    fn v(input: &str) -> IResult<&str, IVec2> {
        let (input, _) = tag("v=")(input)?;
        let (input, (x, y)) = separated_pair(i32, tag(","), i32)(input)?;

        Ok((input, ivec2(x, y)))
    }

    fn robot(input: &str) -> IResult<&str, Robot> {
        let (input, (p, v)) = separated_pair(p, multispace1, v)(input)?;

        Ok((input, Robot { p, v }))
    }

    let (_, robots) = separated_list0(multispace1, robot)(input).unwrap();

    robots
}

fn solution_part_1(robots: &[Robot], size: IVec2) -> i32 {
    let center = size / 2;

    robots
        .iter()
        .map(|robot| (robot.p + robot.v * 100).rem_euclid(size))
        .fold([0; 4], |mut quadrants, pos| {
            let quadrant = [(0..center.x), (center.x + 1..size.x)]
                .into_iter()
                .cartesian_product([(0..center.y), (center.y + 1..size.y)])
                .enumerate()
                .find_map(|(quadrant, (range_x, range_y))| {
                    match range_x.contains(&pos.x) && range_y.contains(&pos.y) {
                        true => Some(quadrant),
                        false => None,
                    }
                });

            if let Some(quadrant) = quadrant {
                quadrants[quadrant] += 1;
            }

            quadrants
        })
        .into_iter()
        .product()
}

fn solution_part_2(robots: &[Robot], size: IVec2) -> i32 {
    let (i, easter_egg) = (0..i32::MAX)
        .map(|i| {
            robots
                .iter()
                .map(|robot| (robot.p + robot.v * i).rem_euclid(size))
                .fold(
                    (0..size.y)
                        .map(|_| (0..size.x).map(|_| b' ').collect_vec())
                        .collect_vec(),
                    |mut map, pos| {
                        map[pos.y as usize][pos.x as usize] = b'X';
                        map
                    },
                )
        })
        .enumerate()
        .find(|(_, map)| {
            map.iter()
                .any(|row| row.windows(10).any(|c| c == [b'X'; 10]))
        })
        .unwrap();

    println!(
        "{}",
        easter_egg
            .iter()
            .flat_map(|row| std::str::from_utf8(row))
            .join("\n")
    );

    i as i32
}

fn main() {
    let (input, size) = (input(14), ivec2(101, 103));

    let robots = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&robots, size));
    println!("Part 2 solution: {}", solution_part_2(&robots, size));
}
