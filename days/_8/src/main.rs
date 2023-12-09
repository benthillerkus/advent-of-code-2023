use std::{collections::HashMap, error::Error, fmt::Display};

use nom::Finish;
use parse::row;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    L,
    R,
}

#[derive(Debug)]
enum ErrorKind {
    NotAnInstruction(char),
    NoInstructionsFound,
    CouldntParseInstructions,
    NoEmptyLineAfterInstructions,
    CouldntParseNodes,
}

#[derive(Debug)]
struct ParseError {
    kind: ErrorKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.kind))
    }
}

impl Error for ParseError {}

impl TryFrom<char> for Instruction {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' | 'l' => Ok(Self::L),
            'R' | 'r' => Ok(Self::R),
            _ => Err(ParseError {
                kind: ErrorKind::NotAnInstruction(value),
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Node([char; 3]);

#[derive(Debug, PartialEq, Eq)]
struct Crossroad(Node, Node);

impl Crossroad {
    fn turn(&self, instruction: &Instruction) -> Node {
        match instruction {
            Instruction::L => self.0,
            Instruction::R => self.1,
        }
    }
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{anychar, char},
        combinator::{map, map_res},
        multi::many1,
        sequence::{delimited, separated_pair, tuple},
        IResult,
    };

    use super::*;

    pub fn node(input: &str) -> IResult<&str, Node> {
        map(tuple((anychar, anychar, anychar)), |chars| {
            Node([chars.0, chars.1, chars.2])
        })(input)
    }

    pub fn crossroad(input: &str) -> IResult<&str, Crossroad> {
        let (input, (left, right)) =
            delimited(char('('), separated_pair(node, tag(", "), node), char(')'))(input)?;

        Ok((input, Crossroad(left, right)))
    }

    pub fn row(input: &str) -> IResult<&str, (Node, Crossroad)> {
        let (input, node) = node(input)?;
        let (input, _) = tag(" = ")(input)?;
        let (input, crossroad) = crossroad(input)?;

        Ok((input, (node, crossroad)))
    }

    pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        many1(map_res(anychar, Instruction::try_from))(input)
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<Node, Crossroad>,
}

impl Map {
    fn steps(&self, from: &Node, to: &Node) -> usize {
        if from == to {
            return 0;
        }
        let mut current = *from;
        for (steps, instruction) in self.instructions.iter().cycle().enumerate() {
            let crossroad = &self.nodes[&current];
            current = crossroad.turn(instruction);
            if current == *to {
                return steps + 1;
            }
        }
        usize::MAX
    }
}

impl TryFrom<&str> for Map {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut lines = s.lines();
        let instructions = lines.next().map_or(
            Err(ParseError {
                kind: ErrorKind::NoInstructionsFound,
            }),
            |input| {
                parse::instructions(input)
                    .finish()
                    .map(|(_, instructions)| instructions)
                    .map_err(|_| ParseError {
                        kind: ErrorKind::CouldntParseInstructions,
                    })
            },
        )?;
        lines.next().ok_or(ParseError {
            kind: ErrorKind::NoEmptyLineAfterInstructions,
        })?;
        let mut nodes = HashMap::new();
        for thing in lines.map(|line| {
            let row = row(line).finish();
            match row {
                Ok((_, (node, crossroad))) => Ok((node, crossroad)),
                Err(_) => Err(ParseError {
                    kind: ErrorKind::CouldntParseNodes,
                }),
            }
        }) {
            let (node, crossroad) = thing?;
            nodes.insert(node, crossroad);
        }

        Ok(Map {
            instructions,
            nodes,
        })
    }
}

fn a(input: &str) {
    let map: Map = input.try_into().expect("the input to be parseable");
    let steps = map.steps(&Node(['A', 'A', 'A']), &Node(['Z', 'Z', 'Z']));

    println!("{steps}")
}

fn main() {
    a(INPUT)
}
