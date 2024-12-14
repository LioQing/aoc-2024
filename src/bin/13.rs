use aoc_2024::input;
use glam::*;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{digit1, multispace1},
    combinator::{map_res, recognize},
    multi::separated_list0,
    IResult,
};

#[derive(Debug)]
struct Button {
    cost: i32,
    delta: IVec2,
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    prize: IVec2,
}

fn parse_input(input: &str) -> Vec<Machine> {
    fn i32(input: &str) -> IResult<&str, i32> {
        let (input, n) = map_res(recognize(digit1), |s: &str| s.parse::<i32>())(input)?;

        Ok((input, n))
    }

    fn delta(input: &str) -> IResult<&str, IVec2> {
        let (input, _) = tag("X+")(input)?;
        let (input, x) = i32(input)?;
        let (input, _) = tag(", ")(input)?;
        let (input, _) = tag("Y+")(input)?;
        let (input, y) = i32(input)?;

        Ok((input, ivec2(x, y)))
    }

    fn button(input: &str) -> IResult<&str, Button> {
        let (input, _) = tag("Button ")(input)?;
        let (input, cost) = map_res(
            take_while_m_n(1, 1, |c| c == 'A' || c == 'B'),
            |s| match s {
                "A" => Ok(3),
                "B" => Ok(1),
                _ => Err(s),
            },
        )(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, delta) = delta(input)?;

        Ok((input, Button { cost, delta }))
    }

    fn prize(input: &str) -> IResult<&str, IVec2> {
        let (input, _) = tag("Prize: ")(input)?;
        let (input, _) = tag("X=")(input)?;
        let (input, x) = i32(input)?;
        let (input, _) = tag(", ")(input)?;
        let (input, _) = tag("Y=")(input)?;
        let (input, y) = i32(input)?;

        Ok((input, ivec2(x, y)))
    }

    fn machine(input: &str) -> IResult<&str, Machine> {
        let (input, a) = button(input)?;
        let (input, _) = multispace1(input)?;
        let (input, b) = button(input)?;
        let (input, _) = multispace1(input)?;
        let (input, prize) = prize(input)?;

        Ok((input, Machine { a, b, prize }))
    }

    let (_, machines) = separated_list0(multispace1, machine)(input).unwrap();

    machines
}

fn solution_part_1(machines: &[Machine]) -> i32 {
    machines
        .iter()
        .map(|machine| {
            match (0..)
                .map(|step| (step, machine.a.delta * step))
                .take_while(|(_, offset_a)| offset_a.cmple(machine.prize).all())
                .find_map(|(step_a, offset_a)| {
                    let offset_rem = machine.prize - offset_a;
                    match offset_rem % machine.b.delta {
                        IVec2::ZERO => match offset_rem / machine.b.delta {
                            step_b if step_b.x == step_b.y => Some((step_a, step_b.x)),
                            _ => None,
                        },
                        _ => None,
                    }
                }) {
                Some((step_a, step_b)) => step_a * machine.a.cost + step_b * machine.b.cost,
                None => 0,
            }
        })
        .sum()
}

fn solution_part_2(machines: &[Machine]) -> i64 {
    machines
        .iter()
        .map(|machine| {
            let p = machine.prize.as_i64vec2() + I64Vec2::splat(10000000000000);
            let a = machine.a.delta.as_i64vec2();
            let b = machine.b.delta.as_i64vec2();

            if p % b == I64Vec2::ZERO && matches!(p / b, coeff if coeff.x == coeff.y) {
                return p.x / b.x * machine.b.cost as i64;
            }

            let det = a.perp_dot(b);

            if det == 0 {
                if p % a == I64Vec2::ZERO && matches!(p / a, coeff if coeff.x == coeff.y) {
                    return p.x / a.x * machine.a.cost as i64;
                } else {
                    return 0;
                }
            }

            let c = -b * p.perp_dot(a);

            if c % det != I64Vec2::ZERO {
                return 0;
            }

            let offset_b = c / det;
            let offset_a = p - offset_b;

            if offset_a % a == I64Vec2::ZERO && offset_b % b == I64Vec2::ZERO {
                offset_a.x / a.x * machine.a.cost as i64 + offset_b.x / b.x * machine.b.cost as i64
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = input(13);

    let machines = parse_input(&input);

    println!("Part 1 solution: {}", solution_part_1(&machines));
    println!("Part 2 solution: {}", solution_part_2(&machines));
}
