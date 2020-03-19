use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: u8,
    pub suit: Suit,
}

impl Card {
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

    pub fn create_deck() -> Vec<Card> {
        let suits = [Suit::Clubs, Suit::Hearts, Suit::Diamonds, Suit::Spades];
        let cards: Vec<Card> = suits
            .iter()
            .copied()
            .flat_map(|suit| (2..15).map(move |rank| Card { rank, suit }))
            .collect();
        cards
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank_as_string(), self.suit_as_char())
    }
}
