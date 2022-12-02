use day2::{Game, Player, Round, Shape, Winnable};

fn main() {
    let input = include_str!("../../day2.txt");

    let rounds = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();

            let player1 = (*parts.first().unwrap()).into();
            let instruction = *parts.last().unwrap();

            let player2 = match (player1, instruction) {
                (Shape::Rock, "X") => Shape::Scissors,
                (Shape::Rock, "Z") => Shape::Paper,
                (Shape::Paper, "X") => Shape::Rock,
                (Shape::Paper, "Z") => Shape::Scissors,
                (Shape::Scissors, "X") => Shape::Paper,
                (Shape::Scissors, "Z") => Shape::Rock,
                (_, "Y") => player1,
                _ => panic!("({:?}, {}) is not a valid input", player1, instruction),
            };

            Round { player1, player2 }
        })
        .collect();

    let game = Game { rounds };
    let score = game.score_for_player(Player::Player2);

    println!("Score: {}", score.0);
}
