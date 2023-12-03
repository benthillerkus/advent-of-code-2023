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
    Junk,
}

impl From<u8> for Token {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Junk,
            _ if value.is_ascii_digit() => Self::Digit,
            _ => Self::Symbol,
        }
    }
}

fn main() {
    let width = INPUT
        .iter()
        .enumerate()
        .filter_map(|(i, &e)| if e == b'\n' { Some(i) } else { None })
        .next()
        .unwrap() as i64
        + 1;

    let len = INPUT.len() as i64;
    let is_symbol_in_vicinity = |index: usize| -> bool {
        let index = index as i64;
        let neighbors = [
            index - width - 1,
            index - width,
            index - width + 1,
            index - 1,
            index,
            index + 1,
            index + width - 1,
            index + width,
            index + width + 1,
        ];

        let mut indexable = neighbors
            .iter()
            .copied()
            .filter(|&index| index >= 0 && index < len);

        // println!(
        //     "{}",
        //     indexable
        //         .clone()
        //         .map(|index| char::from(INPUT[index as usize]))
        //         .collect::<String>()
        // );
        indexable.any(|index| Token::from(INPUT[index as usize]) == Token::Symbol)
    };

    let to_number = |s: &[u8]| -> u32 {
        s.iter()
            .map(|&b| char::from(b))
            .collect::<String>()
            .parse()
            .unwrap()
    };

    let mut state = State::Junk;
    let mut sum = 0u32;

    for (index, token) in INPUT.iter().copied().map(Token::from).enumerate() {
        state = match (token, state) {
            // Inside the gutter
            (Token::Junk | Token::Symbol, State::Junk) => State::Junk,
            // Can promote to part?
            (Token::Digit, State::Junk) => {
                if is_symbol_in_vicinity(index) {
                    State::Part(index)
                } else {
                    State::Number(index)
                }
            }
            (Token::Digit, State::Number(start)) => {
                if is_symbol_in_vicinity(index) {
                    State::Part(start)
                } else {
                    State::Number(start)
                }
            }
            // Inside a part
            (Token::Digit, State::Part(start)) => State::Part(start),
            // A number ends
            (Token::Junk | Token::Symbol, State::Number(_)) => State::Junk,
            // A Part ends
            (Token::Junk | Token::Symbol, State::Part(start)) => {
                let number = to_number(&INPUT[start.to_owned()..index]);
                println!("{number}");
                sum += number;
                State::Junk
            }
        };
    }

    println!("{sum}")
}
