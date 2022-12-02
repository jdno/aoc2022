use day2::{Game, Player, Round, Winnable};

fn main() {
    let input = include_str!("../../day2.txt");

    let rounds = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();

            Round {
                player1: (*parts.first().unwrap()).into(),
                player2: (*parts.last().unwrap()).into(),
            }
        })
        .collect();

    let game = Game { rounds };
    let score = game.score_for_player(Player::Player2);

    println!("Score: {}", score.0);
}
