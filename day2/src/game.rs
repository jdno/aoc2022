use crate::{Player, Round, Score, Winnable, Winner};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Game {
    pub rounds: Vec<Round>,
}

impl Winnable for Game {
    fn score_for_player(&self, player: Player) -> Score {
        self.rounds
            .iter()
            .map(|round| round.score_for_player(player))
            .fold(Score(0), |a, b| a + b)
    }

    fn winner(&self) -> Winner {
        let player1_score = self.score_for_player(Player::Player1);
        let player2_score = self.score_for_player(Player::Player2);

        if player1_score == player2_score {
            return Winner::Draw;
        }

        if player1_score > player2_score {
            Winner::Player(Player::Player1)
        } else {
            Winner::Player(Player::Player2)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Shape;

    use super::*;

    #[test]
    fn score_for_player() {
        let game = Game {
            rounds: vec![Round {
                player1: Shape::Rock,
                player2: Shape::Scissors,
            }],
        };

        assert_eq!(Score(7), game.score_for_player(Player::Player1));
    }

    #[test]
    fn winner() {
        let game = Game {
            rounds: vec![Round {
                player1: Shape::Rock,
                player2: Shape::Scissors,
            }],
        };

        assert_eq!(Winner::Player(Player::Player1), game.winner());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Game>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Game>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Game>();
    }
}
