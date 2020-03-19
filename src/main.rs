mod cards;
use cards::Card;
mod game;
use game::GameSpec;
use game::Game;
mod hands;

const NUM_SIMULATIONS: u64 = 1000000;

fn main() {
    let mut board = Vec::new();
    board.push(Card::from_str("Qs"));
    board.push(Card::from_str("Kd"));
    board.push(Card::from_str("Jc"));
    board.push(Card::from_str("Tc"));
    let mut hole_cards = Vec::new();
    hole_cards.push(Some( (Card::from_str("Qh"), Card::from_str("Qd")) ));
    hole_cards.push(Some( (Card::from_str("Ac"), Card::from_str("As")) ));
    let game_spec = GameSpec { board, hole_cards };

    let mut player_win_counts: Vec<u64> = vec![0, 0];

    for _ in 0..NUM_SIMULATIONS {
        let mut game = Game::from_spec(&game_spec);
        game.deal_down_to_river();
        let winning_players_and_hands = game.get_winning_players_and_hands();
        if winning_players_and_hands.len() == 1 {
            let (player, _) = winning_players_and_hands.iter().next().unwrap();
            player_win_counts[*player] += 1;
        }
    }
    println!("Out of a total of {} runs", NUM_SIMULATIONS);
    println!("{} wins for player 0", player_win_counts[0]);
    println!("{} wins for player 1", player_win_counts[1]);
    println!("{} draws", NUM_SIMULATIONS - player_win_counts[0] - player_win_counts[1]);
    println!("Equity for player 1: {}", player_win_counts[0] as f64 / NUM_SIMULATIONS as f64);
    println!("Equity for player 2: {}", player_win_counts[1] as f64 / NUM_SIMULATIONS as f64);
}
