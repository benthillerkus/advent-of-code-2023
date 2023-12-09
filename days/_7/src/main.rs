use std::{cell::RefCell, cmp::Ordering, collections::HashSet};

use nom::{
    character::complete::char,
    character::complete::{anychar, digit1},
    combinator::map,
    multi::count,
    Finish, IResult,
};
use rayon::prelude::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Label {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Label {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => return Err(()),
        })
    }
}

#[test]
fn order_label() {
    assert!(Label::A > Label::Two)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
enum Kind {
    /// All distinct
    HighCard,
    /// One pair
    OnePair,
    /// Two pairs of cards with the same label + one card with a different label
    TwoPair,
    /// Three labels are the same
    ThreeOfAKind,
    /// 2 distinct - There are only two different labels in the Hand
    FullHouse,
    /// 2 distinct - All but one labels are the same
    FourOfAKind,
    /// All Labels are the same
    FiveOfAKind,
}

thread_local! {static SET: RefCell< HashSet<Label>> = {let mut set = HashSet::new(); set.reserve(5); RefCell::new( set)}}

impl From<&[Label; 5]> for Kind {
    fn from(cards: &[Label; 5]) -> Self {
        SET.with(|set: &RefCell<HashSet<_>>| {
            let mut set = set.try_borrow_mut().expect("it to be able to be borrowed");
            set.clear();
            cards.iter().for_each(|&card| {
                set.insert(card);
            });
            match set.len() {
                5 => Self::HighCard,
                4 => Self::OnePair,
                3 => {
                    if set
                        .iter()
                        .copied()
                        .map(|d| cards.iter().copied().filter(|&c| c == d).count())
                        .any(|n| n == 3)
                    {
                        Kind::ThreeOfAKind
                    } else {
                        Kind::TwoPair
                    }
                }
                2 => {
                    let mut counts = set
                        .iter()
                        .copied()
                        .map(|d| cards.iter().copied().filter(|&c| c == d).count());
                    match counts.next().unwrap() {
                        1 | 4 => Kind::FourOfAKind,
                        2 | 3 => Kind::FullHouse,
                        _ => panic!(),
                    }
                }
                1 => Self::FiveOfAKind,
                _ => panic!(),
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand {
    kind: Kind,
    cards: [Label; 5],
    bid: u16,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        let ordering = self.kind.cmp(&other.kind);
        if ordering != Ordering::Equal {
            return ordering;
        }

        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                return a.cmp(b);
            }
        }

        Ordering::Equal
    }
}

impl Hand {
    fn new(cards: &[Label], bid: u16) -> Self {
        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];
        Hand {
            cards,
            kind: Kind::from(&cards),
            bid,
        }
    }
}

fn hand(input: &str) -> IResult<&str, Hand> {
    let card = map(anychar, |s| Label::try_from(s).unwrap());
    let (input, cards) = count(card, 5)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, bid) = map(digit1, |s| str::parse(s).unwrap())(input)?;

    Ok((input, Hand::new(&cards, bid)))
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (_, hand) = hand(value).finish().unwrap();
        hand
    }
}

fn a(input: &str) -> usize {
    let mut hands: Vec<_> = input.par_lines().map(Hand::from).collect();

    hands.sort_unstable();

    hands
        .par_iter()
        .enumerate()
        .map(|(rank, hand)| rank * hand.bid as usize)
        .sum()
}

fn b() {
    let sum = 0;

    println!("{sum}");
}

fn main() {
    println!("{}", a(INPUT));
    b();
}

#[cfg(test)]
mod tests {
    use super::*;

    mod order {
        use crate::Hand;

        #[test]
        fn order1() {
            let a = Hand::from("T66KJ 1");
            let b = Hand::from("AAKAA 0");

            assert!(a < b)
        }

        #[test]
        fn order2() {
            let a = Hand::from("22224 1");
            let b = Hand::from("22223 1");

            assert!(a > b)
        }
    }

    mod integration {
        use crate::a;

        #[test]
        fn integration1() {
            let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
            assert_eq!(a(input), 6440);
        }
    }

    mod hand {
        use crate::Hand;
        use crate::Kind;
        use crate::Label::*;

        #[test]
        fn hand1() {
            let input = "32T3K 765";
            let hand = Hand::from(input);
            assert_eq!(
                hand,
                Hand {
                    kind: Kind::OnePair,
                    cards: [Three, Two, T, Three, K],
                    bid: 765
                }
            )
        }

        #[test]
        fn hand2() {
            let input = "T55J5 684";
            let hand = Hand::from(input);
            assert_eq!(
                hand,
                Hand {
                    kind: Kind::ThreeOfAKind,
                    cards: [T, Five, Five, J, Five],
                    bid: 684
                }
            )
        }

        #[test]
        fn hand3() {
            let input = "KK677 28";
            let hand = Hand::from(input);
            assert_eq!(
                hand,
                Hand {
                    kind: Kind::TwoPair,
                    cards: [K, K, Six, Seven, Seven],
                    bid: 28
                }
            )
        }

        #[test]
        fn hand4() {
            let input = "KTJJT 220";
            let hand = Hand::from(input);
            assert_eq!(
                hand,
                Hand {
                    kind: Kind::TwoPair,
                    cards: [K, T, J, J, T],
                    bid: 220
                }
            )
        }

        #[test]
        fn hand5() {
            let input = "QQQJA 483";
            let hand = Hand::from(input);
            assert_eq!(
                hand,
                Hand {
                    kind: Kind::ThreeOfAKind,
                    cards: [Q, Q, Q, J, A],
                    bid: 483
                }
            )
        }
    }

    mod kind {
        use super::*;
        use Label::*;

        #[test]
        fn one_pair() {
            let hand = [Three, Two, T, Three, K];
            let kind = Kind::from(&hand);
            assert_eq!(kind, Kind::OnePair)
        }
        #[test]
        fn two_pair1() {
            let hand = [K, K, Six, Six, Seven];
            let kind = Kind::from(&hand);
            assert_eq!(kind, Kind::TwoPair)
        }
        #[test]
        fn one_pair2() {
            let hand = [K, T, J, J, T];
            let kind = Kind::from(&hand);
            assert_eq!(kind, Kind::TwoPair)
        }
        #[test]
        fn three_of_a_kind1() {
            let hand = [T, Five, Five, J, Five];
            let kind = Kind::from(&hand);
            assert_eq!(kind, Kind::ThreeOfAKind)
        }
        #[test]
        fn three_of_a_kind2() {
            let hand = [Q, Q, Q, J, A];
            let kind = Kind::from(&hand);
            assert_eq!(kind, Kind::ThreeOfAKind)
        }
        #[test]
        fn full_house() {
            let hand = [A, Q, Q, A, A];
            let kind = Kind::from(&hand);
            assert_eq!(kind, Kind::FullHouse)
        }
        #[test]
        fn four_of_a_kind() {
            let hand = [A, Q, A, A, A];
            let kind = Kind::from(&hand);
            assert_eq!(kind, Kind::FourOfAKind)
        }
    }
}
