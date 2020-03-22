mod cards;
use cards::Card;
mod equity;
mod game;
use game::GameSpec;
mod hands;

fn main() {
    let mut board = Vec::new();
    board.push(Card::from_str("Qs"));
    board.push(Card::from_str("Kd"));
    board.push(Card::from_str("Jc"));
    board.push(Card::from_str("Tc"));
    let mut hole_cards = Vec::new();
    hole_cards.push((Card::from_str("Qh"), Card::from_str("Qd")));
    hole_cards.push((Card::from_str("Ac"), Card::from_str("As")));
    let game_spec = GameSpec { board, hole_cards };

    let results = equity::simulate_equity_from_game_spec(game_spec, Some(100000));
    let zipped_percs = results
        .win_percentages
        .iter()
        .zip(results.draw_percentages.iter());
    for (player, (win_perc, draw_perc)) in zipped_percs.enumerate() {
        println!(
            "Equity for player {}: {}% ({}% draw)",
            player, win_perc, draw_perc
        );
    }
}
