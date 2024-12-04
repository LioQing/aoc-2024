use aoc_2024::input;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Mul,
    Num(i32),
    Comma,
    CloseBrac,
    Do,
    Dont,
    Fail,
}

type Parser<'a> = fn(&Tokenizer<'a>) -> Option<(usize, Token)>;

struct Tokenizer<'a> {
    source: &'a [u8], // Assuming only ASCII
    index: usize,
    parsers: Vec<Parser<'a>>,
}

impl Tokenizer<'_> {
    fn mul(&self) -> Option<(usize, Token)> {
        match self.source[self.index..]
            .iter()
            .zip("mul(".as_bytes())
            .all(|(a, b)| a == b)
        {
            true => Some(("mul(".len(), Token::Mul)),
            false => None,
        }
    }

    fn num(&self) -> Option<(usize, Token)> {
        match String::from_utf8(
            self.source[self.index..]
                .iter()
                .copied()
                .take_while(|c| c.is_ascii_digit())
                .collect::<Vec<_>>(),
        ) {
            Ok(n) if !n.is_empty() => Some((n.len(), Token::Num(n.parse::<i32>().unwrap()))),
            _ => None,
        }
    }

    fn comma(&self) -> Option<(usize, Token)> {
        match self.source[self.index] {
            b',' => Some((1, Token::Comma)),
            _ => None,
        }
    }

    fn close_brac(&self) -> Option<(usize, Token)> {
        match self.source[self.index] {
            b')' => Some((1, Token::CloseBrac)),
            _ => None,
        }
    }

    fn do_(&self) -> Option<(usize, Token)> {
        match self.source[self.index..]
            .iter()
            .zip("do()".as_bytes())
            .all(|(a, b)| a == b)
        {
            true => Some(("do()".len(), Token::Do)),
            false => None,
        }
    }

    fn dont(&self) -> Option<(usize, Token)> {
        match self.source[self.index..]
            .iter()
            .zip("don't()".as_bytes())
            .all(|(a, b)| a == b)
        {
            true => Some(("don't()".len(), Token::Dont)),
            false => None,
        }
    }

    fn fail(&self) -> Option<(usize, Token)> {
        Some((1, Token::Fail))
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.source.len() {
            return None;
        }

        match self.parsers.iter().find_map(|parser| parser(self)) {
            Some((delta, tok)) => {
                self.index += delta;
                Some(tok)
            }
            None => unreachable!(),
        }
    }
}

trait Tokenize<'a> {
    fn tokenize(self, parsers: Vec<Parser<'a>>) -> Tokenizer<'a>;
}

impl<'a> Tokenize<'a> for &'a [u8] {
    fn tokenize(self, parsers: Vec<Parser<'a>>) -> Tokenizer<'a> {
        Tokenizer {
            source: self,
            index: 0,
            parsers,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    Mul(i32, i32),
    Enable(bool),
}

fn solution_part_1(input: &str) -> i32 {
    input
        .as_bytes()
        .tokenize(vec![
            Tokenizer::mul,
            Tokenizer::num,
            Tokenizer::comma,
            Tokenizer::close_brac,
            Tokenizer::fail,
        ])
        .tuple_windows::<(_, _, _, _, _)>()
        .filter_map(|toks| match toks {
            (Token::Mul, Token::Num(a), Token::Comma, Token::Num(b), Token::CloseBrac) => {
                Some(a * b)
            }
            _ => None,
        })
        .sum()
}

fn solution_part_2(input: &str) -> i32 {
    input
        .as_bytes()
        .tokenize(vec![
            Tokenizer::mul,
            Tokenizer::num,
            Tokenizer::comma,
            Tokenizer::close_brac,
            Tokenizer::do_,
            Tokenizer::dont,
            Tokenizer::fail,
        ])
        .tuple_windows::<(_, _, _, _, _)>()
        .filter_map(|toks| match toks {
            (Token::Mul, Token::Num(a), Token::Comma, Token::Num(b), Token::CloseBrac) => {
                Some(Command::Mul(a, b))
            }
            (Token::Do, ..) => Some(Command::Enable(true)),
            (Token::Dont, ..) => Some(Command::Enable(false)),
            _ => None,
        })
        .fold((0, true), |(value, enabled), command| match command {
            Command::Mul(a, b) if enabled => (value + a * b, enabled),
            Command::Enable(enabled) => (value, enabled),
            _ => (value, enabled),
        })
        .0
}

fn main() {
    let input = input(3);

    println!("Part 1 solution: {}", solution_part_1(&input));
    println!("Part 2 solution: {}", solution_part_2(&input));
}
