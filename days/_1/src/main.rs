use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let sum = INPUT
        .lines()
        .par_bridge()
        .map(|line| {
            let (a, b) = rayon::join(
                || {
                    let mut i = 0;
                    loop {
                        let line = &line[i..];
                        match line {
                            _ if line.starts_with("one") => return 1,
                            _ if line.starts_with("two") => return 2,
                            _ if line.starts_with("three") => return 3,
                            _ if line.starts_with("four") => return 4,
                            _ if line.starts_with("five") => return 5,
                            _ if line.starts_with("six") => return 6,
                            _ if line.starts_with("seven") => return 7,
                            _ if line.starts_with("eight") => return 8,
                            _ if line.starts_with("nine") => return 9,
                            _ if line.starts_with('1') => return 1,
                            _ if line.starts_with('2') => return 2,
                            _ if line.starts_with('3') => return 3,
                            _ if line.starts_with('4') => return 4,
                            _ if line.starts_with('5') => return 5,
                            _ if line.starts_with('6') => return 6,
                            _ if line.starts_with('7') => return 7,
                            _ if line.starts_with('8') => return 8,
                            _ if line.starts_with('9') => return 9,
                            _ => i += 1,
                        };
                    }
                },
                || {
                    let mut i = 0;
                    loop {
                        let line = &line[..(line.len() - i)];
                        match line {
                            _ if line.ends_with("one") => return 1,
                            _ if line.ends_with("two") => return 2,
                            _ if line.ends_with("three") => return 3,
                            _ if line.ends_with("four") => return 4,
                            _ if line.ends_with("five") => return 5,
                            _ if line.ends_with("six") => return 6,
                            _ if line.ends_with("seven") => return 7,
                            _ if line.ends_with("eight") => return 8,
                            _ if line.ends_with("nine") => return 9,
                            _ if line.ends_with('1') => return 1,
                            _ if line.ends_with('2') => return 2,
                            _ if line.ends_with('3') => return 3,
                            _ if line.ends_with('4') => return 4,
                            _ if line.ends_with('5') => return 5,
                            _ if line.ends_with('6') => return 6,
                            _ if line.ends_with('7') => return 7,
                            _ if line.ends_with('8') => return 8,
                            _ if line.ends_with('9') => return 9,
                            _ => i += 1,
                        };
                    }
                },
            );
            a * 10 + b
        })
        .sum::<u16>();

    println!("{sum}");
}
