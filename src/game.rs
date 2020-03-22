use crate::cards::Card;
use crate::hands::Hand;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::iter::FromIterator;

// A GameSpec represents incomplete information about a game situation
// which can be used to construct a Game by randomly filling in the
// unknown cards.
pub struct GameSpec {
    pub board: Vec<Card>,
    pub hole_cards: Vec<(Card, Card)>,
}

pub struct Game {
    deck: Vec<Card>,
    board: Vec<Card>,
    hole_cards: Vec<(Card, Card)>,
}

impl Game {
    pub fn from_spec(spec: &GameSpec) -> Game {
        let mut cards_set: HashSet<Card> = HashSet::from_iter(Card::create_deck());
        let mut board = Vec::new();
        let mut hole_cards = Vec::new();

        // Set up board
        for spec_card in &spec.board {
            let card = cards_set.take(&spec_card).unwrap();
            board.push(card);
        }

        // Set up hole cards
        for (spec_card_1, spec_card_2) in &spec.hole_cards {
            let card_1 = cards_set.take(&spec_card_1).unwrap();
            let card_2 = cards_set.take(&spec_card_2).unwrap();
            hole_cards.push((card_1, card_2));
        }

        // Put remaining cards into deck and shuffle
        let mut deck: Vec<Card> = cards_set.into_iter().collect();
        deck.shuffle(&mut thread_rng());

        Game {
            deck,
            board,
            hole_cards,
        }
    }

    pub fn deal_down_to_river(&mut self) {
        let cards_in_board = self.board.len();
        for n in 0..5 {
            if n >= cards_in_board {
                self.board.push(self.deck.pop().unwrap());
            }
        }
    }

    pub fn get_player_hands(&self) -> Vec<Hand> {
        self.hole_cards
            .iter()
            .map(|player_hole_cards| self.get_scoring_hand_for_player(player_hole_cards))
            .collect()
    }

    pub fn get_winning_players_and_hands(&self) -> Vec<(usize, Hand)> {
        // There must be a better way to do this, but when I try it all in one
        // iterator, the borrow checker complains because I'm trying to move out
        // of `player_hands` while I still have a reference (one_best_hand)
        // Hence the indirection of collecting the player numbers and then moving out
        let player_hands: Vec<Hand> = self.get_player_hands();
        let one_best_hand: &Hand = player_hands.iter().max().unwrap();
        let winning_players: HashSet<usize> = HashSet::from_iter(
            player_hands
                .iter()
                .enumerate()
                .filter(|(_, hand)| hand == &one_best_hand)
                .map(|(player, _)| player),
        );

        let winning_players_and_hands: Vec<(usize, Hand)> = player_hands
            .into_iter()
            .enumerate()
            .filter(|(player, _)| winning_players.contains(player))
            .collect();
        winning_players_and_hands
    }

    fn get_scoring_hand_for_player<'a>(&'a self, player_hole_cards: &'a (Card, Card)) -> Hand<'a> {
        let (hole_card_1, hole_card_2) = player_hole_cards;
        let mut all_cards: Vec<&Card> = Vec::new();
        all_cards.push(&hole_card_1);
        all_cards.push(&hole_card_2);
        all_cards.extend::<Vec<&Card>>(self.board.iter().collect());

        let possible_card_combos: Vec<Vec<&Card>> = all_cards
            .iter()
            .combinations(5)
            .map(|cards: Vec<&&Card>| cards.iter().map(|&&c| c).collect())
            .collect();

        let player_hand: Hand = possible_card_combos
            .iter()
            .map(|cards: &Vec<&Card>| Hand::new(cards.to_vec()))
            .max()
            .unwrap();

        player_hand
    }
}
