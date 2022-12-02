use crate::{Player, Score, Shape, Winnable, Winner};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Round {
    pub player1: Shape,
    pub player2: Shape,
}

impl Winnable for Round {
    fn score_for_player(&self, player: Player) -> Score {
        let score_for_shape: Score = match player {
            Player::Player1 => (&self.player1).into(),
            Player::Player2 => (&self.player2).into(),
        };

        let points_for_winner = match self.winner() {
            Winner::Player(winner) => {
                if winner == player {
                    6
                } else {
                    0
                }
            }
            Winner::Draw => 3,
        };

        Score(points_for_winner) + score_for_shape
    }

    fn winner(&self) -> Winner {
        if self.player1 == self.player2 {
            return Winner::Draw;
        }
        match (self.player1, self.player2) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Winner::Player(Player::Player1),
            _ => Winner::Player(Player::Player2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winner() {
        let pairings = vec![
            (Shape::Rock, Shape::Rock, Winner::Draw),
            (Shape::Rock, Shape::Paper, Winner::Player(Player::Player2)),
            (
                Shape::Rock,
                Shape::Scissors,
                Winner::Player(Player::Player1),
            ),
            (Shape::Paper, Shape::Rock, Winner::Player(Player::Player1)),
            (Shape::Paper, Shape::Paper, Winner::Draw),
            (
                Shape::Paper,
                Shape::Scissors,
                Winner::Player(Player::Player2),
            ),
            (
                Shape::Scissors,
                Shape::Rock,
                Winner::Player(Player::Player2),
            ),
            (
                Shape::Scissors,
                Shape::Paper,
                Winner::Player(Player::Player1),
            ),
            (Shape::Scissors, Shape::Scissors, Winner::Draw),
        ];

        for (player1, player2, result) in pairings {
            let round = Round { player1, player2 };
            let winner = round.winner();

            assert_eq!(result, winner);
        }
    }

    #[test]
    fn score_for_player1() {
        let round = Round {
            player1: Shape::Rock,
            player2: Shape::Scissors,
        };

        assert_eq!(Score(7), round.score_for_player(Player::Player1));
    }

    #[test]
    fn score_for_player2() {
        let round = Round {
            player1: Shape::Paper,
            player2: Shape::Scissors,
        };

        assert_eq!(Score(9), round.score_for_player(Player::Player2));
    }

    #[test]
    fn score_for_draw() {
        let round = Round {
            player1: Shape::Paper,
            player2: Shape::Paper,
        };

        assert_eq!(Score(5), round.score_for_player(Player::Player1));
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Round>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Round>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Round>();
    }
}
