use std::{ops::Range, str::FromStr};

use nom::Finish;
use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    from: Range<i64>,
    offset: i64,
}

impl Mapping {
    fn new(destination: i64, source: i64, range: i64) -> Self {
        Mapping {
            from: source..(source + range),
            offset: destination - source,
        }
    }

    fn try_map(&self, value: i64) -> Option<i64> {
        if self.from.contains(&value) {
            Some(value + self.offset)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Layer {
    mappings: Vec<Mapping>,
}

impl Layer {
    fn map(&self, value: i64) -> i64 {
        if let Some(mapped) = self
            .mappings
            .iter()
            .filter_map(|mapping| mapping.try_map(value))
            .next()
        {
            mapped
        } else {
            value
        }
    }
}

#[derive(Debug)]
struct Game {
    seeds: Vec<i64>,
    layers: Vec<Layer>,
}

impl FromStr for Game {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::error::Error;
        use parse::game;
        match game(s).finish() {
            Ok((_remaining, game)) => Ok(game),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

mod parse {
    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::{char, digit1, multispace1, space0, space1},
        combinator::{map, map_res},
        multi::{many1, separated_list1},
        sequence::{pair, preceded, terminated},
        IResult,
    };

    use crate::{Game, Layer, Mapping};

    fn seeds(input: &str) -> IResult<&str, Vec<i64>> {
        let (input, _) = tag("seeds: ")(input)?;
        separated_list1(char(' '), map(digit1, |s| str::parse(s).unwrap()))(input)
    }

    fn mapping(input: &str) -> IResult<&str, Mapping> {
        let (input, _) = multispace1(input)?;
        let (input, destination) = map_res(digit1, str::parse)(input)?;
        let (input, source) = map_res(preceded(char(' '), digit1), str::parse)(input)?;
        let (input, range) = map_res(preceded(char(' '), digit1), str::parse)(input)?;

        Ok((input, Mapping::new(destination, source, range)))
    }

    fn layer(input: &str) -> IResult<&str, Layer> {
        let (input, _) = multispace1(input)?;
        let (input, _) = pair(take_until("map:"), tag("map:"))(input)?;
        let (input, mappings) = many1(mapping)(input)?;

        Ok((input, Layer { mappings }))
    }

    pub fn game(input: &str) -> IResult<&str, Game> {
        let (input, seeds) = seeds(input)?;
        let (input, layers) = many1(layer)(input)?;

        Ok((input, Game { seeds, layers }))
    }
}

fn a() {
    let game = Game::from_str(INPUT).unwrap();

    let location = game
        .seeds
        .par_iter()
        .map(|&seed| {
            game.layers
                .iter()
                .fold(seed, |value, layer| layer.map(value))
        })
        .min()
        .unwrap();

    println!("{location}")
}

fn main() {
    a();
}
