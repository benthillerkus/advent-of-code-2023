use std::{cmp::max, ops, u16};

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

impl Game {
    fn min_set_of_cubes(&self) -> Round {
        if self.rounds.len() == 1 {
            return *self.rounds.first().unwrap();
        }
        self.rounds
            .iter()
            .copied()
            .reduce(|acc, curr| Round {
                red: max(acc.red, curr.red),
                green: max(acc.green, curr.green),
                blue: max(acc.blue, curr.blue),
            })
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

impl Round {
    fn pow(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
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
    map_res(delimited(tag("Game "), digit1, char(':')), |s: &str| {
        s.parse::<u8>()
    })(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = game_id(input)?;
    let (input, rounds) = rounds(input)?;

    Ok((input, Game { id, rounds }))
}

fn main() {
    let games = INPUT
        .par_lines()
        .map(game)
        .map(Finish::finish)
        .filter_map(Result::ok)
        .map(|(_, game)| game);

    let games2 = games.clone();

    //a
    let sum = games
        .filter_map(|game| {
            game.rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
                .then_some(game.id as u16)
        })
        .sum::<u16>();
    println!("{sum}");

    // b
    let sum = games2
        .map(|game| game.min_set_of_cubes())
        .map(|round| round.pow())
        .sum::<u32>();

    println!("{sum}");
}
