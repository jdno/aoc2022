#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Player {
    Player1,
    Player2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Player>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Player>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Player>();
    }
}
