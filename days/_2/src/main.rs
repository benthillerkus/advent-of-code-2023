use std::ops;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt},
    multi::{fold_many1, separated_list0},
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Game {
    id: u8,
    rounds: Vec<Round>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

impl ops::Add for Round {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

fn color(input: &str) -> IResult<&str, Round> {
    let count = map_res(digit1, str::parse::<u8>);
    let color = alt((tag("green"), tag("blue"), tag("red")));
    let (input, (count, color)) = separated_pair(count, char(' '), color)(input)?;

    Ok((
        input,
        match color {
            "green" => Round {
                green: count,
                red: 0,
                blue: 0,
            },
            "blue" => Round {
                green: 0,
                red: 0,
                blue: count,
            },
            "red" => Round {
                green: 0,
                red: count,
                blue: 0,
            },
            _ => panic!(),
        },
    ))
}

fn round(input: &str) -> IResult<&str, Round> {
    let color = delimited(char(' '), color, opt(char(',')));
    fold_many1(color, Round::default, |acc, curr| acc + curr)(input)
}

fn rounds(input: &str) -> IResult<&str, Vec<Round>> {
    separated_list0(char(';'), round)(input)
}

fn game_id(input: &str) -> IResult<&str, u8> {
    let (input, id) = map_res(delimited(tag("Game "), digit1, char(':')), |s: &str| {
        s.parse::<u8>()
    })(input)?;

    Ok((input, id))
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = game_id(input)?;
    let (_, rounds) = rounds(input)?;

    Ok((input, Game { id, rounds }))
}

fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let sum = INPUT
        .par_lines()
        .map(game)
        .map(Finish::finish)
        .filter_map(Result::ok)
        .filter_map(|(_, game)| {
            game.rounds
                .iter()
                .all(|round| {
                    round.red <= max_red && round.green <= max_green && round.blue <= max_blue
                })
                .then_some(game.id as u16)
        })
        .sum::<u16>();

    println!("{sum}");
}
