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
struct GameA {
    seeds: Vec<i64>,
    layers: Vec<Layer>,
}

impl FromStr for GameA {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::error::Error;
        use parse_a::game;
        match game(s).finish() {
            Ok((_remaining, game)) => Ok(game),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug)]
struct GameB {
    seeds: Vec<Range<i64>>,
    layers: Vec<Layer>,
}

impl FromStr for GameB {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::error::Error;
        use parse_b::game;
        match game(s).finish() {
            Ok((_remaining, game)) => Ok(game),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

mod parse_a {

    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::{char, digit1, multispace1},
        combinator::{map, map_res},
        multi::{many1, separated_list1},
        sequence::{pair, preceded},
        IResult,
    };

    use crate::{GameA, Layer, Mapping};

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

    pub fn game(input: &str) -> IResult<&str, GameA> {
        let (input, seeds) = seeds(input)?;
        let (input, layers) = many1(layer)(input)?;

        Ok((input, GameA { seeds, layers }))
    }
}

mod parse_b {
    use std::ops::Range;

    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::{char, digit1, multispace1},
        combinator::{map, map_res},
        multi::{many1, separated_list1},
        sequence::{pair, preceded, separated_pair},
        IResult,
    };

    use crate::{GameB, Layer, Mapping};

    fn seeds(input: &str) -> IResult<&str, Vec<Range<i64>>> {
        let (input, _) = tag("seeds: ")(input)?;
        separated_list1(
            char(' '),
            map(
                separated_pair(digit1, char(' '), digit1),
                |(start, length)| {
                    let start = str::parse(start).unwrap();
                    let length: i64 = str::parse(length).unwrap();

                    start..(start + length)
                },
            ),
        )(input)
    }

    fn mapping(input: &str) -> IResult<&str, Mapping> {
        let (input, _) = multispace1(input)?;
        let (input, destination) = map_res(digit1, str::parse)(input)?;
        let (input, source) = map_res(preceded(char(' '), digit1), str::parse)(input)?;
        let (input, range) = map_res(preceded(char(' '), digit1), str::parse)(input)?;

        Ok((input, Mapping::new(source, destination, range)))
    }

    fn layer(input: &str) -> IResult<&str, Layer> {
        let (input, _) = multispace1(input)?;
        let (input, _) = pair(take_until("map:"), tag("map:"))(input)?;
        let (input, mappings) = many1(mapping)(input)?;

        Ok((input, Layer { mappings }))
    }

    // A smart person would've done a depth first search from the back
    pub fn game(input: &str) -> IResult<&str, GameB> {
        let (input, seeds) = seeds(input)?;
        let (input, layers) = many1(layer)(input)?;

        Ok((input, GameB { seeds, layers }))
    }
}

fn a() {
    let game = GameA::from_str(INPUT).unwrap();

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

fn b() {
    let game = GameB::from_str(INPUT).unwrap();

    let (location, _) = (0..i64::MAX)
        .map(|location| {
            (
                location,
                game.layers
                    .iter()
                    .rev()
                    .fold(location, |value, layer| layer.map(value)),
            )
        })
        .find(|(_, seed)| game.seeds.iter().any(|range| range.contains(seed)))
        .unwrap();

    println!("{location}")
}

fn main() {
    a();
    b();
}
