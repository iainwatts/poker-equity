use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

impl Card {
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Card {
        let first_char: &str = &s[..1];
        let second_char: &str = &s[1..2];
        let rank: u8 = match first_char {
            "T" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => first_char.parse::<u8>().unwrap(),
        };
        let suit: Suit = match second_char {
            "c" => Suit::Clubs,
            "h" => Suit::Hearts,
            "d" => Suit::Diamonds,
            "s" => Suit::Spades,
            _ => panic!("Invalid suit!"),
        };
        Card { rank, suit }
    }

    pub fn rank_as_string(&self) -> String {
        match self.rank {
            2..=9 => self.rank.to_string(),
            10 => String::from("T"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            14 => String::from("A"),
            _ => panic!("Invalid rank!"),
        }
    }

    #[allow(dead_code)]
    pub fn suit_as_string(&self) -> &str {
        match self.suit {
            Suit::Clubs => "clubs",
            Suit::Hearts => "hearts",
            Suit::Diamonds => "diamonds",
            Suit::Spades => "spaces",
        }
    }

    pub fn suit_as_char(&self) -> &str {
        match self.suit {
            Suit::Clubs => "c",
            Suit::Hearts => "h",
            Suit::Diamonds => "d",
            Suit::Spades => "s",
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank_as_string(), self.suit_as_char())
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    // Construct a fresh desk, ordered
    pub fn new() -> Deck {
        let suits = [Suit::Clubs, Suit::Hearts, Suit::Diamonds, Suit::Spades];
        let cards: Vec<Card> = suits
            .iter()
            .copied()
            .flat_map(|suit| (2..15).map(move |rank| Card { rank, suit }))
            .collect();
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn draw_nth(&self, n: usize) -> &Card {
        // Rust wouldn't let me keep the "next card" state inside the
        // Deck struct because it required this method to have `&mut self`
        // and you can't do multiple mutable borrows, i.e I couldn't draw
        // more than one care
        // An alternative is to have draw() transfer ownership, but I didn't
        // want to do that - wanted the deck to keep track of the cards
        // Q - other way to do this?
        &self.cards[n]
    }
}
