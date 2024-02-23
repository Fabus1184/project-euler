#!/usr/bin/env rust-script

// cargo-deps: file-fetcher = "0.1.4"

use std::io::Read;
use std::collections::HashMap;

let hands = file_fetcher::open_str("https://projecteuler.net/project/resources/p054_poker.txt")
    .unwrap()
    .bytes()
    .map(|x| x.unwrap() as char)
    .collect::<String>()
    .split("
")
    .filter(|x| x.len() > 0)
    .map(|x| (
        x[..14].split(" ").map(|c| Card::from(c)).collect::<Vec<_>>(),
        x[15..].split(" ").map(|c| Card::from(c)).collect::<Vec<_>>())
    )
    .collect::<Vec<_>>();

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl From<char> for Suit {
    fn from(c: char) -> Self {
        match c {
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'H' => Suit::Hearts,
            'S' => Suit::Spades,
            _ => panic!("Invalid suit"),
        }
    }
}

impl From<char> for Rank {
    fn from(c: char) -> Self {
        match c {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => panic!("Invalid rank"),
        }
    }
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        Card {
            rank: Rank::from(s.chars().nth(0).unwrap()),
            suit: Suit::from(s.chars().nth(1).unwrap()),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Hand {
    HighCard { card: Card },
    OnePair { pair: Rank, kicker: Card },
    TwoPair { pair1: Rank, pair2: Rank, kicker: Card },
    ThreeOfAKind { triple: Rank, kicker1: Card, kicker2: Card },
    Straight { high: Rank },
    Flush { high: Rank },
    FullHouse { triple: Rank, pair: Rank },
    FourOfAKind { quad: Rank, kicker: Card },
    StraightFlush { high: Rank },
    RoyalFlush,
}

fn get_hand(_cards: &Vec<Card>) -> Hand {
    let mut cards = _cards.clone();
    cards.sort();

    let counts = cards.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c.rank).or_insert(0) += 1;
        acc
    });

    let mut pairs = counts.iter().filter(|(_, &v)| v == 2).collect::<Vec<_>>();
    pairs.sort_by(|a, b| b.1.cmp(a.1));

    let mut triples = counts.iter().filter(|(_, &v)| v == 3).collect::<Vec<_>>();
    triples.sort_by(|a, b| b.1.cmp(a.1));

    let mut quads = counts.iter().filter(|(_, &v)| v == 4).collect::<Vec<_>>();
    quads.sort_by(|a, b| b.1.cmp(a.1));

    let flush = cards.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c.suit).or_insert(0) += 1;
        acc
    }).iter().filter(|(_, &v)| v >= 5).count() > 0;

    let mut straight = cards.windows(2).all(|w| w[0].rank as u8 + 1 == w[1].rank as u8);

    if straight && cards[0].rank == Rank::Two && cards[4].rank == Rank::Ace {
        straight = false;
        cards = cards.iter().map(|c| {
            if c.rank == Rank::Ace {
                Card { rank: Rank::Ace, suit: c.suit }
            } else {
                *c
            }
        }).collect();
    }

    if straight && flush {
        if cards[4].rank == Rank::Ace {
            Hand::RoyalFlush
        } else {
            Hand::StraightFlush { high: cards[4].rank }
        }
    } else if quads.len() > 0 {
        Hand::FourOfAKind { quad: quads[0].0.clone(), kicker: cards.iter().filter(|c| c.rank != *quads[0].0).max().unwrap().clone() }
    } else if triples.len() > 0 && pairs.len() > 0 {
        Hand::FullHouse { triple: triples[0].0.clone(), pair: pairs[0].0.clone() }
    } else if flush {
        Hand::Flush { high: cards[4].rank }
    } else if straight {
        Hand::Straight { high: cards[4].rank }
    } else if triples.len() > 0 {
        Hand::ThreeOfAKind { triple: triples[0].0.clone(), kicker1: cards.iter().filter(|c| c.rank != *triples[0].0).max().unwrap().clone(), kicker2: cards.iter().filter(|c| c.rank != *triples[0].0 && c.rank != cards.iter().filter(|c| c.rank != *triples[0].0).max().unwrap().rank).max().unwrap().clone() }
    } else if pairs.len() > 1 {
        Hand::TwoPair { pair1: pairs[0].0.clone(), pair2: pairs[1].0.clone(), kicker: cards.iter().filter(|c| c.rank != *pairs[0].0 && c.rank != *pairs[1].0).max().unwrap().clone() }
    } else if pairs.len() > 0 {
        Hand::OnePair { pair: pairs[0].0.clone(), kicker: cards.iter().filter(|c| c.rank != *pairs[0].0).max().unwrap().clone() }
    } else {
        Hand::HighCard { card: cards[4].clone() }
    }
}

println!("{}", 
    hands.iter().filter(|(p1, p2)| {
        let h1 = get_hand(&p1);
        let h2 = get_hand(&p2);
        h1 > h2
    }).count()
)
