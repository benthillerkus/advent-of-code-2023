use std::{
    collections::{BTreeMap, HashMap},
    sync::OnceLock,
};

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug)]
enum State {
    Number(usize),
    Part(usize),
    Junk,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Digit,
    Symbol,
    Gear,
    Junk,
}

impl From<u8> for Token {
    fn from(value: u8) -> Self {
        match value {
            b'*' => Self::Gear,
            b'.' | b'\n' => Self::Junk,
            _ if value.is_ascii_digit() => Self::Digit,
            _ => Self::Symbol,
        }
    }
}

fn vicinity(index: usize) -> impl Iterator<Item = Token> {
    let index = index as i64;
    let width = *WIDTH.get().unwrap();
    let len = INPUT.len() as i64;

    let neighbors = Box::new([
        index - width - 1,
        index - width,
        index - width + 1,
        index - 1,
        index,
        index + 1,
        index + width - 1,
        index + width,
        index + width + 1,
    ]);

    let neighbors: &'static [i64; 9] = Box::leak(neighbors);

    neighbors
        .iter()
        .copied()
        .filter(move |&index| index >= 0 && index < len)
        .map(move |index| Token::from(INPUT[index as usize]))
}

static WIDTH: OnceLock<i64> = OnceLock::new();

fn to_number(s: &[u8]) -> u64 {
    s.iter()
        .map(|&b| char::from(b))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn a() {
    let mut state = State::Junk;
    let mut sum = 0u64;

    for (index, token) in INPUT.iter().copied().map(Token::from).enumerate() {
        state = match (token, state) {
            // Inside the gutter
            (Token::Junk | Token::Symbol | Token::Gear, State::Junk) => State::Junk,
            // Can promote to part?
            (Token::Digit, State::Junk) => {
                if vicinity(index).any(|token| token == Token::Symbol || token == Token::Gear) {
                    State::Part(index)
                } else {
                    State::Number(index)
                }
            }
            (Token::Digit, State::Number(start)) => {
                if vicinity(index).any(|token| token == Token::Symbol || token == Token::Gear) {
                    State::Part(start)
                } else {
                    State::Number(start)
                }
            }
            // Inside a part
            (Token::Digit, State::Part(start)) => State::Part(start),
            // A number ends
            (Token::Junk | Token::Symbol | Token::Gear, State::Number(_)) => State::Junk,
            // A Part ends
            (Token::Junk | Token::Symbol | Token::Gear, State::Part(start)) => {
                let number = to_number(&INPUT[start.to_owned()..index]);
                sum += number;
                State::Junk
            }
        };
    }

    println!("{sum}");
}

fn main() {
    let width = INPUT
        .iter()
        .enumerate()
        .filter_map(|(i, &e)| if e == b'\n' { Some(i) } else { None })
        .next()
        .unwrap() as i64
        + 1;

    WIDTH.set(width).unwrap();

    a();
}
