use crate::game::Game;
use crate::game::GameSpec;

const DEFAULT_NUM_SIMULATIONS: u64 = 100000;

pub struct EquityResults {
    pub win_percentages: Vec<f64>,
    pub draw_percentages: Vec<f64>,
}

pub fn simulate_equity_from_game_spec(
    game_spec: GameSpec,
    num_simulations: Option<u64>,
) -> EquityResults {
    let num_simulations = match num_simulations {
        Some(num) => num,
        None => DEFAULT_NUM_SIMULATIONS,
    };
    let num_players = game_spec.hole_cards.len();
    let mut player_win_counts: Vec<u64> = vec![0; num_players];
    let mut player_draw_counts: Vec<u64> = vec![0; num_players];

    for _ in 0..num_simulations {
        let mut game = Game::from_spec(&game_spec);
        game.deal_down_to_river();
        let winning_players_and_hands = game.get_winning_players_and_hands();
        let split_pot = winning_players_and_hands.len() > 1;
        for (player, _) in winning_players_and_hands {
            if split_pot {
                player_draw_counts[player] += 1;
            } else {
                player_win_counts[player] += 1;
            }
        }
    }
    let win_percentages: Vec<f64> = player_win_counts
        .iter()
        .map(|&count| count as f64 / num_simulations as f64)
        .collect();
    let draw_percentages: Vec<f64> = player_draw_counts
        .iter()
        .map(|&count| count as f64 / num_simulations as f64)
        .collect();
    EquityResults {
        win_percentages,
        draw_percentages,
    }
}
