use rayon::prelude::*;
use std::collections::BTreeSet;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Game {
    #[allow(dead_code)]
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
    fn matches(&self) -> u8 {
        self.winners.intersection(&self.mine).count() as u8
    }

    fn score(&self) -> u16 {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            1 << (matches - 1)
        }
    }
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, digit1, space0, space1},
        combinator::map_res,
        multi::separated_list1,
        sequence::{delimited, pair, tuple},
        IResult,
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

fn a() {
    let sum: u16 = INPUT
        .par_lines()
        .map(Game::from)
        .map(|game| game.score())
        .sum();

    println!("{sum}");
}

fn b() {
    let matches: Vec<_> = INPUT
        .par_lines()
        .map(Game::from)
        .map(|game| game.matches())
        .collect();

    let mut cards = Vec::from_iter(std::iter::repeat(1).take(matches.len()));

    for index in 0..matches.len() {
        let matches = matches[index];
        let instances = cards[index];
        for count in cards.iter_mut().skip(index + 1).take(matches as usize) {
            *count += instances;
        }
    }

    let sum: u32 = cards.iter().sum();

    println!("{sum}");
}

fn main() {
    a();
    b();
}
