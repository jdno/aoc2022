pub use self::game::*;
pub use self::player::*;
pub use self::round::*;
pub use self::score::*;
pub use self::shape::*;
pub use self::winner::*;

mod game;
mod player;
mod round;
mod score;
mod shape;
mod winner;

pub trait Winnable {
    fn score_for_player(&self, player: Player) -> Score;
    fn winner(&self) -> Winner;
}
