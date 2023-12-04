use std::collections::{BTreeSet, HashSet};

use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Game {
    id: u8,
    winners: BTreeSet<u8>,
    mine: BTreeSet<u8>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        use nom::Finish;
        use parse::game;

        game(value).finish().map(|(_, game)| game).unwrap()
    }
}

impl Game {
    fn score(&self) -> u16 {
        let matches = self.winners.intersection(&self.mine).count();
        if matches == 0 {
            0
        } else {
            1 << (matches - 1)
        }
    }
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, digit1, space0, space1},
        combinator::{map_res, opt},
        multi::{fold_many1, separated_list0, separated_list1},
        sequence::{delimited, pair, preceded, separated_pair, tuple},
        Finish, IResult,
    };

    use crate::Game;

    pub fn game(input: &str) -> IResult<&str, Game> {
        let (input, id) = map_res(
            delimited(pair(tag("Card"), space0), digit1, char(':')),
            |s: &str| s.parse::<u8>(),
        )(input)?;
        let (input, _) = space1(input)?;
        let (input, winners) = separated_list1(space1, digit1)(input)?;
        let (input, _) = tuple((space0, char('|'), space0))(input)?;
        let (input, mine) = separated_list1(space1, digit1)(input)?;

        let winners = winners.iter().map(|s| s.parse::<u8>().unwrap()).collect();
        let mine = mine.iter().map(|s| s.parse::<u8>().unwrap()).collect();

        Ok((input, Game { id, winners, mine }))
    }
}

fn main() {
    let sum: u16 = INPUT
        .par_lines()
        .map(Game::from)
        .map(|game| game.score())
        .sum();

    println!("{sum}");
}
