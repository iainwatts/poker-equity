mod deck;
use deck::Card;
use deck::Deck;
mod hands;
use hands::Hand;
use itertools::Itertools;

fn main() {
    simulate_nine_players();
}

fn simulate_nine_players() {
    let mut deck = Deck::new();
    deck.shuffle();

    // not a proper deal order!
    let board_cards: Vec<&Card> = (0..5).map(|n| deck.draw_nth(n)).collect();

    let hole_cards_for_players: Vec<Vec<&Card>> = (0..9)
        .map(|player| {
            (0..2)
                .map(|card| deck.draw_nth(player * 2 + card + 5))
                .collect()
        })
        .collect();

    println!(
        "The board came {} {} {} {} {}",
        board_cards[0], board_cards[1], board_cards[2], board_cards[3], board_cards[4],
    );

    let mut player_hands: Vec<Hand> = Vec::new();

    for (player, hole_cards) in hole_cards_for_players.iter().enumerate() {
        println!(
            "Player {} was dealt {} {}",
            player, hole_cards[0], hole_cards[1]
        );

        let player_hand = get_scoring_hand_from_hole_cards_and_board(hole_cards, &board_cards);
        println!("Their hand was:");
        player_hand.print_hand();
        player_hands.push(player_hand);
    }

    let one_best_hand: &Hand = player_hands.iter().max().unwrap();
    let best_hands: Vec<&Hand> = player_hands
        .iter()
        .filter(|hand| hand == &one_best_hand)
        .collect();
    println!("-----------------------");
    println!("Here are the winning hand(s):");
    for hand in best_hands {
        hand.print_hand();
    }
}

fn get_scoring_hand_from_hole_cards_and_board<'a>(
    hole_cards: &'a Vec<&Card>,
    board_cards: &'a Vec<&Card>,
) -> Hand<'a> {
    let mut all_cards: Vec<&Card> = Vec::new();
    all_cards.extend(hole_cards);
    all_cards.extend(board_cards);

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
