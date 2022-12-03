use crate::Priority;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Item(pub char);

impl Priority for Item {
    fn priority(&self) -> usize {
        let ascii = self.0 as usize;

        match ascii {
            65..=90 => ascii - 38,  // A-Z
            97..=122 => ascii - 96, // a-z
            _ => panic!("{} is not a letter", self.0),
        }
    }
}

impl From<char> for Item {
    fn from(item: char) -> Self {
        Self(item)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case('a', 1)]
    #[case('z', 26)]
    #[case('A', 27)]
    #[case('Z', 52)]
    fn priority(#[case] letter: char, #[case] expected: usize) {
        let item = Item(letter);

        assert_eq!(expected, item.priority());
    }

    #[test]
    fn trait_from_str() {
        let item = 'P';

        assert_eq!(Item('P'), item.into());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Item>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Item>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Item>();
    }
}
