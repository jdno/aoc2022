use std::ops::Add;

use crate::Shape;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Score(pub usize);

impl Add for Score {
    type Output = Score;

    fn add(self, rhs: Self) -> Self::Output {
        Score(self.0 + rhs.0)
    }
}

impl From<usize> for Score {
    fn from(score: usize) -> Self {
        Score(score)
    }
}

impl From<&Shape> for Score {
    fn from(shape: &Shape) -> Self {
        match shape {
            Shape::Rock => Score(1),
            Shape::Paper => Score(2),
            Shape::Scissors => Score(3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Score>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Score>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Score>();
    }
}
