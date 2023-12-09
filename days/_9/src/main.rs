use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res, opt, recognize},
    multi::{many1, separated_list1},
    sequence::preceded,
    Finish, IResult,
};
use rayon::{iter::ParallelIterator, str::ParallelString};

const INPUT: &str = include_str!("input.txt");

// from https://stackoverflow.com/a/74809016
fn integer(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(char('-')), digit1)), |s| {
        str::parse(s)
    })(input)?;

    Ok((i, number))
}

fn history(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(char(' '), integer)(input)
}

fn a(input: &str) -> i32 {
    input
        .par_lines()
        .map(|line| {
            history(line)
                .finish()
                .map(|(_, history)| history)
                .expect("should not be empty 22")
        })
        .map(|history| {
            let mut derivatives = vec![history];

            while derivatives.last().unwrap().iter().any(|&i| i != 0) {
                let derivative = derivatives
                    .last()
                    .expect("should not be empty 30")
                    .windows(2)
                    .map(|l| l.last().expect("should not be empty 32") - l.first().unwrap())
                    .collect::<Vec<_>>();

                derivatives.push(derivative)
            }

            derivatives
                .iter()
                .rev()
                .map(|derivative| derivative.last().expect("should not be empty 41"))
                .sum::<i32>()
        })
        .sum()
}

fn b(input: &str) -> i32 {
    input
        .par_lines()
        .map(|line| {
            history(line)
                .finish()
                .map(|(_, history)| history)
                .expect("should not be empty 22")
        })
        .map(|history| {
            let mut derivatives = vec![history];

            while derivatives.last().unwrap().iter().any(|&i| i != 0) {
                let derivative = derivatives
                    .last()
                    .expect("should not be empty 30")
                    .windows(2)
                    .map(|l| l.last().expect("should not be empty 32") - l.first().unwrap())
                    .collect::<Vec<_>>();

                derivatives.push(derivative)
            }

            derivatives
                .iter()
                .rev()
                .map(|derivative| derivative.first().expect("should not be empty 41"))
                .fold(0, |acc, curr| curr - acc)
        })
        .sum()
}

fn main() {
    let sum = a(INPUT);
    println!("{sum}");
    let sum = b(INPUT);
    println!("{sum}");
}
